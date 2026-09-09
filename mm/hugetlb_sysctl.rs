// SPDX-License-Identifier: GPL-2.0-only
/*
 * HugeTLB sysfs interfaces.
 * (C) Nadia Yvette Chambers, April 2004
 */

// Dependencies supplied by the kernel headers and hugetlb_internal.h.
use core::ffi::c_void;
use core::mem::size_of;

#[repr(C)]
pub struct ctl_table {
    pub procname: *const core::ffi::c_char,
    pub data: *mut c_void,
    pub maxlen: usize,
    pub mode: u16,
    pub proc_handler: Option<unsafe extern "C" fn(*const ctl_table, i32, *mut c_void, *mut usize, *mut i64) -> i32>,
}

#[repr(C)]
pub struct hstate {
    pub max_huge_pages: c_ulong,
    pub nr_overcommit_huge_pages: c_ulong,
}

type c_ulong = usize;
type gid_t = u32;

extern "C" {
    static mut default_hstate: hstate;
    static mut hugetlb_lock: c_void;
    static mut sysctl_hugetlb_shm_group: gid_t;
    fn hugepages_supported() -> bool;
    fn proc_doulongvec_minmax(table: *const ctl_table, write: i32, buffer: *mut c_void,
                              length: *mut usize, ppos: *mut i64) -> i32;
    fn __nr_hugepages_store_common(obey_mempolicy: bool, h: *mut hstate,
                                   nid: i32, tmp: c_ulong, length: usize) -> i32;
    fn hstate_is_gigantic_no_runtime(h: *const hstate) -> bool;
    fn spin_lock_irq(lock: *mut c_void);
    fn spin_unlock_irq(lock: *mut c_void);
    fn register_sysctl_init(name: *const core::ffi::c_char, table: *const ctl_table);
    fn proc_dointvec(table: *const ctl_table, write: i32, buffer: *mut c_void,
                     length: *mut usize, ppos: *mut i64) -> i32;
}

const NUMA_NO_NODE: i32 = -1;
pub static mut movable_gigantic_pages: i32 = 0;

#[cfg(feature = "CONFIG_SYSCTL")]
unsafe extern "C" fn proc_hugetlb_doulongvec_minmax(
    table: *const ctl_table,
    write: i32,
    buffer: *mut c_void,
    length: *mut usize,
    ppos: *mut i64,
    out: *mut c_ulong,
) -> i32 {
    let mut dup_table: ctl_table = *table;
    dup_table.data = out.cast::<c_void>();
    proc_doulongvec_minmax(&dup_table, write, buffer, length, ppos)
}

#[cfg(feature = "CONFIG_SYSCTL")]
unsafe extern "C" fn hugetlb_sysctl_handler_common(
    obey_mempolicy: bool,
    table: *const ctl_table,
    write: i32,
    buffer: *mut c_void,
    length: *mut usize,
    ppos: *mut i64,
) -> i32 {
    let h = &mut default_hstate as *mut hstate;
    let mut tmp = (*h).max_huge_pages;
    let mut ret: i32;

    if !hugepages_supported() {
        return -95; // -EOPNOTSUPP
    }

    ret = proc_hugetlb_doulongvec_minmax(table, write, buffer, length, ppos, &mut tmp);
    if ret != 0 {
        return ret;
    }

    if write != 0 {
        ret = __nr_hugepages_store_common(obey_mempolicy, h, NUMA_NO_NODE, tmp, *length);
    }
    ret
}

#[cfg(feature = "CONFIG_SYSCTL")]
unsafe extern "C" fn hugetlb_sysctl_handler(
    table: *const ctl_table, write: i32, buffer: *mut c_void,
    length: *mut usize, ppos: *mut i64,
) -> i32 {
    hugetlb_sysctl_handler_common(false, table, write, buffer, length, ppos)
}

#[cfg(all(feature = "CONFIG_SYSCTL", feature = "CONFIG_NUMA"))]
unsafe extern "C" fn hugetlb_mempolicy_sysctl_handler(
    table: *const ctl_table, write: i32, buffer: *mut c_void,
    length: *mut usize, ppos: *mut i64,
) -> i32 {
    hugetlb_sysctl_handler_common(true, table, write, buffer, length, ppos)
}

#[cfg(feature = "CONFIG_SYSCTL")]
unsafe extern "C" fn hugetlb_overcommit_handler(
    table: *const ctl_table, write: i32, buffer: *mut c_void,
    length: *mut usize, ppos: *mut i64,
) -> i32 {
    let h = &mut default_hstate as *mut hstate;
    let mut tmp: c_ulong;

    if !hugepages_supported() {
        return -95; // -EOPNOTSUPP
    }
    tmp = (*h).nr_overcommit_huge_pages;
    if write != 0 && hstate_is_gigantic_no_runtime(h) {
        return -22; // -EINVAL
    }

    let ret = proc_hugetlb_doulongvec_minmax(table, write, buffer, length, ppos, &mut tmp);
    if ret != 0 {
        return ret;
    }
    if write != 0 {
        spin_lock_irq(&mut hugetlb_lock);
        (*h).nr_overcommit_huge_pages = tmp;
        spin_unlock_irq(&mut hugetlb_lock);
    }
    ret
}

#[cfg(feature = "CONFIG_SYSCTL")]
static hugetlb_table: &[ctl_table] = &[
    ctl_table { procname: b"nr_hugepages\0".as_ptr().cast(), data: core::ptr::null_mut(), maxlen: size_of::<c_ulong>(), mode: 0o644, proc_handler: Some(hugetlb_sysctl_handler) },
    #[cfg(feature = "CONFIG_NUMA")]
    ctl_table { procname: b"nr_hugepages_mempolicy\0".as_ptr().cast(), data: core::ptr::null_mut(), maxlen: size_of::<c_ulong>(), mode: 0o644, proc_handler: Some(hugetlb_mempolicy_sysctl_handler) },
    ctl_table { procname: b"hugetlb_shm_group\0".as_ptr().cast(), data: unsafe { &raw mut sysctl_hugetlb_shm_group }.cast(), maxlen: size_of::<gid_t>(), mode: 0o644, proc_handler: Some(proc_dointvec) },
    ctl_table { procname: b"nr_overcommit_hugepages\0".as_ptr().cast(), data: core::ptr::null_mut(), maxlen: size_of::<c_ulong>(), mode: 0o644, proc_handler: Some(hugetlb_overcommit_handler) },
    ctl_table { procname: b"movable_gigantic_pages\0".as_ptr().cast(), data: unsafe { &raw mut movable_gigantic_pages }.cast(), maxlen: size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec) },
];

#[cfg(feature = "CONFIG_SYSCTL")]
pub unsafe extern "C" fn hugetlb_sysctl_init() {
    register_sysctl_init(b"vm\0".as_ptr().cast(), hugetlb_table.as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
