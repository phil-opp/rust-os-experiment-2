#![allow(dead_code)]

use super::{VirtualAddress};
use super::frame_stack::{Frame, allocate_frame};
use std::ops::Deref;

pub const PAGE_SIZE: u64 = 4096;

// first allocated address starts on second P4-Page
pub const FIRST_PAGE : Page = Page {
    number: 0o_001_000_000_000,
};

#[derive(Clone, Copy, PartialEq)]
pub struct Page {
   pub number: usize,
}

// a page containing a page table
struct PageTablePage(Page);

struct PageIter(Page);

struct PageTableField(*const u64);

bitflags! {
    flags PageTableFieldFlags: u64 {
        const NOT_FREE = 1 << 11,
        const PRESENT =         NOT_FREE.bits | 1 << 0,
        const WRITABLE =        NOT_FREE.bits | 1 << 1,
        const USER_ACCESSIBLE = NOT_FREE.bits | 1 << 2,
        const WRITE_THROUGH =   NOT_FREE.bits | 1 << 3,
        const NO_CACHE =        NOT_FREE.bits | 1 << 4,
        const ACCESSED =        NOT_FREE.bits | 1 << 5,
        const DIRTY =           NOT_FREE.bits | 1 << 6,
        const OTHER1 =          NOT_FREE.bits | 1 << 9,
        const OTHER2 =          NOT_FREE.bits | 1 << 10,
        const NO_EXECUTE =      NOT_FREE.bits | 1 << 63,
    }
}


impl Page {
    fn from_address(address: &VirtualAddress) -> Page {
        Page {
            number: address.0 as usize >> 12,
        }
    }

    pub fn start_address(&self) -> VirtualAddress {
        if self.number >= 0o400_000_000_000 {
            //sign extension
            VirtualAddress(((self.number << 12) | 0o177777_000_000_000_000_0000) as *const u8)
        } else {
            VirtualAddress((self.number << 12) as *const u8)
        }
    }

    fn p4_index(&self) -> usize {(self.number >> 27) & 0o777}
    fn p3_index(&self) -> usize {(self.number >> 18) & 0o777}
    fn p2_index(&self) -> usize {(self.number >> 9) & 0o777}
    fn p1_index(&self) -> usize {(self.number >> 0) & 0o777}

    fn p4_page(&self) -> PageTablePage {
        PageTablePage(Page {
            number: 0o_777_777_777_777,
        })
    }
    fn p3_page(&self) -> PageTablePage {
        PageTablePage(Page {
            number: 0o_777_777_777_000 | self.p4_index(),
        })
    }
    fn p2_page(&self) -> PageTablePage {
        PageTablePage(Page {
            number: 0o_777_777_000_000 | (self.p4_index() << 9) | self.p3_index(),
        })
    }
    fn p1_page(&self) -> PageTablePage {
        PageTablePage(Page {
            number: 0o_777_000_000_000 | (self.p4_index() << 18) | (self.p3_index() << 9)
                | self.p2_index(),
        })
    }

    pub unsafe fn map_to_new_frame(&self) {
        let p4_field = self.p4_page().field(self.p4_index());
        if p4_field.is_free() {
            p4_field.set(allocate_frame().expect("no frame allocated"), PRESENT | WRITABLE);
            self.p3_page().zero();
        }
        let p3_field = self.p3_page().field(self.p3_index());
        if p3_field.is_free() {
            p3_field.set(allocate_frame().expect("no frame allocated"), PRESENT | WRITABLE);
            self.p2_page().zero();
        }
        let p2_field = self.p2_page().field(self.p2_index());
        if p2_field.is_free() {
            p2_field.set(allocate_frame().expect("no frame allocated"), PRESENT | WRITABLE);
            self.p1_page().zero();
        }
        let p1_field = self.p1_page().field(self.p1_index());
        assert!(p1_field.is_free());
        p1_field.set(allocate_frame().expect("no frame allocated"), PRESENT | WRITABLE);
    }

    unsafe fn zero(&self) {
        let page = self.start_address().0 as *mut [u64; (PAGE_SIZE/64) as usize];
        *page = [0; (PAGE_SIZE/64) as usize];
    }

    pub fn next_pages(self) -> PageIter {
        PageIter(self)
    }
}

impl Iterator for PageIter {
    type Item = Page;

    fn next(&mut self) -> Option<Page> {
        self.0.number += 1;
        Some(self.0)
    }
}

impl PageTablePage {
    unsafe fn field(&self, index: usize) -> PageTableField {
        //print!("index: {} pointer: {:o}\n", index, self.0.start_address().0 as usize + (index * 8));
        PageTableField((self.0.start_address().0 as usize + (index * 8)) as *const u64)
    }
}

impl Deref for PageTablePage {
    type Target = Page;
    fn deref(&self) -> &Page { &self.0 }
}

impl VirtualAddress {
    pub fn page(&self) -> Page {
        Page::from_address(self)
    }

    fn page_offset(&self) -> u32 {
        self.0 as u32 & 0xfff
    }
}

impl PageTableField {

    unsafe fn is(&self, flags: PageTableFieldFlags) -> bool {
        
        //print!("{:o}\n", self.0 as usize);
        PageTableFieldFlags::from_bits_truncate(*(self.0)).contains(flags)
    }

    unsafe fn add_flag(&self, flags: PageTableFieldFlags) {
        *(self.0 as *mut u64) |= flags.bits;
    }

    unsafe fn remove_flag(&self, flags: PageTableFieldFlags) {
        *(self.0 as *mut u64) &= !flags.bits;
    }

    unsafe fn is_free(&self) -> bool {
        !self.is(NOT_FREE)
    }

    unsafe fn pointed_frame(&self) -> Frame {
        Frame {
            number: (((*self.0) & 0x000fffff_fffff000) >> 12) as u32,
        }
    }

    unsafe fn set(&self, frame: Frame, flags: PageTableFieldFlags) {
        let f = self.0 as *mut u64;
        *f = (((frame.number as u64) << 12) & 0x000fffff_fffff000) | flags.bits();
    }
}
