// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2018, Christophe Leroy CS S.I.
 * <christophe.leroy@c-s.fr>
 *
 * This dumps the content of Segment Registers
 */

use core::ffi::c_void;

#[repr(C)]
pub struct SeqFile {
    _private: [u8; 0],
}

extern "C" {
    fn mfsr(address: u32) -> u32;
    fn seq_printf(m: *mut SeqFile, format: *const i8, ...);
    fn seq_puts(m: *mut SeqFile, string: *const i8);
    fn debugfs_create_file(
        name: *const i8,
        mode: u32,
        parent: *mut c_void,
        data: *mut c_void,
        fops: *const c_void,
    ) -> *mut c_void;
    static mut arch_debugfs_dir: *mut c_void;
    static sr_fops: c_void;
}

// These values and the alignment operation are supplied by the architecture headers.
extern "C" {
    static TASK_SIZE: usize;
    static SZ_256M: usize;
}

#[inline(always)]
unsafe fn align(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

unsafe fn seg_show(m: *mut SeqFile, i: i32) {
    let val: u32 = mfsr((i << 28) as u32);

    seq_printf(m, b"0x%01x0000000-0x%01xfffffff \0".as_ptr() as *const i8, i, i);
    seq_printf(m, b"Kern key %d \0".as_ptr() as *const i8, (val >> 30) & 1);
    seq_printf(m, b"User key %d \0".as_ptr() as *const i8, (val >> 29) & 1);
    if val & 0x80000000 != 0 {
        seq_printf(
            m,
            b"Device 0x%03x\0".as_ptr() as *const i8,
            (val >> 20) & 0x1ff,
        );
        seq_printf(m, b"-0x%05x\0".as_ptr() as *const i8, val & 0xfffff);
    } else {
        if val & 0x10000000 != 0 {
            seq_puts(m, b"No Exec \0".as_ptr() as *const i8);
        }
        seq_printf(m, b"VSID 0x%06x\0".as_ptr() as *const i8, val & 0xffffff);
    }
    seq_puts(m, b"\n\0".as_ptr() as *const i8);
}

unsafe fn sr_show(m: *mut SeqFile, _v: *mut c_void) -> i32 {
    let mut i: i32;

    seq_puts(m, b"---[ User Segments ]---\n\0".as_ptr() as *const i8);
    i = 0;
    while i < (align(TASK_SIZE, SZ_256M) >> 28) as i32 {
        seg_show(m, i);
        i += 1;
    }

    seq_puts(m, b"\n---[ Kernel Segments ]---\n\0".as_ptr() as *const i8);
    while i < 16 {
        seg_show(m, i);
        i += 1;
    }

    0
}

// DEFINE_SHOW_ATTRIBUTE(sr);
// The generated sr_fops declaration is supplied by the surrounding kernel bindings.

unsafe fn sr_init() -> i32 {
    debugfs_create_file(
        b"segment_registers\0".as_ptr() as *const i8,
        0o400,
        arch_debugfs_dir,
        core::ptr::null_mut(),
        &sr_fops as *const c_void,
    );
    0
}

// device_initcall(sr_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
