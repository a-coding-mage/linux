/* SPDX-License-Identifier: GPL-2.0-or-later */
// Translated from dev.h. C headers and build-time configuration are supplied by dependencies.

pub const FLOW_LIMIT_HISTORY: usize = 1usize << 7; // must be ^2 and !overflow buckets

#[repr(C)]
pub struct sd_flow_limit {
    pub rcu: rcu_head,
    pub count: ::core::ffi::c_uint,
    pub log_buckets: u8,
    pub history_head: ::core::ffi::c_uint,
    pub history: [u16; FLOW_LIMIT_HISTORY],
    pub buckets: [u8; 0],
}

extern "C" {
    pub static mut netdev_flow_limit_table_len: ::core::ffi::c_int;

    pub fn netdev_napi_by_id_lock(net: *mut net, napi_id: ::core::ffi::c_uint) -> *mut napi_struct;
    pub fn dev_get_by_napi_id(napi_id: ::core::ffi::c_uint) -> *mut net_device;
    pub fn netdev_put_lock(dev: *mut net_device, net: *mut net, tracker: *mut netdevice_tracker) -> *mut net_device;
    pub fn netdev_xa_find_lock(net: *mut net, dev: *mut net_device, index: *mut ::core::ffi::c_ulong) -> *mut net_device;
    pub fn netdev_get_by_index_lock_ops_compat(net: *mut net, ifindex: ::core::ffi::c_int) -> *mut net_device;
    pub fn netdev_xa_find_lock_ops_compat(net: *mut net, dev: *mut net_device, index: *mut ::core::ffi::c_ulong) -> *mut net_device;

    pub fn dev_proc_init() -> ::core::ffi::c_int;
    pub fn linkwatch_init_dev(dev: *mut net_device);
    pub fn linkwatch_run_queue();
    pub fn dev_addr_flush(dev: *mut net_device);
    pub fn dev_addr_init(dev: *mut net_device) -> ::core::ffi::c_int;
    pub fn dev_addr_check(dev: *mut net_device);
    pub fn __hw_addr_flush(list: *mut netdev_hw_addr_list);
    pub fn net_shaper_flush_netdev(dev: *mut net_device);
    pub fn net_shaper_set_real_num_tx_queues(dev: *mut net_device, txq: ::core::ffi::c_uint);
    pub static mut netdev_unregister_timeout_secs: ::core::ffi::c_int;
    pub static mut weight_p: ::core::ffi::c_int;
    pub static mut dev_weight_rx_bias: ::core::ffi::c_int;
    pub static mut dev_weight_tx_bias: ::core::ffi::c_int;
    pub static mut dev_addr_sem: rw_semaphore;
    pub static mut net_todo_list: list_head;
    pub fn netdev_run_todo();
    pub fn netdev_queue_config_validate(dev: *mut net_device, rxq_idx: ::core::ffi::c_int, qcfg: *mut netdev_queue_config, extack: *mut netlink_ext_ack) -> ::core::ffi::c_int;
    pub fn netif_rxq_has_mp(dev: *mut net_device, rxq_idx: ::core::ffi::c_uint) -> bool;
    pub fn netif_rxq_is_leased(dev: *mut net_device, rxq_idx: ::core::ffi::c_uint) -> bool;
    pub fn netif_is_queue_leasee(dev: *const net_device) -> bool;
    pub fn __netif_mp_uninstall_rxq(rxq: *mut netdev_rx_queue, p: *const pp_memory_provider_params);
    pub fn netif_rxq_cleanup_unlease(phys_rxq: *mut netdev_rx_queue, virt_rxq: *mut netdev_rx_queue);
}

#[repr(C)]
pub struct netdev_name_node {
    pub hlist: hlist_node,
    pub list: list_head,
    pub dev: *mut net_device,
    pub name: *const ::core::ffi::c_char,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn netdev_get_name(net: *mut net, name: *mut ::core::ffi::c_char, ifindex: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn netif_change_name(dev: *mut net_device, newname: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn dev_change_name(dev: *mut net_device, newname: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn netdev_name_node_alt_create(dev: *mut net_device, name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn netdev_name_node_alt_destroy(dev: *mut net_device, name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn dev_validate_mtu(dev: *mut net_device, mtu: ::core::ffi::c_int, extack: *mut netlink_ext_ack) -> ::core::ffi::c_int;
    pub fn netif_set_mtu_ext(dev: *mut net_device, new_mtu: ::core::ffi::c_int, extack: *mut netlink_ext_ack) -> ::core::ffi::c_int;
    pub fn dev_get_phys_port_id(dev: *mut net_device, ppid: *mut netdev_phys_item_id) -> ::core::ffi::c_int;
    pub fn dev_get_phys_port_name(dev: *mut net_device, name: *mut ::core::ffi::c_char, len: usize) -> ::core::ffi::c_int;
    pub fn netif_change_proto_down(dev: *mut net_device, proto_down: bool) -> ::core::ffi::c_int;
    pub fn dev_change_proto_down(dev: *mut net_device, proto_down: bool) -> ::core::ffi::c_int;
    pub fn netdev_change_proto_down_reason_locked(dev: *mut net_device, mask: ::core::ffi::c_ulong, value: u32);
    pub fn dev_change_xdp_fd(dev: *mut net_device, extack: *mut netlink_ext_ack, fd: ::core::ffi::c_int, expected_fd: ::core::ffi::c_int, flags: u32) -> ::core::ffi::c_int;
    pub fn netif_change_tx_queue_len(dev: *mut net_device, new_len: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn dev_change_tx_queue_len(dev: *mut net_device, new_len: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn netif_set_group(dev: *mut net_device, new_group: ::core::ffi::c_int);
    pub fn dev_set_group(dev: *mut net_device, new_group: ::core::ffi::c_int);
    pub fn netif_change_carrier(dev: *mut net_device, new_carrier: bool) -> ::core::ffi::c_int;
    pub fn dev_change_carrier(dev: *mut net_device, new_carrier: bool) -> ::core::ffi::c_int;
    pub fn __dev_set_rx_mode(dev: *mut net_device);
    pub fn __dev_set_promiscuity(dev: *mut net_device, inc: ::core::ffi::c_int, notify: bool) -> ::core::ffi::c_int;
    pub fn netif_rx_mode_init(dev: *mut net_device);
    pub fn netif_rx_mode_run(dev: *mut net_device);
    pub fn netif_rx_mode_sync(dev: *mut net_device);
    pub fn netif_rx_mode_cancel_retry(dev: *mut net_device);
}

#[repr(u32)]
pub enum netdev_work_core { NETDEV_WORK_RX_MODE = 1u32 << 0 }

extern "C" {
    pub fn __netdev_work_core_sched(dev: *mut net_device, event: ::core::ffi::c_ulong);
    pub fn __netdev_work_core_cancel(dev: *mut net_device, mask: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn netdev_work_cancel_all(dev: *mut net_device);
    pub fn __dev_notify_flags(dev: *mut net_device, old_flags: ::core::ffi::c_uint, gchanges: ::core::ffi::c_uint, portid: u32, nlh: *const nlmsghdr);
    pub fn unregister_netdevice_many_notify(head: *mut list_head, portid: u32, nlh: *const nlmsghdr);
}

pub type bpf_op_t = unsafe extern "C" fn(*mut net_device, *mut netdev_bpf) -> ::core::ffi::c_int;

pub unsafe fn __netdev_put_lock(dev: *mut net_device, net: *mut net) -> *mut net_device { netdev_put_lock(dev, net, core::ptr::null_mut()) }

pub unsafe fn netif_set_up(dev: *mut net_device, value: bool) {
    if value { (*dev).flags |= IFF_UP; } else { (*dev).flags &= !IFF_UP; }
    if !netdev_need_ops_lock(dev) { netdev_lock(dev); }
    (*dev).up = value;
    if !netdev_need_ops_lock(dev) { netdev_unlock(dev); }
}

pub unsafe fn netif_set_gso_max_size(dev: *mut net_device, size: ::core::ffi::c_uint) {
    // dev->gso_max_size is read locklessly from sk_setup_caps()
    WRITE_ONCE(&mut (*dev).gso_max_size, size);
    if size <= GSO_LEGACY_MAX_SIZE { WRITE_ONCE(&mut (*dev).gso_ipv4_max_size, size); }
}
pub unsafe fn netif_set_gso_max_segs(dev: *mut net_device, segs: ::core::ffi::c_uint) { WRITE_ONCE(&mut (*dev).gso_max_segs, segs); }
pub unsafe fn netif_set_gro_max_size(dev: *mut net_device, size: ::core::ffi::c_uint) {
    WRITE_ONCE(&mut (*dev).gro_max_size, size);
    if size <= GRO_LEGACY_MAX_SIZE { WRITE_ONCE(&mut (*dev).gro_ipv4_max_size, size); }
}
pub unsafe fn netif_set_gso_ipv4_max_size(dev: *mut net_device, size: ::core::ffi::c_uint) { WRITE_ONCE(&mut (*dev).gso_ipv4_max_size, size); }
pub unsafe fn netif_set_gro_ipv4_max_size(dev: *mut net_device, size: ::core::ffi::c_uint) { WRITE_ONCE(&mut (*dev).gro_ipv4_max_size, size); }

pub unsafe fn napi_get_defer_hard_irqs(n: *const napi_struct) -> u32 { READ_ONCE(&(*n).defer_hard_irqs) }
pub unsafe fn napi_set_defer_hard_irqs(n: *mut napi_struct, defer: u32) { WRITE_ONCE(&mut (*n).defer_hard_irqs, defer); }
pub unsafe fn napi_get_gro_flush_timeout(n: *const napi_struct) -> ::core::ffi::c_ulong { READ_ONCE(&(*n).gro_flush_timeout) }
pub unsafe fn napi_set_gro_flush_timeout(n: *mut napi_struct, timeout: ::core::ffi::c_ulong) { WRITE_ONCE(&mut (*n).gro_flush_timeout, timeout); }
pub unsafe fn napi_get_irq_suspend_timeout(n: *const napi_struct) -> ::core::ffi::c_ulong { READ_ONCE(&(*n).irq_suspend_timeout) }
pub unsafe fn napi_set_irq_suspend_timeout(n: *mut napi_struct, timeout: ::core::ffi::c_ulong) { WRITE_ONCE(&mut (*n).irq_suspend_timeout, timeout); }

pub unsafe fn netdev_set_defer_hard_irqs(netdev: *mut net_device, defer: u32) {
    let count = core::cmp::max((*netdev).num_rx_queues, (*netdev).num_tx_queues);
    WRITE_ONCE(&mut (*netdev).napi_defer_hard_irqs, defer);
    // list_for_each_entry(napi, &netdev->napi_list, dev_list)
    for i in 0..count { (*netdev).napi_config.add(i as usize).as_mut().unwrap().defer_hard_irqs = defer; }
}

pub unsafe fn netdev_set_gro_flush_timeout(netdev: *mut net_device, timeout: ::core::ffi::c_ulong) {
    let count = core::cmp::max((*netdev).num_rx_queues, (*netdev).num_tx_queues);
    WRITE_ONCE(&mut (*netdev).gro_flush_timeout, timeout);
    // list_for_each_entry(napi, &netdev->napi_list, dev_list)
    for i in 0..count { (*netdev).napi_config.add(i as usize).as_mut().unwrap().gro_flush_timeout = timeout; }
}

pub unsafe fn napi_get_threaded(n: *mut napi_struct) -> netdev_napi_threaded {
    if test_bit(NAPI_STATE_THREADED_BUSY_POLL, &(*n).state) { return NETDEV_NAPI_THREADED_BUSY_POLL; }
    if test_bit(NAPI_STATE_THREADED, &(*n).state) { return NETDEV_NAPI_THREADED_ENABLED; }
    NETDEV_NAPI_THREADED_DISABLED
}
pub unsafe fn napi_get_threaded_config(dev: *mut net_device, n: *mut napi_struct) -> netdev_napi_threaded {
    if !(*n).config.is_null() { (*n).config.as_ref().unwrap().threaded } else { (*dev).threaded }
}

// netdev_for_each_altname and scoped lock iteration macros retain their C list/cleanup semantics.
pub unsafe fn napi_assert_will_not_race(napi: *const napi_struct) {
    if (*napi).poll_list.next.is_null() { return; }
    WARN_ON(!test_bit(NAPI_STATE_SCHED, &(*napi).state));
    WARN_ON(READ_ONCE(&(*napi).list_owner) != -1);
}

extern "C" {
    pub fn napi_set_threaded(n: *mut napi_struct, threaded: netdev_napi_threaded) -> ::core::ffi::c_int;
    pub fn netif_set_threaded(dev: *mut net_device, threaded: netdev_napi_threaded) -> ::core::ffi::c_int;
    pub fn rps_cpumask_housekeeping(mask: *mut cpumask) -> ::core::ffi::c_int;
    // Available only when CONFIG_DEBUG_NET && CONFIG_BPF_SYSCALL; otherwise this is an empty inline.
    pub fn xdp_do_check_flushed(napi: *mut napi_struct);
    pub fn kick_defer_list_purge(cpu: ::core::ffi::c_uint);
    pub fn dev_set_hwtstamp_phylib(dev: *mut net_device, cfg: *mut kernel_hwtstamp_config, extack: *mut netlink_ext_ack) -> ::core::ffi::c_int;
    pub fn dev_get_hwtstamp_phylib(dev: *mut net_device, cfg: *mut kernel_hwtstamp_config) -> ::core::ffi::c_int;
    pub fn net_hwtstamp_validate(cfg: *const kernel_hwtstamp_config) -> ::core::ffi::c_int;
}

// CONFIG_PROC_FS-disabled builds provide dev_proc_init() = 0.
// CONFIG_NET_SHAPER-disabled builds provide empty net_shaper helpers.

pub unsafe fn dev_isalive(dev: *const net_device) -> bool { READ_ONCE(&(*dev).reg_state) <= NETREG_REGISTERED }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
