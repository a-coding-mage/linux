/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Device memory TCP support
 *
 * Authors: Mina Almasry <almasrymina@google.com>
 *          Willem de Bruijn <willemb@google.com>
 *          Kaiyuan Zhang <kaiyuanz@google.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external; this file translates only the declarations from devmem.h.

#[repr(C)]
pub struct net_devmem_dmabuf_binding {
    pub dmabuf: *mut dma_buf,
    pub attachment: *mut dma_buf_attachment,
    pub sgt: *mut sg_table,
    /* Physical NIC that does the actual DMA for this binding. */
    pub dev: *mut net_device,
    /* Opaque cookie identifying the virtual device (e.g. netkit) the user
     * called bind-tx on. Used only for pointer comparison. Never dereferenced.
     */
    pub vdev: *mut core::ffi::c_void,
    pub chunk_pool: *mut gen_pool,
    /* Protect dev */
    pub lock: mutex,
    /* The user holds a ref (via the netlink API) for as long as they want the
     * binding to remain alive. Each page pool using this binding holds a ref
     * to keep the binding alive. The page_pool does not release the ref until
     * all the net_iovs allocated from this binding are released back to the
     * page_pool.
     *
     * The binding undos itself and unmaps the underlying dmabuf once all those
     * refs are dropped and the binding is no longer desired or in use.
     *
     * net_devmem_get_net_iov() on dmabuf net_iovs will increment this
     * reference, making sure that the binding remains alive until all the
     * net_iovs are no longer used. net_iovs allocated from this binding that
     * are stuck in the TX path for any reason (such as awaiting retransmits)
     * hold a reference to the binding until the skb holding them is freed.
     */
    pub ref_: percpu_ref,
    /* The list of bindings currently active. Used for netlink to notify us
     * of the user dropping the bind.
     */
    pub list: list_head,
    /* rxq's this binding is active on. */
    pub bound_rxqs: xarray,
    /* ID of this binding. Globally unique to all bindings currently active. */
    pub id: u32,
    /* DMA direction, FROM_DEVICE for Rx binding, TO_DEVICE for Tx. */
    pub direction: dma_data_direction,
    /* Array of net_iov pointers for this binding, sorted by virtual address. */
    pub tx_vec: *mut *mut net_iov,
    pub niov_shift: core::ffi::c_uint,
    pub unbind_w: work_struct,
}

#[cfg(CONFIG_NET_DEVMEM)]
#[repr(C)]
pub struct dmabuf_genpool_chunk_owner {
    pub area: net_iov_area,
    pub binding: *mut net_devmem_dmabuf_binding,
    /* dma_addr of the start of the chunk. */
    pub base_dma_addr: dma_addr_t,
}

#[cfg(CONFIG_NET_DEVMEM)]
unsafe extern "C" {
    pub fn __net_devmem_dmabuf_binding_free(wq: *mut work_struct);
    pub fn net_devmem_bind_dmabuf(
        dev: *mut net_device, vdev: *mut core::ffi::c_void,
        dma_dev: *mut device, direction: dma_data_direction,
        dmabuf_fd: core::ffi::c_uint, niov_shift: core::ffi::c_uint,
        priv_: *mut netdev_nl_sock, extack: *mut netlink_ext_ack,
    ) -> *mut net_devmem_dmabuf_binding;
    pub fn net_devmem_lookup_dmabuf(id: u32) -> *mut net_devmem_dmabuf_binding;
    pub fn net_devmem_unbind_dmabuf(binding: *mut net_devmem_dmabuf_binding);
    pub fn net_devmem_bind_dmabuf_to_queue(
        dev: *mut net_device, rxq_idx: u32,
        binding: *mut net_devmem_dmabuf_binding,
        extack: *mut netlink_ext_ack,
    ) -> core::ffi::c_int;
    pub fn net_devmem_get_net_iov(niov: *mut net_iov);
    pub fn net_devmem_put_net_iov(niov: *mut net_iov);
    pub fn net_devmem_alloc_dmabuf(binding: *mut net_devmem_dmabuf_binding) -> *mut net_iov;
    pub fn net_devmem_free_dmabuf(ppiov: *mut net_iov);
    pub fn net_devmem_get_binding(sk: *mut sock, dmabuf_id: core::ffi::c_uint) -> *mut net_devmem_dmabuf_binding;
    pub fn net_devmem_get_niov_at(
        binding: *mut net_devmem_dmabuf_binding, addr: usize,
        off: *mut usize, size: *mut usize,
    ) -> *mut net_iov;
}

#[cfg(not(CONFIG_NET_DEVMEM))]
#[repr(C)]
pub struct net_devmem_dmabuf_binding;

#[cfg(CONFIG_NET_DEVMEM)]
#[inline]
pub unsafe fn net_devmem_iov_binding(niov: *const net_iov) -> *mut net_devmem_dmabuf_binding {
    net_devmem_iov_to_chunk_owner(niov).as_ref().unwrap().binding
}

#[cfg(CONFIG_NET_DEVMEM)]
#[inline]
pub unsafe fn net_devmem_iov_to_chunk_owner(niov: *const net_iov) -> *mut dmabuf_genpool_chunk_owner {
    // Equivalent to container_of(net_iov_owner(niov), dmabuf_genpool_chunk_owner, area).
    net_iov_owner(niov) as *mut net_iov_area as *mut dmabuf_genpool_chunk_owner
}

#[cfg(CONFIG_NET_DEVMEM)]
#[inline]
pub unsafe fn net_devmem_iov_binding_id(niov: *const net_iov) -> u32 {
    (*net_devmem_iov_binding(niov)).id
}

#[cfg(CONFIG_NET_DEVMEM)]
#[inline]
pub unsafe fn net_iov_virtual_addr(niov: *const net_iov) -> core::ffi::c_ulong {
    let co = &*net_devmem_iov_to_chunk_owner(niov);
    (*net_iov_owner(niov)).base_virtual
        .wrapping_add((net_iov_idx(niov) as core::ffi::c_ulong) << (*co.binding).niov_shift)
}

#[cfg(not(CONFIG_NET_DEVMEM))]
#[inline] pub unsafe fn net_iov_virtual_addr(_: *const net_iov) -> core::ffi::c_ulong { 0 }
#[cfg(not(CONFIG_NET_DEVMEM))]
#[inline] pub unsafe fn net_devmem_iov_binding_id(_: *const net_iov) -> u32 { 0 }

#[cfg(not(CONFIG_NET_DEVMEM))]
#[inline] pub unsafe fn net_devmem_iov_binding(_: *const net_iov) -> *mut net_devmem_dmabuf_binding { core::ptr::null_mut() }

#[cfg(CONFIG_NET_DEVMEM)]
#[inline]
pub unsafe fn net_devmem_dmabuf_binding_get(binding: *mut net_devmem_dmabuf_binding) -> bool { percpu_ref_tryget(&mut (*binding).ref_) }
#[cfg(CONFIG_NET_DEVMEM)]
#[inline]
pub unsafe fn net_devmem_dmabuf_binding_put(binding: *mut net_devmem_dmabuf_binding) { percpu_ref_put(&mut (*binding).ref_) }
#[cfg(not(CONFIG_NET_DEVMEM))]
#[inline] pub unsafe fn net_devmem_dmabuf_binding_put(_: *mut net_devmem_dmabuf_binding) {}

#[cfg(not(CONFIG_NET_DEVMEM))]
#[inline] pub unsafe fn net_devmem_get_net_iov(_: *mut net_iov) {}
#[cfg(not(CONFIG_NET_DEVMEM))]
#[inline] pub unsafe fn net_devmem_put_net_iov(_: *mut net_iov) {}
#[cfg(not(CONFIG_NET_DEVMEM))]
#[inline] pub unsafe fn net_devmem_bind_dmabuf(_: *mut net_device, _: *mut core::ffi::c_void, _: *mut device, _: dma_data_direction, _: core::ffi::c_uint, _: core::ffi::c_uint, _: *mut netdev_nl_sock, _: *mut netlink_ext_ack) -> *mut net_devmem_dmabuf_binding { ERR_PTR(-EOPNOTSUPP) }
#[cfg(not(CONFIG_NET_DEVMEM))]
#[inline] pub unsafe fn net_devmem_lookup_dmabuf(_: u32) -> *mut net_devmem_dmabuf_binding { core::ptr::null_mut() }
#[cfg(not(CONFIG_NET_DEVMEM))]
#[inline] pub unsafe fn net_devmem_unbind_dmabuf(_: *mut net_devmem_dmabuf_binding) {}
#[cfg(not(CONFIG_NET_DEVMEM))]
#[inline] pub unsafe fn net_devmem_bind_dmabuf_to_queue(_: *mut net_device, _: u32, _: *mut net_devmem_dmabuf_binding, _: *mut netlink_ext_ack) -> core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(CONFIG_NET_DEVMEM))]
#[inline] pub unsafe fn net_devmem_alloc_dmabuf(_: *mut net_devmem_dmabuf_binding) -> *mut net_iov { core::ptr::null_mut() }
#[cfg(not(CONFIG_NET_DEVMEM))]
#[inline] pub unsafe fn net_devmem_free_dmabuf(_: *mut net_iov) {}
#[cfg(not(CONFIG_NET_DEVMEM))]
#[inline] pub unsafe fn net_devmem_get_binding(_: *mut sock, _: core::ffi::c_uint) -> *mut net_devmem_dmabuf_binding { ERR_PTR(-EOPNOTSUPP) }
#[cfg(not(CONFIG_NET_DEVMEM))]
#[inline] pub unsafe fn net_devmem_get_niov_at(_: *mut net_devmem_dmabuf_binding, _: usize, _: *mut usize, _: *mut usize) -> *mut net_iov { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
