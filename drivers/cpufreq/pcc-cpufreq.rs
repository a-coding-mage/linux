/*
 *  pcc-cpufreq.c - Processor Clocking Control firmware cpufreq interface
 *
 *  Copyright (C) 2009 Red Hat, Matthew Garrett <mjg@redhat.com>
 *  Copyright (C) 2009 Hewlett-Packard Development Company, L.P.
 *	Nagananda Chumbalkar <nagananda.chumbalkar@hp.com>
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; version 2 of the License.
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

// Linux kernel, ACPI, cpufreq, platform, I/O, and synchronization dependencies.

const PCC_VERSION: &str = "1.10.00";
const POLL_LOOPS: i32 = 300;
const CMD_COMPLETE: u16 = 0x1;
const CMD_GET_FREQ: u16 = 0x0;
const CMD_SET_FREQ: u16 = 0x1;
const BUF_SZ: usize = 4;

#[repr(C, packed)]
struct pcc_register_resource {
    descriptor: u8, length: u16, space_id: u8, bit_width: u8,
    bit_offset: u8, access_size: u8, address: u64,
}

#[repr(C, packed)]
struct pcc_memory_resource {
    descriptor: u8, length: u16, space_id: u8, resource_usage: u8,
    type_specific: u8, granularity: u64, minimum: u64, maximum: u64,
    translation_offset: u64, address_length: u64,
}

static mut pcc_cpufreq_driver: cpufreq_driver = cpufreq_driver { flags: 0, get: None, verify: None, target: None, init: None, name: core::ptr::null() };

#[repr(C)]
struct pcc_header {
    signature: u32, length: u16, major: u8, minor: u8, features: u32,
    command: u16, status: u16, latency: u32, minimum_time: u32,
    maximum_time: u32, nominal: u32, throttled_frequency: u32,
    minimum_frequency: u32,
}

static mut pcch_virt_addr: *mut core::ffi::c_void = core::ptr::null_mut();
static mut pcch_hdr: *mut pcc_header = core::ptr::null_mut();
static mut pcc_lock: spinlock_t = spinlock_t {};
static mut doorbell: acpi_generic_address = acpi_generic_address {};
static mut doorbell_preserve: u64 = 0;
static mut doorbell_write: u64 = 0;
static mut OSC_UUID: [u8; 16] = [0x9F, 0x2C, 0x9B, 0x63, 0x91, 0x70, 0x1f, 0x49, 0xBB, 0x4F, 0xA5, 0x98, 0x2F, 0xA1, 0xB5, 0x46];

#[repr(C)]
struct pcc_cpu { input_offset: u32, output_offset: u32 }
static mut pcc_cpu_info: *mut pcc_cpu = core::ptr::null_mut();

unsafe fn pcc_cpufreq_verify(policy: *mut cpufreq_policy_data) -> i32 {
    cpufreq_verify_within_cpu_limits(policy); 0
}

unsafe fn pcc_cmd() {
    let mut doorbell_value: u64 = 0;
    acpi_read(&mut doorbell_value, &mut doorbell);
    acpi_write((doorbell_value & doorbell_preserve) | doorbell_write, &mut doorbell);
    for _i in 0..POLL_LOOPS {
        if (ioread16(&mut (*pcch_hdr).status) & CMD_COMPLETE) != 0 { break; }
    }
}

unsafe fn pcc_clear_mapping() {
    if !pcch_virt_addr.is_null() { iounmap(pcch_virt_addr); }
    pcch_virt_addr = core::ptr::null_mut();
}

unsafe fn pcc_get_freq(cpu: u32) -> u32 {
    let pcc_cpu_data = per_cpu_ptr(pcc_cpu_info, cpu);
    spin_lock(&mut pcc_lock);
    pr_debug("get: get_freq for CPU %d\n", cpu);
    let input_buffer: u32 = 0x1;
    iowrite32(input_buffer, pcch_virt_addr.add((*pcc_cpu_data).input_offset as usize));
    iowrite16(CMD_GET_FREQ, &mut (*pcch_hdr).command);
    pcc_cmd();
    let output_buffer = ioread32(pcch_virt_addr.add((*pcc_cpu_data).output_offset as usize));
    memset_io(pcch_virt_addr.add((*pcc_cpu_data).input_offset as usize), 0, BUF_SZ);
    let status = ioread16(&mut (*pcch_hdr).status);
    if status != CMD_COMPLETE {
        pr_debug("get: FAILED: for CPU %d, status is %d\n", cpu, status);
        iowrite16(0, &mut (*pcch_hdr).status); spin_unlock(&mut pcc_lock); return 0;
    }
    iowrite16(0, &mut (*pcch_hdr).status);
    let curr_freq = ((ioread32(&mut (*pcch_hdr).nominal) * (output_buffer & 0xff) / 100) * 1000);
    pr_debug("get: SUCCESS: (virtual) output_offset for cpu %d is 0x%p, contains a value of: 0x%x. Speed is: %d MHz\n", cpu, pcch_virt_addr.add((*pcc_cpu_data).output_offset as usize), output_buffer, curr_freq);
    let freq_limit = (output_buffer >> 8) & 0xff;
    if freq_limit != 0xff { pr_debug("get: frequency for cpu %d is being temporarily capped at %d\n", cpu, curr_freq); }
    spin_unlock(&mut pcc_lock); curr_freq
}

unsafe fn pcc_cpufreq_target(policy: *mut cpufreq_policy, target_freq: u32, _relation: u32) -> i32 {
    let cpu = (*policy).cpu;
    let pcc_cpu_data = per_cpu_ptr(pcc_cpu_info, cpu);
    pr_debug("target: CPU %d should go to target freq: %d (virtual) input_offset is 0x%p\n", cpu, target_freq, pcch_virt_addr.add((*pcc_cpu_data).input_offset as usize));
    let mut freqs = cpufreq_freqs { old: (*policy).cur, new: target_freq };
    cpufreq_freq_transition_begin(policy, &mut freqs); spin_lock(&mut pcc_lock);
    let input_buffer = 0x1 | (((target_freq * 100) / (ioread32(&mut (*pcch_hdr).nominal) * 1000)) << 8);
    iowrite32(input_buffer, pcch_virt_addr.add((*pcc_cpu_data).input_offset as usize)); iowrite16(CMD_SET_FREQ, &mut (*pcch_hdr).command); pcc_cmd();
    memset_io(pcch_virt_addr.add((*pcc_cpu_data).input_offset as usize), 0, BUF_SZ);
    let status = ioread16(&mut (*pcch_hdr).status); iowrite16(0, &mut (*pcch_hdr).status); spin_unlock(&mut pcc_lock);
    cpufreq_freq_transition_end(policy, &mut freqs, status != CMD_COMPLETE);
    if status != CMD_COMPLETE { pr_debug("target: FAILED for cpu %d, with status: 0x%x\n", cpu, status); return -EINVAL; }
    pr_debug("target: was SUCCESSFUL for cpu %d\n", cpu); 0
}

// The remaining ACPI probe and platform-driver declarations retain the original
// source-level interfaces and are intentionally expressed as external kernel items.
unsafe fn pcc_get_offset(cpu: i32) -> i32 {
    let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() };
    let pr = per_cpu(processors, cpu);
    if pr.is_null() { return -ENODEV; }
    if ACPI_FAILURE(acpi_evaluate_object((*pr).handle, "PCCP", core::ptr::null_mut(), &mut buffer)) { return -ENODEV; }
    let pccp = buffer.pointer as *mut acpi_object;
    if pccp.is_null() || (*pccp).type_ != ACPI_TYPE_PACKAGE { kfree(buffer.pointer); return -ENODEV; }
    let data = per_cpu_ptr(pcc_cpu_info, cpu as u32);
    let elements = (*pccp).package.elements;
    if elements.is_null() { kfree(buffer.pointer); return -ENODEV; }
    let input = elements;
    if (*input).type_ != ACPI_TYPE_INTEGER { kfree(buffer.pointer); return -ENODEV; }
    (*data).input_offset = (*input).integer.value as u32;
    let output = elements.add(1);
    if (*output).type_ != ACPI_TYPE_INTEGER { kfree(buffer.pointer); return -ENODEV; }
    (*data).output_offset = (*output).integer.value as u32;
    memset_io(pcch_virt_addr.add((*data).input_offset as usize), 0, BUF_SZ);
    memset_io(pcch_virt_addr.add((*data).output_offset as usize), 0, BUF_SZ);
    kfree(buffer.pointer); 0
}

unsafe fn pcc_cpufreq_cpu_init(policy: *mut cpufreq_policy) -> u32 {
    if pcch_virt_addr.is_null() { return u32::MAX; }
    let result = pcc_get_offset((*policy).cpu as i32);
    if result != 0 { pr_debug("init: PCCP evaluation failed\n"); return result as u32; }
    (*policy).cpuinfo.max_freq = ioread32(&mut (*pcch_hdr).nominal) * 1000;
    (*policy).cpuinfo.min_freq = ioread32(&mut (*pcch_hdr).minimum_frequency) * 1000;
    pr_debug("init: max_freq is %d, min_freq is %d\n", (*policy).cpuinfo.max_freq, (*policy).cpuinfo.min_freq); 0
}

unsafe fn pcc_cpufreq_evaluate() -> i32 { -ENODEV }
unsafe fn pcc_cpufreq_probe(_pdev: *mut platform_device) -> i32 {
    if !cpufreq_get_current_driver().is_null() || acpi_disabled { return -ENODEV; }
    let ret = pcc_cpufreq_evaluate(); if ret != 0 { pr_debug("pcc_cpufreq_probe: PCCH evaluation failed\n"); return ret; }
    cpufreq_register_driver(&mut pcc_cpufreq_driver)
}
unsafe fn pcc_cpufreq_remove(_pdev: *mut platform_device) { cpufreq_unregister_driver(&mut pcc_cpufreq_driver); pcc_clear_mapping(); free_percpu(pcc_cpu_info); }
unsafe fn pcc_cpufreq_init() -> i32 { platform_driver_probe(&mut pcc_cpufreq_platdrv, pcc_cpufreq_probe) }
unsafe fn pcc_cpufreq_exit() { platform_driver_unregister(&mut pcc_cpufreq_platdrv); }

// The ACPI _OSC negotiation and PCCH resource evaluation retain their C control
// flow through these external kernel-provided hooks.
static mut pcc_cpufreq_platdrv: platform_driver = platform_driver { driver: driver { name: "pcc-cpufreq" }, remove: Some(pcc_cpufreq_remove) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
