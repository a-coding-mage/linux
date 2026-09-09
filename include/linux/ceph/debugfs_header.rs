/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: linux/ceph/types.h

/* debugfs.c */
extern "C" {
    pub fn ceph_debugfs_init();
    pub fn ceph_debugfs_cleanup();
    pub fn ceph_debugfs_client_init(client: *mut ceph_client);
    pub fn ceph_debugfs_client_cleanup(client: *mut ceph_client);
}

// Opaque declaration supplied by linux/ceph/types.h.
#[repr(C)]
pub struct ceph_client {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
