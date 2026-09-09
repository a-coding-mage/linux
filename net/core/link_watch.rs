// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux network device link state notification
 *
 * Author:
 *     Stefan Rompf <sux@loplof.de>
 */

// C dependencies supplied by the surrounding kernel translation unit.

use core::ffi::c_void;

#[repr(C)]
pub struct work_struct { _private: [u8; 0] }
#[repr(C)]
pub struct delayed_work { _private: [u8; 0] }
#[repr(C)]
pub struct net_device { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { _private: [u8; 0] }
#[repr(C)]
pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)]
pub struct net_device_tracker { _private: [u8; 0] }

const LW_URGENT: usize = 0;
static mut linkwatch_flags: usize = 0;
static mut linkwatch_nextevent: usize = 0;

extern "C" {
    fn linkwatch_event(dummy: *mut work_struct);
    static mut linkwatch_work: delayed_work;
    static mut lweventlist: list_head;
    static mut lweventlist_lock: spinlock_t;
}

extern "C" {
    fn netif_testing(dev: *const net_device) -> bool;
    fn netif_carrier_ok(dev: *const net_device) -> bool;
    fn netif_dormant(dev: *const net_device) -> bool;
    fn dev_get_iflink(dev: *const net_device) -> i32;
    fn __dev_get_by_index(net: *mut c_void, iflink: i32) -> *mut net_device;
    fn dev_net(dev: *const net_device) -> *mut c_void;
    fn netif_running(dev: *const net_device) -> bool;
    fn netif_is_lag_port(dev: *const net_device) -> bool;
    fn netif_is_lag_master(dev: *const net_device) -> bool;
    fn qdisc_tx_changing(dev: *const net_device) -> bool;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn list_empty(list: *const list_head) -> bool;
    fn list_add_tail(node: *mut list_head, head: *mut list_head);
    fn list_del_init(node: *mut list_head);
    fn list_splice_init(src: *mut list_head, dst: *mut list_head);
    fn netdev_hold(dev: *mut net_device, tracker: *mut net_device_tracker, gfp: u32);
    fn netdev_tracker_free(dev: *mut net_device, tracker: *mut net_device_tracker);
    fn netdev_lock_ops(dev: *mut net_device);
    fn netdev_unlock_ops(dev: *mut net_device);
    fn __dev_put(dev: *mut net_device);
    fn netdev_assert_locked_ops_compat(dev: *mut net_device);
    fn dev_activate(dev: *mut net_device);
    fn dev_deactivate(dev: *mut net_device, sync: bool);
    fn netif_state_change(dev: *mut net_device);
    fn rtnl_lock();
    fn rtnl_unlock();
    fn test_bit(bit: usize, addr: *const usize) -> bool;
    fn test_and_set_bit(bit: usize, addr: *mut usize) -> bool;
    fn clear_bit(bit: usize, addr: *mut usize);
    fn smp_mb__before_atomic();
    fn mod_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: usize);
    fn queue_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: usize);
    static mut system_dfl_wq: *mut c_void;
    fn time_after(a: usize, b: usize) -> bool;
    fn jiffies() -> usize;
    static HZ: usize;
}

const IF_OPER_TESTING: u32 = 0;
const IF_OPER_DOWN: u32 = 0;
const IF_OPER_LOWERLAYERDOWN: u32 = 0;
const IF_OPER_DORMANT: u32 = 0;
const IF_OPER_UP: u32 = 0;
const IF_LINK_MODE_TESTING: u8 = 0;
const IF_LINK_MODE_DORMANT: u8 = 0;
const IF_LINK_MODE_DEFAULT: u8 = 0;
const NETREG_REGISTERED: u8 = 0;
const IFF_UP: u32 = 0;
const __LINK_STATE_LINKWATCH_PENDING: usize = 0;
const GFP_ATOMIC: u32 = 0;

// Device fields are supplied by the kernel's net_device definition.
extern "C" {
    fn netdev_reg_state(dev: *const net_device) -> u8;
    fn netdev_ifindex(dev: *const net_device) -> i32;
    fn netdev_operstate(dev: *const net_device) -> u32;
    fn netdev_link_mode(dev: *const net_device) -> u8;
    fn netdev_flags(dev: *const net_device) -> u32;
    fn netdev_state(dev: *mut net_device) -> *mut usize;
    fn netdev_link_watch_list(dev: *mut net_device) -> *mut list_head;
    fn netdev_linkwatch_tracker(dev: *mut net_device) -> *mut net_device_tracker;
    fn netif_device_present(dev: *const net_device) -> bool;
}

unsafe fn default_operstate(dev: *const net_device) -> u32 {
    if netif_testing(dev) { return IF_OPER_TESTING; }
    if !netif_carrier_ok(dev) {
        let iflink = if netdev_reg_state(dev) <= NETREG_REGISTERED {
            dev_get_iflink(dev)
        } else { netdev_ifindex(dev) };
        if iflink == netdev_ifindex(dev) { return IF_OPER_DOWN; }
        let peer = __dev_get_by_index(dev_net(dev), iflink);
        if peer.is_null() { return IF_OPER_DOWN; }
        return if netif_carrier_ok(peer) { IF_OPER_DOWN } else { IF_OPER_LOWERLAYERDOWN };
    }
    if netif_dormant(dev) { return IF_OPER_DORMANT; }
    IF_OPER_UP
}

unsafe fn rfc2863_policy(dev: *mut net_device) {
    let mut operstate = default_operstate(dev);
    if operstate == netdev_operstate(dev) { return; }
    match netdev_link_mode(dev) {
        IF_LINK_MODE_TESTING if operstate == IF_OPER_UP => operstate = IF_OPER_TESTING,
        IF_LINK_MODE_DORMANT if operstate == IF_OPER_UP => operstate = IF_OPER_DORMANT,
        IF_LINK_MODE_DEFAULT | _ => (),
    }
    // WRITE_ONCE(dev->operstate, operstate)
    let _ = operstate;
}

pub unsafe fn linkwatch_init_dev(dev: *mut net_device) {
    if !netif_carrier_ok(dev) || netif_dormant(dev) || netif_testing(dev) { rfc2863_policy(dev); }
}

unsafe fn linkwatch_urgent_event(dev: *mut net_device) -> bool {
    if !netif_running(dev) { return false; }
    if netdev_ifindex(dev) != dev_get_iflink(dev) { return true; }
    if netif_is_lag_port(dev) || netif_is_lag_master(dev) { return true; }
    netif_carrier_ok(dev) && qdisc_tx_changing(dev)
}

unsafe fn linkwatch_add_event(dev: *mut net_device) {
    let mut flags = 0usize;
    spin_lock_irqsave(&mut lweventlist_lock, &mut flags);
    let list = netdev_link_watch_list(dev);
    if list_empty(list) {
        list_add_tail(list, &mut lweventlist);
        netdev_hold(dev, netdev_linkwatch_tracker(dev), GFP_ATOMIC);
    }
    spin_unlock_irqrestore(&mut lweventlist_lock, flags);
}

unsafe fn linkwatch_schedule_work(urgent: i32) {
    let mut delay = linkwatch_nextevent.wrapping_sub(jiffies());
    if test_bit(LW_URGENT, &linkwatch_flags) { return; }
    if urgent != 0 {
        if test_and_set_bit(LW_URGENT, &mut linkwatch_flags) { return; }
        delay = 0;
    }
    if delay > HZ { delay = 0; }
    if test_bit(LW_URGENT, &linkwatch_flags) { mod_delayed_work(system_dfl_wq, &mut linkwatch_work, 0); }
    else { queue_delayed_work(system_dfl_wq, &mut linkwatch_work, delay); }
}

unsafe fn linkwatch_do_dev(dev: *mut net_device) {
    smp_mb__before_atomic();
    clear_bit(__LINK_STATE_LINKWATCH_PENDING, netdev_state(dev));
    rfc2863_policy(dev);
    if netdev_flags(dev) & IFF_UP != 0 {
        if netif_carrier_ok(dev) { dev_activate(dev); } else { dev_deactivate(dev, true); }
        netif_state_change(dev);
    }
}

unsafe fn __linkwatch_run_queue(urgent_only: i32) {
    let mut do_dev = 100;
    let mut wrk = list_head { _private: [] };
    if urgent_only != 0 { do_dev += 100; }
    if urgent_only == 0 { linkwatch_nextevent = jiffies().wrapping_add(HZ); }
    else if time_after(linkwatch_nextevent, jiffies().wrapping_add(HZ)) { linkwatch_nextevent = jiffies(); }
    clear_bit(LW_URGENT, &mut linkwatch_flags);
    spin_lock_irq(&mut lweventlist_lock);
    list_splice_init(&mut lweventlist, &mut wrk);
    while !list_empty(&wrk) && do_dev > 0 {
        // list_first_entry(&wrk, struct net_device, link_watch_list)
        let dev = core::ptr::null_mut::<net_device>();
        list_del_init(netdev_link_watch_list(dev));
        if !netif_device_present(dev) || (urgent_only != 0 && !linkwatch_urgent_event(dev)) {
            list_add_tail(netdev_link_watch_list(dev), &mut lweventlist); continue;
        }
        netdev_tracker_free(dev, netdev_linkwatch_tracker(dev));
        spin_unlock_irq(&mut lweventlist_lock);
        netdev_lock_ops(dev); linkwatch_do_dev(dev); netdev_unlock_ops(dev); __dev_put(dev);
        do_dev -= 1; spin_lock_irq(&mut lweventlist_lock);
    }
    list_splice_init(&mut wrk, &mut lweventlist);
    if !list_empty(&lweventlist) { linkwatch_schedule_work(0); }
    spin_unlock_irq(&mut lweventlist_lock);
}

unsafe fn linkwatch_clean_dev(dev: *mut net_device) -> bool {
    let mut flags = 0usize; let mut clean = false;
    spin_lock_irqsave(&mut lweventlist_lock, &mut flags);
    let list = netdev_link_watch_list(dev);
    if !list_empty(list) { list_del_init(list); clean = true; netdev_tracker_free(dev, netdev_linkwatch_tracker(dev)); }
    spin_unlock_irqrestore(&mut lweventlist_lock, flags); clean
}

pub unsafe fn __linkwatch_sync_dev(dev: *mut net_device) {
    netdev_assert_locked_ops_compat(dev);
    if linkwatch_clean_dev(dev) { linkwatch_do_dev(dev); __dev_put(dev); }
}

pub unsafe fn linkwatch_sync_dev(dev: *mut net_device) {
    if linkwatch_clean_dev(dev) { netdev_lock_ops(dev); linkwatch_do_dev(dev); netdev_unlock_ops(dev); __dev_put(dev); }
}

pub unsafe fn linkwatch_run_queue() { __linkwatch_run_queue(0); }

unsafe fn linkwatch_event_impl(_dummy: *mut work_struct) {
    rtnl_lock(); __linkwatch_run_queue(if time_after(linkwatch_nextevent, jiffies()) { 1 } else { 0 }); rtnl_unlock();
}

pub unsafe fn linkwatch_fire_event(dev: *mut net_device) {
    let urgent = linkwatch_urgent_event(dev);
    if !test_and_set_bit(__LINK_STATE_LINKWATCH_PENDING, netdev_state(dev)) { linkwatch_add_event(dev); }
    else if !urgent { return; }
    linkwatch_schedule_work(if urgent { 1 } else { 0 });
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
