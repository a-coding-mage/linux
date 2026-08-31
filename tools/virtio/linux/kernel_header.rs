/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from a C header. C include directives and the header guard are
 * omitted; referenced Linux/libc symbols are expected to be supplied by the
 * surrounding translation.
 */

pub const CONFIG_SMP: bool = true;

pub unsafe fn PAGE_SIZE() -> usize {
    getpagesize() as usize
}

pub unsafe fn PAGE_MASK() -> usize {
    !(PAGE_SIZE().wrapping_sub(1))
}

pub unsafe fn PAGE_ALIGN(x: usize) -> usize {
    x.wrapping_add(PAGE_SIZE()).wrapping_sub(1) & PAGE_MASK()
}

/* generic data direction definitions */
pub const READ: i32 = 0;
pub const WRITE: i32 = 1;

pub type dma_addr_t = ::std::os::raw::c_ulonglong;
pub type __kernel_size_t = usize;
pub type __wsum = ::std::os::raw::c_uint;

#[repr(C)]
pub struct page {
    pub dummy: ::std::os::raw::c_ulonglong,
}

/* Physical == Virtual */
pub fn virt_to_phys(p: *const ::std::ffi::c_void) -> ::std::os::raw::c_ulong {
    p as ::std::os::raw::c_ulong
}

pub fn phys_to_virt(a: ::std::os::raw::c_ulong) -> *mut ::std::ffi::c_void {
    a as usize as *mut ::std::ffi::c_void
}

/* Page address: Virtual / 4K */
pub fn page_to_phys(p: *const page) -> dma_addr_t {
    p as ::std::os::raw::c_ulong as dma_addr_t
}

pub unsafe fn virt_to_page(p: *const ::std::ffi::c_void) -> *mut page {
    ((p as usize) & PAGE_MASK()) as *mut page
}

pub unsafe fn offset_in_page(p: *const ::std::ffi::c_void) -> usize {
    (p as usize) % PAGE_SIZE()
}

/* C __printf(a,b) format-checking attribute has no Rust equivalent. */

#[macro_export]
macro_rules! ARRAY_SIZE {
    ($x:expr) => {
        ::std::mem::size_of_val(&$x) / ::std::mem::size_of_val(&$x[0])
    };
}

unsafe extern "C" {
    pub static mut __kmalloc_fake: *mut ::std::ffi::c_void;
    pub static mut __kfree_ignore_start: *mut ::std::ffi::c_void;
    pub static mut __kfree_ignore_end: *mut ::std::ffi::c_void;

    pub fn malloc(size: usize) -> *mut ::std::ffi::c_void;
    pub fn free(ptr: *mut ::std::ffi::c_void);
    pub fn realloc(ptr: *mut ::std::ffi::c_void, size: usize) -> *mut ::std::ffi::c_void;
    pub fn memset(
        s: *mut ::std::ffi::c_void,
        c: ::std::os::raw::c_int,
        n: usize,
    ) -> *mut ::std::ffi::c_void;
    pub fn posix_memalign(
        memptr: *mut *mut ::std::ffi::c_void,
        alignment: usize,
        size: usize,
    ) -> ::std::os::raw::c_int;
    pub fn getpagesize() -> ::std::os::raw::c_int;
    pub fn fprintf(
        stream: *mut FILE,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;

    pub static mut stderr: *mut FILE;
}

#[repr(C)]
pub struct FILE {
    _unused: [u8; 0],
}

pub unsafe fn kmalloc(s: usize, _gfp: gfp_t) -> *mut ::std::ffi::c_void {
    if !__kmalloc_fake.is_null() {
        return __kmalloc_fake;
    }
    malloc(s)
}

pub unsafe fn kmalloc_array(
    n: ::std::os::raw::c_uint,
    s: usize,
    gfp: gfp_t,
) -> *mut ::std::ffi::c_void {
    kmalloc((n as usize).wrapping_mul(s), gfp)
}

#[macro_export]
macro_rules! kmalloc_obj {
    ($var_or_type:ty $(, $($rest:tt)*)?) => {
        kmalloc(::std::mem::size_of::<$var_or_type>(), 0 as _)
            as *mut $var_or_type
    };
}

#[macro_export]
macro_rules! kmalloc_objs {
    ($var_or_type:ty, $count:expr $(, $($rest:tt)*)?) => {
        kmalloc(
            ::std::mem::size_of::<$var_or_type>().wrapping_mul($count as usize),
            0 as _,
        ) as *mut $var_or_type
    };
}

pub unsafe fn kzalloc(s: usize, gfp: gfp_t) -> *mut ::std::ffi::c_void {
    let p = kmalloc(s, gfp);

    memset(p, 0, s);
    p
}

pub unsafe fn alloc_pages_exact(s: usize, gfp: gfp_t) -> *mut ::std::ffi::c_void {
    kmalloc(s, gfp)
}

pub unsafe fn kfree(p: *mut ::std::ffi::c_void) {
    if p >= __kfree_ignore_start && p < __kfree_ignore_end {
        return;
    }
    free(p);
}

pub unsafe fn free_pages_exact(p: *mut ::std::ffi::c_void, _s: usize) {
    kfree(p);
}

pub unsafe fn krealloc(
    p: *mut ::std::ffi::c_void,
    s: usize,
    _gfp: gfp_t,
) -> *mut ::std::ffi::c_void {
    realloc(p, s)
}

pub unsafe fn __get_free_page(_gfp: gfp_t) -> ::std::os::raw::c_ulong {
    let mut p: *mut ::std::ffi::c_void = ::std::ptr::null_mut();

    posix_memalign(&mut p, PAGE_SIZE(), PAGE_SIZE());
    p as ::std::os::raw::c_ulong
}

pub unsafe fn free_page(addr: ::std::os::raw::c_ulong) {
    free(addr as *mut ::std::ffi::c_void);
}

#[macro_export]
macro_rules! likely {
    ($x:expr) => {
        $x
    };
}

#[macro_export]
macro_rules! unlikely {
    ($x:expr) => {
        $x
    };
}

pub unsafe fn krealloc_array(
    p: *mut ::std::ffi::c_void,
    new_n: usize,
    new_size: usize,
    gfp: gfp_t,
) -> *mut ::std::ffi::c_void {
    let mut bytes: usize = 0;

    if unlikely!(check_mul_overflow(new_n, new_size, &mut bytes)) {
        return ::std::ptr::null_mut();
    }

    krealloc(p, bytes, gfp)
}

#[macro_export]
macro_rules! pr_err {
    ($format:expr $(, $args:expr)* $(,)?) => {
        fprintf(stderr, $format $(, $args)*)
    };
}

/* DEBUG controls whether pr_debug prints or is a no-op in C. */
#[cfg(DEBUG)]
#[macro_export]
macro_rules! pr_debug {
    ($format:expr $(, $args:expr)* $(,)?) => {
        fprintf(stderr, $format $(, $args)*)
    };
}

#[cfg(not(DEBUG))]
#[macro_export]
macro_rules! pr_debug {
    ($format:expr $(, $args:expr)* $(,)?) => {{}};
}

#[macro_export]
macro_rules! dev_err {
    ($dev:expr, $format:expr $(, $args:expr)* $(,)?) => {
        fprintf(stderr, $format $(, $args)*)
    };
}

#[macro_export]
macro_rules! dev_warn {
    ($dev:expr, $format:expr $(, $args:expr)* $(,)?) => {
        fprintf(stderr, $format $(, $args)*)
    };
}

#[macro_export]
macro_rules! dev_warn_once {
    ($dev:expr, $format:expr $(, $args:expr)* $(,)?) => {
        fprintf(stderr, $format $(, $args)*)
    };
}

#[macro_export]
macro_rules! dev_WARN_ONCE {
    ($dev:expr, $condition:expr, $($format:tt)*) => {
        WARN_ONCE!($condition, $($format)*)
    };
}

pub fn is_vmalloc_addr(_x: *const ::std::ffi::c_void) -> bool {
    false
}

#[macro_export]
macro_rules! might_sleep {
    () => {{}};
}

pub fn synchronize_rcu() {
    assert!(false);
}

#[macro_export]
macro_rules! min {
    ($x:expr, $y:expr) => {{
        let _min1 = $x;
        let _min2 = $y;
        if _min1 < _min2 { _min1 } else { _min2 }
    }};
}
