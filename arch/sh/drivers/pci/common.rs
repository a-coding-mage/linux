// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux PCI, interrupt, timer, and kernel APIs.

/*
 * These functions are used early on before PCI scanning is done
 * and all of the pci_dev and pci_bus structures have been created.
 */
unsafe fn fake_pci_dev(
    hose: *mut pci_channel,
    top_bus: ::core::ffi::c_int,
    busnr: ::core::ffi::c_int,
    devfn: ::core::ffi::c_int,
) -> *mut pci_dev {
    static mut DEV: pci_dev = pci_dev::ZERO;
    static mut BUS: pci_bus = pci_bus::ZERO;

    DEV.bus = &raw mut BUS;
    DEV.sysdata = hose as *mut _;
    DEV.devfn = devfn;
    BUS.number = busnr;
    BUS.sysdata = hose as *mut _;
    BUS.ops = (*hose).pci_ops;

    if busnr != top_bus {
        /* Fake a parent bus structure. */
        BUS.parent = &raw mut BUS;
    } else {
        BUS.parent = ::core::ptr::null_mut();
    }

    &raw mut DEV
}

pub unsafe fn early_read_config_byte(
    hose: *mut pci_channel, top_bus: ::core::ffi::c_int,
    bus: ::core::ffi::c_int, devfn: ::core::ffi::c_int,
    offset: ::core::ffi::c_int, value: *mut u8,
) -> ::core::ffi::c_int {
    pci_read_config_byte(fake_pci_dev(hose, top_bus, bus, devfn), offset, value)
}

pub unsafe fn early_read_config_word(
    hose: *mut pci_channel, top_bus: ::core::ffi::c_int,
    bus: ::core::ffi::c_int, devfn: ::core::ffi::c_int,
    offset: ::core::ffi::c_int, value: *mut u16,
) -> ::core::ffi::c_int {
    pci_read_config_word(fake_pci_dev(hose, top_bus, bus, devfn), offset, value)
}

pub unsafe fn early_read_config_dword(
    hose: *mut pci_channel, top_bus: ::core::ffi::c_int,
    bus: ::core::ffi::c_int, devfn: ::core::ffi::c_int,
    offset: ::core::ffi::c_int, value: *mut u32,
) -> ::core::ffi::c_int {
    pci_read_config_dword(fake_pci_dev(hose, top_bus, bus, devfn), offset, value)
}

pub unsafe fn early_write_config_byte(
    hose: *mut pci_channel, top_bus: ::core::ffi::c_int,
    bus: ::core::ffi::c_int, devfn: ::core::ffi::c_int,
    offset: ::core::ffi::c_int, value: u8,
) -> ::core::ffi::c_int {
    pci_write_config_byte(fake_pci_dev(hose, top_bus, bus, devfn), offset, value)
}

pub unsafe fn early_write_config_word(
    hose: *mut pci_channel, top_bus: ::core::ffi::c_int,
    bus: ::core::ffi::c_int, devfn: ::core::ffi::c_int,
    offset: ::core::ffi::c_int, value: u16,
) -> ::core::ffi::c_int {
    pci_write_config_word(fake_pci_dev(hose, top_bus, bus, devfn), offset, value)
}

pub unsafe fn early_write_config_dword(
    hose: *mut pci_channel, top_bus: ::core::ffi::c_int,
    bus: ::core::ffi::c_int, devfn: ::core::ffi::c_int,
    offset: ::core::ffi::c_int, value: u32,
) -> ::core::ffi::c_int {
    pci_write_config_dword(fake_pci_dev(hose, top_bus, bus, devfn), offset, value)
}

pub unsafe fn pci_is_66mhz_capable(
    hose: *mut pci_channel,
    top_bus: ::core::ffi::c_int,
    current_bus: ::core::ffi::c_int,
) -> bool {
    let mut pci_devfn: u32 = 0;
    let mut vid: u16 = 0;
    let mut cap66: ::core::ffi::c_int = -1;
    let mut stat: u16 = 0;
    let mut ret: ::core::ffi::c_int;

    pr_info!("PCI: Checking 66MHz capabilities...\n");

    while pci_devfn < 0xff {
        if PCI_FUNC(pci_devfn) != 0 {
            pci_devfn += 1;
            continue;
        }
        ret = early_read_config_word(hose, top_bus, current_bus,
            pci_devfn as _, PCI_VENDOR_ID, &mut vid);
        if ret != PCIBIOS_SUCCESSFUL || PCI_POSSIBLE_ERROR(vid) {
            pci_devfn += 1;
            continue;
        }

        /* check 66MHz capability */
        if cap66 < 0 { cap66 = 1; }
        if cap66 != 0 {
            early_read_config_word(hose, top_bus, current_bus,
                pci_devfn as _, PCI_STATUS, &mut stat);
            if stat & PCI_STATUS_66MHZ == 0 {
                printk!(KERN_DEBUG, "PCI: {:02x}:{:02x} not 66MHz capable.\n",
                    current_bus, pci_devfn);
                cap66 = 0;
                break;
            }
        }
        pci_devfn += 1;
    }

    cap66 > 0
}

unsafe fn pcibios_enable_err(t: *mut timer_list) {
    let hose: *mut pci_channel = timer_container_of!(hose, t, err_timer);
    timer_delete(&mut (*hose).err_timer);
    printk!(KERN_DEBUG, "PCI: re-enabling error IRQ.\n");
    enable_irq((*hose).err_irq);
}

unsafe fn pcibios_enable_serr(t: *mut timer_list) {
    let hose: *mut pci_channel = timer_container_of!(hose, t, serr_timer);
    timer_delete(&mut (*hose).serr_timer);
    printk!(KERN_DEBUG, "PCI: re-enabling system error IRQ.\n");
    enable_irq((*hose).serr_irq);
}

pub unsafe fn pcibios_enable_timers(hose: *mut pci_channel) {
    if (*hose).err_irq != 0 {
        timer_setup!(&mut (*hose).err_timer, pcibios_enable_err, 0);
    }
    if (*hose).serr_irq != 0 {
        timer_setup!(&mut (*hose).serr_timer, pcibios_enable_serr, 0);
    }
}

/* A simple handler for the regular PCI status errors, called from IRQ context. */
pub unsafe fn pcibios_handle_status_errors(
    addr: ::core::ffi::c_ulong,
    status: ::core::ffi::c_uint,
    hose: *mut pci_channel,
) -> ::core::ffi::c_uint {
    let mut cmd: ::core::ffi::c_uint = 0;
    if status & PCI_STATUS_REC_MASTER_ABORT != 0 {
        printk!(KERN_DEBUG, "PCI: master abort, pc=0x{:08x}\n", addr);
        cmd |= PCI_STATUS_REC_MASTER_ABORT;
    }
    if status & PCI_STATUS_REC_TARGET_ABORT != 0 {
        printk!(KERN_DEBUG, "PCI: target abort: ");
        pcibios_report_status!(PCI_STATUS_REC_TARGET_ABORT | PCI_STATUS_SIG_TARGET_ABORT |
            PCI_STATUS_REC_MASTER_ABORT, 1);
        pr_cont!("\n");
        cmd |= PCI_STATUS_REC_TARGET_ABORT;
    }
    if status & (PCI_STATUS_PARITY | PCI_STATUS_DETECTED_PARITY) != 0 {
        printk!(KERN_DEBUG, "PCI: parity error detected: ");
        pcibios_report_status!(PCI_STATUS_PARITY | PCI_STATUS_DETECTED_PARITY, 1);
        pr_cont!("\n");
        cmd |= PCI_STATUS_PARITY | PCI_STATUS_DETECTED_PARITY;
        if (*hose).err_irq != 0 {
            disable_irq_nosync((*hose).err_irq);
            (*hose).err_timer.expires = jiffies + HZ;
            add_timer(&mut (*hose).err_timer);
        }
    }
    cmd
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
