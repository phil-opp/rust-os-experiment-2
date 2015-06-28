pub mod stdio;
pub mod keyboard;
mod ps2;

pub unsafe fn init() {
    stdio::init();
    ps2::init();
}
