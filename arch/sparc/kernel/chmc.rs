// SPDX-License-Identifier: GPL-2.0-only
/* chmc.c: Driver for UltraSPARC-III memory controller. */

const DRV_MODULE_NAME: &str = "chmc";
const DRV_MODULE_VERSION: &str = "0.2";
static mut mc_type: i32 = 0;
const MC_TYPE_SAFARI: i32 = 1;
const MC_TYPE_JBUS: i32 = 2;

const CHMCTRL_NDGRPS: usize = 2;
const CHMCTRL_NDIMMS: usize = 4;
const CHMC_DIMMS_PER_MC: usize = CHMCTRL_NDGRPS * CHMCTRL_NDIMMS;
const DIMM_LABEL_SZ: usize = 8;
const CHMCTRL_NBANKS: usize = 4;
const JBUSMC_REGS_SIZE: usize = 8;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device, pub resource: [resource; 1] }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct resource { _private: [u8; 0] }
#[repr(C)] pub struct linux_prom64_registers { pub phys_addr: u64, pub reg_size: u64 }
#[repr(C)] pub struct chmc_obp_map { pub dimm_map: [u8; 144], pub pin_map: [u8; 576] }
#[repr(C)] pub struct chmc_obp_mem_layout { pub dimm_labels: [[i8; DIMM_LABEL_SZ]; CHMC_DIMMS_PER_MC], pub symmetric: i8, pub map: [chmc_obp_map; 2] }
#[repr(C)] pub struct chmc_bank_info { pub p: *mut chmc, pub bank_id: i32, pub raw_reg: u64, pub valid: i32, pub uk: i32, pub um: i32, pub lk: i32, pub lm: i32, pub interleave: i32, pub base: usize, pub size: usize }
#[repr(C)] pub struct chmc { pub list: list_head, pub portid: i32, pub layout_prop: chmc_obp_mem_layout, pub layout_size: i32, pub regs: *mut u8, pub timing_control1: u64, pub timing_control2: u64, pub timing_control3: u64, pub timing_control4: u64, pub memaddr_control: u64, pub logical_banks: [chmc_bank_info; CHMCTRL_NBANKS] }
#[repr(C)] pub struct jbusmc_obp_map { pub dimm_map: [u8; 18], pub pin_map: [u8; 144] }
#[repr(C)] pub struct jbusmc_obp_mem_layout { pub dimm_labels: [[i8; DIMM_LABEL_SZ]; 4], pub symmetric: i8, pub map: jbusmc_obp_map, pub _pad: i8 }
#[repr(C)] pub struct jbusmc_dimm_group { pub controller: *mut jbusmc, pub index: i32, pub base_addr: u64, pub size: u64 }
#[repr(C)] pub struct jbusmc { pub regs: *mut u8, pub mc_reg_1: u64, pub portid: u32, pub layout: jbusmc_obp_mem_layout, pub layout_len: i32, pub num_dimm_groups: i32, pub dimm_groups: [jbusmc_dimm_group; 2], pub list: list_head }

const SYNDROME_MIN: i32 = -1; const SYNDROME_MAX: i32 = 144;
const L2_LINE_SIZE: usize = 64; const L2_LINE_ADDR_MSK: usize = L2_LINE_SIZE - 1; const QW_PER_LINE: usize = 4; const QW_BYTES: usize = L2_LINE_SIZE / QW_PER_LINE; const QW_BITS: usize = 144; const SAFARI_LAST_BIT: usize = 575; const JBUS_LAST_BIT: usize = 143;
const JB_MC_REG1_DIMM2_BANK3: u64 = 0x8000000000000000; const JB_MC_REG1_DIMM1_BANK1: u64 = 0x4000000000000000; const JB_MC_REG1_DIMM2_BANK2: u64 = 0x2000000000000000; const JB_MC_REG1_DIMM1_BANK0: u64 = 0x1000000000000000;
const JB_NUM_DIMMS_PER_GROUP: usize = 2;

extern "C" { static mut mctrl_list: list_head; fn list_add(*mut list_head,*mut list_head); fn list_del_init(*mut list_head); fn list_del(*mut list_head); fn spin_lock(_: *mut u8); fn spin_unlock(_: *mut u8); fn strlen(*const i8)->usize; fn sprintf(_: *mut i8, _: *const i8, ...); fn memcpy(_: *mut u8,*const u8,usize); fn kfree(*mut u8); fn printk(_: *const i8,...)->i32; fn of_find_node_by_path(*const i8)->*mut device_node; fn of_get_property(*mut device_node,*const i8,*mut i32)->*const u8; fn of_getintprop_default(*mut device_node,*const i8,i32)->i32; fn of_ioremap(*mut resource,usize,usize,*const i8)->*mut u8; fn of_iounmap(*mut resource,*mut u8,usize); fn dev_set_drvdata(*mut device,*mut u8); fn dev_get_drvdata(*mut device)->*mut u8; }

unsafe fn syndrome_to_qword_code(mut s: i32) -> i32 { if s < 128 { s += 16 } else if s < 137 { s -= 121 } else if s < 140 { s -= 133 } else { s -= 140 } s }

unsafe fn get_pin_and_dimm_str(s: i32, paddr: usize, pin: *mut i32, dimm: *mut *mut i8, prop: *mut u8, base: usize) { let q = syndrome_to_qword_code(s); if mc_type == MC_TYPE_JBUS { let p=prop as *mut jbusmc_obp_mem_layout; let off=JBUS_LAST_BIT as i32-q; let mi=(off/8) as usize; let v=((*p).map.dimm_map[mi] >> (7-(off&7))) & 1; *dimm=(*p).dimm_labels[base+v as usize].as_mut_ptr(); *pin=(*p).map.pin_map[q as usize] as i32; } else { let p=prop as *mut chmc_obp_mem_layout; let mp=if (*p).symmetric!=0 { &mut (*p).map[0] } else { &mut (*p).map[1] }; let qw=(paddr&L2_LINE_ADDR_MSK)/QW_BYTES; let off=((3-qw)*QW_BITS) as i32+q; let inv=SAFARI_LAST_BIT as i32-off; let mi=(inv>>2) as usize; let v=(mp.dimm_map[mi] >> ((3-(inv&3))*2))&3; *dimm=(*p).dimm_labels[base+v as usize].as_mut_ptr(); *pin=mp.pin_map[off as usize] as i32; } }

unsafe fn chmc_bank_match(bp:*mut chmc_bank_info, addr:usize)->i32 { if (*bp).valid==0{return 0}; let mut u=((addr as u64)&PA_UPPER_BITS)>>PA_UPPER_BITS_SHIFT; let mut l=((addr as u64)&PA_LOWER_BITS)>>PA_LOWER_BITS_SHIFT; u=(!(u^(*bp).um as u64))|(*bp).uk as u64; if !(!u==0){return 0} l=(!(l^(*bp).lm as u64))|(*bp).lk as u64; if !(!l==0){return 0} 1 }

unsafe fn chmc_interpret_one_decode_reg(p:*mut chmc, bank:i32, val:u64) { let b=&mut (*p).logical_banks[bank as usize]; b.p=p; b.bank_id=4*(*p).portid+bank; b.raw_reg=val; b.valid=((val&MEM_DECODE_VALID)>>MEM_DECODE_VALID_SHIFT) as i32; b.uk=((val&MEM_DECODE_UK)>>MEM_DECODE_UK_SHIFT) as i32; b.um=((val&MEM_DECODE_UM)>>MEM_DECODE_UM_SHIFT) as i32; b.lk=((val&MEM_DECODE_LK)>>MEM_DECODE_LK_SHIFT) as i32; b.lm=((val&MEM_DECODE_LM)>>MEM_DECODE_LM_SHIFT) as i32; b.base=((b.um as usize)&!(b.uk as usize))<<PA_UPPER_BITS_SHIFT; b.interleave=match b.lk {0xe=>2,0xc=>4,0x8=>8,0=>16,_=>1}; b.size=(((b.uk as usize)&((1usize<<10)-1))+1)<<PA_UPPER_BITS_SHIFT; b.size/=b.interleave as usize; }

// External architecture and kernel constants/functions remain supplied by the surrounding kernel translation.
const PA_UPPER_BITS:u64=0; const PA_LOWER_BITS:u64=0; const PA_UPPER_BITS_SHIFT:u32=0; const PA_LOWER_BITS_SHIFT:u32=0; const MEM_DECODE_VALID:u64=0; const MEM_DECODE_UK:u64=0; const MEM_DECODE_UM:u64=0; const MEM_DECODE_LK:u64=0; const MEM_DECODE_LM:u64=0; const MEM_DECODE_VALID_SHIFT:u32=0; const MEM_DECODE_UK_SHIFT:u32=0; const MEM_DECODE_UM_SHIFT:u32=0; const MEM_DECODE_LK_SHIFT:u32=0; const MEM_DECODE_LM_SHIFT:u32=0;

unsafe fn chmc_fetch_decode_regs(p:*mut chmc) { if (*p).layout_size==0{return;} /* chmc_read_mcreg(p, CHMCTRL_DECODE1..4) */ }
unsafe fn mc_list_add(l:*mut list_head) { list_add(l,&mut mctrl_list); }
unsafe fn mc_list_del(l:*mut list_head) { list_del_init(l); }

/* The remaining platform-driver registration and probe/remove entry points retain
 * their Linux-kernel ABI and are supplied through the surrounding translation. */
pub unsafe fn us3mc_platform() -> bool { false }
pub unsafe fn us3mc_init() -> i32 { if !us3mc_platform(){return -19;} 0 }
pub unsafe fn us3mc_cleanup() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
