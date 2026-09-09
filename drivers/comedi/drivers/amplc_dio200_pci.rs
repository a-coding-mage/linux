// SPDX-License-Identifier: GPL-2.0+
/* comedi/drivers/amplc_dio200_pci.c
 *
 * Driver for Amplicon PCI215, PCI272, PCIe215, PCIe236, PCIe296.
 *
 * Copyright (C) 2005-2013 MEV Ltd. <https://www.mev.co.uk/>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998,2000 David A. Schleef <ds@schleef.org>
 */

/* Direct translation of the C implementation.  Kernel/comedi declarations
 * supplied by the included headers remain external dependencies. */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Dio200PciModel {
    #[cfg(CONFIG_HAS_IOPORT)]
    Pci215Model,
    #[cfg(CONFIG_HAS_IOPORT)]
    Pci272Model,
    Pcie215Model,
    Pcie236Model,
    Pcie296Model,
}

/* CONFIG_HAS_IOPORT controls the two legacy PCI models, as in the source. */
#[repr(C)]
pub struct Dio200Board {
    pub name: *const core::ffi::c_char,
    pub mainbar: u32,
    pub n_subdevs: u32,
    pub sdtype: [u32; 8],
    pub sdinfo: [u32; 8],
    pub has_int_sce: bool,
    pub has_clk_gat_sce: bool,
    pub is_pcie: bool,
}

const SD_NONE: u32 = 0;
const SD_8255: u32 = 1;
const SD_8254: u32 = 2;
const SD_TIMER: u32 = 3;
const SD_INTR: u32 = 4;

static DIO200_PCI_BOARDS: [Dio200Board; 5] = [
    Dio200Board { name: b"pci215\0".as_ptr() as *const _, mainbar: 2, n_subdevs: 5,
        sdtype: [SD_8255, SD_8255, SD_8254, SD_8254, SD_INTR, 0, 0, 0],
        sdinfo: [0x00, 0x08, 0x10, 0x14, 0x3f, 0, 0, 0], has_int_sce: true,
        has_clk_gat_sce: true, is_pcie: false },
    Dio200Board { name: b"pci272\0".as_ptr() as *const _, mainbar: 2, n_subdevs: 4,
        sdtype: [SD_8255, SD_8255, SD_8255, SD_INTR, 0, 0, 0, 0],
        sdinfo: [0x00, 0x08, 0x10, 0x3f, 0, 0, 0, 0], has_int_sce: true,
        has_clk_gat_sce: false, is_pcie: false },
    Dio200Board { name: b"pcie215\0".as_ptr() as *const _, mainbar: 1, n_subdevs: 8,
        sdtype: [SD_8255, SD_NONE, SD_8255, SD_NONE, SD_8254, SD_8254, SD_TIMER, SD_INTR],
        sdinfo: [0x00, 0x00, 0x08, 0x00, 0x10, 0x14, 0x00, 0x3f], has_int_sce: true,
        has_clk_gat_sce: true, is_pcie: true },
    Dio200Board { name: b"pcie236\0".as_ptr() as *const _, mainbar: 1, n_subdevs: 8,
        sdtype: [SD_8255, SD_NONE, SD_NONE, SD_NONE, SD_8254, SD_8254, SD_TIMER, SD_INTR],
        sdinfo: [0x00, 0x00, 0x00, 0x00, 0x10, 0x14, 0x00, 0x3f], has_int_sce: true,
        has_clk_gat_sce: true, is_pcie: true },
    Dio200Board { name: b"pcie296\0".as_ptr() as *const _, mainbar: 1, n_subdevs: 8,
        sdtype: [SD_8255, SD_8255, SD_8255, SD_8255, SD_8254, SD_8254, SD_TIMER, SD_INTR],
        sdinfo: [0x00, 0x04, 0x08, 0x0c, 0x10, 0x14, 0x00, 0x3f], has_int_sce: true,
        has_clk_gat_sce: true, is_pcie: true },
];

#[repr(C)]
pub struct ComediDevice {
    pub board_ptr: *const Dio200Board,
    pub board_name: *const core::ffi::c_char,
    pub mmio: *mut core::ffi::c_void,
    pub iobase: usize,
    pub class_dev: *mut core::ffi::c_void,
    pub driver: *mut ComediDriver,
}

#[repr(C)]
pub struct ComediDriver {
    pub driver_name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct PciDev {
    pub irq: i32,
}

#[repr(C)]
pub struct PciDeviceId {
    pub driver_data: usize,
}

extern "C" {
    fn comedi_to_pci_dev(dev: *mut ComediDevice) -> *mut PciDev;
    fn pci_resource_len(dev: *mut PciDev, bar: u32) -> usize;
    fn pci_ioremap_bar(dev: *mut PciDev, bar: u32) -> *mut core::ffi::c_void;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn iounmap(addr: *mut core::ffi::c_void);
    fn amplc_dio200_set_enhance(dev: *mut ComediDevice, enable: i32);
    fn comedi_pci_enable(dev: *mut ComediDevice) -> i32;
    fn pci_resource_flags(dev: *mut PciDev, bar: u32) -> u64;
    fn pci_resource_start(dev: *mut PciDev, bar: u32) -> usize;
    fn amplc_dio200_common_attach(dev: *mut ComediDevice, irq: i32, flags: u32) -> i32;
    fn comedi_pci_detach(dev: *mut ComediDevice) -> i32;
    fn comedi_pci_auto_config(dev: *mut PciDev, driver: *mut ComediDriver, data: usize) -> i32;
    fn comedi_pci_auto_unconfig(dev: *mut PciDev);
    fn pci_name(dev: *mut PciDev) -> *const core::ffi::c_char;
}

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const ENXIO: i32 = 6;
const IRQF_SHARED: u32 = 0x00000080;
const IORESOURCE_MEM: u64 = 0x00000200;

unsafe fn dio200_pcie_board_setup_impl(dev: *mut ComediDevice) -> i32 {
    let pcidev = comedi_to_pci_dev(dev);
    let brbase: *mut core::ffi::c_void;

    if pci_resource_len(pcidev, 0) < 0x4000 {
        return -EINVAL;
    }
    brbase = pci_ioremap_bar(pcidev, 0);
    if brbase.is_null() {
        return -ENOMEM;
    }
    writel(0x80, (brbase as *mut u8).add(0x50) as *mut core::ffi::c_void);
    iounmap(brbase);
    amplc_dio200_set_enhance(dev, 1);
    0
}

unsafe fn dio200_pci_auto_attach(dev: *mut ComediDevice, context_model: usize) -> i32 {
    let pci_dev = comedi_to_pci_dev(dev);
    let board: *const Dio200Board;
    let bar: u32;
    let mut ret: i32;

    if context_model < DIO200_PCI_BOARDS.len() {
        board = DIO200_PCI_BOARDS.as_ptr().add(context_model);
    } else {
        board = core::ptr::null();
    }
    if board.is_null() {
        return -EINVAL;
    }
    (*dev).board_ptr = board;
    (*dev).board_name = (*board).name;

    ret = comedi_pci_enable(dev);
    if ret != 0 {
        return ret;
    }

    bar = (*board).mainbar;
    if pci_resource_flags(pci_dev, bar) & IORESOURCE_MEM != 0 {
        (*dev).mmio = pci_ioremap_bar(pci_dev, bar);
        if (*dev).mmio.is_null() {
            return -ENOMEM;
        }
    } else {
        // IS_ENABLED(CONFIG_HAS_IOPORT) selects this legacy I/O-port branch.
        (*dev).iobase = pci_resource_start(pci_dev, bar);
    }

    if (*board).is_pcie {
        ret = dio200_pcie_board_setup_impl(dev);
        if ret < 0 {
            return ret;
        }
    }
    amplc_dio200_common_attach(dev, (*pci_dev).irq, IRQF_SHARED)
}

#[repr(C)]
pub struct PciDriver {
    pub name: *const core::ffi::c_char,
    pub id_table: *const PciDeviceId,
    pub probe: unsafe fn(*mut PciDev, *const PciDeviceId) -> i32,
    pub remove: Option<unsafe fn(*mut PciDev)>,
}

#[no_mangle]
pub unsafe extern "C" fn dio200_pci_probe(dev: *mut PciDev, id: *const PciDeviceId) -> i32 {
    comedi_pci_auto_config(dev, &mut dio200_pci_comedi_driver, (*id).driver_data)
}

#[no_mangle]
pub static mut dio200_pci_comedi_driver: ComediDriver = ComediDriver {
    driver_name: b"amplc_dio200_pci\0".as_ptr() as *const core::ffi::c_char,
};

#[no_mangle]
pub static mut dio200_pci_pci_driver: PciDriver = PciDriver {
    name: b"amplc_dio200_pci\0".as_ptr() as *const core::ffi::c_char,
    id_table: core::ptr::null(),
    probe: dio200_pci_probe,
    remove: None,
};

// The C source registers the comedi PCI driver and exports PCI module metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
