/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Dmabuf device memory provider.
 *
 * Authors: Mina Almasry <almasrymina@google.com>
 *
 */

// Dependency supplied by the surrounding kernel translation: `page_pool`,
// `gfp_t`, `netmem_ref`, and `EOPNOTSUPP`.

#[cfg(feature = "CONFIG_NET_DEVMEM")]
unsafe extern "C" {
    pub fn mp_dmabuf_devmem_init(pool: *mut page_pool) -> core::ffi::c_int;

    pub fn mp_dmabuf_devmem_alloc_netmems(
        pool: *mut page_pool,
        gfp: gfp_t,
    ) -> netmem_ref;

    pub fn mp_dmabuf_devmem_destroy(pool: *mut page_pool);

    pub fn mp_dmabuf_devmem_release_page(
        pool: *mut page_pool,
        netmem: netmem_ref,
    ) -> bool;
}

#[cfg(not(feature = "CONFIG_NET_DEVMEM"))]
#[inline]
pub unsafe fn mp_dmabuf_devmem_init(_pool: *mut page_pool) -> core::ffi::c_int {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_NET_DEVMEM"))]
#[inline]
pub unsafe fn mp_dmabuf_devmem_alloc_netmems(
    _pool: *mut page_pool,
    _gfp: gfp_t,
) -> netmem_ref {
    0
}

#[cfg(not(feature = "CONFIG_NET_DEVMEM"))]
#[inline]
pub unsafe fn mp_dmabuf_devmem_destroy(_pool: *mut page_pool) {}

#[cfg(not(feature = "CONFIG_NET_DEVMEM"))]
#[inline]
pub unsafe fn mp_dmabuf_devmem_release_page(
    _pool: *mut page_pool,
    _netmem: netmem_ref,
) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
