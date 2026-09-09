// SPDX-License-Identifier: GPL-2.0
// External Linux kernel declarations and constants are supplied by the surrounding build.

use core::ffi::c_char;

extern "C" {
    fn setup_fault_attr(attr: *mut fault_attr, str_: *mut c_char) -> i32;
    fn should_fail_ex(attr: *mut fault_attr, times: u64, flags: i32) -> bool;
    fn fault_create_debugfs_attr(
        name: *const c_char,
        parent: *mut dentry,
        attr: *mut fault_attr,
    ) -> *mut dentry;
    fn debugfs_create_bool(
        name: *const c_char,
        mode: umode_t,
        parent: *mut dentry,
        value: *mut bool,
    );
    fn debugfs_create_u32(
        name: *const c_char,
        mode: umode_t,
        parent: *mut dentry,
        value: *mut u32,
    );
}

#[repr(C)]
pub struct fault_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

pub type gfp_t = u32;
pub type umode_t = u32;

const __GFP_NOFAIL: gfp_t = 0;
const __GFP_HIGHMEM: gfp_t = 0;
const __GFP_DIRECT_RECLAIM: gfp_t = 0;
const __GFP_NOWARN: gfp_t = 0;
const FAULT_NOWARN: i32 = 0;
const S_IFREG: umode_t = 0;

#[repr(C)]
struct FailPageAlloc {
    attr: fault_attr,
    ignore_gfp_highmem: bool,
    ignore_gfp_reclaim: bool,
    min_order: u32,
}

// FAULT_ATTR_INITIALIZER
static mut fail_page_alloc: FailPageAlloc = FailPageAlloc {
    attr: fault_attr { _private: [] },
    ignore_gfp_highmem: true,
    ignore_gfp_reclaim: true,
    min_order: 1,
};

unsafe extern "C" fn setup_fail_page_alloc(str_: *mut c_char) -> i32 {
    setup_fault_attr(&mut fail_page_alloc.attr, str_)
}

// __setup("fail_page_alloc=", setup_fail_page_alloc);

pub unsafe extern "C" fn should_fail_alloc_page(gfp_mask: gfp_t, order: u32) -> bool {
    let mut flags: i32 = 0;

    if order < fail_page_alloc.min_order {
        return false;
    }
    if gfp_mask & __GFP_NOFAIL != 0 {
        return false;
    }
    if fail_page_alloc.ignore_gfp_highmem && (gfp_mask & __GFP_HIGHMEM != 0) {
        return false;
    }
    if fail_page_alloc.ignore_gfp_reclaim && (gfp_mask & __GFP_DIRECT_RECLAIM != 0) {
        return false;
    }

    /* See comment in __should_failslab() */
    if gfp_mask & __GFP_NOWARN != 0 {
        flags |= FAULT_NOWARN;
    }

    should_fail_ex(&mut fail_page_alloc.attr, 1u64 << order, flags)
}

// ALLOW_ERROR_INJECTION(should_fail_alloc_page, TRUE);

// CONFIG_FAULT_INJECTION_DEBUG_FS
#[allow(dead_code)]
unsafe extern "C" fn fail_page_alloc_debugfs() -> i32 {
    let mode: umode_t = S_IFREG | 0o600;
    let dir: *mut dentry;

    dir = fault_create_debugfs_attr(
        b"fail_page_alloc\0".as_ptr() as *const c_char,
        core::ptr::null_mut(),
        &mut fail_page_alloc.attr,
    );

    debugfs_create_bool(
        b"ignore-gfp-wait\0".as_ptr() as *const c_char,
        mode,
        dir,
        &mut fail_page_alloc.ignore_gfp_reclaim,
    );
    debugfs_create_bool(
        b"ignore-gfp-highmem\0".as_ptr() as *const c_char,
        mode,
        dir,
        &mut fail_page_alloc.ignore_gfp_highmem,
    );
    debugfs_create_u32(
        b"min-order\0".as_ptr() as *const c_char,
        mode,
        dir,
        &mut fail_page_alloc.min_order,
    );

    0
}

// late_initcall(fail_page_alloc_debugfs);
// #endif /* CONFIG_FAULT_INJECTION_DEBUG_FS */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
