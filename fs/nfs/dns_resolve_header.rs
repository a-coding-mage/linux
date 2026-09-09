/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Resolve DNS hostnames into valid ip addresses
 */

pub const NFS_DNS_HOSTNAME_MAXLEN: usize = 128;

/* Opaque types supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockaddr_storage {
    _private: [u8; 0],
}

#[cfg(CONFIG_NFS_USE_KERNEL_DNS)]
#[inline]
pub fn nfs_dns_resolver_init() -> i32 {
    0
}

#[cfg(CONFIG_NFS_USE_KERNEL_DNS)]
#[inline]
pub fn nfs_dns_resolver_destroy() {}

#[cfg(CONFIG_NFS_USE_KERNEL_DNS)]
#[inline]
pub fn nfs_dns_resolver_cache_init(_net: *mut net) -> i32 {
    0
}

#[cfg(CONFIG_NFS_USE_KERNEL_DNS)]
#[inline]
pub fn nfs_dns_resolver_cache_destroy(_net: *mut net) {}

#[cfg(not(CONFIG_NFS_USE_KERNEL_DNS))]
unsafe extern "C" {
    pub fn nfs_dns_resolver_init() -> i32;
    pub fn nfs_dns_resolver_destroy();
    pub fn nfs_dns_resolver_cache_init(net: *mut net) -> i32;
    pub fn nfs_dns_resolver_cache_destroy(net: *mut net);
}

unsafe extern "C" {
    pub fn nfs_dns_resolve_name(
        net: *mut net,
        name: *mut core::ffi::c_char,
        namelen: usize,
        sa: *mut sockaddr_storage,
        salen: usize,
    ) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
