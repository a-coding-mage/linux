// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Declarations supplied by the Linux syscall, page, cacheflush, and cachectl
// dependencies are intentionally left external to this translation unit.

use core::ffi::c_void;

extern "C" {
    static mut current: *mut TaskStruct;
    fn dcache_wb_range(start: c_ulong, end: c_ulong);
    fn flush_icache_mm_range(mm: *mut MmStruct, start: c_ulong, end: c_ulong);
}

#[allow(non_camel_case_types)]
type c_ulong = usize;
#[allow(non_camel_case_types)]
type c_int = i32;
#[allow(non_camel_case_types)]
type c_long = isize;

#[repr(C)]
pub struct TaskStruct {
    pub mm: *mut MmStruct,
}

#[repr(C)]
pub struct MmStruct {
    _private: [u8; 0],
}

// Values provided by <asm/cachectl.h>.
extern "C" {
    static BCACHE: c_int;
    static DCACHE: c_int;
    static ICACHE: c_int;
}

// SYSCALL_DEFINE3(cacheflush, void __user *addr, unsigned long bytes,
//                  int cache)
pub unsafe fn cacheflush(addr: *mut c_void, bytes: c_ulong, cache: c_int) -> c_long {
    if cache == BCACHE || cache == DCACHE {
        dcache_wb_range(
            addr as c_ulong,
            (addr as c_ulong).wrapping_add(bytes),
        );
        if cache != BCACHE {
            return 0;
        }
        // fallthrough
        flush_icache_mm_range(
            (*current).mm,
            addr as c_ulong,
            (addr as c_ulong).wrapping_add(bytes),
        );
    } else if cache == ICACHE {
        flush_icache_mm_range(
            (*current).mm,
            addr as c_ulong,
            (addr as c_ulong).wrapping_add(bytes),
        );
    } else {
        return -22; // -EINVAL
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
