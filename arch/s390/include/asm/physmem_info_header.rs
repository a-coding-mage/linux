/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum physmem_info_source {
    MEM_DETECT_NONE = 0,
    MEM_DETECT_SCLP_STOR_INFO,
    MEM_DETECT_DIAG260,
    MEM_DETECT_DIAG500_STOR_LIMIT,
    MEM_DETECT_SCLP_READ_INFO,
    MEM_DETECT_BIN_SEARCH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct physmem_range {
    pub start: u64,
    pub end: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reserved_range_type {
    RR_DECOMPRESSOR = 0,
    RR_INITRD,
    RR_VMLINUX,
    RR_AMODE31,
    RR_IPLREPORT,
    RR_CERT_COMP_LIST,
    RR_MEM_DETECT_EXT,
    RR_VMEM,
    RR_MAX,
}

#[repr(C)]
pub struct reserved_range {
    pub start: usize,
    pub end: usize,
    pub chain: *mut reserved_range,
}

/* Storage element ids are one byte. 255 physmem_ranges are embedded here. */
pub const MEM_INLINED_ENTRIES: usize = 255; /* (PAGE_SIZE - 16) / 16 */

#[repr(C)]
pub struct physmem_info {
    pub range_count: u32,
    pub info_source: u8,
    pub usable: usize,
    pub reserved: [reserved_range; RR_MAX as usize],
    pub online: [physmem_range; MEM_INLINED_ENTRIES],
    pub online_extended: *mut physmem_range,
}

extern "C" {
    pub static mut physmem_info: physmem_info;
    pub fn add_physmem_online_range(start: u64, end: u64);
    pub fn __va(addr: *mut reserved_range) -> *mut reserved_range;
}

pub unsafe fn __get_physmem_range(
    n: u32,
    start: *mut usize,
    end: *mut usize,
    respect_usable_limit: bool,
) -> i32 {
    if n >= physmem_info.range_count {
        *start = 0;
        *end = 0;
        return -1;
    }

    if (n as usize) < MEM_INLINED_ENTRIES {
        *start = physmem_info.online[n as usize].start as usize;
        *end = physmem_info.online[n as usize].end as usize;
    } else {
        let range = &*physmem_info.online_extended.add(n as usize - MEM_INLINED_ENTRIES);
        *start = range.start as usize;
        *end = range.end as usize;
    }

    if respect_usable_limit && physmem_info.usable != 0 {
        if *start >= physmem_info.usable {
            return -1;
        }
        if *end > physmem_info.usable {
            *end = physmem_info.usable;
        }
    }
    0
}

/* C iterator macros for detected usable and online ranges. */
#[macro_export]
macro_rules! for_each_physmem_usable_range {
    ($i:ident, $p_start:expr, $p_end:expr) => {
        for $i in 0u32.. {
            if unsafe { $crate::__get_physmem_range($i, $p_start, $p_end, true) } != 0 { break; }
        }
    };
}
#[macro_export]
macro_rules! for_each_physmem_online_range {
    ($i:ident, $p_start:expr, $p_end:expr) => {
        for $i in 0u32.. {
            if unsafe { $crate::__get_physmem_range($i, $p_start, $p_end, false) } != 0 { break; }
        }
    };
}

pub unsafe fn get_physmem_info_source() -> *const u8 {
    match physmem_info.info_source {
        x if x == MEM_DETECT_SCLP_STOR_INFO as u8 => b"sclp storage info\0".as_ptr(),
        x if x == MEM_DETECT_DIAG260 as u8 => b"diag260\0".as_ptr(),
        x if x == MEM_DETECT_DIAG500_STOR_LIMIT as u8 => b"diag500 storage limit\0".as_ptr(),
        x if x == MEM_DETECT_SCLP_READ_INFO as u8 => b"sclp read info\0".as_ptr(),
        x if x == MEM_DETECT_BIN_SEARCH as u8 => b"binary search\0".as_ptr(),
        _ => b"none\0".as_ptr(),
    }
}

pub unsafe fn get_rr_type_name(t: reserved_range_type) -> *const u8 {
    match t {
        reserved_range_type::RR_DECOMPRESSOR => b"DECOMPRESSOR\0".as_ptr(),
        reserved_range_type::RR_INITRD => b"INITRD\0".as_ptr(),
        reserved_range_type::RR_VMLINUX => b"VMLINUX\0".as_ptr(),
        reserved_range_type::RR_AMODE31 => b"AMODE31\0".as_ptr(),
        reserved_range_type::RR_IPLREPORT => b"IPLREPORT\0".as_ptr(),
        reserved_range_type::RR_CERT_COMP_LIST => b"CERT_COMP_LIST\0".as_ptr(),
        reserved_range_type::RR_MEM_DETECT_EXT => b"MEM_DETECT_EXT\0".as_ptr(),
        reserved_range_type::RR_VMEM => b"VMEM\0".as_ptr(),
        _ => b"UNKNOWN\0".as_ptr(),
    }
}

pub unsafe fn __physmem_reserved_next(
    t: *mut reserved_range_type,
    mut range: *mut reserved_range,
) -> *mut reserved_range {
    if range.is_null() {
        range = &mut physmem_info.reserved[*t as usize];
        if (*range).end != 0 { return range; }
    }
    if !(*range).chain.is_null() { return __va((*range).chain); }
    while (*t as usize) + 1 < RR_MAX as usize {
        *t = core::mem::transmute((*t as usize) + 1);
        range = &mut physmem_info.reserved[*t as usize];
        if (*range).end != 0 { return range; }
    }
    core::ptr::null_mut()
}

pub unsafe fn get_physmem_reserved(
    type_: reserved_range_type, addr: *mut usize, size: *mut usize,
) -> usize {
    *addr = physmem_info.reserved[type_ as usize].start;
    *size = physmem_info.reserved[type_ as usize].end - *addr;
    *size
}

pub const AMODE31_START: usize = 0; // physmem_info.reserved[RR_AMODE31].start
pub const AMODE31_END: usize = 0; // physmem_info.reserved[RR_AMODE31].end

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
