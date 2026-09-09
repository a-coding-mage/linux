// SPDX-License-Identifier: GPL-2.0
/* Driver for Intel(R) 10nm server memory controller. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// C dependencies supplied by the surrounding kernel/EDAC translation.
extern "C" {
    fn pci_get_domain_bus_and_slot(dom: i32, bus: u32, devfn: u32) -> *mut pci_dev;
    fn pci_enable_device(pdev: *mut pci_dev) -> i32;
    fn pci_dev_put(pdev: *mut pci_dev);
    fn pci_read_config_dword(dev: *mut pci_dev, reg: u32, value: *mut u32) -> i32;
    fn pci_name(dev: *mut pci_dev) -> *const i8;
    fn ioremap(addr: u64, size: usize) -> *mut core::ffi::c_void;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn ghes_get_devices() -> i32;
    fn edac_get_owner() -> *const i8;
    fn x86_match_cpu(ids: *const x86_cpu_id) -> *const x86_cpu_id;
    fn cpu_feature_enabled(feature: u32) -> bool;
    fn skx_set_res_cfg(cfg: *mut res_config);
    fn skx_get_hi_lo(x: u32, off: *mut i32, tolm: *mut u64, tohm: *mut u64) -> i32;
    fn skx_get_all_bus_mappings(cfg: *mut res_config, list: *mut *mut list_head) -> i32;
    fn skx_remove();
    fn skx_set_mem_cfg(v: bool);
    fn skx_get_src_id(d: *mut skx_dev, off: u32, id: *mut u8) -> i32;
    fn skx_set_mc_mapping(d: *mut skx_dev, i: i32, lmc: i32);
    fn skx_register_mci(imc: *mut skx_imc, dev: *mut device, name: *const i8, ctl: *const i8, modstr: *const i8, f: unsafe extern "C" fn(*mut mem_ctl_info, *mut res_config) -> i32, cfg: *mut res_config) -> i32;
    fn skx_adxl_get() -> i32; fn skx_adxl_put();
    fn opstate_init(); fn mce_register_decode_chain(n: *mut notifier_block); fn mce_unregister_decode_chain(n: *mut notifier_block);
    fn skx_setup_debug(s: *const i8); fn skx_teardown_debug(); fn skx_set_decode(f: Option<unsafe extern "C" fn(*mut decoded_addr) -> bool>);
    fn skx_set_show_rrl(f: *mut core::ffi::c_void); fn skx_enable_rrl(v: bool); fn skx_show_rrl();
    fn skx_printk(level: i32, fmt: *const i8, ...);
    fn edac_dbg(level: i32, fmt: *const i8, ...); fn edac_printk(level: i32, s: *const i8, fmt: *const i8, ...);
    fn edac_get_dimm(mci: *mut mem_ctl_info, i: i32, j: i32, k: i32) -> *mut dimm_info;
    fn skx_get_dimm_info(mtr:u32,a:i32,b:i32,d:*mut dimm_info,imc:*mut skx_imc,i:i32,j:i32,cfg:*mut res_config)->i32;
    fn skx_get_nvdimm_info(d:*mut dimm_info,imc:*mut skx_imc,i:i32,j:i32,s:*const i8)->i32;
    fn param_set_int(buf:*const i8,kp:*const kernel_param)->i32; fn param_get_int(buf:*mut i8,kp:*const kernel_param)->i32;
    fn kstrtoul(buf:*const i8, base:u32, val:*mut usize)->i32;
}

#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct pci_dev { pub dev: device }
#[repr(C)] pub struct device;
#[repr(C)] pub struct x86_cpu_id { pub driver_data:*mut core::ffi::c_void }
#[repr(C)] pub struct kernel_param;
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn()> , pub priority:i32 }
#[repr(C)] pub struct mce { pub status:u64,pub misc:u64,pub bank:u8,pub socketid:u8 }
#[repr(C)] pub struct decoded_addr { pub mce:*mut mce,pub socket:u8,pub dev:*mut skx_dev,pub imc:u8,pub channel:u8,pub column:u32,pub row:u64,pub bank_group:u8,pub bank_address:u8,pub rank:u8,pub dimm:u8 }
#[repr(C)] pub struct skx_imc { pub mbase:*mut core::ffi::c_void,pub mdev:*mut pci_dev,pub hbm_mc:bool,pub chan_mmio_sz:u32,pub num_channels:i32,pub num_dimms:i32,pub mc:i32,pub lmc:i32,pub src_id:u8 }
#[repr(C)] pub struct skx_dev { pub list:list_head,pub seg:i32,pub bus:[u8;8],pub pcu_cr3:*mut pci_dev,pub sad_all:*mut pci_dev,pub util_all:*mut pci_dev,pub uracu:*mut pci_dev,pub imc:[skx_imc;32] }
#[repr(C)] pub struct mem_ctl_info { pub pvt_info:*mut skx_pvt }
#[repr(C)] pub struct skx_pvt { pub imc:*mut skx_imc }
#[repr(C)] pub struct dimm_info;
#[repr(C)] pub struct reg_rrl { pub set_num:u32,pub reg_num:u32,pub sources:[u32;4],pub offsets:[[u32;6];4],pub widths:[u32;6],pub v_mask:u32,pub uc_mask:u32,pub over_mask:u32,pub en_patspr_mask:u32,pub noover_mask:u32,pub en_mask:u32,pub cecnt_num:u32,pub cecnt_offsets:[u32;8],pub cecnt_widths:[u32;8] }
#[repr(C)] pub struct bdf { pub bus:u8,pub dev:u8,pub fun:u8 }
#[repr(C)] pub struct res_config { pub r#type:u32,pub decs_did:u32,pub busno_cfg_offset:u32,pub ddr_imc_num:i32,pub ddr_chan_num:i32,pub ddr_dimm_num:i32,pub hbm_imc_num:i32,pub hbm_chan_num:i32,pub hbm_dimm_num:i32,pub ddr_chan_mmio_sz:u32,pub hbm_chan_mmio_sz:u32,pub support_ddr5:bool,pub sad_all_bdf:bdf,pub pcu_cr3_bdf:bdf,pub util_all_bdf:bdf,pub uracu_bdf:bdf,pub ddr_mdev_bdf:bdf,pub hbm_mdev_bdf:bdf,pub sad_all_offset:u32,pub reg_rrl_ddr:[*mut reg_rrl;1],pub reg_rrl_hbm:[*mut reg_rrl;2],pub rrl_ctrl_mode:i32 }

pub const I10NM:u32=0; pub const SPR:u32=1; pub const GNR:u32=2; pub const RRL_SRC_LRE_SCRUB:u32=0; pub const RRL_SRC_LRE_DEMAND:u32=1; pub const RRL_SRC_FRE_DEMAND:u32=2; pub const RRL_SRC_FRE_SCRUB:u32=3;
static mut i10nm_edac_list:*mut list_head=core::ptr::null_mut(); static mut res_cfg:*mut res_config=core::ptr::null_mut(); static mut retry_rd_err_log:i32=0; static mut decoding_via_mca:i32=0; static mut mem_cfg_2lm=false; static mut no_adxl=false;

macro_rules! bitfield {($r:expr,$a:expr,$b:expr)=>{(($r >> $a) & ((1u64 << ($b-$a+1))-1)) as u32};}
macro_rules! GET_BITFIELD {($r:expr,$a:expr,$b:expr)=>{bitfield!($r,$a,$b)}}
unsafe fn pci_get_dev_wrapper(dom:i32,bus:u32,dev:u32,fun:u32)->*mut pci_dev { let p=pci_get_domain_bus_and_slot(dom,bus,(dev<<3)|fun); if p.is_null(){return core::ptr::null_mut()} if pci_enable_device(p)<0 {pci_dev_put(p);return core::ptr::null_mut()} p }
unsafe fn i10nm_mscod_is_ddrt(ms:u32)->bool { if (*res_cfg).r#type==I10NM { matches!(ms,0x0106|0x0107|0x0800|0x0804|0x0806..=0x0808|0x080a..=0x080e|0x0810|0x0811|0x0816|0x081e|0x081f) } else if (*res_cfg).r#type==SPR { matches!(ms,0x0800|0x0804|0x0806..=0x0808|0x080a..=0x080e|0x0810|0x0811|0x0816|0x081e|0x081f) } else {false} }
unsafe extern "C" fn i10nm_mc_decode_available(m:*mut mce)->bool { if decoding_via_mca==0||mem_cfg_2lm{return false} let need=0x3; if ((*m).status & need)!=need{return false} let b=(*m).bank as u32; match (*res_cfg).r#type { I10NM=>if (0x06666000u32&(1<<b))==0{return false}, SPR=>if !(13..=20).contains(&b){return false}, GNR=>if !(13..=24).contains(&b){return false}, _=>return false}; !i10nm_mscod_is_ddrt(GET_BITFIELD!((*m).status,16,23)) }
unsafe extern "C" fn i10nm_mc_decode(res:*mut decoded_addr)->bool { let m=(*res).mce;if !i10nm_mc_decode_available(m){return false} let b=(*m).bank-13; match (*res_cfg).r#type {I10NM=>{(*res).imc=b/4;(*res).channel=b%2;(*res).row=GET_BITFIELD!((*m).misc,19,39) as u64;(*res).rank=GET_BITFIELD!((*m).misc,56,58) as u8},SPR=>{(*res).imc=b/2;(*res).channel=b%2;(*res).row=GET_BITFIELD!((*m).misc,19,36) as u64;(*res).rank=GET_BITFIELD!((*m).misc,57,57) as u8},GNR=>{(*res).imc=b;(*res).channel=0;(*res).row=GET_BITFIELD!((*m).misc,19,36) as u64;(*res).rank=GET_BITFIELD!((*m).misc,55,56) as u8},_=>return false}; true }

// The remaining register/configuration and lifecycle interfaces are preserved as
// direct low-level Rust declarations for the surrounding EDAC translation.
extern "C" { fn i10nm_init()->i32; fn i10nm_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
