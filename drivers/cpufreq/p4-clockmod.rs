// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	Pentium 4/Xeon CPU on demand clock modulation/speed scaling
 *	(C) 2002 - 2003 Dominik Brodowski <linux@brodo.de>
 *	(C) 2002 Zwane Mwaikambo <zwane@commfireservices.com>
 *	(C) 2002 Arjan van de Ven <arjanv@redhat.com>
 *	(C) 2002 Tora T. Engstad
 *	All Rights Reserved
 *
 *      The author(s) of this software shall not be held liable for damages
 *      of any nature resulting due to the use of this software. This
 *      software is provided AS-IS with no warranties.
 *
 *	Date		Errata			Description
 *	20020525	N44, O17	12.5% or 25% DC causes lockup
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * Duty Cycle (3bits), note DC_DISABLE is not specified in
 * intel docs i just use it to mean disable
 */
const DC_RESV: u32 = 0;
const DC_DFLT: u32 = 1;
const DC_25PT: u32 = 2;
const DC_38PT: u32 = 3;
const DC_50PT: u32 = 4;
const DC_64PT: u32 = 5;
const DC_75PT: u32 = 6;
const DC_88PT: u32 = 7;
const DC_DISABLE: u32 = 8;

const DC_ENTRIES: usize = 8;

static mut has_N44_O17_errata: [i32; NR_CPUS] = [0; NR_CPUS];
static mut stock_freq: u32 = 0;

unsafe fn cpufreq_p4_setdc(cpu: u32, mut newstate: u32) -> i32 {
    let mut val: msr = core::mem::zeroed();

    if newstate > DC_DISABLE || newstate == DC_RESV {
        return -EINVAL;
    }

    rdmsrq_on_cpu(cpu, MSR_IA32_THERM_STATUS, &mut val.q);

    if val.l & 0x01 != 0 {
        pr_debug!("CPU#{} currently thermal throttled\n", cpu);
    }

    if has_N44_O17_errata[cpu as usize] != 0
        && (newstate == DC_25PT || newstate == DC_DFLT)
    {
        newstate = DC_38PT;
    }

    rdmsrq_on_cpu(cpu, MSR_IA32_THERM_CONTROL, &mut val.q);
    if newstate == DC_DISABLE {
        pr_debug!("CPU#{} disabling modulation\n", cpu);
        wrmsrq_on_cpu(cpu, MSR_IA32_THERM_CONTROL, val.q & !(1u64 << 4));
    } else {
        pr_debug!("CPU#{} setting duty cycle to {}%\n", cpu, (125 * newstate) / 10);
        /* bits 63 - 5 : reserved
         * bit  4       : enable/disable
         * bits 3-1     : duty cycle
         * bit  0       : reserved
         */
        val.l &= !14;
        val.l |= (1 << 4) | ((newstate & 0x7) << 1);
        wrmsrq_on_cpu(cpu, MSR_IA32_THERM_CONTROL, val.q);
    }

    0
}

static mut p4clockmod_table: [cpufreq_frequency_table; 10] = [
    cpufreq_frequency_table { driver_data: DC_RESV, frequency: CPUFREQ_ENTRY_INVALID },
    cpufreq_frequency_table { driver_data: DC_DFLT, frequency: 0 },
    cpufreq_frequency_table { driver_data: DC_25PT, frequency: 0 },
    cpufreq_frequency_table { driver_data: DC_38PT, frequency: 0 },
    cpufreq_frequency_table { driver_data: DC_50PT, frequency: 0 },
    cpufreq_frequency_table { driver_data: DC_64PT, frequency: 0 },
    cpufreq_frequency_table { driver_data: DC_75PT, frequency: 0 },
    cpufreq_frequency_table { driver_data: DC_88PT, frequency: 0 },
    cpufreq_frequency_table { driver_data: DC_DISABLE, frequency: 0 },
    cpufreq_frequency_table { driver_data: DC_RESV, frequency: CPUFREQ_TABLE_END },
];

unsafe fn cpufreq_p4_target(policy: *mut cpufreq_policy, index: u32) -> i32 {
    let mut i: i32 = 0;
    for_each_cpu!(i, (*policy).cpus) {
        cpufreq_p4_setdc(i as u32, p4clockmod_table[index as usize].driver_data);
    }
    0
}

unsafe fn cpufreq_p4_get_frequency(c: *mut cpuinfo_x86) -> u32 {
    if (*c).x86 == 0x06 {
        if cpu_has!(c, X86_FEATURE_EST) {
            pr_warn_once!("Warning: EST-capable CPU detected. The acpi-cpufreq module offers voltage scaling in addition to frequency scaling. You should use that instead of p4-clockmod, if possible.\n");
        }
        match (*c).x86_model {
            0x0E | 0x0F | 0x16 | 0x1C => {
                p4clockmod_driver.flags |= CPUFREQ_CONST_LOOPS;
                speedstep_get_frequency(SPEEDSTEP_CPU_PCORE)
            }
            0x0D => {
                p4clockmod_driver.flags |= CPUFREQ_CONST_LOOPS;
                speedstep_get_frequency(SPEEDSTEP_CPU_PM)
            }
            0x09 => speedstep_get_frequency(SPEEDSTEP_CPU_PM),
            _ => 0,
        }
    } else {
        if (*c).x86 != 0xF { return 0; }
        p4clockmod_driver.flags |= CPUFREQ_CONST_LOOPS;
        if speedstep_detect_processor() == SPEEDSTEP_CPU_P4M {
            pr_warn!("Warning: Pentium 4-M detected. The speedstep-ich or acpi cpufreq modules offer voltage scaling in addition of frequency scaling. You should use either one instead of p4-clockmod, if possible.\n");
            return speedstep_get_frequency(SPEEDSTEP_CPU_P4M);
        }
        speedstep_get_frequency(SPEEDSTEP_CPU_P4D)
    }
}

unsafe fn cpufreq_p4_cpu_init(policy: *mut cpufreq_policy) -> i32 {
    let c = &mut cpu_data((*policy).cpu);
    let mut cpuid: i32 = 0;
    let mut i: usize = 0;

    // #ifdef CONFIG_SMP
    cpumask_copy((*policy).cpus, topology_sibling_cpumask((*policy).cpu));

    cpuid = (c.x86 << 8) | (c.x86_model << 4) | c.x86_stepping;
    match cpuid {
        0x0f07 | 0x0f0a | 0x0f11 | 0x0f12 => {
            has_N44_O17_errata[(*policy).cpu as usize] = 1;
            pr_debug!("has errata -- disabling low frequencies\n");
        }
        _ => {}
    }

    if speedstep_detect_processor() == SPEEDSTEP_CPU_P4D && c.x86_model < 2 {
        cpufreq_p4_setdc((*policy).cpu, DC_DISABLE);
        recalibrate_cpu_khz();
    }
    stock_freq = cpufreq_p4_get_frequency(c);
    if stock_freq == 0 { return -EINVAL; }

    i = 1;
    while p4clockmod_table[i].frequency != CPUFREQ_TABLE_END {
        if i < 2 && has_N44_O17_errata[(*policy).cpu as usize] != 0 {
            p4clockmod_table[i].frequency = CPUFREQ_ENTRY_INVALID;
        } else {
            p4clockmod_table[i].frequency = (stock_freq * i as u32) / 8;
        }
        i += 1;
    }

    (*policy).cpuinfo.transition_latency = 10000001;
    (*policy).freq_table = &mut p4clockmod_table[0];
    0
}

unsafe fn cpufreq_p4_get(cpu: u32) -> u32 {
    let mut val: msr = core::mem::zeroed();
    rdmsrq_on_cpu(cpu, MSR_IA32_THERM_CONTROL, &mut val.q);
    if val.l & 0x10 != 0 {
        val.l >>= 1;
        val.l &= 0x7;
    } else { val.l = DC_DISABLE; }
    if val.l != DC_DISABLE { stock_freq * val.l / 8 } else { stock_freq }
}

static mut p4clockmod_driver: cpufreq_driver = cpufreq_driver {
    verify: Some(cpufreq_generic_frequency_table_verify),
    target_index: Some(cpufreq_p4_target),
    init: Some(cpufreq_p4_cpu_init),
    get: Some(cpufreq_p4_get),
    name: "p4-clockmod",
    ..cpufreq_driver::default()
};

static cpufreq_p4_id: [x86_cpu_id; 2] = [
    X86_MATCH_VENDOR_FEATURE!(INTEL, X86_FEATURE_ACC, core::ptr::null_mut()),
    x86_cpu_id::default(),
];

/*
 * Intentionally no MODULE_DEVICE_TABLE here: this driver should not
 * be auto loaded.  Please don't add one.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
