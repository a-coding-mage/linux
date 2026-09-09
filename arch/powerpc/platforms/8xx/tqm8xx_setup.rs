/*
 * Platform setup for the MPC8xx based boards from TQM.
 *
 * Heiko Schocher <hs@denx.de>
 * Copyright 2010 DENX Software Engineering GmbH
 *
 * based on:
 * Vitaly Bordug <vbordug@ru.mvista.com>
 *
 * Copyright 2005 MontaVista Software Inc.
 *
 * Heavily modified by Scott Wood <scottwood@freescale.com>
 * Copyright 2007 Freescale Semiconductor, Inc.
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct cpm_pin {
    pub port: ::core::ffi::c_int,
    pub pin: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
}

static mut tqm8xx_pins: [cpm_pin; 9] = [
    cpm_pin { port: CPM_PORTB, pin: 24, flags: CPM_PIN_INPUT },
    cpm_pin { port: CPM_PORTB, pin: 25, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: CPM_PORTA, pin: 5, flags: CPM_PIN_INPUT },
    cpm_pin { port: CPM_PORTA, pin: 7, flags: CPM_PIN_INPUT },
    cpm_pin { port: CPM_PORTA, pin: 14, flags: CPM_PIN_INPUT },
    cpm_pin { port: CPM_PORTA, pin: 15, flags: CPM_PIN_INPUT },
    cpm_pin { port: CPM_PORTC, pin: 15, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: CPM_PORTC, pin: 10, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_GPIO },
    cpm_pin { port: CPM_PORTC, pin: 11, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_GPIO },
];

static mut tqm8xx_fec_pins: [cpm_pin; 13] = [
    cpm_pin { port: CPM_PORTD, pin: 3, flags: CPM_PIN_OUTPUT },
    cpm_pin { port: CPM_PORTD, pin: 4, flags: CPM_PIN_OUTPUT },
    cpm_pin { port: CPM_PORTD, pin: 5, flags: CPM_PIN_OUTPUT },
    cpm_pin { port: CPM_PORTD, pin: 6, flags: CPM_PIN_OUTPUT },
    cpm_pin { port: CPM_PORTD, pin: 7, flags: CPM_PIN_OUTPUT },
    cpm_pin { port: CPM_PORTD, pin: 8, flags: CPM_PIN_OUTPUT },
    cpm_pin { port: CPM_PORTD, pin: 9, flags: CPM_PIN_OUTPUT },
    cpm_pin { port: CPM_PORTD, pin: 10, flags: CPM_PIN_OUTPUT },
    cpm_pin { port: CPM_PORTD, pin: 11, flags: CPM_PIN_OUTPUT },
    cpm_pin { port: CPM_PORTD, pin: 12, flags: CPM_PIN_OUTPUT },
    cpm_pin { port: CPM_PORTD, pin: 13, flags: CPM_PIN_OUTPUT },
    cpm_pin { port: CPM_PORTD, pin: 14, flags: CPM_PIN_OUTPUT },
    cpm_pin { port: CPM_PORTD, pin: 15, flags: CPM_PIN_OUTPUT },
];

unsafe fn init_pins(n: ::core::ffi::c_int, mut pin: *mut cpm_pin) {
    let mut i: ::core::ffi::c_int = 0;
    while i < n {
        cpm1_set_pin((*pin).port, (*pin).pin, (*pin).flags);
        pin = pin.add(1);
        i += 1;
    }
}

unsafe fn init_ioports() {
    let mut dnode: *mut device_node;
    let mut prop: *mut property;
    let mut len: ::core::ffi::c_int = 0;

    init_pins(tqm8xx_pins.len() as ::core::ffi::c_int, tqm8xx_pins.as_mut_ptr());
    cpm1_clk_setup(CPM_CLK_SMC1, CPM_BRG1, CPM_CLK_RTX);

    dnode = of_find_node_by_name(core::ptr::null_mut(), c"aliases".as_ptr());
    if dnode.is_null() {
        return;
    }
    prop = of_find_property(dnode, c"ethernet1".as_ptr(), &mut len);
    of_node_put(dnode);
    if prop.is_null() {
        return;
    }

    init_pins(tqm8xx_fec_pins.len() as ::core::ffi::c_int, tqm8xx_fec_pins.as_mut_ptr());
}

unsafe fn tqm8xx_setup_arch() {
    cpm_reset();
    init_ioports();
}

#[repr(C)]
pub struct of_device_id {
    pub name: *const ::core::ffi::c_char,
    pub compatible: *const ::core::ffi::c_char,
}

static of_bus_ids: [of_device_id; 4] = [
    of_device_id { name: c"soc".as_ptr(), compatible: core::ptr::null() },
    of_device_id { name: c"cpm".as_ptr(), compatible: core::ptr::null() },
    of_device_id { name: c"localbus".as_ptr(), compatible: core::ptr::null() },
    of_device_id { name: core::ptr::null(), compatible: c"simple-bus".as_ptr() },
];

unsafe fn declare_of_platform_devices() -> ::core::ffi::c_int {
    of_platform_bus_probe(core::ptr::null_mut(), of_bus_ids.as_ptr(), core::ptr::null_mut());
    0
}

// machine_device_initcall(tqm8xx, declare_of_platform_devices);
// define_machine(tqm8xx) {
//     .name = "TQM8xx", .compatible = "tqc,tqm8xx",
//     .setup_arch = tqm8xx_setup_arch, .init_IRQ = mpc8xx_pic_init,
//     .get_irq = mpc8xx_get_irq, .restart = mpc8xx_restart,
//     .calibrate_decr = mpc8xx_calibrate_decr,
//     .set_rtc_time = mpc8xx_set_rtc_time,
//     .get_rtc_time = mpc8xx_get_rtc_time, .progress = udbg_progress,
// }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
