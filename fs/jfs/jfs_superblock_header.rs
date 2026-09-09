/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines Corp., 2000-2003
 */

// Dependency supplied externally: linux/uuid.h

/*
 * make the magic number something a human could read
 */
pub const JFS_MAGIC: &[u8; 4] = b"JFS1"; /* Magic word */

pub const JFS_VERSION: i32 = 2; /* Version number: Version 2 */

pub const LV_NAME_SIZE: usize = 11; /* MUST BE 11 for OS/2 boot sector */

/*
 *	aggregate superblock
 *
 * The name superblock is too close to super_block, so the name has been
 * changed to jfs_superblock.  The utilities are still using the old name.
 */
#[repr(C)]
pub struct jfs_superblock {
	pub s_magic: [core::ffi::c_char; 4], /* 4: magic number */
	pub s_version: __le32, /* 4: version number */

	pub s_size: __le64, /* 8: aggregate size in hardware/LVM blocks;
					 * VFS: number of blocks
					 */
	pub s_bsize: __le32, /* 4: aggregate block size in bytes;
					 * VFS: fragment size
					 */
	pub s_l2bsize: __le16, /* 2: log2 of s_bsize */
	pub s_l2bfactor: __le16, /* 2: log2(s_bsize/hardware block size) */
	pub s_pbsize: __le32, /* 4: hardware/LVM block size in bytes */
	pub s_l2pbsize: __le16, /* 2: log2 of s_pbsize */
	pub pad: __le16, /* 2: padding necessary for alignment */

	pub s_agsize: __le32, /* 4: allocation group size in aggr. blocks */

	pub s_flag: __le32, /* 4: aggregate attributes:
					 *    see jfs_filsys.h
					 */
	pub s_state: __le32, /* 4: mount/unmount/recovery state:
					 *    see jfs_filsys.h
					 */
	pub s_compress: __le32, /* 4: > 0 if data compression */

	pub s_ait2: pxd_t, /* 8: first extent of secondary
					 *    aggregate inode table
					 */

	pub s_aim2: pxd_t, /* 8: first extent of secondary
					 *    aggregate inode map
					 */
	pub s_logdev: __le32, /* 4: device address of log */
	pub s_logserial: __le32, /* 4: log serial number at aggregate mount */
	pub s_logpxd: pxd_t, /* 8: inline log extent */

	pub s_fsckpxd: pxd_t, /* 8: inline fsck work space extent */

	pub s_time: timestruc_t, /* 8: time last updated */

	pub s_fsckloglen: __le32, /* 4: Number of filesystem blocks reserved for
					 *    the fsck service log.
					 *    N.B. These blocks are divided among the
					 *         versions kept.  This is not a per
					 *         version size.
					 *    N.B. These blocks are included in the
					 *         length field of s_fsckpxd.
					 */
	pub s_fscklog: s8, /* 1: which fsck service log is most recent
					 *    0 => no service log data yet
					 *    1 => the first one
					 *    2 => the 2nd one
					 */
	pub s_fpack: [core::ffi::c_char; 11], /* 11: file system volume name
					 *     N.B. This must be 11 bytes to
					 *          conform with the OS/2 BootSector
					 *          requirements
					 *          Only used when s_version is 1
					 */

	/* extendfs() parameter under s_state & FM_EXTENDFS */
	pub s_xsize: __le64, /* 8: extendfs s_size */
	pub s_xfsckpxd: pxd_t, /* 8: extendfs fsckpxd */
	pub s_xlogpxd: pxd_t, /* 8: extendfs logpxd */
	pub s_uuid: uuid_t, /* 16: 128-bit uuid for volume */
	pub s_label: [core::ffi::c_char; 16], /* 16: volume label */
	pub s_loguuid: uuid_t, /* 16: 128-bit uuid for log device */
}

unsafe extern "C" {
	pub fn readSuper(sb: *mut super_block, bpp: *mut *mut buffer_head) -> i32;
	pub fn updateSuper(sb: *mut super_block, flag: uint) -> i32;
	pub fn jfs_error(sb: *mut super_block, fmt: *const core::ffi::c_char, ...);
	pub fn jfs_mount(sb: *mut super_block) -> i32;
	pub fn jfs_mount_rw(sb: *mut super_block, rw: i32) -> i32;
	pub fn jfs_umount(sb: *mut super_block) -> i32;
	pub fn jfs_umount_rw(sb: *mut super_block, rw: i32) -> i32;
	pub fn jfs_extendfs(sb: *mut super_block, new_size: s64, flag: i32) -> i32;

	pub static mut jfsIOthread: *mut task_struct;
	pub static mut jfsSyncThread: *mut task_struct;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
