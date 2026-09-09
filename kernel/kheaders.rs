// SPDX-License-Identifier: GPL-2.0
/*
 * Provide kernel headers useful to build tracing programs
 * such as for running eBPF tracing tools.
 *
 * (Borrowed code from kernel/configs.c)
 */

use core::ffi::c_void;

/* The compressed kernel headers are supplied by the linker/input object. */
unsafe extern "C" {
    static mut kernel_headers_data: u8;
    static mut kernel_headers_data_end: u8;
}

/* Equivalent layout for the fields initialized by __BIN_ATTR_SIMPLE_RO. */
#[repr(C)]
struct bin_attribute {
    pub private: *mut c_void,
    pub size: usize,
}

unsafe extern "C" {
    static mut kernel_kobj: *mut c_void;
    fn sysfs_create_bin_file(
        kobj: *mut c_void,
        attr: *mut bin_attribute,
    ) -> i32;
    fn sysfs_remove_bin_file(
        kobj: *mut c_void,
        attr: *mut bin_attribute,
    );
}

/*
 * The C declaration initializes this read-only-after-init binary attribute
 * with name "kheaders.tar.xz", mode 0444, and read-only operations.
 */
static mut kheaders_attr: bin_attribute = bin_attribute {
    private: core::ptr::null_mut(),
    size: 0,
};

unsafe fn ikheaders_init() -> i32 {
    kheaders_attr.private = core::ptr::addr_of_mut!(kernel_headers_data).cast::<c_void>();
    kheaders_attr.size = (core::ptr::addr_of!(kernel_headers_data_end) as usize)
        .wrapping_sub(core::ptr::addr_of!(kernel_headers_data) as usize);
    sysfs_create_bin_file(kernel_kobj, core::ptr::addr_of_mut!(kheaders_attr))
}

unsafe fn ikheaders_cleanup() {
    sysfs_remove_bin_file(kernel_kobj, core::ptr::addr_of_mut!(kheaders_attr));
}

/* module_init(ikheaders_init); */
/* module_exit(ikheaders_cleanup); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_AUTHOR("Joel Fernandes"); */
/* MODULE_DESCRIPTION("Echo the kernel header artifacts used to build the kernel"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
