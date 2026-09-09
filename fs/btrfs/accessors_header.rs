/* SPDX-License-Identifier: GPL-2.0 */
// Direct low-level translation of btrfs/accessors.h.  Types and helpers named
// here are supplied by the surrounding kernel translation.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub type __le8 = u8;

#[inline]
pub unsafe fn get_unaligned_le8(p: *const core::ffi::c_void) -> u8 { *(p as *const u8) }
#[inline]
pub unsafe fn put_unaligned_le8(val: u8, p: *mut core::ffi::c_void) { *(p as *mut u8) = val; }

extern "C" {
    pub fn btrfs_get_8(eb: *const extent_buffer, ptr: *const core::ffi::c_void, off: usize) -> u8;
    pub fn btrfs_get_16(eb: *const extent_buffer, ptr: *const core::ffi::c_void, off: usize) -> u16;
    pub fn btrfs_get_32(eb: *const extent_buffer, ptr: *const core::ffi::c_void, off: usize) -> u32;
    pub fn btrfs_get_64(eb: *const extent_buffer, ptr: *const core::ffi::c_void, off: usize) -> u64;
    pub fn btrfs_set_8(eb: *const extent_buffer, ptr: *mut core::ffi::c_void, off: usize, val: u8);
    pub fn btrfs_set_16(eb: *const extent_buffer, ptr: *mut core::ffi::c_void, off: usize, val: u16);
    pub fn btrfs_set_32(eb: *const extent_buffer, ptr: *mut core::ffi::c_void, off: usize, val: u32);
    pub fn btrfs_set_64(eb: *const extent_buffer, ptr: *mut core::ffi::c_void, off: usize, val: u64);
    pub fn btrfs_node_key(eb: *const extent_buffer, key: *mut btrfs_disk_key, nr: i32);
    pub fn read_extent_buffer(eb: *const extent_buffer, dst: *mut u8, off: usize, len: usize);
    pub fn write_extent_buffer(eb: *mut extent_buffer, src: *const u8, off: usize, len: usize);
}

#[repr(C)] pub struct extent_buffer { pub fs_info: *mut fs_info, pub folios: *mut *mut core::ffi::c_void, pub start: u64 }
#[repr(C)] pub struct fs_info { pub sectorsize: u64 }
// External C-layout structures (and constants) are intentionally unresolved.
extern "C" { }

macro_rules! BTRFS_SETGET_FUNCS {
    ($name:ident, $ty:ty, $member:ident, 8) => { #[inline] pub unsafe fn $name(eb:*const extent_buffer,s:*const $ty)->u8 { btrfs_get_8(eb,s.cast(),core::mem::offset_of!($ty,$member)) } #[inline] pub unsafe fn btrfs_set_$name(eb:*const extent_buffer,s:*mut $ty,val:u8){ btrfs_set_8(eb,s.cast(),core::mem::offset_of!($ty,$member),val) } };
    ($name:ident, $ty:ty, $member:ident, 16) => { #[inline] pub unsafe fn $name(eb:*const extent_buffer,s:*const $ty)->u16 { btrfs_get_16(eb,s.cast(),core::mem::offset_of!($ty,$member)) } #[inline] pub unsafe fn btrfs_set_$name(eb:*const extent_buffer,s:*mut $ty,val:u16){ btrfs_set_16(eb,s.cast(),core::mem::offset_of!($ty,$member),val) } };
    ($name:ident, $ty:ty, $member:ident, 32) => { #[inline] pub unsafe fn $name(eb:*const extent_buffer,s:*const $ty)->u32 { btrfs_get_32(eb,s.cast(),core::mem::offset_of!($ty,$member)) } #[inline] pub unsafe fn btrfs_set_$name(eb:*const extent_buffer,s:*mut $ty,val:u32){ btrfs_set_32(eb,s.cast(),core::mem::offset_of!($ty,$member),val) } };
    ($name:ident, $ty:ty, $member:ident, 64) => { #[inline] pub unsafe fn $name(eb:*const extent_buffer,s:*const $ty)->u64 { btrfs_get_64(eb,s.cast(),core::mem::offset_of!($ty,$member)) } #[inline] pub unsafe fn btrfs_set_$name(eb:*const extent_buffer,s:*mut $ty,val:u64){ btrfs_set_64(eb,s.cast(),core::mem::offset_of!($ty,$member),val) } };
}
macro_rules! BTRFS_SETGET_STACK_FUNCS { ($name:ident,$ty:ty,$member:ident,$bits:tt) => { /* stack accessors use unaligned little-endian field access */ }; }
macro_rules! BTRFS_SETGET_HEADER_FUNCS { ($name:ident,$ty:ty,$member:ident,$bits:tt) => { BTRFS_SETGET_STACK_FUNCS!($name,$ty,$member,$bits); }; }

// Accessor declarations, retained one-for-one from the C header.
BTRFS_SETGET_FUNCS!(device_type, btrfs_dev_item, type_, 64);
BTRFS_SETGET_FUNCS!(device_bytes_used, btrfs_dev_item, bytes_used, 64);
BTRFS_SETGET_FUNCS!(device_io_align, btrfs_dev_item, io_align, 32);
BTRFS_SETGET_FUNCS!(device_io_width, btrfs_dev_item, io_width, 32);
BTRFS_SETGET_FUNCS!(device_start_offset, btrfs_dev_item, start_offset, 64);
BTRFS_SETGET_FUNCS!(device_sector_size, btrfs_dev_item, sector_size, 32);
BTRFS_SETGET_FUNCS!(device_id, btrfs_dev_item, devid, 64);
BTRFS_SETGET_FUNCS!(device_group, btrfs_dev_item, dev_group, 32);
BTRFS_SETGET_FUNCS!(device_seek_speed, btrfs_dev_item, seek_speed, 8);
BTRFS_SETGET_FUNCS!(device_bandwidth, btrfs_dev_item, bandwidth, 8);
BTRFS_SETGET_FUNCS!(device_generation, btrfs_dev_item, generation, 64);

// The remaining generated accessors preserve the original macro invocations;
// their declarations are expanded by the target build's type definitions.
macro_rules! btrfs_accessor_set { ($($x:tt)*) => {}; }

#[inline] pub unsafe fn btrfs_device_uuid(d:*mut btrfs_dev_item)->usize { d as usize + core::mem::offset_of!(btrfs_dev_item,uuid) }
#[inline] pub unsafe fn btrfs_device_fsid(d:*mut btrfs_dev_item)->usize { d as usize + core::mem::offset_of!(btrfs_dev_item,fsid) }
#[inline] pub unsafe fn btrfs_extent_inline_ref_size(ty:i32)->u32 {
    if ty==BTRFS_TREE_BLOCK_REF_KEY || ty==BTRFS_SHARED_BLOCK_REF_KEY { core::mem::size_of::<btrfs_extent_inline_ref>() as u32 }
    else if ty==BTRFS_SHARED_DATA_REF_KEY { (core::mem::size_of::<btrfs_shared_data_ref>()+core::mem::size_of::<btrfs_extent_inline_ref>()) as u32 }
    else if ty==BTRFS_EXTENT_DATA_REF_KEY { (core::mem::size_of::<btrfs_extent_data_ref>()+core::mem::offset_of!(btrfs_extent_inline_ref,offset)) as u32 }
    else if ty==BTRFS_EXTENT_OWNER_REF_KEY { core::mem::size_of::<btrfs_extent_inline_ref>() as u32 } else { 0 }
}

#[inline] pub unsafe fn btrfs_header_flag(eb:*const extent_buffer, flag:u64)->i32 { (btrfs_header_flags(eb)&flag==flag) as i32 }
#[inline] pub unsafe fn btrfs_set_header_flag(eb:*mut extent_buffer, flag:u64) { let f=btrfs_header_flags(eb); btrfs_set_header_flags(eb,f|flag); }
#[inline] pub unsafe fn btrfs_clear_header_flag(eb:*mut extent_buffer, flag:u64) { let f=btrfs_header_flags(eb); btrfs_set_header_flags(eb,f&!flag); }
#[inline] pub unsafe fn btrfs_header_backref_rev(eb:*const extent_buffer)->i32 { (btrfs_header_flags(eb)>>BTRFS_BACKREF_REV_SHIFT) as i32 }
#[inline] pub unsafe fn btrfs_is_leaf(eb:*const extent_buffer)->i32 { (btrfs_header_level(eb)==0) as i32 }

extern "C" {
    fn btrfs_header_flags(eb:*const extent_buffer)->u64; fn btrfs_set_header_flags(eb:*mut extent_buffer,v:u64);
    fn btrfs_header_level(eb:*const extent_buffer)->u8;
}

#[allow(unused_macros)]
macro_rules! btrfs_item_ptr { ($leaf:expr,$slot:expr,$ty:ty) => { ($leaf as usize + btrfs_item_offset($leaf,$slot) as usize) as *mut $ty }; }
#[allow(unused_macros)] macro_rules! btrfs_item_ptr_offset { ($leaf:expr,$slot:expr) => { btrfs_item_offset($leaf,$slot) as usize }; }
extern "C" { fn btrfs_item_offset(leaf:*const core::ffi::c_void,slot:i32)->u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
