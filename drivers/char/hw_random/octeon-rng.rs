/*
 * Hardware Random Number Generator support for Cavium Networks
 * Octeon processor family.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2009 Cavium Networks
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct octeon_rng {
    pub ops: hwrng,
    pub control_status: *mut core::ffi::c_void,
    pub result: *mut core::ffi::c_void,
}

extern "C" {
    fn cvmx_write_csr(address: core::ffi::c_ulong, value: u64);
    fn cvmx_read64_uint32(address: core::ffi::c_ulong) -> u32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut core::ffi::c_void;
    fn platform_get_resource(
        pdev: *mut platform_device,
        resource_type: core::ffi::c_ulong,
        index: core::ffi::c_uint,
    ) -> *mut resource;
    fn devm_ioremap(
        dev: *mut device,
        offset: core::ffi::c_ulong,
        size: usize,
    ) -> *mut core::ffi::c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn devm_hwrng_register(dev: *mut device, rng: *mut hwrng) -> core::ffi::c_int;
    fn dev_info(dev: *mut device, format: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct hwrng {
    pub name: *const core::ffi::c_char,
    pub init: Option<unsafe extern "C" fn(*mut hwrng) -> core::ffi::c_int>,
    pub cleanup: Option<unsafe extern "C" fn(*mut hwrng)>,
    pub data_read: Option<unsafe extern "C" fn(*mut hwrng, *mut u32) -> core::ffi::c_int>,
}

#[repr(C)]
pub struct resource {
    pub start: core::ffi::c_ulong,
}

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

pub type gfp_t = core::ffi::c_uint;

#[repr(C)]
pub union cvmx_rnm_ctl_status {
    pub u64: u64,
    pub s: cvmx_rnm_ctl_status_bits,
}

#[repr(C)]
pub struct cvmx_rnm_ctl_status_bits {
    pub ent_en: u64,
    pub rng_en: u64,
}

unsafe extern "C" fn octeon_rng_init(rng: *mut hwrng) -> core::ffi::c_int {
    let mut ctl = cvmx_rnm_ctl_status { u64: 0 };
    let p = (rng as *mut u8).sub(core::mem::offset_of!(octeon_rng, ops)) as *mut octeon_rng;

    ctl.u64 = 0;
    // Enable the entropy source.
    ctl.s.ent_en = 1;
    // Enable the RNG hardware.
    ctl.s.rng_en = 1;
    cvmx_write_csr((*p).control_status as core::ffi::c_ulong, ctl.u64);
    0
}

unsafe extern "C" fn octeon_rng_cleanup(rng: *mut hwrng) {
    let mut ctl = cvmx_rnm_ctl_status { u64: 0 };
    let p = (rng as *mut u8).sub(core::mem::offset_of!(octeon_rng, ops)) as *mut octeon_rng;

    ctl.u64 = 0;
    // Disable everything.
    cvmx_write_csr((*p).control_status as core::ffi::c_ulong, ctl.u64);
}

unsafe extern "C" fn octeon_rng_data_read(
    rng: *mut hwrng,
    data: *mut u32,
) -> core::ffi::c_int {
    let p = (rng as *mut u8).sub(core::mem::offset_of!(octeon_rng, ops)) as *mut octeon_rng;

    *data = cvmx_read64_uint32((*p).result as core::ffi::c_ulong);
    core::mem::size_of::<u32>() as core::ffi::c_int
}

// The platform-driver registration macro expands to the driver's registration
// and module initialization in the surrounding kernel environment.
#[no_mangle]
pub static mut octeon_rng_driver: platform_driver = platform_driver {
    driver: driver {
        name: b"octeon_rng\0".as_ptr() as *const core::ffi::c_char,
    },
    probe: Some(octeon_rng_probe),
};

#[repr(C)]
pub struct driver {
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> core::ffi::c_int>,
}

unsafe extern "C" fn octeon_rng_probe(pdev: *mut platform_device) -> core::ffi::c_int {
    let res_ports: *mut resource;
    let res_result: *mut resource;
    let rng: *mut octeon_rng;
    let ret: core::ffi::c_int;
    let ops = hwrng {
        name: b"octeon\0".as_ptr() as *const core::ffi::c_char,
        init: Some(octeon_rng_init),
        cleanup: Some(octeon_rng_cleanup),
        data_read: Some(octeon_rng_data_read),
    };

    rng = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<octeon_rng>(), 0) as *mut octeon_rng;
    if rng.is_null() {
        return -12;
    }

    res_ports = platform_get_resource(pdev, 0x0000_0000_0000_0200, 0);
    if res_ports.is_null() {
        return -2;
    }

    res_result = platform_get_resource(pdev, 0x0000_0000_0000_0200, 1);
    if res_result.is_null() {
        return -2;
    }

    (*rng).control_status = devm_ioremap(&mut (*pdev).dev, (*res_ports).start, core::mem::size_of::<u64>());
    if (*rng).control_status.is_null() {
        return -2;
    }

    (*rng).result = devm_ioremap(&mut (*pdev).dev, (*res_result).start, core::mem::size_of::<u64>());
    if (*rng).result.is_null() {
        return -2;
    }

    (*rng).ops = ops;
    platform_set_drvdata(pdev, &mut (*rng).ops as *mut hwrng as *mut core::ffi::c_void);
    ret = devm_hwrng_register(&mut (*pdev).dev, &mut (*rng).ops);
    if ret != 0 {
        return -2;
    }

    dev_info(&mut (*pdev).dev, b"Octeon Random Number Generator\n\0".as_ptr() as *const core::ffi::c_char);
    0
}

// MODULE_AUTHOR("David Daney");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
