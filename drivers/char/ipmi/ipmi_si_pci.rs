// SPDX-License-Identifier: GPL-2.0+
/*
 * ipmi_si_pci.c
 *
 * Handling for IPMI devices on the PCI bus.
 */

// pr_fmt(fmt) = "ipmi_pci: " fmt

// Kernel and ipmi_si.h declarations are supplied by external dependencies.

static mut PCI_REGISTERED: bool = false;
static mut SI_TRYPCI: bool = true;

const PCI_DEVICE_ID_HP_MMC: u16 = 0x121A;

unsafe fn ipmi_pci_probe_regspacing(io: *mut si_sm_io) -> i32 {
    let mut status: u8;
    let mut regspacing: i32;

    if (*(*io).si_info).type_ != SI_KCS {
        return DEFAULT_REGSPACING;
    }

    (*io).regsize = DEFAULT_REGSIZE;
    (*io).regshift = 0;

    /* detect 1, 4, 16byte spacing */
    regspacing = DEFAULT_REGSPACING;
    while regspacing <= 16 {
        (*io).regspacing = regspacing;
        if ((*io).io_setup.unwrap())(io) != 0 {
            dev_err((*io).dev, "Could not setup I/O space\n");
            return DEFAULT_REGSPACING;
        }
        /* write invalid cmd */
        ((*io).outputb.unwrap())(io, 1, 0x10);
        /* read status back */
        status = ((*io).inputb.unwrap())(io, 1);
        ((*io).io_cleanup.unwrap())(io);
        if status != 0 {
            return regspacing;
        }
        regspacing *= 4;
    }

    DEFAULT_REGSPACING
}

// static struct pci_device_id ipmi_pci_blacklist[]
static mut IPMI_PCI_BLACKLIST: [pci_device_id; 2] = [
    pci_device_id::vdevice(REALTEK, 0x816c),
    pci_device_id::zero(),
];

unsafe fn ipmi_pci_probe(
    pdev: *mut pci_dev,
    _ent: *const pci_device_id,
) -> i32 {
    let rv: i32;
    let mut io: si_sm_io = core::mem::zeroed();

    if pci_match_id(IPMI_PCI_BLACKLIST.as_ptr(), pdev) != 0 {
        return -ENODEV;
    }

    io.addr_source = SI_PCI;
    dev_info(&mut (*pdev).dev, "probing via PCI");

    match (*pdev).class {
        PCI_CLASS_SERIAL_IPMI_SMIC => {
            io.si_info = &mut ipmi_smic_si_info;
        }
        PCI_CLASS_SERIAL_IPMI_KCS => {
            io.si_info = &mut ipmi_kcs_si_info;
        }
        PCI_CLASS_SERIAL_IPMI_BT => {
            io.si_info = &mut ipmi_bt_si_info;
        }
        _ => {
            dev_info(&mut (*pdev).dev, "Unknown IPMI class: %x\n", (*pdev).class);
            return -ENOMEM;
        }
    }

    rv = pcim_enable_device(pdev);
    if rv != 0 {
        dev_err(&mut (*pdev).dev, "couldn't enable PCI device\n");
        return rv;
    }

    if pci_resource_flags(pdev, 0) & IORESOURCE_IO != 0 {
        // Preserve the build-time CONFIG_HAS_IOPORT condition from the C source.
        if !IS_ENABLED_CONFIG_HAS_IOPORT {
            return -ENXIO;
        }
        io.addr_space = IPMI_IO_ADDR_SPACE;
        io.io_setup = Some(ipmi_si_port_setup);
    } else {
        io.addr_space = IPMI_MEM_ADDR_SPACE;
        io.io_setup = Some(ipmi_si_mem_setup);
    }
    io.addr_data = pci_resource_start(pdev, 0);
    io.dev = &mut (*pdev).dev;

    io.regspacing = ipmi_pci_probe_regspacing(&mut io);
    io.regsize = DEFAULT_REGSIZE;
    io.regshift = 0;

    io.irq = (*pdev).irq;
    if io.irq != 0 {
        io.irq_setup = Some(ipmi_std_irq_setup);
    }

    dev_info(
        &mut (*pdev).dev,
        "%pR regsize %u spacing %u irq %d\n",
        &(*pdev).resource[0],
        io.regsize,
        io.regspacing,
        io.irq,
    );

    ipmi_si_add_smi(&mut io)
}

unsafe fn ipmi_pci_remove(pdev: *mut pci_dev) {
    ipmi_si_remove_by_dev(&mut (*pdev).dev);
}

static IPMI_PCI_DEVICES: [pci_device_id; 5] = [
    pci_device_id::vdevice(HP, PCI_DEVICE_ID_HP_MMC),
    pci_device_id::device_class(PCI_CLASS_SERIAL_IPMI_SMIC, !0),
    pci_device_id::device_class(PCI_CLASS_SERIAL_IPMI_KCS, !0),
    pci_device_id::device_class(PCI_CLASS_SERIAL_IPMI_BT, !0),
    pci_device_id::zero(),
];

static mut IPMI_PCI_DRIVER: pci_driver = pci_driver {
    name: SI_DEVICE_NAME,
    id_table: IPMI_PCI_DEVICES.as_ptr(),
    probe: Some(ipmi_pci_probe),
    remove: Some(ipmi_pci_remove),
};

pub unsafe fn ipmi_si_pci_init() {
    if SI_TRYPCI {
        let rv = pci_register_driver(&mut IPMI_PCI_DRIVER);
        if rv != 0 {
            pr_err!("Unable to register PCI driver: {}\n", rv);
        } else {
            PCI_REGISTERED = true;
        }
    }
}

pub unsafe fn ipmi_si_pci_shutdown() {
    if PCI_REGISTERED {
        pci_unregister_driver(&mut IPMI_PCI_DRIVER);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
