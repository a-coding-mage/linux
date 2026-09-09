/* SPDX-License-Identifier: GPL-2.0 */

// Translated from net/page_pool/memory_provider.h.
// Required types are supplied by the corresponding external dependencies.

pub struct netdev_rx_queue;
pub struct netlink_ext_ack;
pub struct sk_buff;

#[repr(C)]
pub struct memory_provider_ops {
    pub alloc_netmems: Option<unsafe extern "C" fn(pool: *mut page_pool, gfp: gfp_t) -> netmem_ref>,
    pub release_netmem:
        Option<unsafe extern "C" fn(pool: *mut page_pool, netmem: netmem_ref) -> bool>,
    pub init: Option<unsafe extern "C" fn(pool: *mut page_pool) -> i32>,
    pub destroy: Option<unsafe extern "C" fn(pool: *mut page_pool)>,
    pub nl_fill: Option<unsafe extern "C" fn(
        mp_priv: *mut core::ffi::c_void,
        rsp: *mut sk_buff,
        rxq: *mut netdev_rx_queue,
    ) -> i32>,
    pub uninstall: Option<unsafe extern "C" fn(
        mp_priv: *mut core::ffi::c_void,
        rxq: *mut netdev_rx_queue,
    )>,
}

extern "C" {
    pub fn net_mp_niov_set_dma_addr(niov: *mut net_iov, addr: dma_addr_t) -> bool;
    pub fn net_mp_niov_set_page_pool(pool: *mut page_pool, niov: *mut net_iov);
    pub fn net_mp_niov_clear_page_pool(niov: *mut net_iov);

    pub fn netif_mp_open_rxq(
        dev: *mut net_device,
        rxq_idx: u32,
        p: *const pp_memory_provider_params,
        extack: *mut netlink_ext_ack,
    ) -> i32;
    pub fn netif_mp_close_rxq(
        dev: *mut net_device,
        rxq_idx: u32,
        old_p: *const pp_memory_provider_params,
    );
}

/// net_mp_netmem_place_in_cache() - give a netmem to a page pool
/// @pool:      the page pool to place the netmem into
/// @netmem:    netmem to give
///
/// Push an accounted netmem into the page pool's allocation cache. The caller
/// must ensure that there is space in the cache. It should only be called off
/// the mp_ops->alloc_netmems() path.
#[inline]
pub unsafe fn net_mp_netmem_place_in_cache(pool: *mut page_pool, netmem: netmem_ref) {
    (*pool).alloc.cache[(*pool).alloc.count] = netmem;
    (*pool).alloc.count += 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
