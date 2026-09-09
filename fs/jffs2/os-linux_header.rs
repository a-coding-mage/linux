/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 *
 * Created by David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

// JFFS2 uses Linux mode bits natively -- no need for conversion.

use core::ffi::c_void;

#[repr(C)]
pub struct kstatfs;
#[repr(C)]
pub struct kvec;
pub type UChar = u8;
pub type LoFF = i64;
pub type SizeT = usize;

#[inline]
pub unsafe fn os_to_jffs2_mode<T>(x: T) -> T { x }
#[inline]
pub unsafe fn jffs2_to_os_mode<T>(x: T) -> T { x }

// These macros depend on definitions supplied by the surrounding kernel code.
#[inline]
pub unsafe fn jffs2_inode_info<T>(i: *mut T) -> *mut jffs2_inode_info { 
    container_of(i, core::mem::offset_of!(jffs2_inode_info, vfs_inode))
}

#[inline]
pub unsafe fn ofni_edoni_2sffj(f: *mut jffs2_inode_info) -> *mut inode { &mut (*f).vfs_inode }
#[inline]
pub unsafe fn jffs2_sb_info(sb: *mut super_block) -> *mut c_void { (*sb).s_fs_info }
#[inline]
pub unsafe fn ofni_bs_2sffj(c: *mut jffs2_sb_info) -> *mut super_block { (*c).os_priv as *mut super_block }

#[inline] pub unsafe fn jffs2_f_i_size(f: *mut jffs2_inode_info) -> i64 { (*ofni_edoni_2sffj(f)).i_size }
#[inline] pub unsafe fn jffs2_f_i_mode(f: *mut jffs2_inode_info) -> umode_t { (*ofni_edoni_2sffj(f)).i_mode }
#[inline] pub unsafe fn jffs2_f_i_uid(f: *mut jffs2_inode_info) -> u32 { i_uid_read(ofni_edoni_2sffj(f)) }
#[inline] pub unsafe fn jffs2_f_i_gid(f: *mut jffs2_inode_info) -> u32 { i_gid_read(ofni_edoni_2sffj(f)) }
#[inline] pub unsafe fn jffs2_f_i_rdev(f: *mut jffs2_inode_info) -> dev_t { (*ofni_edoni_2sffj(f)).i_rdev }

#[inline] pub fn jffs2_clamp_time(t: i64) -> u32 { t.clamp(0, u32::MAX as i64) as u32 }
#[inline] pub const fn itime(sec: i64) -> timespec64 { timespec64 { tv_sec: sec, tv_nsec: 0 } }
#[inline] pub unsafe fn jffs2_now() -> u32 { jffs2_clamp_time(ktime_get_real_seconds()) }
#[inline] pub fn i_sec(tv: timespec64) -> u32 { jffs2_clamp_time(tv.tv_sec) }
#[inline] pub unsafe fn jffs2_f_i_ctime(f: *mut jffs2_inode_info) -> u32 { i_sec(inode_get_ctime(ofni_edoni_2sffj(f))) }
#[inline] pub unsafe fn jffs2_f_i_mtime(f: *mut jffs2_inode_info) -> u32 { i_sec(inode_get_mtime(ofni_edoni_2sffj(f))) }
#[inline] pub unsafe fn jffs2_f_i_atime(f: *mut jffs2_inode_info) -> u32 { i_sec(inode_get_atime(ofni_edoni_2sffj(f))) }

#[inline]
pub unsafe fn sleep_on_spinunlock(wq: *mut wait_queue_head, s: *mut spinlock) {
    let mut wait = WaitQueueEntry::declare(current());
    add_wait_queue(wq, &mut wait);
    set_current_state(TASK_UNINTERRUPTIBLE);
    spin_unlock(s);
    schedule();
    remove_wait_queue(wq, &mut wait);
}

#[inline]
pub unsafe fn jffs2_init_inode_info(f: *mut jffs2_inode_info) {
    (*f).highest_version = 0;
    (*f).fragtree = RB_ROOT;
    (*f).metadata = core::ptr::null_mut();
    (*f).dents = core::ptr::null_mut();
    (*f).target = core::ptr::null_mut();
    (*f).flags = 0;
    (*f).usercompr = 0;
}

#[inline] pub unsafe fn jffs2_is_readonly(c: *mut jffs2_sb_info) -> u32 { (*ofni_bs_2sffj(c)).s_flags & SB_RDONLY }
#[inline] pub unsafe fn sector_addr(c: *mut jffs2_sb_info, x: usize) -> usize { (x / (*c).sector_size) * (*c).sector_size }

// CONFIG_JFFS2_FS_WRITEBUFFER is a build-time condition. The following declarations cover both configurations.
#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
#[inline] pub unsafe fn jffs2_can_mark_obsolete<T>(_c: *mut T) -> i32 { 1 }
#[cfg(CONFIG_JFFS2_FS_WRITEBUFFER)]
#[inline] pub unsafe fn jffs2_can_mark_obsolete(c: *mut jffs2_sb_info) -> u32 { (*c).mtd.flags & MTD_BIT_WRITEABLE }

#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
#[inline] pub unsafe fn jffs2_is_writebuffered<T>(_c: *mut T) -> i32 { 0 }
#[cfg(CONFIG_JFFS2_FS_WRITEBUFFER)]
#[inline] pub unsafe fn jffs2_is_writebuffered(c: *mut jffs2_sb_info) -> bool { !(*c).wbuf.is_null() }

#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
#[inline] pub unsafe fn jffs2_cleanmarker_oob<T>(_c: *mut T) -> i32 { 0 }
#[cfg(CONFIG_JFFS2_FS_WRITEBUFFER)]
#[inline] pub unsafe fn jffs2_cleanmarker_oob(c: *mut jffs2_sb_info) -> bool { (*c).mtd.type_ == MTD_NANDFLASH }
#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
#[inline] pub unsafe fn jffs2_wbuf_dirty<T>(_c: *mut T) -> i32 { 0 }
#[cfg(CONFIG_JFFS2_FS_WRITEBUFFER)]
#[inline] pub unsafe fn jffs2_wbuf_dirty(c: *mut jffs2_sb_info) -> bool { (*c).wbuf_len != 0 }

#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
pub unsafe fn jffs2_write_nand_cleanmarker(_c: *mut jffs2_sb_info, _jeb: *mut jffs2_eraseblock) -> i32 { -EIO }
#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
pub unsafe fn jffs2_flush_wbuf_pad<T>(_c: *mut T) -> i32 { 0 }
#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
pub unsafe fn jffs2_flush_wbuf_gc<T>(_c: *mut T, _i: u32) -> i32 { 0 }
#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
pub unsafe fn jffs2_write_nand_badblock<T>(_c: *mut T, _jeb: *mut jffs2_eraseblock, _bad_offset: u32) -> i32 { 1 }
#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
pub unsafe fn jffs2_nand_flash_setup<T>(_c: *mut T) -> i32 { 0 }
#[cfg(not(CONFIG(CONFIG_JFFS2_FS_WRITEBUFFER)))]
pub unsafe fn jffs2_nand_flash_cleanup<T>(_c: *mut T) {}

// The remaining write-buffer entries are null or no-op macros in the non-writebuffer configuration.
#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
pub static mut jffs2_wbuf_timeout: Option<unsafe extern "C" fn(usize)> = None;
#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
pub static mut jffs2_wbuf_process: Option<unsafe extern "C" fn(*mut c_void)> = None;
#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
#[inline] pub unsafe fn jffs2_dataflash<T>(_c: *mut T) -> i32 { 0 }
#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
#[inline] pub unsafe fn jffs2_dataflash_setup<T>(_c: *mut T) -> i32 { 0 }
#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
#[inline] pub unsafe fn jffs2_ubivol<T>(_c: *mut T) -> i32 { 0 }
#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
#[inline] pub unsafe fn jffs2_ubivol_setup<T>(_c: *mut T) -> i32 { 0 }
#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
#[inline] pub unsafe fn jffs2_nor_wbuf_flash<T>(_c: *mut T) -> i32 { 0 }
#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
#[inline] pub unsafe fn jffs2_nor_wbuf_flash_setup<T>(_c: *mut T) -> i32 { 0 }
#[cfg(not(CONFIG_JFFS2_FS_WRITEBUFFER))]
#[inline] pub unsafe fn jffs2_dirty_trigger<T>(_c: *mut T) {}

#[cfg(CONFIG_JFFS2_FS_WRITEBUFFER)]
extern "C" {
    pub fn jffs2_flash_writev(c: *mut jffs2_sb_info, vecs: *const kvec, count: usize, to: LoFF, retlen: *mut usize, ino: u32) -> i32;
    pub fn jffs2_flash_write(c: *mut jffs2_sb_info, ofs: LoFF, len: usize, retlen: *mut usize, buf: *const u8) -> i32;
    pub fn jffs2_flash_read(c: *mut jffs2_sb_info, ofs: LoFF, len: usize, retlen: *mut usize, buf: *mut u8) -> i32;
    pub fn jffs2_check_oob_empty(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock, mode: i32) -> i32;
    pub fn jffs2_check_nand_cleanmarker(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) -> i32;
    pub fn jffs2_write_nand_cleanmarker(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) -> i32;
    pub fn jffs2_write_nand_badblock(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock, bad_offset: u32) -> i32;
    pub fn jffs2_wbuf_timeout(data: usize);
    pub fn jffs2_wbuf_process(data: *mut c_void);
    pub fn jffs2_flush_wbuf_gc(c: *mut jffs2_sb_info, ino: u32) -> i32;
    pub fn jffs2_flush_wbuf_pad(c: *mut jffs2_sb_info) -> i32;
    pub fn jffs2_nand_flash_setup(c: *mut jffs2_sb_info) -> i32;
    pub fn jffs2_nand_flash_cleanup(c: *mut jffs2_sb_info);
    pub fn jffs2_dataflash_setup(c: *mut jffs2_sb_info) -> i32;
    pub fn jffs2_dataflash_cleanup(c: *mut jffs2_sb_info);
    pub fn jffs2_ubivol_setup(c: *mut jffs2_sb_info) -> i32;
    pub fn jffs2_ubivol_cleanup(c: *mut jffs2_sb_info);
    pub fn jffs2_nor_wbuf_flash_setup(c: *mut jffs2_sb_info) -> i32;
    pub fn jffs2_nor_wbuf_flash_cleanup(c: *mut jffs2_sb_info);
    pub fn jffs2_dirty_trigger(c: *mut jffs2_sb_info);
}

extern "C" {
    pub fn jffs2_start_garbage_collect_thread(c: *mut jffs2_sb_info) -> i32;
    pub fn jffs2_stop_garbage_collect_thread(c: *mut jffs2_sb_info);
    pub fn jffs2_garbage_collect_trigger(c: *mut jffs2_sb_info);
    pub static jffs2_dir_operations: file_operations;
    pub static jffs2_dir_inode_operations: inode_operations;
    pub static jffs2_file_operations: file_operations;
    pub static jffs2_file_inode_operations: inode_operations;
    pub static jffs2_file_address_operations: address_space_operations;
    pub fn jffs2_fsync(file: *mut file, start: LoFF, end: LoFF, datasync: i32) -> i32;
    pub fn __jffs2_read_folio(file: *mut file, folio: *mut folio) -> i32;
    pub fn jffs2_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize;
    pub static jffs2_symlink_inode_operations: inode_operations;
    pub fn jffs2_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> i32;
    pub fn jffs2_do_setattr(inode: *mut inode, attr: *mut iattr) -> i32;
    pub fn jffs2_iget(sb: *mut super_block, ino: usize) -> *mut inode;
    pub fn jffs2_evict_inode(inode: *mut inode);
    pub fn jffs2_dirty_inode(inode: *mut inode, flags: i32);
    pub fn jffs2_new_inode(dir_i: *mut inode, mode: umode_t, ri: *mut jffs2_raw_inode) -> *mut inode;
    pub fn jffs2_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> i32;
    pub fn jffs2_do_remount_fs(sb: *mut super_block, fc: *mut fs_context) -> i32;
    pub fn jffs2_do_fill_super(sb: *mut super_block, fc: *mut fs_context) -> i32;
    pub fn jffs2_gc_release_inode(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info);
    pub fn jffs2_gc_fetch_inode(c: *mut jffs2_sb_info, inum: i32, unlinked: i32) -> *mut jffs2_inode_info;
    pub fn jffs2_gc_fetch_page(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info, offset: usize, priv_: *mut usize) -> *mut u8;
    pub fn jffs2_flash_cleanup(c: *mut jffs2_sb_info);
    pub fn jffs2_flash_direct_writev(c: *mut jffs2_sb_info, vecs: *const kvec, count: usize, to: LoFF, retlen: *mut usize) -> i32;
    pub fn jffs2_flash_direct_write(c: *mut jffs2_sb_info, ofs: LoFF, len: usize, retlen: *mut usize, buf: *const u8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
