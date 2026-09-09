/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/net/dsa_stubs.h - Stubs for the Distributed Switch Architecture framework
 */

/* Types and functions supplied by the corresponding kernel dependencies. */

#[cfg(config_net_dsa)]
extern "C" {
    pub static dsa_stubs: *const dsa_stubs;
}

#[cfg(config_net_dsa)]
#[repr(C)]
pub struct dsa_stubs {
    pub conduit_hwtstamp_validate: Option<
        unsafe extern "C" fn(
            dev: *mut net_device,
            config: *const kernel_hwtstamp_config,
            extack: *mut netlink_ext_ack,
        ) -> ::core::ffi::c_int,
    >,
}

#[cfg(config_net_dsa)]
#[inline]
pub unsafe fn dsa_conduit_hwtstamp_validate(
    dev: *mut net_device,
    config: *const kernel_hwtstamp_config,
    extack: *mut netlink_ext_ack,
) -> ::core::ffi::c_int {
    if !netdev_uses_dsa(dev) {
        return 0;
    }

    /* rtnl_lock() is a sufficient guarantee, because as long as
     * netdev_uses_dsa() returns true, the dsa_core module is still
     * registered, and so, dsa_unregister_stubs() couldn't have run.
     * For netdev_uses_dsa() to start returning false, it would imply
     * that dsa_conduit_teardown() has executed, which requires rtnl_lock().
     */
    /* ASSERT_RTNL(); */

    ((*dsa_stubs).conduit_hwtstamp_validate.unwrap())(dev, config, extack)
}

#[cfg(not(config_net_dsa))]
#[inline]
pub unsafe fn dsa_conduit_hwtstamp_validate(
    _dev: *mut net_device,
    _config: *const kernel_hwtstamp_config,
    _extack: *mut netlink_ext_ack,
) -> ::core::ffi::c_int {
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
