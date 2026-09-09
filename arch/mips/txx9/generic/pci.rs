/*
 * linux/arch/mips/txx9/pci.c
 *
 * Based on linux/arch/mips/txx9/rbtx4927/setup.c,
 *          linux/arch/mips/txx9/rbtx4938/setup.c,
 *          and RBTX49xx patch from CELF patch archive.
 *
 * Copyright 2001-2005 MontaVista Software Inc.
 * Copyright (C) 1996, 97, 2001, 04  Ralf Baechle (ralf@linux-mips.org)
 * (C) Copyright TOSHIBA CORPORATION 2000-2001, 2004-2007
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// External Linux/MIPS declarations supplied by other translation units.

unsafe fn early_read_config_word(
    hose: *mut pci_controller,
    top_bus: c_int,
    bus: c_int,
    devfn: c_int,
    offset: c_int,
    value: *mut u16,
) -> c_int {
    let mut fake_bus: pci_bus = core::mem::zeroed();
    (*fake_bus).number = bus;
    (*fake_bus).sysdata = hose as *mut c_void;
    (*fake_bus).ops = (*hose).pci_ops;

    if bus != top_bus {
        // Fake a parent bus structure.
        (*fake_bus).parent = &mut fake_bus;
    } else {
        (*fake_bus).parent = core::ptr::null_mut();
    }

    pci_bus_read_config_word(&mut fake_bus, devfn, offset, value)
}

pub unsafe fn txx9_pci66_check(
    hose: *mut pci_controller,
    top_bus: c_int,
    current_bus: c_int,
) -> c_int {
    let mut pci_devfn: u32;
    let mut vid: u16 = 0;
    let mut cap66: c_int = -1;
    let mut stat: u16 = 0;
    let mut ret: c_int;

    // It seems SLC90E66 needs some time after PCI reset...
    mdelay(80);

    pr_info!("PCI: Checking 66MHz capabilities...\n");

    pci_devfn = 0;
    while pci_devfn < 0xff {
        if PCI_FUNC(pci_devfn) != 0 {
            pci_devfn += 1;
            continue;
        }
        ret = early_read_config_word(hose, top_bus, current_bus, pci_devfn as c_int,
                                      PCI_VENDOR_ID, &mut vid);
        if ret != PCIBIOS_SUCCESSFUL || vid == 0xffff {
            pci_devfn += 1;
            continue;
        }

        // check 66MHz capability
        if cap66 < 0 {
            cap66 = 1;
        }
        if cap66 != 0 {
            early_read_config_word(hose, top_bus, current_bus, pci_devfn as c_int,
                                   PCI_STATUS, &mut stat);
            if stat & PCI_STATUS_66MHZ == 0 {
                pr_debug!("PCI: {:02x}:{:02x} not 66MHz capable.\n", current_bus, pci_devfn);
                cap66 = 0;
                break;
            }
        }
        pci_devfn += 1;
    }
    (cap66 > 0) as c_int
}

static mut primary_pci_mem_res: [resource; 2] = [
    resource { name: "PCI MEM\0".as_ptr() as *mut c_char, ..unsafe { core::mem::zeroed() } },
    resource { name: "PCI MMIO\0".as_ptr() as *mut c_char, ..unsafe { core::mem::zeroed() } },
];
static mut primary_pci_io_res: resource = resource {
    name: "PCI IO\0".as_ptr() as *mut c_char,
    ..unsafe { core::mem::zeroed() }
};
pub static mut txx9_primary_pcic: pci_controller = pci_controller {
    mem_resource: unsafe { primary_pci_mem_res.as_mut_ptr() },
    io_resource: unsafe { &mut primary_pci_io_res },
    ..unsafe { core::mem::zeroed() }
};

#[cfg(target_pointer_width = "64")]
pub static mut txx9_pci_mem_high: c_int = 1;
#[cfg(not(target_pointer_width = "64"))]
pub static mut txx9_pci_mem_high: c_int = 0;

pub unsafe fn txx9_alloc_pci_controller(
    mut pcic: *mut pci_controller,
    mem_base: c_ulong,
    mut mem_size: c_ulong,
    io_base: c_ulong,
    mut io_size: c_ulong,
) -> *mut pci_controller {
    let mut new: *mut pcic_alloc = core::ptr::null_mut();
    let min_size: c_ulong = 0x10000;

    if pcic.is_null() {
        new = kzalloc(core::mem::size_of::<pcic_alloc>(), GFP_KERNEL) as *mut pcic_alloc;
        if new.is_null() {
            return core::ptr::null_mut();
        }
        (*new).r_mem[0].name = "PCI mem\0".as_ptr() as *mut c_char;
        (*new).r_mem[1].name = "PCI mmio\0".as_ptr() as *mut c_char;
        (*new).r_io.name = "PCI io\0".as_ptr() as *mut c_char;
        (*new).c.mem_resource = (*new).r_mem.as_mut_ptr();
        (*new).c.io_resource = &mut (*new).r_io;
        pcic = &mut (*new).c;
    } else {
        BUG_ON!(pcic != &mut txx9_primary_pcic);
    }
    (*(*pcic).io_resource).flags = IORESOURCE_IO;

    if mem_base != 0 {
        (*(*pcic).mem_resource).start = mem_base;
        (*(*pcic).mem_resource).end = mem_base + mem_size - 1;
        if request_resource(&iomem_resource, (*pcic).mem_resource) != 0 {
            goto_free_and_exit!(new);
        }
    } else {
        let mut min: c_ulong = 0;
        let mut max: c_ulong = 0x20000000;
        if mem_size == 0 {
            mem_size = if txx9_pci_mem_high != 0 { 0x20000000 } else { 0x08000000 };
        }
        if txx9_pci_mem_high != 0 {
            min = 0x20000000;
            max = 0xe0000000;
        }
        while mem_size >= min_size {
            if allocate_resource(&iomem_resource, (*pcic).mem_resource, mem_size, min, max,
                                 mem_size, None, None) == 0 { break; }
            mem_size /= 2;
        }
        if mem_size < min_size { goto_free_and_exit!(new); }
    }

    (*(*pcic).mem_resource.add(1)).flags = IORESOURCE_MEM | IORESOURCE_BUSY;
    if io_base != 0 {
        (*(*pcic).mem_resource.add(1)).start = io_base;
        (*(*pcic).mem_resource.add(1)).end = io_base + io_size - 1;
        if request_resource(&iomem_resource, (*pcic).mem_resource.add(1)) != 0 {
            release_and_exit!(pcic, new);
        }
    } else {
        if io_size == 0 { io_size = 0x01000000; }
        while io_size >= min_size {
            if allocate_resource(&iomem_resource, (*pcic).mem_resource.add(1), io_size,
                                 0, 0x20000000, io_size, None, None) == 0 { break; }
            io_size /= 2;
        }
        if io_size < min_size { release_and_exit!(pcic, new); }
    }

    (*(*pcic).mem_resource).flags = IORESOURCE_MEM;
    if pcic == &mut txx9_primary_pcic && mips_io_port_base == (!0 as c_ulong) {
        set_io_port_base(IO_BASE + (*(*pcic).mem_resource.add(1)).start);
        (*(*pcic).io_resource).start = 0;
        (*pcic).io_offset = 0;
        (*pcic).io_map_base = IO_BASE + (*(*pcic).mem_resource.add(1)).start;
    } else {
        (*(*pcic).io_resource).start = io_base - (mips_io_port_base - IO_BASE);
        (*pcic).io_offset = io_base - (mips_io_port_base - IO_BASE);
        (*pcic).io_map_base = mips_io_port_base;
    }
    (*(*pcic).io_resource).end = (*(*pcic).io_resource).start + io_size - 1;
    (*pcic).mem_offset = 0;
    pr_info!("PCI: IO %pR MEM %pR\n", (*pcic).mem_resource.add(1), (*pcic).mem_resource);
    release_resource((*pcic).mem_resource);
    pcic
}

unsafe fn txx9_arch_pci_init() -> c_int {
    PCIBIOS_MIN_IO = 0x8000;
    0
}
arch_initcall!(txx9_arch_pci_init);

pub static mut txx9_pci_option: c_int =
    if cfg!(CONFIG_PICMG_PCI_BACKPLANE_DEFAULT) { TXX9_PCI_OPT_PICMG } else { 0 }
    | TXX9_PCI_OPT_CLK_AUTO;
pub static mut txx9_pci_err_action: txx9_pci_err_action = TXX9_PCI_ERR_REPORT;

#[cfg(CONFIG_TOSHIBA_FPCIB0)]
unsafe fn i8259_interrupt(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    let isairq = i8259_irq();
    if isairq <= I8259A_IRQ_BASE { return IRQ_NONE; }
    generic_handle_irq(isairq);
    IRQ_HANDLED
}

#[cfg(CONFIG_TOSHIBA_FPCIB0)]
unsafe fn txx9_i8259_irq_setup(irq: c_int) -> c_int {
    init_i8259_irqs();
    let err = request_irq(irq, i8259_interrupt, IRQF_SHARED,
                          "cascade(i8259)\0".as_ptr() as *const c_char,
                          irq as isize as *mut c_void);
    if err == 0 { pr_info!("PCI-ISA bridge PIC (irq {})\n", irq); }
    err
}

#[cfg(CONFIG_TOSHIBA_FPCIB0)]
unsafe fn quirk_slc90e66_bridge(dev: *mut pci_dev) {
    let irq = pcibios_map_irq(dev, PCI_SLOT((*dev).devfn) as u8, 1);
    if irq == 0 { return; }
    txx9_i8259_irq_setup(irq);
    let mut reg_64 = 0u8; let mut reg_b0 = 0u32; let mut reg_e1 = 0u8;
    pci_read_config_byte(dev, 0x64, &mut reg_64);
    pci_read_config_dword(dev, 0xb0, &mut reg_b0);
    pci_read_config_byte(dev, 0xe1, &mut reg_e1);
    reg_64 = 0xd0; reg_b0 |= 0x00010000; reg_e1 = (reg_e1 & 0xf0) | 0x0d;
    pci_write_config_byte(dev, 0x64, reg_64); pci_write_config_dword(dev, 0xb0, reg_b0);
    pci_write_config_byte(dev, 0xe1, reg_e1);
    smsc_fdc37m81x_init(0x3f0); smsc_fdc37m81x_config_beg();
    smsc_fdc37m81x_config_set(SMSC_FDC37M81X_DNUM, SMSC_FDC37M81X_KBD);
    smsc_fdc37m81x_config_set(SMSC_FDC37M81X_INT, 1);
    smsc_fdc37m81x_config_set(SMSC_FDC37M81X_INT2, 12);
    smsc_fdc37m81x_config_set(SMSC_FDC37M81X_ACTIVE, 1); smsc_fdc37m81x_config_end();
}

#[cfg(CONFIG_TOSHIBA_FPCIB0)]
unsafe fn quirk_slc90e66_ide(dev: *mut pci_dev) {
    let mut dat = 0u8;
    let regs = [0x41, 0x43];
    pci_write_config_byte(dev, PCI_INTERRUPT_LINE, 14);
    pci_read_config_byte(dev, PCI_INTERRUPT_LINE, &mut dat);
    pr_info!("PCI: %s: IRQ {:02x}", pci_name(dev), dat);
    for (i, reg) in regs.iter().enumerate() {
        pci_read_config_byte(dev, *reg, &mut dat); pci_write_config_byte(dev, *reg, dat | 0x80);
        pci_read_config_byte(dev, *reg, &mut dat); pr_cont!(" IDETIM{} {:02x}", i, dat);
    }
    pci_read_config_byte(dev, 0x5c, &mut dat);
    /*
     * !!! DO NOT REMOVE THIS COMMENT IT IS REQUIRED BY SMSC !!!
     *
     * This line of code is intended to provide the user with a work
     * around solution to the anomalies cited in SMSC's anomaly sheet
     * entitled, "SLC90E66 Functional Rev.J_0.1 Anomalies"".
     *
     * !!! DO NOT REMOVE THIS COMMENT IT IS REQUIRED BY SMSC !!!
     */
    dat |= 0x01; pci_write_config_byte(dev, 0x5c, dat); pci_read_config_byte(dev, 0x5c, &mut dat);
    pr_cont!(" REG5C {:02x}\n", dat);
}

unsafe fn tc35815_fixup(dev: *mut pci_dev) {
    // This device may have PM registers but not they are not supported.
    if (*dev).pm_cap != 0 { dev_info!(&(*dev).dev, "PM disabled\n"); (*dev).pm_cap = 0; }
}

unsafe fn final_fixup(dev: *mut pci_dev) {
    let mut timeout: c_ulong;
    let mut bist = 0u8;
    let ret = pci_read_config_byte(dev, PCI_BIST, &mut bist);
    if ret != PCIBIOS_SUCCESSFUL || bist & PCI_BIST_CAPABLE == 0 { return; }
    pci_set_power_state(dev, PCI_D0); pr_info!("PCI: %s BIST...", pci_name(dev));
    pci_write_config_byte(dev, PCI_BIST, PCI_BIST_START); timeout = jiffies + HZ * 2;
    loop {
        pci_read_config_byte(dev, PCI_BIST, &mut bist);
        if time_after(jiffies, timeout) || bist & PCI_BIST_START == 0 { break; }
    }
    if bist & (PCI_BIST_CODE_MASK | PCI_BIST_START) != 0 { pr_cont!("failed. (0x{:x})\n", bist); }
    else { pr_cont!("OK.\n"); }
}

#[cfg(CONFIG_TOSHIBA_FPCIB0)]
declare_pci_fixup_final!(PCI_VENDOR_ID_EFAR, 0x9460, quirk_slc90e66_bridge);
#[cfg(CONFIG_TOSHIBA_FPCIB0)]
declare_pci_fixup_final!(PCI_VENDOR_ID_EFAR, PCI_DEVICE_ID_EFAR_SLC90E66_1, quirk_slc90e66_ide);
#[cfg(CONFIG_TOSHIBA_FPCIB0)]
declare_pci_fixup_resume!(PCI_VENDOR_ID_EFAR, PCI_DEVICE_ID_EFAR_SLC90E66_1, quirk_slc90e66_ide);
declare_pci_fixup_final!(PCI_VENDOR_ID_TOSHIBA_2, PCI_DEVICE_ID_TOSHIBA_TC35815_NWU, tc35815_fixup);
declare_pci_fixup_final!(PCI_VENDOR_ID_TOSHIBA_2, PCI_DEVICE_ID_TOSHIBA_TC35815_TX4939, tc35815_fixup);
declare_pci_fixup_final!(PCI_ANY_ID, PCI_ANY_ID, final_fixup);
declare_pci_fixup_resume!(PCI_ANY_ID, PCI_ANY_ID, final_fixup);

pub unsafe fn pcibios_plat_dev_init(_dev: *mut pci_dev) -> c_int { 0 }

static mut txx9_pci_map_irq: Option<unsafe fn(*const pci_dev, u8, u8) -> c_int> = None;
pub unsafe fn pcibios_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> c_int {
    txx9_pci_map_irq.unwrap()(dev, slot, pin)
}

static mut txx9_board_pcibios_setup: Option<unsafe fn(*mut c_char) -> *mut c_char> = None;
pub unsafe fn txx9_pcibios_setup(str_: *mut c_char) -> *mut c_char {
    if let Some(setup) = txx9_board_pcibios_setup {
        if setup(str_).is_null() { return core::ptr::null_mut(); }
    }
    if strcmp(str_, c"picmg".as_ptr()) == 0 {
        txx9_pci_option |= TXX9_PCI_OPT_PICMG; return core::ptr::null_mut();
    } else if strcmp(str_, c"nopicmg".as_ptr()) == 0 {
        txx9_pci_option &= !TXX9_PCI_OPT_PICMG; return core::ptr::null_mut();
    } else if strncmp(str_, c"clk=".as_ptr(), 4) == 0 {
        let val = str_.add(4); txx9_pci_option &= !TXX9_PCI_OPT_CLK_MASK;
        if strcmp(val, c"33".as_ptr()) == 0 { txx9_pci_option |= TXX9_PCI_OPT_CLK_33; }
        else if strcmp(val, c"66".as_ptr()) == 0 { txx9_pci_option |= TXX9_PCI_OPT_CLK_66; }
        else { txx9_pci_option |= TXX9_PCI_OPT_CLK_AUTO; }
        return core::ptr::null_mut();
    } else if strncmp(str_, c"err=".as_ptr(), 4) == 0 {
        let val = str_.add(4);
        if strcmp(val, c"panic".as_ptr()) == 0 { txx9_pci_err_action = TXX9_PCI_ERR_PANIC; }
        else if strcmp(val, c"ignore".as_ptr()) == 0 { txx9_pci_err_action = TXX9_PCI_ERR_IGNORE; }
        return core::ptr::null_mut();
    }
    txx9_pci_map_irq = Some((*txx9_board_vec).pci_map_irq);
    str_
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
