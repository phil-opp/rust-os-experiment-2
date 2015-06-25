pub mod stdio;
pub mod keyboard;

pub unsafe fn init() {
    stdio::init();
}
