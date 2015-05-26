use std::intrinsics::offset;

pub struct MultibootHeader(*const Information);

impl MultibootHeader {
    pub fn memory_areas(&self) -> Option<MemoryAreaIter> {
        unsafe{(*self.0).memory_areas()}
    }
}

#[repr(packed)]
#[allow(dead_code)]
struct Information {
    total_size: u32,
    reserved: u32,
    //tags
}

impl Information {

    fn memory_map_tag(&self) -> Option<*const MemoryAreaTag> {
        let self_ptr = self as *const Information;
        let mut tag = unsafe{offset(self_ptr, 1) as *const Tag};
        let tag_end_ptr = ((self_ptr as u32) + self.total_size) as *const Tag;

        //iterate over tags
        while tag < tag_end_ptr {
            match unsafe{&*tag} {
                &Tag{typ:0, size:8} => {break;}, //end tag
                &Tag{typ:6, ..} => {
                    //Memory Map Tag
                    return Some(tag as *const MemoryAreaTag);
                },
                ref t => {
                    let mut tag_addr = tag as u32;
                    tag_addr += t.size;
                    tag_addr = ((tag_addr-1) & 0xfffffff8) + 0x8; //align at 8 byte
                    tag = tag_addr as *const Tag;
                },
            }
        }
        None
    }

    pub fn memory_areas(&self) -> Option<MemoryAreaIter> {
        self.memory_map_tag().map(|tag| (unsafe{&*tag}).areas())
    }

}

#[repr(packed)]
#[allow(dead_code)]
struct Tag {
    typ: u32,
    size: u32,
}

#[repr(packed)]
#[allow(dead_code)]
struct MemoryAreaTag {
    typ: u32,
    size: u32,
    entry_size: u32,
    entry_version: u32,
    first_area: MemoryArea,
}

#[repr(packed)]
#[allow(dead_code)]
pub struct MemoryArea {
    pub base_addr: u64,
    pub length: u64,
    typ: u32,
    reserved: u32,
}

#[allow(raw_pointer_derive)]
#[derive(Clone)]
pub struct MemoryAreaIter {
    current_area: *const MemoryArea,
    last_area: *const MemoryArea,
    entry_size: u32,
}


impl MemoryAreaTag {
    fn areas(&self) -> MemoryAreaIter {
        let self_ptr = self as *const MemoryAreaTag;
        let start_area = (&self.first_area) as *const MemoryArea;
        MemoryAreaIter {
            current_area: start_area,
            last_area: ((self_ptr as u32) + self.size - self.entry_size) as *const MemoryArea,
            entry_size: self.entry_size,
        }
    }
}


impl Iterator for MemoryAreaIter {
    type Item = &'static MemoryArea;
    fn next(&mut self) -> Option<&'static MemoryArea> {
        if self.current_area > self.last_area {
            None
        } else {
            let area = unsafe{&*self.current_area};
            self.current_area = ((self.current_area as u32) + self.entry_size)
                as *const MemoryArea;
            if area.typ == 1 {
                Some(area)
            } else {self.next()}
        }
    }
}
