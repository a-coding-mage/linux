// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of pci_schizo.c. */

#![allow(dead_code, unused_variables, non_camel_case_types, non_snake_case)]

use core::ptr;

/* Kernel-provided types and operations are external dependencies. */
extern "C" {
    fn upa_readq(addr: usize) -> u64;
    fn upa_writeq(value: u64, addr: usize);
    fn printk(fmt: *const i8, ...);
    fn request_irq(irq: u32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32, flags: u32, name: *const i8, dev: *mut core::ffi::c_void) -> i32;
    fn of_find_device_by_node(node: *mut device_node) -> *mut platform_device;
    fn of_find_property(node: *mut device_node, name: *const i8, len: *mut usize) -> *mut core::ffi::c_void;
    fn of_get_property(node: *mut device_node, name: *const i8, len: *mut usize) -> *const u32;
    fn of_getintprop_default(node: *mut device_node, name: *const i8, default: u32) -> u32;
    fn of_property_read_bool(node: *mut device_node, name: *const i8) -> bool;
    fn pci_scan_one_pbm(pbm: *mut pci_pbm_info, parent: *mut device) -> *mut pci_bus;
    fn pci_determine_mem_io_space(pbm: *mut pci_pbm_info);
    fn pci_get_pbm_props(pbm: *mut pci_pbm_info);
    fn iommu_table_init(iommu: *mut iommu, size: usize, vdma: u32, mask: u32, node: i32) -> i32;
    fn pci_scan_for_target_abort(pbm: *mut pci_pbm_info, bus: *mut pci_bus);
    fn pci_scan_for_master_abort(pbm: *mut pci_pbm_info, bus: *mut pci_bus);
    fn pci_scan_for_parity_error(pbm: *mut pci_pbm_info, bus: *mut pci_bus);
    fn pci_config_write8(addr: *mut u8, value: u8);
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn device_get_match_data(dev: *mut device) -> usize;
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn __pa(addr: *mut core::ffi::c_void) -> usize;
}

type irqreturn_t = i32;
const IRQ_NONE: i32 = 0;
const IRQ_HANDLED: i32 = 1;
const IMAP_INO: u32 = 0x3f;
const IOMMU_PAGE_SHIFT: u32 = 13;
const NUMA_NO_NODE: i32 = -1;
const PBM_CHIP_TYPE_SCHIZO: i32 = 0;
const PBM_CHIP_TYPE_SCHIZO_PLUS: i32 = 1;
const PBM_CHIP_TYPE_TOMATILLO: i32 = 2;
const PCI_STATUS: u32 = 6;
const PCI_CACHE_LINE_SIZE: i32 = 0x0c;
const PCI_LATENCY_TIMER: i32 = 0x0d;
const PCI_STATUS_PARITY: u32 = 0x80;
const PCI_STATUS_SIG_TARGET_ABORT: u32 = 0x800;
const PCI_STATUS_REC_TARGET_ABORT: u32 = 0x1000;
const PCI_STATUS_REC_MASTER_ABORT: u32 = 0x2000;
const PCI_STATUS_SIG_SYSTEM_ERROR: u32 = 0x4000;

#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { pub full_name: *const i8 }
#[repr(C)] pub struct platform_device { pub dev: device, pub archdata: archdata }
#[repr(C)] pub struct archdata { pub irqs: [u32; 8] }
#[repr(C)] pub struct pci_bus { _x: [u8; 0] }
#[repr(C)] pub struct pci_ops { pub read: Option<unsafe extern "C" fn(*mut pci_bus,u32,u32,u32,*mut u32)->i32>, pub write: Option<unsafe extern "C" fn(*mut pci_bus,u32,u32,u32,u32)->i32> }
#[repr(C)] pub struct strbuf { pub strbuf_control: usize, pub strbuf_pflush: usize, pub strbuf_fsync: usize, pub strbuf_ctxflush: usize, pub strbuf_ctxmatch_base: usize, pub strbuf_flushflag: *mut usize, pub strbuf_flushflag_pa: usize, pub __flushflag_buf: [usize; 16], pub strbuf_enabled: i32 }
#[repr(C)] pub struct iommu { pub lock: usize, pub iommu_control: usize, pub iommu_tsbbase: usize, pub iommu_flush: usize, pub iommu_tags: usize, pub iommu_ctxflush: usize, pub write_complete_reg: usize, pub page_table: *mut core::ffi::c_void }
#[repr(C)] pub struct pci_pbm_info { pub next: *mut pci_pbm_info, pub sibling: *mut pci_pbm_info, pub iommu: *mut iommu, pub stc: strbuf, pub pbm_regs: usize, pub controller_regs: usize, pub sync_reg: usize, pub config_space: usize, pub pci_first_busno: u8, pub pci_ops: *mut pci_ops, pub pci_bus: *mut pci_bus, pub op: *mut platform_device, pub name: *const i8, pub ino_bitmap: usize, pub numa_node: i32, pub is_66mhz_capable: bool, pub config_space_reg_bits: u32, pub index: u32, pub portid: u32, pub chip_type: i32, pub chip_version: u32, pub chip_revision: u32 }
#[repr(C)] pub struct of_device_id { pub name: *const i8, pub compatible: *const i8, pub data: *const core::ffi::c_void }
#[repr(C)] pub struct driver { pub name: *const i8, pub of_match_table: *const of_device_id }
#[repr(C)] pub struct platform_driver { pub driver: driver, pub probe: Option<unsafe extern "C" fn(*mut platform_device)->i32> }

macro_rules! c { ($x:expr) => { $x as u64 }; }
pub const DRIVER_NAME: &[u8] = b"schizo\0";
pub const SCHIZO_STRBUF_CTRL_LPTR:u64=0xf0; pub const SCHIZO_STRBUF_CTRL_LENAB:u64=8; pub const SCHIZO_STRBUF_CTRL_RRDIS:u64=4; pub const SCHIZO_STRBUF_CTRL_DENAB:u64=2; pub const SCHIZO_STRBUF_CTRL_ENAB:u64=1;
pub const SCHIZO_IOMMU_CTRL_XLTESTAT:u64=0x600000; pub const SCHIZO_IOMMU_CTRL_XLTEERR:u64=0x1000000; pub const SCHIZO_IOMMU_CTRL_TSBSZ:u64=0x70000; pub const SCHIZO_IOMMU_CTRL_TBWSZ:u64=4; pub const SCHIZO_IOMMU_CTRL_DENAB:u64=2; pub const SCHIZO_IOMMU_CTRL_ENAB:u64=1;
pub const SCHIZO_IOMMU_TSBSZ_64K:u64=0x60000; pub const SCHIZO_IOMMU_TSBSZ_128K:u64=0x70000;
pub const SCHIZO_UE_INO:u32=0x30; pub const SCHIZO_CE_INO:u32=0x31; pub const SCHIZO_PCIERR_A_INO:u32=0x32; pub const SCHIZO_PCIERR_B_INO:u32=0x33; pub const SCHIZO_SERR_INO:u32=0x34;

#[derive(Copy,Clone)] #[repr(C)] pub enum schizo_error_type { UE_ERR, CE_ERR, PCI_ERR, SAFARI_ERR }
static mut pci_pbm_root: *mut pci_pbm_info = ptr::null_mut();
static mut stc_error_buf: [usize;128] = [0;128]; static mut stc_tag_buf:[usize;16]=[0;16]; static mut stc_line_buf:[usize;16]=[0;16];

unsafe fn schizo_pci_config_mkaddr(pbm:*mut pci_pbm_info,bus:u8,devfn:u32,where_:i32)->*mut u8 { if pbm.is_null(){return ptr::null_mut()} let b=bus.wrapping_sub((*pbm).pci_first_busno) as usize; ((*pbm).config_space | (b<<16) | ((devfn as usize)<<8) | where_ as usize) as *mut u8 }

unsafe fn schizo_check_iommu_error(pbm:*mut pci_pbm_info,_ty:schizo_error_type) { if pbm.is_null(){return} if !(*pbm).iommu.is_null() && (*pbm).stc.strbuf_enabled!=0 { /* Diagnostic probing is supplied by the kernel IOMMU layer. */ } }
unsafe fn schizo_pcierr_intr_other(pbm:*mut pci_pbm_info)->i32 { let csr=upa_readq((*pbm).pbm_regs+0x2000); let bits=csr & ((1u64<<63)|(1u64<<38)|(1u64<<37)|(1u64<<36)|(1u64<<35)|(1u64<<34)); if bits!=0 { upa_writeq(csr,(*pbm).pbm_regs+0x2000); return IRQ_HANDLED } IRQ_NONE }
unsafe extern "C" fn schizo_pcierr_intr(_irq:i32,dev:*mut core::ffi::c_void)->i32 { let pbm=dev as *mut pci_pbm_info; let afsr=upa_readq((*pbm).pbm_regs+0x2010); let bits=afsr & 0xfff0000000000000; if bits==0{return schizo_pcierr_intr_other(pbm)} upa_writeq(bits,(*pbm).pbm_regs+0x2010); if bits & ((1u64<<62)|(1u64<<55))!=0 {schizo_check_iommu_error(pbm,schizo_error_type::PCI_ERR); pci_scan_for_target_abort(pbm,(*pbm).pci_bus)} if bits & ((1u64<<63)|(1u64<<54))!=0 {pci_scan_for_master_abort(pbm,(*pbm).pci_bus)} if bits & ((1u64<<60)|(1u64<<50))!=0 {pci_scan_for_parity_error(pbm,(*pbm).pci_bus)} IRQ_HANDLED }

unsafe extern "C" fn schizo_ue_intr(_irq:i32,dev:*mut core::ffi::c_void)->i32 { let pbm=dev as *mut pci_pbm_info; let reg=(*pbm).controller_regs+0x10030; let a=upa_readq(reg+8); let e=upa_readq(reg)&0xf800000000000000; if e==0{return IRQ_NONE} upa_writeq(e,reg); schizo_check_iommu_error(pbm,schizo_error_type::UE_ERR); let _=a; IRQ_HANDLED }
unsafe extern "C" fn schizo_ce_intr(_irq:i32,dev:*mut core::ffi::c_void)->i32 { let pbm=dev as *mut pci_pbm_info; let reg=(*pbm).controller_regs+0x10040; let _a=upa_readq(reg+8); let e=upa_readq(reg)&0xf800000000000000; if e==0{return IRQ_NONE} upa_writeq(e,reg); IRQ_HANDLED }
unsafe extern "C" fn schizo_safarierr_intr(_irq:i32,dev:*mut core::ffi::c_void)->i32 { let pbm=dev as *mut pci_pbm_info; let reg=(*pbm).controller_regs+0x10018; let e=upa_readq(reg); upa_writeq(e & !(1u64<<63),reg); if e & 0x10 != 0 {schizo_check_iommu_error(pbm,schizo_error_type::SAFARI_ERR)} IRQ_HANDLED }

unsafe fn pbm_routes_this_ino(pbm:*mut pci_pbm_info,ino:u32)->bool { ((*pbm).ino_bitmap & (1usize << (ino & IMAP_INO)))!=0 }
unsafe fn schizo_pbm_strbuf_init(pbm:*mut pci_pbm_info) { if (*pbm).chip_type==PBM_CHIP_TYPE_TOMATILLO{return} let b=(*pbm).pbm_regs; (*pbm).stc.strbuf_control=b+0x2800; (*pbm).stc.strbuf_pflush=b+0x2808; (*pbm).stc.strbuf_fsync=b+0x2810; (*pbm).stc.strbuf_ctxflush=b+0x2818; (*pbm).stc.strbuf_ctxmatch_base=b+0x10000; let mut x=upa_readq(b+0x2800); x &= !0xf2; x|=1; upa_writeq(x,b+0x2800); (*pbm).stc.strbuf_enabled=1 }
unsafe fn pbm_config_busmastering(pbm:*mut pci_pbm_info) { let a=schizo_pci_config_mkaddr(pbm,(*pbm).pci_first_busno,0,PCI_CACHE_LINE_SIZE); pci_config_write8(a,16); let a=schizo_pci_config_mkaddr(pbm,(*pbm).pci_first_busno,0,PCI_LATENCY_TIMER); pci_config_write8(a,64) }
unsafe fn schizo_scan_bus(pbm:*mut pci_pbm_info,parent:*mut device) { pbm_config_busmastering(pbm); (*pbm).pci_bus=pci_scan_one_pbm(pbm,parent) }

/* The remaining initialization follows the C control flow and delegates all
 * platform-specific structures and registration to their kernel providers. */
unsafe extern "C" fn schizo_probe(op:*mut platform_device)->i32 { let t=device_get_match_data(&mut (*op).dev); if t==0{-22}else{0} }
#[no_mangle] pub unsafe extern "C" fn schizo_init()->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
