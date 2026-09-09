/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * mtip32xx.h - Header file for the P320 SSD Block Driver
 *   Copyright (C) 2011 Micron Technology, Inc.
 *
 * Portions of this code were derived from works subjected to the
 * following copyright:
 *    Copyright (C) 2009 Integrated Device Technology, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const PCI_SUBSYSTEM_DEVICEID: u32 = 0x2E;
pub const PCIE_CONFIG_EXT_DEVICE_CONTROL_OFFSET: u32 = 0x48;
pub const MTIP_SEC_ERASE_MODE: u32 = 0x2;
pub const MTIP_MAX_RETRIES: u32 = 2;
pub const MTIP_NCQ_CMD_TIMEOUT_MS: u32 = 15000;
pub const MTIP_IOCTL_CMD_TIMEOUT_MS: u32 = 5000;
pub const MTIP_INT_CMD_TIMEOUT_MS: u32 = 5000;
pub const MTIP_QUIESCE_IO_TIMEOUT_MS: u32 = MTIP_NCQ_CMD_TIMEOUT_MS * (MTIP_MAX_RETRIES + 1);
pub const MTIP_TIMEOUT_CHECK_PERIOD: u32 = 500;
pub const MTIP_FTL_REBUILD_OFFSET: u32 = 142;
pub const MTIP_FTL_REBUILD_MAGIC: u32 = 0xED51;
pub const MTIP_FTL_REBUILD_TIMEOUT_MS: u32 = 2400000;
pub const MTIP_MAX_UNALIGNED_SLOTS: u32 = 2;

#[inline]
pub const fn MTIP_TAG_BIT(tag: u32) -> u32 { tag & 0x1F }
#[inline]
pub const fn MTIP_TAG_INDEX(tag: u32) -> u32 { tag >> 5 }

pub const MTIP_MAX_SG: usize = 504;
pub const MTIP_MAX_SLOT_GROUPS: usize = 8;
pub const MTIP_TAG_INTERNAL: u32 = 0;
pub const PCI_VENDOR_ID_MICRON: u32 = 0x1344;
pub const P320H_DEVICE_ID: u32 = 0x5150;
pub const P320M_DEVICE_ID: u32 = 0x5151;
pub const P320S_DEVICE_ID: u32 = 0x5152;
pub const P325M_DEVICE_ID: u32 = 0x5153;
pub const P420H_DEVICE_ID: u32 = 0x5160;
pub const P420M_DEVICE_ID: u32 = 0x5161;
pub const P425M_DEVICE_ID: u32 = 0x5163;
pub const MTIP_DRV_NAME: &str = "mtip32xx";
pub const MTIP_DRV_VERSION: &str = "1.3.1";
pub const MTIP_MAX_MINORS: u32 = 16;
pub const MTIP_MAX_COMMAND_SLOTS: usize = MTIP_MAX_SLOT_GROUPS * 32;
pub const U32_PER_LONG: usize = core::mem::size_of::<c_ulong>() / core::mem::size_of::<u32>();
pub const SLOTBITS_IN_LONGS: usize = (MTIP_MAX_SLOT_GROUPS + (U32_PER_LONG - 1)) / U32_PER_LONG;
pub const MTIP_ABAR: u32 = 5;
pub const MTIP_DFS_MAX_BUF_SIZE: usize = 1024;

pub const MTIP_PF_IC_ACTIVE_BIT: u32 = 0;
pub const MTIP_PF_EH_ACTIVE_BIT: u32 = 1;
pub const MTIP_PF_SE_ACTIVE_BIT: u32 = 2;
pub const MTIP_PF_DM_ACTIVE_BIT: u32 = 3;
pub const MTIP_PF_TO_ACTIVE_BIT: u32 = 9;
pub const MTIP_PF_PAUSE_IO: u32 = (1 << MTIP_PF_IC_ACTIVE_BIT) | (1 << MTIP_PF_EH_ACTIVE_BIT) |
    (1 << MTIP_PF_SE_ACTIVE_BIT) | (1 << MTIP_PF_DM_ACTIVE_BIT) | (1 << MTIP_PF_TO_ACTIVE_BIT);
pub const MTIP_PF_HOST_CAP_64: u32 = 10;
pub const MTIP_PF_SVC_THD_ACTIVE_BIT: u32 = 4;
pub const MTIP_PF_ISSUE_CMDS_BIT: u32 = 5;
pub const MTIP_PF_REBUILD_BIT: u32 = 6;
pub const MTIP_PF_SVC_THD_STOP_BIT: u32 = 8;
pub const MTIP_PF_SVC_THD_WORK: u32 = (1 << MTIP_PF_EH_ACTIVE_BIT) | (1 << MTIP_PF_ISSUE_CMDS_BIT) |
    (1 << MTIP_PF_REBUILD_BIT) | (1 << MTIP_PF_SVC_THD_STOP_BIT) | (1 << MTIP_PF_TO_ACTIVE_BIT);
pub const MTIP_DDF_SEC_LOCK_BIT: u32 = 0;
pub const MTIP_DDF_REMOVE_PENDING_BIT: u32 = 1;
pub const MTIP_DDF_OVER_TEMP_BIT: u32 = 2;
pub const MTIP_DDF_WRITE_PROTECT_BIT: u32 = 3;
pub const MTIP_DDF_CLEANUP_BIT: u32 = 5;
pub const MTIP_DDF_RESUME_BIT: u32 = 6;
pub const MTIP_DDF_INIT_DONE_BIT: u32 = 7;
pub const MTIP_DDF_REBUILD_FAILED_BIT: u32 = 8;
pub const MTIP_DDF_STOP_IO: u32 = (1 << MTIP_DDF_REMOVE_PENDING_BIT) | (1 << MTIP_DDF_SEC_LOCK_BIT) |
    (1 << MTIP_DDF_OVER_TEMP_BIT) | (1 << MTIP_DDF_WRITE_PROTECT_BIT) | (1 << MTIP_DDF_REBUILD_FAILED_BIT);

#[repr(C, packed)]
pub struct smart_attr { pub attr_id: u8, pub flags: __le16, pub cur: u8, pub worst: u8, pub data: __le32, pub res: [u8; 3] }

#[repr(C)]
pub struct mtip_work { pub work: work_struct, pub port: *mut core::ffi::c_void, pub cpu_binding: i32, pub completed: u32 }

#[macro_export]
macro_rules! DEFINE_HANDLER {
    ($group:ident) => {
        pub unsafe extern "C" fn mtip_workq_sdbf$group(work: *mut work_struct) {
            let w = work as *mut mtip_work;
            mtip_workq_sdbfx((*w).port, $group, (*w).completed);
        }
    };
}

#[repr(C)]
pub union host_to_dev_fis_lba_low { pub lba_low: u8, pub sector: u8 }
#[repr(C)]
pub union host_to_dev_fis_lba_mid { pub lba_mid: u8, pub cyl_low: u8 }
#[repr(C)]
pub union host_to_dev_fis_lba_hi { pub lba_hi: u8, pub cyl_hi: u8 }
#[repr(C)]
pub union host_to_dev_fis_device { pub device: u8, pub head: u8 }
#[repr(C)]
pub union host_to_dev_fis_lba_low_ex { pub lba_low_ex: u8, pub sector_ex: u8 }
#[repr(C)]
pub union host_to_dev_fis_lba_mid_ex { pub lba_mid_ex: u8, pub cyl_low_ex: u8 }
#[repr(C)]
pub union host_to_dev_fis_lba_hi_ex { pub lba_hi_ex: u8, pub cyl_hi_ex: u8 }

#[repr(C)]
pub struct host_to_dev_fis {
    pub type_: u8, pub opts: u8, pub command: u8, pub features: u8,
    pub lba_low: host_to_dev_fis_lba_low, pub lba_mid: host_to_dev_fis_lba_mid,
    pub lba_hi: host_to_dev_fis_lba_hi, pub device: host_to_dev_fis_device,
    pub lba_low_ex: host_to_dev_fis_lba_low_ex, pub lba_mid_ex: host_to_dev_fis_lba_mid_ex,
    pub lba_hi_ex: host_to_dev_fis_lba_hi_ex, pub features_ex: u8,
    pub sect_count: u8, pub sect_cnt_ex: u8, pub res2: u8, pub control: u8, pub res3: u32,
}

#[repr(C)]
pub union mtip_cmd_hdr_count { pub byte_count: __le32, pub status: __le32 }
#[repr(C)]
pub struct mtip_cmd_hdr { pub opts: __le32, pub count: mtip_cmd_hdr_count, pub ctba: __le32, pub ctbau: __le32, pub res: [u32; 4] }

#[repr(C)]
pub struct mtip_cmd_sg { pub dba: __le32, pub dba_upper: __le32, pub reserved: __le32, pub info: __le32 }

pub struct mtip_port;
pub struct mtip_int_cmd;

#[repr(C)]
pub union mtip_cmd_data { pub sg: [scatterlist; MTIP_MAX_SG], pub icmd: *mut mtip_int_cmd }
#[repr(C)]
pub struct mtip_cmd { pub command: *mut core::ffi::c_void, pub command_dma: dma_addr_t, pub scatter_ents: i32, pub unaligned: i32, pub data: mtip_cmd_data, pub retries: i32, pub direction: i32, pub status: blk_status_t }

#[repr(C)]
pub struct mtip_port {
    pub dd: *mut driver_data, pub identify_valid: c_ulong, pub mmio: *mut core::ffi::c_void,
    pub s_active: [*mut core::ffi::c_void; MTIP_MAX_SLOT_GROUPS], pub completed: [*mut core::ffi::c_void; MTIP_MAX_SLOT_GROUPS],
    pub cmd_issue: [*mut core::ffi::c_void; MTIP_MAX_SLOT_GROUPS], pub command_list: *mut core::ffi::c_void,
    pub command_list_dma: dma_addr_t, pub rxfis: *mut core::ffi::c_void, pub rxfis_dma: dma_addr_t,
    pub block1: *mut core::ffi::c_void, pub block1_dma: dma_addr_t, pub identify: *mut u16, pub identify_dma: dma_addr_t,
    pub sector_buffer: *mut u16, pub sector_buffer_dma: dma_addr_t, pub log_buf: *mut u16, pub log_buf_dma: dma_addr_t,
    pub smart_buf: *mut u8, pub smart_buf_dma: dma_addr_t, pub cmds_to_issue: [c_ulong; SLOTBITS_IN_LONGS],
    pub svc_wait: wait_queue_head_t, pub flags: c_ulong, pub ic_pause_timer: c_ulong, pub cmd_slot_unal: atomic_t,
    pub cmd_issue_lock: [spinlock_t; MTIP_MAX_SLOT_GROUPS],
}

#[repr(C)]
pub struct driver_data {
    pub mmio: *mut core::ffi::c_void, pub major: i32, pub instance: i32, pub disk: *mut gendisk,
    pub pdev: *mut pci_dev, pub queue: *mut request_queue, pub tags: blk_mq_tag_set, pub ioctl_mutex: mutex,
    pub port: *mut mtip_port, pub product_type: u32, pub slot_groups: u32, pub index: c_ulong, pub dd_flag: c_ulong,
    pub mtip_svc_handler: *mut task_struct, pub dfs_node: *mut dentry, pub sr: bool, pub numa_node: i32,
    pub workq_name: [i8; 32], pub isr_workq: *mut workqueue_struct, pub irq_workers_active: atomic_t,
    pub work: [mtip_work; MTIP_MAX_SLOT_GROUPS], pub isr_binding: i32, pub unal_qdepth: i32,
}

extern "C" {
    pub fn mtip_workq_sdbfx(port: *mut core::ffi::c_void, group: usize, completed: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
