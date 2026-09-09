/* arch/powerpc/platforms/8xx/mpc86xads_setup.c
 *
 * Platform setup for the Freescale mpc86xads board
 *
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

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct cpm_pin {
    port: i32,
    pin: i32,
    flags: i32,
}

static mut mpc866ads_pins: [cpm_pin; 26] = [
    /* SMC1 */
    cpm_pin { port: CPM_PORTB, pin: 24, flags: CPM_PIN_INPUT }, /* RX */
    cpm_pin { port: CPM_PORTB, pin: 25, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY }, /* TX */

    /* SMC2 */
    cpm_pin { port: CPM_PORTB, pin: 21, flags: CPM_PIN_INPUT }, /* RX */
    cpm_pin { port: CPM_PORTB, pin: 20, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY }, /* TX */

    /* SCC1 */
    cpm_pin { port: CPM_PORTA, pin: 6, flags: CPM_PIN_INPUT }, /* CLK1 */
    cpm_pin { port: CPM_PORTA, pin: 7, flags: CPM_PIN_INPUT }, /* CLK2 */
    cpm_pin { port: CPM_PORTA, pin: 14, flags: CPM_PIN_INPUT }, /* TX */
    cpm_pin { port: CPM_PORTA, pin: 15, flags: CPM_PIN_INPUT }, /* RX */
    cpm_pin { port: CPM_PORTB, pin: 19, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY }, /* TENA */
    cpm_pin { port: CPM_PORTC, pin: 10, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_GPIO }, /* RENA */
    cpm_pin { port: CPM_PORTC, pin: 11, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_GPIO }, /* CLSN */

    /* MII */
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

    /* I2C */
    cpm_pin { port: CPM_PORTB, pin: 26, flags: CPM_PIN_INPUT | CPM_PIN_OPENDRAIN },
    cpm_pin { port: CPM_PORTB, pin: 27, flags: CPM_PIN_INPUT | CPM_PIN_OPENDRAIN },
];

unsafe fn init_ioports() {
    let mut i = 0usize;
    while i < mpc866ads_pins.len() {
        let pin = &mpc866ads_pins[i];
        cpm1_set_pin(pin.port, pin.pin, pin.flags);
        i += 1;
    }

    cpm1_clk_setup(CPM_CLK_SMC1, CPM_BRG1, CPM_CLK_RTX);
    cpm1_clk_setup(CPM_CLK_SMC2, CPM_BRG2, CPM_CLK_RTX);
    cpm1_clk_setup(CPM_CLK_SCC1, CPM_CLK1, CPM_CLK_TX);
    cpm1_clk_setup(CPM_CLK_SCC1, CPM_CLK2, CPM_CLK_RX);

    /* Set FEC1 and FEC2 to MII mode */
    clrbits32(&mut (*mpc8xx_immr).im_cpm.cp_cptr, 0x00000180);
}

unsafe fn mpc86xads_setup_arch() {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,mpc866ads-bcsr");
    if np.is_null() {
        printk(KERN_CRIT, "Could not find fsl,mpc866ads-bcsr node\\n");
        return;
    }

    let bcsr_io = of_iomap(np, 0);
    of_node_put(np);

    if bcsr_io.is_null() {
        printk(KERN_CRIT, "Could not remap BCSR\\n");
        return;
    }

    clrbits32(bcsr_io, BCSR1_RS232EN_1 | BCSR1_RS232EN_2 | BCSR1_ETHEN);
    iounmap(bcsr_io);
}

#[repr(C)]
struct of_device_id {
    name: *const u8,
}

static of_bus_ids: [of_device_id; 4] = [
    of_device_id { name: b"soc\\0".as_ptr() },
    of_device_id { name: b"cpm\\0".as_ptr() },
    of_device_id { name: b"localbus\\0".as_ptr() },
    of_device_id { name: core::ptr::null() },
];

unsafe fn declare_of_platform_devices() -> i32 {
    of_platform_bus_probe(core::ptr::null_mut(), of_bus_ids.as_ptr(), core::ptr::null_mut());
    0
}

// machine_device_initcall(mpc86x_ads, declare_of_platform_devices);

#[repr(C)]
struct machine_desc {
    name: *const u8,
    compatible: *const u8,
    setup_arch: unsafe fn(),
    init_IRQ: unsafe fn(),
    get_irq: unsafe fn() -> i32,
    restart: unsafe fn(),
    calibrate_decr: unsafe fn(),
    set_rtc_time: unsafe fn(),
    get_rtc_time: unsafe fn(),
    progress: unsafe fn(),
}

static mpc86x_ads: machine_desc = machine_desc {
    name: b"MPC86x ADS\\0".as_ptr(),
    compatible: b"fsl,mpc866ads\\0".as_ptr(),
    setup_arch: mpc86xads_setup_arch,
    init_IRQ: mpc8xx_pic_init,
    get_irq: mpc8xx_get_irq,
    restart: mpc8xx_restart,
    calibrate_decr: mpc8xx_calibrate_decr,
    set_rtc_time: mpc8xx_set_rtc_time,
    get_rtc_time: mpc8xx_get_rtc_time,
    progress: udbg_progress,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
