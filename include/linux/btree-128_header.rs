/* SPDX-License-Identifier: GPL-2.0 */

extern "C" {
    pub static mut btree_geo128: btree_geo;
}

#[repr(C)]
pub struct btree_head128 {
    pub h: btree_head,
}

#[inline]
pub unsafe fn btree_init_mempool128(head: *mut btree_head128, mempool: *mut mempool_t) {
    btree_init_mempool(&mut (*head).h, mempool);
}

#[inline]
pub unsafe fn btree_init128(head: *mut btree_head128) -> ::std::os::raw::c_int {
    btree_init(&mut (*head).h)
}

#[inline]
pub unsafe fn btree_destroy128(head: *mut btree_head128) {
    btree_destroy(&mut (*head).h);
}

#[inline]
pub unsafe fn btree_lookup128(
    head: *mut btree_head128,
    k1: u64,
    k2: u64,
) -> *mut ::std::ffi::c_void {
    let mut key: [u64; 2] = [k1, k2];
    btree_lookup(
        &mut (*head).h,
        &raw mut btree_geo128,
        key.as_mut_ptr() as *mut ::std::os::raw::c_ulong,
    )
}

#[inline]
pub unsafe fn btree_get_prev128(
    head: *mut btree_head128,
    k1: *mut u64,
    k2: *mut u64,
) -> *mut ::std::ffi::c_void {
    let mut key: [u64; 2] = [*k1, *k2];
    let val = btree_get_prev(
        &mut (*head).h,
        &raw mut btree_geo128,
        key.as_mut_ptr() as *mut ::std::os::raw::c_ulong,
    );
    *k1 = key[0];
    *k2 = key[1];
    val
}

#[inline]
pub unsafe fn btree_insert128(
    head: *mut btree_head128,
    k1: u64,
    k2: u64,
    val: *mut ::std::ffi::c_void,
    gfp: gfp_t,
) -> ::std::os::raw::c_int {
    let mut key: [u64; 2] = [k1, k2];
    btree_insert(
        &mut (*head).h,
        &raw mut btree_geo128,
        key.as_mut_ptr() as *mut ::std::os::raw::c_ulong,
        val,
        gfp,
    )
}

#[inline]
pub unsafe fn btree_update128(
    head: *mut btree_head128,
    k1: u64,
    k2: u64,
    val: *mut ::std::ffi::c_void,
) -> ::std::os::raw::c_int {
    let mut key: [u64; 2] = [k1, k2];
    btree_update(
        &mut (*head).h,
        &raw mut btree_geo128,
        key.as_mut_ptr() as *mut ::std::os::raw::c_ulong,
        val,
    )
}

#[inline]
pub unsafe fn btree_remove128(
    head: *mut btree_head128,
    k1: u64,
    k2: u64,
) -> *mut ::std::ffi::c_void {
    let mut key: [u64; 2] = [k1, k2];
    btree_remove(
        &mut (*head).h,
        &raw mut btree_geo128,
        key.as_mut_ptr() as *mut ::std::os::raw::c_ulong,
    )
}

#[inline]
pub unsafe fn btree_last128(
    head: *mut btree_head128,
    k1: *mut u64,
    k2: *mut u64,
) -> *mut ::std::ffi::c_void {
    let mut key: [u64; 2] = [0, 0];
    let val = btree_last(
        &mut (*head).h,
        &raw mut btree_geo128,
        key.as_mut_ptr() as *mut ::std::os::raw::c_ulong,
    );
    if !val.is_null() {
        *k1 = key[0];
        *k2 = key[1];
    }
    val
}

#[inline]
pub unsafe fn btree_merge128(
    target: *mut btree_head128,
    victim: *mut btree_head128,
    gfp: gfp_t,
) -> ::std::os::raw::c_int {
    btree_merge(&mut (*target).h, &mut (*victim).h, &raw mut btree_geo128, gfp)
}

extern "C" {
    pub fn visitor128(
        elem: *mut ::std::ffi::c_void,
        opaque: ::std::os::raw::c_ulong,
        key: *mut ::std::os::raw::c_ulong,
        index: usize,
        func: *mut ::std::ffi::c_void,
    );
}

pub type visitor128_t = unsafe extern "C" fn(
    elem: *mut ::std::ffi::c_void,
    opaque: ::std::os::raw::c_ulong,
    key1: u64,
    key2: u64,
    index: usize,
);

#[inline]
pub unsafe fn btree_visitor128(
    head: *mut btree_head128,
    opaque: ::std::os::raw::c_ulong,
    func2: visitor128_t,
) -> usize {
    btree_visitor(&mut (*head).h, &raw mut btree_geo128, opaque, visitor128, func2)
}

#[inline]
pub unsafe fn btree_grim_visitor128(
    head: *mut btree_head128,
    opaque: ::std::os::raw::c_ulong,
    func2: visitor128_t,
) -> usize {
    btree_grim_visitor(&mut (*head).h, &raw mut btree_geo128, opaque, visitor128, func2)
}

#[macro_export]
macro_rules! btree_for_each_safe128 {
    ($head:expr, $k1:expr, $k2:expr, $val:ident, $body:block) => {{
        loop {
            $val = unsafe { $crate::btree_last128($head, &mut $k1, &mut $k2) };
            if $val.is_null() {
                break;
            }
            $body
            $val = unsafe { $crate::btree_get_prev128($head, &mut $k1, &mut $k2) };
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
