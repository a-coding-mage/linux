/*
 * A Remote Heap.  Rust translation of rheap.c.
 * The types and kernel/list primitives referenced here are supplied by the
 * corresponding platform headers/bindings.
 */

use crate::{rh_block_t, rh_info_t, rh_stats_t, list_head, RHGS_FREE, RHGS_TAKEN,
    RHIF_STATIC_BLOCK, RHIF_STATIC_INFO};

extern "C" {
    fn kmalloc_objs(size: usize, flags: u32) -> *mut rh_block_t;
    fn kmalloc_obj(size: usize, flags: u32) -> *mut rh_info_t;
    fn kfree(p: *mut core::ffi::c_void);
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn printk(fmt: *const i8, ...);
    fn list_add(n: *mut list_head, h: *mut list_head);
    fn list_add_tail(n: *mut list_head, h: *mut list_head);
    fn list_del(n: *mut list_head);
    fn list_del_init(n: *mut list_head);
    fn init_list_head(h: *mut list_head);
    fn err_ptr(e: isize) -> *mut rh_info_t;
    fn list_entry(l: *mut list_head) -> *mut rh_block_t;
}

const EINVAL: isize = 22;
const ENOMEM: isize = 12;
const ERANGE: isize = 34;
const GFP_ATOMIC: u32 = 0x20;

unsafe fn fixup(s: usize, e: usize, d: isize, l: *mut list_head) {
    let p = l as *mut usize;
    if *p >= s && *p < e { *p = (*p as isize).wrapping_add(d) as usize; }
    let p = (l as *mut u8).add(core::mem::size_of::<usize>()) as *mut usize;
    if *p >= s && *p < e { *p = (*p as isize).wrapping_add(d) as usize; }
}

unsafe fn grow(info: *mut rh_info_t, max_blocks: i32) -> i32 {
    if max_blocks <= (*info).max_blocks { return -(EINVAL as i32); }
    let new_blocks = max_blocks - (*info).max_blocks;
    let block = kmalloc_objs(core::mem::size_of::<rh_block_t>() * max_blocks as usize, GFP_ATOMIC);
    if block.is_null() { return -(ENOMEM as i32); }
    if (*info).max_blocks > 0 {
        memcpy(block as *mut _, (*info).block as *const _, core::mem::size_of::<rh_block_t>() * (*info).max_blocks as usize);
        let delta = block as isize - (*info).block as isize;
        let blks = (*info).block as usize;
        let blke = blks + core::mem::size_of::<rh_block_t>() * (*info).max_blocks as usize;
        for i in 0..(*info).max_blocks as usize { fixup(blks, blke, delta, &mut (*block.add(i)).list); }
        fixup(blks, blke, delta, &mut (*info).empty_list);
        fixup(blks, blke, delta, &mut (*info).free_list);
        fixup(blks, blke, delta, &mut (*info).taken_list);
        if (*info).flags & RHIF_STATIC_BLOCK == 0 { kfree((*info).block as *mut _); }
    }
    (*info).block = block;
    (*info).empty_slots += new_blocks;
    (*info).max_blocks = max_blocks;
    (*info).flags &= !RHIF_STATIC_BLOCK;
    for i in 0..new_blocks as usize { list_add(&mut (*block.add((*info).max_blocks - new_blocks) as *mut rh_block_t).add(i).list, &mut (*info).empty_list); }
    0
}

unsafe fn assure_empty(info: *mut rh_info_t, slots: i32) -> i32 {
    if slots >= 4 { return -(EINVAL as i32); }
    if (*info).empty_slots >= slots { return 0; }
    grow(info, (((*info).max_blocks + slots) + 15) & !15)
}

unsafe fn get_slot(info: *mut rh_info_t) -> *mut rh_block_t {
    if (*info).empty_slots == 0 { return core::ptr::null_mut(); }
    let blk = list_entry((*info).empty_list.next);
    list_del_init(&mut (*blk).list);
    (*info).empty_slots -= 1;
    (*blk).start = 0; (*blk).size = 0; (*blk).owner = core::ptr::null();
    blk
}
unsafe fn release_slot(info: *mut rh_info_t, blk: *mut rh_block_t) {
    list_add(&mut (*blk).list, &mut (*info).empty_list); (*info).empty_slots += 1;
}

unsafe fn attach_free_block(info: *mut rh_info_t, blkn: *mut rh_block_t) {
    let size = (*blkn).size; let s = (*blkn).start; let e = s + size;
    let mut before = core::ptr::null_mut(); let mut after = core::ptr::null_mut(); let mut next = core::ptr::null_mut();
    let mut l = (*info).free_list.next;
    while l != &mut (*info).free_list as *mut _ {
        let blk = list_entry(l); let bs = (*blk).start; let be = bs + (*blk).size;
        if next.is_null() && s >= bs { next = blk; }
        if be == s { before = blk; } if e == bs { after = blk; }
        if !before.is_null() && !after.is_null() { break; } l = (*l).next;
    }
    if !before.is_null() && s != (*before).start + (*before).size { before = core::ptr::null_mut(); }
    if !after.is_null() && e != (*after).start { after = core::ptr::null_mut(); }
    if before.is_null() && after.is_null() { if !next.is_null() { list_add(&mut (*blkn).list, &mut (*next).list); } else { list_add(&mut (*blkn).list, &mut (*info).free_list); } return; }
    release_slot(info, blkn);
    if !before.is_null() && after.is_null() { (*before).size += size; return; }
    if before.is_null() && !after.is_null() { (*after).start -= size; (*after).size += size; return; }
    (*before).size += size + (*after).size; list_del(&mut (*after).list); release_slot(info, after);
}

unsafe fn attach_taken_block(info: *mut rh_info_t, blkn: *mut rh_block_t) {
    let mut l = (*info).taken_list.next;
    while l != &mut (*info).taken_list as *mut _ { let blk = list_entry(l); if (*blk).start > (*blkn).start { list_add_tail(&mut (*blkn).list, &mut (*blk).list); return; } l = (*l).next; }
    list_add_tail(&mut (*blkn).list, &mut (*info).taken_list);
}

pub unsafe extern "C" fn rh_create(alignment: u32) -> *mut rh_info_t {
    if alignment & (alignment - 1) != 0 { return err_ptr(-EINVAL); }
    let info = kmalloc_obj(core::mem::size_of::<rh_info_t>(), GFP_ATOMIC); if info.is_null() { return err_ptr(-ENOMEM); }
    (*info).alignment = alignment; (*info).block = core::ptr::null_mut(); (*info).max_blocks = 0; (*info).empty_slots = 0; (*info).flags = 0;
    init_list_head(&mut (*info).empty_list); init_list_head(&mut (*info).free_list); init_list_head(&mut (*info).taken_list); info
}

pub unsafe extern "C" fn rh_destroy(info: *mut rh_info_t) { if (*info).flags & RHIF_STATIC_BLOCK == 0 { kfree((*info).block as *mut _); } if (*info).flags & RHIF_STATIC_INFO == 0 { kfree(info as *mut _); } }

pub unsafe extern "C" fn rh_init(info: *mut rh_info_t, alignment: u32, max_blocks: i32, block: *mut rh_block_t) {
    if alignment & (alignment - 1) != 0 { return; }
    (*info).alignment = alignment; (*info).block = block; (*info).max_blocks = max_blocks; (*info).empty_slots = max_blocks; (*info).flags = RHIF_STATIC_INFO | RHIF_STATIC_BLOCK;
    init_list_head(&mut (*info).empty_list); init_list_head(&mut (*info).free_list); init_list_head(&mut (*info).taken_list);
    for i in 0..max_blocks as usize { list_add(&mut (*block.add(i)).list, &mut (*info).empty_list); }
}

pub unsafe extern "C" fn rh_attach_region(info: *mut rh_info_t, start: usize, size: i32) -> i32 {
    let m = (*info).alignment as usize - 1; let s = (start + m) & !m; let e = (start + size as usize) & !m; if e < s { return -(ERANGE as i32); }
    if assure_empty(info, 1) < 0 { return -(ENOMEM as i32); } let blk = get_slot(info); (*blk).start = s; (*blk).size = e - s; (*blk).owner = core::ptr::null(); attach_free_block(info, blk); 0
}

pub unsafe extern "C" fn rh_alloc(info: *mut rh_info_t, size: i32, owner: *const i8) -> usize { rh_alloc_align(info, size, (*info).alignment as i32, owner) }

pub unsafe extern "C" fn rh_alloc_align(info: *mut rh_info_t, mut size: i32, alignment: i32, owner: *const i8) -> usize {
    if size <= 0 || alignment <= 0 || (alignment & (alignment - 1)) != 0 { return (-EINVAL) as usize; }
    size = (size + (*info).alignment as i32 - 1) & !((*info).alignment as i32 - 1); if assure_empty(info, 2) < 0 { return (-ENOMEM) as usize; }
    let mut l = (*info).free_list.next; let mut blk = core::ptr::null_mut(); let mut start = 0usize;
    while l != &mut (*info).free_list as *mut _ { let b = list_entry(l); if size as usize <= (*b).size { start = ((*b).start + alignment as usize - 1) & !(alignment as usize - 1); if start + size as usize <= (*b).start + (*b).size { blk = b; break; } } l = (*l).next; }
    if blk.is_null() { return (-ENOMEM) as usize; }
    let newblk; if (*blk).size == size as usize { list_del(&mut (*blk).list); newblk = blk; } else { let sp = start - (*blk).start; if sp != 0 { let p = get_slot(info); (*p).start = (*blk).start; (*p).size = sp; list_add(&mut (*p).list, (*blk).list.prev); } let p = get_slot(info); (*p).start = start; (*p).size = size as usize; newblk = p; (*blk).start = start + size as usize; (*blk).size -= sp + size as usize; if (*blk).size == 0 { list_del(&mut (*blk).list); release_slot(info, blk); } }
    (*newblk).owner = owner; attach_taken_block(info, newblk); start
}

pub unsafe extern "C" fn rh_detach_region(info: *mut rh_info_t, start: usize, size: i32) -> usize {
    if size <= 0 { return (-EINVAL) as usize; }
    let m = (*info).alignment as usize - 1; let s = (start + m) & !m; let e = (start + size as usize) & !m;
    if assure_empty(info, 1) < 0 { return (-ENOMEM) as usize; }
    let mut l = (*info).free_list.next; let mut blk = core::ptr::null_mut(); let (mut bs, mut be) = (0, 0);
    while l != &mut (*info).free_list as *mut _ { let b = list_entry(l); bs=(*b).start; be=bs+(*b).size; if s>=bs && e<=be {blk=b;break;} l=(*l).next; }
    if blk.is_null() { return (-ENOMEM) as usize; }
    if bs==s && be==e { list_del(&mut (*blk).list); release_slot(info,blk); return s; }
    if bs==s || be==e { if bs==s {(*blk).start += size as usize;} (*blk).size -= size as usize; }
    else { (*blk).size=s-bs; let p=get_slot(info); (*p).start=e; (*p).size=be-e; list_add(&mut (*p).list,&mut (*blk).list); }
    s
}

pub unsafe extern "C" fn rh_alloc_fixed(info: *mut rh_info_t, start: usize, size: i32, owner: *const i8) -> usize {
    if size <= 0 { return (-EINVAL) as usize; } let m = (*info).alignment as usize - 1; let s = (start + m) & !m; let e = (start + size as usize) & !m; if assure_empty(info, 2) < 0 { return (-ENOMEM) as usize; }
    let mut l = (*info).free_list.next; let mut blk = core::ptr::null_mut(); let (mut bs, mut be) = (0,0); while l != &mut (*info).free_list as *mut _ { let b = list_entry(l); bs=(*b).start; be=bs+(*b).size; if s>=bs && e<=be { blk=b; break; } l=(*l).next; } if blk.is_null(){return (-ENOMEM) as usize;}
    if bs==s && be==e { list_del(&mut (*blk).list); (*blk).owner=owner; attach_taken_block(info,blk); return s; }
    if bs==s || be==e { if bs==s {(*blk).start += size as usize;} (*blk).size -= size as usize; } else { (*blk).size=s-bs; let p=get_slot(info); (*p).start=e; (*p).size=be-e; list_add(&mut (*p).list,&mut (*blk).list); }
    let p=get_slot(info); (*p).start=s; (*p).size=e-s; (*p).owner=owner; attach_taken_block(info,p); s
}

pub unsafe extern "C" fn rh_free(info: *mut rh_info_t, start: usize) -> i32 { let mut l=(*info).taken_list.next; let mut blk=core::ptr::null_mut(); while l != &mut (*info).taken_list as *mut _ { let b=list_entry(l); if start < (*b).start {break;} blk=b; l=(*l).next;} if blk.is_null() || start > (*blk).start+(*blk).size{return -(EINVAL as i32);} list_del(&mut (*blk).list); let n=(*blk).size as i32; attach_free_block(info,blk); n }

pub unsafe extern "C" fn rh_get_stats(info:*mut rh_info_t, what:i32, max_stats:i32, stats:*mut rh_stats_t)->i32 { let h=if what==RHGS_FREE{&mut (*info).free_list}else if what==RHGS_TAKEN{&mut (*info).taken_list}else{return -(EINVAL as i32)}; let mut n=0; let mut l=h.next; while l != h as *mut _ {let b=list_entry(l); if !stats.is_null()&&n<max_stats{(*stats).start=(*b).start;(*stats).size=(*b).size;(*stats).owner=(*b).owner;stats=stats.add(1);}n+=1;l=(*l).next;} n}

pub unsafe extern "C" fn rh_set_owner(info:*mut rh_info_t,start:usize,owner:*const i8)->i32 {let mut l=(*info).taken_list.next;let mut b=core::ptr::null_mut();while l!=&mut (*info).taken_list as *mut _{let x=list_entry(l);if start<(*x).start{break;}b=x;l=(*l).next;}if b.is_null()||start>(*b).start+(*b).size{return -(EINVAL as i32)}(*b).owner=owner;(*b).size as i32}

pub unsafe extern "C" fn rh_dump_blk(_info:*mut rh_info_t,_blk:*mut rh_block_t) {}
pub unsafe extern "C" fn rh_dump(_info:*mut rh_info_t) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
