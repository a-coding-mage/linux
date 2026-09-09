// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2011-2014 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com
 *
 * Coupled cpuidle support based on the work of:
 *	Colin Cross <ccross@android.com>
 *	Daniel Lezcano <daniel.lezcano@linaro.org>
 */

// Dependencies supplied by the surrounding kernel translation.

static mut exynos_idle_barrier: atomic_t = atomic_t { counter: 0 };

static mut exynos_cpuidle_pdata: *mut cpuidle_exynos_data = core::ptr::null_mut();
static mut exynos_enter_aftr: Option<unsafe extern "C" fn()> = None;

unsafe extern "C" fn exynos_enter_coupled_lowpower(
    dev: *mut cpuidle_device,
    drv: *mut cpuidle_driver,
    mut index: i32,
) -> i32 {
    (*exynos_cpuidle_pdata).pre_enter_aftr();

    /*
     * Waiting all cpus to reach this point at the same moment
     */
    cpuidle_coupled_parallel_barrier(dev, &mut exynos_idle_barrier);

    /*
     * Both cpus will reach this point at the same time
     */
    let ret = if (*dev).cpu != 0 {
        ((*exynos_cpuidle_pdata).cpu1_powerdown)()
    } else {
        ((*exynos_cpuidle_pdata).cpu0_enter_aftr)()
    };
    if ret != 0 {
        index = ret;
    }

    /*
     * Waiting all cpus to finish the power sequence before going further
     */
    cpuidle_coupled_parallel_barrier(dev, &mut exynos_idle_barrier);

    (*exynos_cpuidle_pdata).post_enter_aftr();

    index
}

unsafe extern "C" fn exynos_enter_lowpower(
    dev: *mut cpuidle_device,
    drv: *mut cpuidle_driver,
    index: i32,
) -> i32 {
    let mut new_index = index;

    /* AFTR can only be entered when cores other than CPU0 are offline */
    if num_online_cpus() > 1 || (*dev).cpu != 0 {
        new_index = (*drv).safe_state_index;
    }

    if new_index == 0 {
        return arm_cpuidle_simple_enter(dev, drv, new_index);
    }

    if let Some(enter_aftr) = exynos_enter_aftr {
        enter_aftr();
    }

    new_index
}

static mut exynos_idle_driver: cpuidle_driver = cpuidle_driver {
    name: "exynos_idle" as *const str,
    owner: THIS_MODULE,
    states: [
        ARM_CPUIDLE_WFI_STATE,
        cpuidle_state {
            enter: Some(exynos_enter_lowpower),
            exit_latency: 300,
            target_residency: 10000,
            flags: 0,
            name: "C1",
            desc: "ARM power down",
        },
    ],
    state_count: 2,
    safe_state_index: 0,
};

static mut exynos_coupled_idle_driver: cpuidle_driver = cpuidle_driver {
    name: "exynos_coupled_idle" as *const str,
    owner: THIS_MODULE,
    states: [
        ARM_CPUIDLE_WFI_STATE,
        cpuidle_state {
            enter: Some(exynos_enter_coupled_lowpower),
            exit_latency: 5000,
            target_residency: 10000,
            flags: CPUIDLE_FLAG_COUPLED | CPUIDLE_FLAG_TIMER_STOP,
            name: "C1",
            desc: "ARM power down",
        },
    ],
    state_count: 2,
    safe_state_index: 0,
};

unsafe extern "C" fn exynos_cpuidle_probe(pdev: *mut platform_device) -> i32 {
    let ret: i32;

    // CONFIG_SMP is a build-time condition supplied by the surrounding build.
    if IS_ENABLED(CONFIG_SMP)
        && (of_machine_is_compatible("samsung,exynos4210")
            || of_machine_is_compatible("samsung,exynos3250"))
    {
        exynos_cpuidle_pdata = (*pdev).dev.platform_data as *mut cpuidle_exynos_data;

        ret = cpuidle_register(
            &mut exynos_coupled_idle_driver,
            cpu_possible_mask,
        );
    } else {
        exynos_enter_aftr = Some(core::mem::transmute((*pdev).dev.platform_data));

        ret = cpuidle_register(&mut exynos_idle_driver, core::ptr::null_mut());
    }

    if ret != 0 {
        dev_err(&(*pdev).dev, "failed to register cpuidle driver\n");
        return ret;
    }

    0
}

static mut exynos_cpuidle_driver: platform_driver = platform_driver {
    probe: Some(exynos_cpuidle_probe),
    driver: device_driver {
        name: "exynos_cpuidle",
    },
};

builtin_platform_driver!(exynos_cpuidle_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
