/* SPDX-License-Identifier: GPL-2.0 */

// Declarations corresponding to the C header. Types and symbols supplied by
// other headers or translation units are intentionally left external.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct block_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_fs_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_fs_devices {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_block_group {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_space_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_qgroup {
    _private: [u8; 0],
}

// Supplied by the kernel kobject declarations.
#[repr(C)]
pub enum kobject_action {
    _KobjectAction = 0,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum btrfs_feature_set {
    FEAT_COMPAT = 0,
    FEAT_COMPAT_RO,
    FEAT_INCOMPAT,
    FEAT_MAX,
}

extern "C" {
    pub fn btrfs_printable_features(
        set: btrfs_feature_set,
        flags: u64,
    ) -> *mut c_char;
    pub fn btrfs_feature_set_name(set: btrfs_feature_set) -> *const c_char;
    pub fn btrfs_sysfs_add_device(device: *mut btrfs_device) -> c_int;
    pub fn btrfs_sysfs_remove_device(device: *mut btrfs_device);
    pub fn btrfs_sysfs_add_fsid(fs_devs: *mut btrfs_fs_devices) -> c_int;
    pub fn btrfs_sysfs_remove_fsid(fs_devs: *mut btrfs_fs_devices);
    pub fn btrfs_sysfs_update_sprout_fsid(fs_devices: *mut btrfs_fs_devices);
    pub fn btrfs_sysfs_feature_update(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_kobject_uevent(bdev: *mut block_device, action: kobject_action);

    // `__init` attribute in the C declaration.
    pub fn btrfs_init_sysfs() -> c_int;
    // `__cold` attribute in the C declaration.
    pub fn btrfs_exit_sysfs();
    pub fn btrfs_sysfs_add_mounted(fs_info: *mut btrfs_fs_info) -> c_int;
    pub fn btrfs_sysfs_remove_mounted(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_sysfs_add_block_group_type(cache: *mut btrfs_block_group);
    pub fn btrfs_sysfs_add_space_info_type(space_info: *mut btrfs_space_info) -> c_int;
    pub fn btrfs_sysfs_remove_space_info(space_info: *mut btrfs_space_info);
    pub fn btrfs_sysfs_update_devid(device: *mut btrfs_device);

    pub fn btrfs_sysfs_add_one_qgroup(
        fs_info: *mut btrfs_fs_info,
        qgroup: *mut btrfs_qgroup,
    ) -> c_int;
    pub fn btrfs_sysfs_del_qgroups(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_sysfs_add_qgroups(fs_info: *mut btrfs_fs_info) -> c_int;
    pub fn btrfs_sysfs_del_one_qgroup(
        fs_info: *mut btrfs_fs_info,
        qgroup: *mut btrfs_qgroup,
    );
    pub fn btrfs_read_policy_to_enum(str_: *const c_char, value: *mut i64) -> c_int;

    // CONFIG_BTRFS_EXPERIMENTAL conditional declarations.
    #[cfg(CONFIG_BTRFS_EXPERIMENTAL)]
    pub fn btrfs_read_policy_init() -> c_int;
    #[cfg(CONFIG_BTRFS_EXPERIMENTAL)]
    pub fn btrfs_get_mod_read_policy() -> *mut c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
