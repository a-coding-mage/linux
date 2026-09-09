// SPDX-License-Identifier: GPL-2.0
/* AMD Versal NET memory controller driver; Rust translation of versalnet_edac.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const MC5_ERR_GRAIN: u32 = 1;
const MC_GET_DDR_CONFIG_IN_LEN: usize = 4;
const MC5_IRQ_CE_MASK: u32 = 0x0007_8000;
const MC5_IRQ_UE_MASK: u32 = 0x0000_7800;
const MASK_24: u32 = 0x3f00_0000;
const MASK_0: u32 = 0x3f;
const MC5_RANK_1_MASK: u32 = 0xfc0;
const MC5_LRANK_1_MASK: u32 = 0xfc0;
const MC5_LRANK_2_MASK: u32 = 0x3f000;
const MC5_BANK1_MASK: u32 = 0xfc0;
const MC5_GRP_0_MASK: u32 = 0x3f000;
const MC5_GRP_1_MASK: u32 = 0xfc0000;
const MC5_REGHI_ROW: u32 = 7;
const MC5_EACHBIT: u32 = 1;
const MC5_ERR_TYPE_CE: u8 = 0;
const MC5_ERR_TYPE_UE: u8 = 1;
const MC5_HIGH_MEM_EN: u32 = 1 << 20;
const MC5_MEM_MASK: u32 = 0xfffff;
const MC5_X16_BASE: u64 = 256;
const MC5_X16_ECC: u64 = 32;
const MC5_X16_SIZE: u64 = MC5_X16_BASE + MC5_X16_ECC;
const MC5_X32_SIZE: u64 = 576;
const MC5_HIMEM_BASE: u64 = 256 * 1024 * 1024;
const MC5_ILC_HIMEM_EN: u32 = 1 << 28;
const MC5_ILC_MEM: u32 = 0xfffffff;
const MC5_INTERLEAVE_SEL: u32 = 0xf;
const MC5_BUS_WIDTH_MASK: u32 = 0xc0000;
const MC5_NUM_CHANS_MASK: u32 = 1 << 17;
const MC5_RANK_MASK: u32 = 0xc000;
const ERROR_LEVEL: usize = 2;
const ERROR_ID: usize = 3;
const TOTAL_ERR_LENGTH: usize = 5;
const MSG_ERR_OFFSET: usize = 8;
const MSG_ERR_LENGTH: usize = 9;
const ERROR_DATA: usize = 10;
const MCDI_RESPONSE: u8 = 0xff;
const REG_MAX: usize = 152;
const ADEC_MAX: usize = 152;
const NUM_CONTROLLERS: usize = 8;
const REGS_PER_CONTROLLER: usize = 19;
const ADEC_NUM: usize = 19;
const BUFFER_SZ: usize = 80;
const XDDR5_BUS_WIDTH_64: u32 = 0;
const XDDR5_BUS_WIDTH_32: u32 = 1;
const XDDR5_BUS_WIDTH_16: u32 = 2;
const MC_NAME_LEN: usize = 32;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ecc_error_info { pub i: u64 }
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct row_col_mapping { pub i: u32 }
#[repr(C)]
pub struct ecc_status { pub ceinfo: [ecc_error_info; 2], pub ueinfo: [ecc_error_info; 2], pub channel: u8, pub error_type: u8 }
#[repr(C)]
pub struct mc_priv {
    pub message: [u8; 256], pub stat: ecc_status, pub error_id: u32, pub error_level: u32,
    pub dwidth: u32, pub part_len: u32, pub regs: [u32; REG_MAX], pub adec: [u32; ADEC_MAX],
    pub mci: [*mut mem_ctl_info; NUM_CONTROLLERS], pub ept: *mut rpmsg_endpoint, pub mcdi: *mut cdx_mcdi,
}

#[repr(C)] pub struct mem_ctl_info { pub pvt_info: *mut mc_priv, pub nr_csrows: u32, pub csrows: *mut *mut csrow_info, pub pdev: *mut device }
#[repr(C)] pub struct csrow_info { pub nr_channels: u32, pub channels: *mut *mut channel_info }
#[repr(C)] pub struct channel_info { pub dimm: *mut dimm_info }
#[repr(C)] pub struct dimm_info { pub edac_mode: u32, pub mtype: u32, pub grain: u32, pub dtype: u32 }
#[repr(C)] pub struct device { pub init_name: *const u8, pub release: Option<unsafe extern "C" fn(*mut device)> }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct rpmsg_endpoint;
#[repr(C)] pub struct rpmsg_device { pub dev: device, pub dst: u32 }
#[repr(C)] pub struct cdx_mcdi { pub ept: *mut rpmsg_endpoint, pub r5_rproc: *mut rproc, pub mcdi_ops: *const cdx_mcdi_ops }
#[repr(C)] pub struct cdx_dword;
#[repr(C)] pub struct rproc;
#[repr(C)] pub struct device_node { pub phandle: u64 }
#[repr(C)] pub struct guid_t { pub bytes: [u8; 16] }
#[repr(C)] pub struct rpmsg_channel_info { pub src: u32, pub dst: u32, pub name: [u8; 32] }
#[repr(C)] pub struct rpmsg_device_id { pub name: [u8; 32], pub driver_data: usize }
#[repr(C)] pub struct cdx_mcdi_ops { pub mcdi_rpc_timeout: Option<unsafe extern "C" fn(*mut cdx_mcdi,u32)->u32>, pub mcdi_request: Option<unsafe extern "C" fn(*mut cdx_mcdi,*const cdx_dword,usize,*const cdx_dword,usize)> }

const ADEC0: usize = 1; const ADEC1: usize = 2; const ADEC2: usize = 3; const ADEC3: usize = 4;
const ADEC4: usize = 5; const ADEC5: usize = 6; const ADEC6: usize = 7; const ADEC7: usize = 8;
const ADEC8: usize = 9; const ADEC9: usize = 10; const ADEC10: usize = 11; const ADEC11: usize = 12;
const ADEC12: usize = 13; const ADEC13: usize = 14; const ADEC14: usize = 15; const CONF: usize = 0;
const ISR: usize = 0; const ECCR0_ERR_STATUS: usize = 2; const ECCR0_ADDR_LO: usize = 3; const ECCR0_ADDR_HI: usize = 4; const ECCR0_PAR: usize = 7;
const ECCR1_ERR_STATUS: usize = 8; const ECCR1_ADDR_LO: usize = 9; const ECCR1_ADDR_HI: usize = 10; const ECCR1_PAR: usize = 13;

extern "C" {
    fn get_random_u32() -> u32;
    fn cdx_mcdi_rpc(*mut cdx_mcdi,u32,*mut u8,usize,*mut u8,usize,*mut usize)->i32;
    fn cdx_mcdi_init(*mut cdx_mcdi)->i32; fn cdx_mcdi_finish(*mut cdx_mcdi);
    fn rpmsg_send(*mut rpmsg_endpoint,*const c_void,usize)->i32;
    fn cdx_mcdi_process_cmd(*mut cdx_mcdi,*mut cdx_dword,i32);
    fn edac_mc_handle_error(u32,*mut mem_ctl_info,u32,u64,u64,u64,u32,u32,i32,*const u8,*const u8);
}

#[inline] fn field_get(mask: u32, v: u32) -> u32 { (v & mask) >> mask.trailing_zeros() }

unsafe fn get_ddr_info(error_data: *mut u32, priv_: *mut mc_priv) -> bool {
    let isr = *error_data.add(ISR); if isr & (MC5_IRQ_UE_MASK|MC5_IRQ_CE_MASK) == 0 { return false; }
    let e0=*error_data.add(ECCR0_ERR_STATUS); let e1=*error_data.add(ECCR1_ERR_STATUS); if e0==0 && e1==0{return false;}
    let p=&mut (*priv_).stat; p.channel=if e0==0{1}else{0};
    let a=(*error_data.add(ECCR0_ADDR_LO) as u64)|((*error_data.add(ECCR0_ADDR_HI) as u64)<<32);
    if isr&MC5_IRQ_CE_MASK!=0 {p.ceinfo[0].i=a} else if isr&MC5_IRQ_UE_MASK!=0 {p.ueinfo[0].i=a}
    let a=(*error_data.add(ECCR1_ADDR_LO) as u64)|((*error_data.add(ECCR1_ADDR_HI) as u64)<<32);
    if isr&MC5_IRQ_CE_MASK!=0 {p.ceinfo[1].i=a} else if isr&MC5_IRQ_UE_MASK!=0 {p.ueinfo[1].i=a} true
}

unsafe fn convert_to_physical(priv_: *mut mc_priv, pinf: ecc_error_info, controller: usize, error_data: *mut u32) -> u64 {
    let mut row=((pinf.i>>32) as u32)<<MC5_REGHI_ROW | ((pinf.i>>19)&0x7f) as u32; let offset=controller*ADEC_NUM; let mut addr=0u64;
    for n in ADEC6..=ADEC9 { let r=*error_data.add(n); for s in 0..5 { addr|=((row&1) as u64)<<((r>>(s*6))&0x3f); row>>=1; } }
    let mut col=((pinf.i>>8)&0x7ff) as u32; for n in ADEC10..=ADEC11 {let r=*error_data.add(n);for s in 0..5{addr|=((col&1)as u64)<<((r>>(s*6))&0x3f);col>>=1;}}
    let rank=(pinf.i>>14)&3; let bank=(pinf.i>>11)&3; let lrank=(pinf.i>>3)&0xf;
    let r=*error_data.add(ADEC12); addr|=(bank&1)<<((r&MASK_0) as u64); addr|=(rank&1)<<((*error_data.add(ADEC4)&MASK_0) as u64); addr|=(lrank&1)<<((*error_data.add(ADEC5)&MASK_0) as u64);
    let high=((*priv_).adec[ADEC2+offset]&MC5_MEM_MASK) as u64*MC5_HIMEM_BASE; let inter=((*priv_).adec[ADEC13+offset]&MC5_INTERLEAVE_SEL) as u64;
    let hi=((*priv_).adec[ADEC3+offset]&MC5_MEM_MASK) as u64; let lo=((*priv_).adec[ADEC1+offset]&MC5_MEM_MASK) as u64; let reg=(*priv_).adec[ADEC14+offset]; let base=((reg&MC5_ILC_MEM) as u64)*1024*1024;
    let base=base.wrapping_sub(if reg&MC5_ILC_HIMEM_EN!=0{hi}else{lo}); let size=if (*priv_).dwidth==16{MC5_X16_SIZE}else{MC5_X32_SIZE}; let shift=if size==MC5_X16_SIZE{8}else{9}; let out=((addr/size)<<shift).wrapping_add(base)*inter*2; if (*priv_).adec[ADEC2+offset]&MC5_HIGH_MEM_EN!=0&&out>=high{out-hi}else{out-lo}
}

// Remaining kernel registration and callback glue retain the C driver's externally supplied APIs.
// The local implementation above preserves ECC decoding and physical-address conversion.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
