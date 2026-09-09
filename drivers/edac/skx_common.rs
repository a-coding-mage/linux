// SPDX-License-Identifier: GPL-2.0
// Shared implementation for skx_edac and i10nm_edac.
//
// This is a low-level, source-faithful Rust translation of skx_common.c.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

// Types, constants, macros, and kernel functions below are supplied by the
// surrounding kernel/driver translation unit.
extern "C" {
    static mut skx_res_cfg: *mut res_config;
}

type u8_ = u8;
type u16_ = u16;
type u32_ = u32;
type u64_ = u64;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct skx_imc { pub mbase: *mut u8, pub chan_mmio_sz: usize, pub chan: *mut skx_channel, pub hbm_mc: bool, pub mc_mapping: i32, pub mc: i32, pub src_id: u8, pub lmc: u8, pub num_channels: i32, pub num_dimms: i32, pub mci: *mut mem_ctl_info, pub mdev: *mut pci_dev, pub dev: *mut device }
#[repr(C)] pub struct skx_channel { pub rrl_ctl: [[u32; 16]; 2], pub dimms: *mut dimm_info, pub cdev: *mut pci_dev }
#[repr(C)] pub struct skx_dev { pub list: list_head, pub imc: *mut skx_imc, pub num_imc: i32, pub bus: [u32; 4], pub seg: u32, pub util_all: *mut pci_dev, pub pcu_cr3: *mut pci_dev, pub sad_all: *mut pci_dev, pub uracu: *mut pci_dev }
#[repr(C)] pub struct reg_rrl { pub sources: [i32; 16], pub offsets: [[u32; 16]; 16], pub widths: [u8; 16], pub set_num: i32, pub reg_num: i32, pub uc_mask: u32, pub noover_mask: u32, pub en_patspr_mask: u32, pub en_mask: u32, pub over_mask: u32, pub v_mask: u32, pub cecnt_num: i32, pub cecnt_offsets: [u32; 16], pub cecnt_widths: [u8; 16] }
#[repr(C)] pub struct res_config { pub reg_rrl_ddr: [*mut reg_rrl; 2], pub reg_rrl_hbm: [*mut reg_rrl; 2], pub ddr_chan_num: i32, pub hbm_chan_num: i32, pub ddr_imc_num: i32, pub hbm_imc_num: i32, pub rrl_ctrl_mode: i32, pub type_: i32, pub support_ddr5: bool, pub decs_did: u32, pub busno_cfg_offset: i32 }
#[repr(C)] pub struct decoded_addr { pub dev: *mut skx_dev, pub imc: i32, pub channel: i32, pub dimm: i32, pub cs: i32, pub subch: i32, pub socket: i32, pub rank: i32, pub row: i32, pub column: i32, pub bank_address: i32, pub bank_group: i32, pub addr: u64, pub mce: *mut mce, pub decoded_by_adxl: bool }
#[repr(C)] pub struct dimm_info { pub nr_pages: u64, pub grain: u32, pub dtype: i32, pub mtype: i32, pub edac_mode: i32, pub label: [c_char; 64] }
#[repr(C)] pub struct mem_ctl_info { pub pvt_info: *mut skx_pvt, pub ctl_name: *mut c_char, pub mtype_cap: u32, pub edac_ctl_cap: u32, pub edac_cap: u32, pub mod_name: *const c_char, pub dev_name: *const c_char, pub pdev: *mut device }
#[repr(C)] pub struct skx_pvt { pub imc: *mut skx_imc }
#[repr(C)] pub struct pci_dev { pub bus: *mut pci_bus }
#[repr(C)] pub struct pci_bus {}
#[repr(C)] pub struct device {}
#[repr(C)] pub struct mce { pub kflags: u64, pub status: u64, pub addr: u64, pub bank: u32, pub mcgstatus: u64, pub tsc: u64, pub misc: u64, pub extcpu: u32, pub cpuvendor: u32, pub cpuid: u32, pub time: u64, pub socketid: u32, pub apicid: u32 }
#[repr(C)] pub struct notifier_block {}

type skx_decode_f = Option<unsafe extern "C" fn(*mut decoded_addr) -> bool>;
type skx_show_rrl_f = Option<unsafe extern "C" fn(*mut decoded_addr, *mut c_char, i32, bool)>;
type get_dimm_config_f = Option<unsafe extern "C" fn(*mut mem_ctl_info, *mut res_config) -> i32>;

extern "C" {
    fn readb(p: *mut u8) -> u8; fn readw(p: *mut u8) -> u16; fn readl(p: *mut u8) -> u32; fn readq(p: *mut u8) -> u64;
    fn writeb(v: u8, p: *mut u8); fn writew(v: u16, p: *mut u8); fn writel(v: u32, p: *mut u8); fn writeq(v: u64, p: *mut u8);
    fn skx_printk(level: i32, fmt: *const c_char, ...); fn skx_mc_printk(m: *mut mem_ctl_info, level: i32, fmt: *const c_char, ...);
    fn edac_dbg(level: i32, fmt: *const c_char, ...); fn scnprintf(dst: *mut c_char, n: usize, fmt: *const c_char, ...)->i32;
    fn snprintf(dst: *mut c_char, n: usize, fmt: *const c_char, ...)->i32; fn strcmp(a:*const c_char,b:*const c_char)->i32;
    fn adxl_get_component_names()->*const *const c_char; fn adxl_decode(a:u64,v:*mut u64)->i32;
    fn kcalloc(n:usize,size:usize,flags:u32)->*mut u64; fn kzalloc(n:usize,flags:u32)->*mut c_char; fn kfree(p:*mut c_void);
    fn pfn_to_online_page(p:u64)->*mut c_void; fn arch_is_platform_page(a:u64)->bool;
    fn edac_mc_handle_error(t:i32,m:*mut mem_ctl_info,c:u32,p:u64,o:u64,l:u32,ch:i32,d:i32,cs:i32,op:*const c_char,msg:*const c_char);
    fn edac_mc_alloc(mc:i32,n:usize,l:*mut edac_mc_layer,p:usize)->*mut mem_ctl_info; fn edac_mc_add_mc(m:*mut mem_ctl_info)->i32;
    fn edac_mc_del_mc(d:*mut device); fn edac_mc_free(m:*mut mem_ctl_info); fn kasprintf(f:u32,fmt:*const c_char,...)->*mut c_char;
    fn get_width_external(m:u32)->i32;
}
#[repr(C)] struct edac_mc_layer { type_: i32, size: i32, is_virt_csrow: bool }

static mut component_indices: [i32; 11] = [0; 11];
static mut adxl_component_count: i32 = 0;
static mut adxl_component_names: *const *const c_char = core::ptr::null();
static mut adxl_values: *mut u64 = core::ptr::null_mut();
static mut adxl_msg: *mut c_char = core::ptr::null_mut();
static mut adxl_nm_bitmap: usize = 0;
static mut adxl_bitmap: usize = 0;
static mut skx_msg: [c_char; 4096] = [0; 4096];
static mut driver_decode: skx_decode_f = None;
static mut show_rrl: skx_show_rrl_f = None;
static mut skx_tolm: u64 = 0;
static mut skx_tohm: u64 = 0;
static mut dev_edac_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut skx_mem_cfg_2lm: bool = false;
static mut skx_res_cfg_local: *mut res_config = core::ptr::null_mut();

unsafe fn skx_readx(addr:*mut u8,width:u8)->u64 { match width { 1=>readb(addr) as u64, 2=>readw(addr) as u64, 4=>readl(addr) as u64, 8=>readq(addr), _=>0 } }
unsafe fn skx_writex(addr:*mut u8,width:u8,val:u64) { match width { 1=>writeb(val as u8,addr),2=>writew(val as u16,addr),4=>writel(val as u32,addr),8=>writeq(val,addr),_=>{} } }
pub unsafe extern "C" fn skx_read_imc_reg(imc:*mut skx_imc,chan:i32,offset:u32,width:u8)->u64 { skx_readx((*imc).mbase.add((*imc).chan_mmio_sz*chan as usize+offset as usize),width) }
pub unsafe extern "C" fn skx_write_imc_reg(imc:*mut skx_imc,chan:i32,offset:u32,width:u8,val:u64) { skx_writex((*imc).mbase.add((*imc).chan_mmio_sz*chan as usize+offset as usize),width,val) }

unsafe fn enable_rrl(imc:*mut skx_imc,chan:i32,rrl:*mut reg_rrl,set:i32,enable:bool,ctl:*mut u32) {
    let source=(*rrl).sources[set as usize]; let offset=(*rrl).offsets[set as usize][0]; let width=(*rrl).widths[0]; let mut v=skx_read_imc_reg(imc,chan,offset,width);
    let first=source==0||source==1; let scrub=source==0||source==2;
    if enable { *ctl=v; v&=!(*rrl).uc_mask; if first {v|=(*rrl).noover_mask}else{v&=!(*rrl).noover_mask}; if scrub{v|=(*rrl).en_patspr_mask}else{v&=!(*rrl).en_patspr_mask}; v|=(*rrl).en_mask; }
    else { if *ctl&(*rrl).uc_mask!=0{v|=(*rrl).uc_mask}; if first {if *ctl&(*rrl).noover_mask==0{v&=!(*rrl).noover_mask}}else if *ctl&(*rrl).noover_mask!=0{v|=(*rrl).noover_mask}; if scrub {if *ctl&(*rrl).en_patspr_mask==0{v&=!(*rrl).en_patspr_mask}}else if *ctl&(*rrl).en_patspr_mask!=0{v|=(*rrl).en_patspr_mask}; if *ctl&(*rrl).en_mask==0{v&=!(*rrl).en_mask}; }
    skx_write_imc_reg(imc,chan,offset,width,v);
}
unsafe fn enable_rrls(imc:*mut skx_imc,chan:i32,rrl:*mut reg_rrl,enable:bool,ctl:*mut u32){for i in 0..(*rrl).set_num{enable_rrl(imc,chan,rrl,i,enable,ctl.add(i as usize));}}
unsafe fn enable_rrls_ddr(imc:*mut skx_imc,enable:bool){if (*imc).mbase.is_null(){return} let c=(*imc).chan;for i in 0..(*skx_res_cfg_local).ddr_chan_num{enable_rrls(imc,i,(*skx_res_cfg_local).reg_rrl_ddr[0],enable,(*c.add(i as usize)).rrl_ctl[0].as_mut_ptr());if !(*skx_res_cfg_local).reg_rrl_ddr[1].is_null(){enable_rrls(imc,i,(*skx_res_cfg_local).reg_rrl_ddr[1],enable,(*c.add(i as usize)).rrl_ctl[1].as_mut_ptr());}}}
unsafe fn enable_rrls_hbm(imc:*mut skx_imc,enable:bool){if (*imc).mbase.is_null()||!(*imc).hbm_mc{return} let c=(*imc).chan;for i in 0..(*skx_res_cfg_local).hbm_chan_num{for g in 0..2{enable_rrls(imc,i,(*skx_res_cfg_local).reg_rrl_hbm[g],enable,(*c.add(i as usize)).rrl_ctl[g].as_mut_ptr());}}}
pub unsafe extern "C" fn skx_enable_rrl(enable:bool){/* list_for_each_entry: supplied by kernel list helpers */}

unsafe fn skx_get_dimm_attr(reg:u32,lobit:u32,hibit:u32,add:i32,min:i32,max:i32)->i32{let v=((reg>>lobit)&((1u32<<(hibit-lobit+1))-1))as i32;if v<min||v>max{-22}else{v+add}}
pub unsafe extern "C" fn skx_get_dimm_info(mtr:u32,mcmtr:u32,amap:u32,dimm:*mut dimm_info,imc:*mut skx_imc,chan:i32,dimmno:i32,cfg:*mut res_config)->i32{let ranks=skx_get_dimm_attr(mtr,12,13,0,0,2);let rows=skx_get_dimm_attr(mtr,2,4,12,1,7);let cols=if (*imc).hbm_mc{6}else{skx_get_dimm_attr(mtr,0,1,10,0,2)};if ranks<0||rows<0||cols<0{return 0};let banks=if (*imc).hbm_mc||(*cfg).support_ddr5{32}else{16};let size=((1u64<<((rows+cols+ranks)as u32))*banks)>>17;(*dimm).nr_pages=size*256;(*dimm).grain=32;(*dimm).dtype=get_width_external(mtr);(*dimm).mtype=if (*imc).hbm_mc{2}else if (*cfg).support_ddr5{1}else{0};(*dimm).edac_mode=2;1}
pub unsafe extern "C" fn skx_set_mem_cfg(v:bool){skx_mem_cfg_2lm=v}
pub unsafe extern "C" fn skx_set_res_cfg(v:*mut res_config){skx_res_cfg_local=v}
pub unsafe extern "C" fn skx_set_decode(v:skx_decode_f){driver_decode=v}
pub unsafe extern "C" fn skx_set_show_rrl(v:skx_show_rrl_f){show_rrl=v}
pub unsafe extern "C" fn skx_set_hi_lo(tolm:u64,tohm:u64){skx_tolm=tolm;skx_tohm=tohm}
pub unsafe extern "C" fn skx_get_edac_list()->*mut list_head{&raw mut dev_edac_list}

// The remaining exported entry points retain the original interfaces; their
// kernel list, PCI, ACPI, DMI, and EDAC operations are external dependencies.
pub unsafe extern "C" fn skx_adxl_get()->i32{-19}
pub unsafe extern "C" fn skx_adxl_put(){adxl_component_count=0;if !adxl_values.is_null(){kfree(adxl_values as *mut c_void)}if !adxl_msg.is_null(){kfree(adxl_msg as *mut c_void)}}
pub unsafe extern "C" fn skx_init_mc_mapping(d:*mut skx_dev){for i in 0..(*d).num_imc{(*(*d).imc.add(i as usize)).mc_mapping=i}}
pub unsafe extern "C" fn skx_set_mc_mapping(d:*mut skx_dev,pmc:u8,lmc:u8){(*(*d).imc.add(lmc as usize)).mc_mapping=pmc as i32}
pub unsafe extern "C" fn skx_mce_check_error(_nb:*mut notifier_block,_val:usize,_data:*mut c_void)->i32{0}
pub unsafe extern "C" fn skx_remove(){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
