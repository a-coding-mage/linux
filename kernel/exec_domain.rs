// SPDX-License-Identifier: GPL-2.0
/*
 * Handling of different ABIs (personalities).
 *
 * We group personalities into execution domains which have their
 * own handlers for kernel entry points, signal mapping, etc...
 *
 * 2001-05-06	Complete rewrite,  Christoph Hellwig (hch@infradead.org)
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

#[repr(C)]
pub struct SeqFile {
    _private: [u8; 0],
}

extern "C" {
    fn seq_puts(m: *mut SeqFile, s: *const u8);
    fn proc_create_single(
        name: *const u8,
        mode: u16,
        parent: *mut c_void,
        show: unsafe extern "C" fn(*mut SeqFile, *mut c_void) -> i32,
    ) -> *mut c_void;
    fn set_personality(personality: u32);
}

// `current` is supplied by the kernel scheduler context.
#[repr(C)]
pub struct TaskStruct {
    pub personality: u32,
}

extern "C" {
    static mut current: *mut TaskStruct;
}

// The CONFIG_PROC_FS build-time condition is preserved here as a Rust cfg.
#[cfg(feature = "CONFIG_PROC_FS")]
unsafe extern "C" fn execdomains_proc_show(m: *mut SeqFile, _v: *mut c_void) -> i32 {
    static TEXT: &[u8] = b"0-0\tLinux           \t[kernel]\n\0";
    seq_puts(m, TEXT.as_ptr());
    0
}

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe extern "C" fn proc_execdomains_init() -> i32 {
    static NAME: &[u8] = b"execdomains\0";
    proc_create_single(NAME.as_ptr(), 0, core::ptr::null_mut(), execdomains_proc_show);
    0
}

// module_init(proc_execdomains_init);

pub unsafe extern "C" fn personality(personality: u32) -> u32 {
    let old = (*current).personality;

    if personality != 0xffff_ffff {
        set_personality(personality);
    }

    old
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
