/* SPDX-License-Identifier: GPL-2.0 */

// Dependency declarations from <net/page_pool/helpers.h> and "netmem_priv.h"
// are supplied by other translation units.

unsafe extern "C" {
    pub static mut page_pools_lock: mutex;

    pub fn page_pool_inflight(pool: *const page_pool, strict: bool) -> s32;

    pub fn page_pool_list(pool: *mut page_pool) -> i32;
    pub fn page_pool_detached(pool: *mut page_pool);
    pub fn page_pool_unlist(pool: *mut page_pool);

    pub fn netmem_set_dma_addr(netmem: netmem_ref, addr: dma_addr_t);
    pub fn netmem_get_dma_addr(netmem: netmem_ref) -> dma_addr_t;
    pub fn page_to_netmem(page: *mut page) -> netmem_ref;
}

#[inline]
pub unsafe fn page_pool_set_dma_addr_netmem(netmem: netmem_ref, addr: dma_addr_t) -> bool {
    if PAGE_POOL_32BIT_ARCH_WITH_64BIT_DMA {
        netmem_set_dma_addr(netmem, addr >> PAGE_SHIFT);

        /* We assume page alignment to shave off bottom bits,
         * if this "compression" doesn't work we need to drop.
         */
        return addr != (netmem_get_dma_addr(netmem) << PAGE_SHIFT);
    }

    netmem_set_dma_addr(netmem, addr);
    false
}

#[inline]
pub unsafe fn page_pool_set_dma_addr(page: *mut page, addr: dma_addr_t) -> bool {
    page_pool_set_dma_addr_netmem(page_to_netmem(page), addr)
}

// CONFIG_PAGE_POOL selects the externally provided implementations below.
#[cfg(feature = "CONFIG_PAGE_POOL")]
unsafe extern "C" {
    pub fn page_pool_set_pp_info(pool: *mut page_pool, netmem: netmem_ref);
    pub fn page_pool_clear_pp_info(netmem: netmem_ref);
    pub fn page_pool_check_memory_provider(
        dev: *mut net_device,
        rxq: *mut netdev_rx_queue,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_PAGE_POOL"))]
#[inline]
pub unsafe fn page_pool_set_pp_info(_pool: *mut page_pool, _netmem: netmem_ref) {}

#[cfg(not(feature = "CONFIG_PAGE_POOL"))]
#[inline]
pub unsafe fn page_pool_clear_pp_info(_netmem: netmem_ref) {}

#[cfg(not(feature = "CONFIG_PAGE_POOL"))]
#[inline]
pub unsafe fn page_pool_check_memory_provider(
    _dev: *mut net_device,
    _rxq: *mut netdev_rx_queue,
) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
