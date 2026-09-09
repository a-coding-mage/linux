// SPDX-License-Identifier: GPL-2.0-only
/*
 * Intel SpeedStep SMI driver.
 *
 * (C) 2003  Hiroshi Miura <miura@da-cha.org>
 */

// C headers and symbols supplied by the kernel are external dependencies.

static mut smi_port: i32 = 0;
static mut smi_cmd: i32 = 0;
static mut smi_sig: u32 = 0;
static mut speedstep_processor: enum_speedstep_processor = 0;

static mut speedstep_freqs: [cpufreq_frequency_table; 3] = [
    cpufreq_frequency_table { driver_data: 0, frequency: SPEEDSTEP_HIGH },
    cpufreq_frequency_table { driver_data: 0, frequency: SPEEDSTEP_LOW },
    cpufreq_frequency_table { driver_data: 0, frequency: CPUFREQ_TABLE_END },
];

const GET_SPEEDSTEP_OWNER: u32 = 0;
const GET_SPEEDSTEP_STATE: u32 = 1;
const SET_SPEEDSTEP_STATE: u32 = 2;
const GET_SPEEDSTEP_FREQS: u32 = 4;
const SMI_TRIES: u32 = 5;

unsafe fn speedstep_smi_ownership() -> i32 {
    let command: u32 = (smi_sig & 0xffffff00) | (smi_cmd as u32 & 0xff);
    let function: u32 = GET_SPEEDSTEP_OWNER;
    let magic_data: [u8; 37] = *b"Copyright (c) 1999 Intel Corporation\0";
    let magic = virt_to_phys(magic_data.as_ptr() as *const _);
    let mut result: u32;
    let mut dummy: u32;
    core::arch::asm!(
        "push ebp", "out dx, al", "pop ebp",
        inout("edi") 0u32 => result, out("eax") dummy, out("ebx") dummy,
        out("ecx") dummy, out("edx") dummy, in("esi") magic,
        in("eax") command, in("ebx") function, in("ecx") 0u32,
        in("edx") smi_port as u32, options(nostack)
    );
    result as i32
}

unsafe fn speedstep_smi_get_freqs(low: *mut u32, high: *mut u32) -> i32 {
    if (ist_info.event & 0xffff) == 0 { return -ENODEV; }
    let command = (smi_sig & 0xffffff00) | (smi_cmd as u32 & 0xff);
    let function = GET_SPEEDSTEP_FREQS;
    let mut result: u32;
    let mut high_mhz: u32;
    let mut low_mhz: u32;
    let mut state: u32 = 0;
    let mut edi: u32;
    let mut dummy: u32;
    core::arch::asm!(
        "push ebp", "out dx, al", "pop ebp",
        inout("eax") 0u32 => result, out("ebx") high_mhz, out("ecx") low_mhz,
        inout("edx") smi_port as u32 => state, out("edi") edi, out("esi") dummy,
        in("ebx") function, in("ecx") state, in("esi") 0u32, in("edi") 0u32,
        options(nostack)
    );
    if high_mhz + low_mhz < 600 { return -EINVAL; }
    *high = high_mhz * 1000;
    *low = low_mhz * 1000;
    result as i32
}

unsafe fn speedstep_set_state(state: u32) {
    if state > 1 { return; }
    preempt_disable();
    let mut flags: unsigned_long = 0;
    local_irq_save(&mut flags);
    let command = (smi_sig & 0xffffff00) | (smi_cmd as u32 & 0xff);
    let function = SET_SPEEDSTEP_STATE;
    let mut result = 0u32;
    let mut new_state = 0u32;
    let mut retry = 0u32;
    loop {
        if retry != 0 { local_irq_enable(); mdelay(retry * 50); local_irq_disable(); }
        retry += 1;
        let mut dummy: u32;
        core::arch::asm!(
            "push ebp", "out dx, al", "pop ebp",
            out("ebx") new_state, out("edi") result, out("ecx") dummy,
            out("eax") dummy, out("edx") dummy, out("esi") dummy,
            in("eax") command, in("ebx") function, in("ecx") state,
            in("edx") smi_port as u32, in("esi") 0u32, in("edi") 0u32,
            options(nostack)
        );
        if new_state == state || retry > SMI_TRIES { break; }
    }
    local_irq_restore(flags);
    preempt_enable();
}

unsafe fn speedstep_target(_policy: *mut cpufreq_policy, index: u32) -> i32 {
    speedstep_set_state(index); 0
}

unsafe fn speedstep_cpu_init(policy: *mut cpufreq_policy) -> i32 {
    if (*policy).cpu != 0 { return -ENODEV; }
    let mut result = speedstep_smi_ownership();
    if result != 0 { return -EINVAL; }
    let low = &mut speedstep_freqs[SPEEDSTEP_LOW as usize].frequency;
    let high = &mut speedstep_freqs[SPEEDSTEP_HIGH as usize].frequency;
    result = speedstep_smi_get_freqs(low, high);
    if result != 0 {
        result = speedstep_get_freqs(speedstep_processor, low, high, core::ptr::null_mut(), Some(speedstep_set_state));
        if result != 0 { return result; }
    }
    (*policy).freq_table = speedstep_freqs.as_mut_ptr();
    0
}

unsafe fn speedstep_get(cpu: u32) -> u32 {
    if cpu != 0 { return (-ENODEV) as u32; }
    speedstep_get_frequency(speedstep_processor)
}

unsafe fn speedstep_resume(_policy: *mut cpufreq_policy) -> i32 { speedstep_smi_ownership() }

static mut speedstep_driver: cpufreq_driver = cpufreq_driver {
    name: b"speedstep-smi\0".as_ptr() as *const _, flags: CPUFREQ_NO_AUTO_DYNAMIC_SWITCHING,
    verify: Some(cpufreq_generic_frequency_table_verify), target_index: Some(speedstep_target),
    init: Some(speedstep_cpu_init), get: Some(speedstep_get), resume: Some(speedstep_resume),
};

unsafe fn speedstep_init() -> i32 {
    if !x86_match_cpu(ss_smi_ids.as_ptr()) { return -ENODEV; }
    speedstep_processor = speedstep_detect_processor();
    match speedstep_processor { SPEEDSTEP_CPU_PIII_T | SPEEDSTEP_CPU_PIII_C | SPEEDSTEP_CPU_PIII_C_EARLY => {}, _ => speedstep_processor = 0 }
    if speedstep_processor == 0 { return -ENODEV; }
    if ist_info.signature != 0x47534943 && (smi_port == 0 || smi_cmd == 0) { return -ENODEV; }
    smi_sig = if smi_sig == 1 { 0x47534943 } else { ist_info.signature };
    if smi_port > 0xff || smi_port < 0 { return -EINVAL; } else if smi_port == 0 { smi_port = (ist_info.command & 0xff) as i32; }
    if smi_cmd > 0xff || smi_cmd < 0 { return -EINVAL; } else if smi_cmd == 0 { smi_cmd = ((ist_info.command >> 16) & 0xff) as i32; }
    cpufreq_register_driver(&mut speedstep_driver)
}

unsafe fn speedstep_exit() { cpufreq_unregister_driver(&mut speedstep_driver); }

// Module parameters, metadata, and init/exit registration correspond to the C module declarations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
