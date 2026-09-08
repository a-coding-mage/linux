// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
/*
 * libfdt - Flat Device Tree manipulation
 * Copyright (C) 2006 David Gibson, IBM Corporation.
 */

/* Dependencies are supplied by the surrounding libfdt translation unit. */

/*
 * Minimal sanity check for a read-only tree. fdt_ro_probe_() checks
 * that the given buffer contains what appears to be a flattened
 * device tree with sane information in its header.
 */
pub unsafe fn fdt_ro_probe_(fdt: *const core::ffi::c_void) -> i32 {
    let totalsize = fdt_totalsize(fdt);

    if can_assume(VALID_DTB) { return totalsize as i32; }
    if (fdt as usize) & 7 != 0 { return -FDT_ERR_ALIGNMENT; }
    if fdt_magic(fdt) == FDT_MAGIC {
        if !can_assume(LATEST) {
            if fdt_version(fdt) < FDT_FIRST_SUPPORTED_VERSION { return -FDT_ERR_BADVERSION; }
            if fdt_last_comp_version(fdt) > FDT_LAST_SUPPORTED_VERSION { return -FDT_ERR_BADVERSION; }
        }
    } else if fdt_magic(fdt) == FDT_SW_MAGIC {
        if !can_assume(VALID_INPUT) && fdt_size_dt_struct(fdt) == 0 { return -FDT_ERR_BADSTATE; }
    } else { return -FDT_ERR_BADMAGIC; }
    if totalsize < INT32_MAX as u32 { totalsize as i32 } else { -FDT_ERR_TRUNCATED }
}

unsafe fn check_off_(hdrsize: u32, totalsize: u32, off: u32) -> bool { off >= hdrsize && off <= totalsize }

unsafe fn check_block_(hdrsize: u32, totalsize: u32, base: u32, size: u32) -> bool {
    if !check_off_(hdrsize, totalsize, base) { return false; }
    let end = base.wrapping_add(size);
    if end < base { return false; }
    if !check_off_(hdrsize, totalsize, end) { return false; }
    true
}

pub fn fdt_header_size_(version: u32) -> usize {
    if version <= 1 { FDT_V1_SIZE } else if version <= 2 { FDT_V2_SIZE } else if version <= 3 { FDT_V3_SIZE } else if version <= 16 { FDT_V16_SIZE } else { FDT_V17_SIZE }
}

pub unsafe fn fdt_header_size(fdt: *const core::ffi::c_void) -> usize {
    if can_assume(LATEST) { FDT_V17_SIZE } else { fdt_header_size_(fdt_version(fdt)) }
}

pub unsafe fn fdt_check_header(fdt: *const core::ffi::c_void) -> i32 {
    if (fdt as usize) & 7 != 0 { return -FDT_ERR_ALIGNMENT; }
    if fdt_magic(fdt) != FDT_MAGIC { return -FDT_ERR_BADMAGIC; }
    if !can_assume(LATEST) {
        if fdt_version(fdt) < FDT_FIRST_SUPPORTED_VERSION || fdt_last_comp_version(fdt) > FDT_LAST_SUPPORTED_VERSION || fdt_version(fdt) < fdt_last_comp_version(fdt) { return -FDT_ERR_BADVERSION; }
    }
    let hdrsize = fdt_header_size(fdt) as u32;
    if !can_assume(VALID_DTB) {
        let total = fdt_totalsize(fdt);
        if total < hdrsize || total > INT_MAX as u32 { return -FDT_ERR_TRUNCATED; }
        if fdt_off_mem_rsvmap(fdt) % core::mem::size_of::<u64>() as u32 != 0 { return -FDT_ERR_ALIGNMENT; }
        if fdt_off_dt_struct(fdt) % FDT_TAGSIZE != 0 { return -FDT_ERR_ALIGNMENT; }
        if !check_off_(hdrsize, total, fdt_off_mem_rsvmap(fdt)) { return -FDT_ERR_TRUNCATED; }
        if !can_assume(LATEST) && fdt_version(fdt) < 17 {
            if !check_off_(hdrsize, total, fdt_off_dt_struct(fdt)) { return -FDT_ERR_TRUNCATED; }
        } else if !check_block_(hdrsize, total, fdt_off_dt_struct(fdt), fdt_size_dt_struct(fdt)) { return -FDT_ERR_TRUNCATED; }
        if !check_block_(hdrsize, total, fdt_off_dt_strings(fdt), fdt_size_dt_strings(fdt)) { return -FDT_ERR_TRUNCATED; }
    }
    0
}

pub unsafe fn fdt_offset_ptr(fdt: *const core::ffi::c_void, offset: i32, len: u32) -> *const core::ffi::c_void {
    let uoffset = offset as u32;
    let absoffset = uoffset.wrapping_add(fdt_off_dt_struct(fdt));
    if offset < 0 { return core::ptr::null(); }
    if !can_assume(VALID_INPUT) && (absoffset < uoffset || absoffset.wrapping_add(len) < absoffset || absoffset.wrapping_add(len) > fdt_totalsize(fdt)) { return core::ptr::null(); }
    if can_assume(LATEST) || fdt_version(fdt) >= 0x11 { if uoffset.wrapping_add(len) < uoffset || (offset as u32).wrapping_add(len) > fdt_size_dt_struct(fdt) { return core::ptr::null(); } }
    fdt_offset_ptr_(fdt, offset)
}

pub unsafe fn fdt_next_tag(fdt: *const core::ffi::c_void, startoffset: i32, nextoffset: *mut i32) -> u32 {
    *nextoffset = -FDT_ERR_TRUNCATED;
    let mut offset = startoffset;
    let tagp = fdt_offset_ptr(fdt, offset, FDT_TAGSIZE);
    if !can_assume(VALID_DTB) && tagp.is_null() { return FDT_END; }
    let tag = fdt32_to_cpu(*(tagp as *const fdt32_t));
    offset += FDT_TAGSIZE as i32;
    *nextoffset = -FDT_ERR_BADSTRUCTURE;
    match tag {
        FDT_BEGIN_NODE => { loop { let p = fdt_offset_ptr(fdt, offset, 1) as *const i8; offset += 1; if p.is_null() || *p == 0 { if !can_assume(VALID_DTB) && p.is_null() { return FDT_END; } break; } } }
        FDT_PROP => { let lenp = fdt_offset_ptr(fdt, offset, core::mem::size_of::<fdt32_t>() as u32); if !can_assume(VALID_DTB) && lenp.is_null() { return FDT_END; } let len = fdt32_to_cpu(*(lenp as *const fdt32_t)); let sum = len.wrapping_add(offset as u32); if !can_assume(VALID_DTB) && (INT_MAX as u32 <= sum || sum < offset as u32) { return FDT_END; } offset += (core::mem::size_of::<fdt_property>() - FDT_TAGSIZE as usize) as i32 + len as i32; if !can_assume(LATEST) && fdt_version(fdt) < 0x10 && len >= 8 && ((offset - len as i32) % 8) != 0 { offset += 4; } }
        FDT_END | FDT_END_NODE | FDT_NOP => {}
        _ => return FDT_END,
    }
    if fdt_offset_ptr(fdt, startoffset, (offset - startoffset) as u32).is_null() { return FDT_END; }
    *nextoffset = FDT_TAGALIGN(offset as u32) as i32;
    tag
}

pub unsafe fn fdt_check_node_offset_(fdt: *const core::ffi::c_void, offset: i32) -> i32 { if !can_assume(VALID_INPUT) && (offset < 0 || offset % FDT_TAGSIZE as i32 != 0) { return -FDT_ERR_BADOFFSET; } let mut n = 0; if fdt_next_tag(fdt, offset, &mut n) != FDT_BEGIN_NODE { return -FDT_ERR_BADOFFSET; } n }
pub unsafe fn fdt_check_prop_offset_(fdt: *const core::ffi::c_void, offset: i32) -> i32 { if !can_assume(VALID_INPUT) && (offset < 0 || offset % FDT_TAGSIZE as i32 != 0) { return -FDT_ERR_BADOFFSET; } let mut n = 0; if fdt_next_tag(fdt, offset, &mut n) != FDT_PROP { return -FDT_ERR_BADOFFSET; } n }

pub unsafe fn fdt_next_node(fdt: *const core::ffi::c_void, mut offset: i32, depth: *mut i32) -> i32 { let mut nextoffset = 0; let mut tag; if offset >= 0 { nextoffset = fdt_check_node_offset_(fdt, offset); if nextoffset < 0 { return nextoffset; } } loop { offset = nextoffset; tag = fdt_next_tag(fdt, offset, &mut nextoffset); match tag { FDT_PROP | FDT_NOP => {}, FDT_BEGIN_NODE => if !depth.is_null() { *depth += 1; }, FDT_END_NODE => if !depth.is_null() { *depth -= 1; if *depth < 0 { return nextoffset; } }, FDT_END => { if nextoffset >= 0 || (nextoffset == -FDT_ERR_TRUNCATED && depth.is_null()) { return -FDT_ERR_NOTFOUND; } return nextoffset; }, _ => {} } if tag == FDT_BEGIN_NODE { return offset; } } }
pub unsafe fn fdt_first_subnode(fdt: *const core::ffi::c_void, mut offset: i32) -> i32 { let mut depth = 0; offset = fdt_next_node(fdt, offset, &mut depth); if offset < 0 || depth != 1 { -FDT_ERR_NOTFOUND } else { offset } }
pub unsafe fn fdt_next_subnode(fdt: *const core::ffi::c_void, mut offset: i32) -> i32 { let mut depth = 1; loop { offset = fdt_next_node(fdt, offset, &mut depth); if offset < 0 || depth < 1 { return -FDT_ERR_NOTFOUND; } if depth <= 1 { return offset; } } }

pub unsafe fn fdt_find_string_len_(strtab: *const i8, tabsize: i32, s: *const i8, slen: i32) -> *const i8 { let last = strtab.add((tabsize - slen - 1) as usize); let mut p = strtab; while p <= last { if memcmp(p as *const _, s as *const _, slen as usize) == 0 && *p.add(slen as usize) == 0 { return p; } p = p.add(1); } core::ptr::null() }
pub unsafe fn fdt_move(fdt: *const core::ffi::c_void, buf: *mut core::ffi::c_void, bufsize: i32) -> i32 { if !can_assume(VALID_INPUT) && bufsize < 0 { return -FDT_ERR_NOSPACE; } FDT_RO_PROBE(fdt); if fdt_totalsize(fdt) > bufsize as u32 { return -FDT_ERR_NOSPACE; } memmove(buf, fdt, fdt_totalsize(fdt) as usize); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
