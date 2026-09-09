/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* IPL Parameter List header */
#[repr(C, packed)]
pub struct ipl_pl_hdr {
    pub len: u32,
    pub flags: u8,
    pub reserved1: [u8; 2],
    pub version: u8,
}

pub const IPL_PL_FLAG_IPLPS: u8 = 0x80;
pub const IPL_PL_FLAG_SIPL: u8 = 0x40;
pub const IPL_PL_FLAG_IPLSR: u8 = 0x20;
pub const IPL_PL_FLAG_SBP: u8 = 0x10;

/* IPL Parameter Block header */
#[repr(C, packed)]
pub struct ipl_pb_hdr { pub len: u32, pub pbt: u8 }

/* IPL Parameter Block types */
#[repr(u32)]
pub enum ipl_pbt {
    IPL_PBT_FCP = 0,
    IPL_PBT_SCP_DATA = 1,
    IPL_PBT_CCW = 2,
    IPL_PBT_ECKD = 3,
    IPL_PBT_NVME = 4,
}

#[repr(C, packed)]
pub struct ipl_pb0_common {
    pub len: u32, pub pbt: u8, pub flags: u8, pub reserved1: [u8; 2],
    pub loadparm: [u8; 8], pub reserved2: [u8; 84],
}
pub const IPL_PB0_FLAG_LOADPARM: u8 = 0x80;

#[repr(C, packed)]
pub struct ipl_pb0_fcp {
    pub len: u32, pub pbt: u8, pub reserved1: [u8; 3], pub loadparm: [u8; 8],
    pub reserved2: [u8; 304], pub opt: u8, pub reserved3: [u8; 3],
    pub cssid: u8, pub reserved4: [u8; 1], pub devno: u16, pub reserved5: [u8; 4],
    pub wwpn: u64, pub lun: u64, pub bootprog: u32, pub reserved6: [u8; 12],
    pub br_lba: u64, pub scp_data_len: u32, pub reserved7: [u8; 260], pub scp_data: [u8; 0],
}
pub const IPL_PB0_FCP_OPT_IPL: u8 = 0x10;
pub const IPL_PB0_FCP_OPT_DUMP: u8 = 0x20;

#[repr(C, packed)]
pub struct ipl_pb0_nvme {
    pub len: u32, pub pbt: u8, pub reserved1: [u8; 3], pub loadparm: [u8; 8],
    pub reserved2: [u8; 304], pub opt: u8, pub reserved3: [u8; 3], pub fid: u32,
    pub reserved4: [u8; 12], pub nsid: u32, pub reserved5: [u8; 4], pub bootprog: u32,
    pub reserved6: [u8; 12], pub br_lba: u64, pub scp_data_len: u32,
    pub reserved7: [u8; 260], pub scp_data: [u8; 0],
}
pub const IPL_PB0_NVME_OPT_IPL: u8 = 0x10;
pub const IPL_PB0_NVME_OPT_DUMP: u8 = 0x20;

#[repr(C, packed)]
pub struct ipl_pb0_ccw {
    pub len: u32, pub pbt: u8, pub flags: u8, pub reserved1: [u8; 2], pub loadparm: [u8; 8],
    pub reserved2: [u8; 84], pub reserved3_ssid: u16, pub devno: u16, pub vm_flags: u8,
    pub reserved4: [u8; 3], pub vm_parm_len: u32, pub nss_name: [u8; 8],
    pub vm_parm: [u8; 64], pub reserved5: [u8; 8],
}

#[repr(C, packed)]
pub struct ipl_pb0_eckd_br_chr { pub cyl: u16, pub head: u8, pub record: u8, pub reserved: u32 }
#[repr(C, packed)]
pub struct ipl_pb0_eckd {
    pub len: u32, pub pbt: u8, pub reserved1: [u8; 3], pub reserved2: [u32; 78],
    pub opt: u8, pub reserved4: [u8; 4], pub reserved5_ssid: u8, pub devno: u16,
    pub reserved6: [u32; 5], pub bootprog: u32, pub reserved7: [u8; 12],
    pub br_chr: ipl_pb0_eckd_br_chr, pub scp_data_len: u32, pub reserved8: [u8; 260],
    pub scp_data: [u8; 0],
}
pub const IPL_PB0_ECKD_OPT_IPL: u8 = 0x10;
pub const IPL_PB0_ECKD_OPT_DUMP: u8 = 0x20;
pub const IPL_PB0_CCW_VM_FLAG_NSS: u8 = 0x80;
pub const IPL_PB0_CCW_VM_FLAG_VP: u8 = 0x40;

#[repr(C, packed)]
pub struct ipl_pb1_scp_data { pub len: u32, pub pbt: u8, pub scp_data: [u8; 0] }

#[repr(C, packed)]
pub struct ipl_rl_hdr { pub len: u32, pub flags: u8, pub reserved1: [u8; 2], pub version: u8, pub reserved2: [u8; 8] }
#[repr(C, packed)]
pub struct ipl_rb_hdr { pub len: u32, pub rbt: u8, pub reserved1: [u8; 11] }

#[repr(u32)]
pub enum ipl_rbt { IPL_RBT_CERTIFICATES = 1, IPL_RBT_COMPONENTS = 2 }

#[repr(C, packed)]
pub struct ipl_rb_certificate_entry { pub addr: u64, pub len: u64 }
#[repr(C, packed)]
pub struct ipl_rb_certificates { pub len: u32, pub rbt: u8, pub reserved1: [u8; 11], pub entries: [ipl_rb_certificate_entry; 0] }

#[repr(C)]
pub struct ipl_rb_component_entry {
    pub addr: u64, pub len: u64, pub flags: u8, pub reserved1: [u8; 5],
    pub certificate_index: u16, pub reserved2: [u8; 8],
}
pub const IPL_RB_COMPONENT_FLAG_SIGNED: u8 = 0x80;
pub const IPL_RB_COMPONENT_FLAG_VERIFIED: u8 = 0x40;

#[repr(C, packed)]
pub struct ipl_rb_components { pub len: u32, pub rbt: u8, pub reserved1: [u8; 11], pub entries: [ipl_rb_component_entry; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
