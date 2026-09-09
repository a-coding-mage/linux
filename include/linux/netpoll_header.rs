/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common code for low-level network console, dump, and debugger code
 *
 * Derived from netconsole, kgdb-over-ethernet, and netdump patches
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub union inet_addr {
    pub ip: __be32,
    pub in6: in6_addr,
}

#[repr(C)]
pub struct netpoll {
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    /*
     * Either dev_name or dev_mac can be used to specify the local
     * interface - dev_name is used if it is a nonempty string, else
     * dev_mac is used.
     */
    pub dev_name: [c_char; IFNAMSIZ],
    pub dev_mac: [u8; ETH_ALEN],
    pub name: *const c_char,
}

#[macro_export]
macro_rules! np_info {
    ($np:expr, $fmt:expr $(, $args:expr)*) => {
        pr_info!(concat!("%s: ", $fmt), $np.name, $($args),*)
    };
}

#[macro_export]
macro_rules! np_err {
    ($np:expr, $fmt:expr $(, $args:expr)*) => {
        pr_err!(concat!("%s: ", $fmt), $np.name, $($args),*)
    };
}

#[macro_export]
macro_rules! np_notice {
    ($np:expr, $fmt:expr $(, $args:expr)*) => {
        pr_notice!(concat!("%s: ", $fmt), $np.name, $($args),*)
    };
}

#[repr(C)]
pub struct netpoll_info {
    pub refcnt: refcount_t,
    pub dev_lock: semaphore,
    pub txq: sk_buff_head,
    pub tx_work: delayed_work,
    pub rcu: rcu_head,
}

#[cfg(CONFIG_NETPOLL)]
extern "C" {
    pub fn netpoll_poll_dev(dev: *mut net_device);
    pub fn netpoll_poll_disable(dev: *mut net_device);
    pub fn netpoll_poll_enable(dev: *mut net_device);
}

#[cfg(not(CONFIG_NETPOLL))]
#[inline]
pub unsafe fn netpoll_poll_disable(_dev: *mut net_device) {}

#[cfg(not(CONFIG_NETPOLL))]
#[inline]
pub unsafe fn netpoll_poll_enable(_dev: *mut net_device) {}

extern "C" {
    pub fn __netpoll_setup(np: *mut netpoll, ndev: *mut net_device) -> c_int;
    pub fn __netpoll_free(np: *mut netpoll);
    pub fn netpoll_cleanup(np: *mut netpoll);
    pub fn do_netpoll_cleanup(np: *mut netpoll);
    pub fn netpoll_send_skb(np: *mut netpoll, skb: *mut sk_buff) -> netdev_tx_t;
    pub fn netpoll_zap_completion_queue();
    pub fn netpoll_get_carrier_timeout() -> c_uint;
}

#[cfg(CONFIG_NETPOLL)]
#[inline]
pub unsafe fn netpoll_poll_lock(napi: *mut napi_struct) -> *mut c_void {
    let dev = (*napi).dev;
    if !dev.is_null() && !rcu_access_pointer((*dev).npinfo).is_null() {
        let owner: c_int = smp_processor_id();
        while cmpxchg(&mut (*napi).poll_owner, -1, owner) != -1 {
            cpu_relax();
        }
        napi.cast()
    } else {
        core::ptr::null_mut()
    }
}

#[cfg(CONFIG_NETPOLL)]
#[inline]
pub unsafe fn netpoll_poll_unlock(have: *mut c_void) {
    let napi = have.cast::<napi_struct>();
    if !napi.is_null() {
        smp_store_release(&mut (*napi).poll_owner, -1);
    }
}

#[cfg(CONFIG_NETPOLL)]
#[inline]
pub unsafe fn netpoll_tx_running(_dev: *mut net_device) -> bool {
    irqs_disabled()
}

#[cfg(not(CONFIG_NETPOLL))]
#[inline]
pub unsafe fn netpoll_poll_lock(_napi: *mut napi_struct) -> *mut c_void {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_NETPOLL))]
#[inline]
pub unsafe fn netpoll_poll_unlock(_have: *mut c_void) {}

#[cfg(not(CONFIG_NETPOLL))]
#[inline]
pub unsafe fn netpoll_tx_running(_dev: *mut net_device) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
