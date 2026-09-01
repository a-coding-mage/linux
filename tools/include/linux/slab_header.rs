/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies from the original C header:
 * <linux/types.h>, <linux/gfp.h>, and <pthread.h>.
 */

pub const SLAB_RECLAIM_ACCOUNT: c_ulong = 0x00020000;

pub const _SLAB_KMALLOC: c_uint = 0;
pub const _SLAB_HWCACHE_ALIGN: c_uint = 1;
pub const _SLAB_PANIC: c_uint = 2;
pub const _SLAB_TYPESAFE_BY_RCU: c_uint = 3;
pub const _SLAB_ACCOUNT: c_uint = 4;
pub const _SLAB_FLAGS_LAST_BIT: c_uint = 5;

pub const fn __SLAB_FLAG_BIT(nr: c_uint) -> c_uint {
    1u32.wrapping_shl(nr) as c_uint
}

pub const __SLAB_FLAG_UNUSED: c_uint = 0;

pub const SLAB_HWCACHE_ALIGN: c_uint = __SLAB_FLAG_BIT(_SLAB_HWCACHE_ALIGN);
pub const SLAB_PANIC: c_uint = __SLAB_FLAG_BIT(_SLAB_PANIC);
pub const SLAB_TYPESAFE_BY_RCU: c_uint = __SLAB_FLAG_BIT(_SLAB_TYPESAFE_BY_RCU);

/* CONFIG_MEMCG conditional in C:
 * if enabled, SLAB_ACCOUNT is __SLAB_FLAG_BIT(_SLAB_ACCOUNT);
 * otherwise it is __SLAB_FLAG_UNUSED.
 */
#[cfg(CONFIG_MEMCG)]
pub const SLAB_ACCOUNT: c_uint = __SLAB_FLAG_BIT(_SLAB_ACCOUNT);
#[cfg(not(CONFIG_MEMCG))]
pub const SLAB_ACCOUNT: c_uint = __SLAB_FLAG_UNUSED;

unsafe extern "C" {
    pub fn kmalloc(size: size_t, gfp: gfp_t) -> *mut c_void;
    pub fn kfree(p: *mut c_void);
    pub fn kmalloc_array(n: size_t, size: size_t, gfp: gfp_t) -> *mut c_void;

    pub fn slab_is_available() -> bool;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum slab_state {
    DOWN,
    PARTIAL,
    UP,
    FULL,
}

#[repr(C)]
pub struct kmem_cache {
    pub lock: pthread_mutex_t,
    pub size: c_uint,
    pub align: c_uint,
    pub sheaf_capacity: c_uint,
    pub nr_objs: c_int,
    pub objs: *mut c_void,
    pub ctor: Option<unsafe extern "C" fn(*mut c_void)>,
    pub non_kernel_enabled: bool,
    pub non_kernel: c_uint,
    pub nr_allocated: c_ulong,
    pub nr_tallocated: c_ulong,
    pub exec_callback: bool,
    pub callback: Option<unsafe extern "C" fn(*mut c_void)>,
    pub private: *mut c_void,
}

#[repr(C)]
pub struct kmem_cache_args {
    /**
     * @align: The required alignment for the objects.
     *
     * %0 means no specific alignment is requested.
     */
    pub align: c_uint,
    /**
     * @sheaf_capacity: The maximum size of the sheaf.
     */
    pub sheaf_capacity: c_uint,
    /**
     * @useroffset: Usercopy region offset.
     *
     * %0 is a valid offset, when @usersize is non-%0
     */
    pub useroffset: c_uint,
    /**
     * @usersize: Usercopy region size.
     *
     * %0 means no usercopy region is specified.
     */
    pub usersize: c_uint,
    /**
     * @freeptr_offset: Custom offset for the free pointer
     * in &SLAB_TYPESAFE_BY_RCU caches
     *
     * By default &SLAB_TYPESAFE_BY_RCU caches place the free pointer
     * outside of the object. This might cause the object to grow in size.
     * Cache creators that have a reason to avoid this can specify a custom
     * free pointer offset in their struct where the free pointer will be
     * placed.
     *
     * Note that placing the free pointer inside the object requires the
     * caller to ensure that no fields are invalidated that are required to
     * guard against object recycling (See &SLAB_TYPESAFE_BY_RCU for
     * details).
     *
     * Using %0 as a value for @freeptr_offset is valid. If @freeptr_offset
     * is specified, %use_freeptr_offset must be set %true.
     *
     * Note that @ctor currently isn't supported with custom free pointers
     * as a @ctor requires an external free pointer.
     */
    pub freeptr_offset: c_uint,
    /**
     * @use_freeptr_offset: Whether a @freeptr_offset is used.
     */
    pub use_freeptr_offset: bool,
    /**
     * @ctor: A constructor for the objects.
     *
     * The constructor is invoked for each object in a newly allocated slab
     * page. It is the cache user's responsibility to free object in the
     * same state as after calling the constructor, or deal appropriately
     * with any differences between a freshly constructed and a reallocated
     * object.
     *
     * %NULL means no constructor.
     */
    pub ctor: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub union slab_sheaf_union {
    pub barn_list: list_head,
    /* only used for prefilled sheafs */
    pub capacity: c_uint,
}

#[repr(C)]
pub struct slab_sheaf {
    pub u: slab_sheaf_union,
    pub cache: *mut kmem_cache,
    pub size: c_uint,
    pub node: c_int, /* only used for rcu_sheaf */
    pub objects: [*mut c_void; 0],
}

#[inline]
pub unsafe fn kzalloc(size: size_t, gfp: gfp_t) -> *mut c_void {
    unsafe { kmalloc(size, gfp | __GFP_ZERO) }
}

/* struct list_lru; */

unsafe extern "C" {
    pub fn kmem_cache_alloc_lru(
        cachep: *mut kmem_cache,
        arg1: *mut list_lru,
        flags: c_int,
    ) -> *mut c_void;
}

#[inline]
pub unsafe fn kmem_cache_alloc(cachep: *mut kmem_cache, flags: c_int) -> *mut c_void {
    unsafe { kmem_cache_alloc_lru(cachep, core::ptr::null_mut(), flags) }
}

unsafe extern "C" {
    pub fn kmem_cache_free(cachep: *mut kmem_cache, objp: *mut c_void);

    pub fn __kmem_cache_create_args(
        name: *const c_char,
        size: c_uint,
        args: *mut kmem_cache_args,
        flags: c_uint,
    ) -> *mut kmem_cache;
}

/* If NULL is passed for @args, use this variant with default arguments. */
#[inline]
pub unsafe fn __kmem_cache_default_args(
    name: *const c_char,
    size: c_uint,
    _args: *mut kmem_cache_args,
    flags: c_uint,
) -> *mut kmem_cache {
    let mut kmem_default_args = kmem_cache_args {
        align: 0,
        sheaf_capacity: 0,
        useroffset: 0,
        usersize: 0,
        freeptr_offset: 0,
        use_freeptr_offset: false,
        ctor: None,
    };

    unsafe { __kmem_cache_create_args(name, size, &mut kmem_default_args, flags) }
}

#[inline]
pub unsafe fn __kmem_cache_create(
    name: *const c_char,
    size: c_uint,
    align: c_uint,
    flags: c_uint,
    ctor: Option<unsafe extern "C" fn(*mut c_void)>,
) -> *mut kmem_cache {
    let mut kmem_args = kmem_cache_args {
        align,
        sheaf_capacity: 0,
        useroffset: 0,
        usersize: 0,
        freeptr_offset: 0,
        use_freeptr_offset: false,
        ctor,
    };

    unsafe { __kmem_cache_create_args(name, size, &mut kmem_args, flags) }
}

/* C generic-selection macro:
 * #define kmem_cache_create(__name, __object_size, __args, ...)
 * chooses __kmem_cache_create_args for struct kmem_cache_args *,
 * __kmem_cache_default_args for void *, and __kmem_cache_create otherwise.
 */

unsafe extern "C" {
    pub fn kmem_cache_free_bulk(cachep: *mut kmem_cache, size: size_t, list: *mut *mut c_void);
    pub fn kmem_cache_alloc_bulk(
        cachep: *mut kmem_cache,
        gfp: gfp_t,
        size: size_t,
        list: *mut *mut c_void,
    ) -> bool;
    pub fn kmem_cache_prefill_sheaf(
        s: *mut kmem_cache,
        gfp: gfp_t,
        size: c_uint,
    ) -> *mut slab_sheaf;

    pub fn kmem_cache_alloc_from_sheaf(
        s: *mut kmem_cache,
        gfp: gfp_t,
        sheaf: *mut slab_sheaf,
    ) -> *mut c_void;

    pub fn kmem_cache_return_sheaf(s: *mut kmem_cache, gfp: gfp_t, sheaf: *mut slab_sheaf);
    pub fn kmem_cache_refill_sheaf(
        s: *mut kmem_cache,
        gfp: gfp_t,
        sheafp: *mut *mut slab_sheaf,
        size: c_uint,
    ) -> c_int;
}

#[inline]
pub unsafe fn kmem_cache_sheaf_size(sheaf: *mut slab_sheaf) -> c_uint {
    unsafe { (*sheaf).size }
}

/* C statement-expression macro:
 * #define __alloc_objs(KMALLOC, GFP, TYPE, COUNT)
 * computes size_mul(sizeof(TYPE), COUNT) and calls KMALLOC.
 */
#[inline]
pub unsafe fn __alloc_objs<T>(
    kmalloc_fn: unsafe fn(size_t, gfp_t) -> *mut c_void,
    gfp: gfp_t,
    count: size_t,
) -> *mut T {
    let __obj_size: size_t = size_mul(core::mem::size_of::<T>() as size_t, count);
    unsafe { kmalloc_fn(__obj_size, gfp) as *mut T }
}

/* C variadic typeof macro:
 * #define kzalloc_obj(P, ...) __alloc_objs(kzalloc, default_gfp(__VA_ARGS__), typeof(P), 1)
 */
#[inline]
pub unsafe fn kzalloc_obj<T>(gfp: gfp_t) -> *mut T {
    unsafe { __alloc_objs::<T>(kzalloc, default_gfp(gfp), 1) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
