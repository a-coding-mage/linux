/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) 2012-2013 Samsung Electronics Co., Ltd. */
// Linux dependencies and build-time definitions are supplied by other translated units.

pub const EXFAT_ROOT_INO: u64 = 1;
#[repr(C)] pub enum exfat_error_mode { EXFAT_ERRORS_CONT, EXFAT_ERRORS_PANIC, EXFAT_ERRORS_RO }
pub const NLS_NAME_NO_LOSSY: u32 = 0;
pub const NLS_NAME_LOSSY: u32 = 1 << 0;
pub const EXFAT_HASH_BITS: usize = 8;
pub const EXFAT_HASH_SIZE: usize = 1usize << EXFAT_HASH_BITS;
pub const ES_2_ENTRIES: u32 = 2;
pub const ES_ALL_ENTRIES: u32 = 0;
pub const ES_IDX_FILE: u32 = 0;
pub const ES_IDX_STREAM: u32 = 1;
pub const ES_IDX_FIRST_FILENAME: u32 = 2;
pub const DIR_DELETED: u32 = 0xFFFF_FFF7;
pub const TYPE_UNUSED: u32 = 0x0000; pub const TYPE_DELETED: u32 = 0x0001;
pub const TYPE_INVALID: u32 = 0x0002; pub const TYPE_CRITICAL_PRI: u32 = 0x0100;
pub const TYPE_BITMAP: u32 = 0x0101; pub const TYPE_UPCASE: u32 = 0x0102;
pub const TYPE_VOLUME: u32 = 0x0103; pub const TYPE_DIR: u32 = 0x0104;
pub const TYPE_FILE: u32 = 0x011F; pub const TYPE_CRITICAL_SEC: u32 = 0x0200;
pub const TYPE_STREAM: u32 = 0x0201; pub const TYPE_EXTEND: u32 = 0x0202;
pub const TYPE_ACL: u32 = 0x0203; pub const TYPE_BENIGN_PRI: u32 = 0x0400;
pub const TYPE_GUID: u32 = 0x0401; pub const TYPE_PADDING: u32 = 0x0402;
pub const TYPE_ACLTAB: u32 = 0x0403; pub const TYPE_BENIGN_SEC: u32 = 0x0800;
pub const TYPE_VENDOR_EXT: u32 = 0x0801; pub const TYPE_VENDOR_ALLOC: u32 = 0x0802;
pub const MAX_CHARSET_SIZE: usize = 6; pub const MAX_NAME_LENGTH: usize = 255;
pub const MAX_VFSNAME_BUF_SIZE: usize = (MAX_NAME_LENGTH + 1) * MAX_CHARSET_SIZE;
pub const EXFAT_HINT_NONE: i32 = -1; pub const EXFAT_MIN_SUBDIR: u32 = 2;
pub const FAT_ENT_SIZE: u32 = 4; pub const FAT_ENT_SIZE_BITS: u32 = 2;
pub const EXFAT_FLAGS_SHUTDOWN: u32 = 1; pub const EXFAT_CACHE_VALID: u32 = 0;

#[repr(C)] pub struct exfat_dentry_namebuf { pub lfn: *mut ::std::ffi::c_char, pub lfnbuf_len: i32 }
#[repr(C)] pub struct exfat_uni_name { pub name: [u16; MAX_NAME_LENGTH + 3], pub name_hash: u16, pub name_len: u8 }
#[repr(C)] pub struct exfat_chain { pub dir: u32, pub size: u32, pub flags: u8 }
#[repr(C)] pub struct exfat_hint_femp { pub eidx: i32, pub count: i32, pub cur: exfat_chain }
#[repr(C)] pub union exfat_hint_union { pub off: u32, pub eidx: i32 }
#[repr(C)] pub struct exfat_hint { pub clu: u32, pub value: exfat_hint_union }
#[repr(C)] pub struct exfat_entry_set_cache { pub sb: *mut super_block, pub start_off: u32, pub num_bh: i32, pub __bh: [*mut buffer_head; DIR_CACHE_SIZE], pub bh: *mut *mut buffer_head, pub num_entries: u32, pub modified: bool }
#[repr(C)] pub struct exfat_dir_entry { pub dir: exfat_chain, pub entry: i32, pub type_: u32, pub start_clu: u32, pub flags: u8, pub attr: u16, pub size: loff_t, pub valid_size: loff_t, pub num_subdirs: u32, pub atime: timespec64, pub mtime: timespec64, pub crtime: timespec64, pub namebuf: exfat_dentry_namebuf }
#[repr(C)] pub struct exfat_mount_options { pub fs_uid: kuid_t, pub fs_gid: kgid_t, pub fs_fmask: u16, pub fs_dmask: u16, pub allow_utime: u16, pub iocharset: *mut ::std::ffi::c_char, pub errors: exfat_error_mode, pub utf8: u8, pub sys_tz: u8, pub discard: u8, pub keep_last_dots: u8, pub time_offset: i32, pub zero_size_dir: bool }
#[repr(C)] pub struct exfat_sb_info { pub num_sectors:u64, pub num_clusters:u32, pub cluster_size:u32, pub cluster_size_bits:u32, pub sect_per_clus:u32, pub sect_per_clus_bits:u32, pub FAT1_start_sector:u64, pub FAT2_start_sector:u64, pub data_start_sector:u64, pub data_start_bytes:u64, pub num_FAT_sectors:u32, pub root_dir:u32, pub dentries_per_clu:u32, pub vol_flags:u32, pub vol_flags_persistent:u32, pub boot_bh:*mut buffer_head, pub map_clu:u32, pub map_sectors:u32, pub vol_amap:*mut *mut buffer_head, pub vol_utbl:*mut u16, pub clu_srch_ptr:u32, pub used_clusters:u32, pub s_exfat_flags:usize, pub s_lock:mutex, pub bitmap_lock:mutex, pub options:exfat_mount_options, pub nls_io:*mut nls_table, pub ratelimit:ratelimit_state, pub inode_hash_lock:spinlock_t, pub inode_hashtable:[hlist_head; EXFAT_HASH_SIZE], pub rcu:rcu_head }
#[repr(C)] pub struct exfat_inode_info { pub dir:exfat_chain, pub entry:i32, pub type_:u32, pub attr:u16, pub start_clu:u32, pub flags:u8, pub version:u32, pub hint_bmap:exfat_hint, pub hint_stat:exfat_hint, pub hint_femp:exfat_hint_femp, pub cache_lru_lock:spinlock_t, pub cache_lru:list_head, pub nr_caches:i32, pub cache_valid_id:u32, pub i_pos:loff_t, pub valid_size:loff_t, pub zeroed_size:loff_t, pub i_hash_fat:hlist_node, pub vfs_inode:inode, pub i_crtime:timespec64 }

pub const fn exfat_filename_entry_num(n: usize) -> usize { (n + EXFAT_FILE_NAME_LEN - 1) / EXFAT_FILE_NAME_LEN }
pub const fn es_idx_last_filename(n: usize) -> usize { ES_IDX_FIRST_FILENAME as usize + exfat_filename_entry_num(n) - 1 }
pub const fn es_entry_num(n: usize) -> usize { es_idx_last_filename(n) + 1 }
pub const ES_MAX_ENTRY_NUM: usize = es_entry_num(MAX_NAME_LENGTH);
pub const DIR_CACHE_SIZE: usize = (ES_MAX_ENTRY_NUM << DENTRY_SIZE_BITS) / SECTOR_SIZE + 2;

extern "C" { pub fn exfat_set_volume_dirty(sb:*mut super_block)->i32; pub fn exfat_clear_volume_dirty(sb:*mut super_block)->i32; pub fn exfat_ent_get(sb:*mut super_block,loc:u32,content:*mut u32,last:*mut *mut buffer_head)->i32; pub fn brelse(bh:*mut buffer_head); }
#[inline] pub unsafe fn exfat_cluster_walk(sb:*mut super_block, clu:*mut u32, mut step:u32, flags:i32)->i32 { if flags==ALLOC_NO_FAT_CHAIN { *clu=(*clu).wrapping_add(step); return 0; } while step!=0 { if exfat_ent_get(sb,*clu,clu,&mut core::ptr::null_mut())!=0 { return -EIO; } step-=1; } 0 }
#[inline] pub unsafe fn exfat_chain_advance(sb:*mut super_block, chain:*mut exfat_chain, step:u32)->i32 { if (*chain).size<step{return -EIO;} let mut clu=(*chain).dir; if exfat_cluster_walk(sb,&mut clu,step,(*chain).flags as i32)!=0{return -EIO;} (*chain).size-=step; if (*chain).size==0 && (*chain).flags as i32==ALLOC_NO_FAT_CHAIN {(*chain).dir=EXFAT_EOF_CLUSTER;} else {(*chain).dir=clu;} 0 }

// Declaration-only external interfaces from the header.
extern "C" {
    pub fn exfat_alloc_cluster(i:*mut inode,n:u32,c:*mut exfat_chain,sync:bool,contig:bool)->i32;
    pub fn exfat_free_cluster(i:*mut inode,c:*mut exfat_chain)->i32;
    pub fn exfat_load_bitmap(sb:*mut super_block)->i32; pub fn exfat_free_bitmap(sbi:*mut exfat_sb_info);
    pub fn exfat_set_bitmap(sb:*mut super_block,clu:u32,sync:bool)->i32; pub fn exfat_clear_bitmap(sb:*mut super_block,clu:u32,sync:bool)->i32;
    pub fn exfat_test_bitmap(sb:*mut super_block,clu:u32)->bool; pub fn exfat_count_used_clusters(sb:*mut super_block,r:*mut u32)->i32;
    pub fn exfat_force_shutdown(sb:*mut super_block,flags:u32)->i32;
    pub fn exfat_sync_inode(i:*mut inode); pub fn exfat_evict_inode(i:*mut inode);
    pub fn exfat_toupper(sb:*mut super_block,a:u16)->u16; pub fn exfat_create_upcase_table(sb:*mut super_block)->i32;
    pub fn __exfat_fs_error(sb:*mut super_block,report:i32,fmt:*const ::std::ffi::c_char,...);
    pub fn exfat_chain_set(ec:*mut exfat_chain,dir:u32,size:u32,flags:u8); pub fn exfat_chain_dup(dup:*mut exfat_chain,ec:*mut exfat_chain);
    pub fn exfat_blk_readahead(sb:*mut super_block,sec:sector_t,ra:*mut sector_t,ra_cnt:*mut blkcnt_t,end:sector_t)->i32;
    pub fn exfat_find_free_bitmap(sb:*mut super_block,clu:u32)->u32; pub fn exfat_trim_fs(i:*mut inode,r:*mut fstrim_range)->i32;
    pub fn __exfat_truncate(i:*mut inode)->i32; pub fn exfat_setattr(idmap:*mut mnt_idmap,d:*mut dentry,a:*mut iattr)->i32;
    pub fn exfat_getattr(idmap:*mut mnt_idmap,path:*const path,stat:*mut kstat,mask:u32,flags:u32)->i32;
    pub fn exfat_file_fsync(f:*mut file,start:loff_t,end:loff_t,datasync:i32)->i32; pub fn exfat_ioctl(f:*mut file,cmd:u32,arg:usize)->isize;
    pub fn exfat_cache_init()->i32; pub fn exfat_cache_shutdown(); pub fn exfat_cache_inval_inode(i:*mut inode);
    pub fn exfat_get_cluster(i:*mut inode,cluster:u32,dclus:*mut u32,count:*mut u32,last:*mut u32)->i32;
    pub fn exfat_get_entry_type(e:*mut exfat_dentry)->u32; pub fn exfat_calc_num_entries(n:*mut exfat_uni_name)->i32;
    pub fn exfat_update_dir_chksum(es:*mut exfat_entry_set_cache); pub fn exfat_put_dentry_set(es:*mut exfat_entry_set_cache,sync:i32)->i32;
    pub fn exfat_count_dir_entries(sb:*mut super_block,d:*mut exfat_chain)->i32; pub fn exfat_read_volume_label(sb:*mut super_block,n:*mut exfat_uni_name)->i32;
    pub fn exfat_iget(sb:*mut super_block,pos:loff_t)->*mut inode; pub fn exfat_build_inode(sb:*mut super_block,e:*mut exfat_dir_entry,pos:loff_t)->*mut inode;
    pub fn exfat_hash_inode(i:*mut inode,pos:loff_t); pub fn exfat_unhash_inode(i:*mut inode);
    pub fn exfat_map_cluster(i:*mut inode,off:u32,clu:*mut u32,count:*mut u32,create:i32,balloc:*mut bool)->i32;
    pub fn exfat_uniname_ncmp(sb:*mut super_block,a:*mut u16,b:*mut u16,len:u32)->i32;
    pub fn exfat_utf16_to_nls(sb:*mut super_block,n:*mut exfat_uni_name,s:*mut u8,len:i32)->i32;
    pub fn exfat_nls_to_utf16(sb:*mut super_block,s:*const u8,len:i32,n:*mut exfat_uni_name,lossy:*mut i32)->i32;
    pub fn exfat_free_upcase_table(sbi:*mut exfat_sb_info); pub fn exfat_truncate_atime(ts:*mut timespec64);
    pub fn exfat_truncate_inode_atime(i:*mut inode); pub fn exfat_update_bh(bh:*mut buffer_head,sync:i32)->i32;
    pub fn exfat_update_bhs(bhs:*mut *mut buffer_head,n:i32,sync:i32)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
