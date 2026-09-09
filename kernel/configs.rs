// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * kernel/configs.c
 * Echo the kernel .config file used to build the kernel
 *
 * Copyright (C) 2002 Khalid Aziz <khalid_aziz@hp.com>
 * Copyright (C) 2002 Randy Dunlap <rdunlap@xenotime.net>
 * Copyright (C) 2002 Al Stone <ahs3@fc.hp.com>
 * Copyright (C) 2002 Hewlett-Packard Company
 */

// The C source includes Linux kernel headers supplying these types and symbols.
use core::ffi::c_void;

// "IKCFG_ST" and "IKCFG_ED" delimit the embedded compressed configuration.
// The linker/assembly definition is intentionally preserved as an external
// dependency; its original .incbin source is build-time generated.
extern "C" {
    static mut kernel_config_data: u8;
    static mut kernel_config_data_end: u8;
}

#[cfg(CONFIG_IKCONFIG_PROC)]
type SizeT = usize;

#[cfg(CONFIG_IKCONFIG_PROC)]
type SSizeT = isize;

#[cfg(CONFIG_IKCONFIG_PROC)]
type LoffT = i64;

#[cfg(CONFIG_IKCONFIG_PROC)]
#[repr(C)]
pub struct File {
    _private: [u8; 0],
}

#[cfg(CONFIG_IKCONFIG_PROC)]
#[repr(C)]
pub struct ProcDirEntry {
    _private: [u8; 0],
}

#[cfg(CONFIG_IKCONFIG_PROC)]
#[repr(C)]
pub struct ProcOps {
    pub proc_read: Option<unsafe extern "C" fn(
        file: *mut File,
        buf: *mut c_void,
        len: SizeT,
        offset: *mut LoffT,
    ) -> SSizeT>,
    pub proc_lseek: Option<unsafe extern "C" fn(
        file: *mut File,
        offset: LoffT,
        whence: i32,
    ) -> LoffT>,
}

#[cfg(CONFIG_IKCONFIG_PROC)]
extern "C" {
    fn simple_read_from_buffer(
        to: *mut c_void,
        count: SizeT,
        ppos: *mut LoffT,
        from: *const c_void,
        available: SizeT,
    ) -> SSizeT;
    fn default_llseek(file: *mut File, offset: LoffT, whence: i32) -> LoffT;
    fn proc_create(
        name: *const u8,
        mode: u32,
        parent: *mut ProcDirEntry,
        proc_ops: *const ProcOps,
    ) -> *mut ProcDirEntry;
    fn proc_set_size(entry: *mut ProcDirEntry, size: LoffT);
    fn remove_proc_entry(name: *const u8, parent: *mut ProcDirEntry);
}

#[cfg(CONFIG_IKCONFIG_PROC)]
unsafe extern "C" fn ikconfig_read_current(
    _file: *mut File,
    buf: *mut c_void,
    len: SizeT,
    offset: *mut LoffT,
) -> SSizeT {
    simple_read_from_buffer(
        buf,
        len,
        offset,
        &kernel_config_data as *const u8 as *const c_void,
        (&kernel_config_data_end as *const u8 as usize)
            .wrapping_sub(&kernel_config_data as *const u8 as usize),
    )
}

#[cfg(CONFIG_IKCONFIG_PROC)]
static CONFIG_GZ_PROC_OPS: ProcOps = ProcOps {
    proc_read: Some(ikconfig_read_current),
    proc_lseek: Some(default_llseek),
};

#[cfg(CONFIG_IKCONFIG_PROC)]
unsafe extern "C" fn ikconfig_init() -> i32 {
    let entry: *mut ProcDirEntry;

    // create the current config file
    entry = proc_create(
        b"config.gz\0".as_ptr(),
        0o100444,
        core::ptr::null_mut(),
        &CONFIG_GZ_PROC_OPS,
    );
    if entry.is_null() {
        return -12; // -ENOMEM
    }

    proc_set_size(
        entry,
        (&kernel_config_data_end as *const u8 as usize)
            .wrapping_sub(&kernel_config_data as *const u8 as usize) as LoffT,
    );

    0
}

#[cfg(CONFIG_IKCONFIG_PROC)]
unsafe extern "C" fn ikconfig_cleanup() {
    remove_proc_entry(b"config.gz\0".as_ptr(), core::ptr::null_mut());
}

// module_init(ikconfig_init);
// module_exit(ikconfig_cleanup);

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Randy Dunlap");
// MODULE_DESCRIPTION("Echo the kernel .config file used to build the kernel");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
