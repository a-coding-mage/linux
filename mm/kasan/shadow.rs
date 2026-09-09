// SPDX-License-Identifier: GPL-2.0
/* Rust translation of kasan/shadow.c. Kernel dependencies are external. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub type u8 = core::ffi::c_uchar;
pub type gfp_t = core::ffi::c_uint;
pub type ssize_t = isize;
pub type kasan_vmalloc_flags_t = core::ffi::c_ulong;

extern "C" {
    fn kasan_check_range(addr: *mut core::ffi::c_void, size: usize, write: bool, ip: usize) -> bool;
    fn __memset(addr: *mut core::ffi::c_void, c: i32, len: usize) -> *mut core::ffi::c_void;
    fn __memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void;
    fn __memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void;
    fn kasan_enabled() -> bool;
    fn kasan_reset_tag(addr: *const core::ffi::c_void) -> *const core::ffi::c_void;
    fn kasan_mem_to_shadow(addr: *const core::ffi::c_void) -> *mut u8;
    fn get_tag(addr: *const core::ffi::c_void) -> u8;
    fn round_up(x: usize, y: usize) -> usize;
    fn kasan_poison_last_granule(addr: *const core::ffi::c_void, size: usize);
    fn kasan_random_tag() -> u8;
    fn set_tag(addr: *const core::ffi::c_void, tag: u8) -> *const core::ffi::c_void;
    fn is_vmalloc_or_module_addr(addr: *const core::ffi::c_void) -> bool;
}

const KASAN_GRANULE_MASK: usize = 7;
const KASAN_GRANULE_SIZE: usize = 8;

#[no_mangle]
pub unsafe extern "C" fn __kasan_check_read(p: *const core::ffi::c_void, size: u32) -> bool {
    kasan_check_range(p as *mut _, size as usize, false, 0)
}

#[no_mangle]
pub unsafe extern "C" fn __kasan_check_write(p: *const core::ffi::c_void, size: u32) -> bool {
    kasan_check_range(p as *mut _, size as usize, true, 0)
}

#[cfg(not(any(feature = "CONFIG_CC_HAS_KASAN_MEMINTRINSIC_PREFIX", feature = "CONFIG_GENERIC_ENTRY")))]
#[no_mangle]
pub unsafe extern "C" fn memset(addr: *mut core::ffi::c_void, c: i32, len: usize) -> *mut core::ffi::c_void {
    if !kasan_check_range(addr, len, true, 0) { return core::ptr::null_mut(); }
    __memset(addr, c, len)
}

#[cfg(not(any(feature = "CONFIG_CC_HAS_KASAN_MEMINTRINSIC_PREFIX", feature = "CONFIG_GENERIC_ENTRY")))]
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void {
    if !kasan_check_range(src as *mut _, len, false, 0) || !kasan_check_range(dest, len, true, 0) { return core::ptr::null_mut(); }
    __memcpy(dest, src, len)
}

#[cfg(not(any(feature = "CONFIG_CC_HAS_KASAN_MEMINTRINSIC_PREFIX", feature = "CONFIG_GENERIC_ENTRY")))]
#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void {
    if !kasan_check_range(src as *mut _, len, false, 0) || !kasan_check_range(dest, len, true, 0) { return core::ptr::null_mut(); }
    __memmove(dest, src, len)
}

#[no_mangle]
pub unsafe extern "C" fn __asan_memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: ssize_t) -> *mut core::ffi::c_void {
    if !kasan_check_range(src as *mut _, len as usize, false, 0) || !kasan_check_range(dest, len as usize, true, 0) { return core::ptr::null_mut(); }
    __memmove(dest, src, len as usize)
}

#[no_mangle]
pub unsafe extern "C" fn __asan_memset(addr: *mut core::ffi::c_void, c: i32, len: ssize_t) -> *mut core::ffi::c_void {
    if !kasan_check_range(addr, len as usize, true, 0) { return core::ptr::null_mut(); }
    __memset(addr, c, len as usize)
}

#[no_mangle]
pub unsafe extern "C" fn __asan_memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: ssize_t) -> *mut core::ffi::c_void {
    if !kasan_check_range(src as *mut _, len as usize, false, 0) || !kasan_check_range(dest, len as usize, true, 0) { return core::ptr::null_mut(); }
    __memcpy(dest, src, len as usize)
}

#[cfg(feature = "CONFIG_KASAN_SW_TAGS")]
pub use __asan_memset as __hwasan_memset;
#[cfg(feature = "CONFIG_KASAN_SW_TAGS")]
pub use __asan_memcpy as __hwasan_memcpy;

#[no_mangle]
pub unsafe extern "C" fn kasan_poison(addr0: *const core::ffi::c_void, size: usize, value: u8, _init: bool) {
    if !kasan_enabled() { return; }
    let addr = kasan_reset_tag(addr0);
    if (addr as usize & KASAN_GRANULE_MASK) != 0 || (size & KASAN_GRANULE_MASK) != 0 { return; }
    let shadow_start = kasan_mem_to_shadow(addr);
    let shadow_end = kasan_mem_to_shadow(addr.add(size));
    __memset(shadow_start as *mut _, value as i32, shadow_end.offset_from(shadow_start) as usize);
}

#[cfg(feature = "CONFIG_KASAN_GENERIC")]
pub unsafe extern "C" fn kasan_poison_last_granule_local(addr: *const core::ffi::c_void, size: usize) {
    if !kasan_enabled() { return; }
    if size & KASAN_GRANULE_MASK != 0 { *(kasan_mem_to_shadow(addr.add(size))) = (size & KASAN_GRANULE_MASK) as u8; }
}

#[no_mangle]
pub unsafe extern "C" fn kasan_unpoison(addr0: *const core::ffi::c_void, size: usize, _init: bool) {
    let tag = get_tag(addr0);
    let addr = kasan_reset_tag(addr0);
    if addr as usize & KASAN_GRANULE_MASK != 0 { return; }
    kasan_poison(addr, round_up(size, KASAN_GRANULE_SIZE), tag, false);
    #[cfg(feature = "CONFIG_KASAN_GENERIC")]
    kasan_poison_last_granule_local(addr, size);
}

#[cfg(feature = "CONFIG_KASAN_VMALLOC")]
pub unsafe extern "C" fn __kasan_unpoison_vmalloc(start: *const core::ffi::c_void, size: usize, flags: kasan_vmalloc_flags_t) -> *mut core::ffi::c_void {
    if !is_vmalloc_or_module_addr(start) { return start as *mut _; }
    const KASAN_VMALLOC_PROT_NORMAL: usize = 1 << 0;
    const KASAN_VMALLOC_KEEP_TAG: usize = 1 << 1;
    if cfg!(feature = "CONFIG_KASAN_SW_TAGS") && flags & KASAN_VMALLOC_PROT_NORMAL == 0 { return start as *mut _; }
    let s = if flags & KASAN_VMALLOC_KEEP_TAG == 0 { set_tag(start, kasan_random_tag()) } else { start };
    kasan_unpoison(s, size, false);
    s as *mut _
}

#[cfg(feature = "CONFIG_KASAN_VMALLOC")]
pub unsafe extern "C" fn __kasan_poison_vmalloc(start: *const core::ffi::c_void, size: usize) {
    if is_vmalloc_or_module_addr(start) { kasan_poison(start, round_up(size, KASAN_GRANULE_SIZE), 0xFE, false); }
}

#[cfg(not(feature = "CONFIG_KASAN_VMALLOC"))]
pub unsafe extern "C" fn kasan_alloc_module_shadow(_addr: *mut core::ffi::c_void, _size: usize, _gfp_mask: gfp_t) -> i32 { -12 }

#[cfg(not(feature = "CONFIG_KASAN_VMALLOC"))]
pub unsafe extern "C" fn kasan_free_module_shadow(_vm: *const core::ffi::c_void) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
