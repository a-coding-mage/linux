/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Rust translation of edac_module.h.
 * C includes and header guards are intentionally omitted; referenced types
 * and symbols are supplied by the surrounding translation unit.
 */

extern "C" {
    pub fn edac_mc_sysfs_init() -> ::core::ffi::c_int;
    pub fn edac_mc_sysfs_exit();
    pub fn edac_create_sysfs_mci_device(
        mci: *mut mem_ctl_info,
        groups: *const *const attribute_group,
    ) -> ::core::ffi::c_int;
    pub fn edac_remove_sysfs_mci_device(mci: *mut mem_ctl_info);
    pub fn edac_mc_get_log_ue() -> ::core::ffi::c_int;
    pub fn edac_mc_get_log_ce() -> ::core::ffi::c_int;
    pub fn edac_mc_get_panic_on_ue() -> ::core::ffi::c_int;
    pub fn edac_mc_get_poll_msec() -> ::core::ffi::c_uint;

    pub fn edac_dimm_info_location(
        dimm: *mut dimm_info,
        buf: *mut ::core::ffi::c_char,
        len: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_uint;

    pub fn edac_device_register_sysfs_main_kobj(
        edac_dev: *mut edac_device_ctl_info,
    ) -> ::core::ffi::c_int;
    pub fn edac_device_unregister_sysfs_main_kobj(edac_dev: *mut edac_device_ctl_info);
    pub fn edac_device_create_sysfs(edac_dev: *mut edac_device_ctl_info) -> ::core::ffi::c_int;
    pub fn edac_device_remove_sysfs(edac_dev: *mut edac_device_ctl_info);

    pub fn edac_workqueue_setup() -> ::core::ffi::c_int;
    pub fn edac_workqueue_teardown();
    pub fn edac_queue_work(work: *mut delayed_work, delay: ::core::ffi::c_ulong) -> bool;
    pub fn edac_stop_work(work: *mut delayed_work) -> bool;
    pub fn edac_mod_work(work: *mut delayed_work, delay: ::core::ffi::c_ulong) -> bool;
    pub fn edac_device_reset_delay_period(
        edac_dev: *mut edac_device_ctl_info,
        msec: ::core::ffi::c_ulong,
    );
    pub fn edac_mc_reset_delay_period(value: ::core::ffi::c_ulong);
}

/* EDAC debugfs functions. The CONFIG_EDAC_DEBUG condition is supplied by the build. */
#[cfg(CONFIG_EDAC_DEBUG)]
extern "C" {
    pub fn edac_debugfs_init();
    pub fn edac_debugfs_exit();
    pub fn edac_create_debugfs_nodes(mci: *mut mem_ctl_info);
    pub fn edac_debugfs_create_dir(dirname: *const ::core::ffi::c_char) -> *mut dentry;
    pub fn edac_debugfs_create_dir_at(
        dirname: *const ::core::ffi::c_char,
        parent: *mut dentry,
    ) -> *mut dentry;
    pub fn edac_debugfs_create_file(
        name: *const ::core::ffi::c_char,
        mode: umode_t,
        parent: *mut dentry,
        data: *mut ::core::ffi::c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    pub fn edac_debugfs_create_x8(name: *const ::core::ffi::c_char, mode: umode_t, parent: *mut dentry, value: *mut u8);
    pub fn edac_debugfs_create_x16(name: *const ::core::ffi::c_char, mode: umode_t, parent: *mut dentry, value: *mut u16);
    pub fn edac_debugfs_create_x32(name: *const ::core::ffi::c_char, mode: umode_t, parent: *mut dentry, value: *mut u32);
}

/* CONFIG_EDAC_DEBUG-disabled inline functions are represented as no-op Rust functions. */
#[cfg(not(CONFIG_EDAC_DEBUG))]
pub unsafe fn edac_debugfs_init() {}
#[cfg(not(CONFIG_EDAC_DEBUG))]
pub unsafe fn edac_debugfs_exit() {}
#[cfg(not(CONFIG_EDAC_DEBUG))]
pub unsafe fn edac_create_debugfs_nodes(_mci: *mut mem_ctl_info) {}
#[cfg(not(CONFIG_EDAC_DEBUG))]
pub unsafe fn edac_debugfs_create_dir(_dirname: *const ::core::ffi::c_char) -> *mut dentry { core::ptr::null_mut() }
#[cfg(not(CONFIG_EDAC_DEBUG))]
pub unsafe fn edac_debugfs_create_dir_at(_dirname: *const ::core::ffi::c_char, _parent: *mut dentry) -> *mut dentry { core::ptr::null_mut() }
#[cfg(not(CONFIG_EDAC_DEBUG))]
pub unsafe fn edac_debugfs_create_file(_name: *const ::core::ffi::c_char, _mode: umode_t, _parent: *mut dentry, _data: *mut ::core::ffi::c_void, _fops: *const file_operations) -> *mut dentry { core::ptr::null_mut() }
#[cfg(not(CONFIG_EDAC_DEBUG))]
pub unsafe fn edac_debugfs_create_x8(_name: *const ::core::ffi::c_char, _mode: umode_t, _parent: *mut dentry, _value: *mut u8) {}
#[cfg(not(CONFIG_EDAC_DEBUG))]
pub unsafe fn edac_debugfs_create_x16(_name: *const ::core::ffi::c_char, _mode: umode_t, _parent: *mut dentry, _value: *mut u16) {}
#[cfg(not(CONFIG_EDAC_DEBUG))]
pub unsafe fn edac_debugfs_create_x32(_name: *const ::core::ffi::c_char, _mode: umode_t, _parent: *mut dentry, _value: *mut u32) {}

/* C aliases: edac_debugfs_remove_recursive -> debugfs_remove_recursive,
 * and edac_debugfs_remove -> debugfs_remove. */

#[cfg(CONFIG_PCI)]
extern "C" {
    pub fn edac_pci_do_parity_check();
    pub fn edac_pci_clear_parity_errors();
    pub fn edac_sysfs_pci_setup() -> ::core::ffi::c_int;
    pub fn edac_sysfs_pci_teardown();
    pub fn edac_pci_get_check_errors() -> ::core::ffi::c_int;
    pub fn edac_pci_get_poll_msec() -> ::core::ffi::c_int;
    pub fn edac_pci_remove_sysfs(pci: *mut edac_pci_ctl_info);
    pub fn edac_pci_handle_pe(pci: *mut edac_pci_ctl_info, msg: *const ::core::ffi::c_char);
    pub fn edac_pci_handle_npe(pci: *mut edac_pci_ctl_info, msg: *const ::core::ffi::c_char);
}

/* When CONFIG_PCI is disabled, the C preprocessor removes these calls. */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
