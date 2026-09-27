/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub const VECSIZE: usize = 0x200;

extern "C" {
    pub static mut eentry: kernel::ffi::c_ulong;
    pub static mut tlbrentry: kernel::ffi::c_ulong;
    pub static mut pcpu_handlers: [kernel::ffi::c_ulong; NR_CPUS];
    pub static mut exception_handlers:
        [kernel::ffi::c_long; VECSIZE * 128 / core::mem::size_of::<kernel::ffi::c_long>()];
    pub static mut init_command_line: [kernel::ffi::c_char; COMMAND_LINE_SIZE];
    pub fn tlb_init(cpu: kernel::ffi::c_int);
    pub fn cpu_cache_init();
    pub fn cache_error_setup();
    pub fn per_cpu_trap_init(cpu: kernel::ffi::c_int);
    pub fn set_handler(
        offset: kernel::ffi::c_ulong,
        addr: *mut kernel::ffi::c_void,
        len: kernel::ffi::c_ulong,
    );
    pub fn set_merr_handler(
        offset: kernel::ffi::c_ulong,
        addr: *mut kernel::ffi::c_void,
        len: kernel::ffi::c_ulong,
    );
}

#[cfg(CONFIG_RELOCATABLE)]
#[repr(C)]
pub struct rela_la_abs {
    pub pc: kernel::ffi::c_long,
    pub symvalue: kernel::ffi::c_long,
}

#[cfg(CONFIG_RELOCATABLE)]
extern "C" {
    pub static mut __la_abs_begin: kernel::ffi::c_long;
    pub static mut __la_abs_end: kernel::ffi::c_long;
    pub static mut __rela_dyn_begin: kernel::ffi::c_long;
    pub static mut __rela_dyn_end: kernel::ffi::c_long;

    #[cfg(CONFIG_RELR)]
    pub static mut __relr_dyn_begin: kernel::ffi::c_long;
    #[cfg(CONFIG_RELR)]
    pub static mut __relr_dyn_end: kernel::ffi::c_long;

    pub fn relocate_kernel() -> kernel::ffi::c_ulong;
}

extern "C" {
    static _text: u8;
}

#[inline]
pub unsafe fn kaslr_offset() -> kernel::ffi::c_ulong {
    (&_text as *const u8 as kernel::ffi::c_ulong).wrapping_sub(VMLINUX_LOAD_ADDRESS)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
