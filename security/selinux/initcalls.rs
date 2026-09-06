// SPDX-License-Identifier: GPL-2.0-only
/*
 * SELinux initcalls
 */

/* C dependencies:
 * #include <linux/init.h>
 * #include "initcalls.h"
 */

unsafe extern "C" {
    fn init_sel_fs() -> core::ffi::c_int;
    fn sel_netport_init() -> core::ffi::c_int;
    fn sel_netnode_init() -> core::ffi::c_int;
    fn sel_netif_init() -> core::ffi::c_int;
    fn sel_netlink_init() -> core::ffi::c_int;

    /* CONFIG_SECURITY_INFINIBAND */
    fn sel_ib_pkey_init() -> core::ffi::c_int;

    /* CONFIG_NETFILTER */
    fn selinux_nf_ip_init() -> core::ffi::c_int;
}

/**
 * selinux_initcall - Perform the SELinux initcalls
 *
 * Used as a device initcall in the SELinux LSM definition.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn selinux_initcall() -> core::ffi::c_int {
    let mut rc: core::ffi::c_int = 0;
    let mut rc_tmp: core::ffi::c_int;

    rc_tmp = unsafe { init_sel_fs() };
    if rc == 0 && rc_tmp != 0 {
        rc = rc_tmp;
    }

    rc_tmp = unsafe { sel_netport_init() };
    if rc == 0 && rc_tmp != 0 {
        rc = rc_tmp;
    }

    rc_tmp = unsafe { sel_netnode_init() };
    if rc == 0 && rc_tmp != 0 {
        rc = rc_tmp;
    }

    rc_tmp = unsafe { sel_netif_init() };
    if rc == 0 && rc_tmp != 0 {
        rc = rc_tmp;
    }

    rc_tmp = unsafe { sel_netlink_init() };
    if rc == 0 && rc_tmp != 0 {
        rc = rc_tmp;
    }

    /* C conditional: #if defined(CONFIG_SECURITY_INFINIBAND) */
    #[cfg(CONFIG_SECURITY_INFINIBAND)]
    {
        rc_tmp = unsafe { sel_ib_pkey_init() };
        if rc == 0 && rc_tmp != 0 {
            rc = rc_tmp;
        }
    }

    /* C conditional: #if defined(CONFIG_NETFILTER) */
    #[cfg(CONFIG_NETFILTER)]
    {
        rc_tmp = unsafe { selinux_nf_ip_init() };
        if rc == 0 && rc_tmp != 0 {
            rc = rc_tmp;
        }
    }

    rc
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
