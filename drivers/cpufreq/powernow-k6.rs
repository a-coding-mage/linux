// SPDX-License-Identifier: GPL-2.0-only
/*
 *  This file was based upon code in Powertweak Linux (http://powertweak.sf.net)
 *  (C) 2000-2003  Dave Jones, Arjan van de Ven, Janne Pänkälä,
 *                 Dominik Brodowski.
 *
 *  BIG FAT DISCLAIMER: Work in progress code. Possibly *dangerous*
 */

// Dependency includes from the original C source:
// linux/kernel.h, linux/module.h, linux/init.h, linux/cpufreq.h,
// linux/ioport.h, linux/timex.h, linux/io.h, asm/cpu_device_id.h, asm/msr.h

const POWERNOW_IOPORT: u16 = 0xfff0; // it doesn't matter where, as long as it is unused

static mut busfreq: u32 = 0; // FSB, in 10 kHz
static mut max_multiplier: u32 = 0;

static mut param_busfreq: u32 = 0;
static mut param_max_multiplier: u32 = 0;

// module_param_named(max_multiplier, param_max_multiplier, uint, S_IRUGO);
// MODULE_PARM_DESC(max_multiplier, "Maximum multiplier (allowed values: 20 30 35 40 45 50 55 60)");
// module_param_named(bus_frequency, param_busfreq, uint, S_IRUGO);
// MODULE_PARM_DESC(bus_frequency, "Bus frequency in kHz");

#[repr(C)]
struct CpufreqFrequencyTable {
    flags: u32,
    driver_data: u32,
    frequency: u32,
}

const CPUFREQ_TABLE_END: u32 = 0;
const CPUFREQ_ENTRY_INVALID: u32 = 0xffff_ffff;

// Clock ratio multiplied by 10 - see table 27 in AMD#23446
static mut clock_ratio: [CpufreqFrequencyTable; 9] = [
    CpufreqFrequencyTable { flags: 0, driver_data: 60, frequency: 0 }, // 110 -> 6.0x
    CpufreqFrequencyTable { flags: 0, driver_data: 55, frequency: 0 }, // 011 -> 5.5x
    CpufreqFrequencyTable { flags: 0, driver_data: 50, frequency: 0 }, // 001 -> 5.0x
    CpufreqFrequencyTable { flags: 0, driver_data: 45, frequency: 0 }, // 000 -> 4.5x
    CpufreqFrequencyTable { flags: 0, driver_data: 40, frequency: 0 }, // 010 -> 4.0x
    CpufreqFrequencyTable { flags: 0, driver_data: 35, frequency: 0 }, // 111 -> 3.5x
    CpufreqFrequencyTable { flags: 0, driver_data: 30, frequency: 0 }, // 101 -> 3.0x
    CpufreqFrequencyTable { flags: 0, driver_data: 20, frequency: 0 }, // 100 -> 2.0x
    CpufreqFrequencyTable { flags: 0, driver_data: 0, frequency: CPUFREQ_TABLE_END },
];

static index_to_register: [u8; 8] = [6, 3, 1, 0, 2, 7, 5, 4];
static register_to_index: [u8; 8] = [3, 2, 4, 1, 7, 6, 0, 5];

#[repr(C)]
struct UsualFrequency {
    freq: u32,
    mult: u32,
}

static usual_frequency_table: [UsualFrequency; 15] = [
    UsualFrequency { freq: 350000, mult: 35 }, // 100   * 3.5
    UsualFrequency { freq: 400000, mult: 40 }, // 100   * 4
    UsualFrequency { freq: 450000, mult: 45 }, // 100   * 4.5
    UsualFrequency { freq: 475000, mult: 50 }, //  95   * 5
    UsualFrequency { freq: 500000, mult: 50 }, // 100   * 5
    UsualFrequency { freq: 506250, mult: 45 }, // 112.5 * 4.5
    UsualFrequency { freq: 533500, mult: 55 }, //  97   * 5.5
    UsualFrequency { freq: 550000, mult: 55 }, // 100   * 5.5
    UsualFrequency { freq: 562500, mult: 50 }, // 112.5 * 5
    UsualFrequency { freq: 570000, mult: 60 }, //  95   * 6
    UsualFrequency { freq: 600000, mult: 60 }, // 100   * 6
    UsualFrequency { freq: 618750, mult: 55 }, // 112.5 * 5.5
    UsualFrequency { freq: 660000, mult: 55 }, // 120   * 5.5
    UsualFrequency { freq: 675000, mult: 60 }, // 112.5 * 6
    UsualFrequency { freq: 720000, mult: 60 }, // 120   * 6
];

const FREQ_RANGE: u32 = 3000;

// External kernel symbols supplied by other files.
extern "C" {
    static cpu_khz: u32;
    static MSR_K6_EPMR: u32;
    fn local_irq_disable();
    fn local_irq_enable();
    fn read_cr0() -> usize;
    fn write_cr0(value: usize);
    fn wbinvd();
    fn wrmsrq(msr: u32, value: u64);
    fn inl(port: u16) -> u32;
    fn outl(value: u32, port: u16);
    fn cpufreq_for_each_entry(pos: *mut CpufreqFrequencyTable, table: *mut CpufreqFrequencyTable);
    fn cpufreq_generic_frequency_table_verify(policy: *mut CpufreqPolicy) -> i32;
    fn cpufreq_freq_transition_begin(policy: *mut CpufreqPolicy, freqs: *mut CpufreqFreqs);
    fn cpufreq_freq_transition_end(policy: *mut CpufreqPolicy, freqs: *mut CpufreqFreqs, state: i32);
    fn x86_match_cpu(ids: *const X86CpuId) -> bool;
    fn request_region(start: u16, len: u32, name: *const u8) -> *mut u8;
    fn release_region(start: u16, len: u32);
    fn cpufreq_register_driver(driver: *mut CpufreqDriver) -> i32;
    fn cpufreq_unregister_driver(driver: *mut CpufreqDriver);
}

#[repr(C)]
struct CpufreqPolicy {
    cpu: u32,
    cur: u32,
    cpuinfo: CpufreqCpuinfo,
    freq_table: *mut CpufreqFrequencyTable,
}

#[repr(C)]
struct CpufreqCpuinfo { transition_latency: u32 }

#[repr(C)]
struct CpufreqFreqs { old: u32, new: u32, flags: u32 }

#[repr(C)]
struct CpufreqDriver {
    verify: Option<unsafe extern "C" fn(*mut CpufreqPolicy) -> i32>,
    target_index: Option<unsafe extern "C" fn(*mut CpufreqPolicy, u32) -> i32>,
    init: Option<unsafe extern "C" fn(*mut CpufreqPolicy) -> i32>,
    exit: Option<unsafe extern "C" fn(*mut CpufreqPolicy)>,
    get: Option<unsafe extern "C" fn(u32) -> u32>,
    name: *const u8,
}

#[repr(C)]
struct X86CpuId { _opaque: [u8; 0] }

static powernow_k6_ids: [X86CpuId; 3] = [X86CpuId { _opaque: [] }, X86CpuId { _opaque: [] }, X86CpuId { _opaque: [] }];

unsafe fn powernow_k6_get_cpu_multiplier() -> i32 {
    let mut invalue: u32 = 0;
    let mut msrval: u64;

    local_irq_disable();
    msrval = (POWERNOW_IOPORT as u64) + 0x1;
    wrmsrq(MSR_K6_EPMR, msrval); // enable the PowerNow port
    invalue = inl(POWERNOW_IOPORT + 0x8);
    msrval = (POWERNOW_IOPORT as u64) + 0x0;
    wrmsrq(MSR_K6_EPMR, msrval); // disable it again
    local_irq_enable();

    clock_ratio[register_to_index[((invalue >> 5) & 7) as usize]].driver_data as i32
}

unsafe fn powernow_k6_set_cpu_multiplier(best_i: u32) {
    let mut outvalue: u32;
    let mut invalue: u32;
    let cr0: usize;
    let mut msrval: u64;

    // We now need to transform best_i to the BVC format, see AMD#23446.
    // The processor doesn't respond to inquiry cycles while changing the
    // frequency, so we must disable cache.
    local_irq_disable();
    cr0 = read_cr0();
    write_cr0(cr0 | (1usize << 30));
    wbinvd();

    outvalue = (1 << 12) | (1 << 10) | (1 << 9) | ((index_to_register[best_i as usize] as u32) << 5);
    msrval = (POWERNOW_IOPORT as u64) + 0x1;
    wrmsrq(MSR_K6_EPMR, msrval); // enable the PowerNow port
    invalue = inl(POWERNOW_IOPORT + 0x8);
    invalue &= 0x1f;
    outvalue |= invalue;
    outl(outvalue, POWERNOW_IOPORT + 0x8);
    msrval = POWERNOW_IOPORT as u64;
    wrmsrq(MSR_K6_EPMR, msrval); // disable it again
    write_cr0(cr0);
    local_irq_enable();
}

unsafe extern "C" fn powernow_k6_target(_policy: *mut CpufreqPolicy, best_i: u32) -> i32 {
    if clock_ratio[best_i as usize].driver_data > max_multiplier {
        return -22; // -EINVAL
    }
    powernow_k6_set_cpu_multiplier(best_i);
    0
}

unsafe extern "C" fn powernow_k6_cpu_init(policy: *mut CpufreqPolicy) -> i32 {
    if (*policy).cpu != 0 { return -19; } // -ENODEV
    max_multiplier = 0;
    let mut khz = cpu_khz;
    for entry in usual_frequency_table.iter() {
        if khz >= entry.freq - FREQ_RANGE && khz <= entry.freq + FREQ_RANGE {
            khz = entry.freq;
            max_multiplier = entry.mult;
            break;
        }
    }
    if param_max_multiplier != 0 {
        for pos in clock_ratio.iter_mut() {
            if pos.driver_data == param_max_multiplier { max_multiplier = param_max_multiplier; break; }
        }
        if max_multiplier != param_max_multiplier { return -22; } // -EINVAL
    }
    if max_multiplier == 0 { return -95; } // -EOPNOTSUPP
    param_max_multiplier = max_multiplier;
    if param_busfreq != 0 {
        if param_busfreq >= 50000 && param_busfreq <= 150000 { busfreq = param_busfreq / 10; }
        else { return -22; }
    } else { busfreq = khz / max_multiplier; }
    param_busfreq = busfreq * 10;
    for pos in clock_ratio.iter_mut() {
        let f = pos.driver_data;
        pos.frequency = if f > max_multiplier { CPUFREQ_ENTRY_INVALID } else { busfreq * f };
    }
    (*policy).cpuinfo.transition_latency = 500000;
    (*policy).freq_table = clock_ratio.as_mut_ptr();
    0
}

unsafe extern "C" fn powernow_k6_cpu_exit(policy: *mut CpufreqPolicy) {
    for (i, entry) in clock_ratio.iter().enumerate() {
        if entry.frequency != CPUFREQ_TABLE_END && entry.driver_data == max_multiplier {
            let mut freqs = CpufreqFreqs { old: (*policy).cur, new: entry.frequency, flags: 0 };
            cpufreq_freq_transition_begin(policy, &mut freqs);
            powernow_k6_target(policy, i as u32);
            cpufreq_freq_transition_end(policy, &mut freqs, 0);
            return;
        }
    }
}

unsafe extern "C" fn powernow_k6_get(_cpu: u32) -> u32 {
    busfreq * powernow_k6_get_cpu_multiplier() as u32
}

static mut powernow_k6_driver: CpufreqDriver = CpufreqDriver {
    verify: Some(cpufreq_generic_frequency_table_verify),
    target_index: Some(powernow_k6_target),
    init: Some(powernow_k6_cpu_init),
    exit: Some(powernow_k6_cpu_exit),
    get: Some(powernow_k6_get),
    name: b"powernow-k6\0".as_ptr(),
};

unsafe extern "C" fn powernow_k6_init() -> i32 {
    if !x86_match_cpu(powernow_k6_ids.as_ptr()) { return -19; }
    if request_region(POWERNOW_IOPORT, 16, b"PowerNow!\0".as_ptr()).is_null() { return -5; } // -EIO
    if cpufreq_register_driver(&mut powernow_k6_driver) != 0 {
        release_region(POWERNOW_IOPORT, 16);
        return -22;
    }
    0
}

unsafe extern "C" fn powernow_k6_exit() {
    cpufreq_unregister_driver(&mut powernow_k6_driver);
    release_region(POWERNOW_IOPORT, 16);
}

// MODULE_AUTHOR("Arjan van de Ven, Dave Jones, Dominik Brodowski <linux@brodo.de>");
// MODULE_DESCRIPTION("PowerNow! driver for AMD K6-2+ / K6-3+ processors.");
// MODULE_LICENSE("GPL");
// module_init(powernow_k6_init);
// module_exit(powernow_k6_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
