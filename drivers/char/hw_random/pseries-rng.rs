// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 Michael Neuling IBM Corporation
 *
 * Driver for the pseries hardware RNG for POWER7+ and above
 */

// C dependencies supplied by the surrounding kernel translation:
// linux/kernel.h, linux/module.h, linux/hw_random.h, asm/vio.h

use core::ffi::c_void;

// Build-time kernel constants and types are supplied externally.
extern "C" {
    fn plpar_hcall(token: u64, retbuf: *mut u64) -> i32;
    fn memcpy(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
    fn pr_err_ratelimited(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn hwrng_register(rng: *mut hwrng) -> i32;
    fn hwrng_unregister(rng: *mut hwrng);
    fn vio_register_driver(driver: *mut vio_driver) -> i32;
    fn vio_unregister_driver(driver: *mut vio_driver);
}

#[repr(C)]
pub struct hwrng {
    pub name: *const u8,
    pub read: Option<unsafe extern "C" fn(*mut hwrng, *mut c_void, usize, bool) -> i32>,
}

#[repr(C)]
pub struct vio_dev {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct vio_device_id {
    pub type_: *const u8,
    pub compat: *const u8,
}

#[repr(C)]
pub struct vio_driver {
    pub name: *const u8,
    pub probe: Option<unsafe extern "C" fn(*mut vio_dev, *const vio_device_id) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut vio_dev)>,
    pub get_desired_dma: Option<unsafe extern "C" fn(*mut vio_dev) -> u64>,
    pub id_table: *const vio_device_id,
}

const PLPAR_HCALL_BUFSIZE: usize = 4;
const H_RANDOM: u64 = 0;
const H_SUCCESS: i32 = 0;
const EIO: i32 = 5;

static KBUILD_MODNAME: &[u8] = b"pseries_rng\0";

unsafe extern "C" fn pseries_rng_read(
    _rng: *mut hwrng,
    data: *mut c_void,
    _max: usize,
    _wait: bool,
) -> i32 {
    let mut buffer: [u64; PLPAR_HCALL_BUFSIZE] = [0; PLPAR_HCALL_BUFSIZE];
    let rc: i32;

    rc = plpar_hcall(H_RANDOM, buffer.as_mut_ptr());
    if rc != H_SUCCESS {
        pr_err_ratelimited(b"H_RANDOM call failed %d\n\0".as_ptr(), rc);
        return -EIO;
    }
    memcpy(data, buffer.as_ptr() as *const c_void, 8);

    /* The hypervisor interface returns 64 bits */
    8
}

/*
 * pseries_rng_get_desired_dma - Return desired DMA allocate for CMO operations
 *
 * This is a required function for a driver to operate in a CMO environment
 * but this device does not make use of DMA allocations, return 0.
 *
 * Return value:
 *\tNumber of bytes of IO data the driver will need to perform well -> 0
 */
unsafe extern "C" fn pseries_rng_get_desired_dma(_vdev: *mut vio_dev) -> u64 {
    0
}

static mut pseries_rng: hwrng = hwrng {
    name: KBUILD_MODNAME.as_ptr(),
    read: Some(pseries_rng_read),
};

unsafe extern "C" fn pseries_rng_probe(
    _dev: *mut vio_dev,
    _id: *const vio_device_id,
) -> i32 {
    hwrng_register(&mut pseries_rng)
}

unsafe extern "C" fn pseries_rng_remove(_dev: *mut vio_dev) {
    hwrng_unregister(&mut pseries_rng);
}

static pseries_rng_driver_ids: [vio_device_id; 2] = [
    vio_device_id {
        type_: b"ibm,random-v1\0".as_ptr(),
        compat: b"ibm,random\0".as_ptr(),
    },
    vio_device_id {
        type_: b"\0".as_ptr(),
        compat: b"\0".as_ptr(),
    },
];

// MODULE_DEVICE_TABLE(vio, pseries_rng_driver_ids);

static mut pseries_rng_driver: vio_driver = vio_driver {
    name: KBUILD_MODNAME.as_ptr(),
    probe: Some(pseries_rng_probe),
    remove: Some(pseries_rng_remove),
    get_desired_dma: Some(pseries_rng_get_desired_dma),
    id_table: pseries_rng_driver_ids.as_ptr(),
};

unsafe extern "C" fn rng_init() -> i32 {
    pr_info(b"Registering IBM pSeries RNG driver\n\0".as_ptr());
    vio_register_driver(&mut pseries_rng_driver)
}

// module_init(rng_init);

unsafe extern "C" fn rng_exit() {
    vio_unregister_driver(&mut pseries_rng_driver);
}

// module_exit(rng_exit);

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Michael Neuling <mikey@neuling.org>");
// MODULE_DESCRIPTION("H/W RNG driver for IBM pSeries processors");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
