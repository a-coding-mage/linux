// SPDX-License-Identifier: GPL-2.0
//! Freestanding x86-64 compressed-kernel Rust owners.
#![no_std]
#![feature(c_variadic)]

#[path = "boot_bindings.rs"]
mod bindings;
#[path = "cmdline.rs"]
mod boot_cmdline;
#[cfg(CONFIG_X86_64)]
mod boot_idt_bindings;
#[path = "../string.rs"]
mod boot_string;
#[path = "../../../../lib/cmdline.rs"]
pub mod cmdline;
#[path = "boot_cpu_bindings.rs"]
mod cpu_bindings;
mod cpuflags;
#[path = "../../../../lib/ctype.rs"]
pub mod ctype;
#[path = "../../../../lib/decompress_inflate.rs"]
mod decompress_inflate;
#[cfg(CONFIG_EARLY_PRINTK)]
mod early_serial_console;
mod error;
#[cfg(CONFIG_X86_64)]
mod idt_64;
#[cfg(CONFIG_RANDOMIZE_BASE)]
mod kaslr;
mod misc;
mod string;
#[path = "../../../../lib/zlib_inflate/inffast.rs"]
mod zlib_inffast;
#[path = "../../../../lib/zlib_inflate/inflate.rs"]
mod zlib_inflate;
#[path = "../../../../lib/zlib_inflate/inftrees.rs"]
mod zlib_inftrees;
#[path = "../../../../lib/zlib_inflate/infutil.rs"]
mod zlib_infutil;

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    // SAFETY: a Rust contract failure must never unwind into early boot.
    unsafe { error::error(c"Rust compressed-kernel panic".as_ptr().cast_mut()) }
}
