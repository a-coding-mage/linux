/* SPDX-License-Identifier: GPL-2.0 */

// This header is a type-suffix template.  The C token-pasting macros are
// represented by the invocation of `btree_type!` below.

#[macro_export]
macro_rules! btree_type {
    ($suffix:tt, $geo:expr, $keytype:ty, $bits:expr) => {
        #[repr(C)]
        pub struct btree_head_$suffix {
            pub h: btree_head,
        }

        #[inline]
        pub unsafe fn btree_init_mempool_$suffix(
            head: *mut btree_head_$suffix,
            mempool: *mut mempool_t,
        ) {
            btree_init_mempool(&mut (*head).h, mempool);
        }

        #[inline]
        pub unsafe fn btree_init_$suffix(head: *mut btree_head_$suffix) -> i32 {
            btree_init(&mut (*head).h)
        }

        #[inline]
        pub unsafe fn btree_destroy_$suffix(head: *mut btree_head_$suffix) {
            btree_destroy(&mut (*head).h);
        }

        #[inline]
        pub unsafe fn btree_merge_$suffix(
            target: *mut btree_head_$suffix,
            victim: *mut btree_head_$suffix,
            gfp: gfp_t,
        ) -> i32 {
            btree_merge(&mut (*target).h, &mut (*victim).h, $geo, gfp)
        }

        #[inline]
        pub unsafe fn btree_lookup_$suffix(
            head: *mut btree_head_$suffix,
            key: $keytype,
        ) -> *mut kernel::ffi::c_void {
            let mut key: kernel::ffi::c_ulong = key as kernel::ffi::c_ulong;
            btree_lookup(&mut (*head).h, $geo, &mut key)
        }

        #[inline]
        pub unsafe fn btree_insert_$suffix(
            head: *mut btree_head_$suffix,
            key: $keytype,
            val: *mut kernel::ffi::c_void,
            gfp: gfp_t,
        ) -> i32 {
            let mut key: kernel::ffi::c_ulong = key as kernel::ffi::c_ulong;
            btree_insert(&mut (*head).h, $geo, &mut key, val, gfp)
        }

        #[inline]
        pub unsafe fn btree_update_$suffix(
            head: *mut btree_head_$suffix,
            key: $keytype,
            val: *mut kernel::ffi::c_void,
        ) -> i32 {
            let mut key: kernel::ffi::c_ulong = key as kernel::ffi::c_ulong;
            btree_update(&mut (*head).h, $geo, &mut key, val)
        }

        #[inline]
        pub unsafe fn btree_remove_$suffix(
            head: *mut btree_head_$suffix,
            key: $keytype,
        ) -> *mut kernel::ffi::c_void {
            let mut key: kernel::ffi::c_ulong = key as kernel::ffi::c_ulong;
            btree_remove(&mut (*head).h, $geo, &mut key)
        }

        #[inline]
        pub unsafe fn btree_last_$suffix(
            head: *mut btree_head_$suffix,
            key: *mut $keytype,
        ) -> *mut kernel::ffi::c_void {
            let mut local_key: kernel::ffi::c_ulong = 0;
            let val = btree_last(&mut (*head).h, $geo, &mut local_key);
            if !val.is_null() { *key = local_key as $keytype; }
            val
        }

        #[inline]
        pub unsafe fn btree_get_prev_$suffix(
            head: *mut btree_head_$suffix,
            key: *mut $keytype,
        ) -> *mut kernel::ffi::c_void {
            let mut local_key: kernel::ffi::c_ulong = *key as kernel::ffi::c_ulong;
            let val = btree_get_prev(&mut (*head).h, $geo, &mut local_key);
            if !val.is_null() { *key = local_key as $keytype; }
            val
        }

        pub type visitor_::kernel::macros::paste!([<$suffix _t>]) = unsafe extern "C" fn(
            elem: *mut kernel::ffi::c_void,
            opaque: kernel::ffi::c_ulong,
            key: $keytype,
            index: usize,
        );

        extern "C" {
            pub fn visitor_$suffix(
                elem: *mut kernel::ffi::c_void,
                opaque: kernel::ffi::c_ulong,
                key: *mut kernel::ffi::c_ulong,
                index: usize,
                func: *mut kernel::ffi::c_void,
            );
        }

        #[inline]
        pub unsafe fn btree_visitor_$suffix(
            head: *mut btree_head_$suffix,
            opaque: kernel::ffi::c_ulong,
            func2: visitor_::kernel::macros::paste!([<$suffix _t>]),
        ) -> usize {
            btree_visitor(&mut (*head).h, $geo, opaque, visitor_$suffix, func2)
        }

        #[inline]
        pub unsafe fn btree_grim_visitor_$suffix(
            head: *mut btree_head_$suffix,
            opaque: kernel::ffi::c_ulong,
            func2: visitor_::kernel::macros::paste!([<$suffix _t>]),
        ) -> usize {
            btree_grim_visitor(&mut (*head).h, $geo, opaque, visitor_$suffix, func2)
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
