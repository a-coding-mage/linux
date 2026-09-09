// SPDX-License-Identifier: GPL-2.0
/*
 * Raspberry Pi cpufreq driver
 *
 * Copyright (C) 2019, Nicolas Saenz Julienne <nsaenzjulienne@suse.de>
 */

// Dependencies supplied by the kernel environment.

const RASPBERRYPI_FREQ_INTERVAL: c_ulong = 100000000;

static mut CPUFREQ_DT: *mut platform_device = core::ptr::null_mut();

unsafe fn raspberrypi_cpufreq_probe(pdev: *mut platform_device) -> c_int {
    let mut cpu_dev: *mut device;
    let mut min: c_ulong;
    let mut max: c_ulong;
    let mut rate: c_ulong;
    let clk: *mut clk;
    let mut ret: c_int;

    cpu_dev = get_cpu_device(0);
    if cpu_dev.is_null() {
        pr_err!("Cannot get CPU for cpufreq driver\\n");
        return -ENODEV;
    }

    clk = clk_get(cpu_dev, core::ptr::null());
    if IS_ERR(clk) {
        dev_err!(cpu_dev, "Cannot get clock for CPU0\\n");
        return PTR_ERR(clk);
    }

    /*
     * The max and min frequencies are configurable in the Raspberry Pi
     * firmware, so we query them at runtime.
     */
    min = roundup(clk_round_rate(clk, 0), RASPBERRYPI_FREQ_INTERVAL);
    max = roundup(clk_round_rate(clk, ULONG_MAX), RASPBERRYPI_FREQ_INTERVAL);
    clk_put(clk);

    rate = min;
    while rate <= max {
        ret = dev_pm_opp_add(cpu_dev, rate, 0);
        if ret != 0 {
            break;
        }
        rate = rate.wrapping_add(RASPBERRYPI_FREQ_INTERVAL);
    }

    if rate <= max {
        dev_pm_opp_remove_all_dynamic(cpu_dev);
        return ret;
    }

    CPUFREQ_DT = platform_device_register_simple(
        c"cpufreq-dt".as_ptr(), -1, core::ptr::null_mut(), 0,
    );
    ret = PTR_ERR_OR_ZERO(CPUFREQ_DT);
    if ret != 0 {
        dev_err!(cpu_dev, "Failed to create platform device, %d\\n", ret);
        dev_pm_opp_remove_all_dynamic(cpu_dev);
        return ret;
    }

    0
}

unsafe fn raspberrypi_cpufreq_remove(pdev: *mut platform_device) {
    let cpu_dev: *mut device;

    cpu_dev = get_cpu_device(0);
    if !cpu_dev.is_null() {
        dev_pm_opp_remove_all_dynamic(cpu_dev);
    }

    platform_device_unregister(CPUFREQ_DT);
}

/*
 * Since the driver depends on clk-raspberrypi, which may return EPROBE_DEFER,
 * all the activity is performed in the probe, which may be defered as well.
 */
static mut raspberrypi_cpufreq_driver: platform_driver = platform_driver {
    driver: driver {
        name: c"raspberrypi-cpufreq".as_ptr(),
    },
    probe: Some(raspberrypi_cpufreq_probe),
    remove: Some(raspberrypi_cpufreq_remove),
};

// Equivalent of module_platform_driver(raspberrypi_cpufreq_driver).
module_platform_driver!(raspberrypi_cpufreq_driver);

module_author!(c"Nicolas Saenz Julienne <nsaenzjulienne@suse.de");
module_description!(c"Raspberry Pi cpufreq driver");
module_license!(c"GPL");
module_alias!(c"platform:raspberrypi-cpufreq");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
