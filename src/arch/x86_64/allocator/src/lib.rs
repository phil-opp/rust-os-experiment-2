#![feature(no_std, core)]
#![no_std]

#![allow(unused_variables)]

extern crate core;

pub unsafe fn allocate(size: usize, align: usize) -> *mut u8 {
    loop{}
}

pub unsafe fn deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    loop{}
}

pub unsafe fn reallocate(ptr: *mut u8, old_size: usize, size: usize, align: usize) -> *mut u8 {
    loop{}
}

pub unsafe fn reallocate_inplace(ptr: *mut u8, old_size: usize, size: usize,
                                 align: usize) -> usize {
    loop{}
}

pub fn usable_size(size: usize, align: usize) -> usize {
    loop{}
}

pub fn stats_print() {
    loop{}
}