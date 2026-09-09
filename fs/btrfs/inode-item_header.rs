/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by other translation units:
// linux/types.h, linux/crc32c.h

pub enum fscrypt_str {}
pub enum extent_buffer {}
pub enum btrfs_trans_handle {}
pub enum btrfs_root {}
pub enum btrfs_path {}
pub enum btrfs_key {}
pub enum btrfs_inode_extref {}
pub enum btrfs_inode_ref {}
pub enum btrfs_inode {}

/*
 * Return this if we need to call truncate_block for the last bit of the
 * truncate.
 */
pub const BTRFS_NEED_TRUNCATE_BLOCK: i32 = 1;

#[repr(C)]
pub struct btrfs_truncate_control {
	/*
	 * IN: the inode we're operating on, this can be NULL if
	 * ->clear_extent_range is false.
	 */
	pub inode: *mut btrfs_inode,

	/* IN: the size we're truncating to. */
	pub new_size: u64,

	/* OUT: the number of extents truncated. */
	pub extents_found: u64,

	/* OUT: the last size we truncated this inode to. */
	pub last_size: u64,

	/* OUT: the number of bytes to sub from this inode. */
	pub sub_bytes: u64,

	/* IN: the ino we are truncating. */
	pub ino: u64,

	/*
	 * IN: minimum key type to remove.  All key types with this type are
	 * removed only if their offset >= new_size.
	 */
	pub min_type: u32,

	/*
	 * IN: true if we don't want to do extent reference updates for any file
	 * extents we drop.
	 */
	pub skip_ref_updates: bool,

	/*
	 * IN: true if we need to clear the file extent range for the inode as
	 * we drop the file extent items.
	 */
	pub clear_extent_range: bool,
}

/*
 * btrfs_inode_item stores flags in a u64, btrfs_inode stores them in two
 * separate u32s. These two functions convert between the two representations.
 */
#[inline]
pub fn btrfs_inode_combine_flags(flags: u32, ro_flags: u32) -> u64 {
	flags as u64 | ((ro_flags as u64) << 32)
}

#[inline]
pub unsafe fn btrfs_inode_split_flags(
	inode_item_flags: u64,
	flags: *mut u32,
	ro_flags: *mut u32,
) {
	*flags = inode_item_flags as u32;
	*ro_flags = (inode_item_flags >> 32) as u32;
}

/* Figure the key offset of an extended inode ref. */
unsafe extern "C" {
	fn crc32c(crc: u32, address: *const core::ffi::c_void, length: isize) -> u32;
}

#[inline]
pub unsafe fn btrfs_extref_hash(parent_objectid: u64, name: *const i8, len: i32) -> u64 {
	crc32c(parent_objectid as u32, name as *const core::ffi::c_void, len as isize) as u64
}

unsafe extern "C" {
	pub fn btrfs_truncate_inode_items(
		trans: *mut btrfs_trans_handle,
		root: *mut btrfs_root,
		control: *mut btrfs_truncate_control,
	) -> i32;
	pub fn btrfs_insert_inode_ref(
		trans: *mut btrfs_trans_handle,
		root: *mut btrfs_root,
		name: *const fscrypt_str,
		inode_objectid: u64,
		ref_objectid: u64,
		index: u64,
	) -> i32;
	pub fn btrfs_del_inode_ref(
		trans: *mut btrfs_trans_handle,
		root: *mut btrfs_root,
		name: *const fscrypt_str,
		inode_objectid: u64,
		ref_objectid: u64,
		index: *mut u64,
	) -> i32;
	pub fn btrfs_insert_empty_inode(
		trans: *mut btrfs_trans_handle,
		root: *mut btrfs_root,
		path: *mut btrfs_path,
		objectid: u64,
	) -> i32;
	pub fn btrfs_lookup_inode(
		trans: *mut btrfs_trans_handle,
		root: *mut btrfs_root,
		path: *mut btrfs_path,
		location: *mut btrfs_key,
		mod_: i32,
	) -> i32;
	pub fn btrfs_lookup_inode_extref(
		root: *mut btrfs_root,
		path: *mut btrfs_path,
		name: *const fscrypt_str,
		inode_objectid: u64,
		ref_objectid: u64,
	) -> *mut btrfs_inode_extref;
	pub fn btrfs_find_name_in_backref(
		leaf: *const extent_buffer,
		slot: i32,
		name: *const fscrypt_str,
	) -> *mut btrfs_inode_ref;
	pub fn btrfs_find_name_in_ext_backref(
		leaf: *const extent_buffer,
		slot: i32,
		ref_objectid: u64,
		name: *const fscrypt_str,
	) -> *mut btrfs_inode_extref;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
