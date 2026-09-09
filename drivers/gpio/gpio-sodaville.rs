// SPDX-License-Identifier: GPL-2.0
/*
 *  GPIO interface for Intel Sodaville SoCs.
 *
 *  Copyright (c) 2010, 2011 Intel Corporation
 *
 *  Author: Hans J. Koch <hjk@linutronix.de>
 */

// Linux kernel dependencies supplied by the surrounding tree.

const DRV_NAME: &str = "sdv_gpio";
const SDV_NUM_PUB_GPIOS: u32 = 12;
const PCI_DEVICE_ID_SDV_GPIO: u16 = 0x2e67;
const GPIO_BAR: usize = 0;

const GPOUTR: usize = 0x00;
const GPOER: usize = 0x04;
const GPINR: usize = 0x08;

const GPSTR: usize = 0x0c;
const GPIT1R0: usize = 0x10;
const GPIO_INT: usize = 0x14;
const GPIT1R1: usize = 0x18;

const GPMUXCTL: usize = 0x1c;

#[repr(C)]
struct SdvGpioChipData {
    irq_base: i32,
    gpio_pub_base: *mut core::ffi::c_void,
    id: *mut IrqDomain,
    gc: *mut IrqChipGeneric,
    gen_gc: GpioGenericChip,
}

unsafe fn sdv_gpio_pub_set_type(d: *mut IrqData, irq_type: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d);
    let sd = (*gc).private as *mut SdvGpioChipData;
    let type_reg: *mut u32;
    let mut reg: u32;

    if (*d).hwirq < 8 {
        type_reg = ((*sd).gpio_pub_base as *mut u8).add(GPIT1R0) as *mut u32;
    } else {
        type_reg = ((*sd).gpio_pub_base as *mut u8).add(GPIT1R1) as *mut u32;
    }

    reg = readl(type_reg);

    match irq_type {
        IRQ_TYPE_LEVEL_HIGH => {
            reg &= !(1u32 << (4 * ((*d).hwirq % 8)));
        }
        IRQ_TYPE_LEVEL_LOW => {
            reg |= 1u32 << (4 * ((*d).hwirq % 8));
        }
        _ => return -EINVAL,
    }

    writel(reg, type_reg);
    0
}

unsafe fn sdv_gpio_pub_irq_handler(irq: i32, data: *mut core::ffi::c_void) -> IrqReturn {
    let sd = data as *mut SdvGpioChipData;
    let mut irq_stat: usize = readl(((*sd).gpio_pub_base as *mut u8).add(GPSTR) as *mut u32) as usize;
    let irq_bit: i32;

    irq_stat &= readl(((*sd).gpio_pub_base as *mut u8).add(GPIO_INT) as *mut u32) as usize;
    if irq_stat == 0 {
        return IRQ_NONE;
    }

    for_each_set_bit!(irq_bit, &irq_stat, 32);
    generic_handle_domain_irq((*sd).id, irq_bit as u32);

    IRQ_HANDLED
}

unsafe fn sdv_xlate(
    h: *mut IrqDomain,
    node: *mut DeviceNode,
    intspec: *const u32,
    intsize: u32,
    out_hwirq: *mut IrqHwNumber,
    out_type: *mut u32,
) -> i32 {
    let line: u32;
    let irq_type: u32;

    if node != irq_domain_get_of_node(h) {
        return -EINVAL;
    }
    if intsize < 2 {
        return -EINVAL;
    }

    line = *intspec;
    *out_hwirq = line as IrqHwNumber;

    irq_type = *intspec.add(1);
    match irq_type {
        IRQ_TYPE_LEVEL_LOW | IRQ_TYPE_LEVEL_HIGH => *out_type = irq_type,
        _ => return -EINVAL,
    }
    0
}

#[repr(C)]
struct IrqDomainOps {
    xlate: unsafe fn(*mut IrqDomain, *mut DeviceNode, *const u32, u32, *mut IrqHwNumber, *mut u32) -> i32,
}

static IRQ_DOMAIN_SDV_OPS: IrqDomainOps = IrqDomainOps { xlate: sdv_xlate };

unsafe fn sdv_register_irqsupport(sd: *mut SdvGpioChipData, pdev: *mut PciDev) -> i32 {
    let mut ct: *mut IrqChipType;
    let ret: i32;

    (*sd).irq_base = devm_irq_alloc_descs(&mut (*pdev).dev, -1, 0, SDV_NUM_PUB_GPIOS, -1);
    if (*sd).irq_base < 0 { return (*sd).irq_base; }

    writel(0, ((*sd).gpio_pub_base as *mut u8).add(GPIO_INT) as *mut u32);
    writel((1u32 << 11) - 1, ((*sd).gpio_pub_base as *mut u8).add(GPSTR) as *mut u32);

    ret = devm_request_irq(&mut (*pdev).dev, (*pdev).irq, sdv_gpio_pub_irq_handler, IRQF_SHARED, "sdv_gpio", sd as *mut _);
    if ret != 0 { return ret; }

    (*sd).gc = devm_irq_alloc_generic_chip(&mut (*pdev).dev, "sdv-gpio", 1, (*sd).irq_base, (*sd).gpio_pub_base, handle_fasteoi_irq);
    if (*sd).gc.is_null() { return -ENOMEM; }

    (*sd).gc.private = sd as *mut _;
    ct = (*sd).gc.chip_types;
    (*ct).type_ = IRQ_TYPE_LEVEL_MASK;
    (*ct).regs.eoi = GPSTR;
    (*ct).regs.mask = GPIO_INT;
    (*ct).chip.irq_mask = irq_gc_mask_clr_bit;
    (*ct).chip.irq_unmask = irq_gc_mask_set_bit;
    (*ct).chip.irq_eoi = irq_gc_eoi;
    (*ct).chip.irq_set_type = sdv_gpio_pub_set_type;

    irq_setup_generic_chip((*sd).gc, irq_msk(SDV_NUM_PUB_GPIOS), IRQ_GC_INIT_MASK_CACHE, IRQ_NOREQUEST, IRQ_LEVEL | IRQ_NOPROBE);

    (*sd).id = irq_domain_create_legacy(dev_fwnode(&mut (*pdev).dev), SDV_NUM_PUB_GPIOS, (*sd).irq_base as u32, 0, &IRQ_DOMAIN_SDV_OPS, sd as *mut _);
    if (*sd).id.is_null() { return -ENODEV; }
    0
}

unsafe fn sdv_gpio_probe(pdev: *mut PciDev, _pci_id: *const PciDeviceId) -> i32 {
    let mut config: GpioGenericChipConfig;
    let sd: *mut SdvGpioChipData;
    let ret: i32;
    let mut mux_val: u32 = 0;

    sd = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<SdvGpioChipData>(), GFP_KERNEL) as *mut _;
    if sd.is_null() { return -ENOMEM; }

    ret = pcim_enable_device(pdev);
    if ret != 0 { dev_err!(&mut (*pdev).dev, "can't enable device.\n"); return ret; }
    ret = pcim_iomap_regions(pdev, 1 << GPIO_BAR, DRV_NAME);
    if ret != 0 { dev_err!(&mut (*pdev).dev, "can't alloc PCI BAR #%d\n", GPIO_BAR); return ret; }

    (*sd).gpio_pub_base = pcim_iomap_table(pdev)[GPIO_BAR];
    ret = of_property_read_u32((*pdev).dev.of_node, "intel,muxctl", &mut mux_val);
    if ret == 0 { writel(mux_val, ((*sd).gpio_pub_base as *mut u8).add(GPMUXCTL) as *mut u32); }

    config = GpioGenericChipConfig { dev: &mut (*pdev).dev, sz: 4, dat: ((*sd).gpio_pub_base as *mut u8).add(GPINR) as *mut _, set: ((*sd).gpio_pub_base as *mut u8).add(GPOUTR) as *mut _, dirout: ((*sd).gpio_pub_base as *mut u8).add(GPOER) as *mut _ };
    ret = gpio_generic_chip_init(&mut (*sd).gen_gc, &mut config);
    if ret != 0 { return ret; }
    (*sd).gen_gc.gc.ngpio = SDV_NUM_PUB_GPIOS;
    ret = devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*sd).gen_gc.gc, sd as *mut _);
    if ret < 0 { dev_err!(&mut (*pdev).dev, "gpiochip_add() failed.\n"); return ret; }
    ret = sdv_register_irqsupport(sd, pdev);
    if ret != 0 { return ret; }
    pci_set_drvdata(pdev, sd as *mut _);
    dev_info!(&mut (*pdev).dev, "Sodaville GPIO driver registered.\n");
    0
}

static SDV_GPIO_PCI_IDS: [PciDeviceId; 2] = [
    pci_device!(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_SDV_GPIO),
    PciDeviceId { vendor: 0, device: 0 },
];

static mut SDV_GPIO_DRIVER: PciDriver = PciDriver {
    driver: Driver { suppress_bind_attrs: true },
    name: DRV_NAME,
    id_table: SDV_GPIO_PCI_IDS.as_ptr(),
    probe: sdv_gpio_probe,
};

builtin_pci_driver!(SDV_GPIO_DRIVER);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
