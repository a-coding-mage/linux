// SPDX-License-Identifier: GPL-2.0
// Dependencies are supplied by the kernel or compatibility headers in the C
// implementation.

pub unsafe fn crush_bucket_alg_name(alg: i32) -> *const core::ffi::c_char {
    match alg {
        CRUSH_BUCKET_UNIFORM => b"uniform\0".as_ptr() as *const core::ffi::c_char,
        CRUSH_BUCKET_LIST => b"list\0".as_ptr() as *const core::ffi::c_char,
        CRUSH_BUCKET_TREE => b"tree\0".as_ptr() as *const core::ffi::c_char,
        CRUSH_BUCKET_STRAW => b"straw\0".as_ptr() as *const core::ffi::c_char,
        CRUSH_BUCKET_STRAW2 => b"straw2\0".as_ptr() as *const core::ffi::c_char,
        _ => b"unknown\0".as_ptr() as *const core::ffi::c_char,
    }
}

/**
 * crush_get_bucket_item_weight - Get weight of an item in given bucket
 * @b: bucket pointer
 * @p: item index in bucket
 */
pub unsafe fn crush_get_bucket_item_weight(b: *const crush_bucket, p: i32) -> i32 {
    if (p as u32) >= (*b).size {
        return 0;
    }

    match (*b).alg {
        CRUSH_BUCKET_UNIFORM => (*(b as *const crush_bucket_uniform)).item_weight,
        CRUSH_BUCKET_LIST => (*(b as *const crush_bucket_list)).item_weights.add(p as usize).read(),
        CRUSH_BUCKET_TREE => (*(b as *const crush_bucket_tree)).node_weights
            .add(crush_calc_tree_node(p) as usize)
            .read(),
        CRUSH_BUCKET_STRAW => (*(b as *const crush_bucket_straw)).item_weights.add(p as usize).read(),
        CRUSH_BUCKET_STRAW2 => (*(b as *const crush_bucket_straw2)).item_weights.add(p as usize).read(),
        _ => 0,
    }
}

pub unsafe fn crush_destroy_bucket_uniform(b: *mut crush_bucket_uniform) {
    kfree((*b).h.items);
}

pub unsafe fn crush_destroy_bucket_list(b: *mut crush_bucket_list) {
    kfree((*b).item_weights);
    kfree((*b).sum_weights);
    kfree((*b).h.items);
}

pub unsafe fn crush_destroy_bucket_tree(b: *mut crush_bucket_tree) {
    kfree((*b).h.items);
    kfree((*b).node_weights);
}

pub unsafe fn crush_destroy_bucket_straw(b: *mut crush_bucket_straw) {
    kfree((*b).straws);
    kfree((*b).item_weights);
    kfree((*b).h.items);
}

pub unsafe fn crush_destroy_bucket_straw2(b: *mut crush_bucket_straw2) {
    kfree((*b).item_weights);
    kfree((*b).h.items);
}

pub unsafe fn crush_destroy_bucket(b: *mut crush_bucket) {
    match (*b).alg {
        CRUSH_BUCKET_UNIFORM => crush_destroy_bucket_uniform(b as *mut crush_bucket_uniform),
        CRUSH_BUCKET_LIST => crush_destroy_bucket_list(b as *mut crush_bucket_list),
        CRUSH_BUCKET_TREE => crush_destroy_bucket_tree(b as *mut crush_bucket_tree),
        CRUSH_BUCKET_STRAW => crush_destroy_bucket_straw(b as *mut crush_bucket_straw),
        CRUSH_BUCKET_STRAW2 => crush_destroy_bucket_straw2(b as *mut crush_bucket_straw2),
        _ => {}
    }
    kfree(b);
}

/**
 * crush_destroy - Destroy a crush_map
 * @map: crush_map pointer
 */
pub unsafe fn crush_destroy(map: *mut crush_map) {
    /* buckets */
    if !(*map).buckets.is_null() {
        let mut b: i32 = 0;
        while b < (*map).max_buckets {
            let bucket = (*map).buckets.add(b as usize).read();
            if !bucket.is_null() {
                crush_destroy_bucket(bucket);
            }
            b += 1;
        }
        kfree((*map).buckets);
    }

    /* rules */
    if !(*map).rules.is_null() {
        let mut b: u32 = 0;
        while b < (*map).max_rules {
            crush_destroy_rule((*map).rules.add(b as usize).read());
            b += 1;
        }
        kfree((*map).rules);
    }

    #[cfg(not(feature = "kernel"))]
    kfree((*map).choose_tries);
    #[cfg(feature = "kernel")]
    {
        clear_crush_names(&mut (*map).type_names);
        clear_crush_names(&mut (*map).names);
        clear_choose_args(map);
    }
    kfree(map);
}

pub unsafe fn crush_destroy_rule(rule: *mut crush_rule) {
    kfree(rule);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
