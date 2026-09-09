/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 *  PNET table queries
 *
 *  Copyright IBM Corp. 2016
 *
 *  Author(s):  Thomas Richter <tmricht@linux.vnet.ibm.com>
 */

// Dependency declarations supplied by other translation units:
// <net/smc.h>, and <asm/pnet.h> when CONFIG_HAVE_PNETID is enabled.

#[repr(C)]
pub struct smc_ib_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smcd_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smc_init_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smc_link_group {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rwlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

pub type u8 = core::ffi::c_uchar;

pub const SMC_MAX_PNETID_LEN: usize = 16; // Supplied by <net/smc.h>.
pub const ENOENT: i32 = 2; // Supplied by the platform errno definitions.

/**
 * struct smc_pnettable - SMC PNET table anchor
 * @lock: Lock for list action
 * @pnetlist: List of PNETIDs
 */
#[repr(C)]
pub struct smc_pnettable {
    pub lock: mutex,
    pub pnetlist: list_head,
}

#[repr(C)]
pub struct smc_pnetids_ndev { /* list of pnetids for net devices in UP state */
    pub list: list_head,
    pub lock: rwlock_t,
}

#[repr(C)]
pub struct smc_pnetids_ndev_entry {
    pub list: list_head,
    pub pnetid: [u8; SMC_MAX_PNETID_LEN],
    pub refcnt: refcount_t,
}

#[inline]
pub unsafe fn smc_pnetid_by_dev_port(
    dev: *mut device,
    port: core::ffi::c_ushort,
    pnetid: *mut u8,
) -> i32 {
    // CONFIG_HAVE_PNETID is a build-time condition. When enabled, call the
    // external pnet_id_by_dev_port implementation; otherwise return -ENOENT.
    #[cfg(CONFIG_HAVE_PNETID)]
    {
        unsafe extern "C" {
            fn pnet_id_by_dev_port(
                dev: *mut device,
                port: core::ffi::c_ushort,
                pnetid: *mut u8,
            ) -> i32;
        }
        return pnet_id_by_dev_port(dev, port, pnetid);
    }
    #[cfg(not(CONFIG_HAVE_PNETID))]
    {
        let _ = (dev, port, pnetid);
        return -ENOENT;
    }
}

extern "C" {
    pub fn smc_pnet_init() -> i32; // __init
    pub fn smc_pnet_net_init(net: *mut net) -> i32;
    pub fn smc_pnet_exit();
    pub fn smc_pnet_net_exit(net: *mut net);
    pub fn smc_pnet_find_roce_resource(sk: *mut sock, ini: *mut smc_init_info);
    pub fn smc_pnet_find_ism_resource(sk: *mut sock, ini: *mut smc_init_info);
    pub fn smc_pnetid_by_table_ib(smcibdev: *mut smc_ib_device, ib_port: u8) -> i32;
    pub fn smc_pnetid_by_table_smcd(smcd: *mut smcd_dev) -> i32;
    pub fn smc_pnet_find_alt_roce(
        lgr: *mut smc_link_group,
        ini: *mut smc_init_info,
        known_dev: *mut smc_ib_device,
    );
    pub fn smc_pnet_is_ndev_pnetid(net: *mut net, pnetid: *mut u8) -> bool;
    pub fn smc_pnet_is_pnetid_set(pnetid: *mut u8) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
