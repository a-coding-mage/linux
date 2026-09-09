/* SPDX-License-Identifier: GPL-2.0 */
/*
 * s390 diagnose functions
 *
 * Copyright IBM Corp. 2007
 * Author(s): Michael Holzheu <holzheu@de.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub enum diag_stat_enum {
    DIAG_STAT_X008, DIAG_STAT_X00C, DIAG_STAT_X010, DIAG_STAT_X014,
    DIAG_STAT_X044, DIAG_STAT_X064, DIAG_STAT_X08C, DIAG_STAT_X09C,
    DIAG_STAT_X0DC, DIAG_STAT_X204, DIAG_STAT_X210, DIAG_STAT_X224,
    DIAG_STAT_X250, DIAG_STAT_X258, DIAG_STAT_X26C, DIAG_STAT_X288,
    DIAG_STAT_X2C4, DIAG_STAT_X2FC, DIAG_STAT_X304, DIAG_STAT_X308,
    DIAG_STAT_X310, DIAG_STAT_X318, DIAG_STAT_X320, DIAG_STAT_X324,
    DIAG_STAT_X49C, DIAG_STAT_X500, NR_DIAG_STAT,
}

extern "C" {
    pub fn diag_stat_inc(nr: diag_stat_enum);
    pub fn diag_stat_inc_norecursion(nr: diag_stat_enum);
    pub fn diag0c(data: *mut hypfs_diag0c_entry);
    pub fn diag14(rx: c_ulong, ry1: c_ulong, subcode: c_ulong) -> c_int;
    pub fn diag210(addr: *mut diag210) -> c_int;
    pub fn diag8c(out: *mut diag8c, devno: *mut ccw_dev_id) -> c_int;
}

pub enum hypfs_diag0c_entry {}

#[inline]
pub unsafe fn diag10_range(start_pfn: c_ulong, num_pfn: c_ulong) {
    let start_addr = pfn_to_phys(start_pfn);
    let end_addr = pfn_to_phys(start_pfn + num_pfn - 1);
    diag_stat_inc(diag_stat_enum::DIAG_STAT_X010);
    core::arch::asm!(
        "0: diag {0},{1},0x10",
        "1: nopr %r7",
        inlateout(reg) start_addr => _,
        inlateout(reg) end_addr => _,
        options(nostack)
    );
}

#[repr(C, packed)]
pub struct diag210 { pub vrdcdvno: u16, pub vrdclen: u16, pub vrdcvcla: u8, pub vrdcvtyp: u8, pub vrdcvsta: u8, pub vrdcvfla: u8, pub vrdcrccl: u8, pub vrdccrty: u8, pub vrdccrmd: u8, pub vrdccrft: u8 }

#[repr(C, packed)]
pub struct diag8c { pub flags: u8, pub num_partitions: u8, pub width: u16, pub height: u16, pub data: [u8; 0] }

pub const DIAG204_LPAR_PHYS_FLG: u32 = 0x80;
pub const DIAG204_LPAR_NAME_LEN: usize = 8;
pub const DIAG204_CPU_NAME_LEN: usize = 16;

#[repr(C)] pub enum diag204_sc { DIAG204_SUBC_STIB4 = 4, DIAG204_SUBC_RSI = 5, DIAG204_SUBC_STIB6 = 6, DIAG204_SUBC_STIB7 = 7 }
pub const DIAG204_SUBCODE_MASK: u32 = 0xffff;
pub const DIAG204_BIF_BIT: u32 = 0x80000000;
pub const DIAG204_BUSY_WAIT: u32 = HZ / 10;
#[repr(C)] pub enum diag204_format { DIAG204_INFO_SIMPLE = 0, DIAG204_INFO_EXT = 0x00010000 }
#[repr(C)] pub enum diag204_cpu_flags { DIAG204_CPU_ONLINE = 0x20, DIAG204_CPU_CAPPED = 0x40 }

#[repr(C, packed)] pub struct diag204_info_blk_hdr { pub npar:u8, pub flags:u8, pub tslice:u16, pub phys_cpus:u16, pub this_part:u16, pub curtod:u64 }
#[repr(C, packed)] pub struct diag204_x_info_blk_hdr { pub npar:u8, pub flags:u8, pub tslice:u16, pub phys_cpus:u16, pub this_part:u16, pub curtod1:u64, pub curtod2:u64, pub reserved:[c_char;40] }
#[repr(C, packed)] pub struct diag204_part_hdr { pub pn:u8, pub cpus:u8, pub reserved:[c_char;6], pub part_name:[c_char;DIAG204_LPAR_NAME_LEN] }
#[repr(C, packed)] pub struct diag204_x_part_hdr { pub pn:u8,pub cpus:u8,pub rcpus:u8,pub pflag:u8,pub mlu:u32,pub part_name:[c_char;8],pub lpc_name:[c_char;8],pub os_name:[c_char;8],pub online_cs:u64,pub online_es:u64,pub upid:u8,pub reserved_mtid:u8,pub reserved1:[c_char;2],pub group_mlu:u32,pub group_name:[c_char;8],pub hardware_group_name:[c_char;8],pub reserved2:[c_char;24] }
#[repr(C, packed)] pub struct diag204_cpu_info { pub cpu_addr:u16,pub reserved1:[c_char;2],pub ctidx:u8,pub cflag:u8,pub weight:u16,pub acc_time:u64,pub lp_time:u64 }
#[repr(C, packed)] pub struct diag204_x_cpu_info { pub cpu_addr:u16,pub reserved1:[c_char;2],pub ctidx:u8,pub cflag:u8,pub weight:u16,pub acc_time:u64,pub lp_time:u64,pub min_weight:u16,pub cur_weight:u16,pub max_weight:u16,pub reseved2:[c_char;2],pub online_time:u64,pub wait_time:u64,pub pma_weight:u32,pub polar_weight:u32,pub cpu_type_cap:u32,pub group_cpu_type_cap:u32,pub reserved3:[c_char;32] }
#[repr(C, packed)] pub struct diag204_phys_hdr { pub reserved1:[c_char;1],pub cpus:u8,pub reserved2:[c_char;6],pub mgm_name:[c_char;8] }
#[repr(C, packed)] pub struct diag204_x_phys_hdr { pub reserved1:[c_char;1],pub cpus:u8,pub reserved2:[c_char;6],pub mgm_name:[c_char;8],pub reserved3:[c_char;80] }
#[repr(C, packed)] pub struct diag204_phys_cpu { pub cpu_addr:u16,pub reserved1:[c_char;2],pub ctidx:u8,pub reserved2:[c_char;3],pub mgm_time:u64,pub reserved3:[c_char;8] }
#[repr(C, packed)] pub struct diag204_x_phys_cpu { pub cpu_addr:u16,pub reserved1:[c_char;2],pub ctidx:u8,pub reserved2:[c_char;1],pub weight:u16,pub mgm_time:u64,pub reserved3:[c_char;80] }
#[repr(C, packed)] pub struct diag204_x_part_block { pub hdr:diag204_x_part_hdr, pub cpus:[diag204_x_cpu_info;0] }
#[repr(C, packed)] pub struct diag204_x_phys_block { pub hdr:diag204_x_phys_hdr, pub cpus:[diag204_x_phys_cpu;0] }

#[repr(C)] pub enum diag26c_sc { DIAG26C_PORT_VNIC=0x24, DIAG26C_MAC_SERVICES=0x30 }
#[repr(C)] pub enum diag26c_version { DIAG26C_VERSION2=2, DIAG26C_VERSION6_VM65918=0x00020006 }
pub const DIAG26C_VNIC_INFO:u16=2; pub const VNIC_INFO_PROT_L3:u8=1; pub const VNIC_INFO_PROT_L2:u8=2;
#[repr(C, packed)] pub struct diag26c_vnic_req { pub resp_buf_len:u32,pub resp_version:u32,pub req_format:u16,pub vlan_id:u16,pub sys_name:u64,pub res:[u8;2],pub devno:u16 }
#[repr(C, packed)] pub struct diag26c_vnic_resp { pub version:u32,pub entry_cnt:u32,pub next_entry:u32,pub owner:u64,pub devno:u16,pub status:u8,pub r#type:u8,pub lan_owner:u64,pub lan_name:u64,pub port_name:u64,pub port_type:u8,pub ext_status_protocol:u8,pub base_devno:u16,pub port_num:u32,pub ifindex:u32,pub maxinfo:u32,pub dev_count:u32,pub dev_info1:[u8;28],pub dev_info2:[u8;28],pub dev_info3:[u8;28] }
pub const DIAG26C_GET_MAC:u16=0;
#[repr(C)] pub struct diag26c_mac_req { pub resp_buf_len:u32,pub resp_version:u32,pub op_code:u16,pub devno:u16,pub res:[u8;4] }
#[repr(C, align(8))] pub struct diag26c_mac_resp { pub version:u32,pub mac:[u8;6],pub res:[u8;2] }
pub const CPNC_LINUX:usize=4;
#[repr(C)] pub union diag318_info { pub val:c_ulong, pub bits:diag318_info_bits }
#[repr(C)] pub struct diag318_info_bits { pub cpnc:u8, pub cpvc:[u8;7] }

#[inline] pub fn diag204_has_bif() -> bool { unsafe { sclp.has_diag204_bif } }
extern "C" { pub fn diag204(subcode:c_ulong,size:c_ulong,addr:*mut c_void)->c_int; pub fn diag224(ptr:*mut c_void)->c_int; pub fn diag26c(req:*mut c_void,resp:*mut c_void,subcode:diag26c_sc)->c_int; }
#[repr(C)] pub struct diag_ops { pub diag210:Option<unsafe extern "C" fn(*mut diag210)->c_int>,pub diag26c:Option<unsafe extern "C" fn(c_ulong,c_ulong,diag26c_sc)->c_int>,pub diag14:Option<unsafe extern "C" fn(c_ulong,c_ulong,c_ulong)->c_int>,pub diag8c:Option<unsafe extern "C" fn(*mut diag8c,*mut ccw_dev_id,usize)->c_int>,pub diag0c:Option<unsafe extern "C" fn(c_ulong)>,pub diag308_reset:Option<unsafe extern "C" fn()> }
extern "C" { pub static mut diag_amode31_ops:diag_ops; pub static mut __diag210_tmp_amode31:*mut diag210; pub fn _diag210_amode31(addr:*mut diag210)->c_int; pub fn _diag26c_amode31(rx:c_ulong,rx1:c_ulong,subcode:diag26c_sc)->c_int; pub fn _diag14_amode31(rx:c_ulong,ry1:c_ulong,subcode:c_ulong)->c_int; pub fn _diag0c_amode31(rx:c_ulong); pub fn _diag308_reset_amode31(); pub fn _diag8c_amode31(addr:*mut diag8c,devno:*mut ccw_dev_id,len:usize)->c_int; pub fn diag49c(subcode:c_ulong)->c_int; }
#[repr(C)] pub enum diag49c_sc { DIAG49C_SUBC_ACK=0, DIAG49C_SUBC_REG=1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
