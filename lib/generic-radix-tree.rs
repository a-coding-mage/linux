// SPDX-License-Identifier: GPL-2.0
/* Translated from lib/generic-radix-tree.c. */

/* Depends on: linux/atomic.h, linux/export.h, linux/generic-radix-tree.h,
 * linux/gfp.h, linux/kmemleak.h */

/*
 * Returns pointer to the specified byte @offset within @radix, or NULL if not
 * allocated
 */
#[no_mangle]
pub unsafe extern "C" fn __genradix_ptr(
    radix: *mut __genradix,
    offset: usize,
) -> *mut kernel::ffi::c_void {
    unsafe { __genradix_ptr_inlined(radix, offset) }
}
// EXPORT_SYMBOL(__genradix_ptr);

/*
 * Returns pointer to the specified byte @offset within @radix, allocating it if
 * necessary - newly allocated slots are always zeroed out:
 */
#[no_mangle]
pub unsafe extern "C" fn __genradix_ptr_alloc(
    radix: *mut __genradix,
    mut offset: usize,
    preallocated: *mut *mut genradix_node,
    gfp_mask: gfp_t,
) -> *mut kernel::ffi::c_void {
    let mut v: *mut genradix_root = READ_ONCE!((*radix).root);
    let mut n: *mut genradix_node;
    let mut new_node: *mut genradix_node = core::ptr::null_mut();
    let mut level: kernel::ffi::c_uint;

    if !preallocated.is_null() {
        swap!(new_node, *preallocated);
    }

    /* Increase tree depth if necessary: */
    loop {
        let r = v;

        n = genradix_root_to_node(r);
        level = genradix_root_to_depth(r);

        if !n.is_null() && ilog2!(offset) < genradix_depth_shift(level) {
            break;
        }

        if new_node.is_null() {
            new_node = unsafe { genradix_alloc_node(gfp_mask) };
            if new_node.is_null() {
                return core::ptr::null_mut();
            }
        }

        unsafe { (*new_node).children[0] = n };
        let new_root = (new_node as kernel::ffi::c_ulong
            | if n.is_null() { 0 } else { (level + 1) as kernel::ffi::c_ulong })
            as *mut genradix_root;

        v = unsafe { cmpxchg_release!(&raw mut (*radix).root, r, new_root) };
        if v == r {
            v = new_root;
            new_node = core::ptr::null_mut();
        } else {
            unsafe { (*new_node).children[0] = core::ptr::null_mut() };
        }
    }

    while level != 0 {
        level -= 1;

        let p: *mut *mut genradix_node =
            &raw mut (*n).children[offset >> genradix_depth_shift(level)];
        offset &= genradix_depth_size(level) - 1;

        n = READ_ONCE!(*p);
        if n.is_null() {
            if new_node.is_null() {
                new_node = unsafe { genradix_alloc_node(gfp_mask) };
                if new_node.is_null() {
                    return core::ptr::null_mut();
                }
            }

            n = unsafe { cmpxchg_release!(p, core::ptr::null_mut(), new_node) };
            if n.is_null() {
                swap!(n, new_node);
            }
        }
    }

    if !new_node.is_null() {
        unsafe { genradix_free_node(new_node) };
    }

    unsafe { (&raw mut (*n).data).cast::<u8>().add(offset).cast() }
}
// EXPORT_SYMBOL(__genradix_ptr_alloc);

#[no_mangle]
pub unsafe extern "C" fn __genradix_iter_peek(
    iter: *mut genradix_iter,
    radix: *mut __genradix,
    objs_per_page: usize,
) -> *mut kernel::ffi::c_void {
    let iter = unsafe { &mut *iter };
    let mut n: *mut genradix_node;

    if iter.offset == SIZE_MAX {
        return core::ptr::null_mut();
    }

    'restart: loop {
        let r: *mut genradix_root = READ_ONCE!((*radix).root);
        if r.is_null() {
            return core::ptr::null_mut();
        }

        n = genradix_root_to_node(r);
        let mut level = genradix_root_to_depth(r);

        if ilog2!(iter.offset) >= genradix_depth_shift(level) {
            return core::ptr::null_mut();
        }

        while level != 0 {
            level -= 1;

            let mut i = (iter.offset >> genradix_depth_shift(level)) & (GENRADIX_ARY - 1);

            while unsafe { (*n).children[i] }.is_null() {
                let objs_per_ptr = genradix_depth_size(level);

                if iter.offset.checked_add(objs_per_ptr).is_none() {
                    iter.offset = SIZE_MAX;
                    iter.pos = SIZE_MAX;
                    return core::ptr::null_mut();
                }

                i += 1;
                iter.offset = round_down!(iter.offset + objs_per_ptr, objs_per_ptr);
                iter.pos = (iter.offset >> GENRADIX_NODE_SHIFT) * objs_per_page;
                if i == GENRADIX_ARY {
                    continue 'restart;
                }
            }

            n = unsafe { (*n).children[i] };
        }

        break;
    }

    unsafe {
        (&raw mut (*n).data)
            .cast::<u8>()
            .add(iter.offset & (GENRADIX_NODE_SIZE - 1))
            .cast()
    }
}
// EXPORT_SYMBOL(__genradix_iter_peek);

#[no_mangle]
pub unsafe extern "C" fn __genradix_iter_peek_prev(
    iter: *mut genradix_iter,
    radix: *mut __genradix,
    objs_per_page: usize,
    obj_size_plus_page_remainder: usize,
) -> *mut kernel::ffi::c_void {
    let iter = unsafe { &mut *iter };
    let mut n: *mut genradix_node;

    if iter.offset == SIZE_MAX {
        return core::ptr::null_mut();
    }

    'restart: loop {
        let r: *mut genradix_root = READ_ONCE!((*radix).root);
        if r.is_null() {
            return core::ptr::null_mut();
        }

        n = genradix_root_to_node(r);
        let mut level = genradix_root_to_depth(r);

        if ilog2!(iter.offset) >= genradix_depth_shift(level) {
            iter.offset = genradix_depth_size(level);
            iter.pos = (iter.offset >> GENRADIX_NODE_SHIFT) * objs_per_page;

            iter.offset -= obj_size_plus_page_remainder;
            iter.pos = iter.pos.wrapping_sub(1);
        }

        while level != 0 {
            level -= 1;

            let mut i = (iter.offset >> genradix_depth_shift(level)) & (GENRADIX_ARY - 1);

            while unsafe { (*n).children[i] }.is_null() {
                let objs_per_ptr = genradix_depth_size(level);

                iter.offset = round_down!(iter.offset, objs_per_ptr);
                iter.pos = (iter.offset >> GENRADIX_NODE_SHIFT) * objs_per_page;

                if iter.offset == 0 {
                    return core::ptr::null_mut();
                }

                iter.offset -= obj_size_plus_page_remainder;
                iter.pos = iter.pos.wrapping_sub(1);

                if i == 0 {
                    continue 'restart;
                }
                i -= 1;
            }

            n = unsafe { (*n).children[i] };
        }

        break;
    }

    unsafe {
        (&raw mut (*n).data)
            .cast::<u8>()
            .add(iter.offset & (GENRADIX_NODE_SIZE - 1))
            .cast()
    }
}
// EXPORT_SYMBOL(__genradix_iter_peek_prev);

unsafe fn genradix_free_recurse(n: *mut genradix_node, level: kernel::ffi::c_uint) {
    if level != 0 {
        for i in 0..GENRADIX_ARY {
            let child = unsafe { (*n).children[i] };
            if !child.is_null() {
                unsafe { genradix_free_recurse(child, level - 1) };
            }
        }
    }

    unsafe { genradix_free_node(n) };
}

#[no_mangle]
pub unsafe extern "C" fn __genradix_prealloc(
    radix: *mut __genradix,
    size: usize,
    gfp_mask: gfp_t,
) -> kernel::ffi::c_int {
    for offset in (0..size).step_by(GENRADIX_NODE_SIZE) {
        if unsafe { __genradix_ptr_alloc(radix, offset, core::ptr::null_mut(), gfp_mask) }
            .is_null()
        {
            return -ENOMEM;
        }
    }

    0
}
// EXPORT_SYMBOL(__genradix_prealloc);

#[no_mangle]
pub unsafe extern "C" fn __genradix_free(radix: *mut __genradix) {
    let r: *mut genradix_root = unsafe { xchg!(&raw mut (*radix).root, core::ptr::null_mut()) };

    unsafe { genradix_free_recurse(genradix_root_to_node(r), genradix_root_to_depth(r)) };
}
// EXPORT_SYMBOL(__genradix_free);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
