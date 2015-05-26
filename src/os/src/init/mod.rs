pub use self::multiboot::MultibootHeader;

use io;
use allocator;
use thread;

mod multiboot;

pub unsafe fn init(multiboot: MultibootHeader) {
    allocator::init(multiboot);
    thread::init();
    io::init();
}
