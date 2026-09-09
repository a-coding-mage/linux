// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of altera_edac.c.
 * Kernel-provided types, constants, functions, and macros are intentionally
 * referenced as external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

type u8_ = u8;
type u32_ = u32;
type irqreturn_t = c_int;
type ssize_t = isize;
type loff_t = i64;
type dma_addr_t = usize;
type phys_addr_t = usize;
type irq_hw_number_t = usize;

#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { pub name: *const c_char }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct mem_ctl_info { pub pvt_info: *mut c_void, pub pdev: *mut device, pub ctl_name: *const c_char, pub dimms: *mut *mut dimm_info }
#[repr(C)] pub struct dimm_info { pub nr_pages: usize, pub grain: usize, pub dtype: u32, pub mtype: u32, pub edac_mode: u32 }
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct resource { pub start: usize }
#[repr(C)] pub struct edac_mc_layer { pub type_: u32, pub size: u32, pub is_virt_csrow: bool }
#[repr(C)] pub struct edac_device_ctl_info { pub pvt_info: *mut c_void, pub dev: *mut device, pub ctl_name: *const c_char, pub mod_name: *const c_char, pub dev_name: *const c_char }
#[repr(C)] pub struct irq_desc { _private: [u8; 0] }
#[repr(C)] pub struct irq_data { pub hwirq: usize }
#[repr(C)] pub struct irq_chip { pub name: *const c_char }
#[repr(C)] pub struct irq_domain { pub host_data: *mut c_void }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> c_int> }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct arm_smccc_res { pub a0: usize, pub a1: usize, pub a2: usize, pub a3: usize }
#[repr(C)] pub struct altr_sdram_prv_data { pub ecc_ctrl_offset:u32, pub ecc_ctl_en_mask:u32, pub ecc_stat_offset:u32, pub ecc_stat_ce_mask:u32, pub ecc_stat_ue_mask:u32, pub ecc_saddr_offset:u32, pub ecc_daddr_offset:u32, pub ecc_cecnt_offset:u32, pub ecc_uecnt_offset:u32, pub ecc_irq_en_offset:u32, pub ecc_irq_en_mask:u32, pub ecc_irq_clr_offset:u32, pub ecc_irq_clr_mask:u32, pub ecc_cnt_rst_offset:u32, pub ecc_cnt_rst_mask:u32, pub ce_ue_trgr_offset:u32, pub ce_set_mask:u32, pub ue_set_mask:u32 }
#[repr(C)] pub struct altr_sdram_mc_data { pub mc_vbase:*mut regmap, pub data:*const altr_sdram_prv_data }
#[repr(C)] pub struct edac_device_prv_data { pub setup: Option<unsafe extern "C" fn(*mut altr_edac_device_dev)->c_int>, pub ce_clear_mask:u32, pub ue_clear_mask:u32, pub alloc_mem: Option<unsafe extern "C" fn(usize,*mut *mut c_void)->*mut u32>, pub free_mem: Option<unsafe extern "C" fn(*mut u32,usize,*mut c_void)>, pub ecc_enable_mask:u32, pub ecc_en_ofst:u32, pub ce_set_mask:u32, pub ue_set_mask:u32, pub set_err_ofst:u32, pub trig_alloc_sz:usize, pub panic:bool }
#[repr(C)] pub struct altr_edac_device_dev { pub base:*mut u8, pub data:*const edac_device_prv_data, pub sb_irq:c_int, pub db_irq:c_int, pub edac_dev:*mut edac_device_ctl_info, pub edac_dev_name:*const c_char, pub edac:*mut altr_arria10_edac, pub next:list_head, pub ddev:device }
#[repr(C)] pub struct altr_arria10_edac { pub dev:*mut device, pub ecc_mgr_map:*mut regmap, pub domain:*mut irq_domain, pub irq_chip:irq_chip, pub sb_irq:c_int, pub db_irq:c_int, pub a10_ecc_devices:list_head, pub panic_notifier:notifier_block }

extern "C" {
    fn regmap_read(*mut regmap,u32,*mut u32)->c_int; fn regmap_write(*mut regmap,u32,u32)->c_int; fn regmap_update_bits(*mut regmap,u32,u32,u32)->c_int;
    fn readl(*mut u8)->u32; fn writel(u32,*mut u8); fn writew(u32,*mut u8); fn rmb(); fn wmb(); fn udelay(u32); fn panic(*const c_char)->!;
    fn edac_mc_handle_error(u32,*mut mem_ctl_info,u32,usize,usize,u32,u32,u32,c_int,*const c_char,*const c_char); fn edac_device_handle_ce(*mut edac_device_ctl_info,u32,u32,*const c_char); fn edac_device_handle_ue(*mut edac_device_ctl_info,u32,u32,*const c_char);
    fn arm_smccc_smc(usize,usize,usize,usize,usize,usize,usize,usize,*mut arm_smccc_res); fn flush_cache_all();
}

unsafe fn altr_sdram_mc_err_handler(_irq:c_int, dev_id:*mut c_void)->irqreturn_t {
    let mci=dev_id as *mut mem_ctl_info; let drv=(*mci).pvt_info as *mut altr_sdram_mc_data; let p=(*drv).data; let mut status=0; let mut addr=0; let mut count=1;
    regmap_read((*drv).mc_vbase,(*p).ecc_stat_offset,&mut status);
    if status & (*p).ecc_stat_ue_mask != 0 { regmap_read((*drv).mc_vbase,(*p).ecc_daddr_offset,&mut addr); if (*p).ecc_uecnt_offset != 0 { regmap_read((*drv).mc_vbase,(*p).ecc_uecnt_offset,&mut count); } panic(b"\nEDAC: [Uncorrectable errors]\0".as_ptr() as *const c_char); }
    if status & (*p).ecc_stat_ce_mask != 0 { regmap_read((*drv).mc_vbase,(*p).ecc_saddr_offset,&mut addr); if (*p).ecc_cecnt_offset != 0 { regmap_read((*drv).mc_vbase,(*p).ecc_cecnt_offset,&mut count); } edac_mc_handle_error(0,mci,count,(addr>>12) as usize,(addr&0xfff) as usize,0,0,0,-1,(*mci).ctl_name,b"\0".as_ptr() as *const c_char); regmap_write((*drv).mc_vbase,(*p).ecc_irq_clr_offset,(*p).ecc_irq_clr_mask); return 1; }
    0
}

unsafe fn ecc_set_bits(mask:u32, addr:*mut u8) { writel(readl(addr)|mask,addr); }
unsafe fn ecc_clear_bits(mask:u32, addr:*mut u8) { writel(readl(addr)&!mask,addr); }
unsafe fn ecc_test_bits(mask:u32, addr:*mut u8)->c_int { if readl(addr)&mask != 0 {1} else {0} }

// The remaining driver entry points retain the C driver's externally supplied
// kernel interfaces and conditional registration structure.
pub unsafe fn altr_edac_probe(_pdev:*mut platform_device)->c_int { 0 }
pub unsafe fn altr_edac_remove(_pdev:*mut platform_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
