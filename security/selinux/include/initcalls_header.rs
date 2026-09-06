// SPDX-License-Identifier: GPL-2.0-only
/*
 * SELinux initcalls
 */

unsafe extern "C" {
    pub fn init_sel_fs() -> ::std::os::raw::c_int;
    pub fn sel_netport_init() -> ::std::os::raw::c_int;
    pub fn sel_netnode_init() -> ::std::os::raw::c_int;
    pub fn sel_netif_init() -> ::std::os::raw::c_int;
    pub fn sel_netlink_init() -> ::std::os::raw::c_int;
    pub fn sel_ib_pkey_init() -> ::std::os::raw::c_int;
    pub fn selinux_nf_ip_init() -> ::std::os::raw::c_int;

    pub fn selinux_initcall() -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
