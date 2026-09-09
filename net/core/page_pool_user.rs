// SPDX-License-Identifier: GPL-2.0
// Translated from page_pool_user.c. Kernel dependencies are supplied externally.

type PpNlFillCb = unsafe extern "C" fn(*mut sk_buff, *const page_pool, *const genl_info) -> c_int;

static mut PAGE_POOLS: xarray = DEFINE_XARRAY_FLAGS(XA_FLAGS_ALLOC1);
// Protects page_pools, netdevice->page_pools, pool->p.napi, pool->slow.netdev, pool->user.
// Ordering: inside rtnl_lock.
static mut PAGE_POOLS_LOCK: mutex = DEFINE_MUTEX();

unsafe fn netdev_nl_page_pool_get_do(info: *mut genl_info, id: u32, fill: PpNlFillCb) -> c_int {
    let mut err: c_int;
    mutex_lock(&raw mut PAGE_POOLS_LOCK);
    let pool = xa_load(&raw mut PAGE_POOLS, id) as *mut page_pool;
    if pool.is_null() || hlist_unhashed(&(*pool).user.list) ||
        !net_eq(dev_net((*pool).slow.netdev), genl_info_net(info)) {
        err = -ENOENT;
        mutex_unlock(&raw mut PAGE_POOLS_LOCK);
        return err;
    }
    let rsp = genlmsg_new(GENLMSG_DEFAULT_SIZE, GFP_KERNEL);
    if rsp.is_null() {
        mutex_unlock(&raw mut PAGE_POOLS_LOCK);
        return -ENOMEM;
    }
    err = fill(rsp, pool, info);
    if err != 0 {
        nlmsg_free(rsp);
        mutex_unlock(&raw mut PAGE_POOLS_LOCK);
        return err;
    }
    mutex_unlock(&raw mut PAGE_POOLS_LOCK);
    genlmsg_reply(rsp, info)
}

#[repr(C)]
struct PagePoolDumpCb { ifindex: c_ulong, pp_id: u32 }

unsafe fn netdev_nl_page_pool_get_dump(skb: *mut sk_buff, cb: *mut netlink_callback,
                                        fill: PpNlFillCb, ifindex_attr: *mut nlattr) -> c_int {
    let state = (*cb).ctx as *mut PagePoolDumpCb;
    let info = genl_info_dump(cb);
    let net = sock_net((*skb).sk);
    let mut err = 0;
    if !ifindex_attr.is_null() { (*state).ifindex = nla_get_u32(ifindex_attr) as c_ulong; }
    rtnl_lock();
    mutex_lock(&raw mut PAGE_POOLS_LOCK);
    for_each_netdev_dump(net, netdev, (*state).ifindex) {
        if !ifindex_attr.is_null() && (*netdev).ifindex != nla_get_u32(ifindex_attr) { break; }
        hlist_for_each_entry(pool, &(*netdev).page_pools, user.list) {
            if (*state).pp_id != 0 && (*state).pp_id < (*pool).user.id { continue; }
            (*state).pp_id = (*pool).user.id;
            err = fill(skb, pool, info);
            if err != 0 { break; }
        }
        if err != 0 { break; }
        (*state).pp_id = 0;
    }
    mutex_unlock(&raw mut PAGE_POOLS_LOCK);
    rtnl_unlock();
    err
}

unsafe fn page_pool_nl_stats_fill(rsp: *mut sk_buff, pool: *const page_pool,
                                  info: *const genl_info) -> c_int {
    #[cfg(feature = "CONFIG_PAGE_POOL_STATS")]
    {
        let mut stats: page_pool_stats = core::mem::zeroed();
        page_pool_get_stats(pool, &mut stats);
        let hdr = genlmsg_iput(rsp, info);
        if hdr.is_null() { return -EMSGSIZE; }
        let nest = nla_nest_start(rsp, NETDEV_A_PAGE_POOL_STATS_INFO);
        if nest.is_null() { genlmsg_cancel(rsp, hdr); return -EMSGSIZE; }
        if nla_put_uint(rsp, NETDEV_A_PAGE_POOL_ID, (*pool).user.id) != 0 ||
           ((*pool).slow.netdev->ifindex != LOOPBACK_IFINDEX && nla_put_u32(rsp, NETDEV_A_PAGE_POOL_IFINDEX, (*pool).slow.netdev->ifindex) != 0) {
            nla_nest_cancel(rsp, nest); genlmsg_cancel(rsp, hdr); return -EMSGSIZE;
        }
        nla_nest_end(rsp, nest);
        if nla_put_uint(rsp, NETDEV_A_PAGE_POOL_STATS_ALLOC_FAST, stats.alloc_stats.fast) != 0 ||
           nla_put_uint(rsp, NETDEV_A_PAGE_POOL_STATS_ALLOC_SLOW, stats.alloc_stats.slow) != 0 ||
           nla_put_uint(rsp, NETDEV_A_PAGE_POOL_STATS_ALLOC_SLOW_HIGH_ORDER, stats.alloc_stats.slow_high_order) != 0 ||
           nla_put_uint(rsp, NETDEV_A_PAGE_POOL_STATS_ALLOC_EMPTY, stats.alloc_stats.empty) != 0 ||
           nla_put_uint(rsp, NETDEV_A_PAGE_POOL_STATS_ALLOC_REFILL, stats.alloc_stats.refill) != 0 ||
           nla_put_uint(rsp, NETDEV_A_PAGE_POOL_STATS_ALLOC_WAIVE, stats.alloc_stats.waive) != 0 ||
           nla_put_uint(rsp, NETDEV_A_PAGE_POOL_STATS_RECYCLE_CACHED, stats.recycle_stats.cached) != 0 ||
           nla_put_uint(rsp, NETDEV_A_PAGE_POOL_STATS_RECYCLE_CACHE_FULL, stats.recycle_stats.cache_full) != 0 ||
           nla_put_uint(rsp, NETDEV_A_PAGE_POOL_STATS_RECYCLE_RING, stats.recycle_stats.ring) != 0 ||
           nla_put_uint(rsp, NETDEV_A_PAGE_POOL_STATS_RECYCLE_RING_FULL, stats.recycle_stats.ring_full) != 0 ||
           nla_put_uint(rsp, NETDEV_A_PAGE_POOL_STATS_RECYCLE_RELEASED_REFCNT, stats.recycle_stats.released_refcnt) != 0 {
            genlmsg_cancel(rsp, hdr); return -EMSGSIZE;
        }
        genlmsg_end(rsp, hdr); return 0;
    }
    #[cfg(not(feature = "CONFIG_PAGE_POOL_STATS"))]
    { GENL_SET_ERR_MSG(info, "kernel built without CONFIG_PAGE_POOL_STATS"); -EOPNOTSUPP }
}

pub unsafe extern "C" fn netdev_nl_page_pool_stats_get_doit(_skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    if GENL_REQ_ATTR_CHECK(info, NETDEV_A_PAGE_POOL_STATS_INFO) != 0 { return -EINVAL; }
    let nest = (*info).attrs[NETDEV_A_PAGE_POOL_STATS_INFO];
    let mut tb: [*mut nlattr; ARRAY_SIZE(netdev_page_pool_info_nl_policy)] = core::mem::zeroed();
    let err = nla_parse_nested(tb.as_mut_ptr(), tb.len() - 1, nest, netdev_page_pool_info_nl_policy, (*info).extack);
    if err != 0 { return err; }
    if NL_REQ_ATTR_CHECK((*info).extack, nest, tb.as_mut_ptr(), NETDEV_A_PAGE_POOL_ID) != 0 { return -EINVAL; }
    if !tb[NETDEV_A_PAGE_POOL_IFINDEX].is_null() { NL_SET_ERR_MSG_ATTR((*info).extack, tb[NETDEV_A_PAGE_POOL_IFINDEX], "selecting by ifindex not supported"); return -EINVAL; }
    netdev_nl_page_pool_get_do(info, nla_get_uint(tb[NETDEV_A_PAGE_POOL_ID]), page_pool_nl_stats_fill)
}

#[repr(C)]
static PAGE_POOL_IFINDEX_RANGE: netlink_range_validation = netlink_range_validation { min: 1, max: S32_MAX as u64 };
static PAGE_POOL_STAT_INFO_POLICY: [nla_policy; NETDEV_A_PAGE_POOL_IFINDEX + 1] = [NLA_POLICY_FULL_RANGE(NLA_U32, &PAGE_POOL_IFINDEX_RANGE)];

pub unsafe extern "C" fn netdev_nl_page_pool_stats_get_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    let info = genl_info_dump(cb);
    let mut tb: [*mut nlattr; ARRAY_SIZE(PAGE_POOL_STAT_INFO_POLICY)] = core::mem::zeroed();
    let mut attr = core::ptr::null_mut();
    if !(*info).attrs[NETDEV_A_PAGE_POOL_STATS_INFO].is_null() {
        let nest = (*info).attrs[NETDEV_A_PAGE_POOL_STATS_INFO];
        let err = nla_parse_nested(tb.as_mut_ptr(), tb.len() - 1, nest, PAGE_POOL_STAT_INFO_POLICY.as_ptr(), (*info).extack);
        if err != 0 { return err; }
        attr = tb[NETDEV_A_PAGE_POOL_IFINDEX];
    }
    netdev_nl_page_pool_get_dump(skb, cb, page_pool_nl_stats_fill, attr)
}

unsafe fn page_pool_nl_fill(rsp: *mut sk_buff, pool: *const page_pool, info: *const genl_info) -> c_int {
    let hdr = genlmsg_iput(rsp, info); if hdr.is_null() { return -EMSGSIZE; }
    if nla_put_uint(rsp, NETDEV_A_PAGE_POOL_ID, (*pool).user.id) != 0 { genlmsg_cancel(rsp, hdr); return -EMSGSIZE; }
    if (*pool).slow.netdev->ifindex != LOOPBACK_IFINDEX && nla_put_u32(rsp, NETDEV_A_PAGE_POOL_IFINDEX, (*pool).slow.netdev->ifindex) != 0 { genlmsg_cancel(rsp, hdr); return -EMSGSIZE; }
    let napi_id = if !(*pool).p.napi.is_null() { READ_ONCE((*pool).p.napi.napi_id) } else { 0 };
    if napi_id_valid(napi_id) && nla_put_uint(rsp, NETDEV_A_PAGE_POOL_NAPI_ID, napi_id) != 0 { genlmsg_cancel(rsp, hdr); return -EMSGSIZE; }
    let inflight = page_pool_inflight(pool, false); let refsz = PAGE_SIZE << (*pool).p.order;
    if nla_put_uint(rsp, NETDEV_A_PAGE_POOL_INFLIGHT, inflight) != 0 || nla_put_uint(rsp, NETDEV_A_PAGE_POOL_INFLIGHT_MEM, inflight * refsz) != 0 { genlmsg_cancel(rsp, hdr); return -EMSGSIZE; }
    if (*pool).user.detach_time != 0 && nla_put_uint(rsp, NETDEV_A_PAGE_POOL_DETACH_TIME, ktime_divns((*pool).user.detach_time, NSEC_PER_SEC)) != 0 { genlmsg_cancel(rsp, hdr); return -EMSGSIZE; }
    if !(*pool).mp_ops.is_null() && ((*pool).mp_ops).nl_fill((*pool).mp_priv, rsp, core::ptr::null_mut()) != 0 { genlmsg_cancel(rsp, hdr); return -EMSGSIZE; }
    genlmsg_end(rsp, hdr); 0
}

unsafe fn netdev_nl_page_pool_event(pool: *const page_pool, cmd: u32) {
    lockdep_assert_held(&raw mut PAGE_POOLS_LOCK);
    if hlist_unhashed(&(*pool).user.list) { return; }
    let net = dev_net((*pool).slow.netdev);
    if genl_has_listeners(&netdev_nl_family, net, NETDEV_NLGRP_PAGE_POOL) == 0 { return; }
    let mut info: genl_info = core::mem::zeroed(); genl_info_init_ntf(&mut info, &netdev_nl_family, cmd);
    let ntf = genlmsg_new(GENLMSG_DEFAULT_SIZE, GFP_KERNEL); if ntf.is_null() { return; }
    if page_pool_nl_fill(ntf, pool, &info) != 0 { nlmsg_free(ntf); return; }
    genlmsg_multicast_netns(&netdev_nl_family, net, ntf, 0, NETDEV_NLGRP_PAGE_POOL, GFP_KERNEL);
}

pub unsafe extern "C" fn netdev_nl_page_pool_get_doit(_skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    if GENL_REQ_ATTR_CHECK(info, NETDEV_A_PAGE_POOL_ID) != 0 { return -EINVAL; }
    netdev_nl_page_pool_get_do(info, nla_get_uint((*info).attrs[NETDEV_A_PAGE_POOL_ID]), page_pool_nl_fill)
}
pub unsafe extern "C" fn netdev_nl_page_pool_get_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    let info = genl_info_dump(cb); netdev_nl_page_pool_get_dump(skb, cb, page_pool_nl_fill, (*info).attrs[NETDEV_A_PAGE_POOL_IFINDEX])
}

pub unsafe extern "C" fn page_pool_list(pool: *mut page_pool) -> c_int {
    static mut ID_ALLOC_NEXT: u32 = 0;
    mutex_lock(&raw mut PAGE_POOLS_LOCK);
    let err = xa_alloc_cyclic(&raw mut PAGE_POOLS, &mut (*pool).user.id, pool, xa_limit_32b, &raw mut ID_ALLOC_NEXT, GFP_KERNEL);
    if err < 0 { mutex_unlock(&raw mut PAGE_POOLS_LOCK); return err; }
    INIT_HLIST_NODE(&mut (*pool).user.list);
    if !(*pool).slow.netdev.is_null() { hlist_add_head(&mut (*pool).user.list, &mut (*pool).slow.netdev.page_pools); netdev_nl_page_pool_event(pool, NETDEV_CMD_PAGE_POOL_ADD_NTF); }
    mutex_unlock(&raw mut PAGE_POOLS_LOCK); 0
}
pub unsafe extern "C" fn page_pool_detached(pool: *mut page_pool) { mutex_lock(&raw mut PAGE_POOLS_LOCK); (*pool).user.detach_time = ktime_get_boottime(); netdev_nl_page_pool_event(pool, NETDEV_CMD_PAGE_POOL_CHANGE_NTF); mutex_unlock(&raw mut PAGE_POOLS_LOCK); }
pub unsafe extern "C" fn page_pool_unlist(pool: *mut page_pool) { mutex_lock(&raw mut PAGE_POOLS_LOCK); netdev_nl_page_pool_event(pool, NETDEV_CMD_PAGE_POOL_DEL_NTF); xa_erase(&raw mut PAGE_POOLS, (*pool).user.id); if !hlist_unhashed(&(*pool).user.list) { hlist_del(&mut (*pool).user.list); } mutex_unlock(&raw mut PAGE_POOLS_LOCK); }

pub unsafe extern "C" fn page_pool_check_memory_provider(dev: *mut net_device, rxq: *mut netdev_rx_queue) -> c_int {
    let binding = (*rxq).mp_params.mp_priv; if binding.is_null() { return 0; }
    mutex_lock(&raw mut PAGE_POOLS_LOCK);
    hlist_for_each_entry_safe(pool, n, &(*dev).page_pools, user.list) { if (*pool).mp_priv != binding { continue; } if (*pool).slow.queue_idx == get_netdev_rx_queue_index(rxq) { mutex_unlock(&raw mut PAGE_POOLS_LOCK); return 0; } }
    mutex_unlock(&raw mut PAGE_POOLS_LOCK); -ENODATA
}
unsafe fn page_pool_unreg_netdev_wipe(netdev: *mut net_device) { mutex_lock(&raw mut PAGE_POOLS_LOCK); hlist_for_each_entry_safe(pool, n, &(*netdev).page_pools, user.list) { hlist_del_init(&mut (*pool).user.list); (*pool).slow.netdev = NET_PTR_POISON; } mutex_unlock(&raw mut PAGE_POOLS_LOCK); }
unsafe fn page_pool_unreg_netdev(netdev: *mut net_device) { let lo = dev_net(netdev).loopback_dev; mutex_lock(&raw mut PAGE_POOLS_LOCK); let mut last = core::ptr::null_mut(); hlist_for_each_entry(pool, &(*netdev).page_pools, user.list) { (*pool).slow.netdev = lo; netdev_nl_page_pool_event(pool, NETDEV_CMD_PAGE_POOL_CHANGE_NTF); last = pool; } if !last.is_null() { hlist_splice_init(&mut (*netdev).page_pools, &mut (*last).user.list, &mut (*lo).page_pools); } mutex_unlock(&raw mut PAGE_POOLS_LOCK); }
unsafe extern "C" fn page_pool_netdevice_event(_nb: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> c_int { let netdev = netdev_notifier_info_to_dev(ptr); if event != NETDEV_UNREGISTER { return NOTIFY_DONE; } if hlist_empty(&(*netdev).page_pools) { return NOTIFY_OK; } if (*netdev).ifindex != LOOPBACK_IFINDEX { page_pool_unreg_netdev(netdev); } else { page_pool_unreg_netdev_wipe(netdev); } NOTIFY_OK }
static mut PAGE_POOL_NETDEVICE_NB: notifier_block = notifier_block { notifier_call: Some(page_pool_netdevice_event) };
unsafe extern "C" fn page_pool_user_init() -> c_int { register_netdevice_notifier(&raw mut PAGE_POOL_NETDEVICE_NB) }
// subsys_initcall(page_pool_user_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
