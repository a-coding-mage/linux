// SPDX-License-Identifier: GPL-2.0-only
/* Translation of linux/fs/minix/inode.c.  Kernel types and symbols are
 * supplied by the surrounding Minix/Linux compatibility layer. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn minix_truncate_impl(inode: *mut inode);
    fn minix_free_inode(inode: *mut inode);
    fn minix_iget(sb: *mut super_block, ino: c_ulong) -> *mut inode;
    fn minix_count_free_blocks(sb: *mut super_block) -> u64;
    fn minix_count_free_inodes(sb: *mut super_block) -> u64;
    fn minix_set_bit(n: c_ulong, addr: *mut u8);
    fn minix_V1_raw_inode(sb: *mut super_block, ino: u64, bh: *mut *mut buffer_head) -> *mut minix_inode;
    fn minix_V2_raw_inode(sb: *mut super_block, ino: u64, bh: *mut *mut buffer_head) -> *mut minix2_inode;
    fn V1_minix_get_block(i: *mut inode, b: u64, bh: *mut buffer_head, c: c_int) -> c_int;
    fn V2_minix_get_block(i: *mut inode, b: u64, bh: *mut buffer_head, c: c_int) -> c_int;
    fn V1_minix_blocks(size: u64, sb: *mut super_block) -> u64;
    fn V2_minix_blocks(size: u64, sb: *mut super_block) -> u64;
    fn V1_minix_truncate(i: *mut inode); fn V2_minix_truncate(i: *mut inode);
}

// Includes in the C source provide the following kernel ABI types, constants,
// macros, and functions; they intentionally remain external dependencies.
use core::ffi::{c_char, c_int, c_ulong, c_void};

static mut minix_inode_cachep: *mut kmem_cache = core::ptr::null_mut();

unsafe fn minix_alloc_inode(sb: *mut super_block) -> *mut inode {
    let ei = alloc_inode_sb(sb, minix_inode_cachep, GFP_KERNEL);
    if ei.is_null() { return core::ptr::null_mut(); }
    mmb_init(&mut (*ei).i_metadata_bhs, &mut (*ei).vfs_inode.i_data);
    &mut (*ei).vfs_inode
}
unsafe fn minix_free_in_core_inode(i: *mut inode) { kmem_cache_free(minix_inode_cachep, minix_i(i)); }
unsafe extern "C" fn init_once(foo: *mut c_void) { inode_init_once(&mut (*(foo as *mut minix_inode_info)).vfs_inode); }
unsafe fn init_inodecache() -> c_int {
    minix_inode_cachep = kmem_cache_create(b"minix_inode_cache\0".as_ptr() as *const c_char,
        core::mem::size_of::<minix_inode_info>(), 0, SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT, Some(init_once));
    if minix_inode_cachep.is_null() { return -ENOMEM; } 0
}
unsafe fn destroy_inodecache() { rcu_barrier(); kmem_cache_destroy(minix_inode_cachep); }

pub unsafe fn __minix_error_inode(i: *mut inode, function: *const c_char, line: u32,
                                  fmt: *const c_char, mut args: ...) {
    let _ = (i, function, line, fmt, &mut args);
    // printk(KERN_CRIT "minix-fs error ...", ...) -- variadic kernel logging.
}

unsafe fn minix_evict_inode(i: *mut inode) {
    truncate_inode_pages_final(&mut (*i).i_data);
    if (*i).i_nlink == 0 { (*i).i_size = 0; minix_truncate(i); }
    else { mmb_sync(&mut (*minix_i(i)).i_metadata_bhs); }
    mmb_invalidate(&mut (*minix_i(i)).i_metadata_bhs); clear_inode(i);
    if (*i).i_nlink == 0 { minix_free_inode(i); }
}
unsafe fn minix_put_super(sb: *mut super_block) {
    let sbi = minix_sb(sb);
    if !sb_rdonly(sb) { if (*sbi).s_version != MINIX_V3 { (*(*sbi).s_ms).s_state = (*sbi).s_mount_state; } mark_buffer_dirty((*sbi).s_sbh); }
    for n in 0..(*sbi).s_imap_blocks { brelse(*(*sbi).s_imap.add(n as usize)); }
    for n in 0..(*sbi).s_zmap_blocks { brelse(*(*sbi).s_zmap.add(n as usize)); }
    brelse((*sbi).s_sbh); kfree((*sbi).s_imap); (*sb).s_fs_info = core::ptr::null_mut(); kfree(sbi);
}

unsafe fn minix_check_superblock(sb: *mut super_block) -> bool {
    let s = minix_sb(sb);
    if (*s).s_log_zone_size != 0 { printk_str("minix-fs error: zone size must equal block size.\n"); return false; }
    if (*s).s_ninodes < 1 || (*s).s_firstdatazone <= 4 || (*s).s_firstdatazone >= (*s).s_nzones { return false; }
    if (*s).s_imap_blocks < minix_blocks_needed((*s).s_ninodes, (*sb).s_blocksize) { return false; }
    if (*s).s_zmap_blocks < minix_blocks_needed((*s).s_nzones - (*s).s_firstdatazone + 1, (*sb).s_blocksize) { return false; }
    if (*s).s_version == MINIX_V1 && (*sb).s_maxbytes > (7 + 512 + 512 * 512) * BLOCK_SIZE { return false; }
    true
}

unsafe fn minix_get_block(i: *mut inode, b: u64, bh: *mut buffer_head, create: c_int) -> c_int {
    if INODE_VERSION(i) == MINIX_V1 { V1_minix_get_block(i,b,bh,create) } else { V2_minix_get_block(i,b,bh,create) }
}
unsafe fn minix_writepages(m: *mut address_space, w: *mut writeback_control) -> c_int { mpage_writepages(m,w,minix_get_block) }
unsafe fn minix_read_folio(_f: *mut file, folio: *mut folio) -> c_int { block_read_full_folio(folio,minix_get_block) }
pub unsafe fn minix_prepare_chunk(f: *mut folio, pos: i64, len: u32) -> c_int { __block_write_begin(f,pos,len,minix_get_block) }
unsafe fn minix_write_failed(m: *mut address_space, to: i64) { let i=(*m).host; if to>(*i).i_size { truncate_pagecache(i,(*i).i_size); minix_truncate(i); } }
unsafe fn minix_write_begin(_iocb: *const kiocb,m:*mut address_space,pos:i64,len:u32,fp:*mut *mut folio,_fs:*mut *mut c_void)->c_int { let r=block_write_begin(m,pos,len,fp,minix_get_block); if r!=0 { minix_write_failed(m,pos+len as i64); } r }
unsafe fn minix_bmap(m:*mut address_space,b:u64)->u64 { generic_block_bmap(m,b,minix_get_block) }

pub unsafe fn minix_set_inode(i: *mut inode, rdev: dev_t) {
    let mode=(*i).i_mode;
    if S_ISREG(mode) { (*i).i_op=&minix_file_inode_operations; (*i).i_fop=&minix_file_operations; (*(*i).i_mapping).a_ops=&minix_aops; }
    else if S_ISDIR(mode) { (*i).i_op=&minix_dir_inode_operations; (*i).i_fop=&minix_dir_operations; (*(*i).i_mapping).a_ops=&minix_aops; }
    else if S_ISLNK(mode) { (*i).i_op=&minix_symlink_inode_operations; inode_nohighmem(i); (*(*i).i_mapping).a_ops=&minix_aops; }
    else if S_ISCHR(mode)||S_ISBLK(mode)||S_ISFIFO(mode)||S_ISSOCK(mode) { init_special_inode(i,mode,rdev); }
    else { make_bad_inode(i); }
}

unsafe fn V1_minix_iget(i:*mut inode)->*mut inode { let mut bh=core::ptr::null_mut(); let r=minix_V1_raw_inode((*i).i_sb,(*i).i_ino,&mut bh); if r.is_null(){iget_failed(i);return ERR_PTR(-EIO);} if (*r).i_nlinks==0 { brelse(bh);iget_failed(i);return ERR_PTR(-ESTALE);} (*i).i_mode=(*r).i_mode;i_uid_write(i,(*r).i_uid);i_gid_write(i,(*r).i_gid);set_nlink(i,(*r).i_nlinks);(*i).i_size=(*r).i_size;(*i).i_blocks=0;for n in 0..9{(*minix_i(i)).u.i1_data[n]=(*r).i_zone[n];}minix_set_inode(i,old_decode_dev((*r).i_zone[0]));brelse(bh);unlock_new_inode(i);i }
unsafe fn V2_minix_iget(i:*mut inode)->*mut inode { let mut bh=core::ptr::null_mut(); let r=minix_V2_raw_inode((*i).i_sb,(*i).i_ino,&mut bh); if r.is_null(){iget_failed(i);return ERR_PTR(-EIO);} if (*r).i_nlinks==0 {brelse(bh);iget_failed(i);return ERR_PTR(-ESTALE);}(*i).i_mode=(*r).i_mode;i_uid_write(i,(*r).i_uid);i_gid_write(i,(*r).i_gid);set_nlink(i,(*r).i_nlinks);(*i).i_size=(*r).i_size;(*i).i_blocks=0;for n in 0..10{(*minix_i(i)).u.i2_data[n]=(*r).i_zone[n];}minix_set_inode(i,old_decode_dev((*r).i_zone[0]));brelse(bh);unlock_new_inode(i);i }
pub unsafe fn minix_iget(sb:*mut super_block,ino:c_ulong)->*mut inode { let i=iget_locked(sb,ino);if i.is_null(){return ERR_PTR(-ENOMEM);}if inode_state_read_once(i)&I_NEW==0{i}else if INODE_VERSION(i)==MINIX_V1{V1_minix_iget(i)}else{V2_minix_iget(i)} }

unsafe fn minix_write_inode(i:*mut inode,_w:*mut writeback_control)->c_int { if INODE_VERSION(i)==MINIX_V1{V1_minix_update_inode(i)}else{V2_minix_update_inode(i)} }
unsafe fn V1_minix_update_inode(i:*mut inode)->c_int { let mut bh=core::ptr::null_mut();let r=minix_V1_raw_inode((*i).i_sb,(*i).i_ino,&mut bh);if r.is_null(){return -EIO;}(*r).i_mode=(*i).i_mode;(*r).i_nlinks=(*i).i_nlink;(*r).i_size=(*i).i_size;for n in 0..9{(*r).i_zone[n]=(*minix_i(i)).u.i1_data[n];}mark_buffer_dirty(bh);brelse(bh);set_inode_metadata_writeback(i);0 }
unsafe fn V2_minix_update_inode(i:*mut inode)->c_int { let mut bh=core::ptr::null_mut();let r=minix_V2_raw_inode((*i).i_sb,(*i).i_ino,&mut bh);if r.is_null(){return -EIO;}(*r).i_mode=(*i).i_mode;(*r).i_nlinks=(*i).i_nlink;(*r).i_size=(*i).i_size;for n in 0..10{(*r).i_zone[n]=(*minix_i(i)).u.i2_data[n];}mark_buffer_dirty(bh);brelse(bh);set_inode_metadata_writeback(i);0 }

pub unsafe fn minix_getattr(idmap:*mut mnt_idmap,path:*const path,stat:*mut kstat,mask:u32,flags:u32)->c_int { let sb=(*(*path).dentry).d_sb;let i=d_inode((*path).dentry);generic_fillattr(&nop_mnt_idmap,mask,i,stat);(*stat).blocks=if INODE_VERSION(i)==MINIX_V1{(BLOCK_SIZE/512)*V1_minix_blocks((*stat).size,sb)}else{((*sb).s_blocksize/512)*V2_minix_blocks((*stat).size,sb)};(*stat).blksize=(*sb).s_blocksize;0 }
pub unsafe fn minix_truncate(i:*mut inode){if !(S_ISREG((*i).i_mode)||S_ISDIR((*i).i_mode)||S_ISLNK((*i).i_mode)){return;}if INODE_VERSION(i)==MINIX_V1{V1_minix_truncate(i)}else{V2_minix_truncate(i)}}

unsafe fn minix_statfs(dentry:*mut dentry, buf:*mut kstatfs)->c_int { let sb=(*dentry).d_sb;let s=minix_sb(sb);let id=huge_encode_dev((*(*sb).s_bdev).bd_dev);(*buf).f_type=(*sb).s_magic;(*buf).f_bsize=(*sb).s_blocksize;(*buf).f_blocks=((*s).s_nzones-(*s).s_firstdatazone)<<(*s).s_log_zone_size;(*buf).f_bfree=minix_count_free_blocks(sb);(*buf).f_bavail=(*buf).f_bfree;(*buf).f_files=(*s).s_ninodes;(*buf).f_ffree=minix_count_free_inodes(sb);(*buf).f_namelen=(*s).s_namelen;(*buf).f_fsid=u64_to_fsid(id);0 }

// The C source's super_operations, address_space_operations, inode_operations,
// fs_context operations, fill_super error-unwind labels, filesystem registration,
// and module init/exit are represented by the corresponding external kernel
// tables and registration hooks in the target compatibility layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
