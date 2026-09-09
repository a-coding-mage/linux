// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2017-18 Linaro Limited
//
// Based on msm-rng.c and downstream driver

// Dependencies supplied by the surrounding kernel translation.

/* Device specific register offsets */
const PRNG_DATA_OUT: usize = 0x0000;
const PRNG_STATUS: usize = 0x0004;
const PRNG_LFSR_CFG: usize = 0x0100;
const PRNG_CONFIG: usize = 0x0104;

/* Device specific register masks and config values */
const PRNG_LFSR_CFG_MASK: u32 = 0x0000ffff;
const PRNG_LFSR_CFG_CLOCKS: u32 = 0x0000dddd;
const PRNG_CONFIG_HW_ENABLE: u32 = 1 << 1;
const PRNG_STATUS_DATA_AVAIL: u32 = 1 << 0;

const WORD_SZ: usize = 4;
const QCOM_TRNG_QUALITY: i32 = 1024;

#[repr(C)]
pub struct qcom_rng {
    pub base: *mut core::ffi::c_void,
    pub clk: *mut clk,
    pub hwrng: hwrng,
}

#[repr(C)]
pub struct qcom_rng_match_data {
    pub hwrng_support: bool,
}

unsafe fn qcom_rng_read(rng: *mut qcom_rng, data: *mut u8, max: usize) -> i32 {
    let mut currsize: usize = 0;
    let mut val: u32;
    let mut ret: i32;

    /* read random data from hardware */
    loop {
        ret = readl_poll_timeout(
            (*rng).base.add(PRNG_STATUS),
            &mut val,
            val & PRNG_STATUS_DATA_AVAIL != 0,
            200,
            10000,
        );
        if ret != 0 {
            return ret;
        }

        val = readl_relaxed((*rng).base.add(PRNG_DATA_OUT));

        if max - currsize >= WORD_SZ {
            core::ptr::copy_nonoverlapping(
                &val as *const u32 as *const u8,
                data,
                WORD_SZ,
            );
            data = data.add(WORD_SZ);
            currsize += WORD_SZ;
        } else {
            /* copy only remaining bytes */
            core::ptr::copy_nonoverlapping(
                &val as *const u32 as *const u8,
                data,
                max - currsize,
            );
            currsize = max;
        }

        if currsize >= max {
            break;
        }
    }

    currsize as i32
}

unsafe extern "C" fn qcom_hwrng_init(hwrng: *mut hwrng) -> i32 {
    let qrng = container_of!(hwrng, qcom_rng, hwrng);
    clk_prepare_enable((*qrng).clk)
}

unsafe extern "C" fn qcom_hwrng_read(
    hwrng: *mut hwrng,
    data: *mut core::ffi::c_void,
    max: usize,
    _wait: bool,
) -> i32 {
    let qrng = container_of!(hwrng, qcom_rng, hwrng);
    qcom_rng_read(qrng, data as *mut u8, max)
}

unsafe extern "C" fn qcom_hwrng_cleanup(hwrng: *mut hwrng) {
    let qrng = container_of!(hwrng, qcom_rng, hwrng);
    clk_disable_unprepare((*qrng).clk);
}

unsafe extern "C" fn qcom_rng_probe(pdev: *mut platform_device) -> i32 {
    let match_data: *const qcom_rng_match_data;
    let rng: *mut qcom_rng;
    let mut ret: i32;

    match_data = device_get_match_data(&mut (*pdev).dev);
    if match_data.is_null() || !(*match_data).hwrng_support {
        dev_info(&mut (*pdev).dev, "TRNG support not detected\n");
        /*
         * In this case the driver does nothing except the dev_info(),
         * but bind the device anyway to avoid effects on GCC state.
         */
        return 0;
    }

    rng = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<qcom_rng>(), GFP_KERNEL);
    if rng.is_null() {
        return -12;
    }

    (*rng).base = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*rng).base) {
        return ptr_err((*rng).base);
    }

    (*rng).clk = devm_clk_get_optional(&mut (*pdev).dev, "core");
    if is_err((*rng).clk as *mut core::ffi::c_void) {
        return ptr_err((*rng).clk as *mut core::ffi::c_void);
    }

    (*rng).hwrng.name = "qcom_hwrng";
    (*rng).hwrng.init = Some(qcom_hwrng_init);
    (*rng).hwrng.read = Some(qcom_hwrng_read);
    (*rng).hwrng.cleanup = Some(qcom_hwrng_cleanup);
    (*rng).hwrng.quality = QCOM_TRNG_QUALITY;
    ret = devm_hwrng_register(&mut (*pdev).dev, &mut (*rng).hwrng);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, "Register hwrng failed: %d\n", ret);
    }
    ret
}

static mut qcom_prng_match_data: qcom_rng_match_data = qcom_rng_match_data {
    hwrng_support: false,
};

static mut qcom_prng_ee_match_data: qcom_rng_match_data = qcom_rng_match_data {
    hwrng_support: false,
};

static mut qcom_trng_match_data: qcom_rng_match_data = qcom_rng_match_data {
    hwrng_support: true,
};

static qcom_rng_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: "QCOM8160", driver_data: &qcom_prng_ee_match_data as *const _ as usize },
    acpi_device_id { id: "", driver_data: 0 },
];

static qcom_rng_of_match: [of_device_id; 4] = [
    of_device_id { compatible: "qcom,prng", data: &qcom_prng_match_data },
    of_device_id { compatible: "qcom,prng-ee", data: &qcom_prng_ee_match_data },
    of_device_id { compatible: "qcom,trng", data: &qcom_trng_match_data },
    of_device_id { compatible: "", data: core::ptr::null() },
];

static mut qcom_rng_driver: platform_driver = platform_driver {
    probe: Some(qcom_rng_probe),
    driver: device_driver {
        name: KBUILD_MODNAME,
        of_match_table: &qcom_rng_of_match,
        acpi_match_table: ACPI_PTR!(qcom_rng_acpi_match),
    },
};

// module_platform_driver!(qcom_rng_driver);
// MODULE_ALIAS!("platform:" KBUILD_MODNAME);
// MODULE_DESCRIPTION!("Qualcomm random number generator driver");
// MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
