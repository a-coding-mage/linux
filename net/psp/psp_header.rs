/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/list.h, linux/lockdep.h, linux/mutex.h, net/netns/generic.h,
// net/psp.h, and net/sock.h.

extern "C" {
    pub static mut psp_devs: xarray;
    pub static mut psp_devs_lock: mutex;

    pub fn psp_dev_free(psd: *mut psp_dev);
    pub fn psp_dev_check_access(psd: *mut psp_dev, net: *mut net, admin: bool) -> i32;
    pub fn psp_has_assoc_dev_in_ns(psd: *mut psp_dev, net: *mut net) -> bool;
    pub fn psp_attach_netdev_notifier() -> i32;

    pub fn psp_nl_notify_dev(psd: *mut psp_dev, cmd: u32);

    pub fn psp_assoc_create(psd: *mut psp_dev) -> *mut psp_assoc;
    pub fn psp_dev_get_for_sock(sk: *mut sock) -> *mut psp_dev;
    pub fn psp_dev_tx_key_del(psd: *mut psp_dev, pas: *mut psp_assoc);
    pub fn psp_sock_assoc_set_rx(
        sk: *mut sock,
        pas: *mut psp_assoc,
        key: *mut psp_key_parsed,
        extack: *mut netlink_ext_ack,
    ) -> i32;
    pub fn psp_sock_assoc_set_tx(
        sk: *mut sock,
        psd: *mut psp_dev,
        version: u32,
        key: *mut psp_key_parsed,
        extack: *mut netlink_ext_ack,
    ) -> i32;
    pub fn psp_assocs_key_rotated(psd: *mut psp_dev);
}

#[repr(C)]
pub struct psp_dev {
    pub refcnt: refcount_t,
    pub lock: lockdep_map,
    pub ops: *const core::ffi::c_void,
}

extern "C" {
    pub fn refcount_inc(refcnt: *mut refcount_t);
    pub fn refcount_inc_not_zero(refcnt: *mut refcount_t) -> bool;
    pub fn refcount_dec_and_test(refcnt: *mut refcount_t) -> bool;
    pub fn lockdep_assert_held(lock: *mut lockdep_map);
}

#[inline]
pub unsafe fn psp_dev_get(psd: *mut psp_dev) {
    refcount_inc(&mut (*psd).refcnt);
}

#[inline]
pub unsafe fn psp_dev_tryget(psd: *mut psp_dev) -> bool {
    refcount_inc_not_zero(&mut (*psd).refcnt)
}

#[inline]
pub unsafe fn psp_dev_put(psd: *mut psp_dev) {
    if refcount_dec_and_test(&mut (*psd).refcnt) {
        psp_dev_free(psd);
    }
}

#[inline]
pub unsafe fn psp_dev_is_registered(psd: *mut psp_dev) -> bool {
    lockdep_assert_held(&mut (*psd).lock);
    !(*psd).ops.is_null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
