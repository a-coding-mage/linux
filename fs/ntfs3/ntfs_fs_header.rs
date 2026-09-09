/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of ntfs_fs.h. Linux/kernel dependencies are intentionally external.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
use core::{ffi::c_void, mem::size_of, ptr};

pub type u8_=u8; pub type u16_=u16; pub type u32_=u32; pub type u64_=u64;
pub type s32_=i32; pub type s64_=i64; pub type CLST=u64; pub type loff_t=i64;
pub type __le16=u16; pub type __le32=u32; pub type __le64=u64; pub type umode_t=u32; pub type dev_t=u64; pub type sector_t=u64;
pub type ulong=usize;
macro_rules! opaque { ($($n:ident),* $(,)?) => { $(#[repr(C)] pub struct $n { _private:[u8;0] })* }; }
opaque!(dentry,fiemap_extent_info,user_namespace,page,writeback_control,nls_table,super_block,rw_semaphore,rb_root,rb_node,buffer_head,
ATTR_DEF_ENTRY,MFT_REC,ntfs_inode,ntfs_run,lznt,xpress_decompressor,lzx_decompressor,proc_dir_entry,mutex,ratelimit_state,inode,
ATTRIB,ATTR_LIST_ENTRY,folio,ATTR_STD_INFO,ATTR_STD_INFO5,NTFS_DE,INDEX_BUFFER,INDEX_HDR,NTFS_RECORD_HEADER,SECURITY_DESCRIPTOR_RELATIVE,
GUID,MFT_REF,cpu_str,le_str,REPARSE_DATA_BUFFER,INDEX_ROOT,NTFS_DUP_INFO, qstr, iov_iter, file_ra_state, file_kattr,mnt_idmap,path,kstat,iattr,file,
writeback_control,iomap_ops,iomap_write_ops,inode_operations,address_space_operations,file_operations,dentry_operations,xattr_handler,posix_acl,fstrim_range);
pub type file_operations = file_operations; // dependency placeholder identity
pub type FILE_ATTRIBUTE=u32; pub type ATTR_TYPE=u32; pub type RECORD_FLAG=u16;
extern "C" { pub fn kzalloc_obj<T>(_:usize,_:u32)->*mut T; pub fn kfree(_: *mut c_void); pub fn kvfree(_: *mut c_void); }
pub const MINUS_ONE_T:usize=usize::MAX;
pub const MAXIMUM_BYTES_PER_MFT:usize=4096; pub const MAXIMUM_SHIFT_BYTES_PER_MFT:u32=12; pub const NTFS_BLOCKS_PER_MFT_RECORD:usize=8;
pub const MAXIMUM_BYTES_PER_INDEX:usize=4096; pub const MAXIMUM_SHIFT_BYTES_PER_INDEX:u32=12; pub const NTFS_BLOCKS_PER_INODE:usize=8;
pub const E_NTFS_FIXUP:i32=555; pub const E_NTFS_NONRESIDENT:i32=556; pub const E_NTFS_NOTALIGNED:i32=557; pub const E_NTFS_CORRUPT:i32=558;
pub const NTFS_FLAGS_NODISCARD:usize=1; pub const NTFS_FLAGS_SHUTDOWN_BIT:usize=2; pub const NTFS_FLAGS_LOG_REPLAYING:usize=8; pub const NTFS_FLAGS_MFTMIRR:usize=0x1000; pub const NTFS_FLAGS_NEED_REPLAY:usize=0x04000000;
pub const NI_FLAG_COMPRESSED_MASK:usize=0xf; pub const NI_FLAG_DEDUPLICATED:usize=0x10; pub const NI_FLAG_EA:usize=0x20; pub const NI_FLAG_DIR:usize=0x40; pub const NI_FLAG_RESIDENT:usize=0x80; pub const NI_FLAG_UPDATE_PARENT:usize=0x100;

#[repr(C)] pub struct ntfs_mount_options { pub nls_name:*mut i8,pub nls:*mut nls_table,pub fs_uid:u32,pub fs_gid:u32,pub fs_fmask_inv:u16,pub fs_dmask_inv:u16,pub flags:u32 }
#[repr(C)] pub struct runs_tree { pub runs:*mut ntfs_run,pub count:usize,pub allocated:usize }
#[repr(C)] pub struct ntfs_buffers { pub bh:[*mut buffer_head;8],pub bytes:u32,pub nbufs:u32,pub off:u32 }
#[repr(C)] pub struct wnd_bitmap { pub sb:*mut super_block,pub rw_lock:rw_semaphore,pub run:runs_tree,pub nbits:usize,pub total_zeroes:usize,pub free_bits:*mut u16,pub nwnd:usize,pub bits_last:u32,pub start_tree:rb_root,pub count_tree:rb_root,pub count:usize,pub uptodated:i32,pub extent_min:usize,pub extent_max:usize,pub zone_bit:usize,pub zone_end:usize,pub inited:bool }
pub type NTFS_CMP_FUNC=unsafe extern "C" fn(*const c_void,usize,*const c_void,usize,*const c_void)->i32;
#[repr(C)] pub struct ntfs_index { pub bitmap_run:runs_tree,pub alloc_run:runs_tree,pub run_lock:rw_semaphore,pub version:usize,pub index_bits:u8,pub idx2vbn_bits:u8,pub vbn2vbo_bits:u8,pub typ:u8 }
#[repr(C)] pub struct mft_inode { pub node:rb_node,pub sbi:*mut ntfs_sb_info,pub mrec:*mut MFT_REC,pub nb:ntfs_buffers,pub rno:CLST,pub dirty:bool }
#[repr(C)] pub struct ntfs_inode { pub mi:mft_inode,pub i_valid:u64,pub i_crtime:[u8;16],pub ni_lock:mutex,pub std_fa:FILE_ATTRIBUTE,pub std_security_id:__le32,pub mi_tree:rb_root,pub mi_loaded:u8,pub ni_bad:u8,pub nodump:u8,pub dir:ntfs_index,pub attr_list:runs_tree,pub ni_flags:usize,pub base:*mut ntfs_inode,pub vfs_inode:inode }
#[repr(C)] pub struct indx_node { pub nb:ntfs_buffers,pub index:*mut INDEX_BUFFER }
#[repr(C)] pub struct ntfs_fnd { pub level:i32,pub nodes:[*mut indx_node;20],pub de:[*mut NTFS_DE;20],pub root_de:*mut NTFS_DE }
#[repr(C)] pub struct ntfs_sb_info { pub sb:*mut super_block,pub discard_granularity:u32,pub discard_granularity_mask_inv:u64,pub bdev_blocksize:u32,pub cluster_size:u32,pub cluster_mask:u32,pub cluster_mask_inv:u64,pub block_mask:u32,pub blocks_per_cluster:u32,pub record_size:u32,pub index_size:u32,pub cluster_bits:u8,pub record_bits:u8,pub maxbytes:u64,pub maxbytes_sparse:u64,pub flags:usize,pub zone_max:CLST,pub bad_clusters:CLST,pub max_bytes_per_attr:u16,pub attr_size_tr:u16,pub objid_no:CLST,pub quota_no:CLST,pub reparse_no:CLST,pub usn_jrnl_no:CLST,pub def_table:*mut ATTR_DEF_ENTRY,pub def_entries:u32,pub ea_max_size:u32,pub new_rec:*mut MFT_REC,pub upcase:*mut u16,pub mft_mi:*mut ntfs_inode,pub used_bitmap:wnd_bitmap,pub volume_ni:*mut ntfs_inode,pub security_ni:*mut ntfs_inode,pub reparse_ni:*mut ntfs_inode,pub objid_ni:*mut ntfs_inode,pub options:*mut ntfs_mount_options }

pub const NTFS_MIN_MFT_ZONE:usize=100; pub const NTFS_MFT_INCREASE_STEP:usize=1024; pub const NTFS_DIRTY_CLEAR:i32=0; pub const NTFS_DIRTY_DIRTY:i32=1; pub const NTFS_DIRTY_ERROR:i32=2;
pub const REPARSE_NONE:i32=0; pub const REPARSE_COMPRESSED:i32=1; pub const REPARSE_DEDUPLICATED:i32=2; pub const REPARSE_LINK:i32=3;
pub const _100ns2seconds:u64=10_000_000; pub const SecondsToStartOf1970:u64=0x00000002B6109100; pub const NTFS_TIME_GRAN:u64=100;

#[inline] pub unsafe fn is_ni_base(ni:*const ntfs_inode)->bool { (*ni).base==ni as *mut _ }
#[inline] pub unsafe fn is_ntfs3(sbi:*const ntfs_sb_info)->bool { (*sbi).volume_major_ver()>=3 }
impl ntfs_sb_info { #[inline] unsafe fn volume_major_ver(&self)->u8 { 0 } }
#[inline] pub unsafe fn ntfs_up_cluster(sbi:*const ntfs_sb_info,size:u64)->u64 { (size+(*sbi).cluster_mask as u64)&(*sbi).cluster_mask_inv }
#[inline] pub unsafe fn bytes_to_cluster(sbi:*const ntfs_sb_info,size:u64)->CLST { (size+(*sbi).cluster_mask as u64)>>(*sbi).cluster_bits }
#[inline] pub unsafe fn is_compressed(ni:*const ntfs_inode)->bool { ((*ni).std_fa & 0x8000_0000)!=0 || ((*ni).ni_flags&NI_FLAG_COMPRESSED_MASK)!=0 }
#[inline] pub unsafe fn is_dedup(ni:*const ntfs_inode)->bool { (*ni).ni_flags&NI_FLAG_DEDUPLICATED!=0 }
#[inline] pub unsafe fn is_resident(ni:*const ntfs_inode)->i32 { ((*ni).ni_flags&NI_FLAG_RESIDENT) as i32 }
#[inline] pub unsafe fn run_init(run:*mut runs_tree) { (*run).runs=ptr::null_mut();(*run).count=0;(*run).allocated=0 }
#[inline] pub unsafe fn run_is_empty(run:*const runs_tree)->bool { (*run).count==0 }
#[inline] pub unsafe fn ntfs3_bitmap_size(bits:usize)->usize { ((bits+63)/64)*size_of::<u64>() }

// All remaining declarations are external kernel/NTFS interfaces from the header.
extern "C" {
    pub fn attr_allocate_clusters(sbi:*mut ntfs_sb_info,run:*mut runs_tree,run_da:*mut runs_tree,vcn:CLST,lcn:CLST,len:CLST,pre_alloc:*mut CLST,opt:i32,alen:*mut CLST,fr:usize,new_lcn:*mut CLST,new_len:*mut CLST)->i32;
    pub fn attr_set_size_ex(ni:*mut ntfs_inode,typ:ATTR_TYPE,name:*const __le16,name_len:u8,run:*mut runs_tree,new_size:u64,new_valid:*const u64,keep_prealloc:bool,ret:*mut *mut ATTRIB,no_da:bool)->i32;
    pub fn ntfs_iget5_flags(sb:*mut super_block,rf:*const MFT_REF,name:*const cpu_str,flags:u32)->*mut inode;
    pub fn ntfs_set_size(inode:*mut inode,new_size:u64)->i32;
    pub fn ntfs_evict_inode(inode:*mut inode);
    pub fn ntfs3_init_bitmap()->i32; pub fn ntfs3_exit_bitmap(); pub fn wnd_close(wnd:*mut wnd_bitmap);
    pub fn run_unpack(run:*mut runs_tree,sbi:*mut ntfs_sb_info,ino:CLST,svcn:CLST,evcn:CLST,vcn:CLST,buf:*const u8,size:i32)->i32;
    pub fn run_len(run:*const runs_tree)->CLST; pub fn run_get_max_vcn(run:*const runs_tree)->CLST;
    pub fn ntfs_set_label(sbi:*mut ntfs_sb_info,label:*mut u8,len:i32)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
