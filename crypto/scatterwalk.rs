// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Cryptographic API.
 *
 * Cipher operations.
 *
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 *               2002 Adam J. Richter <adam@yggdrasil.com>
 *               2004 Jean-Luc Cooke <jlcooke@certainkey.com>
 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// referenced here as external C symbols.

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scatterlist {
    pub page_link: *mut page,
    pub offset: u32,
    pub length: u32,
}

#[repr(C)]
pub struct scatter_walk {
    pub sg: *mut scatterlist,
    pub offset: u32,
    pub addr: *mut u8,
}

extern "C" {
    fn sg_next(sg: *mut scatterlist) -> *mut scatterlist;
    fn sg_page(sg: *mut scatterlist) -> *mut page;
    fn sg_init_table(sgl: *mut scatterlist, nents: u32);
    fn sg_set_page(sgl: *mut scatterlist, page: *mut page, length: u32, offset: u32);
    fn scatterwalk_crypto_chain(dst: *mut scatterlist, src: *mut scatterlist, nents: u32);
    fn scatterwalk_next(walk: *mut scatter_walk, nbytes: u32) -> u32;
    fn scatterwalk_done_src(walk: *mut scatter_walk, nbytes: u32);
    fn scatterwalk_done_dst(walk: *mut scatter_walk, nbytes: u32);
    fn scatterwalk_start_at_pos(walk: *mut scatter_walk, sg: *mut scatterlist, start: u32);
    fn memcpy_page(dst: *mut page, dst_offset: u32, src: *mut page, src_offset: u32, len: u32);
    fn flush_dcache_page(page: *mut page);
    fn kmap_local_page(page: *mut page) -> *mut u8;
    fn kunmap_local(addr: *mut u8);
    fn page_address(page: *mut page) -> *mut u8;
    fn __scatterwalk_flush_dcache_pages(page: *mut page, offset: u32, len: u32);
}

#[inline]
pub unsafe fn scatterwalk_skip(walk: *mut scatter_walk, mut nbytes: u32) {
    let mut sg = (*walk).sg;

    nbytes += (*walk).offset - (*sg).offset;
    while nbytes > (*sg).length {
        nbytes -= (*sg).length;
        sg = sg_next(sg);
    }
    (*walk).sg = sg;
    (*walk).offset = (*sg).offset + nbytes;
}

#[inline]
pub unsafe fn memcpy_from_scatterwalk(mut buf: *mut u8, walk: *mut scatter_walk, mut nbytes: u32) {
    loop {
        let to_copy = scatterwalk_next(walk, nbytes);
        core::ptr::copy_nonoverlapping((*walk).addr, buf, to_copy as usize);
        scatterwalk_done_src(walk, to_copy);
        buf = buf.add(to_copy as usize);
        nbytes -= to_copy;
        if nbytes == 0 { break; }
    }
}

#[inline]
pub unsafe fn memcpy_to_scatterwalk(walk: *mut scatter_walk, mut buf: *const u8, mut nbytes: u32) {
    loop {
        let to_copy = scatterwalk_next(walk, nbytes);
        core::ptr::copy_nonoverlapping(buf, (*walk).addr, to_copy as usize);
        scatterwalk_done_dst(walk, to_copy);
        buf = buf.add(to_copy as usize);
        nbytes -= to_copy;
        if nbytes == 0 { break; }
    }
}

pub unsafe fn memcpy_from_sglist(buf: *mut u8, sg: *mut scatterlist, start: u32, nbytes: u32) {
    if nbytes == 0 { return; }
    let mut walk = core::mem::zeroed::<scatter_walk>();
    scatterwalk_start_at_pos(&mut walk, sg, start);
    memcpy_from_scatterwalk(buf, &mut walk, nbytes);
}

pub unsafe fn memcpy_to_sglist(sg: *mut scatterlist, start: u32, buf: *const u8, nbytes: u32) {
    if nbytes == 0 { return; }
    let mut walk = core::mem::zeroed::<scatter_walk>();
    scatterwalk_start_at_pos(&mut walk, sg, start);
    memcpy_to_scatterwalk(&mut walk, buf, nbytes);
}

pub unsafe fn memcpy_sglist(mut dst: *mut scatterlist, mut src: *mut scatterlist, mut nbytes: u32) {
    if nbytes == 0 { return; }
    let mut src_offset = (*src).offset;
    let mut dst_offset = (*dst).offset;
    loop {
        let mut len = core::cmp::min(
            core::cmp::min((*src).offset + (*src).length - src_offset,
                           (*dst).offset + (*dst).length - dst_offset), nbytes);
        let mut src_page = sg_page(src);
        let mut dst_page = sg_page(dst);

        // CONFIG_HIGHMEM-dependent mapping branch from the original source.
        if false {
            let src_oip = src_offset % 4096;
            let dst_oip = dst_offset % 4096;
            len = core::cmp::min(len, core::cmp::min(4096 - src_oip, 4096 - dst_oip));
            src_page = src_page.add((src_offset / 4096) as usize);
            dst_page = dst_page.add((dst_offset / 4096) as usize);
            if src_page != dst_page {
                memcpy_page(dst_page, dst_oip, src_page, src_oip, len);
                flush_dcache_page(dst_page);
            } else if src_oip != dst_oip {
                let dst_virt = kmap_local_page(dst_page);
                core::ptr::copy(dst_virt.add(src_oip as usize), dst_virt.add(dst_oip as usize), len as usize);
                kunmap_local(dst_virt);
                flush_dcache_page(dst_page);
            }
        } else {
            let src_virt = page_address(src_page).add(src_offset as usize);
            let dst_virt = page_address(dst_page).add(dst_offset as usize);
            if src_virt != dst_virt {
                core::ptr::copy(src_virt, dst_virt, len as usize);
            }
        }
        nbytes -= len;
        if nbytes == 0 { break; }
        src_offset += len;
        if src_offset >= (*src).offset + (*src).length { src = sg_next(src); src_offset = (*src).offset; }
        dst_offset += len;
        if dst_offset >= (*dst).offset + (*dst).length { dst = sg_next(dst); dst_offset = (*dst).offset; }
    }
}

pub unsafe fn scatterwalk_ffwd(dst: *mut scatterlist, mut src: *mut scatterlist, mut len: u32) -> *mut scatterlist {
    loop {
        if len == 0 { return src; }
        if (*src).length > len { break; }
        len -= (*src).length;
        src = sg_next(src);
    }
    sg_init_table(dst, 2);
    sg_set_page(dst, sg_page(src), (*src).length - len, (*src).offset + len);
    scatterwalk_crypto_chain(dst, sg_next(src), 2);
    dst
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
