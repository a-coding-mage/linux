// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/ni_labpc_pci.c
 * Driver for National Instruments Lab-PC PCI-1200
 * Copyright (C) 2001, 2002, 2003 Frank Mori Hess <fmhess@users.sourceforge.net>
 */

/*
 * Driver: ni_labpc_pci
 * Description: National Instruments Lab-PC PCI-1200
 * Devices: [National Instruments] PCI-1200 (ni_pci-1200)
 * Author: Frank Mori Hess <fmhess@users.sourceforge.net>
 * Status: works
 *
 * This is the PCI-specific support split off from the ni_labpc driver.
 *
 * Configuration Options: not applicable, uses PCI auto config
 *
 * NI manuals:
 * 340914a (pci-1200)
 */

// Linux kernel and "ni_labpc.h" declarations are supplied by the surrounding build.

#[repr(usize)]
enum LabpcPciBoardid {
    BoardNiPci1200,
}

static LABPC_PCI_BOARDS: [LabpcBoardinfo; 1] = [LabpcBoardinfo {
    name: "ni_pci-1200",
    ai_speed: 10000,
    ai_scan_up: 1,
    has_ao: 1,
    is_labpc1200: 1,
}];

/* ripped from mite.h and mite_setup2() to avoid mite dependency */
const MITE_IODWBSR: usize = 0xc0; /* IO Device Window Base Size Register */
const WENAB: u32 = 1 << 7; /* window enable */

unsafe fn labpc_pci_mite_init(pcidev: *mut PciDev) -> i32 {
    let mite_base: *mut core::ffi::c_void;
    let main_phys_addr: u32;

    /* ioremap the MITE registers (BAR 0) temporarily */
    mite_base = pci_ioremap_bar(pcidev, 0);
    if mite_base.is_null() {
        return -12; // -ENOMEM
    }

    /* set data window to main registers (BAR 1) */
    main_phys_addr = pci_resource_start(pcidev, 1);
    writel(main_phys_addr | WENAB, (mite_base as *mut u8).add(MITE_IODWBSR) as *mut u32);

    /* finished with MITE registers */
    iounmap(mite_base);
    0
}

unsafe fn labpc_pci_auto_attach(dev: *mut ComediDevice, context: usize) -> i32 {
    let pcidev: *mut PciDev = comedi_to_pci_dev(dev);
    let mut board: *const LabpcBoardinfo = core::ptr::null();
    let ret: i32;

    if context < LABPC_PCI_BOARDS.len() {
        board = &LABPC_PCI_BOARDS[context];
    }
    if board.is_null() {
        return -19; // -ENODEV
    }
    (*dev).board_ptr = board as *mut core::ffi::c_void;
    (*dev).board_name = (*board).name.as_ptr() as *const i8;

    ret = comedi_pci_enable(dev);
    if ret != 0 {
        return ret;
    }

    ret = labpc_pci_mite_init(pcidev);
    if ret != 0 {
        return ret;
    }

    (*dev).mmio = pci_ioremap_bar(pcidev, 1);
    if (*dev).mmio.is_null() {
        return -12; // -ENOMEM
    }

    labpc_common_attach(dev, (*pcidev).irq, IRQF_SHARED)
}

unsafe fn labpc_pci_detach(dev: *mut ComediDevice) {
    labpc_common_detach(dev);
    comedi_pci_detach(dev);
}

static mut LABPC_PCI_COMEDI_DRIVER: ComediDriver = ComediDriver {
    driver_name: "labpc_pci",
    module: THIS_MODULE,
    auto_attach: Some(labpc_pci_auto_attach),
    detach: Some(labpc_pci_detach),
};

static LABPC_PCI_TABLE: [PciDeviceId; 2] = [
    PciDeviceId { vendor: PCI_VENDOR_ID_NI, device: 0x0161, driver_data: LabpcPciBoardid::BoardNiPci1200 as usize },
    PciDeviceId { vendor: 0, device: 0, driver_data: 0 },
];

unsafe fn labpc_pci_probe(dev: *mut PciDev, id: *const PciDeviceId) -> i32 {
    comedi_pci_auto_config(dev, &mut LABPC_PCI_COMEDI_DRIVER, (*id).driver_data)
}

static mut LABPC_PCI_DRIVER: PciDriver = PciDriver {
    name: "labpc_pci",
    id_table: LABPC_PCI_TABLE.as_ptr(),
    probe: Some(labpc_pci_probe),
    remove: Some(comedi_pci_auto_unconfig),
};

// module_comedi_pci_driver(labpc_pci_comedi_driver, labpc_pci_driver);
// MODULE_DEVICE_TABLE(pci, labpc_pci_table);
// MODULE_DESCRIPTION("Comedi: National Instruments Lab-PC PCI-1200 driver");
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_LICENSE("GPL");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
