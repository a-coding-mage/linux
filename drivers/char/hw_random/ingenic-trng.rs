// SPDX-License-Identifier: GPL-2.0
/*
 * Ingenic True Random Number Generator driver
 * Copyright (c) 2019 漆鹏振 (Qi Pengzhen) <aric.pzqi@ingenic.com>
 * Copyright (c) 2020 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
 */

// Dependencies supplied by the surrounding kernel bindings.

/* DTRNG register offsets */
const TRNG_REG_CFG_OFFSET: usize = 0x00;
const TRNG_REG_RANDOMNUM_OFFSET: usize = 0x04;
const TRNG_REG_STATUS_OFFSET: usize = 0x08;

/* bits within the CFG register */
const CFG_GEN_EN: u32 = 1 << 0;

/* bits within the STATUS register */
const STATUS_RANDOM_RDY: u32 = 1 << 0;

#[repr(C)]
struct IngenicTrng {
    base: *mut core::ffi::c_void,
    rng: Hwrng,
}

unsafe fn ingenic_trng_init(rng: *mut Hwrng) -> i32 {
    let trng = container_of!(rng, IngenicTrng, rng);
    let mut ctrl: u32;

    ctrl = readl(unsafe { (trng.base as *mut u8).add(TRNG_REG_CFG_OFFSET) });
    ctrl |= CFG_GEN_EN;
    writel(
        ctrl,
        unsafe { (trng.base as *mut u8).add(TRNG_REG_CFG_OFFSET) },
    );

    0
}

unsafe fn ingenic_trng_cleanup(rng: *mut Hwrng) {
    let trng = container_of!(rng, IngenicTrng, rng);
    let mut ctrl: u32;

    ctrl = readl(unsafe { (trng.base as *mut u8).add(TRNG_REG_CFG_OFFSET) });
    ctrl &= !CFG_GEN_EN;
    writel(
        ctrl,
        unsafe { (trng.base as *mut u8).add(TRNG_REG_CFG_OFFSET) },
    );
}

unsafe fn ingenic_trng_read(
    rng: *mut Hwrng,
    buf: *mut core::ffi::c_void,
    _max: usize,
    _wait: bool,
) -> i32 {
    let trng = container_of!(rng, IngenicTrng, rng);
    let data = buf as *mut u32;
    let mut status: u32 = 0;
    let ret: i32;

    ret = readl_poll_timeout(
        unsafe { (trng.base as *mut u8).add(TRNG_REG_STATUS_OFFSET) },
        &mut status,
        status & STATUS_RANDOM_RDY,
        10,
        1000,
    );
    if ret == -ETIMEDOUT {
        pr_err!("{}: Wait for DTRNG data ready timeout\n", "ingenic_trng_read");
        return ret;
    }

    *data = readl(unsafe { (trng.base as *mut u8).add(TRNG_REG_RANDOMNUM_OFFSET) });

    4
}

unsafe fn ingenic_trng_probe(pdev: *mut PlatformDevice) -> i32 {
    let trng: *mut IngenicTrng;
    let clk: *mut Clk;
    let ret: i32;

    trng = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<IngenicTrng>(),
        GFP_KERNEL,
    ) as *mut IngenicTrng;
    if trng.is_null() {
        return -ENOMEM;
    }

    (*trng).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR!((*trng).base) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR!((*trng).base),
            "{}: Failed to map DTRNG registers\n",
            "ingenic_trng_probe",
        );
    }

    clk = devm_clk_get_enabled(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR!(clk) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR!(clk),
            "{}: Cannot get and enable DTRNG clock\n",
            "ingenic_trng_probe",
        );
    }

    (*trng).rng.name = (*pdev).name;
    (*trng).rng.init = Some(ingenic_trng_init);
    (*trng).rng.cleanup = Some(ingenic_trng_cleanup);
    (*trng).rng.read = Some(ingenic_trng_read);

    ret = devm_hwrng_register(&mut (*pdev).dev, &mut (*trng).rng);
    if ret != 0 {
        return dev_err_probe(&mut (*pdev).dev, ret, "Failed to register hwrng\n");
    }

    platform_set_drvdata(pdev, trng as *mut core::ffi::c_void);

    dev_info!(&mut (*pdev).dev, "Ingenic DTRNG driver registered\n");
    0
}

static INGENIC_TRNG_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"ingenic,x1830-dtrng\0".as_ptr(),
    },
    OfDeviceId { compatible: core::ptr::null() }, // sentinel
];

static mut INGENIC_TRNG_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(ingenic_trng_probe),
    driver: Driver {
        name: b"ingenic-trng\0".as_ptr(),
        of_match_table: INGENIC_TRNG_OF_MATCH.as_ptr(),
    },
};

module_platform_driver!(INGENIC_TRNG_DRIVER);

module_license!("GPL");
module_author!("漆鹏振 (Qi Pengzhen) <aric.pzqi@ingenic.com>");
module_author!("周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>");
module_description!("Ingenic True Random Number Generator driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
