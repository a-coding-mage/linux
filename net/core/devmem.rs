// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *      Devmem TCP
 *
 *      Authors: Mina Almasry <almasrymina@google.com>
 *               Willem de Bruijn <willemdebruijn.kernel@gmail.com>
 *               Kaiyuan Zhang <kaiyuanz@google.com>
 */

// Kernel dependencies supplied by other translation units.

static mut NET_DEVMEM_DMABUF_BINDINGS: xarray = DEFINE_XARRAY_FLAGS!(XA_FLAGS_ALLOC1);
static DMABUF_DEVMEM_OPS: memory_provider_ops = memory_provider_ops {
    init: Some(mp_dmabuf_devmem_init),
    destroy: Some(mp_dmabuf_devmem_destroy),
    alloc_netmems: Some(mp_dmabuf_devmem_alloc_netmems),
    release_netmem: Some(mp_dmabuf_devmem_release_page),
    nl_fill: Some(mp_dmabuf_devmem_nl_fill),
    uninstall: Some(mp_dmabuf_devmem_uninstall),
};

unsafe fn net_devmem_dmabuf_free_chunk_owner(
    _genpool: *mut gen_pool,
    chunk: *mut gen_pool_chunk,
    _not_used: *mut core::ffi::c_void,
) {
    let owner = (*chunk).owner as *mut dmabuf_genpool_chunk_owner;
    kvfree((*owner).area.niovs as *mut core::ffi::c_void);
    kfree(owner as *mut core::ffi::c_void);
}

unsafe fn net_devmem_get_dma_addr(niov: *const net_iov) -> dma_addr_t {
    let owner = net_devmem_iov_to_chunk_owner(niov);
    (*owner).base_dma_addr
        + ((net_iov_idx(niov) as dma_addr_t) << (*owner).binding.as_ref().unwrap().niov_shift)
}

unsafe extern "C" fn net_devmem_dmabuf_binding_release(ref_: *mut percpu_ref) {
    let binding = container_of!(ref_, net_devmem_dmabuf_binding, ref_);
    INIT_WORK!(&mut (*binding).unbind_w, __net_devmem_dmabuf_binding_free);
    schedule_work(&mut (*binding).unbind_w);
}

pub unsafe extern "C" fn __net_devmem_dmabuf_binding_free(wq: *mut work_struct) {
    let binding = container_of!(wq, net_devmem_dmabuf_binding, unbind_w);
    let size: usize;
    let avail: usize;

    gen_pool_for_each_chunk(
        (*binding).chunk_pool,
        Some(net_devmem_dmabuf_free_chunk_owner),
        core::ptr::null_mut(),
    );
    size = gen_pool_size((*binding).chunk_pool);
    avail = gen_pool_avail((*binding).chunk_pool);
    if !WARN(size != avail, "can't destroy genpool. size={}, avail={}", size, avail) {
        gen_pool_destroy((*binding).chunk_pool);
    }
    dma_buf_unmap_attachment_unlocked(
        (*binding).attachment,
        (*binding).sgt,
        (*binding).direction,
    );
    dma_buf_detach((*binding).dmabuf, (*binding).attachment);
    dma_buf_put((*binding).dmabuf);
    xa_destroy(&mut (*binding).bound_rxqs);
    percpu_ref_exit(&mut (*binding).ref_);
    kvfree((*binding).tx_vec as *mut core::ffi::c_void);
    kfree(binding as *mut core::ffi::c_void);
}

pub unsafe extern "C" fn net_devmem_alloc_dmabuf(
    binding: *mut net_devmem_dmabuf_binding,
) -> *mut net_iov {
    let mut owner: *mut dmabuf_genpool_chunk_owner = core::ptr::null_mut();
    let dma_addr = gen_pool_alloc_owner(
        (*binding).chunk_pool,
        1usize << (*binding).niov_shift,
        &mut owner as *mut _ as *mut *mut core::ffi::c_void,
    );
    if dma_addr == 0 { return core::ptr::null_mut(); }
    let offset = dma_addr - (*owner).base_dma_addr;
    let index = offset >> (*binding).niov_shift;
    let niov = &mut *(*owner).area.niovs.add(index);
    niov.desc.pp_magic = 0;
    niov.desc.pp = core::ptr::null_mut();
    atomic_long_set(&mut niov.desc.pp_ref_count, 0);
    niov
}

pub unsafe extern "C" fn net_devmem_free_dmabuf(niov: *mut net_iov) {
    let binding = net_devmem_iov_binding(niov);
    let dma_addr = net_devmem_get_dma_addr(niov);
    let niov_size = 1usize << (*binding).niov_shift;
    if WARN_ON(!gen_pool_has_addr((*binding).chunk_pool, dma_addr, niov_size)) { return; }
    gen_pool_free((*binding).chunk_pool, dma_addr, niov_size);
}

pub unsafe extern "C" fn net_devmem_unbind_dmabuf(binding: *mut net_devmem_dmabuf_binding) {
    let mut xa_idx: c_ulong = 0;
    let mut rxq: *mut netdev_rx_queue = core::ptr::null_mut();
    xa_erase(&mut NET_DEVMEM_DMABUF_BINDINGS, (*binding).id);
    synchronize_net();
    if !(*binding).list.next.is_null() { list_del(&mut (*binding).list); }
    xa_for_each!(&(*binding).bound_rxqs, xa_idx, rxq, {
        let mp_params = pp_memory_provider_params { mp_priv: binding as *mut _, mp_ops: &DMABUF_DEVMEM_OPS, ..core::mem::zeroed() };
        let rxq_idx = get_netdev_rx_queue_index(rxq);
        netif_mp_close_rxq((*binding).dev, rxq_idx, &mp_params);
    });
    percpu_ref_kill(&mut (*binding).ref_);
}

pub unsafe extern "C" fn net_devmem_bind_dmabuf_to_queue(
    dev: *mut net_device, rxq_idx: u32, binding: *mut net_devmem_dmabuf_binding,
    extack: *mut netlink_ext_ack,
) -> c_int {
    let mut mp_params = pp_memory_provider_params { mp_priv: binding as *mut _, mp_ops: &DMABUF_DEVMEM_OPS, ..core::mem::zeroed() };
    let mut xa_idx: u32 = 0;
    if (*binding).niov_shift != PAGE_SHIFT { mp_params.rx_page_size = 1u32 << (*binding).niov_shift; }
    let err = netif_mp_open_rxq(dev, rxq_idx, &mp_params, extack);
    if err != 0 { return err; }
    let rxq = __netif_get_rx_queue(dev, rxq_idx);
    let err = xa_alloc(&mut (*binding).bound_rxqs, &mut xa_idx, rxq, xa_limit_32b, GFP_KERNEL);
    if err != 0 { netif_mp_close_rxq(dev, rxq_idx, &mp_params); }
    err
}

pub unsafe extern "C" fn net_devmem_bind_dmabuf(
    dev: *mut net_device, vdev: *mut core::ffi::c_void, dma_dev: *mut device,
    direction: dma_data_direction, dmabuf_fd: c_uint, niov_shift: c_uint,
    priv_: *mut netdev_nl_sock, extack: *mut netlink_ext_ack,
) -> *mut net_devmem_dmabuf_binding {
    let niov_size = 1usize << niov_shift;
    static mut ID_ALLOC_NEXT: u32 = 0;
    let mut binding: *mut net_devmem_dmabuf_binding;
    let mut err: c_int;
    if dma_dev.is_null() { NL_SET_ERR_MSG!(extack, "Device doesn't support DMA"); return ERR_PTR(-EOPNOTSUPP); }
    let dmabuf = dma_buf_get(dmabuf_fd);
    if IS_ERR(dmabuf) { return ERR_CAST(dmabuf); }
    binding = kzalloc_node(core::mem::size_of::<net_devmem_dmabuf_binding>(), GFP_KERNEL, dev_to_node(&(*dev).dev)) as *mut _;
    if binding.is_null() { err = -ENOMEM; goto!(err_put_dmabuf); }
    (*binding).dev = dev; (*binding).vdev = vdev; (*binding).niov_shift = niov_shift;
    xa_init_flags(&mut (*binding).bound_rxqs, XA_FLAGS_ALLOC);
    err = percpu_ref_init(&mut (*binding).ref_, Some(net_devmem_dmabuf_binding_release), 0, GFP_KERNEL);
    if err < 0 { goto!(err_free_binding); }
    mutex_init(&mut (*binding).lock);
    (*binding).dmabuf = dmabuf; (*binding).direction = direction;
    (*binding).attachment = dma_buf_attach(dmabuf, dma_dev);
    if IS_ERR((*binding).attachment) { err = PTR_ERR((*binding).attachment); NL_SET_ERR_MSG!(extack, "Failed to bind dmabuf to device"); goto!(err_exit_ref); }
    (*binding).sgt = dma_buf_map_attachment_unlocked((*binding).attachment, direction);
    if IS_ERR((*binding).sgt) { err = PTR_ERR((*binding).sgt); NL_SET_ERR_MSG!(extack, "Failed to map dmabuf attachment"); goto!(err_detach); }
    if direction == DMA_TO_DEVICE {
        if !IS_ALIGNED((*dmabuf).size, PAGE_SIZE) { err = -EINVAL; NL_SET_ERR_MSG!(extack, "TX dma-buf size must be a multiple of PAGE_SIZE"); goto!(err_unmap); }
        (*binding).tx_vec = kvmalloc_objs(core::mem::size_of::<*mut net_iov>(), (*dmabuf).size / PAGE_SIZE) as *mut *mut net_iov;
        if (*binding).tx_vec.is_null() { err = -ENOMEM; goto!(err_unmap); }
    }
    (*binding).chunk_pool = gen_pool_create(niov_shift, dev_to_node(&(*dev).dev));
    if (*binding).chunk_pool.is_null() { err = -ENOMEM; goto!(err_tx_vec); }
    let mut virtual_: c_ulong = 0;
    let mut sg: *mut scatterlist = core::ptr::null_mut();
    let mut sg_idx: c_uint = 0;
    for_each_sgtable_dma_sg!((*binding).sgt, sg, sg_idx, {
        let dma_addr = sg_dma_address(sg); let len = sg_dma_len(sg);
        if !IS_ALIGNED(dma_addr, niov_size) || !IS_ALIGNED(len, niov_size) { err = -EINVAL; goto!(err_free_chunks); }
        let owner = kzalloc_node(core::mem::size_of::<dmabuf_genpool_chunk_owner>(), GFP_KERNEL, dev_to_node(&(*dev).dev)) as *mut dmabuf_genpool_chunk_owner;
        if owner.is_null() { err = -ENOMEM; goto!(err_free_chunks); }
        (*owner).area.base_virtual = virtual_; (*owner).base_dma_addr = dma_addr; (*owner).area.num_niovs = len >> niov_shift; (*owner).binding = binding;
        err = gen_pool_add_owner((*binding).chunk_pool, dma_addr, dma_addr, len, dev_to_node(&(*dev).dev), owner);
        if err != 0 { kfree(owner as *mut _); err = -EINVAL; goto!(err_free_chunks); }
        (*owner).area.niovs = kvmalloc_objs(core::mem::size_of::<net_iov>(), (*owner).area.num_niovs) as *mut net_iov;
        if (*owner).area.niovs.is_null() { err = -ENOMEM; goto!(err_free_chunks); }
        for i in 0..(*owner).area.num_niovs { let niov = (*owner).area.niovs.add(i); net_iov_init(niov, &mut (*owner).area, NET_IOV_DMABUF); page_pool_set_dma_addr_netmem(net_iov_to_netmem(niov), net_devmem_get_dma_addr(niov)); if direction == DMA_TO_DEVICE { *(*binding).tx_vec.add((*owner).area.base_virtual as usize / PAGE_SIZE + i) = niov; } }
        virtual_ += len;
    });
    err = xa_alloc_cyclic(&mut NET_DEVMEM_DMABUF_BINDINGS, &mut (*binding).id, binding, xa_limit_32b, &mut ID_ALLOC_NEXT, GFP_KERNEL);
    if err < 0 { goto!(err_free_chunks); }
    list_add(&mut (*binding).list, &mut (*priv_).bindings); return binding;
    label! { err_free_chunks: gen_pool_for_each_chunk((*binding).chunk_pool, Some(net_devmem_dmabuf_free_chunk_owner), core::ptr::null_mut()); gen_pool_destroy((*binding).chunk_pool); }
    label! { err_tx_vec: kvfree((*binding).tx_vec as *mut _); }
    label! { err_unmap: dma_buf_unmap_attachment_unlocked((*binding).attachment, (*binding).sgt, direction); }
    label! { err_detach: dma_buf_detach(dmabuf, (*binding).attachment); }
    label! { err_exit_ref: percpu_ref_exit(&mut (*binding).ref_); }
    label! { err_free_binding: kfree(binding as *mut _); }
    label! { err_put_dmabuf: dma_buf_put(dmabuf); }
    ERR_PTR(err)
}

pub unsafe extern "C" fn net_devmem_lookup_dmabuf(id: u32) -> *mut net_devmem_dmabuf_binding { rcu_read_lock(); let binding = xa_load(&NET_DEVMEM_DMABUF_BINDINGS, id); let result = if !binding.is_null() && !net_devmem_dmabuf_binding_get(binding) { core::ptr::null_mut() } else { binding }; rcu_read_unlock(); result }
pub unsafe extern "C" fn net_devmem_get_net_iov(niov: *mut net_iov) { net_devmem_dmabuf_binding_get(net_devmem_iov_binding(niov)); }
pub unsafe extern "C" fn net_devmem_put_net_iov(niov: *mut net_iov) { net_devmem_dmabuf_binding_put(net_devmem_iov_binding(niov)); }

pub unsafe extern "C" fn net_devmem_get_binding(sk: *mut sock, dmabuf_id: c_uint) -> *mut net_devmem_dmabuf_binding {
    let binding = net_devmem_lookup_dmabuf(dmabuf_id); if binding.is_null() || (*binding).tx_vec.is_null() { if !binding.is_null() { net_devmem_dmabuf_binding_put(binding); } return ERR_PTR(-EINVAL); }
    rcu_read_lock(); let mut dst = __sk_dst_get(sk); if dst.is_null() { if inet_csk(sk).icsk_af_ops.rebuild_header(sk) != 0 { rcu_read_unlock(); net_devmem_dmabuf_binding_put(binding); return ERR_PTR(-EHOSTUNREACH); } dst = __sk_dst_get(sk); if dst.is_null() { rcu_read_unlock(); net_devmem_dmabuf_binding_put(binding); return ERR_PTR(-ENODEV); } }
    let dst_dev = dst_dev_rcu(dst); if dst_dev.is_null() || (dst_dev != READ_ONCE!((*binding).dev) && dst_dev != READ_ONCE!((*binding).vdev)) { rcu_read_unlock(); net_devmem_dmabuf_binding_put(binding); return ERR_PTR(-ENODEV); } rcu_read_unlock(); binding
}

pub unsafe extern "C" fn net_devmem_get_niov_at(binding: *mut net_devmem_dmabuf_binding, virt_addr: usize, off: *mut usize, size: *mut usize) -> *mut net_iov { if virt_addr >= (*binding).dmabuf.size { return core::ptr::null_mut(); } *off = virt_addr % PAGE_SIZE; *size = PAGE_SIZE - *off; *(*binding).tx_vec.add(virt_addr / PAGE_SIZE) }

pub unsafe extern "C" fn mp_dmabuf_devmem_init(pool: *mut page_pool) -> c_int { let binding = (*pool).mp_priv as *mut net_devmem_dmabuf_binding; if binding.is_null() { return -EINVAL; } (*pool).dma_sync = false; (*pool).dma_sync_for_cpu = false; if (*pool).p.order != (*binding).niov_shift - PAGE_SHIFT { return -E2BIG; } net_devmem_dmabuf_binding_get(binding); 0 }
pub unsafe extern "C" fn mp_dmabuf_devmem_alloc_netmems(pool: *mut page_pool, _gfp: gfp_t) -> netmem_ref { let binding = (*pool).mp_priv as *mut net_devmem_dmabuf_binding; let niov = net_devmem_alloc_dmabuf(binding); if niov.is_null() { return 0; } let netmem = net_iov_to_netmem(niov); page_pool_set_pp_info(pool, netmem); (*pool).pages_state_hold_cnt += 1; trace_page_pool_state_hold(pool, netmem, (*pool).pages_state_hold_cnt); netmem }
pub unsafe extern "C" fn mp_dmabuf_devmem_destroy(pool: *mut page_pool) { net_devmem_dmabuf_binding_put((*pool).mp_priv as *mut net_devmem_dmabuf_binding); }
pub unsafe extern "C" fn mp_dmabuf_devmem_release_page(_pool: *mut page_pool, netmem: netmem_ref) -> bool { let refcount = atomic_long_read(netmem_get_pp_ref_count_ref(netmem)); if WARN_ON_ONCE(!netmem_is_net_iov(netmem)) || WARN_ON_ONCE(refcount != 1) { return false; } page_pool_clear_pp_info(netmem); net_devmem_free_dmabuf(netmem_to_net_iov(netmem)); false }
unsafe extern "C" fn mp_dmabuf_devmem_nl_fill(mp_priv: *mut core::ffi::c_void, rsp: *mut sk_buff, rxq: *mut netdev_rx_queue) -> c_int { let binding = mp_priv as *const net_devmem_dmabuf_binding; nla_put_u32(rsp, if !rxq.is_null() { NETDEV_A_QUEUE_DMABUF } else { NETDEV_A_PAGE_POOL_DMABUF }, (*binding).id) }
unsafe extern "C" fn mp_dmabuf_devmem_uninstall(mp_priv: *mut core::ffi::c_void, rxq: *mut netdev_rx_queue) { let binding = mp_priv as *mut net_devmem_dmabuf_binding; let mut xa_idx = 0; let mut bound_rxq = core::ptr::null_mut(); xa_for_each!(&(*binding).bound_rxqs, xa_idx, bound_rxq, { if bound_rxq == rxq { xa_erase(&mut (*binding).bound_rxqs, xa_idx); if xa_empty(&(*binding).bound_rxqs) { mutex_lock(&mut (*binding).lock); ASSERT_EXCLUSIVE_WRITER!((*binding).dev); WRITE_ONCE!((*binding).dev, core::ptr::null_mut()); mutex_unlock(&mut (*binding).lock); } break; } }); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
