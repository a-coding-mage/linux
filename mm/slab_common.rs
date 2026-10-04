// SPDX-License-Identifier: GPL-2.0
// Historical initial transcription: d482bb509b7d065808de40ce78b5bca39f40b783
// SOURCE-COMMIT: e1d84f501551943a11f4c5271e9f5c85d7e15168
// Historical marker retained. Full reconciliation authority: original
// mm/slab_common.c at e1d84f501551943a11f4c5271e9f5c85d7e15168,
// SHA256 c9d139bf5894a36f2f369d3d7d27a5785c847dc305522343618d50453b41c38a.
//! Allocator-independent slab ownership and deferred RCU reclamation.
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/slab_common_generated.rs"
    ));
}
use bindings::*;
use core::cmp::{max, min};
use core::mem::{align_of, offset_of, size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null, null_mut};
use kernel::ffi::{
    c_char as CChar, c_int as Int, c_long as Long, c_uint as UInt, c_ulong as ULong, c_void as Void,
};
type rcu_head = callback_head;
const _: () = {
    assert!(size_of::<bindings::kmem_cache>() == RSC_SIZEOF_CACHE as usize);
    assert!(align_of::<bindings::kmem_cache>() == RSC_ALIGNOF_CACHE as usize);
    assert!(size_of::<kmem_cache_args>() == RSC_SIZEOF_ARGS as usize);
    assert!(size_of::<kmalloc_info_struct>() == RSC_SIZEOF_KMALLOC_INFO as usize);
};

// The printk macro's disabled branch must not evaluate any arguments.
macro_rules! sc_log {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        #[cfg(CONFIG_PRINTK)]
        { _printk(concat!($fmt, "\0").as_ptr().cast() $(, $arg)*); }
    }};
}
#[inline]
fn align_uint(value: UInt, align: UInt) -> UInt {
    value.wrapping_add(align.wrapping_sub(1)) & !align.wrapping_sub(1)
}
#[inline]
unsafe fn cache_from_list(p: *mut list_head) -> *mut kmem_cache {
    p.byte_sub(offset_of!(kmem_cache, list)).cast()
}

#[no_mangle]
pub static mut slab_state: bindings::slab_state = DOWN;
#[no_mangle]
pub static mut kmem_cache: *mut bindings::kmem_cache = null_mut();
const SLAB_NEVER_MERGE: slab_flags_t = RSC_SLAB_DEBUG_FLAGS
    | RSC_SLAB_TYPESAFE_BY_RCU
    | RSC_SLAB_NOLEAKTRACE
    | RSC_SLAB_FAILSLAB
    | RSC_SLAB_NO_MERGE
    | RSC_SLAB_OBJ_EXT_IN_OBJ;
const SLAB_MERGE_SAME: slab_flags_t = RSC_SLAB_RECLAIM_ACCOUNT
    | RSC_SLAB_CACHE_DMA
    | RSC_SLAB_CACHE_DMA32
    | RSC_SLAB_ACCOUNT
    | RSC_SLAB_MAY_ACCOUNT;
static mut slab_nomerge: bool = !cfg!(CONFIG_SLAB_MERGE_DEFAULT);
#[no_mangle]
#[link_section = ".init.text"]
#[cfg_attr(RSC_INIT_COLD, cold)]
pub unsafe extern "C" fn setup_slab_nomerge(_str: *mut CChar) -> Int {
    slab_nomerge = true;
    1
}
#[no_mangle]
#[link_section = ".init.text"]
#[cfg_attr(RSC_INIT_COLD, cold)]
pub unsafe extern "C" fn setup_slab_merge(_str: *mut CChar) -> Int {
    slab_nomerge = false;
    1
}
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_size(s: *mut bindings::kmem_cache) -> UInt {
    (*s).object_size
}
#[cfg(CONFIG_DEBUG_VM)]
unsafe fn kmem_cache_is_duplicate_name(name: *const CChar) -> bool {
    let head = addr_of_mut!(slab_caches);
    let mut p = (*head).next;
    while p != head {
        let s = cache_from_list(p);
        if strcmp((*s).name, name) == 0 {
            return true;
        }
        p = (*p).next;
    }
    false
}
#[cfg(CONFIG_DEBUG_VM)]
unsafe fn kmem_cache_sanity_check(name: *const CChar, size: UInt) -> Int {
    if name.is_null()
        || rust_slab_common_in_interrupt()
        || size as usize > RSC_KMALLOC_MAX_SIZE as usize
    {
        sc_log!("\x013kmem_cache_create(%s) integrity check failed\n", name);
        return -(RSC_EINVAL as Int);
    }
    rust_slab_common_warn_duplicate(kmem_cache_is_duplicate_name(name), name);
    rust_slab_common_warn_space(!strchr(name, b' ' as Int).is_null());
    0
}
#[cfg(not(CONFIG_DEBUG_VM))]
unsafe fn kmem_cache_sanity_check(_name: *const CChar, _size: UInt) -> Int {
    0
}
unsafe fn calculate_alignment(flags: slab_flags_t, mut align: UInt, size: UInt) -> UInt {
    if flags & RSC_SLAB_HWCACHE_ALIGN != 0 {
        let mut ralign = rust_slab_common_cache_line_size();
        while size <= ralign / 2 {
            ralign /= 2;
        }
        align = max(align, ralign);
    }
    align = max(align, rust_slab_common_arch_slab_minalign());
    align_uint(align, size_of::<*mut Void>() as UInt)
}
#[no_mangle]
pub unsafe extern "C" fn slab_unmergeable(s: *mut bindings::kmem_cache) -> Int {
    if slab_nomerge || (*s).flags & SLAB_NEVER_MERGE != 0 {
        return 1;
    }
    if (*s).ctor.is_some() {
        return 1;
    }
    #[cfg(CONFIG_HARDENED_USERCOPY)]
    if (*s).usersize != 0 {
        return 1;
    }
    if (*s).refcount < 0 {
        return 1;
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn slab_args_unmergeable(
    args: *mut kmem_cache_args,
    flags: slab_flags_t,
) -> bool {
    slab_nomerge
        || (*args).ctor.is_some()
        || (cfg!(CONFIG_HARDENED_USERCOPY) && (*args).usersize != 0)
        || flags & SLAB_NEVER_MERGE != 0
}
unsafe fn find_mergeable(
    mut size: UInt,
    mut flags: slab_flags_t,
    name: *const CChar,
    args: *mut kmem_cache_args,
) -> *mut bindings::kmem_cache {
    flags = kmem_cache_flags(flags, name);
    if slab_args_unmergeable(args, flags) {
        return null_mut();
    }
    size = align_uint(size, size_of::<*mut Void>() as UInt);
    let align = calculate_alignment(flags, (*args).align, size);
    size = align_uint(size, align);
    let head = addr_of_mut!(slab_caches);
    let mut p = (*head).prev;
    while p != head {
        let s = cache_from_list(p);
        p = (*p).prev;
        if slab_unmergeable(s) != 0 || size > (*s).size {
            continue;
        }
        if (flags & SLAB_MERGE_SAME) != ((*s).flags & SLAB_MERGE_SAME) {
            continue;
        }
        if ((*s).size & !align.wrapping_sub(1)) != (*s).size {
            continue;
        }
        if (*s).size.wrapping_sub(size) >= size_of::<*mut Void>() as UInt {
            continue;
        }
        return s;
    }
    null_mut()
}
unsafe fn create_cache(
    name: *const CChar,
    object_size: UInt,
    args: *mut kmem_cache_args,
    flags: slab_flags_t,
) -> *mut bindings::kmem_cache {
    if (*args).use_freeptr_offset
        && ((*args).freeptr_offset >= object_size
            || (flags & RSC_SLAB_TYPESAFE_BY_RCU == 0 && (*args).ctor.is_none())
            || (*args).freeptr_offset as usize & (align_of::<freeptr_t>() - 1) != 0)
    {
        return (-(RSC_EINVAL as Long)) as *mut bindings::kmem_cache;
    }
    let s = rust_slab_common_create_cache_zalloc(kmem_cache, RSC_GFP_KERNEL)
        .cast::<bindings::kmem_cache>();
    if s.is_null() {
        return (-(RSC_ENOMEM as Long)) as *mut bindings::kmem_cache;
    }
    let err = do_kmem_cache_create(s, name, object_size, args, flags);
    if err != 0 {
        kmem_cache_free(kmem_cache, s.cast());
        return (err as Long) as *mut bindings::kmem_cache;
    }
    (*s).refcount = 1;
    rust_slab_common_list_add(addr_of_mut!((*s).list), addr_of_mut!(slab_caches));
    s
}
unsafe fn __kmem_cache_alias(
    name: *const CChar,
    size: UInt,
    flags: slab_flags_t,
    args: *mut kmem_cache_args,
) -> *mut bindings::kmem_cache {
    let s = find_mergeable(size, flags, name, args);
    if !s.is_null() {
        if rust_slab_common_sysfs_slab_alias(s, name) != 0 {
            sc_log!("\x013SLUB: Unable to add cache alias %s to sysfs\n", name);
        }
        (*s).refcount = (*s).refcount.wrapping_add(1);
        (*s).object_size = max((*s).object_size, size);
        (*s).inuse = max((*s).inuse, align_uint(size, size_of::<*mut Void>() as UInt));
    }
    s
}
#[no_mangle]
pub unsafe extern "C" fn __kmem_cache_create_args(
    name: *const CChar,
    object_size: UInt,
    args: *mut kmem_cache_args,
    mut flags: slab_flags_t,
) -> *mut bindings::kmem_cache {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        if flags & RSC_SLAB_DEBUG_FLAGS != 0 {
            rust_slab_common_enable_slub_debug();
        }
        if flags & RSC_SLAB_STORE_USER != 0 {
            rust_slab_common_stack_depot_init();
        }
    }
    #[cfg(not(CONFIG_SLUB_DEBUG))]
    {
        flags &= !RSC_SLAB_DEBUG_FLAGS;
    }
    if (*args).sheaf_capacity != 0 {
        flags |= RSC_SLAB_NO_MERGE;
    }
    rust_slab_common_mutex_lock(addr_of_mut!(slab_mutex));
    let mut err = kmem_cache_sanity_check(name, object_size);
    let mut s = null_mut();
    if err == 0 && flags & !RSC_SLAB_FLAGS_PERMITTED != 0 {
        err = -(RSC_EINVAL as Int);
    }
    if err == 0 {
        if !rust_slab_common_mem_cgroup_kmem_disabled() {
            flags |= RSC_SLAB_MAY_ACCOUNT;
        }
        if !cfg!(CONFIG_HARDENED_USERCOPY)
            || rust_slab_common_warn_useroffset((*args).usersize == 0 && (*args).useroffset != 0)
            || rust_slab_common_warn_usersize(
                object_size < (*args).usersize
                    || object_size.wrapping_sub((*args).usersize) < (*args).useroffset,
            )
        {
            (*args).usersize = 0;
            (*args).useroffset = 0;
        }
        s = __kmem_cache_alias(name, object_size, flags, args);
        if s.is_null() {
            let cache_name = kstrdup_const(name, RSC_GFP_KERNEL);
            if cache_name.is_null() {
                err = -(RSC_ENOMEM as Int);
            } else {
                (*args).align = calculate_alignment(flags, (*args).align, object_size);
                s = create_cache(cache_name, object_size, args, flags);
                if rust_slab_common_is_err(s.cast()) {
                    err = s as Long as Int;
                    kfree_const(cache_name.cast());
                }
            }
        }
    }
    rust_slab_common_mutex_unlock(addr_of_mut!(slab_mutex));
    if err != 0 {
        if flags & RSC_SLAB_PANIC != 0 {
            panic(
                c"%s: Failed to create slab '%s'. Error %d\n".as_ptr(),
                c"__kmem_cache_create_args".as_ptr(),
                name,
                err,
            );
        } else {
            sc_log!(
                "\x014%s(%s) failed with error %d\n",
                c"__kmem_cache_create_args".as_ptr(),
                name,
                err
            );
            rust_slab_common_dump_stack();
        }
        return null_mut();
    }
    s
}
#[link_section = ".data..ro_after_init"]
static mut kmem_buckets_cache: *mut bindings::kmem_cache = null_mut();
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut kmalloc_caches: [kmem_buckets; NR_KMALLOC_TYPES as usize] =
    [[null_mut(); RSC_KMALLOC_SHIFT_HIGH as usize + 1]; NR_KMALLOC_TYPES as usize];
#[cfg(CONFIG_KMALLOC_PARTITION_RANDOM)]
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut random_kmalloc_seed: ULong = 0;

#[no_mangle]
pub unsafe extern "C" fn kmem_buckets_create(
    name: *const CChar,
    mut flags: slab_flags_t,
    useroffset: UInt,
    usersize: UInt,
    ctor: Option<unsafe extern "C" fn(*mut Void)>,
) -> *mut kmem_buckets {
    const COUNT: usize = RSC_KMALLOC_SHIFT_HIGH as usize + 1;
    const {
        assert!(COUNT <= RSC_BITS_PER_LONG as usize);
    }
    if !cfg!(CONFIG_SLAB_BUCKETS) {
        return rust_slab_common_zero_size_ptr().cast();
    }
    if rust_slab_common_warn_buckets_cache(kmem_buckets_cache.is_null()) {
        return null_mut();
    }
    let b = rust_slab_common_cache_alloc(kmem_buckets_cache, RSC_GFP_KERNEL | RSC___GFP_ZERO)
        .cast::<kmem_buckets>();
    if rust_slab_common_warn_buckets_alloc(b.is_null()) {
        return null_mut();
    }
    flags |= RSC_SLAB_NO_MERGE;
    let mut mask: ULong = 0;
    let mut idx = 0usize;
    while idx < COUNT {
        let cache = kmalloc_caches[KMALLOC_NORMAL as usize][idx];
        if cache.is_null() || (*cache).object_size == 0 {
            idx += 1;
            continue;
        }
        let size = (*cache).object_size;
        let short_size = strchr((*cache).name, b'-' as Int);
        if rust_slab_common_warn_short_size(short_size.is_null()) {
            break;
        }
        let (cache_useroffset, cache_usersize) = if useroffset >= size {
            (0, 0)
        } else {
            (useroffset, min(size.wrapping_sub(useroffset), usersize))
        };
        let aligned_idx = rust_slab_common_kmalloc_index(size as usize) as usize;
        if (*b)[aligned_idx].is_null() {
            let cache_name = kasprintf(RSC_GFP_KERNEL, c"%s-%s".as_ptr(), name, short_size.add(1));
            if rust_slab_common_warn_cache_name(cache_name.is_null()) {
                break;
            }
            // kmem_cache_create_usercopy is a header wrapper around the owned body.
            let mut args: kmem_cache_args = zeroed();
            args.useroffset = cache_useroffset;
            args.usersize = cache_usersize;
            args.ctor = ctor;
            (*b)[aligned_idx] =
                __kmem_cache_create_args(cache_name, size, addr_of_mut!(args), flags);
            kfree(cache_name.cast());
            if rust_slab_common_warn_bucket_create((*b)[aligned_idx].is_null()) {
                break;
            }
            mask |= (1 as ULong) << aligned_idx;
        }
        if idx != aligned_idx {
            (*b)[idx] = (*b)[aligned_idx];
        }
        idx += 1;
    }
    if idx == COUNT {
        return b;
    }
    for bit in 0..COUNT {
        if mask & ((1 as ULong) << bit) != 0 {
            bindings::kmem_cache_destroy((*b)[bit]);
        }
    }
    kmem_cache_free(kmem_buckets_cache, b.cast());
    null_mut()
}
unsafe fn kmem_cache_release(s: *mut bindings::kmem_cache) {
    rust_slab_common_kfence_shutdown_cache(s);
    if RSC_SLAB_SUPPORTS_SYSFS != 0 && slab_state >= FULL {
        rust_slab_common_sysfs_slab_release(s);
    } else {
        slab_kmem_cache_release(s);
    }
}
#[no_mangle]
pub unsafe extern "C" fn slab_kmem_cache_release(s: *mut bindings::kmem_cache) {
    __kmem_cache_release(s);
    kfree_const((*s).name.cast());
    kmem_cache_free(kmem_cache, s.cast());
}
// The public native ABI trampoline captures _RET_IP_ at the original call boundary.
#[no_mangle]
pub unsafe extern "C" fn rust_slab_common_destroy(s: *mut bindings::kmem_cache, caller: *mut Void) {
    if s.is_null() || !rust_slab_common_kasan_check_byte(s.cast()) {
        return;
    }
    kvfree_rcu_barrier_on_cache(s);
    if cfg!(CONFIG_SLUB_RCU_DEBUG) && (*s).flags & RSC_SLAB_TYPESAFE_BY_RCU != 0 {
        rcu_barrier();
    }
    deferred_work_barrier();
    rust_slab_common_rcu_cpus_read_lock();
    rust_slab_common_mutex_lock(addr_of_mut!(slab_mutex));
    (*s).refcount = (*s).refcount.wrapping_sub(1);
    if (*s).refcount != 0 {
        rust_slab_common_mutex_unlock(addr_of_mut!(slab_mutex));
        rust_slab_common_rcu_cpus_read_unlock();
        return;
    }
    rust_slab_common_kasan_cache_shutdown(s);
    let err = __kmem_cache_shutdown(s);
    if !rust_slab_common_slab_in_kunit_test() {
        rust_slab_common_warn_destroy(err != 0, s, caller);
    }
    rust_slab_common_list_del(addr_of_mut!((*s).list));
    rust_slab_common_mutex_unlock(addr_of_mut!(slab_mutex));
    rust_slab_common_rcu_cpus_read_unlock();
    if slab_state >= FULL {
        rust_slab_common_sysfs_slab_unlink(s);
    }
    rust_slab_common_debugfs_slab_release(s);
    if err != 0 {
        return;
    }
    if (*s).flags & RSC_SLAB_TYPESAFE_BY_RCU != 0 {
        rcu_barrier();
    }
    kmem_cache_release(s);
}
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_shrink(cachep: *mut bindings::kmem_cache) -> Int {
    rust_slab_common_kasan_cache_shrink(cachep);
    __kmem_cache_shrink(cachep)
}
#[no_mangle]
pub unsafe extern "C" fn slab_is_available() -> bool {
    slab_state >= UP
}

#[cfg(CONFIG_PRINTK)]
unsafe fn kmem_obj_info(
    kpp: *mut bindings::kmem_obj_info,
    object: *mut Void,
    slab: *mut bindings::slab,
) {
    if rust_slab_common_kfence_obj_info(kpp, object, slab) {
        return;
    }
    __kmem_obj_info(kpp, object, slab);
}
#[cfg(CONFIG_PRINTK)]
#[no_mangle]
pub unsafe extern "C" fn kmem_dump_obj(object: *mut Void) -> bool {
    if (object as usize) < RSC_PAGE_SIZE as usize || !rust_slab_common_virt_addr_valid(object) {
        return false;
    }
    let slab = rust_slab_common_virt_to_slab(object);
    if slab.is_null() {
        return false;
    }
    let mut kp: bindings::kmem_obj_info = zeroed();
    kmem_obj_info(addr_of_mut!(kp), object, slab);
    let cp = if cfg!(CONFIG_MMU) {
        c"".as_ptr()
    } else {
        c"/vmalloc".as_ptr()
    };
    if !kp.kp_slab_cache.is_null() {
        sc_log!("\x01c slab%s %s", cp, (*kp.kp_slab_cache).name);
    } else {
        sc_log!("\x01c slab%s", cp);
    }
    if rust_slab_common_is_kfence_address(object) {
        sc_log!("\x01c (kfence)");
    }
    if !kp.kp_objp.is_null() {
        sc_log!("\x01c start %px", kp.kp_objp);
    }
    if kp.kp_data_offset != 0 {
        sc_log!("\x01c data offset %lu", kp.kp_data_offset);
    }
    if !kp.kp_objp.is_null() {
        let ptroffset = (object as ULong)
            .wrapping_sub(kp.kp_objp as ULong)
            .wrapping_sub(kp.kp_data_offset);
        sc_log!("\x01c pointer offset %lu", ptroffset);
    }
    if !kp.kp_slab_cache.is_null() && (*kp.kp_slab_cache).object_size != 0 {
        sc_log!("\x01c size %u", (*kp.kp_slab_cache).object_size);
    }
    if !kp.kp_ret.is_null() {
        sc_log!("\x01c allocated at %pS\n", kp.kp_ret);
    } else {
        sc_log!("\x01c\n");
    }
    for p in kp.kp_stack {
        if p.is_null() {
            break;
        }
        sc_log!("\x016    %pS\n", p);
    }
    if !kp.kp_free_stack[0].is_null() {
        sc_log!("\x01c Free path:\n");
    }
    for p in kp.kp_free_stack {
        if p.is_null() {
            break;
        }
        sc_log!("\x016    %pS\n", p);
    }
    true
}
include!("slab_common_kmalloc.rs");
include!("slab_common_info.rs");
include!("slab_common_rcu.rs");
