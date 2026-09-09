/* SPDX-License-Identifier: GPL-2.0+ */
/* Sysfs support declarations. */

/* The following kernel types are supplied by the surrounding translation. */

pub const NILFS_ROOT_GROUP_NAME: &str = "nilfs2";

#[repr(C)]
pub struct nilfs_sysfs_dev_subgroups {
    /* /sys/fs/<nilfs>/<device>/superblock */
    pub sg_superblock_kobj: kobject,
    pub sg_superblock_kobj_unregister: completion,

    /* /sys/fs/<nilfs>/<device>/segctor */
    pub sg_segctor_kobj: kobject,
    pub sg_segctor_kobj_unregister: completion,

    /* /sys/fs/<nilfs>/<device>/mounted_snapshots */
    pub sg_mounted_snapshots_kobj: kobject,
    pub sg_mounted_snapshots_kobj_unregister: completion,

    /* /sys/fs/<nilfs>/<device>/checkpoints */
    pub sg_checkpoints_kobj: kobject,
    pub sg_checkpoints_kobj_unregister: completion,

    /* /sys/fs/<nilfs>/<device>/segments */
    pub sg_segments_kobj: kobject,
    pub sg_segments_kobj_unregister: completion,
}

#[repr(C)]
pub struct nilfs_feature_attr {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *mut core::ffi::c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *const core::ffi::c_char, usize) -> isize>,
}

#[repr(C)]
pub struct nilfs_dev_attr {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut nilfs_dev_attr, *mut the_nilfs, *mut core::ffi::c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut nilfs_dev_attr, *mut the_nilfs, *const core::ffi::c_char, usize) -> isize>,
}

#[repr(C)]
pub struct nilfs_segments_attr {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut nilfs_segments_attr, *mut the_nilfs, *mut core::ffi::c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut nilfs_segments_attr, *mut the_nilfs, *const core::ffi::c_char, usize) -> isize>,
}

#[repr(C)]
pub struct nilfs_mounted_snapshots_attr {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut nilfs_mounted_snapshots_attr, *mut the_nilfs, *mut core::ffi::c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut nilfs_mounted_snapshots_attr, *mut the_nilfs, *const core::ffi::c_char, usize) -> isize>,
}

#[repr(C)]
pub struct nilfs_checkpoints_attr {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut nilfs_checkpoints_attr, *mut the_nilfs, *mut core::ffi::c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut nilfs_checkpoints_attr, *mut the_nilfs, *const core::ffi::c_char, usize) -> isize>,
}

#[repr(C)]
pub struct nilfs_superblock_attr {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut nilfs_superblock_attr, *mut the_nilfs, *mut core::ffi::c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut nilfs_superblock_attr, *mut the_nilfs, *const core::ffi::c_char, usize) -> isize>,
}

#[repr(C)]
pub struct nilfs_segctor_attr {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut nilfs_segctor_attr, *mut the_nilfs, *mut core::ffi::c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut nilfs_segctor_attr, *mut the_nilfs, *const core::ffi::c_char, usize) -> isize>,
}

#[repr(C)]
pub struct nilfs_snapshot_attr {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut nilfs_snapshot_attr, *mut nilfs_root, *mut core::ffi::c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut nilfs_snapshot_attr, *mut nilfs_root, *const core::ffi::c_char, usize) -> isize>,
}

/* C token-pasting attribute-construction macros are preserved as Rust macro interfaces. */
macro_rules! NILFS_ATTR { ($($tt:tt)*) => { /* __ATTR(name, mode, show, store) */ }; }
macro_rules! NILFS_INFO_ATTR { ($($tt:tt)*) => { NILFS_ATTR!($($tt)*, 0o444, None, None); }; }
macro_rules! NILFS_RO_ATTR { ($($tt:tt)*) => { NILFS_ATTR!($($tt)*); }; }
macro_rules! NILFS_RW_ATTR { ($($tt:tt)*) => { NILFS_ATTR!($($tt)*); }; }

macro_rules! NILFS_FEATURE_INFO_ATTR { ($($tt:tt)*) => { NILFS_INFO_ATTR!($($tt)*); }; }
macro_rules! NILFS_FEATURE_RO_ATTR { ($($tt:tt)*) => { NILFS_RO_ATTR!($($tt)*); }; }
macro_rules! NILFS_FEATURE_RW_ATTR { ($($tt:tt)*) => { NILFS_RW_ATTR!($($tt)*); }; }
macro_rules! NILFS_DEV_INFO_ATTR { ($($tt:tt)*) => { NILFS_INFO_ATTR!($($tt)*); }; }
macro_rules! NILFS_DEV_RO_ATTR { ($($tt:tt)*) => { NILFS_RO_ATTR!($($tt)*); }; }
macro_rules! NILFS_DEV_RW_ATTR { ($($tt:tt)*) => { NILFS_RW_ATTR!($($tt)*); }; }
macro_rules! NILFS_SEGMENTS_RO_ATTR { ($($tt:tt)*) => { NILFS_RO_ATTR!($($tt)*); }; }
macro_rules! NILFS_SEGMENTS_RW_ATTR { ($($tt:tt)*) => { NILFS_RW_ATTR!(segs_info, $($tt)*); }; }
macro_rules! NILFS_MOUNTED_SNAPSHOTS_RO_ATTR { ($($tt:tt)*) => { NILFS_RO_ATTR!($($tt)*); }; }
macro_rules! NILFS_CHECKPOINTS_RO_ATTR { ($($tt:tt)*) => { NILFS_RO_ATTR!($($tt)*); }; }
macro_rules! NILFS_CHECKPOINTS_RW_ATTR { ($($tt:tt)*) => { NILFS_RW_ATTR!($($tt)*); }; }
macro_rules! NILFS_SNAPSHOT_INFO_ATTR { ($($tt:tt)*) => { NILFS_INFO_ATTR!($($tt)*); }; }
macro_rules! NILFS_SNAPSHOT_RO_ATTR { ($($tt:tt)*) => { NILFS_RO_ATTR!($($tt)*); }; }
macro_rules! NILFS_SNAPSHOT_RW_ATTR { ($($tt:tt)*) => { NILFS_RW_ATTR!($($tt)*); }; }
macro_rules! NILFS_SUPERBLOCK_RO_ATTR { ($($tt:tt)*) => { NILFS_RO_ATTR!($($tt)*); }; }
macro_rules! NILFS_SUPERBLOCK_RW_ATTR { ($($tt:tt)*) => { NILFS_RW_ATTR!($($tt)*); }; }
macro_rules! NILFS_SEGCTOR_INFO_ATTR { ($($tt:tt)*) => { NILFS_INFO_ATTR!($($tt)*); }; }
macro_rules! NILFS_SEGCTOR_RO_ATTR { ($($tt:tt)*) => { NILFS_RO_ATTR!($($tt)*); }; }
macro_rules! NILFS_SEGCTOR_RW_ATTR { ($($tt:tt)*) => { NILFS_RW_ATTR!($($tt)*); }; }

/* Attribute-list macros retain the original pointer-to-member intent. */
macro_rules! NILFS_FEATURE_ATTR_LIST { ($name:ident) => { &nilfs_feature_attr_$name.attr }; }
macro_rules! NILFS_DEV_ATTR_LIST { ($name:ident) => { &nilfs_dev_attr_$name.attr }; }
macro_rules! NILFS_SEGMENTS_ATTR_LIST { ($name:ident) => { &nilfs_segments_attr_$name.attr }; }
macro_rules! NILFS_MOUNTED_SNAPSHOTS_ATTR_LIST { ($name:ident) => { &nilfs_mounted_snapshots_attr_$name.attr }; }
macro_rules! NILFS_CHECKPOINTS_ATTR_LIST { ($name:ident) => { &nilfs_checkpoints_attr_$name.attr }; }
macro_rules! NILFS_SNAPSHOT_ATTR_LIST { ($name:ident) => { &nilfs_snapshot_attr_$name.attr }; }
macro_rules! NILFS_SUPERBLOCK_ATTR_LIST { ($name:ident) => { &nilfs_superblock_attr_$name.attr }; }
macro_rules! NILFS_SEGCTOR_ATTR_LIST { ($name:ident) => { &nilfs_segctor_attr_$name.attr }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
