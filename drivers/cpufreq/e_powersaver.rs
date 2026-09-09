// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on documentation provided by Dave Jones. Thanks!
 *
 * BIG FAT DISCLAIMER: Work in progress code. Possibly *dangerous*
 */

// C dependencies supplied by the surrounding kernel/Rust environment.

const EPS_BRAND_C7M: u8 = 0;
const EPS_BRAND_C7: u8 = 1;
const EPS_BRAND_EDEN: u8 = 2;
const EPS_BRAND_C3: u8 = 3;
const EPS_BRAND_C7D: u8 = 4;

#[repr(C)]
struct EpsCpuData {
    fsb: u32,
    #[cfg(feature = "CONFIG_ACPI_PROCESSOR")]
    bios_limit: u32,
    // Flexible array member: allocated immediately after this structure.
    freq_table: [CpufreqFrequencyTable; 0],
}

static mut EPS_CPU: *mut EpsCpuData = core::ptr::null_mut();

/* Module parameters */
static mut FREQ_FAILSAFE_OFF: i32 = 0;
static mut VOLTAGE_FAILSAFE_OFF: i32 = 0;
static mut SET_MAX_VOLTAGE: i32 = 0;

#[cfg(feature = "CONFIG_ACPI_PROCESSOR")]
static mut IGNORE_ACPI_LIMIT: i32 = 0;

#[cfg(feature = "CONFIG_ACPI_PROCESSOR")]
static mut EPS_ACPI_CPU_PERF: *mut AcpiProcessorPerformance = core::ptr::null_mut();

#[cfg(feature = "CONFIG_ACPI_PROCESSOR")]
unsafe fn eps_acpi_init() -> i32 {
    EPS_ACPI_CPU_PERF = kzalloc_obj::<AcpiProcessorPerformance>();
    if EPS_ACPI_CPU_PERF.is_null() {
        return -ENOMEM;
    }

    if !zalloc_cpumask_var(&mut (*EPS_ACPI_CPU_PERF).shared_cpu_map, GFP_KERNEL) {
        kfree(EPS_ACPI_CPU_PERF);
        EPS_ACPI_CPU_PERF = core::ptr::null_mut();
        return -ENOMEM;
    }

    if acpi_processor_register_performance(EPS_ACPI_CPU_PERF, 0) != 0 {
        free_cpumask_var((*EPS_ACPI_CPU_PERF).shared_cpu_map);
        kfree(EPS_ACPI_CPU_PERF);
        EPS_ACPI_CPU_PERF = core::ptr::null_mut();
        return -EIO;
    }
    0
}

#[cfg(feature = "CONFIG_ACPI_PROCESSOR")]
unsafe fn eps_acpi_exit(_policy: *mut CpufreqPolicy) -> i32 {
    if !EPS_ACPI_CPU_PERF.is_null() {
        acpi_processor_unregister_performance(0);
        free_cpumask_var((*EPS_ACPI_CPU_PERF).shared_cpu_map);
        kfree(EPS_ACPI_CPU_PERF);
        EPS_ACPI_CPU_PERF = core::ptr::null_mut();
    }
    0
}

unsafe fn eps_get(cpu: u32) -> u32 {
    let mut val: u64 = 0;
    if cpu != 0 {
        return 0;
    }
    let centaur = EPS_CPU;
    if centaur.is_null() {
        return 0;
    }
    rdmsrq(MSR_IA32_PERF_STATUS, &mut val);
    (*centaur).fsb * ((val >> 8) as u32 & 0xff)
}

unsafe fn eps_set_state(
    _centaur: *mut EpsCpuData,
    _policy: *mut CpufreqPolicy,
    dest_state: u32,
) -> i32 {
    let mut val: u64 = 0;
    let mut i = 0;

    rdmsrq(MSR_IA32_PERF_STATUS, &mut val);
    while val & ((1 << 16) | (1 << 17)) != 0 {
        udelay(16);
        rdmsrq(MSR_IA32_PERF_STATUS, &mut val);
        i += 1;
        if i > 64 {
            return -ENODEV;
        }
    }
    wrmsrq(MSR_IA32_PERF_CTL, (dest_state & 0xffff) as u64);
    i = 0;
    loop {
        udelay(16);
        rdmsrq(MSR_IA32_PERF_STATUS, &mut val);
        i += 1;
        if i > 64 {
            return -ENODEV;
        }
        if val & ((1 << 16) | (1 << 17)) == 0 {
            break;
        }
    }
    0
}

unsafe fn eps_target(policy: *mut CpufreqPolicy, index: u32) -> i32 {
    let cpu = (*policy).cpu;
    if EPS_CPU.is_null() {
        return -ENODEV;
    }
    let centaur = EPS_CPU;
    let dest_state = (*centaur).freq_table.add(index as usize).driver_data & 0xffff;
    let ret = eps_set_state(centaur, policy, dest_state);
    if ret != 0 {
        pr_err!("Timeout!\n");
    }
    ret
}

unsafe fn eps_cpu_init(policy: *mut CpufreqPolicy) -> i32 {
    let mut val: u64 = 0;
    let mut current_multiplier: u8;
    let mut current_voltage: u8;
    let mut max_multiplier: u8;
    let mut max_voltage: u8;
    let mut min_multiplier: u8;
    let mut min_voltage: u8;
    let mut brand: u8 = 0;
    let mut fsb: u32;
    let mut k: u32;
    let mut step: u32;
    let mut voltage: u32;
    let states: u32;

    if (*policy).cpu != 0 {
        return -ENODEV;
    }

    pr_info!("Detected VIA ");
    match cpu_data(0).x86_model {
        10 => {
            rdmsrq(0x1153, &mut val);
            brand = ((((val >> 2) ^ val) >> 18) & 3) as u8;
            pr_cont!("Model A ");
        }
        13 => {
            rdmsrq(0x1154, &mut val);
            brand = (((val >> 4) ^ (val >> 2)) & 0xff) as u8;
            pr_cont!("Model D ");
        }
        _ => {}
    }

    match brand {
        EPS_BRAND_C7M => pr_cont!("C7-M\n"),
        EPS_BRAND_C7 => pr_cont!("C7\n"),
        EPS_BRAND_EDEN => pr_cont!("Eden\n"),
        EPS_BRAND_C7D => pr_cont!("C7-D\n"),
        EPS_BRAND_C3 => { pr_cont!("C3\n"); return -ENODEV; }
        _ => {}
    }

    rdmsrq(MSR_IA32_MISC_ENABLE, &mut val);
    if val & MSR_IA32_MISC_ENABLE_ENHANCED_SPEEDSTEP == 0 {
        val |= MSR_IA32_MISC_ENABLE_ENHANCED_SPEEDSTEP;
        wrmsrq(MSR_IA32_MISC_ENABLE, val);
        rdmsrq(MSR_IA32_MISC_ENABLE, &mut val);
        if val & MSR_IA32_MISC_ENABLE_ENHANCED_SPEEDSTEP == 0 {
            pr_info!("Can't enable Enhanced PowerSaver\n");
            return -ENODEV;
        }
    }

    rdmsrq(MSR_IA32_PERF_STATUS, &mut val);
    current_voltage = val as u8;
    pr_info!("Current voltage = {}mV\n", current_voltage as u32 * 16 + 700);
    current_multiplier = (val >> 8) as u8;
    pr_info!("Current multiplier = {}\n", current_multiplier);
    max_voltage = (val >> 32) as u8;
    pr_info!("Highest voltage = {}mV\n", max_voltage as u32 * 16 + 700);
    max_multiplier = (val >> 40) as u8;
    pr_info!("Highest multiplier = {}\n", max_multiplier);
    min_voltage = (val >> 48) as u8;
    pr_info!("Lowest voltage = {}mV\n", min_voltage as u32 * 16 + 700);
    min_multiplier = (val >> 56) as u8;
    pr_info!("Lowest multiplier = {}\n", min_multiplier);

    if current_multiplier == 0 || max_multiplier == 0 || min_multiplier == 0
        || current_multiplier > max_multiplier || max_multiplier <= min_multiplier
        || current_voltage > 0x1f || max_voltage > 0x1f
        || max_voltage < min_voltage || current_voltage < min_voltage
        || current_voltage > max_voltage { return -EINVAL; }
    if FREQ_FAILSAFE_OFF == 0 && max_multiplier != current_multiplier { return -EINVAL; }
    if VOLTAGE_FAILSAFE_OFF == 0 && max_voltage != current_voltage { return -EINVAL; }

    fsb = cpu_khz / current_multiplier as u32;
    if brand == EPS_BRAND_C7M && SET_MAX_VOLTAGE != 0 {
        let v = ((SET_MAX_VOLTAGE - 700) / 16) as u8;
        if v >= min_voltage && v <= max_voltage { max_voltage = v; }
    }
    states = if brand == EPS_BRAND_C7M { (max_multiplier - min_multiplier + 1) as u32 } else { 2 };
    let size = core::mem::size_of::<EpsCpuData>() + (states as usize + 1) * core::mem::size_of::<CpufreqFrequencyTable>();
    let centaur = kzalloc(size) as *mut EpsCpuData;
    if centaur.is_null() { return -ENOMEM; }
    EPS_CPU = centaur;
    (*centaur).fsb = fsb;
    let f_table = (*centaur).freq_table.as_mut_ptr();
    if brand != EPS_BRAND_C7M {
        (*f_table).frequency = fsb * min_multiplier as u32;
        (*f_table).driver_data = (min_multiplier as u32) << 8 | min_voltage as u32;
        (*f_table.add(1)).frequency = fsb * max_multiplier as u32;
        (*f_table.add(1)).driver_data = (max_multiplier as u32) << 8 | max_voltage as u32;
        (*f_table.add(2)).frequency = CPUFREQ_TABLE_END;
    } else {
        k = 0;
        step = ((max_voltage - min_voltage) as u32 * 256) / (max_multiplier - min_multiplier) as u32;
        for i in min_multiplier..=max_multiplier {
            voltage = (k * step) / 256 + min_voltage as u32;
            (*f_table.add(k as usize)).frequency = fsb * i as u32;
            (*f_table.add(k as usize)).driver_data = (i as u32) << 8 | voltage;
            k += 1;
        }
        (*f_table.add(k as usize)).frequency = CPUFREQ_TABLE_END;
    }
    (*policy).cpuinfo.transition_latency = 140000;
    (*policy).freq_table = f_table;
    0
}

unsafe fn eps_cpu_exit(policy: *mut CpufreqPolicy) {
    let _cpu = (*policy).cpu;
    kfree(EPS_CPU as *mut core::ffi::c_void);
    EPS_CPU = core::ptr::null_mut();
}

static mut EPS_DRIVER: CpufreqDriver = CpufreqDriver {
    verify: Some(cpufreq_generic_frequency_table_verify), target_index: Some(eps_target),
    init: Some(eps_cpu_init), exit: Some(eps_cpu_exit), get: Some(eps_get), name: "e_powersaver",
};

// This driver works only on Centaur C7 processors with Enhanced SpeedStep/PowerSaver registers.
static EPS_CPU_ID: [X86CpuId; 2] = [
    x86_match_vendor_fam_feature(CENTAUR, 6, X86_FEATURE_EST, core::ptr::null()),
    X86CpuId::empty(),
];

unsafe fn eps_init() -> i32 {
    if !x86_match_cpu(EPS_CPU_ID.as_ptr()) || boot_cpu_data().x86_model < 10 { return -ENODEV; }
    if cpufreq_register_driver(&mut EPS_DRIVER) != 0 { return -EINVAL; }
    0
}

unsafe fn eps_exit() { cpufreq_unregister_driver(&mut EPS_DRIVER); }

// Module parameters and metadata:
// freq_failsafe_off, voltage_failsafe_off, ignore_acpi_limit, and set_max_voltage
// are exported as writable integer module parameters with the descriptions from the C source.
// MODULE_AUTHOR("Rafal Bilski <rafalbilski@interia.pl>");
// MODULE_DESCRIPTION("Enhanced PowerSaver driver for VIA C7 CPU's.");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
