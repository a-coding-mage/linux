// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hpfs/buffer.c
 *
 *  Mikulas Patocka (mikulas@artax.karlin.mff.cuni.cz), 1998-1999
 *
 *  general buffer i/o
 */

use core::ffi::c_void;

pub type secno = u32;

#[repr(C)]
pub struct super_block;
#[repr(C)]
pub struct buffer_head {
    pub b_data: *mut u8,
}
#[repr(C)]
pub struct hpfs_sb_info {
    pub n_hotfixes: u32,
    pub hotfix_from: *mut secno,
    pub hotfix_to: *mut secno,
    pub sb_fs_size: secno,
}
#[repr(C)]
pub struct quad_buffer_head {
    pub bh: [*mut buffer_head; 4],
    pub data: *mut u8,
}
#[repr(C)]
pub struct blk_plug {
    _private: [u8; 0],
}

extern "C" {
    fn hpfs_sb(s: *mut super_block) -> *mut hpfs_sb_info;
    fn hpfs_lock_assert(s: *mut super_block);
    fn cond_resched();
    fn sb_find_get_block(s: *mut super_block, block: secno) -> *mut buffer_head;
    fn buffer_uptodate(bh: *mut buffer_head) -> bool;
    fn brelse(bh: *mut buffer_head);
    fn blk_start_plug(plug: *mut blk_plug);
    fn sb_breadahead(s: *mut super_block, block: secno);
    fn blk_finish_plug(plug: *mut blk_plug);
    fn sb_bread(s: *mut super_block, block: secno) -> *mut buffer_head;
    fn sb_getblk(s: *mut super_block, block: secno) -> *mut buffer_head;
    fn wait_on_buffer(bh: *mut buffer_head);
    fn set_buffer_uptodate(bh: *mut buffer_head);
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut u8);
    fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn pr_err(fmt: *const u8, ...);
    fn mark_buffer_dirty(bh: *mut buffer_head);
}

const GFP_NOFS: u32 = 0;

pub unsafe fn hpfs_search_hotfix_map(s: *mut super_block, sec: secno) -> secno {
    let sbi = hpfs_sb(s);
    let mut i: u32 = 0;
    while i < (*sbi).n_hotfixes {
        if *(*sbi).hotfix_from.add(i as usize) == sec {
            return *(*sbi).hotfix_to.add(i as usize);
        }
        i += 1;
    }
    sec
}

pub unsafe fn hpfs_search_hotfix_map_for_range(
    s: *mut super_block, sec: secno, mut n: u32,
) -> u32 {
    let sbi = hpfs_sb(s);
    let mut i: u32 = 0;
    while i < (*sbi).n_hotfixes {
        let from = *(*sbi).hotfix_from.add(i as usize);
        if from >= sec && from < sec.wrapping_add(n) {
            n = from.wrapping_sub(sec);
        }
        i += 1;
    }
    n
}

pub unsafe fn hpfs_prefetch_sectors(s: *mut super_block, mut secno: secno, mut n: i32) {
    let mut plug = core::mem::MaybeUninit::<blk_plug>::uninit();
    if n <= 0 || secno >= (*hpfs_sb(s)).sb_fs_size { return; }
    if hpfs_search_hotfix_map_for_range(s, secno, n as u32) != n as u32 { return; }
    let bh = sb_find_get_block(s, secno);
    if !bh.is_null() {
        if buffer_uptodate(bh) { brelse(bh); return; }
        brelse(bh);
    }
    blk_start_plug(plug.as_mut_ptr());
    while n > 0 {
        if secno >= (*hpfs_sb(s)).sb_fs_size { break; }
        sb_breadahead(s, secno);
        secno = secno.wrapping_add(1);
        n -= 1;
    }
    blk_finish_plug(plug.as_mut_ptr());
}

pub unsafe fn hpfs_map_sector(s: *mut super_block, secno: secno, bhp: *mut *mut buffer_head, ahead: i32) -> *mut u8 {
    hpfs_lock_assert(s);
    hpfs_prefetch_sectors(s, secno, ahead);
    cond_resched();
    let bh = sb_bread(s, hpfs_search_hotfix_map(s, secno));
    *bhp = bh;
    if !bh.is_null() { (*bh).b_data } else { pr_err(b"%s(): read error\n\0".as_ptr(),); core::ptr::null_mut() }
}

pub unsafe fn hpfs_get_sector(s: *mut super_block, secno: secno, bhp: *mut *mut buffer_head) -> *mut u8 {
    hpfs_lock_assert(s); cond_resched();
    let bh = sb_getblk(s, hpfs_search_hotfix_map(s, secno));
    *bhp = bh;
    if !bh.is_null() {
        if !buffer_uptodate(bh) { wait_on_buffer(bh); }
        set_buffer_uptodate(bh); (*bh).b_data
    } else { pr_err(b"%s(): getblk failed\n\0".as_ptr(),); core::ptr::null_mut() }
}

pub unsafe fn hpfs_map_4sectors(s: *mut super_block, secno: secno, qbh: *mut quad_buffer_head, ahead: i32) -> *mut u8 {
    hpfs_lock_assert(s); cond_resched();
    if secno & 3 != 0 { pr_err(b"%s(): unaligned read\n\0".as_ptr(),); return core::ptr::null_mut(); }
    hpfs_prefetch_sectors(s, secno, 4 + ahead);
    for i in 0..4 { if hpfs_map_sector(s, secno + i, (*qbh).bh.as_mut_ptr().add(i), 0).is_null() { for j in 0..i { brelse((*qbh).bh[j as usize]); } return core::ptr::null_mut(); } }
    let base = (*qbh).bh[0];
    if (*qbh).bh[1].is_null() || (*qbh).bh[1].read().b_data == (*base).b_data.add(512) && (*qbh).bh[2].read().b_data == (*base).b_data.add(1024) && (*qbh).bh[3].read().b_data == (*base).b_data.add(1536) {
        (*qbh).data = (*base).b_data; return (*qbh).data;
    }
    let data = kmalloc(2048, GFP_NOFS) as *mut u8;
    if data.is_null() { pr_err(b"%s(): out of memory\n\0".as_ptr(),); for i in (0..4).rev() { brelse((*qbh).bh[i]); } return core::ptr::null_mut(); }
    (*qbh).data = data;
    for i in 0..4 { memcpy(data.add(i * 512), (*qbh).bh[i].read().b_data, 512); }
    data
}

pub unsafe fn hpfs_get_4sectors(s: *mut super_block, secno: secno, qbh: *mut quad_buffer_head) -> *mut u8 {
    cond_resched(); hpfs_lock_assert(s);
    if secno & 3 != 0 { pr_err(b"%s(): unaligned read\n\0".as_ptr(),); return core::ptr::null_mut(); }
    for i in 0..4 { if hpfs_get_sector(s, secno + i, (*qbh).bh.as_mut_ptr().add(i)).is_null() { for j in 0..i { brelse((*qbh).bh[j as usize]); } return core::ptr::null_mut(); } }
    let base = (*qbh).bh[0];
    if (*qbh).bh[1].read().b_data == (*base).b_data.add(512) && (*qbh).bh[2].read().b_data == (*base).b_data.add(1024) && (*qbh).bh[3].read().b_data == (*base).b_data.add(1536) { (*qbh).data = (*base).b_data; return (*qbh).data; }
    (*qbh).data = kmalloc(2048, GFP_NOFS) as *mut u8;
    if (*qbh).data.is_null() { pr_err(b"%s(): out of memory\n\0".as_ptr(),); for i in (0..4).rev() { brelse((*qbh).bh[i]); } return core::ptr::null_mut(); }
    (*qbh).data
}

pub unsafe fn hpfs_brelse4(qbh: *mut quad_buffer_head) {
    if (*qbh).data != (*qbh).bh[0].read().b_data { kfree((*qbh).data); }
    for i in 0..4 { brelse((*qbh).bh[i]); }
}

pub unsafe fn hpfs_mark_4buffers_dirty(qbh: *mut quad_buffer_head) {
    if (*qbh).data != (*qbh).bh[0].read().b_data { for i in 0..4 { memcpy((*qbh).bh[i].read().b_data, (*qbh).data.add(i * 512), 512); } }
    for i in 0..4 { mark_buffer_dirty((*qbh).bh[i]); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
