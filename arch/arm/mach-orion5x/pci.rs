// SPDX-License-Identifier: GPL-2.0-only
/* arch/arm/mach-orion5x/pci.c - translated from C */

// Kernel and platform dependencies are supplied by the surrounding tree.

const PCIE_BASE: usize = ORION5X_PCIE_VIRT_BASE;

pub unsafe fn orion5x_pcie_id(dev: *mut u32, rev: *mut u32) {
    *dev = orion_pcie_dev_id(PCIE_BASE);
    *rev = orion_pcie_rev(PCIE_BASE);
}

unsafe fn pcie_valid_config(bus: i32, dev: i32) -> i32 {
    if bus == 0 && dev == 0 { return 1; }
    if !orion_pcie_link_up(PCIE_BASE) { return 0; }
    if bus == 0 && dev != 1 { return 0; }
    1
}

static mut ORION5X_PCIE_LOCK: Spinlock = Spinlock::new();

unsafe fn pcie_rd_conf(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: *mut u32) -> i32 {
    if pcie_valid_config((*bus).number, PCI_SLOT(devfn)) == 0 {
        *val = 0xffff_ffff; return PCIBIOS_DEVICE_NOT_FOUND;
    }
    let flags: usize = 0;
    spin_lock_irqsave(&mut ORION5X_PCIE_LOCK, flags);
    let ret = orion_pcie_rd_conf(PCIE_BASE, bus, devfn, where_, size, val);
    spin_unlock_irqrestore(&mut ORION5X_PCIE_LOCK, flags);
    ret
}

unsafe fn pcie_rd_conf_wa(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: *mut u32) -> i32 {
    if pcie_valid_config((*bus).number, PCI_SLOT(devfn)) == 0 || where_ >= 0x100 {
        *val = 0xffff_ffff; return PCIBIOS_DEVICE_NOT_FOUND;
    }
    orion_pcie_rd_conf_wa(ORION5X_PCIE_WA_VIRT_BASE, bus, devfn, where_, size, val)
}

unsafe fn pcie_wr_conf(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: u32) -> i32 {
    if pcie_valid_config((*bus).number, PCI_SLOT(devfn)) == 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    let flags: usize = 0;
    spin_lock_irqsave(&mut ORION5X_PCIE_LOCK, flags);
    let ret = orion_pcie_wr_conf(PCIE_BASE, bus, devfn, where_, size, val);
    spin_unlock_irqrestore(&mut ORION5X_PCIE_LOCK, flags);
    ret
}

static mut PCIE_OPS: pci_ops = pci_ops { read: pcie_rd_conf, write: pcie_wr_conf };

unsafe fn pcie_setup(sys: *mut pci_sys_data) -> i32 {
    orion_pcie_setup(PCIE_BASE);
    let dev = orion_pcie_dev_id(PCIE_BASE);
    if dev == MV88F5181_DEV_ID || dev == MV88F5182_DEV_ID {
        printk(KERN_NOTICE, "Applying Orion-1/Orion-NAS PCIe config read transaction workaround\n");
        mvebu_mbus_add_window_by_id(ORION_MBUS_PCIE_WA_TARGET, ORION_MBUS_PCIE_WA_ATTR, ORION5X_PCIE_WA_PHYS_BASE, ORION5X_PCIE_WA_SIZE);
        PCIE_OPS.read = pcie_rd_conf_wa;
    }
    let mut realio = resource { start: (*sys).busnr * SZ_64K, end: (*sys).busnr * SZ_64K + SZ_64K - 1, ..resource::default() };
    pci_remap_iospace(&mut realio, ORION5X_PCIE_IO_PHYS_BASE);
    let res = kzalloc_resource();
    if res.is_null() { panic!("pcie_setup unable to alloc resources"); }
    (*res).name = "PCIe Memory Space"; (*res).flags = IORESOURCE_MEM;
    (*res).start = ORION5X_PCIE_MEM_PHYS_BASE; (*res).end = (*res).start + ORION5X_PCIE_MEM_SIZE - 1;
    if request_resource(&mut iomem_resource, res) != 0 { panic!("Request PCIe Memory resource failed\n"); }
    pci_add_resource_offset(&mut (*sys).resources, res, (*sys).mem_offset); 1
}

const fn ORION5X_PCI_REG(x: usize) -> usize { ORION5X_PCI_VIRT_BASE + x }
const PCI_MODE: usize = ORION5X_PCI_REG(0xd00); const PCI_CMD: usize = ORION5X_PCI_REG(0xc00);
const PCI_P2P_CONF: usize = ORION5X_PCI_REG(0x1d14); const PCI_CONF_ADDR: usize = ORION5X_PCI_REG(0xc78); const PCI_CONF_DATA: usize = ORION5X_PCI_REG(0xc7c);
const PCI_MODE_64BIT: u32 = 1 << 2; const PCI_MODE_PCIX: u32 = (1 << 4) | (1 << 5); const PCI_CMD_HOST_REORDER: u32 = 1 << 29;
const PCI_P2P_BUS_OFFS: u32 = 16; const PCI_P2P_BUS_MASK: u32 = 0xff << PCI_P2P_BUS_OFFS; const PCI_P2P_DEV_OFFS: u32 = 24; const PCI_P2P_DEV_MASK: u32 = 0x1f << PCI_P2P_DEV_OFFS;
const PCIX_STAT: u32 = 0x64; const PCIX_STAT_BUS_OFFS: u32 = 8; const PCIX_STAT_BUS_MASK: u32 = 0xff << PCIX_STAT_BUS_OFFS;
const PCI_CONF_FUNC_STAT_CMD: u32 = 0; const PCI_CONF_REG_STAT_CMD: u32 = 4;
const fn PCI_CONF_REG(reg: u32) -> u32 { reg & 0xfc } const fn PCI_CONF_FUNC(func: u32) -> u32 { (func & 3) << 8 } const fn PCI_CONF_DEV(dev: u32) -> u32 { (dev & 0x1f) << 11 } const fn PCI_CONF_BUS(bus: u32) -> u32 { (bus & 0xff) << 16 } const PCI_CONF_ADDR_EN: u32 = 1 << 31;
const fn PCI_CONF_FUNC_BAR_CS(n: u32) -> u32 { n >> 1 } const fn PCI_CONF_REG_BAR_LO_CS(n: u32) -> u32 { if n & 1 != 0 { 0x18 } else { 0x10 } } const fn PCI_CONF_REG_BAR_HI_CS(n: u32) -> u32 { if n & 1 != 0 { 0x1c } else { 0x14 } }

static mut ORION5X_PCI_LOCK: Spinlock = Spinlock::new();
static mut ORION5X_PCI_CARDBUS_MODE: i32 = 0;
static mut ORION5X_PCI_DISABLED: i32 = 0;

unsafe fn orion5x_pci_local_bus_nr() -> i32 { ((readl(PCI_P2P_CONF) & PCI_P2P_BUS_MASK) >> PCI_P2P_BUS_OFFS) as i32 }
unsafe fn orion5x_pci_hw_rd_conf(bus: i32, dev: i32, func: u32, where_: u32, size: u32, val: *mut u32) -> i32 {
    let flags = 0; spin_lock_irqsave(&mut ORION5X_PCI_LOCK, flags); writel(PCI_CONF_BUS(bus as u32)|PCI_CONF_DEV(dev as u32)|PCI_CONF_REG(where_)|PCI_CONF_FUNC(func)|PCI_CONF_ADDR_EN, PCI_CONF_ADDR); *val=readl(PCI_CONF_DATA); if size==1 {*val=(*val>>(8*(where_&3)))&0xff} else if size==2 {*val=(*val>>(8*(where_&3)))&0xffff} spin_unlock_irqrestore(&mut ORION5X_PCI_LOCK, flags); PCIBIOS_SUCCESSFUL
}
unsafe fn orion5x_pci_hw_wr_conf(bus:i32,dev:i32,func:u32,where_:u32,size:u32,val:u32)->i32 { let flags=0; spin_lock_irqsave(&mut ORION5X_PCI_LOCK,flags); writel(PCI_CONF_BUS(bus as u32)|PCI_CONF_DEV(dev as u32)|PCI_CONF_REG(where_)|PCI_CONF_FUNC(func)|PCI_CONF_ADDR_EN,PCI_CONF_ADDR); let r=match size {4=>{__raw_writel(val,PCI_CONF_DATA);PCIBIOS_SUCCESSFUL},2=>{__raw_writew(val,PCI_CONF_DATA+(where_&3) as usize);PCIBIOS_SUCCESSFUL},1=>{__raw_writeb(val,PCI_CONF_DATA+(where_&3) as usize);PCIBIOS_SUCCESSFUL},_=>PCIBIOS_BAD_REGISTER_NUMBER}; spin_unlock_irqrestore(&mut ORION5X_PCI_LOCK,flags);r }
unsafe fn orion5x_pci_valid_config(bus:i32,devfn:u32)->i32 { if bus==orion5x_pci_local_bus_nr() { if PCI_SLOT(devfn)==0 && PCI_FUNC(devfn)!=0{return 0} if ORION5X_PCI_CARDBUS_MODE!=0 && PCI_SLOT(devfn)>1{return 0} } 1 }
unsafe fn orion5x_pci_rd_conf(b:*mut pci_bus,d:u32,w:i32,s:i32,v:*mut u32)->i32 { if orion5x_pci_valid_config((*b).number,d)==0 {*v=0xffff_ffff;return PCIBIOS_DEVICE_NOT_FOUND} orion5x_pci_hw_rd_conf((*b).number,PCI_SLOT(d) as i32,PCI_FUNC(d),w as u32,s as u32,v) }
unsafe fn orion5x_pci_wr_conf(b:*mut pci_bus,d:u32,w:i32,s:i32,v:u32)->i32 { if orion5x_pci_valid_config((*b).number,d)==0{return PCIBIOS_DEVICE_NOT_FOUND} orion5x_pci_hw_wr_conf((*b).number,PCI_SLOT(d) as i32,PCI_FUNC(d),w as u32,s as u32,v) }
static mut PCI_OPS:pci_ops=pci_ops{read:orion5x_pci_rd_conf,write:orion5x_pci_wr_conf};

pub unsafe fn orion5x_pci_disable(){ORION5X_PCI_DISABLED=1} pub unsafe fn orion5x_pci_set_cardbus_mode(){ORION5X_PCI_CARDBUS_MODE=1}

unsafe fn orion5x_pci_set_bus_nr(nr: i32) {
    let mut p2p=readl(PCI_P2P_CONF);
    if readl(PCI_MODE)&PCI_MODE_PCIX != 0 { let bus=((p2p&PCI_P2P_BUS_MASK)>>PCI_P2P_BUS_OFFS) as i32; let dev=((p2p&PCI_P2P_DEV_MASK)>>PCI_P2P_DEV_OFFS) as i32; let mut s=0; orion5x_pci_hw_rd_conf(bus,dev,0,PCIX_STAT,4,&mut s); s=(s&!PCIX_STAT_BUS_MASK)|(nr as u32<<PCIX_STAT_BUS_OFFS); orion5x_pci_hw_wr_conf(bus,dev,0,PCIX_STAT,4,s); } else { p2p=(p2p&!PCI_P2P_BUS_MASK)|(nr as u32<<PCI_P2P_BUS_OFFS); writel(p2p,PCI_P2P_CONF); }
}
unsafe fn orion5x_pci_master_slave_enable(){let b=orion5x_pci_local_bus_nr();let mut v=0;orion5x_pci_hw_rd_conf(b,0,PCI_CONF_FUNC_STAT_CMD,PCI_CONF_REG_STAT_CMD,4,&mut v);v|=PCI_COMMAND_IO|PCI_COMMAND_MEMORY|PCI_COMMAND_MASTER;orion5x_pci_hw_wr_conf(b,0,PCI_CONF_FUNC_STAT_CMD,PCI_CONF_REG_STAT_CMD,4,v|7);}
unsafe fn orion5x_setup_pci_wins(){let d=mv_mbus_dram_info();let mut e=0xffff_ffff;writel(e,ORION5X_PCI_REG(0xc3c));let b=orion5x_pci_local_bus_nr();for i in 0..(*d).num_cs {let c=&*((*d).cs.add(i as usize));let f=PCI_CONF_FUNC_BAR_CS(c.cs_index);let mut v=0;let lo=if c.cs_index==0{0x10}else{0x18};orion5x_pci_hw_rd_conf(b,0,f,lo,4,&mut v);orion5x_pci_hw_wr_conf(b,0,f,lo,4,(c.base&0xfffff000)|(v&0xfff));let hi=if c.cs_index==0{0x14}else{0x1c};orion5x_pci_hw_wr_conf(b,0,f,hi,4,0);writel((c.size-1)&0xfffff000,ORION5X_PCI_REG(0xc08));writel(c.base&0xfffff000,ORION5X_PCI_REG(0xc48));e&=!(1<<c.cs_index);}writel(e,ORION5X_PCI_REG(0xc3c));orion5x_setbits(ORION5X_PCI_REG(0xd3c),1);}
unsafe fn pci_setup(sys:*mut pci_sys_data)->i32{orion5x_setup_pci_wins();orion5x_pci_master_slave_enable();orion5x_setbits(PCI_CMD,PCI_CMD_HOST_REORDER);let mut r=resource{start:(*sys).busnr*SZ_64K,end:(*sys).busnr*SZ_64K+SZ_64K-1,..resource::default()};pci_remap_iospace(&mut r,ORION5X_PCI_IO_PHYS_BASE);let p=kzalloc_resource();if p.is_null(){panic!("pci_setup unable to alloc resources")}(*p).name="PCI Memory Space";(*p).flags=IORESOURCE_MEM;(*p).start=ORION5X_PCI_MEM_PHYS_BASE;(*p).end=(*p).start+ORION5X_PCI_MEM_SIZE-1;if request_resource(&mut iomem_resource,p)!=0{panic!("Request PCI Memory resource failed\n")}pci_add_resource_offset(&mut (*sys).resources,p,(*sys).mem_offset);1}
unsafe fn rc_pci_fixup(dev:*mut pci_dev){if (*(*dev).bus).parent.is_null()&&(*dev).devfn==0{(*dev).class=((*dev).class&0xff)|(PCI_CLASS_BRIDGE_HOST<<8);}}
pub unsafe fn orion5x_pci_sys_setup(nr:i32,sys:*mut pci_sys_data)->i32{vga_base=ORION5X_PCIE_MEM_PHYS_BASE;if nr==0{orion_pcie_set_local_bus_nr(PCIE_BASE,(*sys).busnr);return pcie_setup(sys)}if nr==1&&ORION5X_PCI_DISABLED==0{orion5x_pci_set_bus_nr((*sys).busnr);return pci_setup(sys)}0}
pub unsafe fn orion5x_pci_sys_scan_bus(nr:i32,bridge:*mut pci_host_bridge)->i32{let sys=pci_host_bridge_priv(bridge);list_splice_init(&mut (*sys).resources,&mut (*bridge).windows);(*bridge).dev.parent=core::ptr::null_mut();(*bridge).sysdata=sys;(*bridge).busnr=(*sys).busnr;if nr==0{(*bridge).ops=&mut PCIE_OPS;return pci_scan_root_bus_bridge(bridge)}if nr==1&&ORION5X_PCI_DISABLED==0{(*bridge).ops=&mut PCI_OPS;return pci_scan_root_bus_bridge(bridge)}BUG();-ENODEV}
pub unsafe fn orion5x_pci_map_irq(dev:*const pci_dev,_slot:u8,_pin:u8)->i32{let bus=(*(*dev).bus).number;if ORION5X_PCI_DISABLED!=0||bus<orion5x_pci_local_bus_nr(){return IRQ_ORION5X_PCIE0_INT}-1}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
