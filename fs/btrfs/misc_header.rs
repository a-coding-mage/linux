/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation are intentionally external. */

/* Convenience macros corresponding to the C cleanup-attribute declarations. */
macro_rules! AUTO_KFREE {
    ($name:ident) => { $name: *mut _ };
}

macro_rules! AUTO_KVFREE {
    ($name:ident) => { $name: *mut _ };
}

/* Enumerate bits using enum autoincrement. */
/* The C ENUM_BIT macro is represented as a declarative macro for callers that provide
 * the surrounding enum context. */
macro_rules! ENUM_BIT {
    ($name:ident) => {
        __$name##_BIT,
        $name = (1u32 << __$name##_BIT),
        __$name##_SEQ = __$name##_BIT
    };
}

#[inline]
unsafe fn bio_iter_phys(bio: *const bio, iter: *const bvec_iter) -> phys_addr_t {
    let bv: bio_vec = bio_iter_iovec(bio, *iter);
    bvec_phys(&bv)
}

/* Iterate bio using btrfs block size; this also handles large folio and highmem. */
macro_rules! btrfs_bio_for_each_block {
    ($paddr:ident, $bio:expr, $iter:expr, $blocksize:expr, $body:block) => {
        while unsafe { (*$iter).bi_size != 0 } {
            $paddr = unsafe { bio_iter_phys($bio, $iter) };
            $body
            unsafe { bio_advance_iter_single($bio, $iter, $blocksize) };
        }
    };
}

#[inline]
unsafe fn bio_get_size(bio: *mut bio) -> u32 {
    let mut bvec: *mut bio_vec;
    let mut ret: u32 = 0;
    let mut i: i32 = 0;

    bio_for_each_bvec_all!(bvec, bio, i, {
        ret = ret.wrapping_add((*bvec).bv_len);
    });
    ret
}

#[inline]
unsafe fn init_bvec_iter_for_bio(bio: *mut bio) -> bvec_iter {
    let bio_size: u32 = bio_get_size(bio);
    bvec_iter {
        bi_sector: 0,
        bi_size: bio_size,
        bi_idx: 0,
        bi_offset: 0,
    }
}

macro_rules! btrfs_bio_for_each_block_all {
    ($paddr:ident, $bio:expr, $blocksize:expr, $body:block) => {{
        let mut iter = unsafe { init_bvec_iter_for_bio($bio) };
        while iter.bi_size != 0 {
            $paddr = unsafe { bio_iter_phys($bio, &iter) };
            $body
            unsafe { bio_advance_iter_single($bio, &mut iter, $blocksize) };
        }
    }};
}

#[inline]
unsafe fn cond_wake_up(wq: *mut wait_queue_head) {
    if wq_has_sleeper(wq) {
        wake_up(wq);
    }
}

#[inline]
unsafe fn cond_wake_up_nomb(wq: *mut wait_queue_head) {
    if waitqueue_active(wq) {
        wake_up(wq);
    }
}

#[inline]
fn mult_perc(num: u64, percent: u32) -> u64 {
    div_u64(num.wrapping_mul(percent as u64), 100)
}

/* Copy of is_power_of_two that is 64-bit safe. */
#[inline]
fn is_power_of_two_u64(n: u64) -> bool {
    n != 0 && (n & n.wrapping_sub(1)) == 0
}

#[inline]
fn has_single_bit_set(n: u64) -> bool {
    is_power_of_two_u64(n)
}

/* Simple bytenr-based rb_tree related structure. */
#[repr(C)]
struct rb_simple_node {
    rb_node: rb_node,
    bytenr: u64,
}

#[inline]
unsafe fn rb_simple_search(root: *const rb_root, bytenr: u64) -> *mut rb_node {
    let mut node = (*root).rb_node;
    while !node.is_null() {
        let entry = rb_entry!(node, rb_simple_node, rb_node);
        if bytenr < (*entry).bytenr {
            node = (*node).rb_left;
        } else if bytenr > (*entry).bytenr {
            node = (*node).rb_right;
        } else {
            return node;
        }
    }
    core::ptr::null_mut()
}

#[inline]
unsafe fn rb_simple_search_first(root: *const rb_root, bytenr: u64) -> *mut rb_node {
    let mut node = (*root).rb_node;
    let mut ret = core::ptr::null_mut();
    let mut ret_entry: *mut rb_simple_node = core::ptr::null_mut();

    while !node.is_null() {
        let entry = rb_entry!(node, rb_simple_node, rb_node);
        if bytenr < (*entry).bytenr {
            if ret.is_null() || (*entry).bytenr < (*ret_entry).bytenr {
                ret = node;
                ret_entry = entry;
            }
            node = (*node).rb_left;
        } else if bytenr > (*entry).bytenr {
            node = (*node).rb_right;
        } else {
            return node;
        }
    }
    ret
}

unsafe fn rb_simple_node_bytenr_cmp(new: *mut rb_node, existing: *const rb_node) -> i32 {
    let new_entry = rb_entry!(new, rb_simple_node, rb_node);
    let existing_entry = rb_entry!(existing, rb_simple_node, rb_node);
    if (*new_entry).bytenr < (*existing_entry).bytenr { -1 }
    else if (*new_entry).bytenr > (*existing_entry).bytenr { 1 }
    else { 0 }
}

#[inline]
unsafe fn rb_simple_insert(root: *mut rb_root, simple_node: *mut rb_simple_node) -> *mut rb_node {
    rb_find_add!(&mut (*simple_node).rb_node, root, rb_simple_node_bytenr_cmp)
}

#[inline]
unsafe fn bitmap_test_range_all_set(addr: *const c_ulong, start: c_ulong, nbits: c_ulong) -> bool {
    let found_zero = find_next_zero_bit(addr, start + nbits, start);
    found_zero == start + nbits
}

#[inline]
unsafe fn bitmap_test_range_all_zero(addr: *const c_ulong, start: c_ulong, nbits: c_ulong) -> bool {
    let found_set = find_next_bit(addr, start + nbits, start);
    found_set == start + nbits
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
