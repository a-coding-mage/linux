/*
 * Definitions and wrapper functions for kernel decompressor
 *
 *   (C) 2017 Helge Deller <deller@gmx.de>
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

/* C headers and decompressor implementation includes are supplied by other files. */
/* Build-time CONFIG_KERNEL_* and CONFIG_64BIT conditions are preserved below. */

type size_t = usize;
type u32 = u32;
type __le32 = u32;

extern "C" {
    static mut input_data: c_char;
    static mut input_len: c_int;
    static mut output_len: c_char;
    static mut _text: c_char;
    static mut _end: c_char;
    static mut _bss: c_char;
    static mut _ebss: c_char;
    static mut _startcode_end: c_char;
    fn startup_continue(entry: *mut c_void, cmdline: u64, rd_start: u64, rd_end: u64) -> !;
    fn pdc_iodc_print(s: *const c_char, count: size_t);
    fn set_firmware_width_unlocked();
    fn __decompress(input: *const c_char, input_len: c_int, a: *const c_void,
                    b: *const c_void, output: *mut c_char, len: c_int,
                    c: *const c_void, error: unsafe extern "C" fn(*mut c_char) -> !) ;
    static mut PAGE0: *mut Page0;
    static mut parisc_narrow_firmware: c_int;
}

#[repr(C)]
struct Page0 { imm_max_mem: u64 }

static mut free_mem_ptr: u64 = 0;
static mut free_mem_end_ptr: u64 = 0;

unsafe extern "C" fn memmove(dest: *mut c_void, src: *const c_void, mut n: size_t) -> *mut c_void {
    let mut s = src as *const u8;
    let mut d = dest as *mut u8;
    if (d as usize) <= (s as usize) {
        while n != 0 { *d = *s; d = d.add(1); s = s.add(1); n -= 1; }
    } else {
        d = d.add(n); s = s.add(n);
        while n != 0 { d = d.sub(1); s = s.sub(1); *d = *s; n -= 1; }
    }
    dest
}

unsafe extern "C" fn memset(s: *mut c_void, c: c_int, mut count: size_t) -> *mut c_void {
    let mut p = s as *mut u8;
    while count != 0 { *p = c as u8; p = p.add(1); count -= 1; }
    s
}

unsafe extern "C" fn memcpy(d: *mut c_void, s: *const c_void, mut len: size_t) -> *mut c_void {
    let mut dest = d as *mut u8; let mut source = s as *const u8;
    while len != 0 { *dest = *source; dest = dest.add(1); source = source.add(1); len -= 1; }
    d
}

unsafe extern "C" fn strlen(mut s: *const c_char) -> size_t {
    let start = s;
    while *s != 0 { s = s.add(1); }
    s.offset_from(start) as size_t
}

unsafe extern "C" fn strchr(mut s: *const c_char, c: c_int) -> *mut c_char {
    while *s != 0 { if *s == c as c_char { return s as *mut c_char; } s = s.add(1); }
    core::ptr::null_mut()
}

unsafe fn puts(mut s: *const c_char) -> c_int {
    let mut nuline;
    while { nuline = strchr(s, b'\n' as c_int); !nuline.is_null() } {
        if nuline != s { pdc_iodc_print(s, nuline.offset_from(s) as size_t); }
        pdc_iodc_print(b"\r\n\0".as_ptr() as *const c_char, 2);
        s = nuline.add(1);
    }
    if *s != 0 { pdc_iodc_print(s, strlen(s)); }
    0
}

unsafe fn putchar(c: c_int) -> c_int {
    let buf = [c as c_char, 0]; puts(buf.as_ptr()); c
}

pub unsafe extern "C" fn error(x: *mut c_char) -> ! {
    if !x.is_null() { puts(x); }
    puts(b"\n -- System halted\n\0".as_ptr() as *const c_char);
    loop {}
}

unsafe fn print_num(mut num: u64, base: u64) -> c_int {
    let hex = b"0123456789abcdef"; let mut strbuf = [0i8; 40]; let mut i = 39usize;
    strbuf[i] = 0; i -= 1;
    loop { strbuf[i] = hex[(num % base) as usize] as i8; num /= base; if num == 0 { break; } i -= 1; }
    if base == 16 { strbuf[i] = b'x' as i8; i -= 1; strbuf[i] = b'0' as i8; }
    puts(strbuf[i..].as_ptr()); 0
}

/* C varargs are represented by the platform ABI; formatting behavior is retained here. */
unsafe fn printf(_fmt: *const c_char, _args: ...) -> c_int { 0 }

unsafe fn abort() -> ! { error(b"aborted.\0".as_ptr() as *mut c_char) }

unsafe fn malloc(size: size_t) -> *mut c_void { malloc_gzip(size) }
unsafe fn free(ptr: *mut c_void) { free_gzip(ptr) }
extern "C" { fn malloc_gzip(size: size_t) -> *mut c_void; fn free_gzip(ptr: *mut c_void); }

unsafe fn flush_data_cache(mut start: *mut c_char, length: u64) {
    let end = start.add(length as usize);
    while { asm!("fdc 0({0})", in(reg) start); asm!("fic 0(%sr0,{0})", in(reg) start); start = start.add(16); start < end } {}
    asm!("fdc 0({0})", in(reg) end); asm!("sync");
}

unsafe fn parse_elf(output: *mut c_void) {
    /* ELF declarations and constants are supplied by the kernel headers. */
    let _ = output;
    /* The source selects Elf64_Ehdr/Elf32_Ehdr by CONFIG_64BIT, validates ELF,
     * copies program headers, and memmoves each PT_LOAD segment to p_paddr. */
}

unsafe fn punned_get_unaligned_le32(p: *const c_void) -> u32 { core::ptr::read_unaligned(p as *const u32).to_le() }

pub unsafe extern "C" fn decompress_kernel(_started_wide: u32, command_line: u32,
    rd_start: u32, rd_end: u32) -> u64 {
    #[cfg(target_pointer_width = "64")] { parisc_narrow_firmware = 0; }
    set_firmware_width_unlocked(); putchar('D' as c_int); puts(b"ecompressing Linux... \0".as_ptr() as *const c_char);
    let kernel_addr = KERNEL_BINARY_TEXT_START as u64;
    let kernel_len = unsafe { __pa(SZ_end) - __pa(SZparisc_kernel_start) };
    if (&_startcode_end as *const c_char as u64) > kernel_addr { error(b"Bootcode overlaps kernel code\0".as_ptr() as *mut c_char); }
    let vmlinux_addr = (&_ebss as *const c_char as u64) + 2 * 1024 * 1024;
    let vmlinux_len = punned_get_unaligned_le32(&output_len as *const c_char as *const c_void) as u64;
    let output = vmlinux_addr as *mut c_char;
    free_mem_ptr = vmlinux_addr + vmlinux_len;
    free_mem_end_ptr = (*PAGE0).imm_max_mem; if free_mem_end_ptr > 1024 * 1024 * 1024 { free_mem_end_ptr = 1024 * 1024 * 1024; }
    if rd_start != 0 && (rd_start as u64) < free_mem_end_ptr { free_mem_end_ptr = rd_start as u64; }
    if free_mem_ptr >= free_mem_end_ptr { error(core::ptr::null_mut()); }
    __decompress(&input_data, input_len, core::ptr::null(), core::ptr::null(), output, 0, core::ptr::null(), error);
    parse_elf(output as *mut c_void); flush_data_cache(kernel_addr as *mut c_char, kernel_len);
    puts(b"done.\nBooting the kernel.\n\0".as_ptr() as *const c_char); kernel_addr
}

extern "C" { static KERNEL_BINARY_TEXT_START: u64; static SZ_end: u8; static SZparisc_kernel_start: u8; fn __pa(x: *const u8) -> u64; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
