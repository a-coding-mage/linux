/* Broadcom specific AMBA PCI core in host mode. */

// Dependencies supplied by the surrounding kernel translation are intentionally external.

const BCMA_PCI_SLOT_MAX: c_uint = 16;
const PCI_CONFIG_SPACE_SIZE: c_uint = 256;

#[inline]
unsafe fn mips_busprobe32(val: *mut u32, addr: *const core::ffi::c_void) -> bool {
    get_dbe(val, addr as *const u32)
}

pub unsafe fn bcma_core_pci_is_in_hostmode(pc: *mut bcma_drv_pci) -> bool {
    let bus = (*(*pc).core).bus;
    let chipid_top = (*bus).chipinfo.id & 0xff00;
    if chipid_top != 0x4700 && chipid_top != 0x5300 { return false; }
    bcma_core_enable((*pc).core, 0);
    let mut tmp = 0u32;
    !mips_busprobe32(&mut tmp, (*(*pc).core).io_addr as *const _)
}

unsafe fn bcma_pcie_read_config(pc: *mut bcma_drv_pci, address: u32) -> u32 {
    pcicore_write32(pc, BCMA_CORE_PCI_CONFIG_ADDR, address);
    pcicore_read32(pc, BCMA_CORE_PCI_CONFIG_ADDR);
    pcicore_read32(pc, BCMA_CORE_PCI_CONFIG_DATA)
}

unsafe fn bcma_pcie_write_config(pc: *mut bcma_drv_pci, address: u32, data: u32) {
    pcicore_write32(pc, BCMA_CORE_PCI_CONFIG_ADDR, address);
    pcicore_read32(pc, BCMA_CORE_PCI_CONFIG_ADDR);
    pcicore_write32(pc, BCMA_CORE_PCI_CONFIG_DATA, data);
}

unsafe fn bcma_get_cfgspace_addr(pc: *mut bcma_drv_pci, dev: c_uint, func: c_uint, off: c_uint) -> u32 {
    let mut addr = 0u32;
    if dev >= 2 || (bcma_pcie_read(pc, BCMA_CORE_PCI_DLLP_LSREG) & BCMA_CORE_PCI_DLLP_LSREG_LINKUP) == 0 { return addr; }
    pcicore_write32(pc, BCMA_CORE_PCI_SBTOPCI1, BCMA_CORE_PCI_SBTOPCI_CFG0);
    addr = (*pc).host_controller.host_cfg_addr;
    addr |= dev << BCMA_CORE_PCI_CFG_SLOT_SHIFT;
    addr |= func << BCMA_CORE_PCI_CFG_FUN_SHIFT;
    addr |= off & !3;
    addr
}

unsafe fn bcma_extpci_read_config(pc: *mut bcma_drv_pci, dev: c_uint, func: c_uint, off: c_uint, buf: *mut core::ffi::c_void, len: c_int) -> c_int {
    let mut err = -EINVAL;
    let mut addr: u32;
    let mut val: u32 = 0;
    let mut mmio: *mut core::ffi::c_void = core::ptr::null_mut();
    WARN_ON(!(*pc).hostmode);
    if len != 1 && len != 2 && len != 4 { return err; }
    if dev == 0 {
        if func > 1 { return err; }
        if off >= PCI_CONFIG_SPACE_SIZE {
            addr = (func << 12) | (off & 0x0ffc);
            val = bcma_pcie_read_config(pc, addr);
        } else {
            addr = BCMA_CORE_PCI_PCICFG0 | (func << 8) | (off & 0xfc);
            val = pcicore_read32(pc, addr);
        }
    } else {
        addr = bcma_get_cfgspace_addr(pc, dev, func, off);
        if addr == 0 { return err; }
        err = -ENOMEM;
        mmio = ioremap(addr, core::mem::size_of::<u32>());
        if mmio.is_null() { return err; }
        if mips_busprobe32(&mut val, mmio) { val = 0xffff_ffff; }
    }
    if !mmio.is_null() && val == 0xffff_ffff { iounmap(mmio); return err; }
    val >>= 8 * (off & 3);
    match len { 1 => *(buf as *mut u8) = val as u8, 2 => *(buf as *mut u16) = val as u16, 4 => *(buf as *mut u32) = val, _ => {} }
    if !mmio.is_null() { iounmap(mmio); }
    0
}

unsafe fn bcma_extpci_write_config(pc: *mut bcma_drv_pci, dev: c_uint, func: c_uint, off: c_uint, buf: *const core::ffi::c_void, len: c_int) -> c_int {
    let mut err = -EINVAL;
    let mut addr: u32;
    let mut val: u32 = 0;
    let mut mmio: *mut core::ffi::c_void = core::ptr::null_mut();
    let chipid = (*(*pc).core).bus.chipinfo.id;
    WARN_ON(!(*pc).hostmode);
    if len != 1 && len != 2 && len != 4 { return err; }
    if dev == 0 {
        if func > 1 { return err; }
        if off >= PCI_CONFIG_SPACE_SIZE { addr = (func << 12) | (off & 0x0ffc); val = bcma_pcie_read_config(pc, addr); }
        else { addr = BCMA_CORE_PCI_PCICFG0 | (func << 8) | (off & 0xfc); val = pcicore_read32(pc, addr); }
    } else {
        addr = bcma_get_cfgspace_addr(pc, dev, func, off); if addr == 0 { return err; }
        err = -ENOMEM; mmio = ioremap(addr, core::mem::size_of::<u32>()); if mmio.is_null() { return err; }
        if mips_busprobe32(&mut val, mmio) { iounmap(mmio); return err; }
    }
    match len {
        1 => { val &= !(0xff << (8 * (off & 3))); val |= (*(buf as *const u8) as u32) << (8 * (off & 3)); }
        2 => { val &= !(0xffff << (8 * (off & 3))); val |= (*(buf as *const u16) as u32) << (8 * (off & 3)); }
        4 => val = *(buf as *const u32), _ => {}
    }
    if dev == 0 { if off >= PCI_CONFIG_SPACE_SIZE { bcma_pcie_write_config(pc, addr, val); } else { pcicore_write32(pc, addr, val); } }
    else { writel(val, mmio); if chipid == BCMA_CHIP_ID_BCM4716 || chipid == BCMA_CHIP_ID_BCM4748 { readl(mmio); } }
    if !mmio.is_null() { iounmap(mmio); }
    0
}

unsafe fn bcma_core_pci_hostmode_read_config(bus: *mut pci_bus, devfn: c_uint, reg: c_int, size: c_int, val: *mut u32) -> c_int {
    let pc_host = container_of((*bus).ops, bcma_drv_pci_host, pci_ops);
    let pc = (*pc_host).pdev; let flags = 0ul;
    spin_lock_irqsave(&mut (*pc_host).cfgspace_lock, &flags);
    let err = bcma_extpci_read_config(pc, PCI_SLOT(devfn), PCI_FUNC(devfn), reg as c_uint, val as *mut _, size);
    spin_unlock_irqrestore(&mut (*pc_host).cfgspace_lock, flags);
    if err != 0 { PCIBIOS_DEVICE_NOT_FOUND } else { PCIBIOS_SUCCESSFUL }
}

unsafe fn bcma_core_pci_hostmode_write_config(bus: *mut pci_bus, devfn: c_uint, reg: c_int, size: c_int, val: u32) -> c_int {
    let pc_host = container_of((*bus).ops, bcma_drv_pci_host, pci_ops);
    let pc = (*pc_host).pdev; let flags = 0ul;
    spin_lock_irqsave(&mut (*pc_host).cfgspace_lock, &flags);
    let err = bcma_extpci_write_config(pc, PCI_SLOT(devfn), PCI_FUNC(devfn), reg as c_uint, &val as *const _ as *const _, size);
    spin_unlock_irqrestore(&mut (*pc_host).cfgspace_lock, flags);
    if err != 0 { PCIBIOS_DEVICE_NOT_FOUND } else { PCIBIOS_SUCCESSFUL }
}

unsafe fn bcma_find_pci_capability(pc: *mut bcma_drv_pci, dev: c_uint, func: c_uint, req_cap_id: u8, buf: *mut u8, buflen: *mut u32) -> u8 {
    let mut cap_id = 0u8; let mut cap_ptr = 0u8; let mut byte_val = 0u8;
    bcma_extpci_read_config(pc, dev, func, PCI_HEADER_TYPE, &mut byte_val as *mut _, 1);
    if (byte_val & PCI_HEADER_TYPE_MASK) != PCI_HEADER_TYPE_NORMAL { return cap_ptr; }
    bcma_extpci_read_config(pc, dev, func, PCI_STATUS, &mut byte_val as *mut _, 1);
    if byte_val & PCI_STATUS_CAP_LIST == 0 { return cap_ptr; }
    bcma_extpci_read_config(pc, dev, func, PCI_CAPABILITY_LIST, &mut cap_ptr as *mut _, 1);
    if cap_ptr == 0 { return cap_ptr; }
    bcma_extpci_read_config(pc, dev, func, cap_ptr as _, &mut cap_id as *mut _, 1);
    while cap_id != req_cap_id { bcma_extpci_read_config(pc, dev, func, (cap_ptr + 1) as _, &mut cap_ptr as *mut _, 1); if cap_ptr == 0 { return cap_ptr; } bcma_extpci_read_config(pc, dev, func, cap_ptr as _, &mut cap_id as *mut _, 1); }
    if !buf.is_null() && !buflen.is_null() { let mut n = *buflen; if n == 0 { return cap_ptr; } *buflen = 0; let mut p = cap_ptr + 2; if n + p as u32 > PCI_CONFIG_SPACE_SIZE { n = PCI_CONFIG_SPACE_SIZE - p as u32; } *buflen = n; while n != 0 { bcma_extpci_read_config(pc, dev, func, p as _, buf as *mut _, 1); p += 1; buf = buf.add(1); n -= 1; } }
    cap_ptr
}

unsafe fn bcma_core_pci_enable_crs(pc: *mut bcma_drv_pci) {
    let bus = (*(*pc).core).bus; let cap_ptr = bcma_find_pci_capability(pc, 0, 0, PCI_CAP_ID_EXP, core::ptr::null_mut(), core::ptr::null_mut());
    let mut val16 = 0u16; bcma_extpci_read_config(pc, 0, 0, (cap_ptr as u32 + PCI_EXP_RTCAP) as _, &mut val16 as *mut _, 2);
    if val16 & BCMA_CORE_PCI_RC_RRS_VISIBILITY != 0 { let root_ctrl = cap_ptr as u32 + PCI_EXP_RTCTL; val16 = PCI_EXP_RTCTL_RRS_SVE; bcma_extpci_read_config(pc, 0, 0, root_ctrl, &mut val16 as *mut _, 2); for dev in 1..BCMA_PCI_SLOT_MAX { let mut i = 0; while i < 100000 { bcma_extpci_read_config(pc, dev, 0, PCI_VENDOR_ID, &mut val16 as *mut _, 2); if val16 != 1 { break; } udelay(10); i += 1; } if val16 == 1 { bcma_err(bus, "PCI: Broken device in slot %d\n", dev); } } }
}

pub unsafe fn bcma_core_pci_hostmode_init(pc: *mut bcma_drv_pci) {
    let bus = (*(*pc).core).bus; let mut tmp; let mut pci_membase_1g; let mut io_map_base;
    bcma_info(bus, "PCIEcore in host mode found\n");
    if (*bus).sprom.boardflags_lo & BCMA_CORE_PCI_BFL_NOPCI != 0 { bcma_info(bus, "This PCIE core is disabled and not working\n"); return; }
    let pc_host = kzalloc_obj::<bcma_drv_pci_host>(); if pc_host.is_null() { bcma_err(bus, "can not allocate memory"); return; }
    spin_lock_init(&mut (*pc_host).cfgspace_lock); (*pc).host_controller = pc_host; (*pc_host).pci_controller.io_resource = &mut (*pc_host).io_resource; (*pc_host).pci_controller.mem_resource = &mut (*pc_host).mem_resource; (*pc_host).pci_controller.pci_ops = &mut (*pc_host).pci_ops; (*pc_host).pdev = pc;
    pci_membase_1g = BCMA_SOC_PCI_DMA; (*pc_host).host_cfg_addr = BCMA_SOC_PCI_CFG; (*pc_host).pci_ops.read = Some(bcma_core_pci_hostmode_read_config); (*pc_host).pci_ops.write = Some(bcma_core_pci_hostmode_write_config);
    (*pc_host).mem_resource.name = cstr("BCMA PCIcore external memory"); (*pc_host).mem_resource.start = BCMA_SOC_PCI_DMA; (*pc_host).mem_resource.end = BCMA_SOC_PCI_DMA + BCMA_SOC_PCI_DMA_SZ - 1; (*pc_host).mem_resource.flags = IORESOURCE_MEM | IORESOURCE_PCI_FIXED;
    (*pc_host).io_resource.name = cstr("BCMA PCIcore external I/O"); (*pc_host).io_resource.start = 0x100; (*pc_host).io_resource.end = 0x7ff; (*pc_host).io_resource.flags = IORESOURCE_IO | IORESOURCE_PCI_FIXED;
    usleep_range(3000, 5000); pcicore_write32(pc, BCMA_CORE_PCI_CTL, BCMA_CORE_PCI_CTL_RST_OE); msleep(50); pcicore_write32(pc, BCMA_CORE_PCI_CTL, BCMA_CORE_PCI_CTL_RST | BCMA_CORE_PCI_CTL_RST_OE);
    if (*bus).chipinfo.id == BCMA_CHIP_ID_BCM4716 || (*bus).chipinfo.id == BCMA_CHIP_ID_BCM4748 { (*pc_host).mem_resource.start = BCMA_SOC_PCI_MEM; (*pc_host).mem_resource.end = BCMA_SOC_PCI_MEM + BCMA_SOC_PCI_MEM_SZ - 1; pcicore_write32(pc, BCMA_CORE_PCI_SBTOPCI0, BCMA_CORE_PCI_SBTOPCI_MEM | BCMA_SOC_PCI_MEM); }
    else if (*bus).chipinfo.id == BCMA_CHIP_ID_BCM4706 { tmp = BCMA_CORE_PCI_SBTOPCI_MEM | BCMA_CORE_PCI_SBTOPCI_PREF | BCMA_CORE_PCI_SBTOPCI_BURST; if (*(*pc).core).core_unit == 0 { (*pc_host).mem_resource.start = BCMA_SOC_PCI_MEM; (*pc_host).mem_resource.end = BCMA_SOC_PCI_MEM + BCMA_SOC_PCI_MEM_SZ - 1; (*pc_host).io_resource.start = 0x100; (*pc_host).io_resource.end = 0x47f; pci_membase_1g = BCMA_SOC_PCIE_DMA_H32; pcicore_write32(pc, BCMA_CORE_PCI_SBTOPCI0, tmp | BCMA_SOC_PCI_MEM); } else if (*(*pc).core).core_unit == 1 { (*pc_host).mem_resource.start = BCMA_SOC_PCI1_MEM; (*pc_host).mem_resource.end = BCMA_SOC_PCI1_MEM + BCMA_SOC_PCI_MEM_SZ - 1; (*pc_host).io_resource.start = 0x480; (*pc_host).io_resource.end = 0x7ff; pci_membase_1g = BCMA_SOC_PCIE1_DMA_H32; (*pc_host).host_cfg_addr = BCMA_SOC_PCI1_CFG; pcicore_write32(pc, BCMA_CORE_PCI_SBTOPCI0, tmp | BCMA_SOC_PCI1_MEM); } }
    else { pcicore_write32(pc, BCMA_CORE_PCI_SBTOPCI0, BCMA_CORE_PCI_SBTOPCI_IO); }
    pcicore_write32(pc, BCMA_CORE_PCI_SBTOPCI1, BCMA_CORE_PCI_SBTOPCI_CFG0); pcicore_write32(pc, BCMA_CORE_PCI_SBTOPCI2, BCMA_CORE_PCI_SBTOPCI_MEM | pci_membase_1g); msleep(100); bcma_core_pci_enable_crs(pc);
    if (*bus).chipinfo.id == BCMA_CHIP_ID_BCM4706 || (*bus).chipinfo.id == BCMA_CHIP_ID_BCM4716 { let mut v = 0u16; bcma_extpci_read_config(pc, 0, 0, BCMA_CORE_PCI_CFG_DEVCTRL, &mut v as *mut _, 2); v |= 2 << 5; v |= 2 << 12; bcma_extpci_write_config(pc, 0, 0, BCMA_CORE_PCI_CFG_DEVCTRL, &v as *const _ as *const _, 2); }
    tmp = PCI_COMMAND_MASTER | PCI_COMMAND_MEMORY; bcma_extpci_write_config(pc, 0, 0, PCI_COMMAND, &tmp as *const _ as *const _, 4); pcicore_write32(pc, BCMA_CORE_PCI_IMASK, BCMA_CORE_PCI_IMASK_INTA);
    io_map_base = ioremap((*pc_host).mem_resource.start, resource_size(&(*pc_host).mem_resource)) as usize; (*pc_host).pci_controller.io_map_base = io_map_base; set_io_port_base(io_map_base); usleep_range(10000, 15000); register_pci_controller(&mut (*pc_host).pci_controller);
}

unsafe fn bcma_core_pci_fixup_pcibridge(dev: *mut pci_dev) { if (*(*dev).bus).ops.read != Some(bcma_core_pci_hostmode_read_config) || PCI_SLOT((*dev).devfn) != 0 { return; } pr_info("PCI: Fixing up bridge %s\n", pci_name(dev)); pci_set_master(dev); if pcibios_enable_device(dev, !0) < 0 { pr_err("PCI: BCMA bridge enable failed\n"); return; } pci_write_config_dword(dev, BCMA_PCI_BAR1_CONTROL, 3); }
DECLARE_PCI_FIXUP_EARLY!(PCI_ANY_ID, PCI_ANY_ID, bcma_core_pci_fixup_pcibridge);

unsafe fn bcma_core_pci_fixup_addresses(dev: *mut pci_dev) { if (*(*dev).bus).ops.read != Some(bcma_core_pci_hostmode_read_config) || PCI_SLOT((*dev).devfn) == 0 { return; } pr_info("PCI: Fixing up addresses %s\n", pci_name(dev)); for pos in 0..6 { let res = &mut (*dev).resource[pos]; if res.flags & (IORESOURCE_IO | IORESOURCE_MEM) != 0 { if pci_assign_resource(dev, pos as c_int) != 0 { pr_err("PCI: Problem fixing up the addresses on %s\n", pci_name(dev)); } } } }
DECLARE_PCI_FIXUP_HEADER!(PCI_ANY_ID, PCI_ANY_ID, bcma_core_pci_fixup_addresses);

pub unsafe fn bcma_core_pci_plat_dev_init(dev: *mut pci_dev) -> c_int { if (*(*dev).bus).ops.read != Some(bcma_core_pci_hostmode_read_config) { return -ENODEV; } let pc_host = container_of((*(*dev).bus).ops, bcma_drv_pci_host, pci_ops); pr_info("PCI: Fixing up device %s\n", pci_name(dev)); (*dev).irq = bcma_core_irq((*(*pc_host).pdev).core, 0); pci_write_config_byte(dev, PCI_INTERRUPT_LINE, (*dev).irq); let readrq = pcie_get_readrq(dev); if readrq > 128 { pr_info("change PCIe max read request size from %i to 128\n", readrq); pcie_set_readrq(dev, 128); } 0 }
EXPORT_SYMBOL!(bcma_core_pci_plat_dev_init);

pub unsafe fn bcma_core_pci_pcibios_map_irq(dev: *const pci_dev) -> c_int { if (*(*dev).bus).ops.read != Some(bcma_core_pci_hostmode_read_config) { return -ENODEV; } let pc_host = container_of((*(*dev).bus).ops, bcma_drv_pci_host, pci_ops); bcma_core_irq((*(*pc_host).pdev).core, 0) }
EXPORT_SYMBOL!(bcma_core_pci_pcibios_map_irq);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
