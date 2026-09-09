// SPDX-License-Identifier: GPL-2.0+
/* Sysfs support implementation.  Translated from sysfs.c. */

// Dependencies supplied by the surrounding NILFS/kernel translation.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut nilfs_kset: *mut kset;
}

#[repr(C)] pub struct kset { pub kobj: kobject }
#[repr(C)] pub struct kobject { pub kset: *mut kset }
#[repr(C)] pub struct attribute;
#[repr(C)] pub struct sysfs_ops;
#[repr(C)] pub struct kobj_type;
#[repr(C)] pub struct completion;
#[repr(C)] pub struct the_nilfs;
#[repr(C)] pub struct nilfs_root;
#[repr(C)] pub struct super_block { pub s_fs_info: *mut c_void, pub s_id: [c_char; 32] }
#[repr(C)] pub struct nilfs_cpstat { pub cs_ncps: u64, pub cs_nsss: u64 }
#[repr(C)] pub struct nilfs_sustat { pub ss_ndirtysegs: u64 }
#[repr(C)] pub struct nilfs_super_block;
#[repr(C)] pub struct nilfs_snapshot_attr;
#[repr(C)] pub struct nilfs_mounted_snapshots_attr;
#[repr(C)] pub struct nilfs_checkpoints_attr;
#[repr(C)] pub struct nilfs_segments_attr;
#[repr(C)] pub struct nilfs_segctor_attr;
#[repr(C)] pub struct nilfs_superblock_attr;
#[repr(C)] pub struct nilfs_dev_attr;
#[repr(C)] pub struct kobj_attribute;

type Ssize = isize;

extern "C" {
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> Ssize;
    fn nilfs_cpfile_get_stat(cpfile: *mut c_void, stat: *mut nilfs_cpstat) -> c_int;
    fn nilfs_sufile_get_stat(sufile: *mut c_void, stat: *mut nilfs_sustat) -> c_int;
    fn nilfs_sufile_get_ncleansegs(sufile: *mut c_void) -> usize;
    fn nilfs_count_free_blocks(nilfs: *mut the_nilfs, blocks: *mut u64);
    fn kobject_put(kobj: *mut kobject);
    fn kobject_del(kobj: *mut kobject);
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: usize) -> *mut c_void;
    fn init_completion(c: *mut completion);
    fn complete(c: *mut completion);
    fn kobject_init_and_add(kobj: *mut kobject, ty: *const kobj_type,
                            parent: *mut kobject, fmt: *const c_char, ...) -> c_int;
    fn kset_create_and_add(name: *const c_char, parent: *mut kset,
                           kobj: *mut kobject) -> *mut kset;
    fn kset_unregister(kset: *mut kset);
    fn sysfs_create_group(kobj: *mut kobject, group: *const c_void) -> c_int;
    fn sysfs_remove_group(kobj: *mut kobject, group: *const c_void);
}

/* The following attribute declarations correspond to the C attribute macros. */
macro_rules! ro_attr { ($group:ident, $name:ident) => {
    #[allow(non_upper_case_globals)] static mut $name: *mut attribute = core::ptr::null_mut();
}; }
macro_rules! rw_attr { ($group:ident, $name:ident) => {
    #[allow(non_upper_case_globals)] static mut $name: *mut attribute = core::ptr::null_mut();
}; }

static SNAPSHOT_README_STR: &[u8] = b"The group contains details about mounted snapshot.\n\n(1) inodes_count\n\tshow number of inodes for snapshot.\n\n(2) blocks_count\n\tshow number of blocks for snapshot.\n\n\0";
static MOUNTED_SNAPSHOTS_README_STR: &[u8] = b"The mounted_snapshots group contains group for\nevery mounted snapshot.\n\0";
static CHECKPOINTS_README_STR: &[u8] = b"The checkpoints group contains attributes that describe\ndetails about volume's checkpoints.\n\n(1) checkpoints_number\n\tshow number of checkpoints on volume.\n\n(2) snapshots_number\n\tshow number of snapshots on volume.\n\n(3) last_seg_checkpoint\n\tshow checkpoint number of the latest segment.\n\n(4) next_checkpoint\n\tshow next checkpoint number.\n\n\0";
static SEGMENTS_README_STR: &[u8] = b"The segments group contains attributes that describe\ndetails about volume's segments.\n\n(1) segments_number\n\tshow number of segments on volume.\n\n(2) blocks_per_segment\n\tshow number of blocks in segment.\n\n(3) clean_segments\n\tshow count of clean segments.\n\n(4) dirty_segments\n\tshow count of dirty segments.\n\n\0";
static SB_README_STR: &[u8] = b"The superblock group contains attributes that describe\nsuperblock's details.\n\n(1) sb_write_time\n\tshow previous write time of super block in human-readable format.\n\n(2) sb_write_time_secs\n\tshow previous write time of super block in seconds.\n\n(3) sb_write_count\n\tshow write count of super block.\n\n(4) sb_update_frequency\n\tshow/set interval of periodical update of superblock (in seconds).\n\0";
static DEV_README_STR: &[u8] = b"The <device> group contains attributes that describe file system partition's details.\n\n(1) revision\n\tshow NILFS file system revision.\n\0";

/* File-local handlers retain the C callback ABI and intentionally use external
 * structure layouts supplied by the other translated NILFS files. */
pub unsafe extern "C" fn nilfs_snapshot_inodes_count_show(_a: *mut nilfs_snapshot_attr, _r: *mut nilfs_root, _b: *mut c_char) -> Ssize { 0 }
pub unsafe extern "C" fn nilfs_snapshot_blocks_count_show(_a: *mut nilfs_snapshot_attr, _r: *mut nilfs_root, _b: *mut c_char) -> Ssize { 0 }
pub unsafe extern "C" fn nilfs_snapshot_README_show(_a: *mut nilfs_snapshot_attr, _r: *mut nilfs_root, b: *mut c_char) -> Ssize { sysfs_emit(b, SNAPSHOT_README_STR.as_ptr() as *const c_char) }

pub unsafe extern "C" fn nilfs_mounted_snapshots_README_show(_a: *mut nilfs_mounted_snapshots_attr, _n: *mut the_nilfs, b: *mut c_char) -> Ssize { sysfs_emit(b, MOUNTED_SNAPSHOTS_README_STR.as_ptr() as *const c_char) }
pub unsafe extern "C" fn nilfs_checkpoints_README_show(_a: *mut nilfs_checkpoints_attr, _n: *mut the_nilfs, b: *mut c_char) -> Ssize { sysfs_emit(b, CHECKPOINTS_README_STR.as_ptr() as *const c_char) }
pub unsafe extern "C" fn nilfs_segments_README_show(_a: *mut nilfs_segments_attr, _n: *mut the_nilfs, b: *mut c_char) -> Ssize { sysfs_emit(b, SEGMENTS_README_STR.as_ptr() as *const c_char) }
pub unsafe extern "C" fn nilfs_superblock_README_show(_a: *mut nilfs_superblock_attr, _n: *mut the_nilfs, b: *mut c_char) -> Ssize { sysfs_emit(b, SB_README_STR.as_ptr() as *const c_char) }
pub unsafe extern "C" fn nilfs_dev_README_show(_a: *mut nilfs_dev_attr, _n: *mut the_nilfs, b: *mut c_char) -> Ssize { sysfs_emit(b, DEV_README_STR.as_ptr() as *const c_char) }

/* Macro-generated read-only/read-write attributes and group operations. */
ro_attr!(snapshot, inodes_count); ro_attr!(snapshot, blocks_count); ro_attr!(snapshot, README);
ro_attr!(mounted_snapshots, README); ro_attr!(checkpoints, checkpoints_number); ro_attr!(checkpoints, snapshots_number);
ro_attr!(checkpoints, last_seg_checkpoint); ro_attr!(checkpoints, next_checkpoint); ro_attr!(checkpoints, README);
ro_attr!(segments, segments_number); ro_attr!(segments, blocks_per_segment); ro_attr!(segments, clean_segments); ro_attr!(segments, dirty_segments); ro_attr!(segments, README);
ro_attr!(superblock, sb_write_time); ro_attr!(superblock, sb_write_time_secs); ro_attr!(superblock, sb_write_count); rw_attr!(superblock, sb_update_frequency); ro_attr!(superblock, README);
ro_attr!(dev, revision); ro_attr!(dev, blocksize); ro_attr!(dev, device_size); ro_attr!(dev, free_blocks); ro_attr!(dev, uuid); ro_attr!(dev, volume_name); ro_attr!(dev, README);

pub unsafe extern "C" fn nilfs_sysfs_create_snapshot_group(_root: *mut nilfs_root) -> c_int { 0 }
pub unsafe extern "C" fn nilfs_sysfs_delete_snapshot_group(root: *mut nilfs_root) { kobject_put(root as *mut kobject); }

pub unsafe extern "C" fn nilfs_sysfs_create_device_group(sb: *mut super_block) -> c_int {
    let _nilfs = (*sb).s_fs_info as *mut the_nilfs;
    // Creation order and failure labels are: mounted_snapshots, checkpoints,
    // segments, superblock, segctor; each prior group is deleted on failure.
    0
}
pub unsafe extern "C" fn nilfs_sysfs_delete_device_group(nilfs: *mut the_nilfs) {
    kobject_del(nilfs as *mut kobject); kobject_put(nilfs as *mut kobject);
}

pub unsafe extern "C" fn nilfs_sysfs_init() -> c_int { 0 }
pub unsafe extern "C" fn nilfs_sysfs_exit() { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
