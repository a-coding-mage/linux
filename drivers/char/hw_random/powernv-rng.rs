// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2013 Michael Ellerman, Guo Chao, IBM Corp.
 */

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Dependencies supplied by the kernel build are intentionally left external.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct hwrng {
    pub name: *const c_char,
    pub read: Option<unsafe extern "C" fn(
        rng: *mut hwrng,
        data: *mut c_void,
        max: usize,
        wait: bool,
    ) -> c_int>,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

extern "C" {
    fn pnv_get_random_long(value: *mut c_ulong);
    fn devm_hwrng_register(dev: *mut device, rng: *mut hwrng) -> c_int;
    fn pr_info(fmt: *const c_char, ...);
}

type c_ulong = usize;

const EEXIST: c_int = 17;
const ENODEV: c_int = 19;

unsafe extern "C" fn powernv_rng_read(
    _rng: *mut hwrng,
    data: *mut c_void,
    max: usize,
    _wait: bool,
) -> c_int {
    let mut buf: *mut c_ulong;
    let len: usize;

    /* We rely on rng_buffer_size() being >= sizeof(unsigned long) */
    len = max / core::mem::size_of::<c_ulong>();

    buf = data as *mut c_ulong;

    for _i in 0..len {
        pnv_get_random_long(buf);
        buf = buf.add(1);
    }

    (len * core::mem::size_of::<c_ulong>()) as c_int
}

static mut powernv_hwrng: hwrng = hwrng {
    name: b"powernv-rng\0".as_ptr() as *const c_char,
    read: Some(powernv_rng_read),
};

unsafe extern "C" fn powernv_rng_probe(pdev: *mut platform_device) -> c_int {
    let mut rc: c_int;

    rc = devm_hwrng_register(&mut (*pdev).dev, &mut powernv_hwrng);
    if rc != 0 {
        /* We only register one device, ignore any others */
        if rc == -EEXIST {
            rc = -ENODEV;
        }

        return rc;
    }

    pr_info(b"Registered powernv hwrng.\n\0".as_ptr() as *const c_char);

    0
}

static powernv_rng_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"ibm,power-rng\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

static mut powernv_rng_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"powernv_rng\0".as_ptr() as *const c_char,
        of_match_table: powernv_rng_match.as_ptr(),
    },
    probe: Some(powernv_rng_probe),
};

// MODULE_DEVICE_TABLE(of, powernv_rng_match);
// module_platform_driver(powernv_rng_driver);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Bare metal HWRNG driver for POWER7+ and above");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
