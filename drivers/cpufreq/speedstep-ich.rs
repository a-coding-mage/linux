// SPDX-License-Identifier: GPL-2.0-only
/*
 * (C) 2001  Dave Jones, Arjan van de ven.
 * (C) 2002 - 2003  Dominik Brodowski <linux@brodo.de>
 *
 * Based upon reverse engineered information, and on Intel documentation
 * for chipsets ICH2-M and ICH3-M.
 *
 * Many thanks to Ducrot Bruno for finding and fixing the last
 * "missing link" for ICH2-M/ICH3-M support, and to Thomas Winkler
 * for extensive testing.
 *
 * BIG FAT DISCLAIMER: Work in progress code. Possibly *dangerous*
 */

// Linux kernel headers and "speedstep-lib.h" provide the external symbols.

extern "C" {
    static mut speedstep_chipset_dev: *mut pci_dev;
    static mut speedstep_processor: speedstep_processor;
    static mut pmbase: u32;

    fn pci_read_config_dword(dev: *mut pci_dev, offset: u32, value: *mut u32);
    fn pci_read_config_word(dev: *mut pci_dev, offset: u32, value: *mut u16);
    fn pci_write_config_word(dev: *mut pci_dev, offset: u32, value: u16);
    fn pci_get_subsys(vendor: u16, device: u16, subvendor: u16, subdevice: u16, from: *mut pci_dev) -> *mut pci_dev;
    fn pci_dev_put(dev: *mut pci_dev);
    fn inb(port: u32) -> u8;
    fn outb(value: u8, port: u32);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn speedstep_get_frequency(processor: speedstep_processor) -> u32;
    fn speedstep_get_freqs(processor: speedstep_processor, low: *mut u32, high: *mut u32, latency: *mut u32, set_state: unsafe extern "C" fn(u32));
    fn speedstep_detect_processor() -> speedstep_processor;
    fn smp_call_function_single(cpu: u32, func: unsafe extern "C" fn(*mut core::ffi::c_void), info: *mut core::ffi::c_void, wait: bool) -> i32;
    fn cpumask_any_and(a: *mut cpumask, b: *const cpumask) -> u32;
    fn cpumask_copy(dst: *mut cpumask, src: *const cpumask);
    fn topology_sibling_cpumask(cpu: u32) -> *const cpumask;
    static cpu_online_mask: cpumask;
    fn x86_match_cpu(ids: *const x86_cpu_id) -> bool;
    fn cpufreq_register_driver(driver: *mut cpufreq_driver) -> i32;
    fn cpufreq_unregister_driver(driver: *mut cpufreq_driver);
}

#[repr(C)] struct pci_dev { revision: u8 }
#[repr(C)] struct cpumask;
#[repr(C)] struct x86_cpu_id;
#[repr(C)] struct cpufreq_frequency_table { driver_data: u32, frequency: u32, flags: u32 }
#[repr(C)] struct cpufreq_policy { cpus: *mut cpumask, cpu: u32, transition_latency: u32, freq_table: *mut cpufreq_frequency_table }
#[repr(C)] struct cpufreq_driver { name: *const u8, verify: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>, target_index: Option<unsafe extern "C" fn(*mut cpufreq_policy, u32) -> i32>, init: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>, get: Option<unsafe extern "C" fn(u32) -> u32> }
#[repr(C)] enum speedstep_processor { Unknown = 0 }

const ENODEV: i32 = 19;
const EINVAL: i32 = 22;
const SPEEDSTEP_HIGH: u32 = 0;
const SPEEDSTEP_LOW: u32 = 1;
const CPUFREQ_TABLE_END: u32 = 0;

static mut speedstep_freqs: [cpufreq_frequency_table; 3] = [
    cpufreq_frequency_table { driver_data: 0, frequency: SPEEDSTEP_HIGH, flags: 0 },
    cpufreq_frequency_table { driver_data: 0, frequency: SPEEDSTEP_LOW, flags: 0 },
    cpufreq_frequency_table { driver_data: 0, frequency: 0, flags: CPUFREQ_TABLE_END },
];

unsafe extern "C" fn speedstep_find_register() -> i32 {
    if speedstep_chipset_dev.is_null() { return -ENODEV; }
    pci_read_config_dword(speedstep_chipset_dev, 0x40, &mut pmbase);
    if pmbase & 1 == 0 { return -ENODEV; }
    pmbase &= 0xffff_fffe;
    if pmbase == 0 { return -ENODEV; }
    0
}

unsafe extern "C" fn speedstep_set_state(state: u32) {
    if state > 1 { return; }
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    let mut value = inb(pmbase.wrapping_add(0x50));
    value = (value & 0xfe) | state as u8;
    let mut pm2_blk = inb(pmbase.wrapping_add(0x20));
    pm2_blk |= 1;
    outb(pm2_blk, pmbase.wrapping_add(0x20));
    outb(value, pmbase.wrapping_add(0x50));
    pm2_blk &= 0xfe;
    outb(pm2_blk, pmbase.wrapping_add(0x20));
    value = inb(pmbase.wrapping_add(0x50));
    local_irq_restore(flags);
    let _ = value;
}

unsafe extern "C" fn _speedstep_set_state(state: *mut core::ffi::c_void) { speedstep_set_state(*(state as *mut u32)); }

unsafe extern "C" fn speedstep_activate() -> i32 {
    if speedstep_chipset_dev.is_null() { return -EINVAL; }
    let mut value = 0u16;
    pci_read_config_word(speedstep_chipset_dev, 0x00a0, &mut value);
    if value & 8 == 0 { value |= 8; pci_write_config_word(speedstep_chipset_dev, 0x00a0, value); }
    0
}

unsafe extern "C" fn speedstep_detect_chipset() -> u32 {
    speedstep_chipset_dev = pci_get_subsys(0x8086, 0x24cc, 0xffff, 0xffff, core::ptr::null_mut());
    if !speedstep_chipset_dev.is_null() { return 4; }
    speedstep_chipset_dev = pci_get_subsys(0x8086, 0x248c, 0xffff, 0xffff, core::ptr::null_mut());
    if !speedstep_chipset_dev.is_null() { return 3; }
    speedstep_chipset_dev = pci_get_subsys(0x8086, 0x244c, 0xffff, 0xffff, core::ptr::null_mut());
    if speedstep_chipset_dev.is_null() { return 0; }
    let hostbridge = pci_get_subsys(0x8086, 0x1130, 0xffff, 0xffff, core::ptr::null_mut());
    if hostbridge.is_null() { return 2; }
    if (*hostbridge).revision < 5 { speedstep_chipset_dev = core::ptr::null_mut(); pci_dev_put(hostbridge); return 0; }
    pci_dev_put(hostbridge); 2
}

unsafe extern "C" fn get_freq_data(speed: *mut core::ffi::c_void) { *(speed as *mut u32) = speedstep_get_frequency(speedstep_processor); }
unsafe extern "C" fn speedstep_get(cpu: u32) -> u32 { let mut speed = 0; let _ = smp_call_function_single(cpu, get_freq_data, &mut speed as *mut _ as *mut _, true); speed }
unsafe extern "C" fn speedstep_target(policy: *mut cpufreq_policy, index: u32) -> i32 { let cpu = cpumask_any_and((*policy).cpus, &cpu_online_mask); smp_call_function_single(cpu, _speedstep_set_state, &index as *const _ as *mut _, true); 0 }

#[repr(C)] struct get_freqs { policy: *mut cpufreq_policy, ret: i32 }
unsafe extern "C" fn get_freqs_on_cpu(data: *mut core::ffi::c_void) { let gf = &mut *(data as *mut get_freqs); gf.ret = speedstep_get_freqs(speedstep_processor, &mut speedstep_freqs[1].frequency, &mut speedstep_freqs[0].frequency, &mut (*gf.policy).transition_latency, speedstep_set_state); }
unsafe extern "C" fn speedstep_cpu_init(policy: *mut cpufreq_policy) -> i32 { cpumask_copy((*policy).cpus, topology_sibling_cpumask((*policy).cpu)); let cpu = cpumask_any_and((*policy).cpus, &cpu_online_mask); let mut gf = get_freqs { policy, ret: 0 }; smp_call_function_single(cpu, get_freqs_on_cpu, &mut gf as *mut _ as *mut _, true); if gf.ret != 0 { return gf.ret; } (*policy).freq_table = speedstep_freqs.as_mut_ptr(); 0 }

unsafe extern "C" fn speedstep_init() -> i32 { speedstep_processor = speedstep_detect_processor(); if matches!(speedstep_processor, speedstep_processor::Unknown) || speedstep_detect_chipset() == 0 || speedstep_activate() != 0 || speedstep_find_register() != 0 { return -ENODEV; } cpufreq_register_driver(core::ptr::addr_of_mut!(speedstep_driver)) }
unsafe extern "C" fn speedstep_exit() { pci_dev_put(speedstep_chipset_dev); cpufreq_unregister_driver(core::ptr::addr_of_mut!(speedstep_driver)); }

static mut speedstep_driver: cpufreq_driver = cpufreq_driver { name: b"speedstep-ich\0".as_ptr(), verify: None, target_index: Some(speedstep_target), init: Some(speedstep_cpu_init), get: Some(speedstep_get) };

// Module metadata and module_init/module_exit registration are supplied by the kernel build system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
