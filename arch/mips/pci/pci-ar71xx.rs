// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Atheros AR71xx PCI host controller driver
 *
 *  Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
 *  Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 *
 *  Parts of this file are based on Atheros' 2.6.15 BSP
 */

// Linux and platform dependencies supplied by the surrounding kernel translation.

const AR71XX_PCI_REG_CRP_AD_CBE: usize = 0x00;
const AR71XX_PCI_REG_CRP_WRDATA: usize = 0x04;
const AR71XX_PCI_REG_CRP_RDDATA: usize = 0x08;
const AR71XX_PCI_REG_CFG_AD: usize = 0x0c;
const AR71XX_PCI_REG_CFG_CBE: usize = 0x10;
const AR71XX_PCI_REG_CFG_WRDATA: usize = 0x14;
const AR71XX_PCI_REG_CFG_RDDATA: usize = 0x18;
const AR71XX_PCI_REG_PCI_ERR: usize = 0x1c;
const AR71XX_PCI_REG_PCI_ERR_ADDR: usize = 0x20;
const AR71XX_PCI_REG_AHB_ERR: usize = 0x24;
const AR71XX_PCI_REG_AHB_ERR_ADDR: usize = 0x28;

const AR71XX_PCI_CRP_CMD_WRITE: u32 = 0x0001_0000;
const AR71XX_PCI_CRP_CMD_READ: u32 = 0x0000_0000;
const AR71XX_PCI_CFG_CMD_READ: u32 = 0x0000_000a;
const AR71XX_PCI_CFG_CMD_WRITE: u32 = 0x0000_000b;

const AR71XX_PCI_INT_CORE: u32 = 1 << 4;
const AR71XX_PCI_INT_DEV2: u32 = 1 << 2;
const AR71XX_PCI_INT_DEV1: u32 = 1 << 1;
const AR71XX_PCI_INT_DEV0: u32 = 1 << 0;
const AR71XX_PCI_IRQ_COUNT: usize = 5;

#[repr(C)]
struct Ar71xxPciController {
    cfg_base: *mut core::ffi::c_void,
    irq: i32,
    irq_base: i32,
    pci_ctrl: pci_controller,
    io_res: resource,
    mem_res: resource,
}

/* Byte lane enable bits */
static AR71XX_PCI_BLE_TABLE: [[u8; 4]; 4] = [
    [0x0, 0xf, 0xf, 0xf],
    [0xe, 0xd, 0xb, 0x7],
    [0xc, 0xf, 0x3, 0xf],
    [0xf, 0xf, 0xf, 0xf],
];

static AR71XX_PCI_READ_MASK: [u32; 8] = [0, 0xff, 0xffff, 0, 0xffff_ffff, 0, 0, 0];

#[inline]
unsafe fn ar71xx_pci_get_ble(where_: i32, size: i32, local: i32) -> u32 {
    let mut t = AR71XX_PCI_BLE_TABLE[(size & 3) as usize][(where_ & 3) as usize] as u32;
    BUG_ON(t == 0xf);
    t <<= if local != 0 { 20 } else { 4 };
    t
}

#[inline]
unsafe fn ar71xx_pci_bus_addr(bus: *mut pci_bus, devfn: u32, where_: i32) -> u32 {
    if (*bus).number == 0 {
        (1 << PCI_SLOT(devfn)) | (PCI_FUNC(devfn) << 8) | ((where_ as u32) & !3)
    } else {
        ((*bus).number << 16) | (PCI_SLOT(devfn) << 11) |
            (PCI_FUNC(devfn) << 8) | ((where_ as u32) & !3) | 1
    }
}

#[inline]
unsafe fn pci_bus_to_ar71xx_controller(bus: *mut pci_bus) -> *mut Ar71xxPciController {
    let hose = (*bus).sysdata as *mut pci_controller;
    container_of!(hose, Ar71xxPciController, pci_ctrl)
}

unsafe fn ar71xx_pci_check_error(apc: *mut Ar71xxPciController, quiet: i32) -> i32 {
    let base = (*apc).cfg_base as *mut u8;
    let pci_err = __raw_readl(base.add(AR71XX_PCI_REG_PCI_ERR)) & 3;
    if pci_err != 0 {
        if quiet == 0 {
            let addr = __raw_readl(base.add(AR71XX_PCI_REG_PCI_ERR_ADDR));
            pr_crit!("ar71xx: {} bus error {} at addr 0x{:x}\n", "PCI", pci_err, addr);
        }
        __raw_writel(pci_err, base.add(AR71XX_PCI_REG_PCI_ERR));
    }
    let ahb_err = __raw_readl(base.add(AR71XX_PCI_REG_AHB_ERR)) & 1;
    if ahb_err != 0 {
        if quiet == 0 {
            let addr = __raw_readl(base.add(AR71XX_PCI_REG_AHB_ERR_ADDR));
            pr_crit!("ar71xx: {} bus error {} at addr 0x{:x}\n", "AHB", ahb_err, addr);
        }
        __raw_writel(ahb_err, base.add(AR71XX_PCI_REG_AHB_ERR));
    }
    if (ahb_err | pci_err) != 0 { 1 } else { 0 }
}

#[inline]
unsafe fn ar71xx_pci_local_write(apc: *mut Ar71xxPciController, where_: i32, size: i32, mut value: u32) {
    let base = (*apc).cfg_base as *mut u8;
    value <<= 8 * ((where_ & 3) as u32);
    let mut ad_cbe = AR71XX_PCI_CRP_CMD_WRITE | ((where_ as u32) & !3);
    ad_cbe |= ar71xx_pci_get_ble(where_, size, 1);
    __raw_writel(ad_cbe, base.add(AR71XX_PCI_REG_CRP_AD_CBE));
    __raw_writel(value, base.add(AR71XX_PCI_REG_CRP_WRDATA));
}

#[inline]
unsafe fn ar71xx_pci_set_cfgaddr(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, cmd: u32) -> i32 {
    let apc = pci_bus_to_ar71xx_controller(bus);
    let base = (*apc).cfg_base as *mut u8;
    let addr = ar71xx_pci_bus_addr(bus, devfn, where_);
    __raw_writel(addr, base.add(AR71XX_PCI_REG_CFG_AD));
    __raw_writel(cmd | ar71xx_pci_get_ble(where_, size, 0), base.add(AR71XX_PCI_REG_CFG_CBE));
    ar71xx_pci_check_error(apc, 1)
}

unsafe fn ar71xx_pci_read_config(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, value: *mut u32) -> i32 {
    let apc = pci_bus_to_ar71xx_controller(bus);
    let base = (*apc).cfg_base as *mut u8;
    let err = ar71xx_pci_set_cfgaddr(bus, devfn, where_, size, AR71XX_PCI_CFG_CMD_READ);
    let data = if err != 0 { !0 } else { __raw_readl(base.add(AR71XX_PCI_REG_CFG_RDDATA)) };
    *value = (data >> (8 * ((where_ & 3) as u32))) & AR71XX_PCI_READ_MASK[(size & 7) as usize];
    if err != 0 { PCIBIOS_DEVICE_NOT_FOUND } else { PCIBIOS_SUCCESSFUL }
}

unsafe fn ar71xx_pci_write_config(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, mut value: u32) -> i32 {
    let apc = pci_bus_to_ar71xx_controller(bus);
    let base = (*apc).cfg_base as *mut u8;
    value <<= 8 * ((where_ & 3) as u32);
    let err = ar71xx_pci_set_cfgaddr(bus, devfn, where_, size, AR71XX_PCI_CFG_CMD_WRITE);
    if err == 0 { __raw_writel(value, base.add(AR71XX_PCI_REG_CFG_WRDATA)); }
    if err != 0 { PCIBIOS_DEVICE_NOT_FOUND } else { PCIBIOS_SUCCESSFUL }
}

static mut AR71XX_PCI_OPS: pci_ops = pci_ops { read: ar71xx_pci_read_config, write: ar71xx_pci_write_config };

unsafe fn ar71xx_pci_irq_handler(desc: *mut irq_desc) {
    let apc = irq_desc_get_handler_data(desc) as *mut Ar71xxPciController;
    let base = ath79_reset_base;
    let pending = __raw_readl(base.add(AR71XX_RESET_REG_PCI_INT_STATUS) as *mut u8) &
        __raw_readl(base.add(AR71XX_RESET_REG_PCI_INT_ENABLE) as *mut u8);
    if pending & AR71XX_PCI_INT_DEV0 != 0 { generic_handle_irq((*apc).irq_base + 0); }
    else if pending & AR71XX_PCI_INT_DEV1 != 0 { generic_handle_irq((*apc).irq_base + 1); }
    else if pending & AR71XX_PCI_INT_DEV2 != 0 { generic_handle_irq((*apc).irq_base + 2); }
    else if pending & AR71XX_PCI_INT_CORE != 0 { generic_handle_irq((*apc).irq_base + 4); }
    else { spurious_interrupt(); }
}

unsafe fn ar71xx_pci_irq_unmask(d: *mut irq_data) {
    let apc = irq_data_get_irq_chip_data(d) as *mut Ar71xxPciController;
    let irq = (*d).irq - (*apc).irq_base;
    let base = ath79_reset_base;
    let t = __raw_readl(base.add(AR71XX_RESET_REG_PCI_INT_ENABLE) as *mut u8);
    __raw_writel(t | (1 << irq), base.add(AR71XX_RESET_REG_PCI_INT_ENABLE) as *mut u8);
    __raw_readl(base.add(AR71XX_RESET_REG_PCI_INT_ENABLE) as *mut u8);
}

unsafe fn ar71xx_pci_irq_mask(d: *mut irq_data) {
    let apc = irq_data_get_irq_chip_data(d) as *mut Ar71xxPciController;
    let irq = (*d).irq - (*apc).irq_base;
    let base = ath79_reset_base;
    let t = __raw_readl(base.add(AR71XX_RESET_REG_PCI_INT_ENABLE) as *mut u8);
    __raw_writel(t & !(1 << irq), base.add(AR71XX_RESET_REG_PCI_INT_ENABLE) as *mut u8);
    __raw_readl(base.add(AR71XX_RESET_REG_PCI_INT_ENABLE) as *mut u8);
}

static mut AR71XX_PCI_IRQ_CHIP: irq_chip = irq_chip {
    name: "AR71XX PCI", irq_mask: ar71xx_pci_irq_mask, irq_unmask: ar71xx_pci_irq_unmask,
    irq_mask_ack: ar71xx_pci_irq_mask,
};

unsafe fn ar71xx_pci_irq_init(apc: *mut Ar71xxPciController) {
    let base = ath79_reset_base;
    __raw_writel(0, base.add(AR71XX_RESET_REG_PCI_INT_ENABLE) as *mut u8);
    __raw_writel(0, base.add(AR71XX_RESET_REG_PCI_INT_STATUS) as *mut u8);
    BUILD_BUG_ON!(ATH79_PCI_IRQ_COUNT < AR71XX_PCI_IRQ_COUNT);
    (*apc).irq_base = ATH79_PCI_IRQ_BASE;
    for i in (*apc).irq_base..(*apc).irq_base + AR71XX_PCI_IRQ_COUNT as i32 {
        irq_set_chip_and_handler(i, &raw mut AR71XX_PCI_IRQ_CHIP, handle_level_irq);
        irq_set_chip_data(i, apc as *mut core::ffi::c_void);
    }
    irq_set_chained_handler_and_data((*apc).irq, ar71xx_pci_irq_handler, apc as *mut core::ffi::c_void);
}

unsafe fn ar71xx_pci_reset() {
    ath79_device_reset_set(AR71XX_RESET_PCI_BUS | AR71XX_RESET_PCI_CORE);
    mdelay(100);
    ath79_device_reset_clear(AR71XX_RESET_PCI_BUS | AR71XX_RESET_PCI_CORE);
    mdelay(100);
    ath79_ddr_set_pci_windows();
    mdelay(100);
}

// The probe, platform-driver registration, and postcore initcall retain the C driver's external kernel interfaces.
unsafe fn ar71xx_pci_probe(pdev: *mut platform_device) -> i32 {
    let apc = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<Ar71xxPciController>(), GFP_KERNEL) as *mut Ar71xxPciController;
    if apc.is_null() { return -ENOMEM; }
    (*apc).cfg_base = devm_platform_ioremap_resource_byname(pdev, "cfg_base");
    if IS_ERR((*apc).cfg_base) { return PTR_ERR((*apc).cfg_base); }
    (*apc).irq = platform_get_irq(pdev, 0);
    if (*apc).irq < 0 { return -EINVAL; }
    let res = platform_get_resource_byname(pdev, IORESOURCE_IO, "io_base");
    if res.is_null() { return -EINVAL; }
    (*apc).io_res.parent = res; (*apc).io_res.name = "PCI IO space"; (*apc).io_res.start = (*res).start; (*apc).io_res.end = (*res).end; (*apc).io_res.flags = IORESOURCE_IO;
    let res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "mem_base");
    if res.is_null() { return -EINVAL; }
    (*apc).mem_res.parent = res; (*apc).mem_res.name = "PCI memory space"; (*apc).mem_res.start = (*res).start; (*apc).mem_res.end = (*res).end; (*apc).mem_res.flags = IORESOURCE_MEM;
    ar71xx_pci_reset();
    let t = PCI_COMMAND_MEMORY | PCI_COMMAND_MASTER | PCI_COMMAND_INVALIDATE | PCI_COMMAND_PARITY | PCI_COMMAND_SERR | PCI_COMMAND_FAST_BACK;
    ar71xx_pci_local_write(apc, PCI_COMMAND, 4, t);
    ar71xx_pci_check_error(apc, 1);
    ar71xx_pci_irq_init(apc);
    (*apc).pci_ctrl.pci_ops = &raw mut AR71XX_PCI_OPS;
    (*apc).pci_ctrl.mem_resource = &mut (*apc).mem_res;
    (*apc).pci_ctrl.io_resource = &mut (*apc).io_res;
    register_pci_controller(&mut (*apc).pci_ctrl);
    0
}

static mut AR71XX_PCI_DRIVER: platform_driver = platform_driver {
    probe: ar71xx_pci_probe,
    driver: driver { name: "ar71xx-pci" },
};

unsafe fn ar71xx_pci_init() -> i32 { platform_driver_register(&raw mut AR71XX_PCI_DRIVER) }

// postcore_initcall(ar71xx_pci_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
