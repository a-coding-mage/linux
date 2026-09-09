// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of cpc925_edac.c. Kernel bindings are supplied externally. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const CPC925_EDAC_REVISION: &str = " Ver: 1.0.0";
const CPC925_EDAC_MOD_STR: &str = "cpc925_edac";
const CPC925_BITS_PER_REG: u32 = 32;
const fn CPC925_BIT(nr: u32) -> u32 { 1u32 << (CPC925_BITS_PER_REG - 1 - nr) }
const CPC925_CPU_ERR_DEV: &str = "cpu";
const CPC925_HT_LINK_DEV: &str = "htlink";
const CPC925_REF_FREQ: u32 = 0xFA69;
const CPC925_SCRUB_BLOCK_SIZE: u32 = 64;
const CPC925_NR_CSROWS: usize = 8;

const REG_APIMASK_OFFSET: usize = 0x30070;
const APIMASK_DART: u32 = CPC925_BIT(0); const APIMASK_ADI0: u32 = CPC925_BIT(1);
const APIMASK_ADI1: u32 = CPC925_BIT(2); const APIMASK_STAT: u32 = CPC925_BIT(3);
const APIMASK_DERR: u32 = CPC925_BIT(4); const APIMASK_ADRS0: u32 = CPC925_BIT(5);
const APIMASK_ADRS1: u32 = CPC925_BIT(6); const APIMASK_ECC_UE_H: u32 = CPC925_BIT(8);
const APIMASK_ECC_CE_H: u32 = CPC925_BIT(9); const APIMASK_ECC_UE_L: u32 = CPC925_BIT(10);
const APIMASK_ECC_CE_L: u32 = CPC925_BIT(11);
const CPU_MASK_ENABLE: u32 = APIMASK_DART|APIMASK_ADI0|APIMASK_ADI1|APIMASK_STAT|APIMASK_DERR|APIMASK_ADRS0|APIMASK_ADRS1;
const ECC_MASK_ENABLE: u32 = APIMASK_ECC_UE_H|APIMASK_ECC_CE_H|APIMASK_ECC_UE_L|APIMASK_ECC_CE_L;
const fn APIMASK_ADI(n: u32) -> u32 { CPC925_BIT(n + 1) }
const REG_APIEXCP_OFFSET: usize = 0x30060;
const APIEXCP_DART: u32 = CPC925_BIT(0); const APIEXCP_ADI0: u32 = CPC925_BIT(1);
const APIEXCP_ADI1: u32 = CPC925_BIT(2); const APIEXCP_STAT: u32 = CPC925_BIT(3);
const APIEXCP_DERR: u32 = CPC925_BIT(4); const APIEXCP_ADRS0: u32 = CPC925_BIT(5);
const APIEXCP_ADRS1: u32 = CPC925_BIT(6); const APIEXCP_ECC_UE_H: u32 = CPC925_BIT(8);
const APIEXCP_ECC_CE_H: u32 = CPC925_BIT(9); const APIEXCP_ECC_UE_L: u32 = CPC925_BIT(10);
const APIEXCP_ECC_CE_L: u32 = CPC925_BIT(11);
const CPU_EXCP_DETECTED: u32 = APIEXCP_DART|APIEXCP_ADI0|APIEXCP_ADI1|APIEXCP_STAT|APIEXCP_DERR|APIEXCP_ADRS0|APIEXCP_ADRS1;
const UECC_EXCP_DETECTED: u32 = APIEXCP_ECC_UE_H|APIEXCP_ECC_UE_L;
const CECC_EXCP_DETECTED: u32 = APIEXCP_ECC_CE_H|APIEXCP_ECC_CE_L;
const ECC_EXCP_DETECTED: u32 = UECC_EXCP_DETECTED|CECC_EXCP_DETECTED;
const REG_MBCR_OFFSET: usize=0x2190; const MBCR_64BITCFG_MASK:u32=1<<23; const MBCR_64BITBUS_MASK:u32=1<<22;
const REG_MBMR_OFFSET: usize=0x21C0; const MBMR_MODE_MASK:u32=0xF<<25; const MBMR_BBA_MASK:u32=1<<24;
const REG_MBBAR_OFFSET: usize=0x21D0; const MBBAR_BBA_MASK:u32=0xFF<<24;
const REG_MSCR_OFFSET:usize=0x2400; const MSCR_SCRUB_MOD_MASK:u32=0xC0000000; const MSCR_BACKGR_SCRUB:u32=0x40000000; const MSCR_SI_SHIFT:u32=16; const MSCR_SI_MASK:u32=0xFF<<16;
const REG_MSRSR_OFFSET:usize=0x2410; const REG_MSRER_OFFSET:usize=0x2420; const REG_MSPR_OFFSET:usize=0x2430;
const REG_MCCR_OFFSET:usize=0x2440; const MCCR_ECC_EN:u32=CPC925_BIT(0); const REG_MCRER_OFFSET:usize=0x2450;
const REG_MEAR_OFFSET:usize=0x2460; const MEAR_BCNT_MASK:u32=3<<30; const MEAR_RANK_MASK:u32=7<<27; const MEAR_COL_MASK:u32=0x7ff<<16; const MEAR_BANK_MASK:u32=3<<14; const MEAR_ROW_MASK:u32=0x3fff;
const REG_MESR_OFFSET:usize=0x2470; const MESR_ECC_SYN_H_MASK:u16=0xff00; const MESR_ECC_SYN_L_MASK:u16=0xff;
const REG_MMCR_OFFSET:usize=0x2500; const MMCR_REG_DIMM_MODE:u32=CPC925_BIT(3);
const REG_ERRCTRL_OFFSET:usize=0x70140; const ERRCTRL_SERR_NF:u32=CPC925_BIT(0); const ERRCTRL_CRC_NF:u32=CPC925_BIT(1); const ERRCTRL_RSP_NF:u32=CPC925_BIT(2); const ERRCTRL_EOC_NF:u32=CPC925_BIT(3); const ERRCTRL_OVF_NF:u32=CPC925_BIT(4); const ERRCTRL_PROT_NF:u32=CPC925_BIT(5); const ERRCTRL_RSP_ERR:u32=CPC925_BIT(6); const ERRCTRL_CHN_FAL:u32=CPC925_BIT(7);
const HT_ERRCTRL_ENABLE:u32=ERRCTRL_SERR_NF|ERRCTRL_CRC_NF|ERRCTRL_RSP_NF|ERRCTRL_EOC_NF|ERRCTRL_OVF_NF|ERRCTRL_PROT_NF; const HT_ERRCTRL_DETECTED:u32=ERRCTRL_RSP_ERR|ERRCTRL_CHN_FAL;
const REG_LINKCTRL_OFFSET:usize=0x70110; const LINKCTRL_CRC_ERR:u32=CPC925_BIT(22)|CPC925_BIT(23); const LINKCTRL_LINK_FAIL:u32=CPC925_BIT(27); const HT_LINKCTRL_DETECTED:u32=LINKCTRL_CRC_ERR|LINKCTRL_LINK_FAIL;
const REG_LINKERR_OFFSET:usize=0x70120; const LINKERR_EOC_ERR:u32=CPC925_BIT(17); const LINKERR_OVF_ERR:u32=CPC925_BIT(18); const LINKERR_PROT_ERR:u32=CPC925_BIT(19); const HT_LINKERR_DETECTED:u32=LINKERR_EOC_ERR|LINKERR_OVF_ERR|LINKERR_PROT_ERR;
const REG_BRGCTRL_OFFSET:usize=0x70300; const BRGCTRL_DETSERR:u32=CPC925_BIT(0); const BRGCTRL_SECBUSRESET:u32=CPC925_BIT(9);

#[repr(C)] pub struct cpc925_mc_pdata { pub vbase:*mut c_void, pub total_mem:c_ulong, pub name:*const c_char, pub edac_idx:c_int }
#[repr(C)] pub struct cpc925_dev_info { pub vbase:*mut c_void, pub pdev:*mut platform_device, pub ctl_name:*mut c_char, pub edac_idx:c_int, pub edac_dev:*mut edac_device_ctl_info, pub init:Option<unsafe extern "C" fn(*mut cpc925_dev_info)>, pub exit:Option<unsafe extern "C" fn(*mut cpc925_dev_info)>, pub check:Option<unsafe extern "C" fn(*mut edac_device_ctl_info)> }
#[repr(C)] pub struct device { _priv:[u8;0] } #[repr(C)] pub struct device_node{_priv:[u8;0]} #[repr(C)] pub struct platform_device{pub dev:device,pub name:*const c_char} #[repr(C)] pub struct edac_device_ctl_info{pub pvt_info:*mut c_void,pub dev:*mut device,pub ctl_name:*mut c_char,pub mod_name:*const c_char,pub dev_name:*const c_char,pub edac_check:Option<unsafe extern "C" fn(*mut edac_device_ctl_info)>} #[repr(C)] pub struct mem_ctl_info{pub pvt_info:*mut c_void,pub csrows:*mut *mut csrow_info,pub nr_csrows:c_int,pub ctl_name:*const c_char,pub edac_check:Option<unsafe extern "C" fn(*mut mem_ctl_info)>} #[repr(C)] pub struct csrow_info{pub first_page:c_ulong,pub last_page:c_ulong,pub nr_channels:c_int,pub channels:*mut *mut channel_info} #[repr(C)] pub struct channel_info{pub dimm:*mut dimm_info} #[repr(C)] pub struct dimm_info{pub nr_pages:c_ulong,pub mtype:c_int,pub edac_mode:c_int,pub grain:u32,pub dtype:c_int} #[repr(C)] pub struct resource{pub start:c_ulong} #[repr(C)] pub struct edac_mc_layer{pub type_:c_int,pub size:c_int,pub is_virt_csrow:bool} #[repr(C)] pub struct platform_driver{pub probe:Option<unsafe extern "C" fn(*mut platform_device)->c_int>,pub remove:Option<unsafe extern "C" fn(*mut platform_device)>,pub driver:driver} #[repr(C)] pub struct driver{pub name:*const c_char}
extern "C" { fn __raw_readl(p:*mut c_void)->u32; fn __raw_writel(v:u32,p:*mut c_void); fn edac_mc_handle_error(a:c_int,m:*mut mem_ctl_info,n:u32,p:c_ulong,o:c_ulong,s:u16,r:c_int,ch:c_int,x:c_int,name:*const c_char,msg:*const c_char); fn edac_device_handle_ue(*mut edac_device_ctl_info,c_int,c_int,*const c_char); fn edac_device_handle_ce(*mut edac_device_ctl_info,c_int,c_int,*const c_char); static mut edac_op_state:c_int; fn platform_driver_register(*mut platform_driver)->c_int; fn platform_driver_unregister(*mut platform_driver); }
const PAGE_SHIFT:u32=12; const PAGE_SIZE:c_ulong=1<<PAGE_SHIFT;
unsafe fn rd(base:*mut c_void, off:usize)->u32 { __raw_readl((base as *mut u8).add(off) as *mut c_void) } unsafe fn wr(base:*mut c_void,off:usize,v:u32){__raw_writel(v,(base as *mut u8).add(off) as *mut c_void)}

unsafe fn cpc925_mc_get_pfn(mci:*mut mem_ctl_info,mear:u32,pfn:*mut c_ulong,offset:*mut c_ulong,csrow:*mut c_int){let mut b=(mear>>30)&3;let rank=((mear>>27)&7) as usize;let mut col=(mear>>16)&0x7ff;let bank=(mear>>14)&3;let mut row=mear&0x3fff;*csrow=rank as c_int;let rows=(*mci).csrows;let mut pa=(*rows.add(rank)).first_page<<PAGE_SHIFT;col+=b;for i in 0..11{pa|=((col&1) as c_ulong)<<(14-i);col>>=1;}pa|=(bank as c_ulong)<<19;for i in 0..3{pa|=((row&1) as c_ulong)<<(26-i);row>>=1;}for i in 0..3{pa|=((row&1) as c_ulong)<<(21+i);row>>=1;}for i in 0..4{pa|=((row&1) as c_ulong)<<(18-i);row>>=1;}for i in 0..3{pa|=((row&1) as c_ulong)<<(29-i);row>>=1;}*offset=pa&(PAGE_SIZE-1);*pfn=pa>>PAGE_SHIFT;}
unsafe fn cpc925_mc_find_channel(_mci:*mut mem_ctl_info,s:u16)->c_int{if s&MESR_ECC_SYN_H_MASK==0{0}else if s&MESR_ECC_SYN_L_MASK==0{1}else{1}}
unsafe fn cpc925_mc_check(mci:*mut mem_ctl_info){let p=(*mci).pvt_info as *mut cpc925_mc_pdata;let e=rd((*p).vbase,REG_APIEXCP_OFFSET);if e&ECC_EXCP_DETECTED==0{return}let mesr=rd((*p).vbase,REG_MESR_OFFSET);let syn=(mesr as u16)|(MESR_ECC_SYN_H_MASK|MESR_ECC_SYN_L_MASK);let mut pf=0;let mut off=0;let mut row=0;cpc925_mc_get_pfn(mci,rd((*p).vbase,REG_MEAR_OFFSET),&mut pf,&mut off,&mut row);let ch=cpc925_mc_find_channel(mci,syn);if e&CECC_EXCP_DETECTED!=0{edac_mc_handle_error(0,mci,1,pf,off,syn,row,ch,-1,(*mci).ctl_name,b"\0".as_ptr() as _)}if e&UECC_EXCP_DETECTED!=0{edac_mc_handle_error(1,mci,1,pf,off,0,row,-1,-1,(*mci).ctl_name,b"\0".as_ptr() as _);}}
unsafe fn cpc925_mc_init(mci:*mut mem_ctl_info){let p=(*mci).pvt_info as *mut cpc925_mc_pdata;let mut v=rd((*p).vbase,REG_APIMASK_OFFSET);if v&ECC_MASK_ENABLE==0{v|=ECC_MASK_ENABLE;wr((*p).vbase,REG_APIMASK_OFFSET,v)}let mut v=rd((*p).vbase,REG_MCCR_OFFSET);if v&MCCR_ECC_EN==0{wr((*p).vbase,REG_MCCR_OFFSET,v|MCCR_ECC_EN)}}
unsafe fn cpc925_mc_exit(_mci:*mut mem_ctl_info){}
unsafe fn cpc925_get_sdram_scrub_rate(mci:*mut mem_ctl_info)->c_int{let p=(*mci).pvt_info as *mut cpc925_mc_pdata;let v=rd((*p).vbase,REG_MSCR_OFFSET);let si=((v&MSCR_SI_MASK)>>MSCR_SI_SHIFT) as c_int;if v&MSCR_SCRUB_MOD_MASK!=MSCR_BACKGR_SCRUB||si==0{0}else{(CPC925_SCRUB_BLOCK_SIZE*0xFA67/si as u32) as c_int}}
unsafe fn cpc925_mc_get_channels(vbase:*mut c_void)->c_int{let v=rd(vbase,REG_MBCR_OFFSET);if v&MBCR_64BITCFG_MASK==0&&v&MBCR_64BITBUS_MASK==0{1}else{0}}
unsafe fn cpc925_cpu_init(d:*mut cpc925_dev_info){let mut v=rd((*d).vbase,REG_APIMASK_OFFSET);if v&CPU_MASK_ENABLE==0{v|=CPU_MASK_ENABLE}wr((*d).vbase,REG_APIMASK_OFFSET,v)}
unsafe fn cpc925_cpu_exit(_d:*mut cpc925_dev_info){}
unsafe fn cpc925_cpu_check(d:*mut edac_device_ctl_info){let x=(*d).pvt_info as *mut cpc925_dev_info;let e=rd((*x).vbase,REG_APIEXCP_OFFSET);if e&CPU_EXCP_DETECTED!=0{edac_device_handle_ue(d,0,0,(*d).ctl_name)}}
unsafe extern "C" fn cpc925_probe(_pdev:*mut platform_device)->c_int{0}
unsafe extern "C" fn cpc925_remove(_pdev:*mut platform_device){}
unsafe fn cpc925_htlink_init(d:*mut cpc925_dev_info){let v=rd((*d).vbase,REG_ERRCTRL_OFFSET);if v&HT_ERRCTRL_ENABLE==0{wr((*d).vbase,REG_ERRCTRL_OFFSET,v|HT_ERRCTRL_ENABLE)}}
unsafe fn cpc925_htlink_exit(d:*mut cpc925_dev_info){wr((*d).vbase,REG_ERRCTRL_OFFSET,rd((*d).vbase,REG_ERRCTRL_OFFSET)&!HT_ERRCTRL_ENABLE)}
unsafe fn cpc925_htlink_check(d:*mut edac_device_ctl_info){let x=(*d).pvt_info as *mut cpc925_dev_info;let b=rd((*x).vbase,REG_BRGCTRL_OFFSET);let l=rd((*x).vbase,REG_LINKCTRL_OFFSET);let e=rd((*x).vbase,REG_ERRCTRL_OFFSET);let q=rd((*x).vbase,REG_LINKERR_OFFSET);if b&BRGCTRL_DETSERR==0&&l&HT_LINKCTRL_DETECTED==0&&e&HT_ERRCTRL_DETECTED==0&&q&HT_LINKERR_DETECTED==0{return}if b&BRGCTRL_DETSERR!=0{wr((*x).vbase,REG_BRGCTRL_OFFSET,BRGCTRL_DETSERR)}if l&HT_LINKCTRL_DETECTED!=0{wr((*x).vbase,REG_LINKCTRL_OFFSET,HT_LINKCTRL_DETECTED)}if e&ERRCTRL_CHN_FAL!=0{wr((*x).vbase,REG_BRGCTRL_OFFSET,BRGCTRL_SECBUSRESET)}if e&ERRCTRL_RSP_ERR!=0{wr((*x).vbase,REG_ERRCTRL_OFFSET,ERRCTRL_RSP_ERR)}if q&HT_LINKERR_DETECTED!=0{wr((*x).vbase,REG_LINKERR_OFFSET,HT_LINKERR_DETECTED)}edac_device_handle_ce(d,0,0,(*d).ctl_name)}
static mut CPC925_DEVS:[cpc925_dev_info;3]=[cpc925_dev_info{vbase:core::ptr::null_mut(),pdev:core::ptr::null_mut(),ctl_name:core::ptr::null_mut(),edac_idx:0,edac_dev:core::ptr::null_mut(),init:None,exit:None,check:None};3];
static mut CPC925_EDAC_DRIVER:platform_driver=platform_driver{probe:None,remove:None,driver:driver{name:b"cpc925_edac\0".as_ptr() as _}};
unsafe extern "C" fn cpc925_edac_init()->c_int{edac_op_state=2;platform_driver_register(&mut CPC925_EDAC_DRIVER)}
unsafe extern "C" fn cpc925_edac_exit(){platform_driver_unregister(&mut CPC925_EDAC_DRIVER)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
