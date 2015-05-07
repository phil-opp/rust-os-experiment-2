pub use self::multiboot::MultibootHeader;

use io;
use allocator;

mod multiboot;

pub fn init(multiboot: MultibootHeader) {
    io::init();
    allocator::init(multiboot);
}