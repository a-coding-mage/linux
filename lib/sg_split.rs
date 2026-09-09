// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 Robert Jarzmik <robert.jarzmik@free.fr>
 *
 * Scatterlist splitting helpers.
 */

use core::ffi::{c_int, c_uint, c_void};

// Supplied by the scatterlist and slab dependencies.
#[repr(C)]
pub struct scatterlist {
    pub page_link: usize,
    pub offset: c_uint,
    pub length: c_uint,
    pub dma_address: u64,
    pub dma_length: c_uint,
}
pub type gfp_t = c_uint;
pub type off_t = isize;

extern "C" {
    fn sg_nents(sg: *mut scatterlist) -> c_int;
    fn sg_next(sg: *mut scatterlist) -> *mut scatterlist;
    fn sg_mark_end(sg: *mut scatterlist);
    fn sg_dma_len(sg: *mut scatterlist) -> *mut c_uint;
    fn sg_dma_address(sg: *mut scatterlist) -> *mut u64;
    fn kzalloc_objs(size: usize, flags: gfp_t) -> *mut c_void;
    fn kmalloc_objs(size: usize, count: usize, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

#[repr(C)]
struct sg_splitter {
    in_sg0: *mut scatterlist,
    nents: c_int,
    skip_sg0: off_t,
    length_last_sg: c_uint,
    out_sg: *mut scatterlist,
}

unsafe fn sg_calculate_split(
    mut in_sg: *mut scatterlist,
    nents: c_int,
    mut nb_splits: c_int,
    mut skip: off_t,
    sizes: *const usize,
    splitters: *mut sg_splitter,
    mapped: bool,
) -> c_int {
    let mut size = *sizes;
    let mut curr = splitters;
    let mut sg: *mut scatterlist;

    for i in 0..nb_splits {
        (*splitters.add(i as usize)).in_sg0 = core::ptr::null_mut();
        (*splitters.add(i as usize)).nents = 0;
    }

    sg = in_sg;
    for i in 0..nents {
        let sglen = if mapped { *sg_dma_len(sg) as usize } else { (*sg).length as usize };
        if skip as usize > sglen {
            skip -= sglen as off_t;
            sg = sg_next(sg);
            continue;
        }

        let mut len = core::cmp::min(size, sglen - skip as usize);
        if (*curr).in_sg0.is_null() {
            (*curr).in_sg0 = sg;
            (*curr).skip_sg0 = skip;
        }
        size -= len;
        (*curr).nents += 1;
        (*curr).length_last_sg = len as c_uint;

        while size == 0 && skip as usize + len < sglen && { nb_splits -= 1; nb_splits > 0 } {
            curr = curr.add(1);
            size = *sizes.add((i as usize) + 1);
            skip += len as off_t;
            len = core::cmp::min(size, sglen - skip as usize);
            (*curr).in_sg0 = sg;
            (*curr).skip_sg0 = skip;
            (*curr).nents = 1;
            (*curr).length_last_sg = len as c_uint;
            size -= len;
        }
        skip = 0;

        if size == 0 && { nb_splits -= 1; nb_splits > 0 } {
            curr = curr.add(1);
            size = *sizes.add((i as usize) + 1);
        }
        if nb_splits == 0 { break; }
        sg = sg_next(sg);
    }

    if size != 0 || (*splitters).in_sg0.is_null() { -22 } else { 0 }
}

unsafe fn sg_split_phys(splitters: *mut sg_splitter, nb_splits: c_int) {
    for i in 0..nb_splits {
        let split = splitters.add(i as usize);
        let mut in_sg = (*split).in_sg0;
        let mut out_sg = (*split).out_sg;
        for j in 0..(*split).nents {
            *out_sg = *in_sg;
            if j == 0 {
                (*out_sg).offset = (*out_sg).offset.wrapping_add((*split).skip_sg0 as c_uint);
                (*out_sg).length = (*out_sg).length.wrapping_sub((*split).skip_sg0 as c_uint);
            }
            *sg_dma_address(out_sg) = 0;
            *sg_dma_len(out_sg) = 0;
            in_sg = sg_next(in_sg);
            out_sg = out_sg.add(1);
        }
        (*out_sg.sub(1)).length = (*split).length_last_sg;
        sg_mark_end(out_sg.sub(1));
    }
}

unsafe fn sg_split_mapped(splitters: *mut sg_splitter, nb_splits: c_int) {
    for i in 0..nb_splits {
        let split = splitters.add(i as usize);
        let mut in_sg = (*split).in_sg0;
        let mut out_sg = (*split).out_sg;
        for j in 0..(*split).nents {
            *sg_dma_address(out_sg) = *sg_dma_address(in_sg);
            (*out_sg).dma_length = (*in_sg).dma_length;
            if j == 0 {
                *sg_dma_address(out_sg) += (*split).skip_sg0 as u64;
                (*out_sg).dma_length = (*out_sg).dma_length.wrapping_sub((*split).skip_sg0 as c_uint);
            }
            in_sg = sg_next(in_sg);
            out_sg = out_sg.add(1);
        }
        (*out_sg.sub(1)).dma_length = (*split).length_last_sg;
    }
}

pub unsafe fn sg_split(
    in_sg: *mut scatterlist, in_mapped_nents: c_int, skip: off_t,
    nb_splits: c_int, split_sizes: *const usize, out: *mut *mut scatterlist,
    out_mapped_nents: *mut c_int, gfp_mask: gfp_t,
) -> c_int {
    let splitters = kzalloc_objs(core::mem::size_of::<sg_splitter>() * nb_splits as usize, gfp_mask) as *mut sg_splitter;
    if splitters.is_null() { return -12; }
    let mut ret = sg_calculate_split(in_sg, sg_nents(in_sg), nb_splits, skip, split_sizes, splitters, false);
    if ret < 0 { kfree(splitters as *mut c_void); return ret; }
    ret = -12;
    for i in 0..nb_splits {
        (*splitters.add(i as usize)).out_sg = kmalloc_objs(
            core::mem::size_of::<scatterlist>(), (*splitters.add(i as usize)).nents as usize, gfp_mask) as *mut scatterlist;
        if (*splitters.add(i as usize)).out_sg.is_null() { break; }
    }
    if ret < 0 { for i in 0..nb_splits { kfree((*splitters.add(i as usize)).out_sg as *mut c_void); } kfree(splitters as *mut c_void); return ret; }
    sg_split_phys(splitters, nb_splits);
    if in_mapped_nents != 0 {
        ret = sg_calculate_split(in_sg, in_mapped_nents, nb_splits, skip, split_sizes, splitters, true);
        if ret < 0 { kfree(splitters as *mut c_void); return ret; }
        sg_split_mapped(splitters, nb_splits);
    }
    for i in 0..nb_splits {
        *out.add(i as usize) = (*splitters.add(i as usize)).out_sg;
        if !out_mapped_nents.is_null() { *out_mapped_nents.add(i as usize) = (*splitters.add(i as usize)).nents; }
    }
    kfree(splitters as *mut c_void);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
