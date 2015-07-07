pub mod stdio;
pub mod keyboard;
mod ps2;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"] pub mod arch;

pub unsafe fn init() {
    stdio::init();
    ps2::init();
}
