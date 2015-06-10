pub use self::multiboot::MultibootHeader;

use io;
use allocator;
use global;
use thread;

mod multiboot;

pub unsafe fn init(multiboot: MultibootHeader) {
    allocator::init(multiboot);
    global::init();
    thread::init();
    io::init();
}
