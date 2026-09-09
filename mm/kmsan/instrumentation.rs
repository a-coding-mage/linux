// SPDX-License-Identifier: GPL-2.0
/*
 * KMSAN compiler API.
 *
 * This file implements __msan_XXX hooks that Clang inserts into the code
 * compiled with -fsanitize=kernel-memory.
 * See Documentation/dev-tools/kmsan.rst for more information on how KMSAN
 * instrumentation works.
 *
 * Copyright (C) 2017-2022 Google LLC
 * Author: Alexander Potapenko <glider@google.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

type U64 = u64;
type U32 = u32;
type Uintptr = usize;
type DepotStackHandle = u32;

#[repr(C)]
pub struct ShadowOriginPtr {
    pub shadow: *mut c_void,
    pub origin: *mut c_void,
}

#[repr(C)]
pub struct KmsanContextState {
    pub param_tls: *mut c_void,
    pub param_origin_tls: [DepotStackHandle; 1],
    pub retval_tls: *mut c_void,
    pub retval_origin_tls: DepotStackHandle,
}

#[repr(C)]
pub struct KmsanCtx {
    pub cstate: KmsanContextState,
}

extern "C" {
    static mut kmsan_enabled: bool;
    fn kmsan_get_metadata(addr: *mut c_void, meta: i32) -> *mut c_void;
    fn user_access_save() -> u64;
    fn user_access_restore(flags: u64);
    fn kmsan_get_shadow_origin_ptr(
        addr: *mut c_void,
        size: U64,
        store: bool,
    ) -> ShadowOriginPtr;
    fn kmsan_get_context() -> *mut KmsanCtx;
    fn __memmove(dst: *mut c_void, src: *const c_void, n: Uintptr) -> *mut c_void;
    fn __memcpy(dst: *mut c_void, src: *const c_void, n: Uintptr) -> *mut c_void;
    fn __memset(dst: *mut c_void, c: i32, n: Uintptr) -> *mut c_void;
    fn kmsan_in_runtime() -> bool;
    fn kmsan_enter_runtime();
    fn kmsan_leave_runtime();
    fn kmsan_internal_memmove_metadata(dst: *mut c_void, src: *mut c_void, n: Uintptr);
    fn kmsan_internal_unpoison_memory(addr: *mut c_void, size: Uintptr, checked: bool);
    fn kmsan_internal_chain_origin(origin: DepotStackHandle) -> DepotStackHandle;
    fn kmsan_internal_set_shadow_origin(
        address: *mut c_void,
        size: Uintptr,
        shadow: i32,
        origin: DepotStackHandle,
        checked: bool,
    );
    fn kmsan_report(
        origin: U32,
        address: *mut c_void,
        size: Uintptr,
        off_first: Uintptr,
        off_last: Uintptr,
        user_addr: *mut c_void,
        reason: i32,
    );
    fn stack_depot_save(entries: *const u64, nr_entries: usize, gfp: u32) -> DepotStackHandle;
}

const KMSAN_META_SHADOW: i32 = 0;
const KMSAN_ALLOCA_MAGIC_ORIGIN: u64 = 0;
const REASON_ANY: i32 = 0;
const __GFP_HIGH: u32 = 0;

#[inline]
unsafe fn is_bad_asm_addr(addr: *mut c_void, _size: Uintptr, _is_store: bool) -> bool {
    // CONFIG_ARCH_HAS_NON_OVERLAPPING_ADDRESS_SPACE is a build-time condition.
    if (addr as usize as u64) < 0 {
        return true;
    }
    if kmsan_get_metadata(addr, KMSAN_META_SHADOW).is_null() {
        return true;
    }
    false
}

#[inline]
unsafe fn get_shadow_origin_ptr(addr: *mut c_void, size: U64, store: bool) -> ShadowOriginPtr {
    let ua_flags = user_access_save();
    let ret = kmsan_get_shadow_origin_ptr(addr, size, store);
    user_access_restore(ua_flags);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn __msan_metadata_ptr_for_load_n(
    addr: *mut c_void,
    size: Uintptr,
) -> ShadowOriginPtr {
    get_shadow_origin_ptr(addr, size as U64, false)
}

#[no_mangle]
pub unsafe extern "C" fn __msan_metadata_ptr_for_store_n(
    addr: *mut c_void,
    size: Uintptr,
) -> ShadowOriginPtr {
    get_shadow_origin_ptr(addr, size as U64, true)
}

macro_rules! declare_metadata_ptr_getter {
    ($size:expr, $load:ident, $store:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $load(addr: *mut c_void) -> ShadowOriginPtr {
            get_shadow_origin_ptr(addr, $size, false)
        }
        #[no_mangle]
        pub unsafe extern "C" fn $store(addr: *mut c_void) -> ShadowOriginPtr {
            get_shadow_origin_ptr(addr, $size, true)
        }
    };
}

declare_metadata_ptr_getter!(1, __msan_metadata_ptr_for_load_1, __msan_metadata_ptr_for_store_1);
declare_metadata_ptr_getter!(2, __msan_metadata_ptr_for_load_2, __msan_metadata_ptr_for_store_2);
declare_metadata_ptr_getter!(4, __msan_metadata_ptr_for_load_4, __msan_metadata_ptr_for_store_4);
declare_metadata_ptr_getter!(8, __msan_metadata_ptr_for_load_8, __msan_metadata_ptr_for_store_8);

#[no_mangle]
pub unsafe extern "C" fn __msan_instrument_asm_store(addr: *mut c_void, mut size: Uintptr) {
    if !kmsan_enabled {
        return;
    }
    let ua_flags = user_access_save();
    if size > 4096 {
        // WARN_ONCE(1, "assembly store size too big: %ld\n", size)
        size = 8;
    }
    if is_bad_asm_addr(addr, size, true) {
        user_access_restore(ua_flags);
        return;
    }
    kmsan_internal_unpoison_memory(addr, size, false);
    user_access_restore(ua_flags);
}

#[inline]
unsafe fn get_param0_metadata(shadow: &mut U64, origin: &mut DepotStackHandle) {
    let ctx = kmsan_get_context();
    *shadow = *( (*ctx).cstate.param_tls as *const U64);
    *origin = (*ctx).cstate.param_origin_tls[0];
}

#[inline]
unsafe fn set_retval_metadata(shadow: U64, origin: DepotStackHandle) {
    let ctx = kmsan_get_context();
    *((*ctx).cstate.retval_tls as *mut U64) = shadow;
    (*ctx).cstate.retval_origin_tls = origin;
}

#[no_mangle]
pub unsafe extern "C" fn __msan_memmove(dst: *mut c_void, src: *const c_void, n: Uintptr) -> *mut c_void {
    let mut origin = 0;
    let mut shadow = 0;
    get_param0_metadata(&mut shadow, &mut origin);
    let result = __memmove(dst, src, n);
    if n == 0 || !kmsan_enabled || kmsan_in_runtime() { return result; }
    kmsan_enter_runtime();
    kmsan_internal_memmove_metadata(dst, src as *mut c_void, n);
    kmsan_leave_runtime();
    set_retval_metadata(shadow, origin);
    result
}

#[no_mangle]
pub unsafe extern "C" fn __msan_memcpy(dst: *mut c_void, src: *const c_void, n: Uintptr) -> *mut c_void {
    let mut origin = 0;
    let mut shadow = 0;
    get_param0_metadata(&mut shadow, &mut origin);
    let result = __memcpy(dst, src, n);
    if n == 0 || !kmsan_enabled || kmsan_in_runtime() { return result; }
    kmsan_enter_runtime();
    kmsan_internal_memmove_metadata(dst, src as *mut c_void, n);
    kmsan_leave_runtime();
    set_retval_metadata(shadow, origin);
    result
}

#[no_mangle]
pub unsafe extern "C" fn __msan_memset(dst: *mut c_void, c: i32, n: Uintptr) -> *mut c_void {
    let mut origin = 0;
    let mut shadow = 0;
    get_param0_metadata(&mut shadow, &mut origin);
    let result = __memset(dst, c, n);
    if !kmsan_enabled || kmsan_in_runtime() { return result; }
    kmsan_enter_runtime();
    kmsan_internal_unpoison_memory(dst, n, false);
    kmsan_leave_runtime();
    set_retval_metadata(shadow, origin);
    result
}

#[no_mangle]
pub unsafe extern "C" fn __msan_chain_origin(origin: DepotStackHandle) -> DepotStackHandle {
    if !kmsan_enabled || kmsan_in_runtime() { return 0; }
    let ua_flags = user_access_save();
    kmsan_enter_runtime();
    let ret = kmsan_internal_chain_origin(origin);
    kmsan_leave_runtime();
    user_access_restore(ua_flags);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn __msan_poison_alloca(address: *mut c_void, size: Uintptr, descr: *mut i8) {
    if !kmsan_enabled || kmsan_in_runtime() { return; }
    let ua_flags = user_access_save();
    let mut entries = [0u64; 4];
    entries[0] = KMSAN_ALLOCA_MAGIC_ORIGIN;
    entries[1] = descr as u64;
    entries[2] = 0; // __builtin_return_address(0)
    entries[3] = 0; // __builtin_return_address(1), when CONFIG_UNWINDER_FRAME_POINTER
    kmsan_enter_runtime();
    let handle = stack_depot_save(entries.as_ptr(), entries.len(), __GFP_HIGH);
    kmsan_leave_runtime();
    kmsan_internal_set_shadow_origin(address, size, -1, handle, true);
    user_access_restore(ua_flags);
}

#[no_mangle]
pub unsafe extern "C" fn __msan_unpoison_alloca(address: *mut c_void, size: Uintptr) {
    if !kmsan_enabled || kmsan_in_runtime() { return; }
    kmsan_enter_runtime();
    kmsan_internal_unpoison_memory(address, size, true);
    kmsan_leave_runtime();
}

#[no_mangle]
pub unsafe extern "C" fn __msan_warning(origin: U32) {
    kmsan_report(origin, core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), REASON_ANY);
}

#[no_mangle]
pub unsafe extern "C" fn __msan_get_context_state() -> *mut KmsanContextState {
    &mut (*kmsan_get_context()).cstate
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
