#![feature(test)]
#![feature(concat_idents)]

extern crate test;

enum Buffer {}
enum BufferBytesIter {}
enum BufferBytesRIter {}


use std::ops::Add;
use std::os::raw::{c_char, c_int, c_ulonglong};
use std::ffi::CString;

extern "C" {
    fn buffer_new()->*mut Buffer;
    fn buffer_clone(buffer: *mut Buffer)->*mut Buffer;
    fn buffer_delete(buffer: *mut Buffer);
    fn buffer_push_back_byte(buffer: *mut Buffer, byte:c_char)->*mut Buffer;
    fn buffer_push_back_string(buffer: *mut Buffer, string:*const c_char)->*mut Buffer;

    fn buffer_push_front_byte(buffer: *mut Buffer, byte:c_char)->*mut Buffer;
    fn buffer_push_front_string(buffer: *mut Buffer, string:*const c_char)->*mut Buffer;

    fn buffer_insert_byte(buffer: *mut Buffer,index: c_ulonglong, byte:c_char)->*mut Buffer;
    fn buffer_insert_string(buffer: *mut Buffer,index: c_ulonglong,  string:*const c_char)->*mut Buffer;

    fn buffer_delete_byte(buffer: *mut Buffer, index: c_ulonglong)->*mut Buffer;
    fn buffer_delete_range(buffer: *mut Buffer, start: c_ulonglong, end: c_ulonglong)->*mut Buffer;

    fn buffer_concat(left: *mut Buffer, right: *mut Buffer)->*mut Buffer;
    fn buffer_get_size(buffer: *mut Buffer)->c_ulonglong;

    fn buffer_iter_delete(iter: *mut BufferBytesIter);
    fn buffer_get_iter(buffer: *mut Buffer)->*mut BufferBytesIter;
    fn buffer_iter_has_next(iter: *mut BufferBytesIter)->c_int;
    fn buffer_iter_next(iter: *mut BufferBytesIter)->c_char;

    fn buffer_riter_delete(iter: *mut BufferBytesRIter);
    fn buffer_get_riter(buffer: *mut Buffer)->*mut BufferBytesRIter;
    fn buffer_riter_has_next(iter: *mut BufferBytesRIter)->c_int;
    fn buffer_riter_next(iter: *mut BufferBytesRIter)->c_char;
}

pub struct FlexVector{
    buf: *mut Buffer,
}


impl FlexVector{
    pub fn new() -> FlexVector{
        unsafe{
            return FlexVector{
                buf: buffer_new(),
            }
        }
    }

    pub fn len(&self)->usize{
        unsafe{
            return buffer_get_size(self.buf) as usize;
        }
    }


    pub fn push_back(&self, elem: u8)->FlexVector{
        unsafe{
            return FlexVector{
                buf: buffer_push_back_byte(self.buf, elem as c_char),
            }
        }
    }

    pub fn push_back_char(&self, elem: char)->FlexVector{

        let mut b = [0; 4];
        return self.push_back_string(elem.encode_utf8(&mut b));
    }


    pub fn push_back_string(&self, string: &str)->FlexVector{
        unsafe{
            return FlexVector{
                buf: buffer_push_back_string(self.buf, CString::new(string).unwrap().as_ptr()),
            }
        }
    }

    pub fn push_front(&self, elem: u8)->FlexVector{
        unsafe{
            return FlexVector{
                buf: buffer_push_front_byte(self.buf, elem as c_char),
            }
        }
    }

    pub fn push_front_char(&self, elem: char)->FlexVector{

        let mut b = [0; 4];
        return self.push_front_string(elem.encode_utf8(&mut b));
    }


    pub fn push_front_string(&self, string: &str)->FlexVector{
        unsafe{
            return FlexVector{
                buf: buffer_push_front_string(self.buf, CString::new(string).unwrap().as_ptr()),
            }
        }
    }

    pub fn insert(&self, index: usize, elem: u8)->Option<FlexVector>{
        unsafe{
            if index>self.len(){
                None
            }else{
                Some(FlexVector{
                    buf: buffer_insert_byte(self.buf, index as c_ulonglong, elem as c_char),
                })
            }
        }
    }

    pub fn insert_char(&self, index: usize, elem: char)->Option<FlexVector>{

        let mut b = [0; 4];
        return self.insert_string(index, elem.encode_utf8(&mut b));
    }

    pub fn insert_string(&self, index: usize, string: &str)->Option<FlexVector>{
        unsafe{
            if index>self.len(){
                None
            }else{
                Some(FlexVector{
                    buf: buffer_insert_string(self.buf, index as c_ulonglong, CString::new(string).unwrap().as_ptr()),
                })
            }
        }
    }

    pub fn delete(&self, index: usize)->Option<FlexVector>{
        unsafe{
            if index>=self.len(){
                None
            }else{
                Some(FlexVector{
                    buf: buffer_delete_byte(self.buf, index as c_ulonglong),
                })
            }
        }
    }

    pub fn delete_range(&self, start: usize, end: usize)->Option<FlexVector>{
        unsafe{
            if start>=end || end>self.len(){
                None
            }else{
                Some(FlexVector{
                    buf: buffer_delete_range(self.buf, start as c_ulonglong, end as c_ulonglong),
                })
            }
        }
    }

    pub fn concat(&self, other: &FlexVector) -> FlexVector{
        unsafe{
            return FlexVector{
                buf: buffer_concat(self.buf, other.buf),
            }
        }
    }

    pub fn bytes(&self)->FlexVectorIter{
        return FlexVectorIter::new(self);
    }

    pub fn reverse_bytes(&self)->FlexVectorRIter{
        return FlexVectorRIter::new(self);
    }
}

impl Add for FlexVector{
    type Output = FlexVector;
    fn add(self, other: FlexVector) -> Self::Output{
        return self.concat(&other);
    }
}

impl <'a, 'b> Add<&'a FlexVector> for &'b FlexVector{
    type Output = FlexVector;
    fn add(self, other: &FlexVector) -> Self::Output{
        return self.concat(&other);
    }
}

impl <'a> Add<&'a FlexVector> for FlexVector{
    type Output = FlexVector;
    fn add(self, other: &FlexVector) -> Self::Output{
        return self.concat(&other);
    }
}

impl Clone for FlexVector{
    fn clone(&self)->FlexVector{
        unsafe{
            return FlexVector{
                buf: buffer_clone(self.buf),
            }
        }
    }
}

impl Drop for FlexVector{
    fn drop(&mut self){
        unsafe{
            buffer_delete(self.buf);
        }
    }
}



pub struct FlexVectorIter{
    iter: *mut BufferBytesIter,
}

impl FlexVectorIter{
    fn new(vec: &FlexVector)->FlexVectorIter{
        unsafe{
            return FlexVectorIter{
                iter: buffer_get_iter(vec.buf),
            }
        }
    }
}

impl Drop for FlexVectorIter{
    fn drop(&mut self){
        unsafe{
            buffer_iter_delete(self.iter);
        }
    }
}

impl Iterator for FlexVectorIter{
    type Item = u8;

    fn next(&mut self)->Option<Self::Item>{
        unsafe{
            match buffer_iter_has_next(self.iter){
                0=>None,
                _=>Some(buffer_iter_next(self.iter) as u8),

            }
        }
    }
}

pub struct FlexVectorRIter{
    iter: *mut BufferBytesRIter,
}

impl FlexVectorRIter{
    fn new(vec: &FlexVector)->FlexVectorRIter{
        unsafe{
            return FlexVectorRIter{
                iter: buffer_get_riter(vec.buf),
            }
        }
    }
}

impl Drop for FlexVectorRIter{
    fn drop(&mut self){
        unsafe{
            buffer_riter_delete(self.iter);
        }
    }
}

impl Iterator for FlexVectorRIter{
    type Item = u8;

    fn next(&mut self)->Option<Self::Item>{
        unsafe{
            match buffer_riter_has_next(self.iter){
                0=>None,
                _=>Some(buffer_riter_next(self.iter) as u8),

            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str::from_utf8;
    use test::Bencher;

    macro_rules! assert_vec_eq {
        ($left:expr, $right:expr)=> ({
            let _b: Vec<u8> = ($left).bytes().collect();
            assert_eq!(from_utf8(_b.as_slice()).unwrap(), $right);
        })
    }

    macro_rules! bench_magnitude_push_back {
        ($name1:ident, $name2:ident, $magnitude:expr) =>(
            #[bench]
            fn $name1(b: &mut Bencher){

                b.iter(||{
                    let mut flex = FlexVector::new();
                    for _ in 0..($magnitude){
                        flex = flex.push_back(65);;
                    }
                });
            }
            #[bench]
            fn $name2(b: &mut Bencher){
                let s = "A".repeat($magnitude);
                let s2 = s.as_str();
                b.iter(||{
                    FlexVector::new().push_back_string(s2);
                });
            }
        )
    }

    macro_rules! bench_magnitude_push_front {
        ($name1:ident, $name2:ident, $magnitude:expr) =>(
            #[bench]
            fn $name1(b: &mut Bencher){

                b.iter(||{
                    let mut flex = FlexVector::new();
                    for _ in 0..($magnitude){
                        flex = flex.push_front(65);;
                    }
                });
            }
            #[bench]
            fn $name2(b: &mut Bencher){
                let s = "A".repeat($magnitude);
                let s2 = s.as_str();
                b.iter(||{
                    FlexVector::new().push_front_string(s2);
                });
            }
        )
    }


    macro_rules! bench_magnitude_insert {
        ($name1:ident, $name2:ident, $mag1:expr, $mag2:expr) =>(
            #[bench]
            fn $name1(b: &mut Bencher){
                let s = "A".repeat($mag1);
                let s2 = s.as_str();
                let flex = FlexVector::new().push_back_string(s2);
                b.iter(||{
                    let mut flex = flex.clone();
                    for i in 0..($mag2){
                        flex = flex.insert($mag1/2+i as usize, 65).unwrap();
                    }
                });
            }
            #[bench]
            fn $name2(b: &mut Bencher){
                let s = "A".repeat($mag1);
                let t = "A".repeat($mag2);
                let s2 = s.as_str();
                let t2 = t.as_str();
                let flex = FlexVector::new().push_back_string(s2);
                b.iter(||{
                    flex.insert_string($mag1/2, t2);
                });
            }
        )
    }



    #[test]
    fn push_back_test(){
        let vec1 = FlexVector::new();
        let vec2 = vec1.push_back(65);
        let vec3 = vec2.push_back_char('B');
        let vec4 = vec3.push_back_string("CD");

        assert_vec_eq!(vec1, "");
        assert_vec_eq!(vec2, "A");
        assert_vec_eq!(vec3, "AB");
        assert_vec_eq!(vec4, "ABCD");
    }

    #[test]
    fn push_front_test(){
        let vec1 = FlexVector::new();
        let vec2 = vec1.push_front(65);
        let vec3 = vec2.push_front_char('B');
        let vec4 = vec3.push_front_string("CD");

        assert_vec_eq!(vec1, "");
        assert_vec_eq!(vec2, "A");
        assert_vec_eq!(vec3, "BA");
        assert_vec_eq!(vec4, "CDBA");
    }

    #[test]
    fn insert_test(){
        let vec1 = FlexVector::new().push_back_string("ABCDEFGHIJKLMNOP");
        let vec2 = vec1.insert(0, 65).unwrap();
        let vec3 = vec2.insert_char(3, 'C').unwrap();
        let vec4 = vec3.insert_string(9, "XYZ").unwrap();

        assert_vec_eq!(vec1, "ABCDEFGHIJKLMNOP");
        assert_vec_eq!(vec2, "AABCDEFGHIJKLMNOP");
        assert_vec_eq!(vec3, "AABCCDEFGHIJKLMNOP");
        assert_vec_eq!(vec4, "AABCCDEFGXYZHIJKLMNOP");
    }

    #[test]
    fn delete_test(){
        let vec1 = FlexVector::new().push_back_string("ABCDEFGHIJKLMNOP");

        let vec2 = vec1.delete(15).unwrap();
        let vec3 = vec1.delete_range(2,4).unwrap();
        let vec4 = vec1.delete_range(14,16).unwrap();
        let vec5 = vec1.delete_range(0,1).unwrap();
        let vec6 = vec1.delete_range(0,16).unwrap();

        assert_vec_eq!(vec2, "ABCDEFGHIJKLMNO");
        assert_vec_eq!(vec3, "ABEFGHIJKLMNOP");
        assert_vec_eq!(vec4, "ABCDEFGHIJKLMN");
        assert_vec_eq!(vec5, "BCDEFGHIJKLMNOP");
        assert_vec_eq!(vec6, "");
    }


    bench_magnitude_push_back!(bench_push_back_byte_1, bench_push_back_string_1, 1);
    bench_magnitude_push_back!(bench_push_back_byte_10, bench_push_back_string_10, 10);
    bench_magnitude_push_back!(bench_push_back_byte_100, bench_push_back_string_100, 100);
    bench_magnitude_push_back!(bench_push_back_byte_1000, bench_push_back_string_1000, 1000);
    bench_magnitude_push_back!(bench_push_back_byte_10000, bench_push_back_string_10000, 10000);


    bench_magnitude_push_front!(bench_push_front_byte_1, bench_push_front_string_1, 1);
    bench_magnitude_push_front!(bench_push_front_byte_10, bench_push_front_string_10, 10);
    bench_magnitude_push_front!(bench_push_front_byte_100, bench_push_front_string_100, 100);
    bench_magnitude_push_front!(bench_push_front_byte_1000, bench_push_front_string_1000, 1000);
    bench_magnitude_push_front!(bench_push_front_byte_10000, bench_push_front_string_10000, 10000);


    bench_magnitude_insert!(bench_insert_byte_1_1, bench_insert_string_1_1, 1, 1);
    bench_magnitude_insert!(bench_insert_byte_1_10, bench_insert_string_1_10, 1, 10);
    bench_magnitude_insert!(bench_insert_byte_1_100, bench_insert_string_1_100, 1, 100);
    bench_magnitude_insert!(bench_insert_byte_1_1000, bench_insert_string_1_1000, 1, 1000);
    bench_magnitude_insert!(bench_insert_byte_1_10000, bench_insert_string_1_10000, 1, 10000);

    bench_magnitude_insert!(bench_insert_byte_10_1, bench_insert_string_10_1, 10, 1);
    bench_magnitude_insert!(bench_insert_byte_10_10, bench_insert_string_10_10, 10, 10);
    bench_magnitude_insert!(bench_insert_byte_10_100, bench_insert_string_10_100, 10, 100);
    bench_magnitude_insert!(bench_insert_byte_10_1000, bench_insert_string_10_1000, 10, 1000);
    bench_magnitude_insert!(bench_insert_byte_10_10000, bench_insert_string_10_10000, 10, 10000);

    bench_magnitude_insert!(bench_insert_byte_100_1, bench_insert_string_100_1, 100, 1);
    bench_magnitude_insert!(bench_insert_byte_100_10, bench_insert_string_100_10, 100, 10);
    bench_magnitude_insert!(bench_insert_byte_100_100, bench_insert_string_100_100, 100, 100);
    bench_magnitude_insert!(bench_insert_byte_100_1000, bench_insert_string_100_1000, 100, 1000);
    bench_magnitude_insert!(bench_insert_byte_100_10000, bench_insert_string_100_10000, 100, 10000);

    bench_magnitude_insert!(bench_insert_byte_1000_1, bench_insert_string_1000_1, 1000, 1);
    bench_magnitude_insert!(bench_insert_byte_1000_10, bench_insert_string_1000_10, 1000, 10);
    bench_magnitude_insert!(bench_insert_byte_1000_100, bench_insert_string_1000_100, 1000, 100);
    bench_magnitude_insert!(bench_insert_byte_1000_1000, bench_insert_string_1000_1000, 1000, 1000);
    bench_magnitude_insert!(bench_insert_byte_1000_10000, bench_insert_string_1000_10000, 1000, 10000);

    bench_magnitude_insert!(bench_insert_byte_10000_1, bench_insert_string_10000_1, 10000, 1);
    bench_magnitude_insert!(bench_insert_byte_10000_10, bench_insert_string_10000_10, 10000, 10);
    bench_magnitude_insert!(bench_insert_byte_10000_100, bench_insert_string_10000_100, 10000, 100);
    bench_magnitude_insert!(bench_insert_byte_10000_1000, bench_insert_string_10000_1000, 10000, 1000);
    bench_magnitude_insert!(bench_insert_byte_10000_10000, bench_insert_string_10000_10000, 10000, 10000);
    bench_magnitude_insert!(aabench_insert_byte_100000000_100000000, aabench_insert_string_1000000000_100000000, 100000000, 100000000);
}
