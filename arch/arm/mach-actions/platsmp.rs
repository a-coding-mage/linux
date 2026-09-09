// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Actions Semi Leopard
 *
 * This file is based on arm realview smp platform.
 *
 * Copyright 2012 Actions Semi Inc.
 * Author: Actions Semi, Inc.
 *
 * Copyright (c) 2017 Andreas Färber
 */

// Linux kernel dependencies supplied by other translation units.

const OWL_CPU1_ADDR: usize = 0x50;
const OWL_CPU1_FLAG: usize = 0x5c;

const OWL_CPUX_FLAG_BOOT: u32 = 0x55aa;

const OWL_SPS_PG_CTL_PWR_CPU2: u32 = 1 << 5;
const OWL_SPS_PG_CTL_PWR_CPU3: u32 = 1 << 6;
const OWL_SPS_PG_CTL_ACK_CPU2: u32 = 1 << 21;
const OWL_SPS_PG_CTL_ACK_CPU3: u32 = 1 << 22;

type CInt = i32;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smp_operations {
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(max_cpus: u32)>,
    pub smp_boot_secondary:
        Option<unsafe extern "C" fn(cpu: u32, idle: *mut task_struct) -> CInt>,
}

extern "C" {
    fn owl_sps_set_pg(
        base: *mut core::ffi::c_void,
        power: u32,
        ack: u32,
        enable: bool,
    ) -> CInt;
    fn udelay(usecs: u32);
    fn __pa_symbol(symbol: unsafe extern "C" fn());
    fn secondary_startup();
    fn writel(value: u32, address: *mut core::ffi::c_void);
    fn dsb_sev();
    fn mb();
    fn smp_send_reschedule(cpu: u32);
    fn of_find_compatible_node(
        from: *mut device_node,
        ty: *const core::ffi::c_char,
        compatible: *const core::ffi::c_char,
    ) -> *mut device_node;
    fn pr_err(format: *const core::ffi::c_char, ...);
    fn of_iomap(node: *mut device_node, index: CInt) -> *mut core::ffi::c_void;
    fn read_cpuid_part() -> u32;
    fn scu_get_core_count(base: *mut core::ffi::c_void) -> CInt;
    fn pr_debug(format: *const core::ffi::c_char, ...);
    fn scu_enable(base: *mut core::ffi::c_void);
}

static mut scu_base_addr: *mut core::ffi::c_void = core::ptr::null_mut();
static mut sps_base_addr: *mut core::ffi::c_void = core::ptr::null_mut();
static mut timer_base_addr: *mut core::ffi::c_void = core::ptr::null_mut();
static mut ncores: CInt = 0;

unsafe extern "C" fn s500_wakeup_secondary(cpu: u32) -> CInt {
    let mut ret: CInt;

    if cpu > 3 {
        return -22; // -EINVAL
    }

    /* The generic PM domain driver is not available this early. */
    match cpu {
        2 => {
            ret = owl_sps_set_pg(
                sps_base_addr,
                OWL_SPS_PG_CTL_PWR_CPU2,
                OWL_SPS_PG_CTL_ACK_CPU2,
                true,
            );
            if ret != 0 {
                return ret;
            }
        }
        3 => {
            ret = owl_sps_set_pg(
                sps_base_addr,
                OWL_SPS_PG_CTL_PWR_CPU3,
                OWL_SPS_PG_CTL_ACK_CPU3,
                true,
            );
            if ret != 0 {
                return ret;
            }
        }
        _ => {}
    }

    /* wait for CPUx to run to WFE instruction */
    udelay(200);

    writel(
        __pa_symbol(secondary_startup),
        timer_base_addr.add(OWL_CPU1_ADDR + ((cpu - 1) * 4)),
    );
    writel(
        OWL_CPUX_FLAG_BOOT,
        timer_base_addr.add(OWL_CPU1_FLAG + ((cpu - 1) * 4)),
    );

    dsb_sev();
    mb();

    0
}

unsafe extern "C" fn s500_smp_boot_secondary(
    cpu: u32,
    _idle: *mut task_struct,
) -> CInt {
    let ret = s500_wakeup_secondary(cpu);
    if ret != 0 {
        return ret;
    }

    udelay(10);

    smp_send_reschedule(cpu);

    writel(0, timer_base_addr.add(OWL_CPU1_ADDR + ((cpu - 1) * 4)));
    writel(0, timer_base_addr.add(OWL_CPU1_FLAG + ((cpu - 1) * 4)));

    0
}

unsafe extern "C" fn s500_smp_prepare_cpus(_max_cpus: u32) {
    let mut node: *mut device_node;

    node = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"actions,s500-timer\0".as_ptr() as *const core::ffi::c_char,
    );
    if node.is_null() {
        pr_err(b"%s: missing timer\n\0".as_ptr() as *const core::ffi::c_char);
        return;
    }

    timer_base_addr = of_iomap(node, 0);
    if timer_base_addr.is_null() {
        pr_err(b"%s: could not map timer registers\n\0".as_ptr() as *const core::ffi::c_char);
        return;
    }

    node = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"actions,s500-sps\0".as_ptr() as *const core::ffi::c_char,
    );
    if node.is_null() {
        pr_err(b"%s: missing sps\n\0".as_ptr() as *const core::ffi::c_char);
        return;
    }

    sps_base_addr = of_iomap(node, 0);
    if sps_base_addr.is_null() {
        pr_err(b"%s: could not map sps registers\n\0".as_ptr() as *const core::ffi::c_char);
        return;
    }

    if read_cpuid_part() == 0xC09 {
        node = of_find_compatible_node(
            core::ptr::null_mut(),
            core::ptr::null(),
            b"arm,cortex-a9-scu\0".as_ptr() as *const core::ffi::c_char,
        );
        if node.is_null() {
            pr_err(b"%s: missing scu\n\0".as_ptr() as *const core::ffi::c_char);
            return;
        }

        scu_base_addr = of_iomap(node, 0);
        if scu_base_addr.is_null() {
            pr_err(b"%s: could not map scu registers\n\0".as_ptr() as *const core::ffi::c_char);
            return;
        }

        /*
         * While the number of cpus is gathered from dt, also get the
         * number of cores from the scu to verify this value when
         * booting the cores.
         */
        ncores = scu_get_core_count(scu_base_addr);
        pr_debug(b"%s: ncores %d\n\0".as_ptr() as *const core::ffi::c_char);

        scu_enable(scu_base_addr);
    }
}

static S500_SMP_OPS: smp_operations = smp_operations {
    smp_prepare_cpus: Some(s500_smp_prepare_cpus),
    smp_boot_secondary: Some(s500_smp_boot_secondary),
};

// CPU_METHOD_OF_DECLARE(s500_smp, "actions,s500-smp", &s500_smp_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
