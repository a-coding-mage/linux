// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Faithful low-level Rust translation of pci-ar2315.c.
 * Kernel types, constants, and functions are supplied by external dependencies.
 */

pub const AR2315_PCI_1MS_REG: u32 = 0x0008;
pub const AR2315_PCI_1MS_MASK: u32 = 0x3ffff;
pub const AR2315_PCI_MISC_CONFIG: u32 = 0x000c;
pub const AR2315_PCIMISC_TXD_EN: u32 = 0x00000001;
pub const AR2315_PCIMISC_CFG_SEL: u32 = 0x00000002;
pub const AR2315_PCIMISC_GIG_MASK: u32 = 0x0000000c;
pub const AR2315_PCIMISC_RST_MODE: u32 = 0x00000030;
pub const AR2315_PCIRST_INPUT: u32 = 0x00000000;
pub const AR2315_PCIRST_LOW: u32 = 0x00000010;
pub const AR2315_PCIRST_HIGH: u32 = 0x00000020;
pub const AR2315_PCIGRANT_EN: u32 = 0x00000000;
pub const AR2315_PCIGRANT_FRAME: u32 = 0x00000040;
pub const AR2315_PCIGRANT_IDLE: u32 = 0x00000080;
pub const AR2315_PCIGRANT_GAP: u32 = 0x00000000;
pub const AR2315_PCICACHE_DIS: u32 = 0x00001000;
pub const AR2315_PCI_OUT_TSTAMP: u32 = 0x0010;
pub const AR2315_PCI_UNCACHE_CFG: u32 = 0x0014;
pub const AR2315_PCI_IN_EN: u32 = 0x0100;
pub const AR2315_PCI_IN_EN0: u32 = 0x01;
pub const AR2315_PCI_IN_EN1: u32 = 0x02;
pub const AR2315_PCI_IN_EN2: u32 = 0x04;
pub const AR2315_PCI_IN_EN3: u32 = 0x08;
pub const AR2315_PCI_IN_DIS: u32 = 0x0104;
pub const AR2315_PCI_IN_DIS0: u32 = 0x01;
pub const AR2315_PCI_IN_DIS1: u32 = 0x02;
pub const AR2315_PCI_IN_DIS2: u32 = 0x04;
pub const AR2315_PCI_IN_DIS3: u32 = 0x08;
pub const AR2315_PCI_IN_PTR: u32 = 0x0200;
pub const AR2315_PCI_OUT_EN: u32 = 0x0400;
pub const AR2315_PCI_OUT_EN0: u32 = 0x01;
pub const AR2315_PCI_OUT_DIS: u32 = 0x0404;
pub const AR2315_PCI_OUT_DIS0: u32 = 0x01;
pub const AR2315_PCI_OUT_PTR: u32 = 0x0408;
pub const AR2315_PCI_ISR: u32 = 0x0500;
pub const AR2315_PCI_INT_TX: u32 = 0x00000001;
pub const AR2315_PCI_INT_TXOK: u32 = 0x00000002;
pub const AR2315_PCI_INT_TXERR: u32 = 0x00000004;
pub const AR2315_PCI_INT_TXEOL: u32 = 0x00000008;
pub const AR2315_PCI_INT_RX: u32 = 0x00000010;
pub const AR2315_PCI_INT_RXOK: u32 = 0x00000020;
pub const AR2315_PCI_INT_RXERR: u32 = 0x00000040;
pub const AR2315_PCI_INT_RXEOL: u32 = 0x00000080;
pub const AR2315_PCI_INT_TXOOD: u32 = 0x00000200;
pub const AR2315_PCI_INT_DESCMASK: u32 = 0x0000ffff;
pub const AR2315_PCI_INT_EXT: u32 = 0x02000000;
pub const AR2315_PCI_INT_ABORT: u32 = 0x04000000;
pub const AR2315_PCI_IMR: u32 = 0x0504;
pub const AR2315_PCI_IER: u32 = 0x0508;
pub const AR2315_PCI_IER_DISABLE: u32 = 0;
pub const AR2315_PCI_IER_ENABLE: u32 = 1;
pub const AR2315_PCI_HOST_IN_EN: u32 = 0x0800;
pub const AR2315_PCI_HOST_IN_DIS: u32 = 0x0804;
pub const AR2315_PCI_HOST_IN_PTR: u32 = 0x0810;
pub const AR2315_PCI_HOST_OUT_EN: u32 = 0x0900;
pub const AR2315_PCI_HOST_OUT_DIS: u32 = 0x0904;
pub const AR2315_PCI_HOST_OUT_PTR: u32 = 0x0908;
pub const AR2315_PCI_IRQ_EXT: u32 = 25;
pub const AR2315_PCI_IRQ_ABORT: u32 = 26;
pub const AR2315_PCI_IRQ_COUNT: u32 = 27;
pub const AR2315_PCI_CFG_SIZE: usize = 0x00100000;
pub const AR2315_PCI_HOST_SLOT: u32 = 3;
pub const AR2315_PCI_HOST_SDRAM_BASEADDR: u64 = 0x20000000;
pub const AR2315_PCI_HOST_MBAR0: u32 = 0x10000000;
pub const AR2315_PCI_HOST_MBAR1: u64 = AR2315_PCI_HOST_SDRAM_BASEADDR;
pub const AR2315_PCI_HOST_MBAR2: u32 = 0x30000000;

#[repr(C)]
pub struct ar2315_pci_ctrl {
    pub cfg_mem: *mut u8,
    pub mmr_mem: *mut u8,
    pub irq: u32,
    pub irq_ext: u32,
    pub domain: *mut irq_domain,
    pub pci_ctrl: pci_controller,
    pub mem_res: resource,
    pub io_res: resource,
}

#[inline]
pub unsafe fn ar2315_dev_offset(dev: *mut device) -> u64 {
    if !dev.is_null() && dev_is_pci(dev) { AR2315_PCI_HOST_SDRAM_BASEADDR } else { 0 }
}

pub unsafe fn phys_to_dma(dev: *mut device, paddr: u64) -> u64 { paddr.wrapping_add(ar2315_dev_offset(dev)) }
pub unsafe fn dma_to_phys(dev: *mut device, dma_addr: u64) -> u64 { dma_addr.wrapping_sub(ar2315_dev_offset(dev)) }

#[inline]
pub unsafe fn ar2315_pci_bus_to_apc(bus: *mut pci_bus) -> *mut ar2315_pci_ctrl {
    container_of((*bus).sysdata, ar2315_pci_ctrl, pci_ctrl)
}

#[inline]
pub unsafe fn ar2315_pci_reg_read(apc: *mut ar2315_pci_ctrl, reg: u32) -> u32 {
    core::ptr::read_volatile((*apc).mmr_mem.add(reg as usize) as *const u32)
}

#[inline]
pub unsafe fn ar2315_pci_reg_write(apc: *mut ar2315_pci_ctrl, reg: u32, val: u32) {
    core::ptr::write_volatile((*apc).mmr_mem.add(reg as usize) as *mut u32, val)
}

#[inline]
pub unsafe fn ar2315_pci_reg_mask(apc: *mut ar2315_pci_ctrl, reg: u32, mask: u32, val: u32) {
    let mut ret = ar2315_pci_reg_read(apc, reg);
    ret &= !mask;
    ret |= val;
    ar2315_pci_reg_write(apc, reg, ret);
}

pub unsafe fn ar2315_pci_cfg_access(apc: *mut ar2315_pci_ctrl, devfn: u32, where_: i32, size: i32, ptr: *mut u32, write: bool) -> i32 {
    let func = PCI_FUNC(devfn);
    let dev = PCI_SLOT(devfn);
    let addr = (1u32 << (13 + dev)) | (func << 8) | ((where_ as u32) & !3);
    let mask = 0xffff_ffffu32 >> (8 * (4 - size));
    let sh = ((where_ as u32) & 3) * 8;
    let mut value: u32;
    let mut isr: u32;
    if addr >= AR2315_PCI_CFG_SIZE as u32 || dev > 18 { return PCIBIOS_DEVICE_NOT_FOUND; }
    ar2315_pci_reg_write(apc, AR2315_PCI_ISR, AR2315_PCI_INT_ABORT);
    ar2315_pci_reg_mask(apc, AR2315_PCI_MISC_CONFIG, 0, AR2315_PCIMISC_CFG_SEL);
    mb();
    value = core::ptr::read_volatile((*apc).cfg_mem.add(addr as usize) as *const u32);
    isr = ar2315_pci_reg_read(apc, AR2315_PCI_ISR);
    if isr & AR2315_PCI_INT_ABORT != 0 { goto_exit_err(apc, ptr, write, &mut isr); }
    if write {
        value = (value & !(mask << sh)) | (*ptr << sh);
        core::ptr::write_volatile((*apc).cfg_mem.add(addr as usize) as *mut u32, value);
        isr = ar2315_pci_reg_read(apc, AR2315_PCI_ISR);
        if isr & AR2315_PCI_INT_ABORT != 0 { goto_exit_err(apc, ptr, write, &mut isr); }
    } else { *ptr = (value >> sh) & mask; }
    ar2315_pci_reg_mask(apc, AR2315_PCI_MISC_CONFIG, AR2315_PCIMISC_CFG_SEL, 0);
    return if isr & AR2315_PCI_INT_ABORT != 0 { PCIBIOS_DEVICE_NOT_FOUND } else { PCIBIOS_SUCCESSFUL };
}

unsafe fn goto_exit_err(apc: *mut ar2315_pci_ctrl, ptr: *mut u32, write: bool, isr: &mut u32) {
    ar2315_pci_reg_write(apc, AR2315_PCI_ISR, AR2315_PCI_INT_ABORT);
    if !write { *ptr = 0xffff_ffff; }
    *isr |= AR2315_PCI_INT_ABORT;
}

#[inline] pub unsafe fn ar2315_pci_local_cfg_rd(a: *mut ar2315_pci_ctrl, d: u32, w: i32, v: *mut u32) -> i32 { ar2315_pci_cfg_access(a,d,w,4,v,false) }
#[inline] pub unsafe fn ar2315_pci_local_cfg_wr(a: *mut ar2315_pci_ctrl, d: u32, w: i32, v: u32) -> i32 { ar2315_pci_cfg_access(a,d,w,4,&v as *const u32 as *mut u32,true) }

pub unsafe fn ar2315_pci_cfg_read(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, value: *mut u32) -> i32 {
    let apc = ar2315_pci_bus_to_apc(bus);
    if PCI_SLOT(devfn) == AR2315_PCI_HOST_SLOT { return PCIBIOS_DEVICE_NOT_FOUND; }
    ar2315_pci_cfg_access(apc, devfn, where_, size, value, false)
}
pub unsafe fn ar2315_pci_cfg_write(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, value: u32) -> i32 {
    let apc = ar2315_pci_bus_to_apc(bus);
    if PCI_SLOT(devfn) == AR2315_PCI_HOST_SLOT { return PCIBIOS_DEVICE_NOT_FOUND; }
    ar2315_pci_cfg_access(apc, devfn, where_, size, &value as *const u32 as *mut u32, true)
}

#[repr(C)] pub struct pci_ops { pub read: unsafe fn(*mut pci_bus,u32,i32,i32,*mut u32)->i32, pub write: unsafe fn(*mut pci_bus,u32,i32,i32,u32)->i32 }
pub static mut ar2315_pci_ops: pci_ops = pci_ops { read: ar2315_pci_cfg_read, write: ar2315_pci_cfg_write };

// Remaining kernel driver structures and callbacks are represented below with their source-level control flow.
pub unsafe fn ar2315_pci_host_setup(apc: *mut ar2315_pci_ctrl) -> i32 {
    let devfn = PCI_DEVFN(AR2315_PCI_HOST_SLOT, 0); let mut id = 0u32;
    let res = ar2315_pci_local_cfg_rd(apc, devfn, PCI_VENDOR_ID, &mut id);
    if res != PCIBIOS_SUCCESSFUL || id != AR2315_PCI_HOST_DEVID { return -ENODEV; }
    ar2315_pci_local_cfg_wr(apc, devfn, PCI_BASE_ADDRESS_0, AR2315_PCI_HOST_MBAR0);
    ar2315_pci_local_cfg_wr(apc, devfn, PCI_BASE_ADDRESS_1, AR2315_PCI_HOST_MBAR1 as u32);
    ar2315_pci_local_cfg_wr(apc, devfn, PCI_BASE_ADDRESS_2, AR2315_PCI_HOST_MBAR2);
    ar2315_pci_local_cfg_wr(apc, devfn, PCI_COMMAND, PCI_COMMAND_MEMORY|PCI_COMMAND_MASTER|PCI_COMMAND_SPECIAL|PCI_COMMAND_INVALIDATE|PCI_COMMAND_PARITY|PCI_COMMAND_SERR|PCI_COMMAND_FAST_BACK);
    0
}

pub unsafe fn ar2315_pci_irq_handler(desc: *mut irq_desc) {
    let apc = irq_desc_get_handler_data(desc) as *mut ar2315_pci_ctrl;
    let pending = ar2315_pci_reg_read(apc, AR2315_PCI_ISR) & ar2315_pci_reg_read(apc, AR2315_PCI_IMR);
    let mut ret = 0;
    if pending != 0 { ret = generic_handle_domain_irq((*apc).domain, pending.trailing_zeros()); }
    if pending == 0 || ret != 0 { spurious_interrupt(); }
}

pub unsafe fn ar2315_pci_irq_mask(d: *mut irq_data) { let a = irq_data_get_irq_chip_data(d) as *mut ar2315_pci_ctrl; ar2315_pci_reg_mask(a, AR2315_PCI_IMR, 1u32 << (*d).hwirq, 0); }
pub unsafe fn ar2315_pci_irq_mask_ack(d: *mut irq_data) { let a = irq_data_get_irq_chip_data(d) as *mut ar2315_pci_ctrl; let m = 1u32 << (*d).hwirq; ar2315_pci_reg_mask(a, AR2315_PCI_IMR,m,0); ar2315_pci_reg_write(a,AR2315_PCI_ISR,m); }
pub unsafe fn ar2315_pci_irq_unmask(d: *mut irq_data) { let a = irq_data_get_irq_chip_data(d) as *mut ar2315_pci_ctrl; ar2315_pci_reg_mask(a,AR2315_PCI_IMR,0,1u32 << (*d).hwirq); }

pub unsafe fn ar2315_pci_irq_map(d: *mut irq_domain, irq: u32, _hw: u32) -> i32 { irq_set_chip_and_handler(irq, &mut ar2315_pci_irq_chip, handle_level_irq); irq_set_chip_data(irq, (*d).host_data); 0 }
pub static mut ar2315_pci_irq_chip: irq_chip = irq_chip { name: "AR2315-PCI", irq_mask: Some(ar2315_pci_irq_mask), irq_mask_ack: Some(ar2315_pci_irq_mask_ack), irq_unmask: Some(ar2315_pci_irq_unmask) };

pub unsafe fn ar2315_pci_irq_init(apc: *mut ar2315_pci_ctrl) {
    ar2315_pci_reg_mask(apc,AR2315_PCI_IER,AR2315_PCI_IER_ENABLE,0);
    ar2315_pci_reg_mask(apc,AR2315_PCI_IMR,AR2315_PCI_INT_ABORT|AR2315_PCI_INT_EXT,0);
    (*apc).irq_ext = irq_create_mapping((*apc).domain, AR2315_PCI_IRQ_EXT);
    irq_set_chained_handler_and_data((*apc).irq, ar2315_pci_irq_handler, apc as *mut _);
    ar2315_pci_reg_write(apc,AR2315_PCI_ISR,AR2315_PCI_INT_ABORT|AR2315_PCI_INT_EXT);
    ar2315_pci_reg_mask(apc,AR2315_PCI_IER,0,AR2315_PCI_IER_ENABLE);
}

// Platform probe follows the C implementation; external kernel allocation, mapping,
// resource, IRQ-domain, and PCI-registration helpers are intentionally unresolved.
pub unsafe fn ar2315_pci_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let apc = devm_kzalloc(dev, core::mem::size_of::<ar2315_pci_ctrl>(), GFP_KERNEL) as *mut ar2315_pci_ctrl;
    if apc.is_null() { return -ENOMEM; }
    let irq = platform_get_irq(pdev,0); if irq < 0 { return -EINVAL; } (*apc).irq = irq as u32;
    (*apc).mmr_mem = devm_platform_ioremap_resource_byname(pdev,"ar2315-pci-ctrl");
    if (*apc).mmr_mem.is_null() { return PTR_ERR((*apc).mmr_mem); }
    let res = platform_get_resource_byname(pdev,IORESOURCE_MEM,"ar2315-pci-ext"); if res.is_null() { return -EINVAL; }
    (*apc).mem_res = resource { name:"AR2315 PCI mem space", parent:res, start:(*res).start, end:(*res).end, flags:IORESOURCE_MEM };
    (*apc).cfg_mem = devm_ioremap(dev,(*res).start,AR2315_PCI_CFG_SIZE); if (*apc).cfg_mem.is_null() { dev_err(dev,"failed to remap PCI config space\n"); return -ENOMEM; }
    ar2315_pci_reg_mask(apc,AR2315_PCI_MISC_CONFIG,AR2315_PCIMISC_RST_MODE,AR2315_PCIRST_LOW); msleep(100);
    ar2315_pci_reg_mask(apc,AR2315_PCI_MISC_CONFIG,AR2315_PCIMISC_RST_MODE,AR2315_PCIRST_HIGH|AR2315_PCICACHE_DIS|8);
    ar2315_pci_reg_write(apc,AR2315_PCI_UNCACHE_CFG,0x1e|(1<<5)|(2<<30)); ar2315_pci_reg_read(apc,AR2315_PCI_UNCACHE_CFG); msleep(500);
    let err=ar2315_pci_host_setup(apc); if err!=0{return err;}
    (*apc).domain=irq_domain_create_linear(core::ptr::null_mut(),AR2315_PCI_IRQ_COUNT,&ar2315_pci_irq_domain_ops,apc as *mut _); if (*apc).domain.is_null(){return -ENOMEM;}
    ar2315_pci_irq_init(apc); (*apc).io_res=resource{name:"AR2315 IO space",parent:core::ptr::null_mut(),start:0,end:0,flags:IORESOURCE_IO}; register_pci_controller(&mut (*apc).pci_ctrl); 0
}

pub static mut ar2315_pci_irq_domain_ops: irq_domain_ops = irq_domain_ops { map: Some(ar2315_pci_irq_map) };

pub unsafe fn pcibios_map_irq(dev: *const pci_dev, slot: u8, _pin: u8) -> u32 { let apc = ar2315_pci_bus_to_apc((*dev).bus); if slot != 0 { 0 } else { (*apc).irq_ext } }
pub unsafe fn pcibios_plat_dev_init(_dev: *mut pci_dev) -> i32 { 0 }

pub unsafe fn ar2315_pci_init() -> i32 { platform_driver_register(&mut ar2315_pci_driver) }
pub static mut ar2315_pci_driver: platform_driver = platform_driver { probe: Some(ar2315_pci_probe), name: "ar2315-pci" };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
