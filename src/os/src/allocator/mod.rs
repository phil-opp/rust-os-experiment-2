//TODO rewrite

use self::paging::{Page, FIRST_PAGE};

mod paging;
mod frame_stack;

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy)]
struct PhysicalAddress(*const u8);

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy)]
struct VirtualAddress(*const u8);

struct Allocator {
    current_page: Page,
    next_byte: VirtualAddress,
}

static mut allocator: Option<Allocator> = None;

#[no_mangle]
pub unsafe extern fn rust_allocate(size: usize, align: usize) -> *mut u8 {
    if allocator.is_none() {
        FIRST_PAGE.map_to_new_frame();
        allocator = Some(Allocator {
            next_byte: FIRST_PAGE.start_address(),
            current_page: FIRST_PAGE,
        });
    }
    allocator.as_mut().expect("allocator must be initialized")
        .allocate(size, align).0 as *mut u8
}

#[no_mangle]
pub extern fn rust_deallocate(_ptr: *mut u8, _old_size: usize, _align: usize){
    //print!("start: {:x}, size: {:x}, align: {:x}\n", _ptr as usize, _old_size,
    // _align);
}

#[no_mangle]
pub unsafe extern fn rust_reallocate(old_ptr: *mut u8, old_size: usize, size: usize,
    align: usize) -> *mut u8
{
    let new_ptr = rust_allocate(size, align);
    for i in 0..(old_size as isize) {
        *new_ptr.offset(i) = *old_ptr.offset(i);
    }
    new_ptr
}

#[no_mangle]
pub extern fn rust_reallocate_inplace(_ptr: *mut u8, _old_size: usize, _size: usize, 
    _align: usize) -> usize
{
    0
}

#[no_mangle]
pub extern fn rust_usable_size(size: usize, _align: usize) -> usize {
    size
}

#[no_mangle]
pub extern fn rust_stats_print(){
    unimplemented!();
}

pub fn init(multiboot: ::MultibootHeader) {
    frame_stack::init(multiboot);
}

impl Allocator {
    unsafe fn allocate(&mut self, size: usize, align: usize) -> VirtualAddress {
        let addr = self.next_byte.0 as usize;

        //align
        if align > 0 && addr % align != 0 {
            self.next_byte = VirtualAddress((addr + align - (addr % align)) as *const u8);
        }

        //map unmapped pages if allocation is on new pages
        let end_page = VirtualAddress((addr + size - 1) as *const u8).page();
        for page in self.current_page.next_pages().take(
                end_page.number - self.current_page.number) {
            page.map_to_new_frame();
        }

        //allocate
        let start = self.next_byte;
        self.next_byte = VirtualAddress((addr + size) as *const u8);
        self.current_page = end_page;
       
        // DEBUGGING: zero allocated bytes
        for i in (0..size) {
            *((start.0 as usize + i) as *mut u8) = 0;
        }

        start
    }
}
