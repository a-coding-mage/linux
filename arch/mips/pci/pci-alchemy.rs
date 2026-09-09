// SPDX-License-Identifier: GPL-2.0
/* Alchemy PCI host mode support. */

// Kernel and architecture dependencies are supplied by the surrounding tree.

const PCI_ACCESS_READ: u8 = 0;
const PCI_ACCESS_WRITE: u8 = 1;

#[repr(C)]
struct alchemy_pci_context {
    alchemy_pci_ctrl: pci_controller,
    regs: *mut core::ffi::c_void,
    last_elo0: usize,
    last_elo1: usize,
    wired_entry: i32,
    pci_cfg_vm: *mut vm_struct,
    pm: [usize; 12],
    board_map_irq: Option<unsafe extern "C" fn(*const pci_dev, u8, u8) -> i32>,
    board_pci_idsel: Option<unsafe extern "C" fn(u32, i32) -> i32>,
}

static mut __alchemy_pci_ctx: *mut alchemy_pci_context = core::ptr::null_mut();

static mut alchemy_pci_def_memres: resource = resource {
    start: ALCHEMY_PCI_MEMWIN_START,
    end: ALCHEMY_PCI_MEMWIN_END,
    name: b"PCI memory space\0".as_ptr() as *const i8,
    flags: IORESOURCE_MEM,
};

static mut alchemy_pci_def_iores: resource = resource {
    start: ALCHEMY_PCI_IOWIN_START,
    end: ALCHEMY_PCI_IOWIN_END,
    name: b"PCI IO space\0".as_ptr() as *const i8,
    flags: IORESOURCE_IO,
};

unsafe fn mod_wired_entry(entry: i32, entrylo0: usize, entrylo1: usize,
                          entryhi: usize, pagemask: usize) {
    let old_ctx = read_c0_entryhi() & MIPS_ENTRYHI_ASID;
    let old_pagemask = read_c0_pagemask();
    write_c0_index(entry);
    write_c0_pagemask(pagemask);
    write_c0_entryhi(entryhi);
    write_c0_entrylo0(entrylo0);
    write_c0_entrylo1(entrylo1);
    tlb_write_indexed();
    write_c0_entryhi(old_ctx);
    write_c0_pagemask(old_pagemask);
}

unsafe fn alchemy_pci_wired_entry(ctx: *mut alchemy_pci_context) {
    (*ctx).wired_entry = read_c0_wired();
    add_wired_entry(0, 0, (*(*ctx).pci_cfg_vm).addr as usize, PM_4K);
    (*ctx).last_elo0 = !0;
    (*ctx).last_elo1 = !0;
}

unsafe fn config_access(access_type: u8, bus: *mut pci_bus, dev_fn: u32,
                        where_: u8, data: *mut u32) -> i32 {
    let ctx = (*bus).sysdata as *mut alchemy_pci_context;
    let device = PCI_SLOT(dev_fn);
    let function = PCI_FUNC(dev_fn);
    let mut offset: usize;
    let mut status: usize;
    let mut cfg_base: usize;
    let mut flags: usize = 0;
    let mut entryLo0: usize;
    let mut entryLo1: usize;
    let mut error = PCIBIOS_SUCCESSFUL;
    if device > 19 { *data = 0xffff_ffff; return -1; }
    local_irq_save(&mut flags);
    let mut r = __raw_readl((*ctx).regs.add(PCI_REG_STATCMD)) & 0xffff;
    r |= PCI_STATCMD_STATUS(0x2000);
    __raw_writel(r, (*ctx).regs.add(PCI_REG_STATCMD));
    wmb();
    if ((*ctx).board_pci_idsel.unwrap()) (device, 1) == 0 {
        *data = 0xffff_ffff; local_irq_restore(flags); return -1;
    }
    if (*bus).number == 0 { cfg_base = (1usize << device) << 11; }
    else { cfg_base = 0x8000_0000 | ((*bus).number as usize << 16) | (device as usize << 11); }
    offset = ((function as usize) << 8) | ((where_ as usize) & !3);
    offset |= cfg_base & !PAGE_MASK;
    cfg_base &= PAGE_MASK;
    entryLo0 = (6usize << 26) | (cfg_base >> 6) | (2 << 3) | 7;
    entryLo1 = (6usize << 26) | (cfg_base >> 6) | (0x1000 >> 6) | (2 << 3) | 7;
    if entryLo0 != (*ctx).last_elo0 || entryLo1 != (*ctx).last_elo1 {
        mod_wired_entry((*ctx).wired_entry, entryLo0, entryLo1,
                        (*(*ctx).pci_cfg_vm).addr as usize, PM_4K);
        (*ctx).last_elo0 = entryLo0; (*ctx).last_elo1 = entryLo1;
    }
    if access_type == PCI_ACCESS_WRITE { __raw_writel(*data, (*(*ctx).pci_cfg_vm).addr.add(offset)); }
    else { *data = __raw_readl((*(*ctx).pci_cfg_vm).addr.add(offset)); }
    wmb();
    status = __raw_readl((*ctx).regs.add(PCI_REG_STATCMD));
    if status & (1usize << 29) != 0 { *data = 0xffff_ffff; error = -1; }
    else if (status >> 28) & 0xf != 0 {
        __raw_writel(status & 0xf000_ffff, (*ctx).regs.add(PCI_REG_STATCMD));
        *data = 0xffff_ffff; error = -1;
    }
    ((*ctx).board_pci_idsel.unwrap()) (device, 0);
    local_irq_restore(flags);
    error
}

unsafe fn read_config_byte(bus: *mut pci_bus, devfn: u32, where_: i32, val: *mut u8) -> i32 {
    let mut data = 0; let ret = config_access(PCI_ACCESS_READ, bus, devfn, where_ as u8, &mut data);
    if where_ & 1 != 0 { data >>= 8; } if where_ & 2 != 0 { data >>= 16; }
    *val = (data & 0xff) as u8; ret
}
unsafe fn read_config_word(bus: *mut pci_bus, devfn: u32, where_: i32, val: *mut u16) -> i32 {
    let mut data = 0; let ret = config_access(PCI_ACCESS_READ, bus, devfn, where_ as u8, &mut data);
    if where_ & 2 != 0 { data >>= 16; } *val = (data & 0xffff) as u16; ret
}
unsafe fn read_config_dword(bus: *mut pci_bus, devfn: u32, where_: i32, val: *mut u32) -> i32 { config_access(PCI_ACCESS_READ, bus, devfn, where_ as u8, val) }
unsafe fn write_config_byte(bus: *mut pci_bus, devfn: u32, where_: i32, val: u8) -> i32 {
    let mut data = 0; if config_access(PCI_ACCESS_READ, bus, devfn, where_ as u8, &mut data) != 0 { return -1; }
    data = (data & !(0xff << ((where_ & 3) << 3))) | ((val as u32) << ((where_ & 3) << 3));
    if config_access(PCI_ACCESS_WRITE, bus, devfn, where_ as u8, &mut data) != 0 { return -1; } PCIBIOS_SUCCESSFUL
}
unsafe fn write_config_word(bus: *mut pci_bus, devfn: u32, where_: i32, val: u16) -> i32 {
    let mut data = 0; if config_access(PCI_ACCESS_READ, bus, devfn, where_ as u8, &mut data) != 0 { return -1; }
    data = (data & !(0xffff << ((where_ & 3) << 3))) | ((val as u32) << ((where_ & 3) << 3));
    if config_access(PCI_ACCESS_WRITE, bus, devfn, where_ as u8, &mut data) != 0 { return -1; } PCIBIOS_SUCCESSFUL
}
unsafe fn write_config_dword(bus: *mut pci_bus, devfn: u32, where_: i32, val: u32) -> i32 { config_access(PCI_ACCESS_WRITE, bus, devfn, where_ as u8, &mut (val as u32)) }

unsafe fn alchemy_pci_read(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: *mut u32) -> i32 {
    match size { 1 => { let mut v=0; let r=read_config_byte(bus,devfn,where_,&mut v); *val=v as u32; r }, 2 => { let mut v=0; let r=read_config_word(bus,devfn,where_,&mut v); *val=v as u32; r }, _ => read_config_dword(bus,devfn,where_,val) }
}
unsafe fn alchemy_pci_write(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: u32) -> i32 {
    match size { 1 => write_config_byte(bus,devfn,where_,val as u8), 2 => write_config_word(bus,devfn,where_,val as u16), _ => write_config_dword(bus,devfn,where_,val) }
}

static mut alchemy_pci_ops: pci_ops = pci_ops { read: alchemy_pci_read, write: alchemy_pci_write };

unsafe fn alchemy_pci_def_idsel(_devsel: u32, _assert: i32) -> i32 { 1 }

unsafe fn alchemy_pci_suspend(_data: *mut core::ffi::c_void) -> i32 {
    let ctx = __alchemy_pci_ctx; if ctx.is_null() { return 0; }
    (*ctx).pm[0]=__raw_readl((*ctx).regs.add(PCI_REG_CMEM)); (*ctx).pm[1]=__raw_readl((*ctx).regs.add(PCI_REG_CONFIG)) & 0x0009ffff;
    (*ctx).pm[2]=__raw_readl((*ctx).regs.add(PCI_REG_B2BMASK_CCH)); (*ctx).pm[3]=__raw_readl((*ctx).regs.add(PCI_REG_B2BBASE0_VID));
    (*ctx).pm[4]=__raw_readl((*ctx).regs.add(PCI_REG_B2BBASE1_SID)); (*ctx).pm[5]=__raw_readl((*ctx).regs.add(PCI_REG_MWMASK_DEV));
    (*ctx).pm[6]=__raw_readl((*ctx).regs.add(PCI_REG_MWBASE_REV_CCL)); (*ctx).pm[7]=__raw_readl((*ctx).regs.add(PCI_REG_ID));
    (*ctx).pm[8]=__raw_readl((*ctx).regs.add(PCI_REG_CLASSREV)); (*ctx).pm[9]=__raw_readl((*ctx).regs.add(PCI_REG_PARAM));
    (*ctx).pm[10]=__raw_readl((*ctx).regs.add(PCI_REG_MBAR)); (*ctx).pm[11]=__raw_readl((*ctx).regs.add(PCI_REG_TIMEOUT)); 0
}

unsafe fn alchemy_pci_resume(_data: *mut core::ffi::c_void) { let ctx=__alchemy_pci_ctx; if ctx.is_null(){return;}
    let regs=(*ctx).regs; for (i, reg) in [PCI_REG_CMEM,PCI_REG_B2BMASK_CCH,PCI_REG_B2BBASE0_VID,PCI_REG_B2BBASE1_SID,PCI_REG_MWMASK_DEV,PCI_REG_MWBASE_REV_CCL,PCI_REG_ID,PCI_REG_CLASSREV,PCI_REG_PARAM,PCI_REG_MBAR,PCI_REG_TIMEOUT].iter().enumerate(){__raw_writel((*ctx).pm[if i==0{0}else{i+1}],regs.add(*reg));} wmb(); __raw_writel((*ctx).pm[1],regs.add(PCI_REG_CONFIG)); wmb(); (*ctx).wired_entry=8191; alchemy_pci_wired_entry(ctx); }

unsafe fn alchemy_pci_probe(_pdev: *mut platform_device) -> i32 { todo!("translate external platform/kernel integration") }
static mut alchemy_pcictl_driver: platform_driver = platform_driver { probe: alchemy_pci_probe };
unsafe fn alchemy_pci_init() -> i32 { match alchemy_get_cputype() { ALCHEMY_CPU_AU1500 | ALCHEMY_CPU_AU1550 => platform_driver_register(&mut alchemy_pcictl_driver), _ => 0 } }

unsafe fn pcibios_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 { let ctx=(*dev).sysdata as *mut alchemy_pci_context; if !ctx.is_null(){if let Some(f)=(*ctx).board_map_irq{return f(dev,slot,pin);}} -1 }
unsafe fn pcibios_plat_dev_init(_dev: *mut pci_dev) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
