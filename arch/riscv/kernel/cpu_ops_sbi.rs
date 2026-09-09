// SPDX-License-Identifier: GPL-2.0-only
/*
 * HSM extension and cpu_ops implementation.
 *
 * Copyright (c) 2020 Western Digital Corporation or its affiliates.
 */

// Dependencies supplied by the kernel headers are intentionally left external.

extern "C" {
    static mut secondary_start_sbi: ::core::ffi::c_char;
}

/*
 * Ordered booting via HSM brings one cpu at a time. However, cpu hotplug can
 * be invoked from multiple threads in parallel. Define an array of boot data
 * to handle that.
 */
static mut boot_data: [sbi_hart_boot_data; NR_CPUS] = [sbi_hart_boot_data {
    task_ptr: ::core::ptr::null_mut(),
    stack_ptr: ::core::ptr::null_mut(),
}; NR_CPUS];

unsafe fn sbi_hsm_hart_start(hartid: ::core::ffi::c_ulong, saddr: ::core::ffi::c_ulong,
                             priv_: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    let ret: sbiret = sbi_ecall(
        SBI_EXT_HSM,
        SBI_EXT_HSM_HART_START,
        hartid,
        saddr,
        priv_,
        0,
        0,
        0,
    );

    sbi_err_map_linux_errno(ret.error)
}

// CONFIG_HOTPLUG_CPU condition from the C source.
#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn sbi_hsm_hart_stop() -> ::core::ffi::c_int {
    let ret: sbiret = sbi_ecall(
        SBI_EXT_HSM,
        SBI_EXT_HSM_HART_STOP,
        0,
        0,
        0,
        0,
        0,
        0,
    );

    sbi_err_map_linux_errno(ret.error)
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn sbi_hsm_hart_get_status(hartid: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    let ret: sbiret = sbi_ecall(
        SBI_EXT_HSM,
        SBI_EXT_HSM_HART_STATUS,
        hartid,
        0,
        0,
        0,
        0,
        0,
    );
    if ret.error != 0 {
        sbi_err_map_linux_errno(ret.error)
    } else {
        ret.value
    }
}

unsafe fn sbi_cpu_start(cpuid: ::core::ffi::c_uint, tidle: *mut task_struct) -> ::core::ffi::c_int {
    let boot_addr: ::core::ffi::c_ulong = __pa_symbol(&secondary_start_sbi as *const _);
    let hartid: ::core::ffi::c_ulong = cpuid_to_hartid_map(cpuid);
    let hsm_data: ::core::ffi::c_ulong;
    let bdata: *mut sbi_hart_boot_data = &mut boot_data[cpuid as usize];

    /* Make sure tidle is updated */
    smp_mb();
    (*bdata).task_ptr = tidle;
    (*bdata).stack_ptr = task_pt_regs(tidle);
    /* Make sure boot data is updated */
    smp_mb();
    hsm_data = __pa(bdata);
    sbi_hsm_hart_start(hartid, boot_addr, hsm_data)
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn sbi_cpu_stop() {
    let ret: ::core::ffi::c_int;

    ret = sbi_hsm_hart_stop();
    pr_crit!("Unable to stop the cpu %d (%d)\n", smp_processor_id(), ret);
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn sbi_cpu_is_stopped(cpuid: ::core::ffi::c_uint) -> bool {
    let rc: ::core::ffi::c_int;
    let hartid: ::core::ffi::c_ulong = cpuid_to_hartid_map(cpuid);

    rc = sbi_hsm_hart_get_status(hartid);

    if rc != SBI_HSM_STATE_STOPPED {
        pr_warn!("HART%lu isn't stopped; status %d\n", hartid, rc);
        return false;
    }

    true
}

#[no_mangle]
pub static cpu_ops_sbi: cpu_operations = cpu_operations {
    cpu_start: Some(sbi_cpu_start),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_stop: Some(sbi_cpu_stop),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_is_stopped: Some(sbi_cpu_is_stopped),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
