// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_void};
use core::mem::size_of;
use core::ptr;

pub type c_int = i32;
pub type c_uint = u32;
pub type c_ulong = u64;
pub type size_t = usize;
pub type gfp_t = c_int;

// C dependencies removed from executable Rust:
// stdlib.h, string.h, malloc.h, pthread.h, unistd.h, assert.h
// linux/gfp.h, linux/poison.h, linux/slab.h, linux/radix-tree.h,
// urcu/uatomic.h

// Values supplied by the original included headers.
pub const __GFP_DIRECT_RECLAIM: gfp_t = 0x400000;
pub const __GFP_ZERO: gfp_t = 0x8000;
pub const POISON_FREE: c_int = 0x6b;
pub const SLAB_PANIC: c_uint = 0x00040000;
pub const ENOMEM: c_int = 12;

#[repr(C)]
pub struct pthread_mutex_t {
    _private: [usize; 5],
}

#[repr(C)]
pub struct list_lru {
    _private: [u8; 0],
}

#[repr(C)]
pub struct radix_tree_node {
    pub parent: *mut radix_tree_node,
}

#[repr(C)]
pub struct kmem_cache {
    pub lock: pthread_mutex_t,
    pub size: c_uint,
    pub align: c_uint,
    pub sheaf_capacity: c_uint,
    pub nr_objs: size_t,
    pub nr_allocated: c_ulong,
    pub nr_tallocated: c_ulong,
    pub objs: *mut radix_tree_node,
    pub ctor: Option<unsafe extern "C" fn(*mut c_void)>,
    pub non_kernel: c_uint,
    pub exec_callback: bool,
    pub callback: Option<unsafe extern "C" fn(*mut c_void)>,
    pub private: *mut c_void,
}

#[repr(C)]
pub struct kmem_cache_args {
    pub align: c_uint,
    pub sheaf_capacity: c_uint,
    pub ctor: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct slab_sheaf {
    pub cache: *mut kmem_cache,
    pub capacity: c_uint,
    pub size: c_uint,
    pub objects: [*mut c_void; 0],
}

unsafe extern "C" {
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn posix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;
    fn pthread_mutex_init(
        mutex: *mut pthread_mutex_t,
        attr: *const c_void,
    ) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn pr_debug(format: *const c_char, ...) -> c_int;
    fn kmem_cache_create(
        name: *const c_char,
        size: c_uint,
        align: c_uint,
        flags: c_uint,
        ctor: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> *mut kmem_cache;
}

#[unsafe(no_mangle)]
pub static mut nr_allocated: c_int = 0;
#[unsafe(no_mangle)]
pub static mut preempt_count: c_int = 0;
#[unsafe(no_mangle)]
pub static mut test_verbose: c_int = 0;

// Supplied by linux/slab.h in the original test environment.
unsafe extern "C" {
    static mut kmalloc_verbose: bool;
}

unsafe fn uatomic_inc(v: *mut c_ulong) {
    *v = (*v).wrapping_add(1);
}

unsafe fn uatomic_dec(v: *mut c_ulong) {
    *v = (*v).wrapping_sub(1);
}

unsafe fn uatomic_inc_int(v: *mut c_int) {
    *v = (*v).wrapping_add(1);
}

unsafe fn uatomic_dec_int(v: *mut c_int) {
    *v = (*v).wrapping_sub(1);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_set_callback(
    cachep: *mut kmem_cache,
    callback: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    (*cachep).callback = callback;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_set_private(cachep: *mut kmem_cache, private: *mut c_void) {
    (*cachep).private = private;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_set_non_kernel(cachep: *mut kmem_cache, val: c_uint) {
    (*cachep).non_kernel = val;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_get_alloc(cachep: *mut kmem_cache) -> c_ulong {
    ((*cachep).size as c_ulong).wrapping_mul((*cachep).nr_allocated)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_nr_allocated(cachep: *mut kmem_cache) -> c_ulong {
    (*cachep).nr_allocated
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_nr_tallocated(cachep: *mut kmem_cache) -> c_ulong {
    (*cachep).nr_tallocated
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_zero_nr_tallocated(cachep: *mut kmem_cache) {
    (*cachep).nr_tallocated = 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_alloc_lru(
    cachep: *mut kmem_cache,
    _lru: *mut list_lru,
    gfp: c_int,
) -> *mut c_void {
    let mut p: *mut c_void = ptr::null_mut();

    if (*cachep).exec_callback {
        if let Some(callback) = (*cachep).callback {
            callback((*cachep).private);
        }
        (*cachep).exec_callback = false;
    }

    if (gfp & __GFP_DIRECT_RECLAIM) == 0 {
        if (*cachep).non_kernel == 0 {
            if (*cachep).callback.is_some() {
                (*cachep).exec_callback = true;
            }
            return ptr::null_mut();
        }

        (*cachep).non_kernel = (*cachep).non_kernel.wrapping_sub(1);
    }

    pthread_mutex_lock(&mut (*cachep).lock);
    if (*cachep).nr_objs != 0 {
        let node: *mut radix_tree_node = (*cachep).objs;
        (*cachep).nr_objs = (*cachep).nr_objs.wrapping_sub(1);
        (*cachep).objs = (*node).parent;
        pthread_mutex_unlock(&mut (*cachep).lock);
        (*node).parent = ptr::null_mut();
        p = node as *mut c_void;
    } else {
        pthread_mutex_unlock(&mut (*cachep).lock);
        if (*cachep).align != 0 {
            if posix_memalign(&mut p, (*cachep).align as size_t, (*cachep).size as size_t) < 0 {
                return ptr::null_mut();
            }
        } else {
            p = malloc((*cachep).size as size_t);
        }

        if let Some(ctor) = (*cachep).ctor {
            ctor(p);
        } else if (gfp & __GFP_ZERO) != 0 {
            memset(p, 0, (*cachep).size as size_t);
        }
    }

    uatomic_inc(&mut (*cachep).nr_allocated);
    uatomic_inc_int(&raw mut nr_allocated);
    uatomic_inc(&mut (*cachep).nr_tallocated);
    if kmalloc_verbose {
        printf(c"Allocating %p from slab\n".as_ptr(), p);
    }
    p
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __kmem_cache_free_locked(cachep: *mut kmem_cache, objp: *mut c_void) {
    assert!(!objp.is_null());
    if (*cachep).nr_objs > 10 || (*cachep).align != 0 {
        memset(objp, POISON_FREE, (*cachep).size as size_t);
        free(objp);
    } else {
        let node: *mut radix_tree_node = objp as *mut radix_tree_node;
        (*cachep).nr_objs = (*cachep).nr_objs.wrapping_add(1);
        (*node).parent = (*cachep).objs;
        (*cachep).objs = node;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_free_locked(cachep: *mut kmem_cache, objp: *mut c_void) {
    uatomic_dec_int(&raw mut nr_allocated);
    uatomic_dec(&mut (*cachep).nr_allocated);
    if kmalloc_verbose {
        printf(c"Freeing %p to slab\n".as_ptr(), objp);
    }
    __kmem_cache_free_locked(cachep, objp);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_free(cachep: *mut kmem_cache, objp: *mut c_void) {
    pthread_mutex_lock(&mut (*cachep).lock);
    kmem_cache_free_locked(cachep, objp);
    pthread_mutex_unlock(&mut (*cachep).lock);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_free_bulk(
    cachep: *mut kmem_cache,
    size: size_t,
    list: *mut *mut c_void,
) {
    if kmalloc_verbose {
        pr_debug(c"Bulk free %p[0-%zu]\n".as_ptr(), list, size.wrapping_sub(1));
    }

    if (*cachep).exec_callback {
        if let Some(callback) = (*cachep).callback {
            callback((*cachep).private);
        }
        (*cachep).exec_callback = false;
    }

    pthread_mutex_lock(&mut (*cachep).lock);
    let mut i: c_int = 0;
    while (i as size_t) < size {
        kmem_cache_free_locked(cachep, *list.add(i as size_t));
        i += 1;
    }
    pthread_mutex_unlock(&mut (*cachep).lock);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_shrink(_cachep: *mut kmem_cache) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_alloc_bulk(
    cachep: *mut kmem_cache,
    gfp: gfp_t,
    mut size: size_t,
    p: *mut *mut c_void,
) -> bool {
    let mut i: size_t;

    if kmalloc_verbose {
        pr_debug(c"Bulk alloc %zu\n".as_ptr(), size);
    }

    pthread_mutex_lock(&mut (*cachep).lock);
    if (*cachep).nr_objs >= size {
        let mut node: *mut radix_tree_node;

        i = 0;
        while i < size {
            if (gfp & __GFP_DIRECT_RECLAIM) == 0 {
                if (*cachep).non_kernel == 0 {
                    break;
                }
                (*cachep).non_kernel = (*cachep).non_kernel.wrapping_sub(1);
            }

            node = (*cachep).objs;
            (*cachep).nr_objs = (*cachep).nr_objs.wrapping_sub(1);
            (*cachep).objs = (*node).parent;
            *p.add(i) = node as *mut c_void;
            (*node).parent = ptr::null_mut();
            i += 1;
        }
        pthread_mutex_unlock(&mut (*cachep).lock);
    } else {
        pthread_mutex_unlock(&mut (*cachep).lock);
        i = 0;
        while i < size {
            if (gfp & __GFP_DIRECT_RECLAIM) == 0 {
                if (*cachep).non_kernel == 0 {
                    break;
                }
                (*cachep).non_kernel = (*cachep).non_kernel.wrapping_sub(1);
            }

            if (*cachep).align != 0 {
                if posix_memalign(
                    p.add(i),
                    (*cachep).align as size_t,
                    (*cachep).size as size_t,
                ) < 0
                {
                    break;
                }
            } else {
                *p.add(i) = malloc((*cachep).size as size_t);
                if (*p.add(i)).is_null() {
                    break;
                }
            }
            if let Some(ctor) = (*cachep).ctor {
                ctor(*p.add(i));
            } else if (gfp & __GFP_ZERO) != 0 {
                memset(*p.add(i), 0, (*cachep).size as size_t);
            }
            i += 1;
        }
    }

    if i < size {
        size = i;
        pthread_mutex_lock(&mut (*cachep).lock);
        i = 0;
        while i < size {
            __kmem_cache_free_locked(cachep, *p.add(i));
            i += 1;
        }
        pthread_mutex_unlock(&mut (*cachep).lock);
        if (*cachep).callback.is_some() {
            (*cachep).exec_callback = true;
        }
        return false;
    }

    i = 0;
    while i < size {
        uatomic_inc_int(&raw mut nr_allocated);
        uatomic_inc(&mut (*cachep).nr_allocated);
        uatomic_inc(&mut (*cachep).nr_tallocated);
        if kmalloc_verbose {
            printf(c"Allocating %p from slab\n".as_ptr(), *p.add(i));
        }
        i += 1;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __kmem_cache_create_args(
    _name: *const c_char,
    size: c_uint,
    args: *mut kmem_cache_args,
    _flags: c_uint,
) -> *mut kmem_cache {
    let ret: *mut kmem_cache = malloc(size_of::<kmem_cache>()) as *mut kmem_cache;

    pthread_mutex_init(&mut (*ret).lock, ptr::null());
    (*ret).size = size;
    (*ret).align = (*args).align;
    (*ret).sheaf_capacity = (*args).sheaf_capacity;
    (*ret).nr_objs = 0;
    (*ret).nr_allocated = 0;
    (*ret).nr_tallocated = 0;
    (*ret).objs = ptr::null_mut();
    (*ret).ctor = (*args).ctor;
    (*ret).non_kernel = 0;
    (*ret).exec_callback = false;
    (*ret).callback = None;
    (*ret).private = ptr::null_mut();

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_prefill_sheaf(
    s: *mut kmem_cache,
    gfp: gfp_t,
    size: c_uint,
) -> *mut slab_sheaf {
    let mut sheaf: *mut slab_sheaf;
    let capacity: c_uint;

    if (*s).exec_callback {
        if let Some(callback) = (*s).callback {
            callback((*s).private);
        }
        (*s).exec_callback = false;
    }

    capacity = if size > (*s).sheaf_capacity {
        size
    } else {
        (*s).sheaf_capacity
    };

    sheaf = calloc(
        1,
        size_of::<slab_sheaf>().wrapping_add(size_of::<*mut c_void>().wrapping_mul(capacity as usize)),
    ) as *mut slab_sheaf;
    if sheaf.is_null() {
        return ptr::null_mut();
    }

    (*sheaf).cache = s;
    (*sheaf).capacity = capacity;
    (*sheaf).size = size;
    if !kmem_cache_alloc_bulk(s, gfp, size as size_t, (*sheaf).objects.as_mut_ptr()) {
        free(sheaf as *mut c_void);
        return ptr::null_mut();
    }

    sheaf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_refill_sheaf(
    s: *mut kmem_cache,
    gfp: gfp_t,
    sheafp: *mut *mut slab_sheaf,
    size: c_uint,
) -> c_int {
    let mut sheaf: *mut slab_sheaf = *sheafp;

    if (*sheaf).size >= size {
        return 0;
    }

    if size > (*sheaf).capacity {
        sheaf = kmem_cache_prefill_sheaf(s, gfp, size);
        if sheaf.is_null() {
            return -ENOMEM;
        }

        kmem_cache_return_sheaf(s, gfp, *sheafp);
        *sheafp = sheaf;
        return 0;
    }

    if !kmem_cache_alloc_bulk(
        s,
        gfp,
        size.wrapping_sub((*sheaf).size) as size_t,
        (*sheaf).objects.as_mut_ptr().add((*sheaf).size as usize),
    ) {
        return -ENOMEM;
    }
    (*sheaf).size = size;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_return_sheaf(
    s: *mut kmem_cache,
    _gfp: gfp_t,
    sheaf: *mut slab_sheaf,
) {
    if (*sheaf).size != 0 {
        kmem_cache_free_bulk(s, (*sheaf).size as size_t, (*sheaf).objects.as_mut_ptr());
    }

    free(sheaf as *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmem_cache_alloc_from_sheaf(
    _s: *mut kmem_cache,
    _gfp: gfp_t,
    sheaf: *mut slab_sheaf,
) -> *mut c_void {
    let obj: *mut c_void;

    if (*sheaf).size == 0 {
        printf(c"Nothing left in sheaf!\n".as_ptr());
        return ptr::null_mut();
    }

    (*sheaf).size = (*sheaf).size.wrapping_sub(1);
    obj = *(*sheaf).objects.as_mut_ptr().add((*sheaf).size as usize);
    *(*sheaf).objects.as_mut_ptr().add((*sheaf).size as usize) = ptr::null_mut();

    obj
}

/*
 * Test the test infrastructure for kem_cache_alloc/free and bulk counterparts.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_kmem_cache_bulk() {
    let mut i: c_int;
    let mut list: [*mut c_void; 12] = [ptr::null_mut(); 12];
    static mut test_cache: *mut kmem_cache = ptr::null_mut();
    static mut test_cache2: *mut kmem_cache = ptr::null_mut();

    /*
     * Testing the bulk allocators without aligned kmem_cache to force the
     * bulk alloc/free to reuse
     */
    test_cache = kmem_cache_create(c"test_cache".as_ptr(), 256, 0, SLAB_PANIC, None);

    i = 0;
    while i < 5 {
        list[i as usize] = kmem_cache_alloc_lru(test_cache, ptr::null_mut(), __GFP_DIRECT_RECLAIM);
        i += 1;
    }

    i = 0;
    while i < 5 {
        kmem_cache_free(test_cache, list[i as usize]);
        i += 1;
    }
    assert!((*test_cache).nr_objs == 5);

    kmem_cache_alloc_bulk(test_cache, __GFP_DIRECT_RECLAIM, 5, list.as_mut_ptr());
    kmem_cache_free_bulk(test_cache, 5, list.as_mut_ptr());

    i = 0;
    while i < 12 {
        list[i as usize] = kmem_cache_alloc_lru(test_cache, ptr::null_mut(), __GFP_DIRECT_RECLAIM);
        i += 1;
    }

    i = 0;
    while i < 12 {
        kmem_cache_free(test_cache, list[i as usize]);
        i += 1;
    }

    /* The last free will not be kept around */
    assert!((*test_cache).nr_objs == 11);

    /* Aligned caches will immediately free */
    test_cache2 = kmem_cache_create(c"test_cache2".as_ptr(), 128, 128, SLAB_PANIC, None);

    kmem_cache_alloc_bulk(test_cache2, __GFP_DIRECT_RECLAIM, 10, list.as_mut_ptr());
    kmem_cache_free_bulk(test_cache2, 10, list.as_mut_ptr());
    assert!((*test_cache2).nr_objs == 0);
}
