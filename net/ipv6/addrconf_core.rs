// SPDX-License-Identifier: GPL-2.0-only
/*
 * IPv6 library code, needed by static components when full IPv6 support is
 * not configured or static.
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

#[repr(C)]
pub struct net;
#[repr(C)]
pub struct notifier_block;
#[repr(C)]
pub struct in6_addr {
    pub s6_addr32: [u32; 4],
}
#[repr(C)]
pub struct inet6_dev;
#[repr(C)]
pub struct rcu_head;
#[repr(C)]
pub struct net_device;

extern "C" {
    pub fn atomic_notifier_chain_register(chain: *mut c_void, nb: *mut notifier_block) -> i32;
    pub fn atomic_notifier_chain_unregister(chain: *mut c_void, nb: *mut notifier_block) -> i32;
    pub fn atomic_notifier_call_chain(chain: *mut c_void, val: usize, v: *mut c_void) -> i32;
    pub fn blocking_notifier_chain_register(chain: *mut c_void, nb: *mut notifier_block) -> i32;
    pub fn blocking_notifier_chain_unregister(chain: *mut c_void, nb: *mut notifier_block) -> i32;
    pub fn blocking_notifier_call_chain(chain: *mut c_void, val: usize, v: *mut c_void) -> i32;
    pub fn kfree(p: *mut c_void);
    pub fn free_percpu(p: *mut c_void);
    pub fn netdev_put(dev: *mut net_device, tracker: *mut c_void);
    pub fn call_rcu(head: *mut rcu_head, func: unsafe extern "C" fn(*mut rcu_head));
    pub fn list_empty(head: *const c_void) -> bool;
    pub fn rcu_access_pointer(p: *const c_void) -> *mut c_void;
    pub fn timer_pending(timer: *const c_void) -> bool;
    pub fn warn_on(condition: bool) -> bool;
}

pub const IPV6_ADDR_SCOPE_TYPE: u32 = 0x0001_0000;

// The following constants and macros are supplied by the IPv6 headers.
extern "C" {
    pub static mut inet6addr_chain: c_void;
    pub static mut inet6addr_validator_chain: c_void;
}

pub static mut __fib6_flush_trees: Option<unsafe extern "C" fn(*mut net)> = None;

#[inline]
unsafe fn ipv6_addr_scope2type(scope: u32) -> u32 {
    match scope {
        IPV6_ADDR_SCOPE_NODELOCAL => (scope << 16) | IPV6_ADDR_LOOPBACK,
        IPV6_ADDR_SCOPE_LINKLOCAL => (scope << 16) | IPV6_ADDR_LINKLOCAL,
        IPV6_ADDR_SCOPE_SITELOCAL => (scope << 16) | IPV6_ADDR_SITELOCAL,
        _ => scope << 16,
    }
}

pub unsafe fn __ipv6_addr_type(addr: *const in6_addr) -> i32 {
    let st = (*addr).s6_addr32[0];
    if (st & u32::from_be(0xE0000000)) != u32::from_be(0x00000000)
        && (st & u32::from_be(0xE0000000)) != u32::from_be(0xE0000000)
    {
        return (IPV6_ADDR_UNICAST | (IPV6_ADDR_SCOPE_GLOBAL << 16)) as i32;
    }
    if (st & u32::from_be(0xFF000000)) == u32::from_be(0xFF000000) {
        return (IPV6_ADDR_MULTICAST | ipv6_addr_scope2type(IPV6_ADDR_MC_SCOPE(addr))) as i32;
    }
    if (st & u32::from_be(0xFFC00000)) == u32::from_be(0xFE800000) {
        return (IPV6_ADDR_LINKLOCAL | IPV6_ADDR_UNICAST | (IPV6_ADDR_SCOPE_LINKLOCAL << 16)) as i32;
    }
    if (st & u32::from_be(0xFFC00000)) == u32::from_be(0xFEC00000) {
        return (IPV6_ADDR_SITELOCAL | IPV6_ADDR_UNICAST | (IPV6_ADDR_SCOPE_SITELOCAL << 16)) as i32;
    }
    if (st & u32::from_be(0xFE000000)) == u32::from_be(0xFC000000) {
        return (IPV6_ADDR_UNICAST | (IPV6_ADDR_SCOPE_GLOBAL << 16)) as i32;
    }
    if ((*addr).s6_addr32[0] | (*addr).s6_addr32[1]) == 0 {
        if (*addr).s6_addr32[2] == 0 {
            if (*addr).s6_addr32[3] == 0 { return IPV6_ADDR_ANY as i32; }
            if (*addr).s6_addr32[3] == u32::from_be(1) {
                return (IPV6_ADDR_LOOPBACK | IPV6_ADDR_UNICAST | (IPV6_ADDR_SCOPE_LINKLOCAL << 16)) as i32;
            }
            return (IPV6_ADDR_COMPATV4 | IPV6_ADDR_UNICAST | (IPV6_ADDR_SCOPE_GLOBAL << 16)) as i32;
        }
        if (*addr).s6_addr32[2] == u32::from_be(0x0000ffff) {
            return (IPV6_ADDR_MAPPED | (IPV6_ADDR_SCOPE_GLOBAL << 16)) as i32;
        }
    }
    (IPV6_ADDR_UNICAST | (IPV6_ADDR_SCOPE_GLOBAL << 16)) as i32
}

pub unsafe fn register_inet6addr_notifier(nb: *mut notifier_block) -> i32 { atomic_notifier_chain_register(&mut inet6addr_chain, nb) }
pub unsafe fn unregister_inet6addr_notifier(nb: *mut notifier_block) -> i32 { atomic_notifier_chain_unregister(&mut inet6addr_chain, nb) }
pub unsafe fn inet6addr_notifier_call_chain(val: usize, v: *mut c_void) -> i32 { atomic_notifier_call_chain(&mut inet6addr_chain, val, v) }
pub unsafe fn register_inet6addr_validator_notifier(nb: *mut notifier_block) -> i32 { blocking_notifier_chain_register(&mut inet6addr_validator_chain, nb) }
pub unsafe fn unregister_inet6addr_validator_notifier(nb: *mut notifier_block) -> i32 { blocking_notifier_chain_unregister(&mut inet6addr_validator_chain, nb) }
pub unsafe fn inet6addr_validator_notifier_call_chain(val: usize, v: *mut c_void) -> i32 { blocking_notifier_call_chain(&mut inet6addr_validator_chain, val, v) }

pub static in6addr_loopback: in6_addr = in6_addr { s6_addr32: [0, 0, 0, u32::from_be(1)] };
pub static in6addr_any: in6_addr = in6_addr { s6_addr32: [0, 0, 0, 0] };
pub static in6addr_linklocal_allnodes: in6_addr = in6_addr { s6_addr32: [u32::from_be(0xff020000), 0, 0, u32::from_be(1)] };
pub static in6addr_linklocal_allrouters: in6_addr = in6_addr { s6_addr32: [u32::from_be(0xff020000), 0, 0, u32::from_be(2)] };
pub static in6addr_interfacelocal_allnodes: in6_addr = in6_addr { s6_addr32: [u32::from_be(0xff010000), 0, 0, u32::from_be(1)] };
pub static in6addr_interfacelocal_allrouters: in6_addr = in6_addr { s6_addr32: [u32::from_be(0xff010000), 0, 0, u32::from_be(2)] };
pub static in6addr_sitelocal_allrouters: in6_addr = in6_addr { s6_addr32: [u32::from_be(0xff050000), 0, 0, u32::from_be(2)] };

unsafe fn snmp6_free_dev(idev: *mut inet6_dev) { let _ = idev; }
unsafe extern "C" fn in6_dev_finish_destroy_rcu(head: *mut rcu_head) { let _ = head; }
pub unsafe fn in6_dev_finish_destroy(idev: *mut inet6_dev) { let _ = idev; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
