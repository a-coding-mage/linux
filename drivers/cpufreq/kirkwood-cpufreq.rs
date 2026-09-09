// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * kirkwood_freq.c: cpufreq driver for the Marvell kirkwood
 *
 * Copyright (C) 2013 Andrew Lunn <andrew@lunn.ch>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const CPU_SW_INT_BLK: c_ulong = 1 << 28;

#[repr(C)]
struct Priv {
    cpu_clk: *mut clk,
    ddr_clk: *mut clk,
    powersave_clk: *mut clk,
    dev: *mut device,
    base: *mut core::ffi::c_void,
}

static mut priv_: Priv = Priv {
    cpu_clk: core::ptr::null_mut(),
    ddr_clk: core::ptr::null_mut(),
    powersave_clk: core::ptr::null_mut(),
    dev: core::ptr::null_mut(),
    base: core::ptr::null_mut(),
};

const STATE_CPU_FREQ: u32 = 0x01;
const STATE_DDR_FREQ: u32 = 0x02;

/*
 * Kirkwood can swap the clock to the CPU between two clocks:
 *
 * - cpu clk
 * - ddr clk
 *
 * The frequencies are set at runtime before registering this table.
 */
static mut kirkwood_freq_table: [cpufreq_frequency_table; 3] = [
    cpufreq_frequency_table { flags: 0, driver_data: STATE_CPU_FREQ, frequency: 0 },
    cpufreq_frequency_table { flags: 0, driver_data: STATE_DDR_FREQ, frequency: 0 },
    cpufreq_frequency_table { flags: 0, driver_data: 0, frequency: CPUFREQ_TABLE_END },
];

unsafe fn kirkwood_cpufreq_get_cpu_frequency(_cpu: c_uint) -> c_uint {
    clk_get_rate(priv_.powersave_clk) / 1000
}

unsafe fn kirkwood_cpufreq_target(
    _policy: *mut cpufreq_policy,
    index: c_uint,
) -> c_int {
    let state = kirkwood_freq_table[index as usize].driver_data;
    let mut reg: c_ulong;

    local_irq_disable();

    /* Disable interrupts to the CPU */
    reg = readl_relaxed(priv_.base);
    reg |= CPU_SW_INT_BLK;
    writel_relaxed(reg, priv_.base);

    match state {
        STATE_CPU_FREQ => {
            clk_set_parent(priv_.powersave_clk, priv_.cpu_clk);
        }
        STATE_DDR_FREQ => {
            clk_set_parent(priv_.powersave_clk, priv_.ddr_clk);
        }
        _ => {}
    }

    /* Wait-for-Interrupt, while the hardware changes frequency */
    cpu_do_idle();

    /* Enable interrupts to the CPU */
    reg = readl_relaxed(priv_.base);
    reg &= !CPU_SW_INT_BLK;
    writel_relaxed(reg, priv_.base);

    local_irq_enable();

    0
}

/* Module init and exit code */
unsafe fn kirkwood_cpufreq_cpu_init(policy: *mut cpufreq_policy) -> c_int {
    cpufreq_generic_init(policy, kirkwood_freq_table.as_mut_ptr(), 5000);
    0
}

static mut kirkwood_cpufreq_driver: cpufreq_driver = cpufreq_driver {
    flags: CPUFREQ_NEED_INITIAL_FREQ_CHECK,
    get: Some(kirkwood_cpufreq_get_cpu_frequency),
    verify: Some(cpufreq_generic_frequency_table_verify),
    target_index: Some(kirkwood_cpufreq_target),
    init: Some(kirkwood_cpufreq_cpu_init),
    name: b"kirkwood-cpufreq\0".as_ptr() as *const c_char,
    ..cpufreq_driver::ZERO
};

unsafe fn kirkwood_cpufreq_probe(pdev: *mut platform_device) -> c_int {
    let np: *mut device_node;
    let mut err: c_int;

    priv_.dev = &mut (*pdev).dev;

    priv_.base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(priv_.base) {
        return PTR_ERR(priv_.base);
    }

    np = of_cpu_device_node_get(0);
    if np.is_null() {
        dev_err(&mut (*pdev).dev, b"failed to get cpu device node\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    priv_.cpu_clk = of_clk_get_by_name(np, b"cpu_clk\0".as_ptr() as *const c_char);
    if IS_ERR(priv_.cpu_clk) {
        dev_err(priv_.dev, b"Unable to get cpuclk\n\0".as_ptr() as *const c_char);
        err = PTR_ERR(priv_.cpu_clk);
        goto out_node;
    }

    err = clk_prepare_enable(priv_.cpu_clk);
    if err != 0 {
        dev_err(priv_.dev, b"Unable to prepare cpuclk\n\0".as_ptr() as *const c_char);
        goto out_node;
    }

    kirkwood_freq_table[0].frequency = clk_get_rate(priv_.cpu_clk) / 1000;

    priv_.ddr_clk = of_clk_get_by_name(np, b"ddrclk\0".as_ptr() as *const c_char);
    if IS_ERR(priv_.ddr_clk) {
        dev_err(priv_.dev, b"Unable to get ddrclk\n\0".as_ptr() as *const c_char);
        err = PTR_ERR(priv_.ddr_clk);
        goto out_cpu;
    }

    err = clk_prepare_enable(priv_.ddr_clk);
    if err != 0 {
        dev_err(priv_.dev, b"Unable to prepare ddrclk\n\0".as_ptr() as *const c_char);
        goto out_cpu;
    }
    kirkwood_freq_table[1].frequency = clk_get_rate(priv_.ddr_clk) / 1000;

    priv_.powersave_clk = of_clk_get_by_name(np, b"powersave\0".as_ptr() as *const c_char);
    if IS_ERR(priv_.powersave_clk) {
        dev_err(priv_.dev, b"Unable to get powersave\n\0".as_ptr() as *const c_char);
        err = PTR_ERR(priv_.powersave_clk);
        goto out_ddr;
    }
    err = clk_prepare_enable(priv_.powersave_clk);
    if err != 0 {
        dev_err(priv_.dev, b"Unable to prepare powersave clk\n\0".as_ptr() as *const c_char);
        goto out_ddr;
    }

    err = cpufreq_register_driver(&mut kirkwood_cpufreq_driver);
    if err != 0 {
        dev_err(priv_.dev, b"Failed to register cpufreq driver\n\0".as_ptr() as *const c_char);
        goto out_powersave;
    }

    of_node_put(np);
    return 0;

out_powersave:
    clk_disable_unprepare(priv_.powersave_clk);
out_ddr:
    clk_disable_unprepare(priv_.ddr_clk);
out_cpu:
    clk_disable_unprepare(priv_.cpu_clk);
out_node:
    of_node_put(np);
    err
}

unsafe fn kirkwood_cpufreq_remove(_pdev: *mut platform_device) {
    cpufreq_unregister_driver(&mut kirkwood_cpufreq_driver);
    clk_disable_unprepare(priv_.powersave_clk);
    clk_disable_unprepare(priv_.ddr_clk);
    clk_disable_unprepare(priv_.cpu_clk);
}

static mut kirkwood_cpufreq_platform_driver: platform_driver = platform_driver {
    probe: Some(kirkwood_cpufreq_probe),
    remove: Some(kirkwood_cpufreq_remove),
    driver: device_driver {
        name: b"kirkwood-cpufreq\0".as_ptr() as *const c_char,
        ..device_driver::ZERO
    },
    ..platform_driver::ZERO
};

// Equivalent of module_platform_driver(kirkwood_cpufreq_platform_driver).
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Andrew Lunn <andrew@lunn.ch");
// MODULE_DESCRIPTION("cpufreq driver for Marvell's kirkwood CPU");
// MODULE_ALIAS("platform:kirkwood-cpufreq");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
