/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2012 Cavium, Inc.
 * Copyright (C) 2009 Wind River Systems,
 *   written by Ralf Baechle <ralf@linux-mips.org>
 */

// Linux and Octeon headers from the original implementation provide the
// declarations and definitions referenced below.

extern "C" {
    fn octeon_npi_read32(address: u64) -> u32;
    fn octeon_npi_write32(address: u64, value: u32);
    fn edac_pci_handle_pe(pci: *mut edac_pci_ctl_info, msg: *const i8);
    fn edac_pci_handle_npe(pci: *mut edac_pci_ctl_info, msg: *const i8);
    fn edac_pci_alloc_ctl_info(rank: i32, name: *const i8) -> *mut edac_pci_ctl_info;
    fn edac_pci_add_device(pci: *mut edac_pci_ctl_info, dev_idx: i32) -> i32;
    fn edac_pci_free_ctl_info(pci: *mut edac_pci_ctl_info);
    fn edac_pci_del_device(dev: *mut device);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut edac_pci_ctl_info);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut edac_pci_ctl_info;
    fn dev_name(dev: *mut device) -> *const i8;
    fn pr_err(fmt: *const i8, ...);
}

// These types, constants, and callback-bearing structures are supplied by the
// corresponding kernel and Octeon dependencies.
#[repr(C)]
pub struct edac_pci_ctl_info {
    pub dev: *mut device,
    pub dev_name: *const i8,
    pub mod_name: *const i8,
    pub ctl_name: *const i8,
    pub edac_check: Option<unsafe extern "C" fn(*mut edac_pci_ctl_info)>,
}

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub union cvmx_pci_cfg01 {
    pub u32: u32,
    pub s: cvmx_pci_cfg01_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pci_cfg01_s {
    pub dpe: u32,
    pub sse: u32,
    pub rma: u32,
    pub rta: u32,
    pub sta: u32,
    pub mdpe: u32,
}

extern "C" {
    static mut octeon_pci_driver: platform_driver;
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const i8,
}

const CVMX_NPI_PCI_CFG01: u64 = 0; // supplied by the Octeon headers

unsafe extern "C" fn octeon_pci_poll(pci: *mut edac_pci_ctl_info) {
    let mut cfg01 = cvmx_pci_cfg01 { u32: octeon_npi_read32(CVMX_NPI_PCI_CFG01) };

    if (*cfg01.s).dpe != 0 {
        edac_pci_handle_pe(pci, (*pci).ctl_name);
        (*cfg01.s).dpe = 1;
        octeon_npi_write32(CVMX_NPI_PCI_CFG01, cfg01.u32);
    }
    if (*cfg01.s).sse != 0 {
        edac_pci_handle_npe(pci, b"Signaled System Error\0".as_ptr() as *const i8);
        (*cfg01.s).sse = 1;
        octeon_npi_write32(CVMX_NPI_PCI_CFG01, cfg01.u32);
    }
    if (*cfg01.s).rma != 0 {
        edac_pci_handle_npe(pci, b"Received Master Abort\0".as_ptr() as *const i8);
        (*cfg01.s).rma = 1;
        octeon_npi_write32(CVMX_NPI_PCI_CFG01, cfg01.u32);
    }
    if (*cfg01.s).rta != 0 {
        edac_pci_handle_npe(pci, b"Received Target Abort\0".as_ptr() as *const i8);
        (*cfg01.s).rta = 1;
        octeon_npi_write32(CVMX_NPI_PCI_CFG01, cfg01.u32);
    }
    if (*cfg01.s).sta != 0 {
        edac_pci_handle_npe(pci, b"Signaled Target Abort\0".as_ptr() as *const i8);
        (*cfg01.s).sta = 1;
        octeon_npi_write32(CVMX_NPI_PCI_CFG01, cfg01.u32);
    }
    if (*cfg01.s).mdpe != 0 {
        edac_pci_handle_npe(pci, b"Master Data Parity Error\0".as_ptr() as *const i8);
        (*cfg01.s).mdpe = 1;
        octeon_npi_write32(CVMX_NPI_PCI_CFG01, cfg01.u32);
    }
}

unsafe extern "C" fn octeon_pci_probe(pdev: *mut platform_device) -> i32 {
    let mut res: i32 = 0;
    let pci = edac_pci_alloc_ctl_info(0, b"octeon_pci_err\0".as_ptr() as *const i8);
    if pci.is_null() {
        return -12; // -ENOMEM
    }

    (*pci).dev = &mut (*pdev).dev;
    platform_set_drvdata(pdev, pci);
    (*pci).dev_name = dev_name(&mut (*pdev).dev);
    (*pci).mod_name = b"octeon-pci\0".as_ptr() as *const i8;
    (*pci).ctl_name = b"octeon_pci_err\0".as_ptr() as *const i8;
    (*pci).edac_check = Some(octeon_pci_poll);

    if edac_pci_add_device(pci, 0) > 0 {
        pr_err(b"%s: edac_pci_add_device() failed\n\0".as_ptr() as *const i8);
        edac_pci_free_ctl_info(pci);
        return res;
    }
    0
}

unsafe extern "C" fn octeon_pci_remove(pdev: *mut platform_device) {
    let pci = platform_get_drvdata(pdev);
    edac_pci_del_device(&mut (*pdev).dev);
    edac_pci_free_ctl_info(pci);
}

#[no_mangle]
pub static mut OCTEON_PCI_DRIVER: platform_driver = platform_driver {
    probe: Some(octeon_pci_probe),
    remove: Some(octeon_pci_remove),
    driver: driver { name: b"octeon_pci_edac\0".as_ptr() as *const i8 },
};

// module_platform_driver(octeon_pci_driver);
// MODULE_DESCRIPTION("Cavium Octeon PCI Controller EDAC driver");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Ralf Baechle <ralf@linux-mips.org>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
