// SPDX-License-Identifier: GPL-2.0

/* Rust translation of kho_block.c. External kernel/KHO declarations are
 * intentionally left to the surrounding translation unit. */

use core::ffi::c_void;

pub const KHO_MAX_BLOCKS: usize = 10000;

#[repr(C)]
pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }

#[repr(C)]
pub struct KhoBlockHeaderSer {
    pub next: u64,
    pub count: u64,
}

#[repr(C)]
pub struct KhoBlock { pub list: ListHead, pub ser: *mut KhoBlockHeaderSer }

#[repr(C)]
pub struct KhoBlockSet {
    pub blocks: ListHead,
    pub entry_size: usize,
    pub count_per_block: u64,
    pub nblocks: u64,
    pub head_pa: u64,
    pub incoming: bool,
}

#[repr(C)]
pub struct KhoBlockSetIt {
    pub bs: *mut KhoBlockSet,
    pub block: *mut KhoBlock,
    pub i: u64,
}

extern "C" {
    fn warn_on_once(condition: bool) -> bool;
    fn kho_restore_free(ptr: *mut KhoBlockHeaderSer);
    fn kho_unpreserve_free(ptr: *mut KhoBlockHeaderSer);
    fn kho_alloc_preserve(size: usize) -> *mut KhoBlockHeaderSer;
    fn virt_to_phys(ptr: *mut KhoBlockHeaderSer) -> u64;
    fn phys_to_virt(pa: u64) -> *mut KhoBlockHeaderSer;
    fn kzalloc_block() -> *mut KhoBlock;
    fn kfree(ptr: *mut KhoBlock);
    fn pr_err(msg: *const u8);
    fn pr_warn(msg: *const u8, count: u64);
}

const KHO_BLOCK_SIZE: usize = 4096;
const ENOSPC: i32 = 28;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;

unsafe fn block_entries(block: *mut KhoBlock) -> *mut u8 {
    (*block).ser.add(1) as *mut u8
}

unsafe fn block_entry(it: *mut KhoBlockSetIt, index: u64) -> *mut c_void {
    block_entries((*it).block).add(index as usize * (*(*it).bs).entry_size) as *mut c_void
}

unsafe fn block_free_ser(bs: *mut KhoBlockSet, ser: *mut KhoBlockHeaderSer) {
    if (*bs).incoming { kho_restore_free(ser); } else { kho_unpreserve_free(ser); }
}

unsafe fn block_alloc_ser(bs: *mut KhoBlockSet) -> *mut KhoBlockHeaderSer {
    let _ = warn_on_once((*bs).incoming);
    kho_alloc_preserve(KHO_BLOCK_SIZE)
}

unsafe fn block_add(bs: *mut KhoBlockSet, ser: *mut KhoBlockHeaderSer) -> i32 {
    if (*bs).nblocks >= KHO_MAX_BLOCKS as u64 { return -ENOSPC; }
    let block = kzalloc_block();
    if block.is_null() { return -ENOMEM; }
    (*block).ser = ser;
    let head = &mut (*bs).blocks;
    let list = &mut (*block).list;
    list.prev = head.prev;
    list.next = head;
    (*head.prev).next = list;
    head.prev = list;
    (*bs).nblocks += 1;
    if list.prev != head as *mut ListHead {
        let last = (list.prev as *mut u8).sub(core::mem::offset_of!(KhoBlock, list)) as *mut KhoBlock;
        (*(*last).ser).next = virt_to_phys(ser);
    } else { (*bs).head_pa = virt_to_phys(ser); }
    0
}

unsafe fn block_set_grow_one(bs: *mut KhoBlockSet) -> i32 {
    let ser = block_alloc_ser(bs);
    if ser.is_null() { return -ENOMEM; }
    let err = block_add(bs, ser);
    if err != 0 { block_free_ser(bs, ser); }
    err
}

unsafe fn block_set_shrink_one(bs: *mut KhoBlockSet) {
    let head = &mut (*bs).blocks;
    if head.prev == head as *mut ListHead { return; }
    let list = head.prev;
    let last = (list as *mut u8).sub(core::mem::offset_of!(KhoBlock, list)) as *mut KhoBlock;
    (*list.prev).next = list.next;
    (*list.next).prev = list.prev;
    (*bs).nblocks -= 1;
    block_free_ser(bs, (*last).ser);
    kfree(last);
    if head.prev != head as *mut ListHead {
        let new_last = (head.prev as *mut u8).sub(core::mem::offset_of!(KhoBlock, list)) as *mut KhoBlock;
        (*(*new_last).ser).next = 0;
    } else { (*bs).head_pa = 0; }
}

pub unsafe fn kho_block_set_init(bs: *mut KhoBlockSet, entry_size: usize) {
    // KHO_BLOCK_SET_INIT(*bs, entry_size), with the surrounding ABI's layout.
    (*bs).entry_size = entry_size;
    (*bs).count_per_block = ((KHO_BLOCK_SIZE - core::mem::size_of::<KhoBlockHeaderSer>()) / entry_size) as u64;
    (*bs).nblocks = 0; (*bs).head_pa = 0; (*bs).incoming = false;
    (*bs).blocks.next = &mut (*bs).blocks; (*bs).blocks.prev = &mut (*bs).blocks;
    let _ = warn_on_once((*bs).count_per_block == 0);
}

pub unsafe fn kho_block_set_grow(bs: *mut KhoBlockSet, count: u64) -> i32 {
    let orig = (*bs).nblocks;
    if warn_on_once((*bs).incoming) { return -EINVAL; }
    while count > (*bs).nblocks * (*bs).count_per_block {
        let err = block_set_grow_one(bs); if err != 0 { while (*bs).nblocks > orig { block_set_shrink_one(bs); } return err; }
    } 0
}

pub unsafe fn kho_block_set_shrink(bs: *mut KhoBlockSet, count: u64) {
    while (*bs).nblocks > 0 && count <= ((*bs).nblocks - 1) * (*bs).count_per_block { block_set_shrink_one(bs); }
}

pub unsafe fn kho_block_set_restore(bs: *mut KhoBlockSet, mut next_pa: u64) -> i32 {
    (*bs).incoming = true; if next_pa == 0 { return 0; }
    (*bs).head_pa = next_pa;
    while next_pa != 0 {
        let ser = phys_to_virt(next_pa);
        if (*ser).count == 0 || (*ser).count > (*bs).count_per_block { pr_warn(b"Block contains invalid entry count: %llu\0".as_ptr(), (*ser).count); kho_block_set_destroy(bs); return -EINVAL; }
        let err = block_add(bs, ser); if err != 0 { kho_block_set_destroy(bs); return err; }
        next_pa = (*ser).next;
    } 0
}

pub unsafe fn kho_block_set_destroy(bs: *mut KhoBlockSet) {
    while (*bs).blocks.prev != &mut (*bs).blocks as *mut ListHead { block_set_shrink_one(bs); }
    (*bs).nblocks = 0; (*bs).head_pa = 0;
}

pub unsafe fn kho_block_set_clear(bs: *mut KhoBlockSet) {
    let mut p = (*bs).blocks.next;
    while p != &mut (*bs).blocks as *mut ListHead {
        let block = (p as *mut u8).sub(core::mem::offset_of!(KhoBlock, list)) as *mut KhoBlock;
        (*(*block).ser).count = 0;
        core::ptr::write_bytes((*block).ser.add(1) as *mut u8, 0, KHO_BLOCK_SIZE - core::mem::size_of::<KhoBlockHeaderSer>());
        p = (*p).next;
    }
}

pub unsafe fn kho_block_set_it_init(it: *mut KhoBlockSetIt, bs: *mut KhoBlockSet) {
    (*it).bs = bs;
    (*it).block = if (*bs).blocks.next == &mut (*bs).blocks as *mut ListHead { core::ptr::null_mut() } else { ((*bs).blocks.next as *mut u8).sub(core::mem::offset_of!(KhoBlock, list)) as *mut KhoBlock };
    (*it).i = 0;
}

pub unsafe fn kho_block_set_it_reserve_entry(it: *mut KhoBlockSetIt) -> *mut c_void {
    if (*it).block.is_null() { return core::ptr::null_mut(); }
    if (*it).i == (*(*it).bs).count_per_block { return core::ptr::null_mut(); }
    let entry = block_entry(it, (*it).i); (*it).i += 1; (*(*it).block).ser.as_mut().unwrap().count = (*it).i; entry
}

pub unsafe fn kho_block_set_it_read_entry(it: *mut KhoBlockSetIt) -> *mut c_void { if (*it).block.is_null() { core::ptr::null_mut() } else { let p = block_entry(it, (*it).i); (*it).i += 1; p } }

pub unsafe fn kho_block_set_it_prev(it: *mut KhoBlockSetIt) -> *mut c_void { if (*it).block.is_null() || (*it).i == 0 { core::ptr::null_mut() } else { (*it).i -= 1; block_entry(it, (*it).i) } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
