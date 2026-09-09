/*
 * Cpufreq driver for the loongson-2 processors
 *
 * The 2E revision of loongson processor not support this feature.
 *
 * Copyright (C) 2006 - 2008 Lemote Inc. & Institute of Computing Technology
 * Author: Yanhua, yanh@lemote.com
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Dependencies supplied by the Linux kernel and Loongson platform code.

static mut nowait: u32 = 0;

static mut saved_cpu_wait: Option<unsafe extern "C" fn()> = None;

extern "C" {
    static mut cpu_clock_freq: usize;
    static mut loops_per_jiffy: usize;
    static mut current_cpu_data: CpuData;
    static mut cpu_wait: Option<unsafe extern "C" fn()>;
    static mut loongson2_clockmod_table: [CpufreqFrequencyTable; 0];

    fn loongson2_cpu_freq_notifier(nb: *mut NotifierBlock,
                                   val: usize,
                                   data: *mut core::ffi::c_void) -> i32;
    fn loongson2_cpu_set_rate(freq: u32) -> i32;
    fn cpufreq_generic_init(policy: *mut CpufreqPolicy,
                            table: *const CpufreqFrequencyTable,
                            transition_latency: u32) -> i32;
    fn cpufreq_generic_frequency_table_verify(policy: *mut CpufreqPolicy) -> i32;
    fn cpufreq_generic_get(policy: *mut CpufreqPolicy) -> u32;
    fn platform_driver_register(driver: *mut PlatformDriver) -> i32;
    fn platform_driver_unregister(driver: *mut PlatformDriver);
    fn cpufreq_register_notifier(nb: *mut NotifierBlock, list: u32) -> i32;
    fn cpufreq_unregister_notifier(nb: *mut NotifierBlock, list: u32) -> i32;
    fn cpufreq_register_driver(driver: *mut CpufreqDriver) -> i32;
    fn cpufreq_unregister_driver(driver: *mut CpufreqDriver);
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: usize);
    fn readl(addr: usize) -> u32;
    fn writel(value: u32, addr: usize);
    fn local_irq_enable();
    fn pr_info(fmt: *const u8);
}

#[repr(C)]
struct CpuData { udelay_val: usize }
#[repr(C)] struct NotifierBlock { notifier_call: Option<unsafe extern "C" fn(*mut NotifierBlock, usize, *mut core::ffi::c_void) -> i32> }
#[repr(C)] struct CpufreqPolicy;
#[repr(C)] struct CpufreqFrequencyTable { frequency: u32, driver_data: u32 }
#[repr(C)] struct CpufreqDriver {
    name: *const u8,
    init: Option<unsafe extern "C" fn(*mut CpufreqPolicy) -> i32>,
    verify: Option<unsafe extern "C" fn(*mut CpufreqPolicy) -> i32>,
    target_index: Option<unsafe extern "C" fn(*mut CpufreqPolicy, u32) -> i32>,
    get: Option<unsafe extern "C" fn(*mut CpufreqPolicy) -> u32>,
}
#[repr(C)] struct PlatformDeviceId { name: *const u8 }
#[repr(C)] struct PlatformDriver { driver: Driver, id_table: *const PlatformDeviceId }
#[repr(C)] struct Driver { name: *const u8 }
#[repr(C)] struct Spinlock;

const CPUFREQ_POSTCHANGE: usize = 0;
const CPUFREQ_TRANSITION_NOTIFIER: u32 = 0;
const CPUFREQ_TABLE_END: u32 = 0;
const LOONGSON_CHIPCFG: usize = 0;

static mut loongson2_cpufreq_notifier_block: NotifierBlock = NotifierBlock {
    notifier_call: Some(loongson2_cpu_freq_notifier),
};

unsafe extern "C" fn loongson2_cpu_freq_notifier(_nb: *mut NotifierBlock,
                                                   val: usize,
                                                   _data: *mut core::ffi::c_void) -> i32 {
    if val == CPUFREQ_POSTCHANGE {
        current_cpu_data.udelay_val = loops_per_jiffy;
    }
    0
}

/* Here we notify other drivers of the proposed change and the final change. */
unsafe extern "C" fn loongson2_cpufreq_target(_policy: *mut CpufreqPolicy,
                                                index: u32) -> i32 {
    let freq = ((cpu_clock_freq / 1000) *
        loongson2_clockmod_table[index as usize].driver_data as usize) / 8;
    loongson2_cpu_set_rate(freq as u32);
    0
}

unsafe extern "C" fn loongson2_cpufreq_cpu_init(policy: *mut CpufreqPolicy) -> i32 {
    let rate = cpu_clock_freq / 1000;
    if rate == 0 { return -22; }

    let mut i = 2usize;
    while loongson2_clockmod_table[i].frequency != CPUFREQ_TABLE_END {
        loongson2_clockmod_table[i].frequency = ((rate * i) / 8) as u32;
        i += 1;
    }

    let ret = loongson2_cpu_set_rate(rate as u32);
    if ret != 0 { return ret; }
    cpufreq_generic_init(policy, loongson2_clockmod_table.as_ptr(), 0);
    0
}

static mut loongson2_cpufreq_driver: CpufreqDriver = CpufreqDriver {
    name: b"loongson2\0".as_ptr(),
    init: Some(loongson2_cpufreq_cpu_init),
    verify: Some(cpufreq_generic_frequency_table_verify),
    target_index: Some(loongson2_cpufreq_target),
    get: Some(cpufreq_generic_get),
};

static platform_device_ids: [PlatformDeviceId; 2] = [
    PlatformDeviceId { name: b"loongson2_cpufreq\0".as_ptr() },
    PlatformDeviceId { name: core::ptr::null() },
];

static mut platform_driver: PlatformDriver = PlatformDriver {
    driver: Driver { name: b"loongson2_cpufreq\0".as_ptr() },
    id_table: platform_device_ids.as_ptr(),
};

/*
 * This is the simple version of Loongson-2 wait, Maybe we need do this in
 * interrupt disabled context.
 */
static mut loongson2_wait_lock: Spinlock = Spinlock;

unsafe extern "C" fn loongson2_cpu_wait() {
    let mut flags = 0usize;
    let cpu_freq;
    spin_lock_irqsave(&mut loongson2_wait_lock, &mut flags);
    cpu_freq = readl(LOONGSON_CHIPCFG);
    /* Put CPU into wait mode */
    writel(readl(LOONGSON_CHIPCFG) & !0x7, LOONGSON_CHIPCFG);
    /* Restore CPU state */
    writel(cpu_freq, LOONGSON_CHIPCFG);
    spin_unlock_irqrestore(&mut loongson2_wait_lock, flags);
    local_irq_enable();
}

unsafe extern "C" fn cpufreq_init() -> i32 {
    let ret = platform_driver_register(&mut platform_driver);
    if ret != 0 { return ret; }
    pr_info(b"Loongson-2F CPU frequency driver\n\0".as_ptr());
    cpufreq_register_notifier(&mut loongson2_cpufreq_notifier_block,
                              CPUFREQ_TRANSITION_NOTIFIER);
    let ret = cpufreq_register_driver(&mut loongson2_cpufreq_driver);
    if ret != 0 {
        platform_driver_unregister(&mut platform_driver);
    } else if nowait == 0 {
        saved_cpu_wait = cpu_wait;
        cpu_wait = Some(loongson2_cpu_wait);
    }
    ret
}

unsafe extern "C" fn cpufreq_exit() {
    if nowait == 0 && saved_cpu_wait.is_some() { cpu_wait = saved_cpu_wait; }
    cpufreq_unregister_driver(&mut loongson2_cpufreq_driver);
    cpufreq_unregister_notifier(&mut loongson2_cpufreq_notifier_block,
                                CPUFREQ_TRANSITION_NOTIFIER);
    platform_driver_unregister(&mut platform_driver);
}

// module_init(cpufreq_init); module_exit(cpufreq_exit);
// module_param(nowait, uint, 0644);
// MODULE_PARM_DESC(nowait, "Disable Loongson-2F specific wait");
// MODULE_AUTHOR("Yanhua <yanh@lemote.com>");
// MODULE_DESCRIPTION("cpufreq driver for Loongson2F");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
