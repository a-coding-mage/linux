// SPDX-License-Identifier: GPL-2.0-only
/*
 * CPU idle Marvell Kirkwood SoCs
 *
 * The cpu idle uses wait-for-interrupt and DDR self refresh in order
 * to implement two idle states -
 * #1 wait-for-interrupt
 * #2 wait-for-interrupt and DDR self refresh
 *
 * Maintainer: Jason Cooper <jason@lakedaemon.net>
 * Maintainer: Andrew Lunn <andrew@lunn.ch>
 */

// Dependencies supplied by the kernel and by other translated files.
use core::ffi::{c_char, c_int, c_void};

const KIRKWOOD_MAX_STATES: usize = 2;

#[repr(C)]
pub struct cpuidle_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpuidle_state {
    pub enter: Option<unsafe extern "C" fn(
        dev: *mut cpuidle_device,
        drv: *mut cpuidle_driver,
        index: c_int,
    ) -> c_int>,
    pub exit_latency: u32,
    pub target_residency: u32,
    pub name: *const c_char,
    pub desc: *const c_char,
}

#[repr(C)]
pub struct cpuidle_driver {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub states: [cpuidle_state; KIRKWOOD_MAX_STATES],
    pub state_count: c_int,
}

extern "C" {
    static mut THIS_MODULE: c_void;

    fn writel(value: u32, address: *mut c_void);
    fn cpu_do_idle();
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: c_int,
    ) -> *mut c_void;
    fn IS_ERR(pointer: *mut c_void) -> bool;
    fn PTR_ERR(pointer: *mut c_void) -> c_int;
    fn cpuidle_register(driver: *mut cpuidle_driver, device: *mut c_void) -> c_int;
    fn cpuidle_unregister(driver: *mut cpuidle_driver);
}

// ARM_CPUIDLE_WFI_STATE is supplied by asm/cpuidle.h.
extern "C" {
    static ARM_CPUIDLE_WFI_STATE: cpuidle_state;
}

static mut ddr_operation_base: *mut c_void = core::ptr::null_mut();

/* Actual code that puts the SoC in different idle states */
unsafe extern "C" fn kirkwood_enter_idle(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    index: c_int,
) -> c_int {
    writel(0x7, ddr_operation_base);
    cpu_do_idle();

    index
}

static mut kirkwood_idle_driver: cpuidle_driver = cpuidle_driver {
    name: b"kirkwood_idle\0".as_ptr() as *const c_char,
    owner: unsafe { &mut THIS_MODULE as *mut c_void },
    states: [
        unsafe { ARM_CPUIDLE_WFI_STATE },
        cpuidle_state {
            enter: Some(kirkwood_enter_idle),
            exit_latency: 10,
            target_residency: 100000,
            name: b"DDR SR\0".as_ptr() as *const c_char,
            desc: b"WFI and DDR Self Refresh\0".as_ptr() as *const c_char,
        },
    ],
    state_count: KIRKWOOD_MAX_STATES as c_int,
};

/* Initialize CPU idle by registering the idle states */
unsafe extern "C" fn kirkwood_cpuidle_probe(pdev: *mut platform_device) -> c_int {
    ddr_operation_base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(ddr_operation_base) {
        return PTR_ERR(ddr_operation_base);
    }

    cpuidle_register(&mut kirkwood_idle_driver, core::ptr::null_mut())
}

unsafe extern "C" fn kirkwood_cpuidle_remove(_pdev: *mut platform_device) {
    cpuidle_unregister(&mut kirkwood_idle_driver);
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: driver,
}

#[repr(C)]
struct driver {
    name: *const c_char,
}

static mut kirkwood_cpuidle_driver: platform_driver = platform_driver {
    probe: Some(kirkwood_cpuidle_probe),
    remove: Some(kirkwood_cpuidle_remove),
    driver: driver {
        name: b"kirkwood_cpuidle\0".as_ptr() as *const c_char,
    },
};

// module_platform_driver(kirkwood_cpuidle_driver);
extern "C" {
    fn module_platform_driver(driver: *mut platform_driver);
}

// MODULE_AUTHOR("Andrew Lunn <andrew@lunn.ch>");
// MODULE_DESCRIPTION("Kirkwood cpu idle driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:kirkwood-cpuidle");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
