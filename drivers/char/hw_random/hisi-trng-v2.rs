// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 HiSilicon Limited. */

// Dependencies supplied by the surrounding kernel environment are intentionally
// left as external Rust declarations.

const HISI_TRNG_REG: usize = 0x00F0;
const HISI_TRNG_BYTES: usize = 4;
const HISI_TRNG_QUALITY: u32 = 512;
const SLEEP_US: u32 = 10;
const TIMEOUT_US: u32 = 10000;

#[repr(C)]
pub struct HisiTrng {
    pub base: *mut core::ffi::c_void,
    pub rng: Hwrng,
}

#[repr(C)]
pub struct Hwrng {
    pub name: *const core::ffi::c_char,
    pub read: Option<unsafe extern "C" fn(
        rng: *mut Hwrng,
        buf: *mut core::ffi::c_void,
        max: usize,
        wait: bool,
    ) -> i32>,
    pub quality: u32,
}

#[repr(C)]
pub struct PlatformDevice {
    pub dev: Device,
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct Device;

#[repr(C)]
pub struct AcpiDeviceId {
    pub id: *const core::ffi::c_char,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
    pub driver: Driver,
}

#[repr(C)]
pub struct Driver {
    pub name: *const core::ffi::c_char,
    pub acpi_match_table: *const AcpiDeviceId,
}

extern "C" {
    fn readl_poll_timeout(
        addr: *mut core::ffi::c_void,
        val: *mut u32,
        condition: u32,
        delay_us: u32,
        timeout_us: u32,
    ) -> i32;
    fn devm_kzalloc(
        dev: *mut Device,
        size: usize,
        flags: u32,
    ) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(
        pdev: *mut PlatformDevice,
        index: u32,
    ) -> *mut core::ffi::c_void;
    fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
    fn devm_hwrng_register(dev: *mut Device, rng: *mut Hwrng) -> i32;
    fn dev_err(dev: *mut Device, fmt: *const core::ffi::c_char, ...);
    fn platform_driver_register(driver: *mut PlatformDriver) -> i32;
    fn platform_driver_unregister(driver: *mut PlatformDriver);
}

const GFP_KERNEL: u32 = 0;

unsafe extern "C" fn hisi_trng_read(
    rng: *mut Hwrng,
    buf: *mut core::ffi::c_void,
    max: usize,
    _wait: bool,
) -> i32 {
    let trng = (rng as *mut u8).sub(core::mem::offset_of!(HisiTrng, rng)) as *mut HisiTrng;
    let mut currsize: usize = 0;
    let mut val: u32 = 0;
    let ret: i32;

    loop {
        ret = readl_poll_timeout(
            ((*trng).base as *mut u8).add(HISI_TRNG_REG) as *mut core::ffi::c_void,
            &mut val,
            val,
            SLEEP_US,
            TIMEOUT_US,
        );
        if ret != 0 {
            return currsize as i32;
        }

        if max - currsize >= HISI_TRNG_BYTES {
            core::ptr::copy_nonoverlapping(
                &val as *const u32 as *const u8,
                (buf as *mut u8).add(currsize),
                HISI_TRNG_BYTES,
            );
            currsize += HISI_TRNG_BYTES;
            if currsize == max {
                return currsize as i32;
            }
            continue;
        }

        /* copy remaining bytes */
        core::ptr::copy_nonoverlapping(
            &val as *const u32 as *const u8,
            (buf as *mut u8).add(currsize),
            max - currsize,
        );
        currsize = max;
    }
}

unsafe extern "C" fn hisi_trng_probe(pdev: *mut PlatformDevice) -> i32 {
    let trng = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<HisiTrng>(),
        GFP_KERNEL,
    ) as *mut HisiTrng;
    if trng.is_null() {
        return -12; // -ENOMEM
    }

    (*trng).base = devm_platform_ioremap_resource(pdev, 0);
    if ((*trng).base as isize) < 0 {
        return ptr_err((*trng).base);
    }

    (*trng).rng.name = (*pdev).name;
    (*trng).rng.read = Some(hisi_trng_read);
    (*trng).rng.quality = HISI_TRNG_QUALITY;

    let ret = devm_hwrng_register(&mut (*pdev).dev, &mut (*trng).rng);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"failed to register hwrng: %d!\n\0".as_ptr() as *const core::ffi::c_char,
            ret,
        );
    }
    ret
}

static HISI_TRNG_ACPI_MATCH: [AcpiDeviceId; 2] = [
    AcpiDeviceId { id: b"HISI02B3\0".as_ptr() as *const core::ffi::c_char },
    AcpiDeviceId { id: core::ptr::null() },
];

static mut HISI_TRNG_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(hisi_trng_probe),
    driver: Driver {
        name: b"hisi-trng-v2\0".as_ptr() as *const core::ffi::c_char,
        acpi_match_table: HISI_TRNG_ACPI_MATCH.as_ptr(),
    },
};

#[used]
static HISI_TRNG_MODULE_DEVICE_TABLE: *const AcpiDeviceId = HISI_TRNG_ACPI_MATCH.as_ptr();

#[used]
static HISI_TRNG_MODULE: () = {
    // module_platform_driver(hisi_trng_driver)
    // MODULE_LICENSE("GPL v2")
    // MODULE_AUTHOR("Weili Qian <qianweili@huawei.com>")
    // MODULE_AUTHOR("Zaibo Xu <xuzaibo@huawei.com>")
    // MODULE_DESCRIPTION("HiSilicon true random number generator V2 driver")
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
