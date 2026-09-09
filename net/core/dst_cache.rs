// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/core/dst_cache.c - dst entry cache
 *
 * Copyright (c) 2016 Paolo Abeni <pabeni@redhat.com>
 */

// Kernel dependencies supplied by other translation units.

#[repr(C)]
pub union dst_cache_pcpu_addr {
    pub in_saddr: in_addr,
    pub in6_saddr: in6_addr,
}

#[repr(C)]
pub struct dst_cache_pcpu {
    pub refresh_ts: c_ulong,
    pub dst: *mut dst_entry,
    pub bh_lock: local_lock_t,
    pub cookie: u32,
    pub addr: dst_cache_pcpu_addr,
}

unsafe fn dst_cache_per_cpu_dst_set(
    dst_cache: *mut dst_cache_pcpu,
    dst: *mut dst_entry,
    cookie: u32,
) {
    DEBUG_NET_WARN_ON_ONCE(!in_softirq());
    dst_release((*dst_cache).dst);
    if !dst.is_null() {
        dst_hold(dst);
    }

    (*dst_cache).cookie = cookie;
    (*dst_cache).dst = dst;
}

unsafe fn dst_cache_per_cpu_get(
    dst_cache: *mut dst_cache,
    idst: *mut dst_cache_pcpu,
) -> *mut dst_entry {
    let dst: *mut dst_entry;

    DEBUG_NET_WARN_ON_ONCE(!in_softirq());
    dst = (*idst).dst;
    if dst.is_null() {
        (*idst).refresh_ts = jiffies;
        return core::ptr::null_mut();
    }

    // The cache already holds a dst reference; it cannot go away.
    dst_hold(dst);

    if unlikely(
        !time_after((*idst).refresh_ts, READ_ONCE((*dst_cache).reset_ts))
            || (READ_ONCE((*dst).obsolete) != 0
                && !(*(*dst).ops).check.unwrap()(dst, (*idst).cookie)),
    ) {
        dst_cache_per_cpu_dst_set(idst, core::ptr::null_mut(), 0);
        dst_release(dst);
        (*idst).refresh_ts = jiffies;
        return core::ptr::null_mut();
    }
    return dst;
}

pub unsafe fn dst_cache_get(dst_cache: *mut dst_cache) -> *mut dst_entry {
    let dst: *mut dst_entry;

    if (*dst_cache).cache.is_null() {
        return core::ptr::null_mut();
    }

    local_lock_nested_bh(&(*(*dst_cache).cache).bh_lock);
    dst = dst_cache_per_cpu_get(dst_cache, this_cpu_ptr((*dst_cache).cache));
    local_unlock_nested_bh(&(*(*dst_cache).cache).bh_lock);
    dst
}
EXPORT_SYMBOL_GPL!(dst_cache_get);

pub unsafe fn dst_cache_get_ip4(
    dst_cache: *mut dst_cache,
    saddr: *mut __be32,
) -> *mut rtable {
    let idst: *mut dst_cache_pcpu;
    let dst: *mut dst_entry;

    if (*dst_cache).cache.is_null() {
        return core::ptr::null_mut();
    }

    local_lock_nested_bh(&(*(*dst_cache).cache).bh_lock);
    idst = this_cpu_ptr((*dst_cache).cache);
    dst = dst_cache_per_cpu_get(dst_cache, idst);
    if dst.is_null() {
        local_unlock_nested_bh(&(*(*dst_cache).cache).bh_lock);
        return core::ptr::null_mut();
    }

    *saddr = (*idst).addr.in_saddr.s_addr;
    local_unlock_nested_bh(&(*(*dst_cache).cache).bh_lock);
    dst_rtable(dst)
}
EXPORT_SYMBOL_GPL!(dst_cache_get_ip4);

pub unsafe fn dst_cache_set_ip4(
    dst_cache: *mut dst_cache,
    dst: *mut dst_entry,
    saddr: __be32,
) {
    let idst: *mut dst_cache_pcpu;

    if (*dst_cache).cache.is_null() {
        return;
    }

    local_lock_nested_bh(&(*(*dst_cache).cache).bh_lock);
    idst = this_cpu_ptr((*dst_cache).cache);
    dst_cache_per_cpu_dst_set(idst, dst, 0);
    (*idst).addr.in_saddr.s_addr = saddr;
    local_unlock_nested_bh(&(*(*dst_cache).cache).bh_lock);
}
EXPORT_SYMBOL_GPL!(dst_cache_set_ip4);

// The following functions are compiled when CONFIG_IPV6 is enabled.
#[cfg(feature = "CONFIG_IPV6")]
pub unsafe fn dst_cache_set_ip6(
    dst_cache: *mut dst_cache,
    dst: *mut dst_entry,
    saddr: *const in6_addr,
) {
    let idst: *mut dst_cache_pcpu;

    if (*dst_cache).cache.is_null() {
        return;
    }

    local_lock_nested_bh(&(*(*dst_cache).cache).bh_lock);
    idst = this_cpu_ptr((*dst_cache).cache);
    dst_cache_per_cpu_dst_set(idst, dst, rt6_get_cookie(dst_rt6_info(dst)));
    (*idst).addr.in6_saddr = *saddr;
    local_unlock_nested_bh(&(*(*dst_cache).cache).bh_lock);
}
#[cfg(feature = "CONFIG_IPV6")]
EXPORT_SYMBOL_GPL!(dst_cache_set_ip6);

#[cfg(feature = "CONFIG_IPV6")]
pub unsafe fn dst_cache_get_ip6(
    dst_cache: *mut dst_cache,
    saddr: *mut in6_addr,
) -> *mut dst_entry {
    let idst: *mut dst_cache_pcpu;
    let dst: *mut dst_entry;

    if (*dst_cache).cache.is_null() {
        return core::ptr::null_mut();
    }

    local_lock_nested_bh(&(*(*dst_cache).cache).bh_lock);
    idst = this_cpu_ptr((*dst_cache).cache);
    dst = dst_cache_per_cpu_get(dst_cache, idst);
    if dst.is_null() {
        local_unlock_nested_bh(&(*(*dst_cache).cache).bh_lock);
        return core::ptr::null_mut();
    }

    *saddr = (*idst).addr.in6_saddr;
    local_unlock_nested_bh(&(*(*dst_cache).cache).bh_lock);
    dst
}
#[cfg(feature = "CONFIG_IPV6")]
EXPORT_SYMBOL_GPL!(dst_cache_get_ip6);

pub unsafe fn dst_cache_init(dst_cache: *mut dst_cache, gfp: gfp_t) -> c_int {
    let mut i: c_uint;

    (*dst_cache).cache = alloc_percpu_gfp!(dst_cache_pcpu, gfp | __GFP_ZERO);
    if (*dst_cache).cache.is_null() {
        return -ENOMEM;
    }
    for_each_possible_cpu!(i) {
        local_lock_init(&mut (*per_cpu_ptr((*dst_cache).cache, i)).bh_lock);
    }

    dst_cache_reset(dst_cache);
    0
}
EXPORT_SYMBOL_GPL!(dst_cache_init);

pub unsafe fn dst_cache_destroy(dst_cache: *mut dst_cache) {
    let mut i: c_int;

    if (*dst_cache).cache.is_null() {
        return;
    }

    for_each_possible_cpu!(i) {
        dst_release((*per_cpu_ptr((*dst_cache).cache, i)).dst);
    }

    free_percpu((*dst_cache).cache);
}
EXPORT_SYMBOL_GPL!(dst_cache_destroy);

pub unsafe fn dst_cache_reset_now(dst_cache: *mut dst_cache) {
    let mut i: c_int;

    if (*dst_cache).cache.is_null() {
        return;
    }

    dst_cache_reset(dst_cache);
    for_each_possible_cpu!(i) {
        let idst: *mut dst_cache_pcpu = per_cpu_ptr((*dst_cache).cache, i);
        let dst: *mut dst_entry = (*idst).dst;

        (*idst).cookie = 0;
        (*idst).dst = core::ptr::null_mut();
        dst_release(dst);
    }
}
EXPORT_SYMBOL_GPL!(dst_cache_reset_now);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
