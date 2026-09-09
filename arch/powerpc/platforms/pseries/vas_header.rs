/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2020-21 IBM Corp.
 */

// Dependencies supplied by the surrounding kernel translation:
// <asm/vas.h>, <linux/mutex.h>, and <linux/stringify.h>

/* VAS window modify flags */
pub const VAS_MOD_WIN_CLOSE: u64 = PPC_BIT(0);
pub const VAS_MOD_WIN_JOBS_KILL: u64 = PPC_BIT(1);
pub const VAS_MOD_WIN_DR: u64 = PPC_BIT(3);
pub const VAS_MOD_WIN_PR: u64 = PPC_BIT(4);
pub const VAS_MOD_WIN_SF: u64 = PPC_BIT(5);
pub const VAS_MOD_WIN_TA: u64 = PPC_BIT(6);
pub const VAS_MOD_WIN_FLAGS: u64 = VAS_MOD_WIN_JOBS_KILL | VAS_MOD_WIN_DR | VAS_MOD_WIN_PR | VAS_MOD_WIN_SF;

pub const VAS_WIN_ACTIVE: u32 = 0x0;
pub const VAS_WIN_CLOSED: u32 = 0x1;
pub const VAS_WIN_INACTIVE: u32 = 0x2; /* Inactive due to HW failure */
/* Process of being modified, deallocated, or quiesced */
pub const VAS_WIN_MOD_IN_PROCESS: u32 = 0x3;

pub const VAS_COPY_PASTE_USER_MODE: u32 = 0x00000001;
pub const VAS_COP_OP_USER_MODE: u32 = 0x00000010;

pub const VAS_GZIP_QOS_CAPABILITIES: u64 = 0x56516F73477A6970;
pub const VAS_GZIP_DEFAULT_CAPABILITIES: u64 = 0x56446566477A6970;

#[repr(C)]
pub enum vas_migrate_action {
    VAS_SUSPEND,
    VAS_RESUME,
}

#[repr(C)]
pub enum vas_cop_feat_type {
    VAS_GZIP_QOS_FEAT_TYPE,
    VAS_GZIP_DEF_FEAT_TYPE,
    VAS_MAX_FEAT_TYPE,
}

#[repr(C, align(4096))]
pub struct hv_vas_cop_feat_caps {
    pub descriptor: __be64,
    pub win_type: u8,
    pub user_mode: u8,
    pub max_lpar_creds: __be16,
    pub max_win_creds: __be16,
    pub creds: hv_vas_cop_feat_caps__creds,
    pub target_lpar_creds: __be16,
}

#[repr(C)]
pub union hv_vas_cop_feat_caps__creds {
    pub reserved: __be16,
    pub def_lpar_creds: __be16,
}

#[repr(C)]
pub struct vas_cop_feat_caps {
    pub descriptor: u64,
    pub win_type: u8,
    pub user_mode: u8,
    pub max_lpar_creds: u16,
    pub max_win_creds: u16,
    pub creds: vas_cop_feat_caps__creds,
    pub nr_total_credits: atomic_t,
    pub nr_used_credits: atomic_t,
}

#[repr(C)]
pub union vas_cop_feat_caps__creds {
    pub reserved: u16,
    pub def_lpar_creds: u16,
}

#[repr(C)]
pub struct vas_caps {
    pub caps: vas_cop_feat_caps,
    pub list: list_head,
    pub nr_open_wins_progress: i32,
    pub nr_close_wins: i32,
    pub nr_open_windows: i32,
    pub feat: u8,
}

#[repr(C, align(4096))]
pub struct hv_vas_win_lpar {
    pub version: __be16,
    pub win_type: u8,
    pub status: u8,
    pub credits: __be16,
    pub reserved: __be16,
    pub pid: __be32,
    pub tid: __be32,
    pub win_addr: __be64,
    pub interrupt: __be32,
    pub fault: __be32,
    pub domain: [__be64; 6],
    pub win_util: __be64,
}

#[repr(C)]
pub struct pseries_vas_window {
    pub vas_win: vas_window,
    pub win_addr: u64,
    pub win_type: u8,
    pub complete_irq: u32,
    pub fault_irq: u32,
    pub domain: [u64; 6],
    pub util: u64,
    pub pid: u32,
    pub win_list: list_head,
    pub flags: u64,
    pub name: *mut core::ffi::c_char,
    pub fault_virq: i32,
    pub pending_faults: atomic_t,
}

unsafe extern "C" {
    pub fn sysfs_add_vas_caps(caps: *mut vas_cop_feat_caps) -> i32;
    pub fn vas_reconfig_capabilties(type_: u8, new_nr_creds: i32) -> i32;
    pub fn sysfs_pseries_vas_init(vas_caps: *mut vas_all_caps) -> i32;
}

#[cfg(feature = "CONFIG_PPC_VAS")]
unsafe extern "C" {
    pub fn vas_migration_handler(action: i32) -> i32;
    pub fn pseries_vas_dlpar_cpu() -> i32;
}

#[cfg(not(feature = "CONFIG_PPC_VAS"))]
#[inline]
pub unsafe fn vas_migration_handler(_action: i32) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_PPC_VAS"))]
#[inline]
pub unsafe fn pseries_vas_dlpar_cpu() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
