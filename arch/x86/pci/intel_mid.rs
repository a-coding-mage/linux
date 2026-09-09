// SPDX-License-Identifier: GPL-2.0
/*
 * Intel MID PCI support
 *   Copyright (c) 2008 Intel Corporation
 *     Jesse Barnes <jesse.barnes@intel.com>
 *
 * Moorestown has an interesting PCI implementation:
 *   - configuration space is memory mapped (as defined by MCFG)
 *   - Lincroft devices also have a real, type 1 configuration space
 *   - Early Lincroft silicon has a type 1 access bug that will cause
 *     a hang if non-existent devices are accessed
 *   - some devices have the "fixed BAR" capability, which means
 *     they can't be relocated or modified; check for that during
 *     BAR sizing
 *
 * So, we use the MCFG space for all reads and writes, but also send
 * Lincroft writes to type 1 space.  But only read/write if the device
 * actually exists, otherwise return all 1s for reads and bit bucket
 * the writes.
 */

// Kernel dependencies are supplied by other translated units.

const PCIE_CAP_OFFSET: i32 = 0x100;
const PCI_DEVICE_ID_INTEL_MRFLD_MMC: u16 = 0x1190;
const PCI_DEVICE_ID_INTEL_MRFLD_HSU: u16 = 0x1191;
const PCIE_VNDR_CAP_ID_FIXED_BAR: u32 = 0x00;
const PCI_FIXED_BAR_0_SIZE: u32 = 0x04;
const PCI_FIXED_BAR_1_SIZE: u32 = 0x08;
const PCI_FIXED_BAR_2_SIZE: u32 = 0x0c;
const PCI_FIXED_BAR_3_SIZE: u32 = 0x10;
const PCI_FIXED_BAR_4_SIZE: u32 = 0x14;
const PCI_FIXED_BAR_5_SIZE: u32 = 0x1c;

static mut pci_soc_mode: i32 = 0;

unsafe fn fixed_bar_cap(bus: *mut pci_bus, devfn: u32) -> i32 {
    let mut pos: i32 = PCIE_CAP_OFFSET;
    let mut pcie_cap: u32 = 0;
    let mut cap_data: u32;

    if raw_pci_ext_ops.is_null() { return 0; }
    while pos != 0 {
        if ((*raw_pci_ext_ops).read)(pci_domain_nr(bus), (*bus).number, devfn, pos, 4, &mut pcie_cap) != 0 { return 0; }
        if PCI_EXT_CAP_ID(pcie_cap) == 0x0000 || PCI_EXT_CAP_ID(pcie_cap) == 0xffff { break; }
        if PCI_EXT_CAP_ID(pcie_cap) == PCI_EXT_CAP_ID_VNDR {
            (*raw_pci_ext_ops).read(pci_domain_nr(bus), (*bus).number, devfn, pos + 4, 4, &mut cap_data);
            if (cap_data & 0xffff) == PCIE_VNDR_CAP_ID_FIXED_BAR { return pos; }
        }
        pos = PCI_EXT_CAP_NEXT(pcie_cap);
    }
    0
}

unsafe fn pci_device_update_fixed(bus: *mut pci_bus, devfn: u32, reg: i32, len: i32, val: u32, offset: i32) -> i32 {
    let mut size: u32 = 0;
    let domain = pci_domain_nr(bus);
    let busnum = (*bus).number;
    let bar = (reg - PCI_BASE_ADDRESS_0) >> 2;
    if val == !0u32 && len == 4 {
        ((*raw_pci_ext_ops).read)(domain, busnum, devfn, offset + 8 + bar * 4, 4, &mut size);
        let decode: u32 = if size != 0 {
            let mut d = size - 1;
            d |= d >> 1; d |= d >> 2; d |= d >> 4; d |= d >> 8; d |= d >> 16;
            d += 1; !(d - 1)
        } else { 0 };
        return ((*raw_pci_ext_ops).write)(domain, busnum, devfn, reg, 4, decode);
    }
    ((*raw_pci_ext_ops).write)(domain, busnum, devfn, reg, len, val)
}

unsafe fn type1_access_ok(bus: u32, devfn: u32, reg: i32) -> bool {
    if reg >= 0x100 || reg == PCI_STATUS || reg == PCI_HEADER_TYPE { return false; }
    if bus == 0 && (devfn == PCI_DEVFN(2, 0) || devfn == PCI_DEVFN(0, 0) || devfn == PCI_DEVFN(3, 0)) { return true; }
    false // Langwell on others
}

unsafe fn pci_read(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, value: *mut u32) -> i32 {
    if type1_access_ok((*bus).number, devfn, where_) {
        return (pci_direct_conf1.read)(pci_domain_nr(bus), (*bus).number, devfn, where_, size, value);
    }
    ((*raw_pci_ext_ops).read)(pci_domain_nr(bus), (*bus).number, devfn, where_, size, value)
}

unsafe fn pci_write(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, value: u32) -> i32 {
    if where_ == PCI_ROM_ADDRESS { return 0; }
    let offset = fixed_bar_cap(bus, devfn);
    if offset != 0 && where_ >= PCI_BASE_ADDRESS_0 && where_ <= PCI_BASE_ADDRESS_5 {
        return pci_device_update_fixed(bus, devfn, where_, size, value, offset);
    }
    if type1_access_ok((*bus).number, devfn, where_) {
        return (pci_direct_conf1.write)(pci_domain_nr(bus), (*bus).number, devfn, where_, size, value);
    }
    ((*raw_pci_ext_ops).write)(pci_domain_nr(bus), (*bus).number, devfn, where_, size, value)
}

static intel_mid_cpu_ids: [x86_cpu_id; 2] = [X86_MATCH_VFM(INTEL_ATOM_SILVERMONT_MID, core::ptr::null()), X86_CPU_ID_EMPTY];

unsafe fn intel_mid_pci_irq_enable(dev: *mut pci_dev) -> i32 {
    let mut info: irq_alloc_info = core::mem::zeroed();
    let mut polarity_low: bool;
    let mut model: u16 = 0;
    let mut gsi: u8 = 0;
    if (*dev).irq_managed && (*dev).irq > 0 { return 0; }
    let ret = pci_read_config_byte(dev, PCI_INTERRUPT_LINE, &mut gsi);
    if ret != 0 { dev_warn!(&(*dev).dev, "Failed to read interrupt line: %d\n", ret); return pcibios_err_to_errno(ret); }
    let id = x86_match_cpu(intel_mid_cpu_ids.as_ptr());
    if !id.is_null() { model = (*id).model; }
    match model {
        VFM_MODEL(INTEL_ATOM_SILVERMONT_MID) => {
            polarity_low = false;
            if gsi == 0 {
                if (*dev).device == PCI_DEVICE_ID_INTEL_MRFLD_HSU { return -EBUSY; }
                if (*dev).device != PCI_DEVICE_ID_INTEL_MRFLD_MMC { return 0; }
            }
        }
        _ => polarity_low = true,
    }
    ioapic_set_alloc_attr(&mut info, dev_to_node(&(*dev).dev), 1, polarity_low);
    let ret = mp_map_gsi_to_irq(gsi, IOAPIC_MAP_ALLOC, &mut info);
    if ret < 0 { return ret; }
    (*dev).irq = ret; (*dev).irq_managed = 1; 0
}

unsafe fn intel_mid_pci_irq_disable(dev: *mut pci_dev) {
    if !mp_should_keep_irq(&(*dev).dev) && (*dev).irq_managed && (*dev).irq > 0 {
        mp_unmap_irq((*dev).irq); (*dev).irq_managed = 0;
    }
}

static intel_mid_pci_ops: pci_ops = pci_ops { read: pci_read, write: pci_write };

unsafe fn intel_mid_pci_init() -> i32 {
    pr_info!("Intel MID platform detected, using MID PCI ops\n");
    pci_mmcfg_late_init();
    pcibios_enable_irq = Some(intel_mid_pci_irq_enable);
    pcibios_disable_irq = Some(intel_mid_pci_irq_disable);
    pci_root_ops = intel_mid_pci_ops;
    pci_soc_mode = 1;
    acpi_noirq_set();
    1
}

unsafe fn pci_d3delay_fixup(dev: *mut pci_dev) {
    if pci_soc_mode == 0 { return; }
    if type1_access_ok((*(*dev).bus).number, (*dev).devfn, PCI_DEVICE_ID) { return; }
    (*dev).d3hot_delay = 0;
}

unsafe fn mid_power_off_one_device(dev: *mut pci_dev) {
    let mut pmcsr: u16 = 0;
    pci_read_config_word(dev, (*dev).pm_cap + PCI_PM_CTRL, &mut pmcsr);
    (*dev).current_state = (pmcsr & PCI_PM_CTRL_STATE_MASK) as pci_power_t;
    pci_set_power_state(dev, PCI_D3hot);
}

unsafe fn mid_power_off_devices(dev: *mut pci_dev) {
    if pci_soc_mode == 0 { return; }
    if intel_mid_pwr_get_lss_id(dev) < 0 { return; }
    mid_power_off_one_device(dev);
}

unsafe fn pci_fixed_bar_fixup(dev: *mut pci_dev) {
    if pci_soc_mode == 0 || (*dev).cfg_size < PCIE_CAP_OFFSET + 4 { return; }
    let offset = fixed_bar_cap((*dev).bus, (*dev).devfn);
    if offset == 0 || PCI_DEVFN(2, 0) == (*dev).devfn || PCI_DEVFN(2, 2) == (*dev).devfn { return; }
    for i in 0..PCI_STD_NUM_BARS {
        let mut size: u32 = 0;
        pci_read_config_dword(dev, offset + 8 + i * 4, &mut size);
        (*dev).resource[i].end = (*dev).resource[i].start + size as u64 - 1;
        (*dev).resource[i].flags |= IORESOURCE_PCI_FIXED;
    }
}

// DECLARE_PCI_FIXUP_FINAL/HEADER registrations are retained by the kernel integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
