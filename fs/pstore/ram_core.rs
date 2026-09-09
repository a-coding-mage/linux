// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Google, Inc.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct persistent_ram_buffer {
    pub sig: u32,
    pub start: atomic_t,
    pub size: atomic_t,
    pub data: [u8; 0],
}

pub const PERSISTENT_RAM_SIG: u32 = 0x43474244;

unsafe fn buffer_size(prz: *mut persistent_ram_zone) -> usize {
    atomic_read(&mut (*(*prz).buffer).size) as usize
}

unsafe fn buffer_start(prz: *mut persistent_ram_zone) -> usize {
    atomic_read(&mut (*(*prz).buffer).start) as usize
}

unsafe fn buffer_start_add(prz: *mut persistent_ram_zone, a: usize) -> usize {
    let mut old: c_int;
    let mut new: c_int;
    let mut flags: c_ulong = 0;
    if (*prz).flags & PRZ_FLAG_NO_LOCK == 0 { raw_spin_lock_irqsave(&mut (*prz).buffer_lock, &mut flags); }
    old = atomic_read(&mut (*(*prz).buffer).start);
    new = old.wrapping_add(a as c_int);
    while new >= (*prz).buffer_size as c_int { new -= (*prz).buffer_size as c_int; }
    atomic_set(&mut (*(*prz).buffer).start, new);
    if (*prz).flags & PRZ_FLAG_NO_LOCK == 0 { raw_spin_unlock_irqrestore(&mut (*prz).buffer_lock, flags); }
    old as usize
}

unsafe fn buffer_size_add(prz: *mut persistent_ram_zone, a: usize) {
    let mut flags: c_ulong = 0;
    if (*prz).flags & PRZ_FLAG_NO_LOCK == 0 { raw_spin_lock_irqsave(&mut (*prz).buffer_lock, &mut flags); }
    let old = atomic_read(&mut (*(*prz).buffer).size) as usize;
    if old != (*prz).buffer_size {
        let new = core::cmp::min(old + a, (*prz).buffer_size);
        atomic_set(&mut (*(*prz).buffer).size, new as c_int);
    }
    if (*prz).flags & PRZ_FLAG_NO_LOCK == 0 { raw_spin_unlock_irqrestore(&mut (*prz).buffer_lock, flags); }
}

unsafe fn persistent_ram_encode_rs8(prz: *mut persistent_ram_zone, data: *mut u8, len: usize, ecc: *mut u8) {
    memset((*prz).ecc_info.par as *mut c_void, 0, (*prz).ecc_info.ecc_size * core::mem::size_of::<u8>());
    encode_rs8((*prz).rs_decoder, data, len, (*prz).ecc_info.par, 0);
    for i in 0..(*prz).ecc_info.ecc_size { *ecc.add(i) = *(*prz).ecc_info.par.add(i); }
}

unsafe fn persistent_ram_decode_rs8(prz: *mut persistent_ram_zone, data: *mut c_void, len: usize, ecc: *mut u8) -> c_int {
    for i in 0..(*prz).ecc_info.ecc_size { *(*prz).ecc_info.par.add(i) = *ecc.add(i); }
    decode_rs8((*prz).rs_decoder, data, (*prz).ecc_info.par, len, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0, core::ptr::null_mut())
}

unsafe fn persistent_ram_update_ecc(prz: *mut persistent_ram_zone, start: c_uint, count: c_uint) {
    let buffer = (*prz).buffer;
    let buffer_end = (*buffer).data.as_mut_ptr().add((*prz).buffer_size);
    let block_size = (*prz).ecc_info.block_size;
    let ecc_size = (*prz).ecc_info.ecc_size;
    if ecc_size == 0 { return; }
    let mut block = (*buffer).data.as_mut_ptr().add((start as usize) & !(block_size - 1));
    let mut par = (*prz).par_buffer.add((start as usize / block_size) * ecc_size);
    let mut size = block_size;
    loop {
        if block.add(block_size) > buffer_end { size = buffer_end.offset_from(block) as usize; }
        persistent_ram_encode_rs8(prz, block, size, par);
        block = block.add(block_size); par = par.add(ecc_size);
        if block >= (*buffer).data.as_mut_ptr().add(start as usize + count as usize) { break; }
    }
}

unsafe fn persistent_ram_update_header_ecc(prz: *mut persistent_ram_zone) {
    if (*prz).ecc_info.ecc_size != 0 { persistent_ram_encode_rs8(prz, (*prz).buffer as *mut u8, core::mem::size_of::<persistent_ram_buffer>(), (*prz).par_header); }
}

unsafe fn persistent_ram_ecc_old(prz: *mut persistent_ram_zone) {
    if (*prz).ecc_info.ecc_size == 0 { return; }
    let mut block = (*prz).buffer.data.as_mut_ptr();
    let mut par = (*prz).par_buffer;
    while block < (*prz).buffer.data.as_mut_ptr().add(buffer_size(prz)) {
        let mut size = (*prz).ecc_info.block_size;
        if block.add(size) > (*prz).buffer.data.as_mut_ptr().add((*prz).buffer_size) { size = (*prz).buffer.data.as_mut_ptr().add((*prz).buffer_size).offset_from(block) as usize; }
        let numerr = persistent_ram_decode_rs8(prz, block as *mut c_void, size, par);
        if numerr > 0 { (*prz).corrected_bytes += numerr as usize; } else if numerr < 0 { (*prz).bad_blocks += 1; }
        block = block.add((*prz).ecc_info.block_size); par = par.add((*prz).ecc_info.ecc_size);
    }
}

unsafe fn persistent_ram_init_ecc(prz: *mut persistent_ram_zone, ecc_info: *mut persistent_ram_ecc_info) -> c_int {
    if ecc_info.is_null() || (*ecc_info).ecc_size == 0 { return 0; }
    (*prz).ecc_info.block_size = if (*ecc_info).block_size != 0 { (*ecc_info).block_size } else { 128 };
    (*prz).ecc_info.ecc_size = if (*ecc_info).ecc_size != 0 { (*ecc_info).ecc_size } else { 16 };
    (*prz).ecc_info.symsize = if (*ecc_info).symsize != 0 { (*ecc_info).symsize } else { 8 };
    (*prz).ecc_info.poly = if (*ecc_info).poly != 0 { (*ecc_info).poly } else { 0x11d };
    let ecc_blocks = ((*prz).buffer_size - (*prz).ecc_info.ecc_size + (*prz).ecc_info.block_size + (*prz).ecc_info.ecc_size - 1) / ((*prz).ecc_info.block_size + (*prz).ecc_info.ecc_size);
    let ecc_total = (ecc_blocks + 1) * (*prz).ecc_info.ecc_size;
    if ecc_total >= (*prz).buffer_size { return -EINVAL; }
    (*prz).buffer_size -= ecc_total;
    (*prz).par_buffer = (*prz).buffer.data.as_mut_ptr().add((*prz).buffer_size);
    (*prz).par_header = (*prz).par_buffer.add(ecc_blocks * (*prz).ecc_info.ecc_size);
    (*prz).rs_decoder = init_rs((*prz).ecc_info.symsize, (*prz).ecc_info.poly, 0, 1, (*prz).ecc_info.ecc_size);
    if (*prz).rs_decoder.is_null() { return -EINVAL; }
    (*prz).ecc_info.par = kmalloc_objs((*prz).ecc_info.ecc_size);
    if (*prz).ecc_info.par.is_null() { return -ENOMEM; }
    (*prz).corrected_bytes = 0; (*prz).bad_blocks = 0;
    let numerr = persistent_ram_decode_rs8(prz, (*prz).buffer as *mut c_void, core::mem::size_of::<persistent_ram_buffer>(), (*prz).par_header);
    if numerr > 0 { (*prz).corrected_bytes += numerr as usize; } else if numerr < 0 { (*prz).bad_blocks += 1; }
    0
}

pub unsafe fn persistent_ram_ecc_string(prz: *mut persistent_ram_zone, str_: *mut c_char, len: usize) -> isize {
    if (*prz).ecc_info.ecc_size == 0 { return 0; }
    if (*prz).corrected_bytes != 0 || (*prz).bad_blocks != 0 { snprintf(str_, len, b"\nECC: %d Corrected bytes, %d unrecoverable blocks\0".as_ptr() as *const c_char, (*prz).corrected_bytes, (*prz).bad_blocks) as isize } else { snprintf(str_, len, b"\nECC: No errors detected\0".as_ptr() as *const c_char) as isize }
}

unsafe fn persistent_ram_update(prz: *mut persistent_ram_zone, s: *const c_void, start: c_uint, count: c_uint) { memcpy_toio((*prz).buffer.data.as_mut_ptr().add(start as usize), s, count as usize); persistent_ram_update_ecc(prz, start, count); }
unsafe fn persistent_ram_update_user(prz: *mut persistent_ram_zone, s: *const c_void, start: c_uint, count: c_uint) -> c_int { let ret = if copy_from_user((*prz).buffer.data.as_mut_ptr().add(start as usize), s, count as usize) != 0 { -EFAULT } else { 0 }; persistent_ram_update_ecc(prz, start, count); ret }

pub unsafe fn persistent_ram_save_old(prz: *mut persistent_ram_zone) {
    let size = buffer_size(prz); let start = buffer_start(prz); if size == 0 { return; }
    if !(*prz).old_log.is_null() && (*prz).old_log_size != size { persistent_ram_free_old(prz); }
    if (*prz).old_log.is_null() { persistent_ram_ecc_old(prz); (*prz).old_log = kvzalloc(size, GFP_KERNEL); }
    if (*prz).old_log.is_null() { return; }
    (*prz).old_log_size = size;
    memcpy_fromio((*prz).old_log, (*prz).buffer.data.as_mut_ptr().add(start), size - start);
    memcpy_fromio((*prz).old_log.add(size - start), (*prz).buffer.data.as_mut_ptr(), start);
}

pub unsafe fn persistent_ram_write(prz: *mut persistent_ram_zone, mut s: *const c_void, count: c_uint) -> c_int {
    let mut c = count as usize; if c > (*prz).buffer_size { s = s.add(c - (*prz).buffer_size); c = (*prz).buffer_size; }
    buffer_size_add(prz, c); let mut start = buffer_start_add(prz, c); let mut rem = (*prz).buffer_size - start;
    if rem < c { persistent_ram_update(prz, s, start as c_uint, rem as c_uint); s = s.add(rem); c -= rem; start = 0; }
    persistent_ram_update(prz, s, start as c_uint, c as c_uint); persistent_ram_update_header_ecc(prz); count as c_int
}

pub unsafe fn persistent_ram_write_user(prz: *mut persistent_ram_zone, mut s: *const c_void, count: c_uint) -> c_int {
    let mut c = count as usize; let mut ret = 0; if c > (*prz).buffer_size { s = s.add(c - (*prz).buffer_size); c = (*prz).buffer_size; }
    buffer_size_add(prz, c); let mut start = buffer_start_add(prz, c); let rem = (*prz).buffer_size - start;
    if rem < c { ret = persistent_ram_update_user(prz, s, start as c_uint, rem as c_uint); s = s.add(rem); c -= rem; start = 0; }
    if ret == 0 { ret = persistent_ram_update_user(prz, s, start as c_uint, c as c_uint); }
    persistent_ram_update_header_ecc(prz); if ret != 0 { ret } else { count as c_int }
}

pub unsafe fn persistent_ram_old_size(prz: *mut persistent_ram_zone) -> usize { (*prz).old_log_size }
pub unsafe fn persistent_ram_old(prz: *mut persistent_ram_zone) -> *mut c_void { (*prz).old_log }
pub unsafe fn persistent_ram_free_old(prz: *mut persistent_ram_zone) { kvfree((*prz).old_log); (*prz).old_log = core::ptr::null_mut(); (*prz).old_log_size = 0; }
pub unsafe fn persistent_ram_zap(prz: *mut persistent_ram_zone) { atomic_set(&mut (*(*prz).buffer).start, 0); atomic_set(&mut (*(*prz).buffer).size, 0); persistent_ram_update_header_ecc(prz); }

pub const MEM_TYPE_WCOMBINE: c_uint = 0;
pub const MEM_TYPE_NONCACHED: c_uint = 1;
pub const MEM_TYPE_NORMAL: c_uint = 2;

unsafe fn persistent_ram_vmap(start: phys_addr_t, size: usize, memtype: c_uint) -> *mut c_void {
    let page_start = start - offset_in_page(start); let page_count = (size + offset_in_page(start) + PAGE_SIZE - 1) / PAGE_SIZE;
    let prot = match memtype { MEM_TYPE_NORMAL => PAGE_KERNEL, MEM_TYPE_NONCACHED => pgprot_noncached(PAGE_KERNEL), MEM_TYPE_WCOMBINE => pgprot_writecombine(PAGE_KERNEL), _ => return core::ptr::null_mut() };
    let pages = kmalloc_objs::<*mut page>(page_count); if pages.is_null() { return core::ptr::null_mut(); }
    for i in 0..page_count { *pages.add(i) = pfn_to_page((page_start + i * PAGE_SIZE) >> PAGE_SHIFT); }
    let vaddr = vmap(pages, page_count, VM_MAP | VM_IOREMAP, prot); kfree(pages as *mut c_void); if vaddr.is_null() { return vaddr; }
    vaddr.add(offset_in_page(start))
}

unsafe fn persistent_ram_iomap(start: phys_addr_t, size: usize, memtype: c_uint, label: *mut c_char) -> *mut c_void {
    if request_mem_region(start, size, if label.is_null() { b"ramoops\0".as_ptr() as *const c_char } else { label }).is_null() { return core::ptr::null_mut(); }
    let va = if memtype != 0 { ioremap(start, size) } else { ioremap_wc(start, size) }; if va.is_null() { release_mem_region(start, size); } va
}

unsafe fn persistent_ram_buffer_map(start: phys_addr_t, size: phys_addr_t, prz: *mut persistent_ram_zone, memtype: c_int) -> c_int {
    (*prz).paddr = start; (*prz).size = size; (*prz).vaddr = if pfn_valid(start >> PAGE_SHIFT) { persistent_ram_vmap(start, size as usize, memtype as c_uint) } else { persistent_ram_iomap(start, size as usize, memtype as c_uint, (*prz).label) };
    if (*prz).vaddr.is_null() { return -ENOMEM; } (*prz).buffer = (*prz).vaddr as *mut persistent_ram_buffer; (*prz).buffer_size = size as usize - core::mem::size_of::<persistent_ram_buffer>(); 0
}

unsafe fn persistent_ram_post_init(prz: *mut persistent_ram_zone, mut sig: u32, ecc_info: *mut persistent_ram_ecc_info) -> c_int {
    if persistent_ram_init_ecc(prz, ecc_info) != 0 { return -EINVAL; } let mut zap = ((*prz).flags & PRZ_FLAG_ZAP_OLD) != 0; sig ^= PERSISTENT_RAM_SIG;
    if (*prz).buffer.sig == sig { if buffer_size(prz) == 0 && buffer_start(prz) == 0 { return 0; } if buffer_size(prz) > (*prz).buffer_size || buffer_start(prz) > buffer_size(prz) { zap = true; } else { persistent_ram_save_old(prz); } } else { (*prz).buffer.sig = sig; zap = true; }
    if zap { persistent_ram_zap(prz); } 0
}

pub unsafe fn persistent_ram_free(prz: *mut *mut persistent_ram_zone) {
    if prz.is_null() || (*prz).is_null() { return; } let p = *prz;
    if !(*p).vaddr.is_null() { if pfn_valid((*p).paddr >> PAGE_SHIFT) { vunmap((*p).vaddr.sub(offset_in_page((*p).paddr))); } else { iounmap((*p).vaddr); release_mem_region((*p).paddr, (*p).size); } (*p).vaddr = core::ptr::null_mut(); }
    if !(*p).rs_decoder.is_null() { free_rs((*p).rs_decoder); (*p).rs_decoder = core::ptr::null_mut(); } kfree((*p).ecc_info.par as *mut c_void); (*p).ecc_info.par = core::ptr::null_mut(); persistent_ram_free_old(p); kfree((*p).label as *mut c_void); kfree(p as *mut c_void); *prz = core::ptr::null_mut();
}

pub unsafe fn persistent_ram_new(start: phys_addr_t, size: usize, sig: u32, ecc_info: *mut persistent_ram_ecc_info, memtype: c_uint, flags: u32, label: *mut c_char) -> *mut persistent_ram_zone {
    let p = kzalloc_obj::<persistent_ram_zone>(); if p.is_null() { return ERR_PTR(-ENOMEM as isize) as *mut persistent_ram_zone; } raw_spin_lock_init(&mut (*p).buffer_lock); (*p).flags = flags; (*p).label = kstrdup(label, GFP_KERNEL); if (*p).label.is_null() || persistent_ram_buffer_map(start, size as phys_addr_t, p, memtype as c_int) != 0 || persistent_ram_post_init(p, sig, ecc_info) != 0 { persistent_ram_free(&mut (p as *mut persistent_ram_zone)); return ERR_PTR(-ENOMEM as isize) as *mut persistent_ram_zone; } p
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
