// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Kernel and ABI dependencies are supplied by other translation units.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn seq_printf(m: *mut seq_file, format: *const c_char, ...);
    fn smp_processor_id() -> c_int;
    fn smp_call_function_single(
        cpu: c_int,
        func: unsafe extern "C" fn(*mut c_void),
        info: *mut c_void,
        wait: bool,
    ) -> c_int;
    fn mfcr(register: *const c_char) -> u32;
    fn mfcr_hint() -> u32;
    fn mfcr_ccr2() -> u32;
}

extern "C" {
    static CSKYCPU_DEF_NAME: *const c_char;
}

unsafe extern "C" fn percpu_print(arg: *mut c_void) {
    let m = arg as *mut seq_file;
    let mut cur: u32;
    let mut next: u32;
    let mut i: u32;

    seq_printf(
        m,
        b"processor       : %d\n\0".as_ptr() as *const c_char,
        smp_processor_id(),
    );
    seq_printf(
        m,
        b"C-SKY CPU model : %s\n\0".as_ptr() as *const c_char,
        CSKYCPU_DEF_NAME,
    );

    /* read processor id, max is 100 */
    cur = mfcr(b"cr13\0".as_ptr() as *const c_char);
    i = 0;
    while i < 100 {
        seq_printf(
            m,
            b"product info[%d] : 0x%08x\n\0".as_ptr() as *const c_char,
            i,
            cur,
        );

        next = mfcr(b"cr13\0".as_ptr() as *const c_char);

        /* some CPU only has one id reg */
        if cur == next {
            break;
        }

        cur = next;

        /* cpid index is 31-28, reset */
        if (next >> 28) == 0 {
            while ((mfcr(b"cr13\0".as_ptr() as *const c_char) >> 28) != i) {}
            break;
        }

        i = i.wrapping_add(1);
    }

    /* CPU feature regs, setup by bootloader or gdbinit */
    seq_printf(
        m,
        b"hint (CPU funcs): 0x%08x\n\0".as_ptr() as *const c_char,
        mfcr_hint(),
    );
    seq_printf(
        m,
        b"ccr  (L1C & MMU): 0x%08x\n\0".as_ptr() as *const c_char,
        mfcr(b"cr18\0".as_ptr() as *const c_char),
    );
    seq_printf(
        m,
        b"ccr2 (L2C)      : 0x%08x\n\0".as_ptr() as *const c_char,
        mfcr_ccr2(),
    );
    seq_printf(m, b"\n\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn c_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    let mut cpu: c_int;

    // Translation of the kernel's for_each_online_cpu(cpu) macro.
    for_each_online_cpu!(cpu) {
        smp_call_function_single(cpu, percpu_print, m as *mut c_void, true);
    }

    // #ifdef CSKY_ARCH_VERSION: retain this build-time conditional for the
    // configuration supplying the architecture version symbol.
    #[cfg(CSKY_ARCH_VERSION)]
    {
        seq_printf(
            m,
            b"arch-version : %s\n\0".as_ptr() as *const c_char,
            CSKY_ARCH_VERSION,
        );
        seq_printf(m, b"\n\0".as_ptr() as *const c_char);
    }

    0
}

unsafe extern "C" fn c_start(_m: *mut seq_file, pos: *mut i64) -> *mut c_void {
    if *pos < 1 {
        1 as *mut c_void
    } else {
        core::ptr::null_mut()
    }
}

unsafe extern "C" fn c_next(
    _m: *mut seq_file,
    _v: *mut c_void,
    pos: *mut i64,
) -> *mut c_void {
    *pos = (*pos).wrapping_add(1);
    core::ptr::null_mut()
}

unsafe extern "C" fn c_stop(_m: *mut seq_file, _v: *mut c_void) {}

#[repr(C)]
pub struct seq_operations {
    pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut i64) -> *mut c_void>,
    pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut i64) -> *mut c_void>,
    pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void)>,
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>,
}

#[no_mangle]
pub static cpuinfo_op: seq_operations = seq_operations {
    start: Some(c_start),
    next: Some(c_next),
    stop: Some(c_stop),
    show: Some(c_show),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
