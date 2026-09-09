// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the surrounding kernel translation unit:
// linux/fs.h, linux/init.h, linux/interrupt.h, linux/irqnr.h,
// linux/proc_fs.h, linux/seq_file.h

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_operations {
    pub start: Option<unsafe fn(*mut seq_file, *mut i64) -> *mut c_void>,
    pub next: Option<unsafe fn(*mut seq_file, *mut c_void, *mut i64) -> *mut c_void>,
    pub stop: Option<unsafe fn(*mut seq_file, *mut c_void)>,
    pub show: Option<unsafe fn(*mut seq_file, *mut c_void) -> c_int>,
}

unsafe extern "C" {
    fn irq_get_nr_irqs() -> c_uint;
    fn show_interrupts(f: *mut seq_file, v: *mut c_void) -> c_int;
    fn proc_create_seq(
        name: *const c_char,
        mode: u16,
        parent: *mut proc_dir_entry,
        ops: *const seq_operations,
    ) -> *mut proc_dir_entry;
}

/*
 * /proc/interrupts
 */
unsafe fn int_seq_start(_f: *mut seq_file, pos: *mut i64) -> *mut c_void {
    if *pos <= irq_get_nr_irqs() as i64 {
        pos.cast()
    } else {
        core::ptr::null_mut()
    }
}

unsafe fn int_seq_next(
    _f: *mut seq_file,
    _v: *mut c_void,
    pos: *mut i64,
) -> *mut c_void {
    *pos += 1;
    if *pos > irq_get_nr_irqs() as i64 {
        return core::ptr::null_mut();
    }
    pos.cast()
}

unsafe fn int_seq_stop(_f: *mut seq_file, _v: *mut c_void) {
    /* Nothing to do */
}

static INT_SEQ_OPS: seq_operations = seq_operations {
    start: Some(int_seq_start),
    next: Some(int_seq_next),
    stop: Some(int_seq_stop),
    show: Some(show_interrupts),
};

unsafe fn proc_interrupts_init() -> c_int {
    static INTERRUPTS: &[u8] = b"interrupts\0";
    proc_create_seq(
        INTERRUPTS.as_ptr().cast(),
        0,
        core::ptr::null_mut(),
        &INT_SEQ_OPS,
    );
    0
}

// Equivalent to the kernel's fs_initcall(proc_interrupts_init) registration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
