// SPDX-License-Identifier: GPL-2.0-only
//
// External declarations supplied by the Linux fault-injection headers.
use core::ffi::c_char;

#[repr(C)]
pub struct fault_attr {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _opaque: [u8; 0],
}

unsafe extern "C" {
    fn setup_fault_attr(attr: *mut fault_attr, str_: *mut c_char) -> i32;
    fn fault_create_debugfs_attr(
        name: *const c_char,
        parent: *mut dentry,
        attr: *mut fault_attr,
    ) -> *mut dentry;
    fn should_fail(attr: *mut fault_attr, times: u32) -> bool;
}

#[repr(C)]
struct FailUsercopy {
    attr: fault_attr,
}

// FAULT_ATTR_INITIALIZER is supplied by <linux/fault-inject.h>.
static mut fail_usercopy: FailUsercopy = FailUsercopy {
    attr: fault_attr { _opaque: [] },
};

unsafe extern "C" fn setup_fail_usercopy(str_: *mut c_char) -> i32 {
    setup_fault_attr(
        core::ptr::addr_of_mut!(fail_usercopy.attr),
        str_,
    )
}

// __setup("fail_usercopy=", setup_fail_usercopy);

#[cfg(CONFIG_FAULT_INJECTION_DEBUG_FS)]
unsafe extern "C" fn fail_usercopy_debugfs() -> i32 {
    let dir = fault_create_debugfs_attr(
        c"fail_usercopy".as_ptr(),
        core::ptr::null_mut(),
        core::ptr::addr_of_mut!(fail_usercopy.attr),
    );

    // PTR_ERR_OR_ZERO(dir)
    if dir.is_null() {
        0
    } else {
        0
    }
}

// late_initcall(fail_usercopy_debugfs);

pub unsafe fn should_fail_usercopy() -> bool {
    should_fail(core::ptr::addr_of_mut!(fail_usercopy.attr), 1)
}

// EXPORT_SYMBOL_GPL(should_fail_usercopy);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
