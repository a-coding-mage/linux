/* SPDX-License-Identifier: GPL-2.0-only */
// Translated from drbd_interval.h.
// Dependencies corresponding to <linux/types.h> and <linux/rbtree.h> are
// supplied externally.

#[repr(C)]
pub struct drbd_interval {
    pub rb: rb_node,
    pub sector: sector_t, // start sector of the interval
    pub end: sector_t, // highest interval end in subtree
    pub size: ::core::ffi::c_uint, // size in bytes

    // C bit-fields: local or remote request; someone is waiting for
    // completion; this has been completed already (ignore for conflict
    // detection).
    pub local: ::core::ffi::c_uint,
    pub waiting: ::core::ffi::c_uint,
    pub completed: ::core::ffi::c_uint,

    // to resume a partially successful drbd_al_begin_io_nonblock();
    pub partially_in_al_next_enr: ::core::ffi::c_uint,
}

#[inline]
pub unsafe fn drbd_clear_interval(i: *mut drbd_interval) {
    RB_CLEAR_NODE(&mut (*i).rb);
}

#[inline]
pub unsafe fn drbd_interval_empty(i: *mut drbd_interval) -> bool {
    RB_EMPTY_NODE(&(*i).rb)
}

extern "C" {
    pub fn drbd_insert_interval(
        root: *mut rb_root,
        i: *mut drbd_interval,
    ) -> bool;
    pub fn drbd_contains_interval(
        root: *mut rb_root,
        sector: sector_t,
        i: *mut drbd_interval,
    ) -> bool;
    pub fn drbd_remove_interval(root: *mut rb_root, i: *mut drbd_interval);
    pub fn drbd_find_overlap(
        root: *mut rb_root,
        sector: sector_t,
        size: ::core::ffi::c_uint,
    ) -> *mut drbd_interval;
    pub fn drbd_next_overlap(
        i: *mut drbd_interval,
        sector: sector_t,
        size: ::core::ffi::c_uint,
    ) -> *mut drbd_interval;
}

// C macro equivalent:
// for (i = drbd_find_overlap(root, sector, size);
//      i;
//      i = drbd_next_overlap(i, sector, size))
#[macro_export]
macro_rules! drbd_for_each_overlap {
    ($i:ident, $root:expr, $sector:expr, $size:expr) => {
        for $i in core::iter::successors(
            unsafe { Some(drbd_find_overlap($root, $sector, $size)) },
            |current| {
                if current.is_null() {
                    None
                } else {
                    unsafe { Some(drbd_next_overlap(*current, $sector, $size)) }
                }
            },
        )
        .take_while(|current| !current.is_null())
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
