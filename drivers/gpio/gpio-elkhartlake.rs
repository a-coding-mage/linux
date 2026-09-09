// SPDX-License-Identifier: GPL-2.0-only
/*
 * Intel Elkhart Lake PSE GPIO driver
 *
 * Copyright (c) 2023, 2025 Intel Corporation.
 *
 * Authors: Pandith N <pandith.n@intel.com>
 *          Raag Jadav <raag.jadav@intel.com>
 */

// External Linux kernel and GPIO Tangier declarations are supplied by other
// translation units.

/* Each Intel EHL PSE GPIO Controller has 30 GPIO pins */
const EHL_PSE_NGPIO: i32 = 30;

unsafe fn ehl_gpio_probe(
    adev: *mut auxiliary_device,
    _id: *const auxiliary_device_id,
) -> i32 {
    let dev: *mut device = unsafe { &mut (*adev).dev };
    let mut data: *mut ehl_pse_io_data;
    let mut priv_: *mut tng_gpio;
    let mut ret: i32;

    data = unsafe { dev_get_platdata(dev) };
    if data.is_null() {
        return -ENODATA;
    }

    priv_ = unsafe { devm_kzalloc(dev, core::mem::size_of::<tng_gpio>(), GFP_KERNEL) }
        as *mut tng_gpio;
    if priv_.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*priv_).reg_base = devm_ioremap_resource(dev, &mut (*data).mem);
    }
    if unsafe { IS_ERR((*priv_).reg_base) } {
        return unsafe { PTR_ERR((*priv_).reg_base) };
    }

    unsafe {
        (*priv_).dev = dev;
        (*priv_).irq = (*data).irq;

        (*priv_).info.base = -1;
        (*priv_).info.ngpio = EHL_PSE_NGPIO;

        (*priv_).wake_regs.gwmr = GWMR_EHL;
        (*priv_).wake_regs.gwsr = GWSR_EHL;
        (*priv_).wake_regs.gsir = GSIR_EHL;

        ret = devm_tng_gpio_probe(dev, priv_);
    }
    if ret != 0 {
        return unsafe { dev_err_probe(dev, ret, c"tng_gpio_probe error\n".as_ptr()) };
    }

    unsafe { auxiliary_set_drvdata(adev, priv_ as *mut core::ffi::c_void) };
    0
}

static EHL_GPIO_IDS: [auxiliary_device_id; 2] = [
    auxiliary_device_id {
        name: concat!(EHL_PSE_IO_NAME, ".", EHL_PSE_GPIO_NAME),
    },
    auxiliary_device_id { name: "" },
];

static mut EHL_GPIO_DRIVER: auxiliary_driver = auxiliary_driver {
    driver: device_driver {
        pm: pm_sleep_ptr(&tng_gpio_pm_ops),
    },
    probe: Some(ehl_gpio_probe),
    id_table: EHL_GPIO_IDS.as_ptr(),
};

// MODULE_DEVICE_TABLE(auxiliary, ehl_gpio_ids);
// module_auxiliary_driver(ehl_gpio_driver);
// MODULE_AUTHOR("Pandith N <pandith.n@intel.com>");
// MODULE_AUTHOR("Raag Jadav <raag.jadav@intel.com>");
// MODULE_DESCRIPTION("Intel Elkhart Lake PSE GPIO driver");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("GPIO_TANGIER");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
