// SPDX-License-Identifier: GPL-2.0+
/*
 * COMEDI driver for generic PCI based 8255 digital i/o boards
 * Copyright (C) 2012 H Hartley Sweeten <hsweeten@visionengravers.com>
 *
 * Rust translation of the original C implementation.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Pci8255BoardId {
    BoardAdlinkPci7224,
    BoardAdlinkPci7248,
    BoardAdlinkPci7296,
    BoardCbPcidio24,
    BoardCbPcidio24h,
    BoardCbPcidio48hOld,
    BoardCbPcidio48hNew,
    BoardCbPcidio96h,
    BoardNiPcidio96,
    BoardNiPcidio96b,
    BoardNiPxi6508,
    BoardNiPci6503,
    BoardNiPci6503b,
    BoardNiPci6503x,
    BoardNiPxi6503,
}

#[repr(C)]
pub struct Pci8255Boardinfo {
    pub name: *const core::ffi::c_char,
    pub dio_badr: i32,
    pub n_8255: i32,
    pub has_mite: u32,
}

// C build-time CONFIG_HAS_IOPORT condition is preserved by retaining its items here.
pub static PCI_8255_BOARDS: [Pci8255Boardinfo; 15] = [
    Pci8255Boardinfo { name: b"adl_pci-7224\0".as_ptr() as _, dio_badr: 2, n_8255: 1, has_mite: 0 },
    Pci8255Boardinfo { name: b"adl_pci-7248\0".as_ptr() as _, dio_badr: 2, n_8255: 2, has_mite: 0 },
    Pci8255Boardinfo { name: b"adl_pci-7296\0".as_ptr() as _, dio_badr: 2, n_8255: 4, has_mite: 0 },
    Pci8255Boardinfo { name: b"cb_pci-dio24\0".as_ptr() as _, dio_badr: 2, n_8255: 1, has_mite: 0 },
    Pci8255Boardinfo { name: b"cb_pci-dio24h\0".as_ptr() as _, dio_badr: 2, n_8255: 1, has_mite: 0 },
    Pci8255Boardinfo { name: b"cb_pci-dio48h\0".as_ptr() as _, dio_badr: 1, n_8255: 2, has_mite: 0 },
    Pci8255Boardinfo { name: b"cb_pci-dio48h\0".as_ptr() as _, dio_badr: 2, n_8255: 2, has_mite: 0 },
    Pci8255Boardinfo { name: b"cb_pci-dio96h\0".as_ptr() as _, dio_badr: 2, n_8255: 4, has_mite: 0 },
    Pci8255Boardinfo { name: b"ni_pci-dio-96\0".as_ptr() as _, dio_badr: 1, n_8255: 4, has_mite: 1 },
    Pci8255Boardinfo { name: b"ni_pci-dio-96b\0".as_ptr() as _, dio_badr: 1, n_8255: 4, has_mite: 1 },
    Pci8255Boardinfo { name: b"ni_pxi-6508\0".as_ptr() as _, dio_badr: 1, n_8255: 4, has_mite: 1 },
    Pci8255Boardinfo { name: b"ni_pci-6503\0".as_ptr() as _, dio_badr: 1, n_8255: 1, has_mite: 1 },
    Pci8255Boardinfo { name: b"ni_pci-6503b\0".as_ptr() as _, dio_badr: 1, n_8255: 1, has_mite: 1 },
    Pci8255Boardinfo { name: b"ni_pci-6503x\0".as_ptr() as _, dio_badr: 1, n_8255: 1, has_mite: 1 },
    Pci8255Boardinfo { name: b"ni_pxi-6503\0".as_ptr() as _, dio_badr: 1, n_8255: 1, has_mite: 1 },
];

pub const MITE_IODWBSR: usize = 0xc0;
pub const WENAB: u32 = 1 << 7;

unsafe fn pci_8255_mite_init(pcidev: *mut pci_dev) -> i32 {
    let mite_base = pci_ioremap_bar(pcidev, 0);
    if mite_base.is_null() { return -ENOMEM; }
    let main_phys_addr: u32 = pci_resource_start(pcidev, 1) as u32;
    writel(main_phys_addr | WENAB, mite_base.add(MITE_IODWBSR));
    iounmap(mite_base);
    0
}

unsafe fn pci_8255_auto_attach(dev: *mut comedi_device, context: usize) -> i32 {
    let pcidev = comedi_to_pci_dev(dev);
    let board = PCI_8255_BOARDS.get(context);
    if board.is_none() { return -ENODEV; }
    let board = board.unwrap();
    (*dev).board_ptr = board as *const _ as *mut _;
    (*dev).board_name = board.name;
    let mut ret = comedi_pci_enable(dev);
    if ret != 0 { return ret; }
    if board.has_mite != 0 { ret = pci_8255_mite_init(pcidev); if ret != 0 { return ret; } }
    if (pci_resource_flags(pcidev, board.dio_badr) & IORESOURCE_MEM) != 0 {
        (*dev).mmio = pci_ioremap_bar(pcidev, board.dio_badr);
        if (*dev).mmio.is_null() { return -ENOMEM; }
    } else if IS_ENABLED_CONFIG_HAS_IOPORT {
        (*dev).iobase = pci_resource_start(pcidev, board.dio_badr);
    } else { dev_err((*dev).class_dev, b"error! need I/O port support\n"); return -ENXIO; }
    ret = comedi_alloc_subdevices(dev, board.n_8255);
    if ret != 0 { return ret; }
    for i in 0..board.n_8255 {
        let s = (*dev).subdevices.add(i as usize);
        ret = if !(*dev).mmio.is_null() { subdev_8255_mm_init(dev, s, i * I8255_SIZE) } else { subdev_8255_io_init(dev, s, i * I8255_SIZE) };
        if ret != 0 { return ret; }
    }
    0
}

// External kernel, PCI, Comedi, and module-registration declarations are supplied by dependent files.
extern "C" {
    static mut pci_8255_driver: comedi_driver;
}

#[no_mangle]
pub unsafe extern "C" fn pci_8255_pci_probe(dev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    comedi_pci_auto_config(dev, &mut pci_8255_driver, (*id).driver_data)
}

// PCI device table and module registration retain the original entries; PCI_VDEVICE
// and module_comedi_pci_driver are supplied by the kernel/Comedi bindings.
#[allow(non_upper_case_globals)]
pub static pci_8255_pci_table: [pci_device_id; 16] = [
    PCI_VDEVICE(ADLINK, 0x7224, BoardAdlinkPci7224),
    PCI_VDEVICE(ADLINK, 0x7248, BoardAdlinkPci7248),
    PCI_VDEVICE(ADLINK, 0x7296, BoardAdlinkPci7296),
    PCI_VDEVICE(CB, 0x0028, BoardCbPcidio24),
    PCI_VDEVICE(CB, 0x0014, BoardCbPcidio24h),
    PCI_VDEVICE_SUB(CB, 0x000b, 0x0000, 0x0000, BoardCbPcidio48hOld),
    PCI_VDEVICE_SUB(CB, 0x000b, PCI_VENDOR_ID_CB, 0x000b, BoardCbPcidio48hNew),
    PCI_VDEVICE(CB, 0x0017, BoardCbPcidio96h),
    PCI_VDEVICE(NI, 0x0160, BoardNiPcidio96),
    PCI_VDEVICE(NI, 0x1630, BoardNiPcidio96b),
    PCI_VDEVICE(NI, 0x13c0, BoardNiPxi6508),
    PCI_VDEVICE(NI, 0x0400, BoardNiPci6503),
    PCI_VDEVICE(NI, 0x1250, BoardNiPci6503b),
    PCI_VDEVICE(NI, 0x17d0, BoardNiPci6503x),
    PCI_VDEVICE(NI, 0x1800, BoardNiPxi6503),
    PCI_DEVICE_TABLE_END,
];

pub static mut pci_8255_pci_driver: pci_driver = pci_driver {
    name: b"8255_pci\0".as_ptr() as _,
    id_table: pci_8255_pci_table.as_ptr(),
    probe: Some(pci_8255_pci_probe),
    remove: Some(comedi_pci_auto_unconfig),
};

// module_comedi_pci_driver(pci_8255_driver, pci_8255_pci_driver);
// MODULE_DESCRIPTION("COMEDI - Generic PCI based 8255 Digital I/O boards");
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
