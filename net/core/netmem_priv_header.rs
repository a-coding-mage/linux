/* SPDX-License-Identifier: GPL-2.0 */

// Translated from netmem_priv.h.  The following types, functions, constants,
// and macros are supplied by the surrounding translation unit.

#[inline]
unsafe fn netmem_get_pp_magic(netmem: netmem_ref) -> ::core::ffi::c_ulong {
    (*netmem_to_nmdesc(netmem)).pp_magic & !PP_DMA_INDEX_MASK
}

#[inline]
unsafe fn netmem_or_pp_magic(
    netmem: netmem_ref,
    pp_magic: ::core::ffi::c_ulong,
) {
    (*netmem_to_nmdesc(netmem)).pp_magic |= pp_magic;
}

#[inline]
unsafe fn netmem_clear_pp_magic(netmem: netmem_ref) {
    WARN_ON_ONCE((*netmem_to_nmdesc(netmem)).pp_magic & PP_DMA_INDEX_MASK);

    (*netmem_to_nmdesc(netmem)).pp_magic = 0;
}

#[inline]
unsafe fn netmem_is_pp(netmem: netmem_ref) -> bool {
    (netmem_get_pp_magic(netmem) & PP_MAGIC_MASK) == PP_SIGNATURE
}

#[inline]
unsafe fn netmem_set_pp(netmem: netmem_ref, pool: *mut page_pool) {
    (*netmem_to_nmdesc(netmem)).pp = pool;
}

#[inline]
unsafe fn netmem_set_dma_addr(
    netmem: netmem_ref,
    dma_addr: ::core::ffi::c_ulong,
) {
    (*netmem_to_nmdesc(netmem)).dma_addr = dma_addr;
}

#[inline]
unsafe fn netmem_get_dma_index(netmem: netmem_ref) -> ::core::ffi::c_ulong {
    let magic: ::core::ffi::c_ulong;

    if WARN_ON_ONCE(netmem_is_net_iov(netmem)) {
        return 0;
    }

    magic = (*netmem_to_nmdesc(netmem)).pp_magic;

    (magic & PP_DMA_INDEX_MASK) >> PP_DMA_INDEX_SHIFT
}

#[inline]
unsafe fn netmem_set_dma_index(
    netmem: netmem_ref,
    id: ::core::ffi::c_ulong,
) {
    let magic: ::core::ffi::c_ulong;

    if WARN_ON_ONCE(netmem_is_net_iov(netmem)) {
        return;
    }

    magic = netmem_get_pp_magic(netmem) | (id << PP_DMA_INDEX_SHIFT);
    (*netmem_to_nmdesc(netmem)).pp_magic = magic;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
