/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_uint, c_void};

/* The C header includes <linux/debugfs.h>; those declarations are supplied by
 * the surrounding translation unit. */

#[cfg(feature = "CONFIG_GENERIC_IRQ_DEBUGFS")]
#[repr(C)]
pub struct irq_bit_descr {
    pub mask: c_uint,
    pub name: *mut c_char,
}

#[cfg(feature = "CONFIG_GENERIC_IRQ_DEBUGFS")]
#[macro_export]
macro_rules! BIT_MASK_DESCR {
    ($m:expr) => {
        $crate::irq_bit_descr {
            mask: $m,
            name: concat!(stringify!($m), "\0").as_ptr() as *mut core::ffi::c_char,
        }
    };
}

#[cfg(feature = "CONFIG_GENERIC_IRQ_DEBUGFS")]
extern "C" {
    pub fn irq_debug_show_bits(
        m: *mut seq_file,
        ind: c_int,
        state: c_uint,
        sd: *const irq_bit_descr,
        size: c_int,
    );

    pub fn irq_add_debugfs_entry(irq: c_uint, desc: *mut irq_desc);

    pub fn irq_debugfs_copy_devname(irq: c_int, dev: *mut device);
}

#[cfg(feature = "CONFIG_GENERIC_IRQ_DEBUGFS")]
#[inline]
pub unsafe fn irq_remove_debugfs_entry(desc: *mut irq_desc) {
    debugfs_remove((*desc).debugfs_file);
    kfree((*desc).dev_name as *mut c_void);
}

#[cfg(all(feature = "CONFIG_GENERIC_IRQ_DEBUGFS", feature = "CONFIG_IRQ_DOMAIN"))]
extern "C" {
    pub fn irq_domain_debugfs_init(root: *mut dentry);
}

#[cfg(all(feature = "CONFIG_GENERIC_IRQ_DEBUGFS", not(feature = "CONFIG_IRQ_DOMAIN")))]
#[inline]
pub unsafe fn irq_domain_debugfs_init(_root: *mut dentry) {}

/* CONFIG_GENERIC_IRQ_DEBUGFS is a build-time condition from the C header. */
#[cfg(not(feature = "CONFIG_GENERIC_IRQ_DEBUGFS"))]
#[inline]
pub unsafe fn irq_add_debugfs_entry(_irq: c_uint, _d: *mut irq_desc) {}

#[cfg(not(feature = "CONFIG_GENERIC_IRQ_DEBUGFS"))]
#[inline]
pub unsafe fn irq_remove_debugfs_entry(_d: *mut irq_desc) {}

#[cfg(not(feature = "CONFIG_GENERIC_IRQ_DEBUGFS"))]
#[inline]
pub unsafe fn irq_debugfs_copy_devname(_irq: c_int, _dev: *mut device) {}

/* External types and functions supplied by the surrounding translation unit. */
extern "C" {
    pub type seq_file;
    pub type irq_desc;
    pub type device;
    pub type dentry;

    fn debugfs_remove(file: *mut dentry);
    fn kfree(ptr: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
