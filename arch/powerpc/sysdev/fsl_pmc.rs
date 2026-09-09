// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Suspend/resume support
 *
 * Copyright 2009  MontaVista Software, Inc.
 *
 * Author: Anton Vorontsov <avorontsov@ru.mvista.com>
 */

// Dependencies supplied by the corresponding kernel headers.

#[repr(C)]
struct pmc_regs {
    devdisr: u32,
    devdisr2: u32,
    _reserved: [u32; 2],
    pmcsr: u32,
}

const PMCSR_SLP: u32 = 1 << 17;

static mut pmc_dev: *mut device = core::ptr::null_mut();
static mut pmc_regs_ptr: *mut pmc_regs = core::ptr::null_mut();

unsafe fn pmc_suspend_enter(state: suspend_state_t) -> i32 {
    let mut ret: i32;

    setbits32(&mut (*pmc_regs_ptr).pmcsr, PMCSR_SLP);
    /* At this point, the CPU is asleep. */

    /* Upon resume, wait for SLP bit to be clear. */
    ret = if spin_event_timeout(
        (in_be32(&(*pmc_regs_ptr).pmcsr) & PMCSR_SLP) == 0,
        10000,
        10,
    ) {
        0
    } else {
        -ETIMEDOUT
    };
    if ret != 0 {
        dev_err(pmc_dev, "tired waiting for SLP bit to clear\n");
    }
    ret
}

unsafe fn pmc_suspend_valid(state: suspend_state_t) -> i32 {
    if state != PM_SUSPEND_STANDBY {
        return 0;
    }
    1
}

static pmc_suspend_ops: platform_suspend_ops = platform_suspend_ops {
    valid: Some(pmc_suspend_valid),
    enter: Some(pmc_suspend_enter),
};

unsafe fn pmc_probe(ofdev: *mut platform_device) -> i32 {
    pmc_regs_ptr = of_iomap((*ofdev).dev.of_node, 0);
    if pmc_regs_ptr.is_null() {
        return -ENOMEM;
    }

    pmc_dev = &mut (*ofdev).dev;
    suspend_set_ops(&pmc_suspend_ops);
    0
}

static pmc_ids: [of_device_id; 3] = [
    of_device_id {
        compatible: "fsl,mpc8548-pmc",
    },
    of_device_id {
        compatible: "fsl,mpc8641d-pmc",
    },
    of_device_id {
        compatible: "",
    },
];

static mut pmc_driver: platform_driver = platform_driver {
    driver: driver {
        name: "fsl-pmc",
        of_match_table: pmc_ids.as_ptr(),
    },
    probe: Some(pmc_probe),
};

extern "C" {
    fn setbits32(addr: *mut u32, mask: u32);
    fn in_be32(addr: *const u32) -> u32;
    fn spin_event_timeout(condition: bool, timeout: u32, delay: u32) -> bool;
    fn dev_err(dev: *mut device, format: *const core::ffi::c_char, ...);
    fn of_iomap(node: *mut device_node, index: i32) -> *mut pmc_regs;
    fn suspend_set_ops(ops: *const platform_suspend_ops);
    fn builtin_platform_driver(driver: *mut platform_driver);
}

// External kernel types and constants supplied by the corresponding headers:
// device, device_node, suspend_state_t, platform_device, platform_suspend_ops,
// of_device_id, platform_driver, driver, ETIMEDOUT, ENOMEM,
// PM_SUSPEND_STANDBY.

#[allow(non_upper_case_globals)]
const _builtin_registration: unsafe extern "C" fn(*mut platform_driver) = builtin_platform_driver;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
