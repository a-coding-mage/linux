/* SPDX-License-Identifier: GPL-2.0-only */

// C dependencies: <sys/mman.h>, <sys/resource.h>, <stddef.h>, <strings.h>,
// and "kselftest_harness.h".

pub const TOP_DOWN: i32 = 0;
pub const BOTTOM_UP: i32 = 1;

pub const PROT: i32 = PROT_READ | PROT_WRITE;
pub const FLAGS: i32 = MAP_PRIVATE | MAP_ANONYMOUS;

unsafe extern "C" {
    pub static PROT_READ: i32;
    pub static PROT_WRITE: i32;
    pub static MAP_PRIVATE: i32;
    pub static MAP_ANONYMOUS: i32;

    pub fn mmap(
        addr: *mut core::ffi::c_void,
        length: usize,
        prot: i32,
        flags: i32,
        fd: i32,
        offset: isize,
    ) -> *mut core::ffi::c_void;
}

pub unsafe fn memory_layout() -> i32 {
    let value1 = unsafe {
        mmap(
            core::ptr::null_mut(),
            core::mem::size_of::<i32>(),
            PROT,
            FLAGS,
            0,
            0,
        )
    };
    let value2 = unsafe {
        mmap(
            core::ptr::null_mut(),
            core::mem::size_of::<i32>(),
            PROT,
            FLAGS,
            0,
            0,
        )
    };

    (value2 > value1) as i32
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
