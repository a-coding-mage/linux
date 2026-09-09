// SPDX-License-Identifier: GPL-2.0-only

// Linux clock, mutex, platform-device, PM-domain, OPP, runtime-PM, slab,
// Tegra common, and local clk dependencies are supplied externally.

#[repr(C)]
struct TegraClkDevice {
    clk_nb: notifier_block,
    dev: *mut device,
    hw: *mut clk_hw,
    lock: mutex,
}

unsafe fn tegra_clock_set_pd_state(clk_dev: *mut TegraClkDevice, mut rate: c_ulong) -> c_int {
    let dev = (*clk_dev).dev;
    let mut opp: *mut dev_pm_opp;
    let pstate: c_uint;

    opp = dev_pm_opp_find_freq_ceil(dev, &mut rate);
    if opp == ERR_PTR(-ERANGE) {
        /*
         * Some clocks may be unused by a particular board and they
         * may have uninitiated clock rate that is overly high.  In
         * this case clock is expected to be disabled, but still we
         * need to set up performance state of the power domain and
         * not error out clk initialization.  A typical example is
         * a PCIe clock on Android tablets.
         */
        dev_dbg(dev, "failed to find ceil OPP for %luHz\n", rate);
        opp = dev_pm_opp_find_freq_floor(dev, &mut rate);
    }

    if IS_ERR(opp) {
        dev_err(dev, "failed to find OPP for %luHz: %pe\n", rate, opp);
        return PTR_ERR(opp);
    }

    pstate = dev_pm_opp_get_required_pstate(opp, 0);
    dev_pm_opp_put(opp);

    dev_pm_genpd_set_performance_state(dev, pstate)
}

unsafe extern "C" fn tegra_clock_change_notify(
    nb: *mut notifier_block,
    msg: c_ulong,
    data: *mut c_void,
) -> c_int {
    let cnd = data as *mut clk_notifier_data;
    let clk_dev: *mut TegraClkDevice;
    let mut err: c_int = 0;

    clk_dev = container_of(nb, TegraClkDevice, clk_nb);

    mutex_lock(&mut (*clk_dev).lock);
    match msg {
        PRE_RATE_CHANGE => {
            if (*cnd).new_rate > (*cnd).old_rate {
                err = tegra_clock_set_pd_state(clk_dev, (*cnd).new_rate);
            }
        }
        ABORT_RATE_CHANGE => {
            err = tegra_clock_set_pd_state(clk_dev, (*cnd).old_rate);
        }
        POST_RATE_CHANGE => {
            if (*cnd).new_rate < (*cnd).old_rate {
                err = tegra_clock_set_pd_state(clk_dev, (*cnd).new_rate);
            }
        }
        _ => {}
    }
    mutex_unlock(&mut (*clk_dev).lock);

    notifier_from_errno(err)
}

unsafe fn tegra_clock_sync_pd_state(clk_dev: *mut TegraClkDevice) -> c_int {
    let rate: c_ulong;
    let ret: c_int;

    mutex_lock(&mut (*clk_dev).lock);

    rate = clk_hw_get_rate((*clk_dev).hw);
    ret = tegra_clock_set_pd_state(clk_dev, rate);

    mutex_unlock(&mut (*clk_dev).lock);

    ret
}

unsafe extern "C" fn tegra_clock_probe(pdev: *mut platform_device) -> c_int {
    let mut opp_params: tegra_core_opp_params = core::mem::zeroed();
    let mut clk_dev: *mut TegraClkDevice;
    let dev: *mut device = &mut (*pdev).dev;
    let mut clk: *mut clk;
    let mut err: c_int;

    if (*dev).pm_domain.is_null() {
        return -EINVAL;
    }

    clk_dev = devm_kzalloc(dev, core::mem::size_of::<TegraClkDevice>(), GFP_KERNEL)
        as *mut TegraClkDevice;
    if clk_dev.is_null() {
        return -ENOMEM;
    }

    clk = devm_clk_get(dev, core::ptr::null());
    if IS_ERR(clk) {
        return PTR_ERR(clk);
    }

    (*clk_dev).dev = dev;
    (*clk_dev).hw = __clk_get_hw(clk);
    (*clk_dev).clk_nb.notifier_call = Some(tegra_clock_change_notify);
    mutex_init(&mut (*clk_dev).lock);

    platform_set_drvdata(pdev, clk_dev as *mut c_void);

    /*
     * Runtime PM was already enabled for this device by the parent clk
     * driver and power domain state should be synced under clk_dev lock,
     * hence we don't use the common OPP helper that initializes OPP
     * state. For some clocks common OPP helper may fail to find ceil
     * rate, it's handled by this driver.
     */
    err = devm_tegra_core_dev_init_opp_table(dev, &mut opp_params);
    if err != 0 {
        return err;
    }

    err = clk_notifier_register(clk, &mut (*clk_dev).clk_nb);
    if err != 0 {
        dev_err(dev, "failed to register clk notifier: %d\n", err);
        return err;
    }

    /*
     * The driver is attaching to a potentially active/resumed clock, hence
     * we need to sync the power domain performance state in a accordance to
     * the clock rate if clock is resumed.
     */
    err = tegra_clock_sync_pd_state(clk_dev);
    if err != 0 {
        clk_notifier_unregister(clk, &mut (*clk_dev).clk_nb);
        return err;
    }

    0
}

/*
 * Tegra GENPD driver enables clocks during NOIRQ phase. It can't be done
 * for clocks served by this driver because runtime PM is unavailable in
 * NOIRQ phase. We will keep clocks resumed during suspend to mitigate this
 * problem. In practice this makes no difference from a power management
 * perspective since voltage is kept at a nominal level during suspend anyways.
 */
unsafe extern "C" fn tegra_clock_suspend(dev: *mut device) -> c_int {
    let ret = pm_runtime_resume(dev);
    if ret < 0 {
        return ret;
    }

    0
}

static tegra_clock_pm: dev_pm_ops = dev_pm_ops {
    SET_SYSTEM_SLEEP_PM_OPS(tegra_clock_suspend, None)
};

static tegra_clock_match: [of_device_id; 6] = [
    of_device_id { compatible: c"nvidia,tegra20-sclk".as_ptr() },
    of_device_id { compatible: c"nvidia,tegra30-sclk".as_ptr() },
    of_device_id { compatible: c"nvidia,tegra30-pllc".as_ptr() },
    of_device_id { compatible: c"nvidia,tegra30-plle".as_ptr() },
    of_device_id { compatible: c"nvidia,tegra30-pllm".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

static mut tegra_clock_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"tegra-clock".as_ptr(),
        of_match_table: tegra_clock_match.as_ptr(),
        pm: &tegra_clock_pm,
        suppress_bind_attrs: true,
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(tegra_clock_probe),
    ..unsafe { core::mem::zeroed() }
};

builtin_platform_driver!(tegra_clock_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
