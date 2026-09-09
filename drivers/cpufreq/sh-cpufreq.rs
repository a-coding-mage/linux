/*
 * cpufreq driver for the SuperH processors.
 *
 * Copyright (C) 2002 - 2012 Paul Mundt
 * Copyright (C) 2002 M. R. Brown
 *
 * Clock framework bits from arch/avr32/mach-at32ap/cpufreq.c
 *
 *   Copyright (C) 2004-2007 Atmel Corporation
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */
// pr_fmt(fmt) = "cpufreq: " fmt

// External Linux kernel declarations and definitions are supplied by dependencies.

static mut SH_CPUCLK: PerCpu<clk> = DEFINE_PER_CPU();

#[repr(C)]
struct cpufreq_target {
    policy: *mut cpufreq_policy,
    freq: c_uint,
}

unsafe fn sh_cpufreq_get(cpu: c_uint) -> c_uint {
    (clk_get_rate(&per_cpu(SH_CPUCLK, cpu)).wrapping_add(500)) / 1000
}

unsafe fn __sh_cpufreq_target(arg: *mut c_void) -> c_long {
    let target = arg as *mut cpufreq_target;
    let policy = (*target).policy;
    let cpu = (*policy).cpu;
    let cpuclk = &mut per_cpu(SH_CPUCLK, cpu);
    let mut freqs: cpufreq_freqs;
    let dev: *mut device;
    let freq: c_long;

    if smp_processor_id() != cpu {
        return -ENODEV;
    }

    dev = get_cpu_device(cpu);

    /* Convert target_freq from kHz to Hz */
    freq = clk_round_rate(cpuclk, (*target).freq * 1000);

    if freq < ((*policy).min * 1000) as c_long || freq > ((*policy).max * 1000) as c_long {
        return -EINVAL;
    }

    dev_dbg(dev, "requested frequency %u Hz\n", (*target).freq * 1000);

    freqs.old = sh_cpufreq_get(cpu);
    freqs.new = ((freq + 500) / 1000) as c_uint;
    freqs.flags = 0;

    cpufreq_freq_transition_begin(policy, &mut freqs);
    clk_set_rate(cpuclk, freq);
    cpufreq_freq_transition_end(policy, &mut freqs, 0);

    dev_dbg(dev, "set frequency %lu Hz\n", freq);
    0
}

/*
 * Here we notify other drivers of the proposed change and the final change.
 */
unsafe fn sh_cpufreq_target(
    policy: *mut cpufreq_policy,
    target_freq: c_uint,
    relation: c_uint,
) -> c_int {
    let mut data = cpufreq_target { policy, freq: target_freq };

    work_on_cpu((*policy).cpu, __sh_cpufreq_target, &mut data as *mut _ as *mut c_void)
}

unsafe fn sh_cpufreq_verify(policy: *mut cpufreq_policy_data) -> c_int {
    let cpuclk = &mut per_cpu(SH_CPUCLK, (*policy).cpu);

    if !(*policy).freq_table.is_null() {
        return cpufreq_frequency_table_verify(policy);
    }

    cpufreq_verify_within_cpu_limits(policy);

    (*policy).min = ((clk_round_rate(cpuclk, 1) + 500) / 1000) as c_uint;
    (*policy).max = ((clk_round_rate(cpuclk, !0 as c_ulong) + 500) / 1000) as c_uint;

    cpufreq_verify_within_cpu_limits(policy);
    0
}

unsafe fn sh_cpufreq_cpu_init(policy: *mut cpufreq_policy) -> c_int {
    let cpu = (*policy).cpu;
    let mut cpuclk = &mut per_cpu(SH_CPUCLK, cpu);
    let freq_table: *mut cpufreq_frequency_table;
    let dev: *mut device;

    dev = get_cpu_device(cpu);

    cpuclk = clk_get(dev, "cpu_clk");
    if IS_ERR(cpuclk) {
        dev_err(dev, "couldn't get CPU clk\n");
        return PTR_ERR(cpuclk);
    }

    freq_table = if (*cpuclk).nr_freqs != 0 { (*cpuclk).freq_table } else { core::ptr::null_mut() };
    if !freq_table.is_null() {
        (*policy).freq_table = freq_table;
    } else {
        dev_notice(dev, "no frequency table found, falling back to rate rounding.\n");

        (*policy).cpuinfo.min_freq = ((clk_round_rate(cpuclk, 1) + 500) / 1000) as c_uint;
        (*policy).cpuinfo.max_freq = ((clk_round_rate(cpuclk, !0 as c_ulong) + 500) / 1000) as c_uint;
    }

    0
}

unsafe fn sh_cpufreq_cpu_exit(policy: *mut cpufreq_policy) {
    let cpu = (*policy).cpu;
    let cpuclk = &mut per_cpu(SH_CPUCLK, cpu);

    clk_put(cpuclk);
}

static mut sh_cpufreq_driver: cpufreq_driver = cpufreq_driver {
    name: "sh",
    flags: CPUFREQ_NO_AUTO_DYNAMIC_SWITCHING,
    get: Some(sh_cpufreq_get),
    target: Some(sh_cpufreq_target),
    verify: Some(sh_cpufreq_verify),
    init: Some(sh_cpufreq_cpu_init),
    exit: Some(sh_cpufreq_cpu_exit),
};

unsafe fn sh_cpufreq_module_init() -> c_int {
    pr_notice("SuperH CPU frequency driver.\n");
    cpufreq_register_driver(&mut sh_cpufreq_driver)
}

unsafe fn sh_cpufreq_module_exit() {
    cpufreq_unregister_driver(&mut sh_cpufreq_driver);
}

module_init!(sh_cpufreq_module_init);
module_exit!(sh_cpufreq_module_exit);

MODULE_AUTHOR!("Paul Mundt <lethal@linux-sh.org>");
MODULE_DESCRIPTION!("cpufreq driver for SuperH");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
