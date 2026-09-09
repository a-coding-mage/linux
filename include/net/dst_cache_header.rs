/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by other translated headers are intentionally left external.

#[repr(C)]
pub struct dst_cache {
    pub cache: *mut dst_cache_pcpu,
    pub reset_ts: ::core::ffi::c_ulong,
}

extern "C" {
    /**
     * dst_cache_get - perform cache lookup
     * @dst_cache: the cache
     *
     * The caller should use dst_cache_get_ip4() if it need to retrieve the
     * source address to be used when xmitting to the cached dst.
     * local BH must be disabled.
     */
    pub fn dst_cache_get(dst_cache: *mut dst_cache) -> *mut dst_entry;

    /**
     * dst_cache_get_ip4 - perform cache lookup and fetch ipv4 source address
     * @dst_cache: the cache
     * @saddr: return value for the retrieved source address
     *
     * local BH must be disabled.
     */
    pub fn dst_cache_get_ip4(
        dst_cache: *mut dst_cache,
        saddr: *mut __be32,
    ) -> *mut rtable;

    /**
     * dst_cache_set_ip4 - store the ipv4 dst into the cache
     * @dst_cache: the cache
     * @dst: the entry to be cached
     * @saddr: the source address to be stored inside the cache
     *
     * local BH must be disabled.
     */
    pub fn dst_cache_set_ip4(
        dst_cache: *mut dst_cache,
        dst: *mut dst_entry,
        saddr: __be32,
    );

    // Preserved from #if IS_ENABLED(CONFIG_IPV6).
    #[cfg(feature = "CONFIG_IPV6")]
    pub fn dst_cache_set_ip6(
        dst_cache: *mut dst_cache,
        dst: *mut dst_entry,
        saddr: *const in6_addr,
    );

    // Preserved from #if IS_ENABLED(CONFIG_IPV6).
    #[cfg(feature = "CONFIG_IPV6")]
    pub fn dst_cache_get_ip6(
        dst_cache: *mut dst_cache,
        saddr: *mut in6_addr,
    ) -> *mut dst_entry;

    /**
     * dst_cache_reset - invalidate the cache contents
     * @dst_cache: the cache
     *
     * This does not free the cached dst to avoid races and contentions.
     * the dst will be freed on later cache lookup.
     */
    pub fn dst_cache_reset(dst_cache: *mut dst_cache);

    /**
     * dst_cache_reset_now - invalidate the cache contents immediately
     * @dst_cache: the cache
     *
     * The caller must be sure there are no concurrent users, as this frees
     * all dst_cache users immediately, rather than waiting for the next
     * per-cpu usage like dst_cache_reset does. Most callers should use the
     * higher speed lazily-freed dst_cache_reset function instead.
     */
    pub fn dst_cache_reset_now(dst_cache: *mut dst_cache);

    /**
     * dst_cache_init - initialize the cache, allocating the required storage
     * @dst_cache: the cache
     * @gfp: allocation flags
     */
    pub fn dst_cache_init(dst_cache: *mut dst_cache, gfp: gfp_t) -> ::core::ffi::c_int;

    /**
     * dst_cache_destroy - empty the cache and free the allocated storage
     * @dst_cache: the cache
     *
     * No synchronization is enforced: it must be called only when the cache
     * is unused.
     */
    pub fn dst_cache_destroy(dst_cache: *mut dst_cache);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
