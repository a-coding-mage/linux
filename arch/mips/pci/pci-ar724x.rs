// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Atheros AR724X PCI host controller driver
 *
 *  Copyright (C) 2011 René Bolldorf <xsecute@googlemail.com>
 *  Copyright (C) 2009-2011 Gabor Juhos <juhosg@openwrt.org>
 */

// Linux kernel and ath79 dependencies supplied by other files.

const AR724X_PCI_REG_APP: usize = 0x00;
const AR724X_PCI_REG_RESET: usize = 0x18;
const AR724X_PCI_REG_INT_STATUS: usize = 0x4c;
const AR724X_PCI_REG_INT_MASK: usize = 0x50;

const AR724X_PCI_APP_LTSSM_ENABLE: u32 = 1 << 0;
const AR724X_PCI_RESET_LINK_UP: u32 = 1 << 0;
const AR724X_PCI_INT_DEV0: u32 = 1 << 14;
const AR724X_PCI_IRQ_COUNT: i32 = 1;
const AR7240_BAR0_WAR_VALUE: u32 = 0xffff;

const AR724X_PCI_CMD_INIT: u32 = PCI_COMMAND_MEMORY
    | PCI_COMMAND_MASTER
    | PCI_COMMAND_INVALIDATE
    | PCI_COMMAND_PARITY
    | PCI_COMMAND_SERR
    | PCI_COMMAND_FAST_BACK;

#[repr(C)]
struct Ar724xPciController {
    devcfg_base: *mut core::ffi::c_void,
    ctrl_base: *mut core::ffi::c_void,
    crp_base: *mut core::ffi::c_void,
    irq: i32,
    irq_base: i32,
    link_up: bool,
    bar0_is_cached: bool,
    bar0_value: u32,
    pci_controller: PciController,
    io_res: Resource,
    mem_res: Resource,
}

#[inline]
unsafe fn ar724x_pci_check_link(apc: *mut Ar724xPciController) -> bool {
    let reset = __raw_readl((*apc).ctrl_base.add(AR724X_PCI_REG_RESET));
    reset & AR724X_PCI_RESET_LINK_UP != 0
}

#[inline]
unsafe fn pci_bus_to_ar724x_controller(bus: *mut PciBus) -> *mut Ar724xPciController {
    let hose = (*bus).sysdata as *mut PciController;
    container_of!(hose, Ar724xPciController, pci_controller)
}

unsafe fn ar724x_pci_local_write(
    apc: *mut Ar724xPciController,
    where_: i32,
    size: i32,
    value: u32,
) -> i32 {
    WARN_ON!(where_ & (size - 1));
    if !(*apc).link_up { return PCIBIOS_DEVICE_NOT_FOUND; }
    let base = (*apc).crp_base;
    let mut data = __raw_readl(base.add((where_ & !3) as usize));
    match size {
        1 => { let s = (where_ & 3) * 8; data &= !(0xff << s); data |= (value & 0xff) << s; }
        2 => { let s = (where_ & 2) * 8; data &= !(0xffff << s); data |= (value & 0xffff) << s; }
        4 => data = value,
        _ => return PCIBIOS_BAD_REGISTER_NUMBER,
    }
    __raw_writel(data, base.add((where_ & !3) as usize));
    __raw_readl(base.add((where_ & !3) as usize));
    PCIBIOS_SUCCESSFUL
}

unsafe fn ar724x_pci_read(bus: *mut PciBus, devfn: u32, where_: i32, size: i32, value: *mut u32) -> i32 {
    let apc = pci_bus_to_ar724x_controller(bus);
    if !(*apc).link_up || devfn != 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    let base = (*apc).devcfg_base;
    let mut data = __raw_readl(base.add((where_ & !3) as usize));
    match size {
        1 => { if where_ & 1 != 0 { data >>= 8; } if where_ & 2 != 0 { data >>= 16; } data &= 0xff; }
        2 => { if where_ & 2 != 0 { data >>= 16; } data &= 0xffff; }
        4 => {}
        _ => return PCIBIOS_BAD_REGISTER_NUMBER,
    }
    if where_ == PCI_BASE_ADDRESS_0 && size == 4 && (*apc).bar0_is_cached { *value = (*apc).bar0_value; } else { *value = data; }
    PCIBIOS_SUCCESSFUL
}

unsafe fn ar724x_pci_write(bus: *mut PciBus, devfn: u32, where_: i32, size: i32, mut value: u32) -> i32 {
    let apc = pci_bus_to_ar724x_controller(bus);
    if !(*apc).link_up || devfn != 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    if soc_is_ar7240() && where_ == PCI_BASE_ADDRESS_0 && size == 4 {
        if value != 0xffff_ffff {
            /* WAR for a hw issue: cache the intended BAR0 value and write the SoC-specific value. */
            (*apc).bar0_is_cached = true;
            (*apc).bar0_value = value;
            value = AR7240_BAR0_WAR_VALUE;
        } else { (*apc).bar0_is_cached = false; }
    }
    let base = (*apc).devcfg_base;
    let mut data = __raw_readl(base.add((where_ & !3) as usize));
    match size {
        1 => { let s = (where_ & 3) * 8; data &= !(0xff << s); data |= (value & 0xff) << s; }
        2 => { let s = (where_ & 2) * 8; data &= !(0xffff << s); data |= (value & 0xffff) << s; }
        4 => data = value,
        _ => return PCIBIOS_BAD_REGISTER_NUMBER,
    }
    __raw_writel(data, base.add((where_ & !3) as usize));
    __raw_readl(base.add((where_ & !3) as usize));
    PCIBIOS_SUCCESSFUL
}

static mut AR724X_PCI_OPS: PciOps = PciOps { read: ar724x_pci_read, write: ar724x_pci_write };

unsafe fn ar724x_pci_irq_handler(desc: *mut IrqDesc) {
    let apc = irq_desc_get_handler_data(desc) as *mut Ar724xPciController;
    let base = (*apc).ctrl_base;
    let pending = __raw_readl(base.add(AR724X_PCI_REG_INT_STATUS)) & __raw_readl(base.add(AR724X_PCI_REG_INT_MASK));
    if pending & AR724X_PCI_INT_DEV0 != 0 { generic_handle_irq((*apc).irq_base); } else { spurious_interrupt(); }
}

unsafe fn ar724x_pci_irq_unmask(d: *mut IrqData) {
    let apc = irq_data_get_irq_chip_data(d) as *mut Ar724xPciController;
    let base = (*apc).ctrl_base;
    match (*apc).irq_base - (*d).irq { 0 => { let t = __raw_readl(base.add(AR724X_PCI_REG_INT_MASK)); __raw_writel(t | AR724X_PCI_INT_DEV0, base.add(AR724X_PCI_REG_INT_MASK)); __raw_readl(base.add(AR724X_PCI_REG_INT_MASK)); }, _ => {} }
}

unsafe fn ar724x_pci_irq_mask(d: *mut IrqData) {
    let apc = irq_data_get_irq_chip_data(d) as *mut Ar724xPciController;
    let base = (*apc).ctrl_base;
    match (*apc).irq_base - (*d).irq { 0 => {
        let mut t = __raw_readl(base.add(AR724X_PCI_REG_INT_MASK));
        __raw_writel(t & !AR724X_PCI_INT_DEV0, base.add(AR724X_PCI_REG_INT_MASK)); __raw_readl(base.add(AR724X_PCI_REG_INT_MASK));
        t = __raw_readl(base.add(AR724X_PCI_REG_INT_STATUS)); __raw_writel(t | AR724X_PCI_INT_DEV0, base.add(AR724X_PCI_REG_INT_STATUS)); __raw_readl(base.add(AR724X_PCI_REG_INT_STATUS));
    }, _ => {} }
}

static mut AR724X_PCI_IRQ_CHIP: IrqChip = IrqChip { name: "AR724X PCI ", irq_mask: ar724x_pci_irq_mask, irq_unmask: ar724x_pci_irq_unmask, irq_mask_ack: ar724x_pci_irq_mask };

unsafe fn ar724x_pci_irq_init(apc: *mut Ar724xPciController, id: i32) {
    let base = (*apc).ctrl_base;
    __raw_writel(0, base.add(AR724X_PCI_REG_INT_MASK)); __raw_writel(0, base.add(AR724X_PCI_REG_INT_STATUS));
    (*apc).irq_base = ATH79_PCI_IRQ_BASE + id * AR724X_PCI_IRQ_COUNT;
    for i in (*apc).irq_base..(*apc).irq_base + AR724X_PCI_IRQ_COUNT { irq_set_chip_and_handler(i, &mut AR724X_PCI_IRQ_CHIP, handle_level_irq); irq_set_chip_data(i, apc as *mut _); }
    irq_set_chained_handler_and_data((*apc).irq, ar724x_pci_irq_handler, apc as *mut _);
}

unsafe fn ar724x_pci_hw_init(apc: *mut Ar724xPciController) {
    let mut ppl; let mut app; let mut wait = 0;
    ath79_device_reset_clear(AR724X_RESET_PCIE); ath79_device_reset_clear(AR724X_RESET_PCIE_PHY);
    ppl = ath79_pll_rr(AR724X_PLL_REG_PCIE_CONFIG); ppl &= !AR724X_PLL_REG_PCIE_CONFIG_PPL_RESET; ath79_pll_wr(AR724X_PLL_REG_PCIE_CONFIG, ppl);
    ppl = ath79_pll_rr(AR724X_PLL_REG_PCIE_CONFIG); ppl &= !AR724X_PLL_REG_PCIE_CONFIG_PPL_BYPASS; ath79_pll_wr(AR724X_PLL_REG_PCIE_CONFIG, ppl);
    app = __raw_readl((*apc).ctrl_base.add(AR724X_PCI_REG_APP)); app |= AR724X_PCI_APP_LTSSM_ENABLE; __raw_writel(app, (*apc).ctrl_base.add(AR724X_PCI_REG_APP));
    loop { mdelay(10); wait += 1; if !(wait < 10 && !ar724x_pci_check_link(apc)) { break; } }
}

unsafe fn ar724x_pci_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut id = (*pdev).id; if id == -1 { id = 0; }
    let apc = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<Ar724xPciController>(), GFP_KERNEL) as *mut Ar724xPciController;
    if apc.is_null() { return -ENOMEM; }
    (*apc).ctrl_base = devm_platform_ioremap_resource_byname(pdev, "ctrl_base"); if IS_ERR!((*apc).ctrl_base) { return PTR_ERR!((*apc).ctrl_base); }
    (*apc).devcfg_base = devm_platform_ioremap_resource_byname(pdev, "cfg_base"); if IS_ERR!((*apc).devcfg_base) { return PTR_ERR!((*apc).devcfg_base); }
    (*apc).crp_base = devm_platform_ioremap_resource_byname(pdev, "crp_base"); if IS_ERR!((*apc).crp_base) { return PTR_ERR!((*apc).crp_base); }
    (*apc).irq = platform_get_irq(pdev, 0); if (*apc).irq < 0 { return -EINVAL; }
    let res = platform_get_resource_byname(pdev, IORESOURCE_IO, "io_base"); if res.is_null() { return -EINVAL; }
    (*apc).io_res.parent = res; (*apc).io_res.name = "PCI IO space"; (*apc).io_res.start = (*res).start; (*apc).io_res.end = (*res).end; (*apc).io_res.flags = IORESOURCE_IO;
    let res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "mem_base"); if res.is_null() { return -EINVAL; }
    (*apc).mem_res.parent = res; (*apc).mem_res.name = "PCI memory space"; (*apc).mem_res.start = (*res).start; (*apc).mem_res.end = (*res).end; (*apc).mem_res.flags = IORESOURCE_MEM;
    (*apc).pci_controller.pci_ops = &mut AR724X_PCI_OPS; (*apc).pci_controller.io_resource = &mut (*apc).io_res; (*apc).pci_controller.mem_resource = &mut (*apc).mem_res;
    if ath79_reset_rr(AR724X_RESET_REG_RESET_MODULE) & AR724X_RESET_PCIE != 0 { ar724x_pci_hw_init(apc); }
    (*apc).link_up = ar724x_pci_check_link(apc); if !(*apc).link_up { dev_warn!(&mut (*pdev).dev, "PCIe link is down\n"); }
    ar724x_pci_irq_init(apc, id); ar724x_pci_local_write(apc, PCI_COMMAND, 4, AR724X_PCI_CMD_INIT); register_pci_controller(&mut (*apc).pci_controller); 0
}

static mut AR724X_PCI_DRIVER: PlatformDriver = PlatformDriver { probe: ar724x_pci_probe, driver: Driver { name: "ar724x-pci" } };

unsafe fn ar724x_pci_init() -> i32 { platform_driver_register(&mut AR724X_PCI_DRIVER) }

postcore_initcall!(ar724x_pci_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
