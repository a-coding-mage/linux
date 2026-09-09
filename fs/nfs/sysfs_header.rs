/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2019 Hammerspace Inc
 */

// C header guard: __NFS_SYSFS_H

pub const CONTAINER_ID_MAXLEN: i32 = 64;

// These types are supplied by the surrounding kernel/NFS dependencies.
pub enum kobject {}
pub enum net {}
pub enum nfs_net {}
pub enum nfs_server {}
pub enum rpc_clnt {}
pub enum super_block {}

#[repr(C)]
pub struct nfs_netns_client {
    pub kobject: kobject,
    pub nfs_net_kobj: kobject,
    pub net: *mut net,
    // const char __rcu *identifier;
    pub identifier: *const core::ffi::c_char,
}

extern "C" {
    pub static mut nfs_net_kobj: *mut kobject;

    pub fn nfs_sysfs_init() -> core::ffi::c_int;
    pub fn nfs_sysfs_exit();

    pub fn nfs_netns_sysfs_setup(netns: *mut nfs_net, net: *mut net);
    pub fn nfs_netns_sysfs_destroy(netns: *mut nfs_net);

    pub fn nfs_sysfs_link_rpc_client(
        server: *mut nfs_server,
        clnt: *mut rpc_clnt,
        sysfs_prefix: *const core::ffi::c_char,
    );
    pub fn nfs_sysfs_add_server(s: *mut nfs_server);
    pub fn nfs_sysfs_move_server_to_sb(s: *mut super_block);
    pub fn nfs_sysfs_move_sb_to_server(s: *mut nfs_server);
    pub fn nfs_sysfs_remove_server(s: *mut nfs_server);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
