pub use self::multiboot::MultibootHeader;

use io;
use allocator;
use thread;

mod multiboot;

pub fn init(multiboot: MultibootHeader) {
    io::init();
    allocator::init(multiboot);
    thread::init();
}
