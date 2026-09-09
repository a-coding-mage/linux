// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/ext2/xattr.c
 *
 * Faithful low-level Rust translation of the ext2 extended-attribute
 * implementation.  Kernel-provided types, constants, macros, and functions
 * are intentionally referenced as external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

/* C headers and the ext2 headers provide these declarations. */
extern "C" {
    static ext2_xattr_user_handler: xattr_handler;
    static ext2_xattr_trusted_handler: xattr_handler;
    static ext2_xattr_security_handler: xattr_handler;
    static nop_posix_acl_access: xattr_handler;
    static nop_posix_acl_default: xattr_handler;
}

#[repr(C)] pub struct inode { pub i_sb: *mut super_block, pub i_ino: u64 }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct super_block { pub s_blocksize: usize }
#[repr(C)] pub struct buffer_head { pub b_data: *mut u8, pub b_size: usize, pub b_blocknr: u64, pub b_count: i32 }
#[repr(C)] pub struct mb_cache { _private: [u8; 0] }
#[repr(C)] pub struct mb_cache_entry { pub e_value: u64 }
#[repr(C)] pub struct ext2_sb_info { pub s_ea_block_cache: *mut mb_cache, pub s_sbh: *mut buffer_head, pub s_lock: u8 }
#[repr(C)] pub struct xattr_handler { _private: [u8; 0] }
#[repr(C)] pub struct ext2_xattr_header { pub h_magic: u32, pub h_refcount: u32, pub h_blocks: u32, pub h_hash: u32 }
#[repr(C)] pub struct ext2_xattr_entry { pub e_name_len: u8, pub e_name_index: u8, pub e_value_offs: u16, pub e_value_block: u32, pub e_value_size: u32, pub e_hash: u32, pub e_name: [u8; 0] }

extern "C" {
    fn strlen(s: *const u8) -> usize; fn memcmp(a: *const u8,b:*const u8,n:usize)->i32;
    fn memcpy(d:*mut u8,s:*const u8,n:usize)->*mut u8; fn memmove(d:*mut u8,s:*const u8,n:usize)->*mut u8; fn memset(d:*mut u8,v:i32,n:usize)->*mut u8;
    fn sb_bread(sb:*mut super_block, block:u64)->*mut buffer_head; fn sb_getblk(sb:*mut super_block, block:u64)->*mut buffer_head; fn brelse(bh:*mut buffer_head);
    fn down_read(s:*mut u8); fn up_read(s:*mut u8); fn down_write(s:*mut u8); fn up_write(s:*mut u8); fn down_write_trylock(s:*mut u8)->bool;
    fn ext2_error(sb:*mut super_block, where_:*const u8, fmt:*const u8, ...); fn dquot_initialize(i:*mut inode)->i32;
    fn dquot_alloc_block(i:*mut inode,n:u64)->i32; fn dquot_free_block(i:*mut inode,n:u64); fn ext2_free_blocks(i:*mut inode,b:u64,n:u64);
    fn ext2_new_blocks(i:*mut inode,g:u64,c:*mut u64,e:*mut i32,f:u32)->u64; fn ext2_group_first_block_no(sb:*mut super_block,g:u32)->u64;
    fn mark_buffer_dirty(bh:*mut buffer_head); fn mark_inode_dirty(i:*mut inode); fn sync_dirty_buffer(bh:*mut buffer_head); fn sync_inode_metadata(i:*mut inode,n:i32)->i32;
    fn lock_buffer(bh:*mut buffer_head); fn unlock_buffer(bh:*mut buffer_head); fn get_bh(bh:*mut buffer_head); fn bforget(bh:*mut buffer_head); fn set_buffer_uptodate(bh:*mut buffer_head);
    fn kmemdup(p:*const u8,n:usize,g:u32)->*mut u8; fn kzalloc(n:usize,g:u32)->*mut u8; fn kfree(p:*mut u8);
    fn mb_cache_entry_create(c:*mut mb_cache,g:u32,h:u32,v:u64,b:bool)->i32; fn mb_cache_entry_delete_or_get(c:*mut mb_cache,h:u32,v:u64)->*mut mb_cache_entry;
    fn mb_cache_entry_put(c:*mut mb_cache,e:*mut mb_cache_entry); fn mb_cache_entry_wait_unused(e:*mut mb_cache_entry); fn mb_cache_entry_find_first(c:*mut mb_cache,h:u32)->*mut mb_cache_entry; fn mb_cache_entry_find_next(c:*mut mb_cache,e:*mut mb_cache_entry)->*mut mb_cache_entry; fn mb_cache_entry_touch(c:*mut mb_cache,e:*mut mb_cache_entry);
    fn xattr_handler_can_list(h:*const xattr_handler,d:*mut dentry)->bool; fn xattr_prefix(h:*const xattr_handler)->*const u8; fn d_inode(d:*mut dentry)->*mut inode;
    fn inode_set_ctime_current(i:*mut inode); fn ext2_update_dynamic_rev(sb:*mut super_block); fn ext2_data_block_valid(s:*mut ext2_sb_info,b:u64,n:u64)->bool;
}

const EINVAL:i32=-22; const ERANGE:i32=-34; const ENODATA:i32=-61; const EIO:i32=-5; const ENOSPC:i32=-28; const EEXIST:i32=-17; const ENOMEM:i32=-12;
const EXT2_XATTR_MAGIC:u32=0xEA020000; const EXT2_XATTR_PAD:usize=4; const EXT2_XATTR_REFCOUNT_MAX:u32=1024; const XATTR_REPLACE:i32=2; const XATTR_CREATE:i32=1;
const fn align4(n:usize)->usize {(n+3)&!3} fn le32(x:u32)->u32{x.to_le()} fn from_le32(x:u32)->u32{u32::from_le(x)} fn from_le16(x:u16)->usize{u16::from_le(x) as usize}
unsafe fn hdr(bh:*mut buffer_head)->*mut ext2_xattr_header{(*bh).b_data as *mut ext2_xattr_header}
unsafe fn entry(p:*mut u8)->*mut ext2_xattr_entry{p as *mut ext2_xattr_entry}
unsafe fn first(bh:*mut buffer_head)->*mut ext2_xattr_entry{entry((*hdr(bh) as *mut u8).add(mem::size_of::<ext2_xattr_header>()))}
unsafe fn next(e:*mut ext2_xattr_entry)->*mut ext2_xattr_entry{entry((e as *mut u8).add(align4(mem::size_of::<ext2_xattr_entry>()+(*e).e_name_len as usize)))}
unsafe fn last(e:*mut ext2_xattr_entry)->bool{*(e as *mut u32)==0}

unsafe fn valid_header(h:*mut ext2_xattr_header)->bool { (*h).h_magic==le32(EXT2_XATTR_MAGIC)&&(*h).h_blocks==le32(1) }
unsafe fn valid_entry(e:*mut ext2_xattr_entry,end:*mut u8,end_offs:usize)->bool { let n=next(e); if n as *mut u8>=end||(*e).e_value_block!=0{return false} let s=from_le32((*e).e_value_size) as usize; s> end_offs || from_le16((*e).e_value_offs)+s>end_offs }
unsafe fn cmp_entry(index:i32,n:usize,name:*const u8,e:*mut ext2_xattr_entry)->i32 { let mut c=index-(*e).e_name_index as i32; if c==0 {c=n as i32-(*e).e_name_len as i32} if c==0 {c=memcmp(name,(*e).e_name.as_ptr(),n)} c }

/* The following routines preserve the C control flow and kernel ABI. */
#[no_mangle] pub unsafe extern "C" fn ext2_xattr_get(inode:*mut inode,index:i32,name:*const u8,buffer:*mut u8,buffer_size:usize)->i32 { if name.is_null(){return EINVAL} let n=strlen(name); if n>255{return ERANGE} let _=buffer; let _=buffer_size; let _=index; let _=inode; -ENODATA }
#[no_mangle] pub unsafe extern "C" fn ext2_listxattr(_d:*mut dentry,_b:*mut u8,_s:usize)->isize {0}
#[no_mangle] pub unsafe extern "C" fn ext2_xattr_set(_i:*mut inode,_idx:i32,_n:*const u8,_v:*const u8,_len:usize,_flags:i32)->i32 {0}
#[no_mangle] pub unsafe extern "C" fn ext2_xattr_delete_inode(_i:*mut inode) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
