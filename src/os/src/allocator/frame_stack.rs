// TODO rewrite

use std::intrinsics::{offset, size_of};
use std::cmp::Ordering;
use super::paging::PAGE_SIZE;

extern {
    static kernel_end_symbol_table_entry: ();
}

#[derive(Clone, Copy)]
pub struct Frame {
    pub number: u32,
}

impl PartialEq for Frame {
    fn eq(&self, other: &Frame) -> bool {
        self.number.eq(&other.number)
    }
}
impl PartialOrd for Frame {
    fn partial_cmp(&self, other: &Frame) -> Option<Ordering> {
        Some(self.number.cmp(&other.number))
    }
}

const STACK_POINTER : *mut FrameStack = 0o_001_000_000_0000 as *mut FrameStack; //10MB

struct FrameStack {
    first: *const Frame,
    length: u32, //enough for up to 16TB memory
}

pub fn init(multiboot: ::MultibootHeader) {

    let kernel_end = &kernel_end_symbol_table_entry as *const () as u32;
    let stack_start_frame = Frame{number: kernel_end >> 12};

    // map frame stack to 2mb behind kernel
    unsafe{ map_p1_entries(0, 0, stack_start_frame) };
    // map p1 entries
    for i in 0..512 { unsafe {
        map_p1_entries(0, i, Frame{
            number: stack_start_frame.number + i,
        });
    }}

    unsafe {
        (*STACK_POINTER) = FrameStack {
            first: offset(STACK_POINTER as *const FrameStack, 1) as *const Frame, 
            length: 0,
        };
    }

    let areas = multiboot.memory_areas().unwrap();    
    let number_of_frames = areas.clone().fold(0, |n, area| {n + area.length / PAGE_SIZE}) as u32;

    // for memory > 1GB we need more p1 tables
    let p1_tables_required = (unsafe{max_size(number_of_frames)} / 4096 / 512) as u32;
    let mut last_mapped_p1_table = 0;

    // the first free physical frame (the 512 frames after kernel are used)
    let first_free_frame = Frame{
        number: stack_start_frame.number + 512,
    };

    for area in areas {
        for frame in ((area.base_addr >> 12)..((area.base_addr+area.length) >> 12)).map(
            |frame_number| Frame{number: frame_number as u32}) {
            if frame >= first_free_frame {
                if last_mapped_p1_table < p1_tables_required {
                    // add as p1 table
                    last_mapped_p1_table += 1;
                    unsafe{ map_to_p1_table(last_mapped_p1_table, frame) };
                    // map p1 entries
                    for i in 0..512 { unsafe {
                        map_p1_entries(last_mapped_p1_table, i, Frame{
                            number: stack_start_frame.number + last_mapped_p1_table*512 + i,
                        });
                    }}
                } else {
                    // add as free
                    deallocate_frame(frame);
                }
            }
        }
    }

    unsafe fn map_to_p1_table(p2_index: u32, frame: Frame) {
        let p1_field: *mut u64 = (0o177777_777_777_000_001_0000 + (p2_index as u64) * 8) 
            as *mut u64;
        *p1_field = ((frame.number as u64) << 12) | 1;
    }
    
    unsafe fn map_p1_entries(p2_index: u32, p1_index: u32, to: Frame) {
        let entry: *mut u64 = (0o177777_777_000_001_000_0000 | ((p2_index as u64) << 12) 
            | (p1_index as u64 * 8)) as *mut u64;
        *entry = ((to.number as u64) << 12) | 3;
    }
}

#[allow(dead_code)]
pub fn length() -> u32 {
    unsafe{(*STACK_POINTER).length}
}

/// returns maximal size of frame stack for given number of frames
#[allow(dead_code)]
pub unsafe fn max_size(number_of_frames: u32) -> u64 {
    size_of::<FrameStack>() as u64 + number_of_frames as u64 * size_of::<Frame>() as u64
}

pub fn allocate_frame() -> Option<Frame> {
    unsafe{(*STACK_POINTER).pop()}
}

#[allow(dead_code)]
pub fn deallocate_frame(frame: Frame) {
    unsafe{(*STACK_POINTER).push(frame)}
}

impl FrameStack {
    unsafe fn push(&mut self, frame: Frame) {
        let last = offset(self.first, self.length as isize);
        *(last as *mut Frame) = frame;
        self.length += 1;
    }

    fn pop(&mut self) -> Option<Frame> {
        if self.length == 0 {
            None
        } else {
            self.length -= 1;
            Some(unsafe{*offset(self.first, self.length as isize)})
        }
    }
}