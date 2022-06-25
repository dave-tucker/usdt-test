#![no_std]
#![no_main]

use aya_bpf::{macros::usdt, programs::UsdtContext};
use aya_log_ebpf::info;

#[usdt]
pub fn usdt(ctx: UsdtContext) -> u32 {
    match unsafe { try_usdt(ctx) } {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

unsafe fn try_usdt(ctx: UsdtContext) -> Result<u32, u32> {
    let tick = ctx.arg(0).unwrap_or_default();
    let elapsed = ctx.arg(1).unwrap_or_default();
    info!(&ctx, "clock: tick {}, elapsed: {}", tick, elapsed);
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
