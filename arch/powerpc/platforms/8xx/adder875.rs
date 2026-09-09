// SPDX-License-Identifier: GPL-2.0-only
/* Analogue & Micro Adder MPC875 board support
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the Linux PowerPC platform and CPM1 headers.

#[repr(C)]
struct CpmPin {
    port: core::ffi::c_int,
    pin: core::ffi::c_int,
    flags: core::ffi::c_int,
}

extern "C" {
    static mut mpc8xx_immr: *mut Mpc8xxImmr;

    fn cpm1_set_pin(port: core::ffi::c_int, pin: core::ffi::c_int, flags: core::ffi::c_int);
    fn cpm1_clk_setup(
        clk: core::ffi::c_int,
        brg: core::ffi::c_int,
        mode: core::ffi::c_int,
    );
    fn cpm_reset();
    fn clrbits32(address: *mut u32, mask: u32);
    fn of_platform_bus_probe(
        node: *mut core::ffi::c_void,
        ids: *const OfDeviceId,
        parent: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn mpc8xx_pic_init();
    fn mpc8xx_get_irq() -> core::ffi::c_int;
    fn mpc8xx_restart(cmd: *const core::ffi::c_char) -> !;
    fn udbg_progress(s: *const core::ffi::c_char, hex: core::ffi::c_uint);
}

#[repr(C)]
struct Mpc8xxImmr {
    _reserved: [u8; 0],
}

#[repr(C)]
struct OfDeviceId {
    compatible: *const core::ffi::c_char,
}

// CPM_PORT*, CPM_PIN_*, CPM_CLK_*, CPM_BRG1, and CPM_CLK_RTX are supplied by
// the CPM1 platform headers.
static mut ADDER875_PINS: [CpmPin; 31] = [
    // SMC1
    CpmPin { port: CPM_PORTB, pin: 24, flags: CPM_PIN_INPUT },
    CpmPin { port: CPM_PORTB, pin: 25, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },

    // MII1
    CpmPin { port: CPM_PORTA, pin: 0, flags: CPM_PIN_INPUT },
    CpmPin { port: CPM_PORTA, pin: 1, flags: CPM_PIN_INPUT },
    CpmPin { port: CPM_PORTA, pin: 2, flags: CPM_PIN_INPUT },
    CpmPin { port: CPM_PORTA, pin: 3, flags: CPM_PIN_INPUT },
    CpmPin { port: CPM_PORTA, pin: 4, flags: CPM_PIN_OUTPUT },
    CpmPin { port: CPM_PORTA, pin: 10, flags: CPM_PIN_OUTPUT },
    CpmPin { port: CPM_PORTA, pin: 11, flags: CPM_PIN_OUTPUT },
    CpmPin { port: CPM_PORTB, pin: 19, flags: CPM_PIN_INPUT },
    CpmPin { port: CPM_PORTB, pin: 31, flags: CPM_PIN_INPUT },
    CpmPin { port: CPM_PORTC, pin: 12, flags: CPM_PIN_INPUT },
    CpmPin { port: CPM_PORTC, pin: 13, flags: CPM_PIN_INPUT },
    CpmPin { port: CPM_PORTE, pin: 30, flags: CPM_PIN_OUTPUT },
    CpmPin { port: CPM_PORTE, pin: 31, flags: CPM_PIN_OUTPUT },

    // MII2
    CpmPin { port: CPM_PORTE, pin: 14, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    CpmPin { port: CPM_PORTE, pin: 15, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    CpmPin { port: CPM_PORTE, pin: 16, flags: CPM_PIN_OUTPUT },
    CpmPin { port: CPM_PORTE, pin: 17, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    CpmPin { port: CPM_PORTE, pin: 18, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    CpmPin { port: CPM_PORTE, pin: 19, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    CpmPin { port: CPM_PORTE, pin: 20, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    CpmPin { port: CPM_PORTE, pin: 21, flags: CPM_PIN_OUTPUT },
    CpmPin { port: CPM_PORTE, pin: 22, flags: CPM_PIN_OUTPUT },
    CpmPin { port: CPM_PORTE, pin: 23, flags: CPM_PIN_OUTPUT },
    CpmPin { port: CPM_PORTE, pin: 24, flags: CPM_PIN_OUTPUT },
    CpmPin { port: CPM_PORTE, pin: 25, flags: CPM_PIN_OUTPUT },
    CpmPin { port: CPM_PORTE, pin: 26, flags: CPM_PIN_OUTPUT },
    CpmPin { port: CPM_PORTE, pin: 27, flags: CPM_PIN_OUTPUT },
    CpmPin { port: CPM_PORTE, pin: 28, flags: CPM_PIN_OUTPUT },
    CpmPin { port: CPM_PORTE, pin: 29, flags: CPM_PIN_OUTPUT },
];

unsafe fn init_ioports() {
    let mut i = 0;
    while i < ADDER875_PINS.len() {
        let pin = &ADDER875_PINS[i];
        cpm1_set_pin(pin.port, pin.pin, pin.flags);
        i += 1;
    }

    cpm1_clk_setup(CPM_CLK_SMC1, CPM_BRG1, CPM_CLK_RTX);

    /* Set FEC1 and FEC2 to MII mode */
    clrbits32(
        &mut (*mpc8xx_immr).im_cpm.cp_cptr as *mut u32,
        0x00000180,
    );
}

unsafe fn adder875_setup() {
    cpm_reset();
    init_ioports();
}

static OF_BUS_IDS: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"simple-bus\0".as_ptr() as *const core::ffi::c_char },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe extern "C" fn declare_of_platform_devices() -> core::ffi::c_int {
    of_platform_bus_probe(core::ptr::null_mut(), OF_BUS_IDS.as_ptr(), core::ptr::null_mut());
    0
}

// machine_device_initcall(adder875, declare_of_platform_devices);
// define_machine(adder875) {
//     .name = "Adder MPC875",
//     .compatible = "analogue-and-micro,adder875",
//     .setup_arch = adder875_setup,
//     .init_IRQ = mpc8xx_pic_init,
//     .get_irq = mpc8xx_get_irq,
//     .restart = mpc8xx_restart,
//     .progress = udbg_progress,
// };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
