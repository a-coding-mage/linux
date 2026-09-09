// SPDX-License-Identifier: GPL-2.0+
// Copyright 2017 IBM Corp.
// Translated from the C implementation; kernel-provided types and functions
// are intentionally referenced as external dependencies.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const PNV_OCXL_TL_P9_RECV_CAP: u64 = 0x000000000000000f;
const PNV_OCXL_ACTAG_MAX: u16 = 64;
const PNV_OCXL_PASID_BITS: u32 = 15;
const PNV_OCXL_PASID_MAX: i32 = (1 << PNV_OCXL_PASID_BITS) - 1;
const AFU_PRESENT: u32 = 1 << 31;
const AFU_INDEX_MASK: u32 = 0x3f000000;
const AFU_INDEX_SHIFT: u32 = 24;
const ACTAG_MASK: u16 = 0xfff;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct pci_bus { pub number: u8 }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { pub bus: *mut pci_bus, pub devfn: u8, pub dev: device }
#[repr(C)] pub struct pci_controller { pub private_data: *mut c_void, pub dn: *mut c_void }
#[repr(C)] pub struct pnv_phb { pub r#type: c_int, pub opal_id: u64 }

#[repr(C)] struct actag_range { start: u16, count: u16 }
#[repr(C)] struct npu_link {
    list: list_head, domain: c_int, bus: c_int, dev: c_int,
    fn_desired_actags: [u16; 8], fn_actags: [actag_range; 8], assignment_done: bool,
}
static mut LINKS_LIST: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut LINKS_LIST_LOCK: mutex = mutex { _private: [] };

extern "C" {
    fn pci_find_next_ext_capability(dev: *mut pci_dev, pos: c_int, cap: c_int) -> c_int;
    fn pci_read_config_word(dev: *mut pci_dev, pos: c_int, val: *mut u16) -> c_int;
    fn pci_read_config_byte(dev: *mut pci_dev, pos: c_int, val: *mut u8) -> c_int;
    fn pci_find_dvsec_capability(dev: *mut pci_dev, vendor: c_int, id: c_int) -> c_int;
    fn pci_read_config_dword(dev: *mut pci_dev, pos: c_int, val: *mut u32) -> c_int;
    fn pci_domain_nr(bus: *mut pci_bus) -> c_int;
    fn pci_bus_to_host(bus: *mut pci_bus) -> *mut pci_controller;
    fn pci_dev_id(dev: *mut pci_dev) -> u32;
    fn pci_func(devfn: u8) -> usize;
    fn of_property_read_u32(node: *mut c_void, name: *const c_char, val: *mut c_int) -> c_int;
    fn of_property_read_u64_index(node: *mut c_void, name: *const c_char, idx: c_int, val: *mut u64) -> c_int;
    fn opal_npu_tl_set(id: u64, devfn: u8, cap: c_long, phys: u64, size: c_int) -> c_int;
    fn opal_npu_spa_setup(id: u64, bdfn: u32, spa: u64, mask: c_int) -> c_int;
    fn opal_npu_spa_clear_cache(id: u64, bdfn: u32, pe: c_int) -> c_int;
    fn opal_npu_map_lpar(id: u64, bdfn: u32, lpar: u64, lpcr: u64) -> c_int;
    fn ioremap(addr: u64, size: usize) -> *mut u8;
    fn iounmap(addr: *mut c_void);
    fn virt_to_phys(p: *mut c_void) -> u64;
    fn kfree(p: *mut c_void);
    fn memset(dst: *mut c_void, val: c_int, n: usize) -> *mut c_void;
    fn in_be64(p: *mut u8) -> u64;
    fn out_be64(p: *mut u8, v: u64);
}

const OCXL_EXT_CAP_ID_DVSEC: c_int = 0x23;
const PCI_VENDOR_ID_IBM: u16 = 0x1014;
const OCXL_DVSEC_VENDOR_OFFSET: c_int = 4;
const OCXL_DVSEC_ID_OFFSET: c_int = 8;
const OCXL_DVSEC_AFU_CTRL_ID: c_int = 2;
const OCXL_DVSEC_AFU_CTRL_AFU_IDX: c_int = 0x0c;
const OCXL_DVSEC_FUNC_ID: c_int = 1;
const OCXL_DVSEC_FUNC_OFF_INDEX: c_int = 0x0c;
const OCXL_DVSEC_AFU_CTRL_ACTAG_SUP: c_int = 0x10;

unsafe fn find_dvsec_from_pos(dev: *mut pci_dev, dvsec_id: c_int, mut pos: c_int) -> c_int {
    let mut vendor = 0u16; let mut id = 0u16;
    while { pos = pci_find_next_ext_capability(dev, pos, OCXL_EXT_CAP_ID_DVSEC); pos != 0 } {
        pci_read_config_word(dev, pos + OCXL_DVSEC_VENDOR_OFFSET, &mut vendor);
        pci_read_config_word(dev, pos + OCXL_DVSEC_ID_OFFSET, &mut id);
        if vendor == PCI_VENDOR_ID_IBM && id as c_int == dvsec_id { return pos; }
    } 0
}
unsafe fn find_dvsec_afu_ctrl(dev: *mut pci_dev, afu_idx: u8) -> c_int {
    let mut vsec = 0; let mut idx = 0u8;
    while { vsec = find_dvsec_from_pos(dev, OCXL_DVSEC_AFU_CTRL_ID, vsec); vsec != 0 } {
        pci_read_config_byte(dev, vsec + OCXL_DVSEC_AFU_CTRL_AFU_IDX, &mut idx);
        if idx == afu_idx { return vsec; }
    } 0
}
unsafe fn get_max_afu_index(dev: *mut pci_dev, afu_idx: *mut c_int) -> c_int {
    let pos = pci_find_dvsec_capability(dev, PCI_VENDOR_ID_IBM as c_int, OCXL_DVSEC_FUNC_ID); if pos == 0 { return -3; }
    let mut val = 0u32; pci_read_config_dword(dev, pos + OCXL_DVSEC_FUNC_OFF_INDEX, &mut val);
    *afu_idx = if val & AFU_PRESENT != 0 { ((val & AFU_INDEX_MASK) >> AFU_INDEX_SHIFT) as c_int } else { -1 }; 0
}
unsafe fn get_actag_count(dev: *mut pci_dev, afu_idx: c_int, actag: *mut c_int) -> c_int {
    let pos = find_dvsec_afu_ctrl(dev, afu_idx as u8); if pos == 0 { return -3; }
    let mut v = 0u16; pci_read_config_word(dev, pos + OCXL_DVSEC_AFU_CTRL_ACTAG_SUP, &mut v); *actag = (v & ACTAG_MASK) as c_int; 0
}

unsafe fn find_link(_dev: *mut pci_dev) -> *mut npu_link { core::ptr::null_mut() }

unsafe fn assign_fn_actags(desired: u16, total: u16) -> u16 { if total <= PNV_OCXL_ACTAG_MAX { desired } else { PNV_OCXL_ACTAG_MAX.wrapping_mul(desired) / total } }
unsafe fn assign_actags(link: *mut npu_link) {
    let mut total = 0u16; for x in (*link).fn_desired_actags { total = total.wrapping_add(x); }
    let mut start = 0u16;
    for i in 0..8 { let desired = (*link).fn_desired_actags[i]; if desired != 0 { let count = assign_fn_actags(desired, total); (*link).fn_actags[i] = actag_range { start, count }; start = start.wrapping_add(count); } }
    (*link).assignment_done = true;
}

pub unsafe fn pnv_ocxl_get_actag(dev: *mut pci_dev, base: *mut u16, enabled: *mut u16, supported: *mut u16) -> c_int {
    let link = find_link(dev); if link.is_null() { return -19; } if !(*link).assignment_done { assign_actags(link); }
    let i = pci_func((*dev).devfn); *base = (*link).fn_actags[i].start; *enabled = (*link).fn_actags[i].count; *supported = (*link).fn_desired_actags[i]; 0
}
pub unsafe fn pnv_ocxl_get_pasid_count(dev: *mut pci_dev, count: *mut c_int) -> c_int {
    let link = find_link(dev); if link.is_null() { return -19; } let i = pci_func((*dev).devfn);
    if (*link).fn_desired_actags[i] != 0 { *count = PNV_OCXL_PASID_MAX; 0 } else { -22 }
}
unsafe fn set_templ_rate(templ: c_uint, rate: c_uint, buf: *mut u8) { let idx = (3 - templ) / 2; let shift = 4 * (1 - ((3 - templ) % 2)); *buf.add(idx as usize) |= (rate << shift) as u8; }
type c_uint = u32;
pub unsafe fn pnv_ocxl_get_tl_cap(_dev: *mut pci_dev, cap: *mut c_long, buf: *mut c_char, size: c_int) -> c_int { if size != 4 { return -22; } memset(buf as *mut c_void, 0, size as usize); set_templ_rate(2, 1, buf as *mut u8); *cap = PNV_OCXL_TL_P9_RECV_CAP as c_long; 0 }
pub unsafe fn pnv_ocxl_set_tl_conf(dev: *mut pci_dev, cap: c_long, phys: u64, size: c_int) -> c_int { if size != 4 { return -22; } let hose = pci_bus_to_host((*dev).bus); let phb = (*hose).private_data as *mut pnv_phb; if opal_npu_tl_set((*phb).opal_id, (*dev).devfn, cap, phys, size) != 0 { return -22; } 0 }
pub unsafe fn pnv_ocxl_get_xsl_irq(dev: *mut pci_dev, irq: *mut c_int) -> c_int { of_property_read_u32(core::ptr::null_mut(), c"ibm,opal-xsl-irq".as_ptr(), irq) }
pub unsafe fn pnv_ocxl_unmap_xsl_regs(a: *mut c_void, b: *mut c_void, c: *mut c_void, d: *mut c_void) { iounmap(a); iounmap(b); iounmap(c); iounmap(d); }
pub unsafe fn pnv_ocxl_map_xsl_regs(_dev: *mut pci_dev, dsisr: *mut *mut c_void, dar: *mut *mut c_void, tfc: *mut *mut c_void, pe: *mut *mut c_void) -> c_int {
    let mut regs = [core::ptr::null_mut(); 4]; for i in 0..4 { let mut addr = 0u64; if of_property_read_u64_index(core::ptr::null_mut(), c"ibm,opal-xsl-mmio".as_ptr(), i, &mut addr) != 0 { return -22; } regs[i as usize] = ioremap(addr, 8) as *mut c_void; if regs[i as usize].is_null() { return -22; } }
    *dsisr=regs[0]; *dar=regs[1]; *tfc=regs[2]; *pe=regs[3]; 0
}

#[repr(C)] struct spa_data { phb_opal_id: u64, bdfn: u32 }
pub unsafe fn pnv_ocxl_spa_setup(dev: *mut pci_dev, spa: *mut c_void, mask: c_int, out: *mut *mut c_void) -> c_int { let hose = pci_bus_to_host((*dev).bus); let phb = (*hose).private_data as *mut pnv_phb; let data = Box::into_raw(Box::new(spa_data { phb_opal_id: (*phb).opal_id, bdfn: pci_dev_id(dev) })); let rc = opal_npu_spa_setup((*phb).opal_id, (*data).bdfn, virt_to_phys(spa), mask); if rc != 0 { kfree(data as *mut c_void); return rc; } *out = data as *mut c_void; 0 }
pub unsafe fn pnv_ocxl_spa_release(p: *mut c_void) { let d = p as *mut spa_data; let _ = opal_npu_spa_setup((*d).phb_opal_id, (*d).bdfn, 0, 0); kfree(p); }
pub unsafe fn pnv_ocxl_spa_remove_pe_from_cache(p: *mut c_void, pe: c_int) -> c_int { let d = p as *mut spa_data; opal_npu_spa_clear_cache((*d).phb_opal_id, (*d).bdfn, pe) }

pub unsafe fn pnv_ocxl_map_lpar(dev: *mut pci_dev, lparid: u64, lpcr: u64, arva: *mut *mut c_void) -> c_int {
    let hose = pci_bus_to_host((*dev).bus); let phb = (*hose).private_data as *mut pnv_phb; let mut atsd = 0u64;
    let rc = of_property_read_u64_index((*hose).dn, c"ibm,mmio-atsd".as_ptr(), 0, &mut atsd); if rc != 0 { return rc; }
    let rc = opal_npu_map_lpar((*phb).opal_id, pci_dev_id(dev), lparid, lpcr); if rc != 0 { return rc; }
    *arva = ioremap(atsd, 24) as *mut c_void; if (*arva).is_null() { return -12; } 0
}
pub unsafe fn pnv_ocxl_unmap_lpar(arva: *mut c_void) { iounmap(arva); }

const PNV_OCXL_ATSD_TIMEOUT: c_ulong = 100;
const PNV_OCXL_ATSD_AVA: usize = 0;
const PNV_OCXL_ATSD_LNCH: usize = 8;
const PNV_OCXL_ATSD_STAT: usize = 16;
const PNV_OCXL_ATSD_LNCH_R: u64 = 1 << 63;
const PNV_OCXL_ATSD_LNCH_RIC: u64 = 3 << 61;
const PNV_OCXL_ATSD_LNCH_IS: u64 = 3 << 59;
const PNV_OCXL_ATSD_LNCH_OCAPI_SINGLETON: u64 = 1 << 58;
const PNV_OCXL_ATSD_LNCH_PRS: u64 = 1 << 57;
pub unsafe fn pnv_ocxl_tlb_invalidate(arva: *mut c_void, pid: c_ulong, addr: c_ulong, page_size: c_ulong) {
    if arva.is_null() { return; }
    let base = arva as *mut u8; let mut val = 0u64;
    if addr != 0 { val |= (addr >> 12) & ((1 << 13) - 1); out_be64(base.add(PNV_OCXL_ATSD_AVA), val); }
    val = PNV_OCXL_ATSD_LNCH_R | PNV_OCXL_ATSD_LNCH_RIC | PNV_OCXL_ATSD_LNCH_PRS;
    val |= if addr != 0 { 0 } else { PNV_OCXL_ATSD_LNCH_IS | PNV_OCXL_ATSD_LNCH_OCAPI_SINGLETON };
    let size = if page_size == 0x1000 { 0 } else if page_size == 0x200000 { 1 } else if page_size == 0x40000000 { 2 } else { 5 };
    val |= (size as u64) << 55; val |= pid as u64; out_be64(base.add(PNV_OCXL_ATSD_LNCH), val);
    let mut status = in_be64(base.add(PNV_OCXL_ATSD_STAT)); let mut loops = 0u32;
    while status >> 63 != 0 { loops = loops.wrapping_add(1); if loops > PNV_OCXL_ATSD_TIMEOUT as u32 { return; } status = in_be64(base.add(PNV_OCXL_ATSD_STAT)); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
