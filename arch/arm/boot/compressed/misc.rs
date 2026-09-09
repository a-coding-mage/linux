// SPDX-License-Identifier: GPL-2.0
/*
 * misc.c
 *
 * This is a collection of several routines from gzip-1.0.3
 * adapted for Linux.
 *
 * malloc by Hannu Savolainen 1993 and Matthias Urlichs 1994
 *
 * Modified for ARM Linux by Russell King
 *
 * Nicolas Pitre <nico@visuaide.com>  1999/04/14 :
 *  For this code to run directly from Flash, all constant variables must
 *  be marked with 'const' and all other variables initialized at run-time
 *  only.  This way all non constant variables will end up in the bss segment,
 *  which should point to addresses in RAM and cleared to 0 on start.
 *  This allows for a much quicker boot time.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub static mut __machine_arch_type: c_uint = 0;

// Declarations supplied by the included Linux and architecture headers.
unsafe extern "C" {
    fn flush();
    #[cfg(not(feature = "CONFIG_DEBUG_ICEDCC"))]
    fn putc(ch: c_int);
    fn arch_error(x: *mut c_char);
    fn arch_decomp_setup();
    fn do_decompress(
        input_data: *const u8,
        input_len: c_ulong,
        output_data: *mut u8,
        error: unsafe extern "C" fn(*mut c_char),
    ) -> c_int;
    static input_data: *const u8;
    static input_data_end: *const u8;
    #[cfg(feature = "CONFIG_ARCH_EP93XX")]
    fn ep93xx_decomp_setup();
}

// CONFIG_DEBUG_ICEDCC selects the debug-console implementation at build time.
#[cfg(feature = "CONFIG_DEBUG_ICEDCC")]
#[cfg(any(
    feature = "CONFIG_CPU_V6",
    feature = "CONFIG_CPU_V6K",
    feature = "CONFIG_CPU_V7"
))]
unsafe fn icedcc_putc(mut ch: c_int) {
    let mut status: c_int;
    let mut i: c_int = 0x4000000;
    loop {
        i = i.wrapping_sub(1);
        if i < 0 {
            return;
        }
        core::arch::asm!("mrc p14, 0, {status}, c0, c1, 0", status = out(reg) status);
        if status & (1 << 29) == 0 {
            break;
        }
    }
    core::arch::asm!("mcr p14, 0, {ch}, c0, c5, 0", ch = in(reg) ch);
}

#[cfg(feature = "CONFIG_DEBUG_ICEDCC")]
#[cfg(feature = "CONFIG_CPU_XSCALE")]
unsafe fn icedcc_putc(mut ch: c_int) {
    let mut status: c_int;
    let mut i: c_int = 0x4000000;
    loop {
        i = i.wrapping_sub(1);
        if i < 0 { return; }
        core::arch::asm!("mrc p14, 0, {status}, c14, c0, 0", status = out(reg) status);
        if status & (1 << 28) == 0 { break; }
    }
    core::arch::asm!("mcr p14, 0, {ch}, c8, c0, 0", ch = in(reg) ch);
}

#[cfg(feature = "CONFIG_DEBUG_ICEDCC")]
#[cfg(not(any(
    feature = "CONFIG_CPU_V6", feature = "CONFIG_CPU_V6K", feature = "CONFIG_CPU_V7",
    feature = "CONFIG_CPU_XSCALE"
)))]
unsafe fn icedcc_putc(mut ch: c_int) {
    let mut status: c_int;
    let mut i: c_int = 0x4000000;
    loop {
        i = i.wrapping_sub(1);
        if i < 0 { return; }
        core::arch::asm!("mrc p14, 0, {status}, c0, c0, 0", status = out(reg) status);
        if status & 2 == 0 { break; }
    }
    core::arch::asm!("mcr p14, 0, {ch}, c1, c0, 0", ch = in(reg) ch);
}

#[cfg(feature = "CONFIG_DEBUG_ICEDCC")]
unsafe fn putc(ch: c_int) { icedcc_putc(ch); }

static mut output_data: *mut u8 = core::ptr::null_mut();
static mut free_mem_ptr: c_ulong = 0;
static mut free_mem_end_ptr: c_ulong = 0;

pub unsafe extern "C" fn error(x: *mut c_char) -> ! {
    arch_error(x);
    putstr(b"\n\n\0".as_ptr() as *const c_char);
    putstr(x);
    putstr(b"\n\n -- System halted\0".as_ptr() as *const c_char);
    loop {}
}

unsafe fn putstr(ptr: *const c_char) {
    let mut ptr = ptr;
    loop {
        let c = *ptr;
        ptr = ptr.add(1);
        if c == 0 { break; }
        if c == b'\n' as c_char { putc(b'\r' as c_int); }
        putc(c as c_int);
    }
    flush();
}

pub unsafe extern "C" fn __div0() -> ! {
    error(b"Attempting division by 0!\0".as_ptr() as *mut c_char);
}

pub unsafe extern "C" fn decompress_kernel(
    output_start: c_ulong,
    free_mem_ptr_p: c_ulong,
    free_mem_ptr_end_p: c_ulong,
    arch_id: c_int,
) {
    output_data = output_start as *mut u8;
    free_mem_ptr = free_mem_ptr_p;
    free_mem_end_ptr = free_mem_ptr_end_p;
    __machine_arch_type = arch_id as c_uint;

    #[cfg(feature = "CONFIG_ARCH_EP93XX")]
    ep93xx_decomp_setup();
    arch_decomp_setup();

    putstr(b"Uncompressing Linux...\0".as_ptr() as *const c_char);
    let ret = do_decompress(input_data, input_data_end.offset_from(input_data) as c_ulong,
                            output_data, error);
    if ret != 0 {
        error(b"decompressor returned an error\0".as_ptr() as *mut c_char);
    } else {
        putstr(b" done, booting the kernel.\n\0".as_ptr() as *const c_char);
    }
}

pub unsafe extern "C" fn __fortify_panic(_reason: u8, _avail: usize, _size: usize) -> ! {
    error(b"detected buffer overflow\0".as_ptr() as *mut c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
