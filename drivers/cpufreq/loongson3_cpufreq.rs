// SPDX-License-Identifier: GPL-2.0-only
/*
 * CPUFreq driver for the Loongson-3 processors.
 *
 * All revisions of Loongson-3 processor support cpu_has_scalefreq feature.
 *
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2024 Loongson Technology Corporation Limited
 */

// Dependencies are supplied by the surrounding kernel translation.

#[repr(C)]
pub union SmcMessage {
    pub value: u32,
    pub bits: SmcMessageBits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SmcMessageBits {
    pub id: u32,
    pub info: u32,
    pub val: u32,
    pub cmd: u32,
    pub extra: u32,
    pub complete: u32,
}

pub const CMD_OK: u32 = 0;
pub const CMD_ERROR: u32 = 1;
pub const CMD_NOCMD: u32 = 2;
pub const CMD_INVAL: u32 = 3;

pub const CMD_GET_VERSION: u32 = 0x1;
pub const CMD_GET_FEATURE: u32 = 0x2;
pub const CMD_SET_FEATURE: u32 = 0x3;

pub const FEATURE_SENSOR: u32 = 0;
pub const FEATURE_FAN: u32 = 1;
pub const FEATURE_DVFS: u32 = 2;

pub const FEATURE_SENSOR_ENABLE: u32 = 1 << 0;
pub const FEATURE_SENSOR_SAMPLE: u32 = 1 << 1;
pub const FEATURE_FAN_ENABLE: u32 = 1 << 0;
pub const FEATURE_FAN_AUTO: u32 = 1 << 1;
pub const FEATURE_DVFS_ENABLE: u32 = 1 << 0;
pub const FEATURE_DVFS_BOOST: u32 = 1 << 1;
pub const FEATURE_DVFS_AUTO: u32 = 1 << 2;
pub const FEATURE_DVFS_SINGLE_BOOST: u32 = 1 << 3;

pub const CMD_GET_SENSOR_NUM: u32 = 0x4;
pub const CMD_GET_SENSOR_STATUS: u32 = 0x5;
pub const SENSOR_INFO_TYPE: u32 = 0;
pub const SENSOR_INFO_TYPE_TEMP: u32 = 1;
pub const CMD_GET_FAN_NUM: u32 = 0x6;
pub const CMD_GET_FAN_INFO: u32 = 0x7;
pub const CMD_SET_FAN_INFO: u32 = 0x8;
pub const FAN_INFO_TYPE_LEVEL: u32 = 0;
pub const CMD_GET_FREQ_LEVEL_NUM: u32 = 0x9;
pub const CMD_GET_FREQ_BOOST_LEVEL: u32 = 0x10;
pub const CMD_GET_FREQ_LEVEL_INFO: u32 = 0x11;
pub const CMD_GET_FREQ_INFO: u32 = 0x12;
pub const CMD_SET_FREQ_INFO: u32 = 0x13;
pub const FREQ_INFO_TYPE_FREQ: u32 = 0;
pub const FREQ_INFO_TYPE_LEVEL: u32 = 1;
pub const FREQ_MAX_LEVEL: i32 = 16;

#[repr(C)]
pub struct Loongson3FreqData {
    pub def_freq_level: u32,
    pub table: [CpufreqFrequencyTable; 0],
}

static mut CPUFREQ_MUTEX: [Mutex; MAX_PACKAGES] = [Mutex::new(); MAX_PACKAGES];
static mut LOONGSON3_CPUFREQ_DRIVER: CpufreqDriver = CpufreqDriver::zeroed();
static mut FREQ_DATA: PerCpu<*mut Loongson3FreqData> = PerCpu::zeroed();

unsafe fn do_service_request(id: u32, info: u32, cmd: u32, val: u32, extra: u32) -> i32 {
    let mut retries: i32;
    let cpu = raw_smp_processor_id();
    let package = cpu_data[cpu].package;
    let mut msg: SmcMessage;
    let mut last: SmcMessage;

    mutex_lock(&mut CPUFREQ_MUTEX[package]);
    last.value = iocsr_read32(LOONGARCH_IOCSR_SMCMBX);
    if (*(&last.bits)).complete == 0 {
        mutex_unlock(&mut CPUFREQ_MUTEX[package]);
        return -EPERM;
    }

    msg.bits = SmcMessageBits { id, info, val, cmd, extra, complete: 0 };
    iocsr_write32(msg.value, LOONGARCH_IOCSR_SMCMBX);
    iocsr_write32(iocsr_read32(LOONGARCH_IOCSR_MISC_FUNC) | IOCSR_MISC_FUNC_SOFT_INT,
                  LOONGARCH_IOCSR_MISC_FUNC);

    retries = 0;
    while retries < 10000 {
        msg.value = iocsr_read32(LOONGARCH_IOCSR_SMCMBX);
        if (*(&msg.bits)).complete != 0 {
            break;
        }
        usleep_range(8, 12);
        retries += 1;
    }

    if (*(&msg.bits)).complete == 0 || (*(&msg.bits)).cmd != CMD_OK {
        mutex_unlock(&mut CPUFREQ_MUTEX[package]);
        return -EPERM;
    }
    mutex_unlock(&mut CPUFREQ_MUTEX[package]);
    (*(&msg.bits)).val as i32
}

unsafe fn loongson3_cpufreq_get(cpu: u32) -> u32 {
    let ret = do_service_request(cpu, FREQ_INFO_TYPE_FREQ, CMD_GET_FREQ_INFO, 0, 0);
    (ret as u32).wrapping_mul(KILO)
}

unsafe fn loongson3_cpufreq_target(policy: *mut CpufreqPolicy, index: u32) -> i32 {
    let ret = do_service_request(cpu_data[(*policy).cpu].core,
                                 FREQ_INFO_TYPE_LEVEL, CMD_SET_FREQ_INFO, index, 0);
    if ret >= 0 { 0 } else { ret }
}

unsafe fn configure_freq_table(cpu: i32) -> i32 {
    let mut i: i32;
    let mut ret: i32;
    let mut boost_level: i32;
    let mut max_level: i32;
    let mut freq_level: i32;
    let pdev = cpufreq_get_driver_data();
    let mut data: *mut Loongson3FreqData;

    if per_cpu(FREQ_DATA, cpu) != core::ptr::null_mut() { return 0; }
    ret = do_service_request(cpu as u32, 0, CMD_GET_FREQ_LEVEL_NUM, 0, 0);
    if ret < 0 { return ret; }
    max_level = ret;
    ret = do_service_request(cpu as u32, 0, CMD_GET_FREQ_BOOST_LEVEL, 0, 0);
    if ret < 0 { return ret; }
    boost_level = ret;
    freq_level = core::cmp::min(max_level, FREQ_MAX_LEVEL);
    data = devm_kzalloc(&mut (*pdev).dev, struct_size::<Loongson3FreqData>(freq_level + 1), GFP_KERNEL);
    if data.is_null() { return -ENOMEM; }
    (*data).def_freq_level = (boost_level - 1) as u32;
    i = 0;
    while i < freq_level {
        ret = do_service_request(cpu as u32, FREQ_INFO_TYPE_FREQ, CMD_GET_FREQ_LEVEL_INFO, i as u32, 0);
        if ret < 0 { devm_kfree(&mut (*pdev).dev, data); return ret; }
        table_at(data, i).frequency = (ret as u32).wrapping_mul(KILO);
        table_at(data, i).flags = if i >= boost_level { CPUFREQ_BOOST_FREQ } else { 0 };
        i += 1;
    }
    table_at(data, freq_level).flags = 0;
    table_at(data, freq_level).frequency = CPUFREQ_TABLE_END;
    per_cpu(FREQ_DATA, cpu) = data;
    0
}

unsafe fn loongson3_cpufreq_cpu_init(policy: *mut CpufreqPolicy) -> i32 {
    let cpu = (*policy).cpu;
    let ret = configure_freq_table(cpu);
    if ret < 0 { return ret; }
    (*policy).cpuinfo.transition_latency = 10000;
    (*policy).freq_table = (*per_cpu(FREQ_DATA, cpu)).table.as_ptr() as *mut CpufreqFrequencyTable;
    (*policy).suspend_freq = table_at(per_cpu(FREQ_DATA, cpu), (*per_cpu(FREQ_DATA, cpu)).def_freq_level as i32).frequency;
    cpumask_copy((*policy).cpus, topology_sibling_cpumask(cpu));
    for_each_cpu(i, (*policy).cpus) {
        if i != cpu { per_cpu(FREQ_DATA, i) = per_cpu(FREQ_DATA, cpu); }
    }
    0
}

unsafe fn loongson3_cpufreq_cpu_exit(policy: *mut CpufreqPolicy) {
    loongson3_cpufreq_target(policy, (*per_cpu(FREQ_DATA, (*policy).cpu)).def_freq_level);
}
unsafe fn loongson3_cpufreq_cpu_online(_policy: *mut CpufreqPolicy) -> i32 { 0 }
unsafe fn loongson3_cpufreq_cpu_offline(_policy: *mut CpufreqPolicy) -> i32 { 0 }

static mut LOONGSON3_CPUFREQ_DRIVER: CpufreqDriver = CpufreqDriver {
    name: "loongson3\0".as_ptr() as *const i8,
    flags: CPUFREQ_CONST_LOOPS,
    init: Some(loongson3_cpufreq_cpu_init),
    exit: Some(loongson3_cpufreq_cpu_exit),
    online: Some(loongson3_cpufreq_cpu_online),
    offline: Some(loongson3_cpufreq_cpu_offline),
    get: Some(loongson3_cpufreq_get),
    target_index: Some(loongson3_cpufreq_target),
    verify: Some(cpufreq_generic_frequency_table_verify),
    set_boost: Some(cpufreq_boost_set_sw),
    suspend: Some(cpufreq_generic_suspend),
    driver_data: core::ptr::null_mut(),
};

static mut LOONGSON3_PLATFORM_DRIVER: PlatformDriver = PlatformDriver {
    driver: Driver { name: "loongson3_cpufreq\0".as_ptr() as *const i8 },
    id_table: CPUFREQ_ID_TABLE.as_ptr(),
    probe: Some(loongson3_cpufreq_probe),
    remove: Some(loongson3_cpufreq_remove),
};

#[repr(C)]
static CPUFREQ_ID_TABLE: [PlatformDeviceId; 2] = [
    PlatformDeviceId { name: "loongson3_cpufreq\0".as_ptr() as *const i8 },
    PlatformDeviceId { name: core::ptr::null() },
];

unsafe fn loongson3_cpufreq_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut i = 0;
    while i < MAX_PACKAGES {
        let ret = devm_mutex_init(&mut (*pdev).dev, &mut CPUFREQ_MUTEX[i]);
        if ret != 0 { return ret; }
        i += 1;
    }
    let ret = do_service_request(0, 0, CMD_GET_VERSION, 0, 0);
    if ret <= 0 { return -EPERM; }
    let ret = do_service_request(FEATURE_DVFS, 0, CMD_SET_FEATURE,
                                 FEATURE_DVFS_ENABLE | FEATURE_DVFS_BOOST, 0);
    if ret < 0 { return -EPERM; }
    LOONGSON3_CPUFREQ_DRIVER.driver_data = pdev as *mut core::ffi::c_void;
    let ret = cpufreq_register_driver(&mut LOONGSON3_CPUFREQ_DRIVER);
    if ret != 0 { return ret; }
    pr_info!("cpufreq: Loongson-3 CPU frequency driver.\n");
    0
}

unsafe fn loongson3_cpufreq_remove(_pdev: *mut PlatformDevice) {
    cpufreq_unregister_driver(&mut LOONGSON3_CPUFREQ_DRIVER);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
