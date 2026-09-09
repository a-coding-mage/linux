// SPDX-License-Identifier: GPL-2.0
/*
 * Hardware Random Number Generator support.
 * Cavium Thunder, Marvell OcteonTx/Tx2 processor families.
 *
 * Copyright (C) 2016 Cavium, Inc.
 */

// Linux kernel dependencies supplied by other translation units.

pub const THUNDERX_RNM_ENT_EN: u64 = 0x1;
pub const THUNDERX_RNM_RNG_EN: u64 = 0x2;

#[repr(C)]
pub struct cavium_rng_pf {
    pub control_status: *mut core::ffi::c_void,
}

// Opaque Linux kernel types and externally supplied functions.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
}

unsafe extern "C" {
    pub fn devm_kzalloc(
        dev: *mut device,
        size: usize,
        flags: u32,
    ) -> *mut core::ffi::c_void;
    pub fn pcim_iomap(
        pdev: *mut pci_dev,
        bar: i32,
        maxlen: usize,
    ) -> *mut core::ffi::c_void;
    pub fn writeq(value: u64, address: *mut core::ffi::c_void);
    pub fn pci_set_drvdata(pdev: *mut pci_dev, data: *mut core::ffi::c_void);
    pub fn pci_get_drvdata(pdev: *mut pci_dev) -> *mut core::ffi::c_void;
    pub fn pci_enable_sriov(pdev: *mut pci_dev, num_vfs: u16) -> i32;
    pub fn pci_disable_sriov(pdev: *mut pci_dev);
    pub fn dev_err(dev: *mut device, format: *const core::ffi::c_char, ...);
}

pub const GFP_KERNEL: u32 = 0;
pub const ENOMEM: i32 = 12;
pub const PCI_VENDOR_ID_CAVIUM: u32 = 0x177d;

#[inline]
unsafe fn pci_device(vendor: u32, device: u32) -> pci_device_id {
    pci_device_id { vendor, device }
}

/* Enable the RNG hardware and activate the VF */
pub unsafe extern "C" fn cavium_rng_probe(
    pdev: *mut pci_dev,
    _id: *const pci_device_id,
) -> i32 {
    let mut rng: *mut cavium_rng_pf;
    let iov_err: i32;

    rng = devm_kzalloc(
        pdev as *mut device,
        core::mem::size_of::<cavium_rng_pf>(),
        GFP_KERNEL,
    ) as *mut cavium_rng_pf;
    if rng.is_null() {
        return -ENOMEM;
    }

    /* Map the RNG control */
    (*rng).control_status = pcim_iomap(pdev, 0, 0);
    if (*rng).control_status.is_null() {
        dev_err(
            pdev as *mut device,
            c"Error iomap failed retrieving control_status.\n".as_ptr(),
        );
        return -ENOMEM;
    }

    /* Enable the RNG hardware and entropy source */
    writeq(
        THUNDERX_RNM_RNG_EN | THUNDERX_RNM_ENT_EN,
        (*rng).control_status,
    );

    pci_set_drvdata(pdev, rng as *mut core::ffi::c_void);

    /* Enable the Cavium RNG as a VF */
    iov_err = pci_enable_sriov(pdev, 1);
    if iov_err != 0 {
        /* Disable the RNG hardware and entropy source */
        writeq(0, (*rng).control_status);
        dev_err(
            pdev as *mut device,
            c"Error initializing RNG virtual function,(%i).\n".as_ptr(),
            iov_err,
        );
        return iov_err;
    }

    0
}

/* Disable VF and RNG Hardware */
pub unsafe extern "C" fn cavium_rng_remove(pdev: *mut pci_dev) {
    let rng: *mut cavium_rng_pf;

    rng = pci_get_drvdata(pdev) as *mut cavium_rng_pf;

    /* Remove the VF */
    pci_disable_sriov(pdev);

    /* Disable the RNG hardware and entropy source */
    writeq(0, (*rng).control_status);
}

pub static cavium_rng_pf_id_table: [pci_device_id; 2] = [
    // PCI_DEVICE(PCI_VENDOR_ID_CAVIUM, 0xa018) /* Thunder RNM */
    pci_device(PCI_VENDOR_ID_CAVIUM, 0xa018),
    pci_device_id { vendor: 0, device: 0 },
];

// MODULE_DEVICE_TABLE(pci, cavium_rng_pf_id_table);

#[repr(C)]
pub struct pci_driver {
    pub name: *const core::ffi::c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
}

pub static mut cavium_rng_pf_driver: pci_driver = pci_driver {
    name: c"cavium_rng_pf".as_ptr(),
    id_table: cavium_rng_pf_id_table.as_ptr(),
    probe: Some(cavium_rng_probe),
    remove: Some(cavium_rng_remove),
};

// module_pci_driver(cavium_rng_pf_driver);
// MODULE_AUTHOR("Omer Khaliq <okhaliq@caviumnetworks.com>");
// MODULE_DESCRIPTION("Cavium ThunderX Random Number Generator support");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
