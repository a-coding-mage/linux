// SPDX-License-Identifier: GPL-2.0
//! Initial x86-64 descriptor tables and page-table relocation.
#![no_std]

mod startup_gdt_bindings;
mod gdt_idt;
mod startup_map_bindings;
mod map_kernel;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    // Startup cannot unwind or recover from a violated Rust contract.
    loop {
        core::hint::spin_loop();
    }
}
