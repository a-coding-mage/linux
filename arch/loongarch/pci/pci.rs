// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */
// Linux kernel includes and architecture dependencies are supplied externally.

pub const PCI_DEVICE_ID_LOONGSON_HOST: u16 = 0x7a00;
pub const PCI_DEVICE_ID_LOONGSON_DC1: u16 = 0x7a06;
pub const PCI_DEVICE_ID_LOONGSON_DC2: u16 = 0x7a36;
pub const PCI_DEVICE_ID_LOONGSON_DC3: u16 = 0x7a46;
pub const PCI_DEVICE_ID_LOONGSON_GPU1: u16 = 0x7a15;
pub const PCI_DEVICE_ID_LOONGSON_GPU2: u16 = 0x7a25;
pub const PCI_DEVICE_ID_LOONGSON_GPU3: u16 = 0x7a35;

pub unsafe fn raw_pci_read(
    domain: u32,
    bus: u32,
    devfn: u32,
    reg: i32,
    len: i32,
    val: *mut u32,
) -> i32 {
    let bus_tmp = pci_find_bus(domain, bus);

    if !bus_tmp.is_null() {
        return ((*(*bus_tmp).ops).read)(bus_tmp, devfn, reg, len, val);
    }
    -EINVAL
}

pub unsafe fn raw_pci_write(
    domain: u32,
    bus: u32,
    devfn: u32,
    reg: i32,
    len: i32,
    val: u32,
) -> i32 {
    let bus_tmp = pci_find_bus(domain, bus);

    if !bus_tmp.is_null() {
        return ((*(*bus_tmp).ops).write)(bus_tmp, devfn, reg, len, val);
    }
    -EINVAL
}

pub fn mcfg_addr_init(node: i32) -> phys_addr_t {
    ((node as u64) << 44) | MCFG_EXT_PCICFG_BASE
}

pub unsafe extern "C" fn pcibios_init() -> i32 {
    let lsize: u32;

    /*
     * Set PCI cacheline size to that of the last level in the
     * cache hierarchy.
     */
    lsize = cpu_last_level_cache_line_size();

    if lsize != 0 {
        pci_dfl_cache_line_size = lsize >> 2;

        pr_debug!("PCI: pci_cache_line_size set to %d bytes\n", lsize);
    }

    0
}

// subsys_initcall(pcibios_init);

pub unsafe fn pcibios_device_add(dev: *mut pci_dev) -> i32 {
    let id: i32;
    let dom: *mut irq_domain;

    id = pci_domain_nr((*dev).bus);
    dom = irq_find_matching_fwnode(get_pch_msi_handle(id), DOMAIN_BUS_PCI_MSI);
    dev_set_msi_domain(&mut (*dev).dev, dom);

    0
}

pub unsafe fn pcibios_alloc_irq(dev: *mut pci_dev) -> i32 {
    if acpi_disabled {
        return 0;
    }
    if pci_dev_msi_enabled(dev) {
        return 0;
    }
    acpi_pci_irq_enable(dev)
}

unsafe fn pci_fixup_vgadev(pdev: *mut pci_dev) {
    let mut devp: *mut pci_dev = core::ptr::null_mut();

    loop {
        devp = pci_get_class(PCI_CLASS_DISPLAY_VGA << 8, devp);
        if devp.is_null() {
            break;
        }
        if (*devp).vendor != PCI_VENDOR_ID_LOONGSON {
            vga_set_default_device(devp);
            dev_info!(&(*pdev).dev, "Overriding boot device as %X:%X\n", (*devp).vendor, (*devp).device);
        }
    }
}

// DECLARE_PCI_FIXUP_FINAL(PCI_VENDOR_ID_LOONGSON, PCI_DEVICE_ID_LOONGSON_DC1, pci_fixup_vgadev);
// DECLARE_PCI_FIXUP_FINAL(PCI_VENDOR_ID_LOONGSON, PCI_DEVICE_ID_LOONGSON_DC2, pci_fixup_vgadev);
// DECLARE_PCI_FIXUP_FINAL(PCI_VENDOR_ID_LOONGSON, PCI_DEVICE_ID_LOONGSON_DC3, pci_fixup_vgadev);

pub const CRTC_NUM_MAX: usize = 2;
pub const CRTC_OUTPUT_ENABLE: u32 = 0x100;
static mut crtc_status: [u32; CRTC_NUM_MAX] = [0; CRTC_NUM_MAX];

unsafe fn loongson_gpu_fixup_dma_hang(pdev: *mut pci_dev, on: bool) {
    let base = ((*(*(*pdev).bus).ops).map_bus)((*pdev).bus, (*pdev).devfn + 1, 0);
    let device = readw(base.add(PCI_DEVICE_ID as usize));
    let regbase = ioremap(readq(base.add(PCI_BASE_ADDRESS_0 as usize)) & !0xffu64, SZ_64K);

    if regbase.is_null() {
        pci_err!(pdev, "Failed to ioremap()\n");
        return;
    }

    let (mut crtc_reg, crtc_offset) = match device {
        PCI_DEVICE_ID_LOONGSON_DC2 => (regbase.add(0x1240), 0x10usize),
        PCI_DEVICE_ID_LOONGSON_DC3 => (regbase, 0x400usize),
        _ => {
            iounmap(regbase);
            return;
        }
    };

    for i in 0..CRTC_NUM_MAX {
        let mut val = readl(crtc_reg);

        if !on {
            crtc_status[i] = val;
        }

        /* No need to fixup if the status is off at startup. */
        if crtc_status[i] & CRTC_OUTPUT_ENABLE == 0 {
            crtc_reg = crtc_reg.add(crtc_offset);
            continue;
        }

        if on {
            val |= CRTC_OUTPUT_ENABLE;
        } else {
            val &= !CRTC_OUTPUT_ENABLE;
        }

        mb();
        writel(val, crtc_reg);

        let mut count = 0;
        while count < 40 {
            val = readl(crtc_reg) & CRTC_OUTPUT_ENABLE;
            if (on && val != 0) || (!on && val == 0) {
                break;
            }
            udelay(1000);
            count += 1;
        }

        pci_info!(pdev, "DMA hang fixup at reg[0x%lx]: 0x%x\n", crtc_reg as usize & 0xffff, readl(crtc_reg));
        crtc_reg = crtc_reg.add(crtc_offset);
    }

    iounmap(regbase);
}

unsafe fn pci_fixup_dma_hang_early(pdev: *mut pci_dev) {
    loongson_gpu_fixup_dma_hang(pdev, false);
}

// DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_LOONGSON, PCI_DEVICE_ID_LOONGSON_GPU2, pci_fixup_dma_hang_early);
// DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_LOONGSON, PCI_DEVICE_ID_LOONGSON_GPU3, pci_fixup_dma_hang_early);

unsafe fn pci_fixup_dma_hang_final(pdev: *mut pci_dev) {
    loongson_gpu_fixup_dma_hang(pdev, true);
}

// DECLARE_PCI_FIXUP_FINAL(PCI_VENDOR_ID_LOONGSON, PCI_DEVICE_ID_LOONGSON_GPU2, pci_fixup_dma_hang_final);
// DECLARE_PCI_FIXUP_FINAL(PCI_VENDOR_ID_LOONGSON, PCI_DEVICE_ID_LOONGSON_GPU3, pci_fixup_dma_hang_final);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
