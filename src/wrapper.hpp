#include <immer/flex_vector.hpp>
#include <immer/flex_vector_transient.hpp>
#include <iostream>

typedef unsigned long long          usize;

typedef immer::flex_vector <char>   Buffer;

typedef struct buffer_bytes_iter {
    Buffer *buffer;
    usize   size;
    usize   index;
} Buffer_bytes_iter;

typedef struct buffer_bytes_riter {
    Buffer *buffer;
    usize   index;
} Buffer_bytes_riter;

extern "C" {
void buffer_delete(Buffer *buffer);
Buffer *buffer_new();
Buffer *buffer_clone(Buffer *buf);
Buffer *buffer_push_front_byte(Buffer *buffer, char elem);
Buffer *buffer_push_back_byte(Buffer *buffer, char elem);
Buffer *buffer_push_front_string(Buffer *buffer, char *elem);
Buffer *buffer_push_back_string(Buffer *buffer, char *elem);
int buffer_equals(Buffer *left, Buffer *right);
Buffer *buffer_concat(Buffer *left, Buffer *right);
Buffer *buffer_first_n(Buffer *buffer, usize n);
Buffer *buffer_last_n(Buffer *buffer, usize n);
Buffer *buffer_insert_byte(Buffer *buffer, usize index, char elem);
Buffer *buffer_insert_string(Buffer *buffer, usize index, char *elem);
Buffer *buffer_delete_byte(Buffer *buffer, usize index);
Buffer *buffer_delete_range(Buffer *buffer, usize start, usize end);
Buffer *buffer_set(Buffer *buffer, usize index, char elem);
char buffer_get(Buffer *buffer, usize index);
usize buffer_get_size(Buffer *buffer);

void buffer_iter_delete(Buffer_bytes_iter *iter);
Buffer_bytes_iter *buffer_get_iter(Buffer *buffer);
int buffer_iter_has_next(Buffer_bytes_iter *iter);
char buffer_iter_next(Buffer_bytes_iter *iter);

void buffer_riter_delete(Buffer_bytes_riter *iter);
Buffer_bytes_riter *buffer_get_riter(Buffer *buffer);
int buffer_riter_has_next(Buffer_bytes_riter *iter);
char buffer_riter_next(Buffer_bytes_riter *iter);
}
void buffer_delete(Buffer *buffer) {
    if (buffer == NULL) {
        return;
    }

    delete buffer;
}



Buffer *buffer_new() {
    return new Buffer;
}



Buffer *buffer_push_front_byte(Buffer *buffer, char elem) {
    return new Buffer(buffer->push_front(elem));
}



Buffer *buffer_push_back_byte(Buffer *buffer, char elem) {
    return new Buffer(buffer->push_back(elem));
}



Buffer *buffer_push_back_string(Buffer *buffer, char *str) {
    auto trans = buffer->transient();

    while (*str != '\0') {
        trans.push_back(*str);
        str++;
    }

    return new Buffer(trans.persistent());
}



Buffer *buffer_push_front_string(Buffer *buffer, char *str) {
    auto trans = immer::flex_vector <char>().transient();

    while (*str != '\0') {
        trans.push_back(*str);
        ++str;
    }

    return new Buffer(trans.persistent() + *buffer);
}



int buffer_equals(Buffer *left, Buffer *right) {
    return *left == *right;
}



Buffer *buffer_clone(Buffer *buffer) {
    return new Buffer(*buffer);
}



char buffer_get(Buffer *buffer, usize index) {
    return buffer->at(index);
}



Buffer *buffer_set(Buffer *buffer, usize index, char elem) {
    return new Buffer(buffer->set(index, elem));
}



Buffer *buffer_concat(Buffer *left, Buffer *right) {
    return new Buffer(*left + *right);
}



Buffer *buffer_last_n(Buffer *buffer, usize index) {
    return new Buffer(buffer->drop(index));
}



Buffer *buffer_first_n(Buffer *buffer, usize index) {
    return new Buffer(buffer->take(index));
}



Buffer *buffer_insert_byte(Buffer *buffer, usize index, char elem) {
    return new Buffer(buffer->insert(index, elem));
}



Buffer *buffer_insert_string(Buffer *buffer, usize index, char *str) {
    auto trans = buffer->take(index).transient();

    while (*str != '\0') {
        trans.push_back(*str);
        ++str;
    }

    return new Buffer(trans.persistent() + buffer->drop(index));
}



Buffer *buffer_delete_byte(Buffer *buffer, usize index) {
    return new Buffer(buffer->erase(index));
}



Buffer *buffer_delete_range(Buffer *buffer, usize start, usize end) {
    return new Buffer(buffer->take(start) + buffer->drop(end));
}



usize buffer_get_size(Buffer *buffer) {
    return buffer->size();
}



void buffer_iter_delete(Buffer_bytes_iter *iter) {
    if (iter == NULL) {
        return;
    }

    delete iter;
}



Buffer_bytes_iter *buffer_get_iter(Buffer *buffer) {
    Buffer_bytes_iter *iter = new Buffer_bytes_iter;

    *iter = {
        .buffer = buffer_clone(buffer),
        .size   = buffer_get_size(buffer),
        .index  = 0,
    };

    return iter;
}



int buffer_iter_has_next(Buffer_bytes_iter *iter) {
    int res = iter->size > iter->index;

    return res;
}



char buffer_iter_next(Buffer_bytes_iter *iter) {
    int has_next = buffer_iter_has_next(iter);

    if (!has_next) {
        return 0;
    }

    char ret = buffer_get(iter->buffer, iter->index++);

    return ret;
}



void buffer_riter_delete(Buffer_bytes_riter *iter) {
    if (iter == NULL) {
        return;
    }

    delete iter;
}



Buffer_bytes_riter *buffer_get_riter(Buffer *buffer) {
    Buffer_bytes_riter *iter = new Buffer_bytes_riter;

    *iter = {
        .buffer = buffer_clone(buffer),
        .index  = buffer_get_size(buffer),
    };

    return iter;
}



int buffer_riter_has_next(Buffer_bytes_riter *iter) {
    int res = iter->index > 0;

    return res;
}



char buffer_riter_next(Buffer_bytes_riter *iter) {
    int has_next = buffer_riter_has_next(iter);

    if (!has_next) {
        return 0;
    }

    char ret = buffer_get(iter->buffer, --iter->index);

    return ret;
}
