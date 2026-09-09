/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Microsemi Switchtec PCIe Driver ioctl UAPI. */

// The C header includes <linux/types.h>; these Rust types correspond to its
// fixed-width userspace ABI types.

pub const SWITCHTEC_IOCTL_PART_CFG0: u32 = 0;
pub const SWITCHTEC_IOCTL_PART_CFG1: u32 = 1;
pub const SWITCHTEC_IOCTL_PART_IMG0: u32 = 2;
pub const SWITCHTEC_IOCTL_PART_IMG1: u32 = 3;
pub const SWITCHTEC_IOCTL_PART_NVLOG: u32 = 4;
pub const SWITCHTEC_IOCTL_PART_VENDOR0: u32 = 5;
pub const SWITCHTEC_IOCTL_PART_VENDOR1: u32 = 6;
pub const SWITCHTEC_IOCTL_PART_VENDOR2: u32 = 7;
pub const SWITCHTEC_IOCTL_PART_VENDOR3: u32 = 8;
pub const SWITCHTEC_IOCTL_PART_VENDOR4: u32 = 9;
pub const SWITCHTEC_IOCTL_PART_VENDOR5: u32 = 10;
pub const SWITCHTEC_IOCTL_PART_VENDOR6: u32 = 11;
pub const SWITCHTEC_IOCTL_PART_VENDOR7: u32 = 12;
pub const SWITCHTEC_IOCTL_PART_BL2_0: u32 = 13;
pub const SWITCHTEC_IOCTL_PART_BL2_1: u32 = 14;
pub const SWITCHTEC_IOCTL_PART_MAP_0: u32 = 15;
pub const SWITCHTEC_IOCTL_PART_MAP_1: u32 = 16;
pub const SWITCHTEC_IOCTL_PART_KEY_0: u32 = 17;
pub const SWITCHTEC_IOCTL_PART_KEY_1: u32 = 18;

pub const SWITCHTEC_NUM_PARTITIONS_GEN3: u32 = 13;
pub const SWITCHTEC_NUM_PARTITIONS_GEN4: u32 = 19;
pub const SWITCHTEC_IOCTL_NUM_PARTITIONS: u32 = SWITCHTEC_NUM_PARTITIONS_GEN3;

#[repr(C)]
pub struct switchtec_ioctl_flash_info {
    pub flash_length: u64,
    pub num_partitions: u32,
    pub padding: u32,
}

pub const SWITCHTEC_IOCTL_PART_ACTIVE: u32 = 1;
pub const SWITCHTEC_IOCTL_PART_RUNNING: u32 = 2;

#[repr(C)]
pub struct switchtec_ioctl_flash_part_info {
    pub flash_partition: u32,
    pub address: u32,
    pub length: u32,
    pub active: u32,
}

#[repr(C)]
pub struct switchtec_ioctl_event_summary_legacy {
    pub global: u64,
    pub part_bitmap: u64,
    pub local_part: u32,
    pub padding: u32,
    pub part: [u32; 48],
    pub pff: [u32; 48],
}

#[repr(C)]
pub struct switchtec_ioctl_event_summary {
    pub global: u64,
    pub part_bitmap: u64,
    pub local_part: u32,
    pub padding: u32,
    pub part: [u32; 48],
    pub pff: [u32; 255],
}

pub const SWITCHTEC_IOCTL_EVENT_STACK_ERROR: u32 = 0;
pub const SWITCHTEC_IOCTL_EVENT_PPU_ERROR: u32 = 1;
pub const SWITCHTEC_IOCTL_EVENT_ISP_ERROR: u32 = 2;
pub const SWITCHTEC_IOCTL_EVENT_SYS_RESET: u32 = 3;
pub const SWITCHTEC_IOCTL_EVENT_FW_EXC: u32 = 4;
pub const SWITCHTEC_IOCTL_EVENT_FW_NMI: u32 = 5;
pub const SWITCHTEC_IOCTL_EVENT_FW_NON_FATAL: u32 = 6;
pub const SWITCHTEC_IOCTL_EVENT_FW_FATAL: u32 = 7;
pub const SWITCHTEC_IOCTL_EVENT_TWI_MRPC_COMP: u32 = 8;
pub const SWITCHTEC_IOCTL_EVENT_TWI_MRPC_COMP_ASYNC: u32 = 9;
pub const SWITCHTEC_IOCTL_EVENT_CLI_MRPC_COMP: u32 = 10;
pub const SWITCHTEC_IOCTL_EVENT_CLI_MRPC_COMP_ASYNC: u32 = 11;
pub const SWITCHTEC_IOCTL_EVENT_GPIO_INT: u32 = 12;
pub const SWITCHTEC_IOCTL_EVENT_PART_RESET: u32 = 13;
pub const SWITCHTEC_IOCTL_EVENT_MRPC_COMP: u32 = 14;
pub const SWITCHTEC_IOCTL_EVENT_MRPC_COMP_ASYNC: u32 = 15;
pub const SWITCHTEC_IOCTL_EVENT_DYN_PART_BIND_COMP: u32 = 16;
pub const SWITCHTEC_IOCTL_EVENT_AER_IN_P2P: u32 = 17;
pub const SWITCHTEC_IOCTL_EVENT_AER_IN_VEP: u32 = 18;
pub const SWITCHTEC_IOCTL_EVENT_DPC: u32 = 19;
pub const SWITCHTEC_IOCTL_EVENT_CTS: u32 = 20;
pub const SWITCHTEC_IOCTL_EVENT_HOTPLUG: u32 = 21;
pub const SWITCHTEC_IOCTL_EVENT_IER: u32 = 22;
pub const SWITCHTEC_IOCTL_EVENT_THRESH: u32 = 23;
pub const SWITCHTEC_IOCTL_EVENT_POWER_MGMT: u32 = 24;
pub const SWITCHTEC_IOCTL_EVENT_TLP_THROTTLING: u32 = 25;
pub const SWITCHTEC_IOCTL_EVENT_FORCE_SPEED: u32 = 26;
pub const SWITCHTEC_IOCTL_EVENT_CREDIT_TIMEOUT: u32 = 27;
pub const SWITCHTEC_IOCTL_EVENT_LINK_STATE: u32 = 28;
pub const SWITCHTEC_IOCTL_EVENT_GFMS: u32 = 29;
pub const SWITCHTEC_IOCTL_EVENT_INTERCOMM_REQ_NOTIFY: u32 = 30;
pub const SWITCHTEC_IOCTL_EVENT_UEC: u32 = 31;
pub const SWITCHTEC_IOCTL_MAX_EVENTS: u32 = 32;

pub const SWITCHTEC_IOCTL_EVENT_LOCAL_PART_IDX: i32 = -1;
pub const SWITCHTEC_IOCTL_EVENT_IDX_ALL: i32 = -2;
pub const SWITCHTEC_IOCTL_EVENT_FLAG_CLEAR: u32 = 1 << 0;
pub const SWITCHTEC_IOCTL_EVENT_FLAG_EN_POLL: u32 = 1 << 1;
pub const SWITCHTEC_IOCTL_EVENT_FLAG_EN_LOG: u32 = 1 << 2;
pub const SWITCHTEC_IOCTL_EVENT_FLAG_EN_CLI: u32 = 1 << 3;
pub const SWITCHTEC_IOCTL_EVENT_FLAG_EN_FATAL: u32 = 1 << 4;
pub const SWITCHTEC_IOCTL_EVENT_FLAG_DIS_POLL: u32 = 1 << 5;
pub const SWITCHTEC_IOCTL_EVENT_FLAG_DIS_LOG: u32 = 1 << 6;
pub const SWITCHTEC_IOCTL_EVENT_FLAG_DIS_CLI: u32 = 1 << 7;
pub const SWITCHTEC_IOCTL_EVENT_FLAG_DIS_FATAL: u32 = 1 << 8;
pub const SWITCHTEC_IOCTL_EVENT_FLAG_UNUSED: u32 = !0x1ff;

#[repr(C)]
pub struct switchtec_ioctl_event_ctl {
    pub event_id: u32,
    pub index: i32,
    pub flags: u32,
    pub occurred: u32,
    pub count: u32,
    pub data: [u32; 5],
}

pub const SWITCHTEC_IOCTL_PFF_VEP: u32 = 100;

#[repr(C)]
pub struct switchtec_ioctl_pff_port {
    pub pff: u32,
    pub partition: u32,
    pub port: u32,
}

// These ioctl constants use the kernel _IOR/_IOWR macros supplied by the
// target platform; retain the original expressions for dependent bindings.
// SWITCHTEC_IOCTL_FLASH_INFO       = _IOR('W', 0x40, struct switchtec_ioctl_flash_info)
// SWITCHTEC_IOCTL_FLASH_PART_INFO  = _IOWR('W', 0x41, struct switchtec_ioctl_flash_part_info)
// SWITCHTEC_IOCTL_EVENT_SUMMARY    = _IOR('W', 0x42, struct switchtec_ioctl_event_summary)
// SWITCHTEC_IOCTL_EVENT_SUMMARY_LEGACY = _IOR('W', 0x42, struct switchtec_ioctl_event_summary_legacy)
// SWITCHTEC_IOCTL_EVENT_CTL        = _IOWR('W', 0x43, struct switchtec_ioctl_event_ctl)
// SWITCHTEC_IOCTL_PFF_TO_PORT      = _IOWR('W', 0x44, struct switchtec_ioctl_pff_port)
// SWITCHTEC_IOCTL_PORT_TO_PFF       = _IOWR('W', 0x45, struct switchtec_ioctl_pff_port)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
