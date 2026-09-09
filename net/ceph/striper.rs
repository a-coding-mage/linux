/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies are supplied by the surrounding Ceph/Linux bindings. */

pub unsafe fn ceph_calc_file_object_mapping(
    l: *mut ceph_file_layout,
    off: u64,
    len: u64,
    objno: *mut u64,
    objoff: *mut u64,
    xlen: *mut u32,
) {
    let stripes_per_object = (*l).object_size / (*l).stripe_unit;
    let mut blockoff: u32 = 0;
    let mut stripepos: u32 = 0;
    let mut objsetpos: u32 = 0;

    let blockno = div_u64_rem(off, (*l).stripe_unit, &mut blockoff);
    let stripeno = div_u64_rem(blockno, (*l).stripe_count, &mut stripepos);
    let objsetno = div_u64_rem(stripeno, stripes_per_object, &mut objsetpos);

    *objno = objsetno * (*l).stripe_count as u64 + stripepos as u64;
    *objoff = objsetpos as u64 * (*l).stripe_unit + blockoff as u64;
    *xlen = core::cmp::min(len, (*l).stripe_unit - blockoff as u64) as u32;
}

unsafe fn lookup_last(
    object_extents: *mut list_head,
    objno: u64,
    add_pos: *mut *mut list_head,
) -> *mut ceph_object_extent {
    let mut pos: *mut list_head = (*object_extents).prev;
    while pos != object_extents {
        let ex = list_entry_object_extent(pos);
        if (*ex).oe_objno == objno {
            return ex;
        }
        if (*ex).oe_objno < objno {
            break;
        }
        pos = (*pos).prev;
    }
    *add_pos = pos;
    core::ptr::null_mut()
}

unsafe fn lookup_containing(
    object_extents: *mut list_head,
    objno: u64,
    objoff: u64,
    xlen: u32,
) -> *mut ceph_object_extent {
    let mut pos = (*object_extents).next;
    while pos != object_extents {
        let ex = list_entry_object_extent(pos);
        if (*ex).oe_objno == objno
            && (*ex).oe_off <= objoff
            && (*ex).oe_off + (*ex).oe_len >= objoff + xlen as u64
        {
            return ex;
        }
        if (*ex).oe_objno > objno {
            break;
        }
        pos = (*pos).next;
    }
    core::ptr::null_mut()
}

pub unsafe fn ceph_file_to_extents(
    l: *mut ceph_file_layout,
    mut off: u64,
    mut len: u64,
    object_extents: *mut list_head,
    alloc_fn: unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut ceph_object_extent,
    alloc_arg: *mut core::ffi::c_void,
    action_fn: Option<unsafe extern "C" fn(*mut ceph_object_extent, u32, *mut core::ffi::c_void)>,
    action_arg: *mut core::ffi::c_void,
) -> i32 {
    while len != 0 {
        let mut add_pos: *mut list_head = core::ptr::null_mut();
        let mut objno = 0u64;
        let mut objoff = 0u64;
        let mut xlen = 0u32;
        ceph_calc_file_object_mapping(l, off, len, &mut objno, &mut objoff, &mut xlen);
        let last_ex = lookup_last(object_extents, objno, &mut add_pos);
        if last_ex.is_null() || (*last_ex).oe_off + (*last_ex).oe_len != objoff {
            let ex = alloc_fn(alloc_arg);
            if ex.is_null() { return -12; }
            (*ex).oe_objno = objno;
            (*ex).oe_off = objoff;
            (*ex).oe_len = xlen as u64;
            if let Some(f) = action_fn { f(ex, xlen, action_arg); }
            if last_ex.is_null() { list_add(&mut (*ex).oe_item, add_pos); }
            else { list_add(&mut (*ex).oe_item, &mut (*last_ex).oe_item); }
        } else {
            (*last_ex).oe_len += xlen as u64;
            if let Some(f) = action_fn { f(last_ex, xlen, action_arg); }
        }
        off += xlen as u64;
        len -= xlen as u64;
    }
    let mut last = list_first_entry_object_extent(object_extents);
    let mut ex = list_next_entry_object_extent(last);
    while &mut (*ex).oe_item as *mut list_head != object_extents {
        if (*last).oe_objno > (*ex).oe_objno
            || ((*last).oe_objno == (*ex).oe_objno
                && (*last).oe_off + (*last).oe_len >= (*ex).oe_off)
        { warn_unsorted(); return -22; }
        last = ex;
        ex = list_next_entry_object_extent(ex);
    }
    0
}

pub unsafe fn ceph_iterate_extents(
    l: *mut ceph_file_layout, mut off: u64, mut len: u64,
    object_extents: *mut list_head,
    action_fn: unsafe extern "C" fn(*mut ceph_object_extent, u32, *mut core::ffi::c_void),
    action_arg: *mut core::ffi::c_void,
) -> i32 {
    while len != 0 {
        let (mut objno, mut objoff, mut xlen) = (0u64, 0u64, 0u32);
        ceph_calc_file_object_mapping(l, off, len, &mut objno, &mut objoff, &mut xlen);
        let ex = lookup_containing(object_extents, objno, objoff, xlen);
        if ex.is_null() { warn_not_found(objno, objoff, xlen); return -22; }
        action_fn(ex, xlen, action_arg);
        off += xlen as u64; len -= xlen as u64;
    }
    0
}

pub unsafe fn ceph_extent_to_file(
    l: *mut ceph_file_layout, objno: u64, mut objoff: u64, mut objlen: u64,
    file_extents: *mut *mut ceph_file_extent, num_file_extents: *mut u32,
) -> i32 {
    let stripes_per_object = (*l).object_size / (*l).stripe_unit;
    if objlen == 0 { *file_extents = core::ptr::null_mut(); *num_file_extents = 0; return 0; }
    *num_file_extents = div_round_up(objoff + objlen, (*l).stripe_unit) as u32
        - div_round_down(objoff, (*l).stripe_unit) as u32;
    *file_extents = kmalloc_file_extents(*num_file_extents);
    if (*file_extents).is_null() { return -12; }
    let mut blockoff = 0u32;
    div_u64_rem(objoff, (*l).stripe_unit, &mut blockoff);
    let mut i = 0u32;
    while objlen != 0 {
        let mut stripepos = 0u32;
        let objsetno = div_u64_rem(objno, (*l).stripe_count, &mut stripepos);
        let stripeno = div_u64(objoff, (*l).stripe_unit) + objsetno * stripes_per_object;
        let blockno = stripeno * (*l).stripe_count as u64 + stripepos as u64;
        let off = blockno * (*l).stripe_unit + blockoff as u64;
        let len = core::cmp::min(objlen, (*l).stripe_unit - blockoff as u64);
        (*(*file_extents).add(i as usize)).fe_off = off;
        (*(*file_extents).add(i as usize)).fe_len = len;
        blockoff = 0; objoff += len; objlen -= len; i += 1;
    }
    bug_on(i != *num_file_extents); 0
}

pub unsafe fn ceph_get_num_objects(l: *mut ceph_file_layout, size: u64) -> u64 {
    let period = (*l).stripe_count as u64 * (*l).object_size;
    let num_periods = div64_u64_round_up(size, period);
    let mut remainder_bytes = 0u64;
    div64_u64_rem(size, period, &mut remainder_bytes);
    let remainder_objs = if remainder_bytes > 0 && remainder_bytes < (*l).stripe_count as u64 * (*l).stripe_unit {
        (*l).stripe_count as u64 - div_round_up(remainder_bytes, (*l).stripe_unit)
    } else { 0 };
    num_periods * (*l).stripe_count as u64 - remainder_objs
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
