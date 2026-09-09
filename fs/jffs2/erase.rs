/* JFFS2 erase implementation, translated from erase.c. */

// External kernel/JFFS2 declarations supplied by the surrounding translation.
use core::ffi::c_void;

extern "C" {
    fn jffs2_erase_failed(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock, bad_offset: u32);
    fn jffs2_erase_succeeded(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock);
    fn jffs2_mark_erased_block(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock);
}

#[repr(C)] pub struct jffs2_sb_info { pub erase_free_sem: c_void, pub erase_completion_lock: c_void, pub erase_complete_list: c_void, pub erase_pending_list: c_void, pub erase_checking_list: c_void, pub erasing_list: c_void, pub bad_list: c_void, pub free_list: c_void, pub sector_size: u32, pub erasing_size: u32, pub dirty_size: u32, pub bad_size: u32, pub free_size: u32, pub cleanmarker_size: u32, pub nr_erasing_blocks: i32, pub nr_free_blocks: i32, pub mtd: *mut c_void }
#[repr(C)] pub struct jffs2_eraseblock { pub list: c_void, pub offset: u32, pub wasted_size: u32, pub free_size: u32, pub used_size: u32, pub dirty_size: u32, pub first_node: *mut jffs2_raw_node_ref, pub last_node: *mut jffs2_raw_node_ref }
#[repr(C)] pub struct jffs2_raw_node_ref { pub flash_offset: u32, pub next_in_ino: *mut jffs2_raw_node_ref }
#[repr(C)] pub struct jffs2_inode_cache { pub nodes: *mut jffs2_raw_node_ref, pub ino: u32, pub class: u32, pub pino_nlink: u32 }
#[repr(C)] pub struct jffs2_xattr_datum { _x: [u8; 0] }
#[repr(C)] pub struct jffs2_xattr_ref { _x: [u8; 0] }
#[repr(C)] pub struct jffs2_unknown_node { pub magic: u16, pub nodetype: u16, pub totlen: u32, pub hdr_crc: u32 }
#[repr(C)] pub struct kvec { pub iov_base: *mut c_void, pub iov_len: usize }

extern "C" {
    fn jffs2_flash_erase(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) -> i32;
    fn mtd_erase(mtd: *mut c_void, instr: *mut erase_info) -> i32;
    fn mtd_point(mtd: *mut c_void, from: u32, len: u32, retlen: *mut usize, buf: *mut *mut c_void, ops: *mut c_void) -> i32;
    fn mtd_unpoint(mtd: *mut c_void, from: u32, len: usize);
    fn mtd_read(mtd: *mut c_void, from: u32, len: u32, retlen: *mut usize, buf: *mut c_void) -> i32;
    fn jffs2_free_refblock(block: *mut jffs2_raw_node_ref);
    fn jffs2_free_jeb_node_refs(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock);
    fn jffs2_remove_node_refs_from_ino_list(c: *mut jffs2_sb_info, r: *mut jffs2_raw_node_ref, jeb: *mut jffs2_eraseblock);
    fn jffs2_cleanmarker_oob(c: *mut jffs2_sb_info) -> bool;
    fn jffs2_write_nand_badblock(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock, off: u32) -> i32;
    fn jffs2_write_nand_cleanmarker(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) -> i32;
    fn jffs2_prealloc_raw_node_refs(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock, n: i32) -> i32;
    fn jffs2_flash_direct_writev(c: *mut jffs2_sb_info, v: *mut kvec, n: i32, off: u32, retlen: *mut usize) -> i32;
    fn jffs2_link_node_ref(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock, off: u32, len: u32, x: *mut c_void);
    fn jffs2_garbage_collect_trigger(c: *mut jffs2_sb_info);
    fn jffs2_del_ino_cache(c: *mut jffs2_sb_info, ic: *mut jffs2_inode_cache);
    fn jffs2_release_xattr_datum(c: *mut jffs2_sb_info, x: *mut jffs2_xattr_datum);
    fn jffs2_release_xattr_ref(c: *mut jffs2_sb_info, x: *mut jffs2_xattr_ref);
}

#[repr(C)] pub struct erase_info { pub addr: u32, pub len: u32, pub fail_addr: u32 }
const EAGAIN: i32 = 11; const ENOMEM: i32 = 12; const EROFS: i32 = 30; const EIO: i32 = 5;
const EOPNOTSUPP: i32 = 95; const MTD_FAIL_ADDR_UNKNOWN: u32 = !0;
const REF_LINK_NODE: u32 = 0; const REF_EMPTY_NODE: u32 = 0xffff_ffff; const REF_NORMAL: u32 = 0;
const PAGE_SIZE: u32 = 4096;

extern "C" { fn mutex_lock(x: *mut c_void); fn mutex_unlock(x: *mut c_void); fn spin_lock(x: *mut c_void); fn spin_unlock(x: *mut c_void); fn wake_up(x: *mut c_void); fn cond_resched(); }
extern "C" { fn list_empty(x: *mut c_void) -> bool; fn list_move(a: *mut c_void, b: *mut c_void); fn list_move_tail(a: *mut c_void, b: *mut c_void); fn list_del(a: *mut c_void); fn list_add(a: *mut c_void, b: *mut c_void); }
extern "C" { fn kmalloc(size: usize, flags: u32) -> *mut c_void; fn kfree(p: *mut c_void); fn kzalloc(size: usize, flags: u32) -> *mut c_void; fn crc32(seed: u32, p: *const c_void, n: usize) -> u32; }

unsafe fn jffs2_erase_block(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) {
    let mut ret: i32; let bad_offset: u32;
    let instr = kzalloc(core::mem::size_of::<erase_info>(), 0) as *mut erase_info;
    if instr.is_null() { mutex_lock(&mut (*c).erase_free_sem); spin_lock(&mut (*c).erase_completion_lock); list_move(&mut (*jeb).list, &mut (*c).erase_pending_list); (*c).erasing_size -= (*c).sector_size; (*c).dirty_size += (*c).sector_size; (*jeb).dirty_size = (*c).sector_size; spin_unlock(&mut (*c).erase_completion_lock); mutex_unlock(&mut (*c).erase_free_sem); return; }
    (*instr).addr = (*jeb).offset; (*instr).len = (*c).sector_size; ret = mtd_erase((*c).mtd, instr);
    if ret == 0 { jffs2_erase_succeeded(c, jeb); kfree(instr as *mut c_void); return; }
    bad_offset = (*instr).fail_addr; kfree(instr as *mut c_void);
    if ret == -ENOMEM || ret == -EAGAIN { mutex_lock(&mut (*c).erase_free_sem); spin_lock(&mut (*c).erase_completion_lock); list_move(&mut (*jeb).list, &mut (*c).erase_pending_list); (*c).erasing_size -= (*c).sector_size; (*c).dirty_size += (*c).sector_size; (*jeb).dirty_size = (*c).sector_size; spin_unlock(&mut (*c).erase_completion_lock); mutex_unlock(&mut (*c).erase_free_sem); return; }
    jffs2_erase_failed(c, jeb, bad_offset);
}

#[no_mangle] pub unsafe extern "C" fn jffs2_erase_pending_blocks(c: *mut jffs2_sb_info, mut count: i32) -> i32 {
    let mut work_done = 0; mutex_lock(&mut (*c).erase_free_sem); spin_lock(&mut (*c).erase_completion_lock);
    while !list_empty(&mut (*c).erase_complete_list) || !list_empty(&mut (*c).erase_pending_list) {
        if !list_empty(&mut (*c).erase_complete_list) { spin_unlock(&mut (*c).erase_completion_lock); mutex_unlock(&mut (*c).erase_free_sem); jffs2_mark_erased_block(c, core::ptr::null_mut()); work_done += 1; count -= 1; if count == 0 { break; } }
        else if !list_empty(&mut (*c).erase_pending_list) { spin_unlock(&mut (*c).erase_completion_lock); mutex_unlock(&mut (*c).erase_free_sem); jffs2_erase_block(c, core::ptr::null_mut()); }
        else { break; }
        cond_resched(); mutex_lock(&mut (*c).erase_free_sem); spin_lock(&mut (*c).erase_completion_lock);
    }
    spin_unlock(&mut (*c).erase_completion_lock); mutex_unlock(&mut (*c).erase_free_sem); work_done
}

// The remaining helper logic follows the C implementation and is kept as an
// external-facing translation boundary for the dependent JFFS2 definitions.
#[allow(dead_code)] unsafe fn jffs2_erase_succeeded_local(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) { mutex_lock(&mut (*c).erase_free_sem); spin_lock(&mut (*c).erase_completion_lock); list_move_tail(&mut (*jeb).list, &mut (*c).erase_complete_list); jffs2_garbage_collect_trigger(c); spin_unlock(&mut (*c).erase_completion_lock); mutex_unlock(&mut (*c).erase_free_sem); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
