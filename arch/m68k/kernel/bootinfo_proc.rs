// SPDX-License-Identifier: GPL-2.0
/*
 * Based on arch/arm/kernel/atags_proc.c
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

static mut BOOTINFO_TMP: [u8; 1536] = [0; 1536];

static mut bootinfo_copy: *mut c_void = core::ptr::null_mut();
static mut bootinfo_size: usize = 0;

unsafe extern "C" {
    fn simple_read_from_buffer(
        buf: *mut u8,
        count: usize,
        ppos: *mut i64,
        from: *const c_void,
        available: usize,
    ) -> isize;

    fn default_llseek(file: *mut file, offset: i64, whence: i32) -> i64;
    fn kmemdup(src: *const c_void, size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn proc_create_data(
        name: *const u8,
        mode: u32,
        parent: *mut proc_dir_entry,
        proc_ops: *const proc_ops,
        data: *mut c_void,
    ) -> *mut proc_dir_entry;
    fn be16_to_cpu(value: u16) -> u16;
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bi_record {
    pub tag: u16,
    pub size: u16,
}

#[repr(C)]
pub struct proc_ops {
    pub proc_read: Option<unsafe extern "C" fn(*mut file, *mut u8, usize, *mut i64) -> isize>,
    pub proc_lseek: Option<unsafe extern "C" fn(*mut file, i64, i32) -> i64>,
}

const BI_LAST: u16 = 0;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const GFP_KERNEL: u32 = 0;

unsafe extern "C" fn bootinfo_read(
    _file: *mut file,
    buf: *mut u8,
    count: usize,
    ppos: *mut i64,
) -> isize {
    unsafe { simple_read_from_buffer(buf, count, ppos, bootinfo_copy, bootinfo_size) }
}

static bootinfo_proc_ops: proc_ops = proc_ops {
    proc_read: Some(bootinfo_read),
    proc_lseek: Some(default_llseek),
};

pub unsafe extern "C" fn save_bootinfo(mut bi: *const bi_record) {
    let start = bi as *const c_void;
    let mut size = core::mem::size_of::<u16>();

    while unsafe { be16_to_cpu((*bi).tag) } != BI_LAST {
        let n = unsafe { be16_to_cpu((*bi).size) } as usize;
        size += n;
        bi = (bi as usize + n) as *const bi_record;
    }

    if size > core::mem::size_of::<[u8; 1536]>() {
        unsafe { pr_err(b"Cannot save %zu bytes of bootinfo\n\0".as_ptr(), size) };
        return;
    }

    unsafe { pr_info(b"Saving %zu bytes of bootinfo\n\0".as_ptr(), size) };
    unsafe {
        core::ptr::copy_nonoverlapping(start as *const u8, BOOTINFO_TMP.as_mut_ptr(), size);
        bootinfo_size = size;
    }
}

unsafe extern "C" {
    fn pr_err(format: *const u8, ...);
    fn pr_info(format: *const u8, ...);
}

pub unsafe extern "C" fn init_bootinfo_procfs() -> i32 {
    /*
     * This cannot go into save_bootinfo() because kmalloc and proc don't
     * work yet when it is called.
     */
    let pde: *mut proc_dir_entry;

    if unsafe { bootinfo_size } == 0 {
        return -EINVAL;
    }

    unsafe {
        bootinfo_copy = kmemdup(BOOTINFO_TMP.as_ptr() as *const c_void, bootinfo_size, GFP_KERNEL);
    }
    if unsafe { bootinfo_copy.is_null() } {
        return -ENOMEM;
    }

    pde = unsafe {
        proc_create_data(
            b"bootinfo\0".as_ptr(),
            0o400,
            core::ptr::null_mut(),
            &bootinfo_proc_ops,
            core::ptr::null_mut(),
        )
    };
    if pde.is_null() {
        unsafe { kfree(bootinfo_copy) };
        return -ENOMEM;
    }

    0
}

// arch_initcall(init_bootinfo_procfs);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
