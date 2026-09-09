/* SPDX-License-Identifier: GPL-2.0 */
/*
 * can in net namespaces
 */

// Translated from the C header. The CONFIG_PROC_FS condition is represented
// by the corresponding Rust feature when available.

#[repr(C)]
pub struct can_dev_rcv_lists {
    _private: [u8; 0],
}

#[repr(C)]
pub struct can_pkg_stats {
    _private: [u8; 0],
}

#[repr(C)]
pub struct can_rcv_lists_stats {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hlist_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netns_can {
    #[cfg(feature = "CONFIG_PROC_FS")]
    pub proc_dir: *mut proc_dir_entry,
    #[cfg(feature = "CONFIG_PROC_FS")]
    pub pde_stats: *mut proc_dir_entry,
    #[cfg(feature = "CONFIG_PROC_FS")]
    pub pde_reset_stats: *mut proc_dir_entry,
    #[cfg(feature = "CONFIG_PROC_FS")]
    pub pde_rcvlist_all: *mut proc_dir_entry,
    #[cfg(feature = "CONFIG_PROC_FS")]
    pub pde_rcvlist_fil: *mut proc_dir_entry,
    #[cfg(feature = "CONFIG_PROC_FS")]
    pub pde_rcvlist_inv: *mut proc_dir_entry,
    #[cfg(feature = "CONFIG_PROC_FS")]
    pub pde_rcvlist_sff: *mut proc_dir_entry,
    #[cfg(feature = "CONFIG_PROC_FS")]
    pub pde_rcvlist_eff: *mut proc_dir_entry,
    #[cfg(feature = "CONFIG_PROC_FS")]
    pub pde_rcvlist_err: *mut proc_dir_entry,
    #[cfg(feature = "CONFIG_PROC_FS")]
    pub bcmproc_dir: *mut proc_dir_entry,

    /* receive filters subscribed for 'all' CAN devices */
    pub rx_alldev_list: *mut can_dev_rcv_lists,
    pub rcvlists_lock: spinlock_t,
    pub stattimer: timer_list, /* timer for statistics update */
    pub pkg_stats: *mut can_pkg_stats,
    pub rcv_lists_stats: *mut can_rcv_lists_stats,

    /* CAN GW per-net gateway jobs */
    pub cgw_list: hlist_head,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
