/*
 * Copyright (c) 2011 Peter Korsgaard <jacmet@sunsite.dk>
 *
 * This file is licensed under  the terms of the GNU General Public
 * License version 2. This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const TRNG_CR: usize = 0x00;
const TRNG_MR: usize = 0x04;
const TRNG_ISR: usize = 0x1c;
const TRNG_ISR_DATRDY: u32 = 1 << 0;
const TRNG_ODATA: usize = 0x50;

const TRNG_KEY: u32 = 0x524e4700; // RNG

const TRNG_HALFR: u32 = 1 << 0; /* generate RN every 168 cycles */

#[repr(C)]
struct atmel_trng_data {
    has_half_rate: bool,
}

#[repr(C)]
struct atmel_trng {
    clk: *mut clk,
    base: *mut core::ffi::c_void,
    rng: hwrng,
    dev: *mut device,
    has_half_rate: bool,
}

unsafe fn atmel_trng_wait_ready(trng: *mut atmel_trng, wait: bool) -> bool {
    let mut ready: i32;

    ready = readl((*trng).base.add(TRNG_ISR) as *const core::ffi::c_void) as i32
        & TRNG_ISR_DATRDY as i32;
    if ready == 0 && wait {
        readl_poll_timeout(
            (*trng).base.add(TRNG_ISR) as *const core::ffi::c_void,
            &mut ready,
            ready & TRNG_ISR_DATRDY as i32,
            1000,
            20000,
        );
    }

    ready != 0
}

unsafe fn atmel_trng_read(rng: *mut hwrng, buf: *mut core::ffi::c_void, max: usize, wait: bool) -> i32 {
    let trng = container_of!(rng, atmel_trng, rng);
    let data = buf as *mut u32;
    let mut ret: i32;

    ret = pm_runtime_get_sync((*trng).dev);
    if ret < 0 {
        pm_runtime_put_sync((*trng).dev);
        return ret;
    }

    ret = if atmel_trng_wait_ready(trng, wait) { 1 } else { 0 };
    if ret == 0 {
        pm_runtime_put_sync_autosuspend((*trng).dev);
        return ret;
    }

    *data = readl((*trng).base.add(TRNG_ODATA) as *const core::ffi::c_void);
    /*
     * ensure data ready is only set again AFTER the next data word is ready
     * in case it got set between checking ISR and reading ODATA, so we
     * don't risk re-reading the same word
     */
    readl((*trng).base.add(TRNG_ISR) as *const core::ffi::c_void);
    ret = 4;

    pm_runtime_put_sync_autosuspend((*trng).dev);
    ret
}

unsafe fn atmel_trng_init(trng: *mut atmel_trng) -> i32 {
    let mut rate: c_ulong;
    let ret = clk_prepare_enable((*trng).clk);
    if ret != 0 {
        return ret;
    }

    if (*trng).has_half_rate {
        rate = clk_get_rate((*trng).clk);

        /* if peripheral clk is above 100MHz, set HALFR */
        if rate > 100000000 {
            writel(TRNG_HALFR, (*trng).base.add(TRNG_MR));
        }
    }

    writel(TRNG_KEY | 1, (*trng).base.add(TRNG_CR));
    0
}

unsafe fn atmel_trng_cleanup(trng: *mut atmel_trng) {
    writel(TRNG_KEY, (*trng).base.add(TRNG_CR));
    clk_disable_unprepare((*trng).clk);
}

unsafe fn atmel_trng_probe(pdev: *mut platform_device) -> i32 {
    let trng = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<atmel_trng>(), GFP_KERNEL) as *mut atmel_trng;
    if trng.is_null() {
        return -ENOMEM;
    }

    (*trng).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*trng).base) {
        return PTR_ERR((*trng).base);
    }

    (*trng).clk = devm_clk_get(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR((*trng).clk) {
        return PTR_ERR((*trng).clk);
    }
    let data = of_device_get_match_data(&mut (*pdev).dev) as *const atmel_trng_data;
    if data.is_null() {
        return -ENODEV;
    }

    (*trng).has_half_rate = (*data).has_half_rate;
    (*trng).dev = &mut (*pdev).dev;
    (*trng).rng.name = (*pdev).name;
    (*trng).rng.read = Some(atmel_trng_read);
    platform_set_drvdata(pdev, trng as *mut core::ffi::c_void);

    // The contained code is compiled when CONFIG_PM is not enabled.
    #[cfg(not(CONFIG_PM))]
    {
        let ret = atmel_trng_init(trng);
        if ret != 0 {
            return ret;
        }
    }

    pm_runtime_set_autosuspend_delay(&mut (*pdev).dev, 100);
    pm_runtime_use_autosuspend(&mut (*pdev).dev);
    pm_runtime_enable(&mut (*pdev).dev);

    let ret = devm_hwrng_register(&mut (*pdev).dev, &mut (*trng).rng);
    if ret != 0 {
        pm_runtime_disable(&mut (*pdev).dev);
        pm_runtime_set_suspended(&mut (*pdev).dev);
        #[cfg(not(CONFIG_PM))]
        atmel_trng_cleanup(trng);
    }

    ret
}

unsafe fn atmel_trng_remove(pdev: *mut platform_device) {
    let trng = platform_get_drvdata(pdev) as *mut atmel_trng;
    atmel_trng_cleanup(trng);
    pm_runtime_disable(&mut (*pdev).dev);
    pm_runtime_set_suspended(&mut (*pdev).dev);
}

unsafe fn atmel_trng_runtime_suspend(dev: *mut device) -> i32 {
    let trng = dev_get_drvdata(dev) as *mut atmel_trng;
    atmel_trng_cleanup(trng);
    0
}

unsafe fn atmel_trng_runtime_resume(dev: *mut device) -> i32 {
    let trng = dev_get_drvdata(dev) as *mut atmel_trng;
    atmel_trng_init(trng)
}

static atmel_trng_pm_ops: dev_pm_ops = dev_pm_ops {
    // SET_RUNTIME_PM_OPS(atmel_trng_runtime_suspend, atmel_trng_runtime_resume, NULL)
    // SET_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
};

static at91sam9g45_config: atmel_trng_data = atmel_trng_data {
    has_half_rate: false,
};

static sam9x60_config: atmel_trng_data = atmel_trng_data {
    has_half_rate: true,
};

static atmel_trng_dt_ids: [of_device_id; 3] = [
    of_device_id {
        compatible: "atmel,at91sam9g45-trng",
        data: &at91sam9g45_config as *const _ as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: "microchip,sam9x60-trng",
        data: &sam9x60_config as *const _ as *const core::ffi::c_void,
    },
    of_device_id { /* sentinel */ },
];

static mut atmel_trng_driver: platform_driver = platform_driver {
    probe: Some(atmel_trng_probe),
    remove: Some(atmel_trng_remove),
    driver: device_driver {
        name: "atmel-trng",
        pm: pm_ptr!(&atmel_trng_pm_ops),
        of_match_table: atmel_trng_dt_ids.as_ptr(),
    },
};

module_platform_driver!(atmel_trng_driver);

module_license!("GPL");
module_author!("Peter Korsgaard <jacmet@sunsite.dk>");
module_description!("Atmel true random number generator driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
