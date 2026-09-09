// SPDX-License-Identifier: GPL-2.0-only
/* adi_64.c: support for ADI (Application Data Integrity) feature on
 * sparc m7 and newer processors. This feature is also known as
 * SSM (Silicon Secured Memory).
 *
 * Copyright (C) 2016 Oracle and/or its affiliates. All rights reserved.
 * Author: Khalid Aziz (khalid.aziz@oracle.com)
 */

/* C dependencies supplied by the surrounding kernel translation unit. */

const TAG_STORAGE_PAGES: usize = 8;

extern "C" {
    static mut adi_state: adi_config;
    fn mdesc_grab() -> *mut mdesc_handle;
    fn mdesc_node_by_name(hp: *mut mdesc_handle, node: u64, name: *const i8) -> u64;
    fn mdesc_get_property(hp: *mut mdesc_handle, node: u64, name: *const i8, len: *mut i32) -> *const u8;
    fn mdesc_release(hp: *mut mdesc_handle);
    fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn strlen(s: *const u8) -> usize;
    fn printk(format: *const i8, ...);
    fn kzalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn adi_blksize() -> usize;
    fn pte_val(pte: pte_t) -> usize;
}

#[repr(C)]
pub struct adi_config { pub enabled: bool, pub caps: adi_caps }
#[repr(C)]
pub struct adi_caps { pub blksz: u64, pub nbits: u64, pub ue_on_adi: u64 }
#[repr(C)] pub struct mdesc_handle;
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct pte_t;
#[repr(C)] pub struct mm_struct { pub context: mm_context }
#[repr(C)] pub struct vm_area_struct;
#[repr(C)] pub struct mm_context { pub tag_store: *mut tag_storage_desc_t, pub tag_lock: spinlock_t }
#[repr(C)] pub struct tag_storage_desc_t { pub start: usize, pub end: usize, pub tags: *mut u8, pub tag_users: usize }

const MDESC_NODE_NULL: u64 = 0;
const GFP_NOWAIT: u32 = 0;
const PAGE_SIZE: usize = 4096;
const ULONG_MAX: usize = usize::MAX;
const _PAGE_PADDR_4V: usize = 0;
const ASI_MCD_REAL: usize = 0;

pub unsafe fn mdesc_adi_init() {
    let hp = mdesc_grab();
    let mut prop: *const u8;
    let mut pn: u64;
    let mut val: *const u64;
    let mut len: i32;
    if hp.is_null() { goto_adi_not_found(hp); return; }
    pn = mdesc_node_by_name(hp, MDESC_NODE_NULL, b"cpu\0".as_ptr() as *const i8);
    if pn == MDESC_NODE_NULL { goto_adi_not_found(hp); return; }
    prop = mdesc_get_property(hp, pn, b"hwcap-list\0".as_ptr() as *const i8, &mut len);
    if prop.is_null() { goto_adi_not_found(hp); return; }
    adi_state.enabled = false;
    while len != 0 {
        if strcmp(prop as *const i8, b"adp\0".as_ptr() as *const i8) == 0 { adi_state.enabled = true; break; }
        let plen = strlen(prop) + 1;
        prop = prop.add(plen); len -= plen as i32;
    }
    if !adi_state.enabled { goto_adi_not_found(hp); return; }
    pn = mdesc_node_by_name(hp, MDESC_NODE_NULL, b"platform\0".as_ptr() as *const i8);
    if pn == MDESC_NODE_NULL { goto_adi_not_found(hp); return; }
    val = mdesc_get_property(hp, pn, b"adp-blksz\0".as_ptr() as *const i8, &mut len) as *const u64;
    if val.is_null() { goto_adi_not_found(hp); return; } adi_state.caps.blksz = *val;
    val = mdesc_get_property(hp, pn, b"adp-nbits\0".as_ptr() as *const i8, &mut len) as *const u64;
    if val.is_null() { goto_adi_not_found(hp); return; } adi_state.caps.nbits = *val;
    val = mdesc_get_property(hp, pn, b"ue-on-adp\0".as_ptr() as *const i8, &mut len) as *const u64;
    if val.is_null() { goto_adi_not_found(hp); return; } adi_state.caps.ue_on_adi = *val;
    if adi_state.caps.nbits > 4 { printk(b"WARNING: ADI tag size >4 on this platform. Disabling AADI support\n\0".as_ptr() as *const i8); adi_state.enabled = false; }
    mdesc_release(hp); return;
}

unsafe fn goto_adi_not_found(hp: *mut mdesc_handle) {
    adi_state.enabled = false; adi_state.caps.blksz = 0; adi_state.caps.nbits = 0;
    if !hp.is_null() { mdesc_release(hp); }
}

unsafe fn find_tag_store(mm: *mut mm_struct, _vma: *mut vm_area_struct, addr: usize) -> *mut tag_storage_desc_t {
    let mut tag_desc = (*mm).context.tag_store;
    let max_desc = PAGE_SIZE / core::mem::size_of::<tag_storage_desc_t>();
    if !tag_desc.is_null() {
        let mut flags = 0; spin_lock_irqsave(&mut (*mm).context.tag_lock, &mut flags);
        let mut i = 0; while i < max_desc { if addr >= (*tag_desc).start && addr + PAGE_SIZE - 1 <= (*tag_desc).end { break; } tag_desc = tag_desc.add(1); i += 1; }
        spin_unlock_irqrestore(&mut (*mm).context.tag_lock, flags); if i >= max_desc { tag_desc = core::ptr::null_mut(); }
    }
    tag_desc
}

unsafe fn alloc_tag_store(mm: *mut mm_struct, _vma: *mut vm_area_struct, mut addr: usize) -> *mut tag_storage_desc_t {
    let max_desc = PAGE_SIZE / core::mem::size_of::<tag_storage_desc_t>(); let mut flags = 0; let mut open_desc = core::ptr::null_mut(); let mut hole_start = 0; let mut hole_end = ULONG_MAX; let mut size; let mut end_addr = addr + PAGE_SIZE - 1;
    spin_lock_irqsave(&mut (*mm).context.tag_lock, &mut flags);
    if (*mm).context.tag_store.is_null() { size = core::mem::size_of::<tag_storage_desc_t>() * max_desc; (*mm).context.tag_store = kzalloc(size, GFP_NOWAIT) as *mut tag_storage_desc_t; if (*mm).context.tag_store.is_null() { spin_unlock_irqrestore(&mut (*mm).context.tag_lock, flags); return core::ptr::null_mut(); } for i in 0..max_desc { (*(*mm).context.tag_store.add(i)).tag_users = 0; } open_desc = (*mm).context.tag_store; } else {
        let mut d = (*mm).context.tag_store; for _ in 0..max_desc { if (*d).tag_users == 0 { if open_desc.is_null() { open_desc = d; } } else if addr >= (*d).start && (*d).end >= addr + PAGE_SIZE - 1 { (*d).tag_users += 1; spin_unlock_irqrestore(&mut (*mm).context.tag_lock, flags); return d; } if (*d).start > end_addr && (*d).start < hole_end { hole_end = (*d).start; } if (*d).end < addr && (*d).end > hole_start { hole_start = (*d).end; } d = d.add(1); }
    }
    if open_desc.is_null() { spin_unlock_irqrestore(&mut (*mm).context.tag_lock, flags); return core::ptr::null_mut(); }
    let d = open_desc; (*d).tag_users = 1; size = TAG_STORAGE_PAGES * PAGE_SIZE; end_addr = addr + size * 2 * adi_blksize() - 1;
    if end_addr < addr { size = PAGE_SIZE; end_addr = addr + size * 2 * adi_blksize() - 1; if end_addr < addr { end_addr = ULONG_MAX; } }
    if hole_end < end_addr { end_addr = hole_end - 1; let mut tmp = end_addr - size * 2 * adi_blksize() + 1; if tmp > addr { size = PAGE_SIZE; tmp = end_addr - size * 2 * adi_blksize() - 1; if tmp > addr { tmp = 0; } } if tmp < hole_start { tmp = hole_start + 1; } addr = tmp; size = (end_addr + 1 - addr) / (2 * adi_blksize()); size = (size + PAGE_SIZE - adi_blksize()) / PAGE_SIZE * PAGE_SIZE; }
    (*d).tags = kzalloc(size, GFP_NOWAIT); if (*d).tags.is_null() { (*d).tag_users = 0; spin_unlock_irqrestore(&mut (*mm).context.tag_lock, flags); return core::ptr::null_mut(); } (*d).start = addr; (*d).end = end_addr; spin_unlock_irqrestore(&mut (*mm).context.tag_lock, flags); d
}

unsafe fn del_tag_store(tag_desc: *mut tag_storage_desc_t, mm: *mut mm_struct) { let mut flags = 0; let mut tags = core::ptr::null_mut(); spin_lock_irqsave(&mut (*mm).context.tag_lock, &mut flags); (*tag_desc).tag_users -= 1; if (*tag_desc).tag_users == 0 { (*tag_desc).start = 0; (*tag_desc).end = 0; if tag_desc != (*mm).context.tag_store { tags = (*tag_desc).tags; (*tag_desc).tags = core::ptr::null_mut(); } } spin_unlock_irqrestore(&mut (*mm).context.tag_lock, flags); kfree(tags); }

unsafe fn tag_start(addr: usize, d: *mut tag_storage_desc_t) -> *mut u8 { (*d).tags.add((addr - (*d).start) / (2 * adi_blksize())) }

pub unsafe fn adi_restore_tags(mm: *mut mm_struct, vma: *mut vm_area_struct, addr: usize, pte: pte_t) { let d = find_tag_store(mm, vma, addr); if d.is_null() { return; } let mut tag = tag_start(addr, d); let paddr = pte_val(pte) & _PAGE_PADDR_4V; let mut tmp = paddr; while tmp < paddr + PAGE_SIZE { let version1 = (*tag >> 4) as usize; let version2 = (*tag & 0x0f) as usize; *tag = 0; tmp += adi_blksize(); tmp += adi_blksize(); let _ = (version1, version2, tmp); tag = tag.add(1); } del_tag_store(d, mm); }

pub unsafe fn adi_save_tags(mm: *mut mm_struct, vma: *mut vm_area_struct, addr: usize, oldpte: pte_t) -> i32 { let d = alloc_tag_store(mm, vma, addr); if d.is_null() { return -1; } let mut tag = tag_start(addr, d); let paddr = pte_val(oldpte) & _PAGE_PADDR_4V; let mut tmp = paddr; while tmp < paddr + PAGE_SIZE { let version1: usize = 0; let version2: usize = 0; tmp += adi_blksize(); tmp += adi_blksize(); *tag = ((version1 << 4) | version2) as u8; tag = tag.add(1); } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
