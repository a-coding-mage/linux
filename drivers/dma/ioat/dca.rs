// SPDX-License-Identifier: GPL-2.0-only
/*
 * Intel I/OAT DMA Linux driver
 * Copyright(c) 2007 - 2009 Intel Corporation.
 */

// Linux, architecture, dma, and register declarations are supplied by the
// surrounding translation unit.

const DCA_TAG_MAP_VALID: u8 = 0x80;
const DCA3_TAG_MAP_BIT_TO_INV: u8 = 0x80;
const DCA3_TAG_MAP_BIT_TO_SEL: u8 = 0x40;
const DCA3_TAG_MAP_LITERAL_VAL: u8 = 0x1;
const DCA_TAG_MAP_MASK: u8 = 0xDF;
const DCA2_TAG_MAP_BYTE0: u8 = 0x80;
const DCA2_TAG_MAP_BYTE1: u8 = 0x0;
const DCA2_TAG_MAP_BYTE2: u8 = 0x81;
const DCA2_TAG_MAP_BYTE3: u8 = 0x82;
const DCA2_TAG_MAP_BYTE4: u8 = 0x82;
const IOAT_TAG_MAP_LEN: usize = 8;

#[inline]
fn apicid_bit(x: u8) -> u8 { DCA_TAG_MAP_VALID | x }

#[repr(C)]
struct IoatDcaSlot {
    pdev: *mut pci_dev,
    rid: u16,
}

const IOAT_DCA_MAX_REQ: i32 = 6;
const IOAT3_DCA_MAX_REQ: i32 = 2;

#[repr(C)]
struct IoatDcaPriv {
    iobase: *mut core::ffi::c_void,
    dca_base: *mut core::ffi::c_void,
    max_requesters: i32,
    requester_count: i32,
    tag_map: [u8; IOAT_TAG_MAP_LEN],
    req_slots: [IoatDcaSlot; 0],
}

#[inline]
unsafe fn dcaid_from_pcidev(pci: *mut pci_dev) -> u16 { pci_dev_id(pci) }

unsafe fn dca_enabled_in_bios(pdev: *mut pci_dev) -> i32 {
    // CPUID level 9 returns DCA configuration.
    // Bit 0 indicates DCA enabled by the BIOS.
    let eax: u32 = cpuid_eax(CPUID_LEAF_DCA);
    let res = (eax & BIT(0)) as i32;
    if res == 0 { dev_dbg(&mut (*pdev).dev, "DCA is disabled in BIOS\n"); }
    res
}

#[no_mangle]
pub unsafe extern "C" fn system_has_dca_enabled(pdev: *mut pci_dev) -> i32 {
    if boot_cpu_has(X86_FEATURE_DCA) { return dca_enabled_in_bios(pdev); }
    dev_dbg(&mut (*pdev).dev, "boot cpu doesn't have X86_FEATURE_DCA\n");
    0
}

unsafe fn ioat_dca_dev_managed(dca: *mut dca_provider, dev: *mut device) -> i32 {
    let ioatdca = dca_priv(dca) as *mut IoatDcaPriv;
    let pdev = to_pci_dev(dev);
    for i in 0..(*ioatdca).max_requesters {
        if (*ioatdca).req_slots.as_ptr().add(i as usize).read().pdev == pdev { return 1; }
    }
    0
}

unsafe fn ioat_dca_add_requester(dca: *mut dca_provider, dev: *mut device) -> i32 {
    let ioatdca = dca_priv(dca) as *mut IoatDcaPriv;
    if !dev_is_pci(dev) { return -ENODEV; }
    let pdev = to_pci_dev(dev);
    let id = dcaid_from_pcidev(pdev);
    if (*ioatdca).requester_count == (*ioatdca).max_requesters { return -ENODEV; }
    for i in 0..(*ioatdca).max_requesters {
        let slot = (*ioatdca).req_slots.as_mut_ptr().add(i as usize);
        if (*slot).pdev.is_null() {
            (*ioatdca).requester_count += 1;
            (*slot).pdev = pdev;
            (*slot).rid = id;
            let table = readw((*ioatdca).dca_base.add(IOAT3_DCA_GREQID_OFFSET as usize));
            writel((id as u32) | IOAT_DCA_GREQID_VALID, (*ioatdca).iobase.add(table as usize + i as usize * 4));
            return i;
        }
    }
    -EFAULT
}

unsafe fn ioat_dca_remove_requester(dca: *mut dca_provider, dev: *mut device) -> i32 {
    let ioatdca = dca_priv(dca) as *mut IoatDcaPriv;
    if !dev_is_pci(dev) { return -ENODEV; }
    let pdev = to_pci_dev(dev);
    for i in 0..(*ioatdca).max_requesters {
        let slot = (*ioatdca).req_slots.as_mut_ptr().add(i as usize);
        if (*slot).pdev == pdev {
            let table = readw((*ioatdca).dca_base.add(IOAT3_DCA_GREQID_OFFSET as usize));
            writel(0, (*ioatdca).iobase.add(table as usize + i as usize * 4));
            (*slot).pdev = core::ptr::null_mut(); (*slot).rid = 0;
            (*ioatdca).requester_count -= 1;
            return i;
        }
    }
    -ENODEV
}

unsafe fn ioat_dca_get_tag(dca: *mut dca_provider, _dev: *mut device, cpu: i32) -> u8 {
    let ioatdca = dca_priv(dca) as *mut IoatDcaPriv;
    let apic_id = cpu_physical_id(cpu);
    let mut tag: u8 = 0;
    for i in 0..IOAT_TAG_MAP_LEN {
        let entry = (*ioatdca).tag_map[i];
        let value: u8;
        if entry & DCA3_TAG_MAP_BIT_TO_SEL != 0 {
            let bit = entry & !(DCA3_TAG_MAP_BIT_TO_SEL | DCA3_TAG_MAP_BIT_TO_INV);
            value = if apic_id & (1 << bit) != 0 { 1 } else { 0 };
        } else if entry & DCA3_TAG_MAP_BIT_TO_INV != 0 {
            let bit = entry & !DCA3_TAG_MAP_BIT_TO_INV;
            value = if apic_id & (1 << bit) != 0 { 0 } else { 1 };
        } else { value = if entry & DCA3_TAG_MAP_LITERAL_VAL != 0 { 1 } else { 0 }; }
        tag |= value << i;
    }
    tag
}

#[repr(C)]
struct DcaOps {
    add_requester: unsafe fn(*mut dca_provider, *mut device) -> i32,
    remove_requester: unsafe fn(*mut dca_provider, *mut device) -> i32,
    get_tag: unsafe fn(*mut dca_provider, *mut device, i32) -> u8,
    dev_managed: unsafe fn(*mut dca_provider, *mut device) -> i32,
}

static IOAT_DCA_OPS: DcaOps = DcaOps { add_requester: ioat_dca_add_requester, remove_requester: ioat_dca_remove_requester, get_tag: ioat_dca_get_tag, dev_managed: ioat_dca_dev_managed };

unsafe fn ioat_dca_count_dca_slots(iobase: *mut core::ffi::c_void, dca_offset: u16) -> i32 {
    let table = readw(iobase.add(dca_offset as usize + IOAT3_DCA_GREQID_OFFSET as usize));
    if table == 0 { return 0; }
    let mut slots = 0;
    loop {
        let req = readl(iobase.add(table as usize + slots as usize * core::mem::size_of::<u32>()));
        slots += 1;
        if req & IOAT_DCA_GREQID_LASTID != 0 { break; }
    }
    slots
}

#[inline]
unsafe fn dca3_tag_map_invalid(tag_map: *mut u8) -> bool {
    (*tag_map.add(0) == DCA_TAG_MAP_VALID) && (*tag_map.add(1) == DCA_TAG_MAP_VALID) &&
    (*tag_map.add(2) == DCA_TAG_MAP_VALID) && (*tag_map.add(3) == DCA_TAG_MAP_VALID) &&
    (*tag_map.add(4) == DCA_TAG_MAP_VALID)
}

pub unsafe extern "C" fn ioat_dca_init(pdev: *mut pci_dev, iobase: *mut core::ffi::c_void) -> *mut dca_provider {
    if system_has_dca_enabled(pdev) == 0 { return core::ptr::null_mut(); }
    let dca_offset = readw(iobase.add(IOAT_DCAOFFSET_OFFSET as usize));
    if dca_offset == 0 { return core::ptr::null_mut(); }
    let slots = ioat_dca_count_dca_slots(iobase, dca_offset);
    if slots == 0 { return core::ptr::null_mut(); }
    let dca = alloc_dca_provider(&IOAT_DCA_OPS as *const DcaOps, struct_size::<IoatDcaPriv>(slots as usize));
    if dca.is_null() { return core::ptr::null_mut(); }
    let ioatdca = dca_priv(dca) as *mut IoatDcaPriv;
    (*ioatdca).iobase = iobase; (*ioatdca).dca_base = iobase.add(dca_offset as usize); (*ioatdca).max_requesters = slots;
    let mut csi = readw((*ioatdca).dca_base.add(IOAT3_CSI_CONTROL_OFFSET as usize));
    if csi & IOAT3_CSI_CONTROL_PREFETCH == 0 { csi |= IOAT3_CSI_CONTROL_PREFETCH; writew(csi, (*ioatdca).dca_base.add(IOAT3_CSI_CONTROL_OFFSET as usize)); }
    let mut pcie = readw((*ioatdca).dca_base.add(IOAT3_PCI_CONTROL_OFFSET as usize));
    if pcie & IOAT3_PCI_CONTROL_MEMWR == 0 { pcie |= IOAT3_PCI_CONTROL_MEMWR; writew(pcie, (*ioatdca).dca_base.add(IOAT3_PCI_CONTROL_OFFSET as usize)); }
    let low = readl((*ioatdca).dca_base.add(IOAT3_APICID_TAG_MAP_OFFSET_LOW as usize)) as u64;
    let high = readl((*ioatdca).dca_base.add(IOAT3_APICID_TAG_MAP_OFFSET_HIGH as usize)) as u64;
    let full = low | (high << 32);
    for i in 0..8 { (*ioatdca).tag_map[i] = ((full >> (8 * i)) as u8) & DCA_TAG_MAP_MASK; }
    if dca3_tag_map_invalid((*ioatdca).tag_map.as_mut_ptr()) {
        add_taint(TAINT_FIRMWARE_WORKAROUND, LOCKDEP_STILL_OK);
        pr_warn_once("%s %s: APICID_TAG_MAP set incorrectly by BIOS, disabling DCA\n", dev_driver_string(&(*pdev).dev), dev_name(&(*pdev).dev));
        free_dca_provider(dca); return core::ptr::null_mut();
    }
    let err = register_dca_provider(dca, &mut (*pdev).dev);
    if err != 0 { free_dca_provider(dca); return core::ptr::null_mut(); }
    dca
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
