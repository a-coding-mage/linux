// SPDX-License-Identifier: GPL-2.0
/*
 * Handle caching attributes in page tables (PAT)
 *
 * Authors: Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>
 *          Suresh B Siddha <suresh.b.siddha@intel.com>
 *
 * Interval tree used to store the PAT memory type reservations.
 */

// Linux header dependencies are supplied by the surrounding translation unit.

/*
 * The memtype tree keeps track of memory type for specific
 * physical memory areas. Without proper tracking, conflicting memory
 * types in different mappings can cause CPU cache corruption.
 *
 * The tree is an interval tree (augmented rbtree) which tree is ordered
 * by the starting address. The tree can contain multiple entries for
 * different regions which overlap. All the aliases have the same
 * cache attributes of course, as enforced by the PAT logic.
 *
 * memtype_lock protects the rbtree.
 */

#[inline]
unsafe fn interval_start(entry: *mut memtype) -> u64 {
    (*entry).start
}

#[inline]
unsafe fn interval_end(entry: *mut memtype) -> u64 {
    (*entry).end.wrapping_sub(1)
}

// Equivalent to INTERVAL_TREE_DEFINE(struct memtype, rb, u64, subtree_max_end,
// interval_start, interval_end, static, interval).

static mut memtype_rbroot: rb_root_cached = RB_ROOT_CACHED;

unsafe fn memtype_check_conflict(
    start: u64,
    end: u64,
    reqtype: page_cache_mode,
    newtype: *mut page_cache_mode,
) -> i32 {
    let mut entry_match: *mut memtype;
    let mut found_type = reqtype;

    entry_match = interval_iter_first(&mut memtype_rbroot, start, end.wrapping_sub(1));
    if entry_match.is_null() {
        if !newtype.is_null() {
            *newtype = found_type;
        }
        return 0;
    }

    if (*entry_match).r#type != found_type && newtype.is_null() {
        return -EBUSY;
    }

    dprintk!("Overlap at 0x%Lx-0x%Lx\n", (*entry_match).start, (*entry_match).end);
    found_type = (*entry_match).r#type;

    entry_match = interval_iter_next(entry_match, start, end.wrapping_sub(1));
    while !entry_match.is_null() {
        if (*entry_match).r#type != found_type {
            pr_info!(
                "x86/PAT: %s:%d conflicting memory types %Lx-%Lx %s<->%s\n",
                current.comm,
                current.pid,
                start,
                end,
                cattr_name(found_type),
                cattr_name((*entry_match).r#type)
            );
            return -EBUSY;
        }
        entry_match = interval_iter_next(entry_match, start, end.wrapping_sub(1));
    }

    if !newtype.is_null() {
        *newtype = found_type;
    }
    return 0;
}

pub unsafe fn memtype_check_insert(
    entry_new: *mut memtype,
    ret_type: *mut page_cache_mode,
) -> i32 {
    let err = memtype_check_conflict(
        (*entry_new).start,
        (*entry_new).end,
        (*entry_new).r#type,
        ret_type,
    );
    if err != 0 {
        return err;
    }

    if !ret_type.is_null() {
        (*entry_new).r#type = *ret_type;
    }

    interval_insert(entry_new, &mut memtype_rbroot);
    0
}

pub unsafe fn memtype_erase(start: u64, end: u64) -> *mut memtype {
    let mut entry = interval_iter_first(&mut memtype_rbroot, start, end.wrapping_sub(1));

    while !entry.is_null() && (*entry).start < end {
        if (*entry).start == start && (*entry).end == end {
            interval_remove(entry, &mut memtype_rbroot);
            return entry;
        }
        entry = interval_iter_next(entry, start, end.wrapping_sub(1));
    }
    ERR_PTR(-EINVAL)
}

pub unsafe fn memtype_lookup(addr: u64) -> *mut memtype {
    interval_iter_first(
        &mut memtype_rbroot,
        addr,
        addr.wrapping_add(PAGE_SIZE).wrapping_sub(1),
    )
}

/*
 * Debugging helper, copy the Nth entry of the tree into a
 * a copy for printout. This allows us to print out the tree
 * via debugfs, without holding the memtype_lock too long:
 */
#[cfg(CONFIG_DEBUG_FS)]
pub unsafe fn memtype_copy_nth_element(entry_out: *mut memtype, pos: loff_t) -> i32 {
    let mut entry_match = interval_iter_first(&mut memtype_rbroot, 0, ULONG_MAX);
    let mut i: i32 = 1;

    while !entry_match.is_null() && pos != i as loff_t {
        entry_match = interval_iter_next(entry_match, 0, ULONG_MAX);
        i += 1;
    }

    if !entry_match.is_null() {
        *entry_out = *entry_match;
        0
    } else {
        1
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
