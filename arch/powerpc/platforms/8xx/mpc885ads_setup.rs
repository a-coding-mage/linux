/*
 * Platform setup for the Freescale mpc885ads board
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

// Linux kernel headers and local headers from the original implementation
// provide the external symbols, constants, types, and macros referenced here.

#[repr(C)]
struct CpmPin {
    port: i32,
    pin: i32,
    flags: i32,
}

static mut BCSR: *mut u32 = core::ptr::null_mut();
static mut BCSR5: *mut u32 = core::ptr::null_mut();

static mut MPC885ADS_PINS: [CpmPin; {
    2 +
    #[cfg(not(CONFIG_MPC8XX_SECOND_ETH_FEC2))]
    { 2 } +
    7 + 13 +
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    { 16 } +
    2
}] = [
    /* SMC1 */
    CpmPin { port: CPM_PORTB, pin: 24, flags: CPM_PIN_INPUT }, /* RX */
    CpmPin { port: CPM_PORTB, pin: 25, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY }, /* TX */

    /* SMC2 */
    #[cfg(not(CONFIG_MPC8XX_SECOND_ETH_FEC2))]
    CpmPin { port: CPM_PORTE, pin: 21, flags: CPM_PIN_INPUT }, /* RX */
    #[cfg(not(CONFIG_MPC8XX_SECOND_ETH_FEC2))]
    CpmPin { port: CPM_PORTE, pin: 20, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY }, /* TX */

    /* SCC3 */
    CpmPin { port: CPM_PORTA, pin: 9, flags: CPM_PIN_INPUT }, /* RX */
    CpmPin { port: CPM_PORTA, pin: 8, flags: CPM_PIN_INPUT }, /* TX */
    CpmPin { port: CPM_PORTC, pin: 4, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_GPIO }, /* RENA */
    CpmPin { port: CPM_PORTC, pin: 5, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_GPIO }, /* CLSN */
    CpmPin { port: CPM_PORTE, pin: 27, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY }, /* TENA */
    CpmPin { port: CPM_PORTE, pin: 17, flags: CPM_PIN_INPUT }, /* CLK5 */
    CpmPin { port: CPM_PORTE, pin: 16, flags: CPM_PIN_INPUT }, /* CLK6 */

    /* MII1 */
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

    /* MII2 */
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 14, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 15, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 16, flags: CPM_PIN_OUTPUT },
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 17, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 18, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 19, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 20, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 21, flags: CPM_PIN_OUTPUT },
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 22, flags: CPM_PIN_OUTPUT },
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 23, flags: CPM_PIN_OUTPUT },
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 24, flags: CPM_PIN_OUTPUT },
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 25, flags: CPM_PIN_OUTPUT },
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 26, flags: CPM_PIN_OUTPUT },
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 27, flags: CPM_PIN_OUTPUT },
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 28, flags: CPM_PIN_OUTPUT },
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    CpmPin { port: CPM_PORTE, pin: 29, flags: CPM_PIN_OUTPUT },

    /* I2C */
    CpmPin { port: CPM_PORTB, pin: 26, flags: CPM_PIN_INPUT | CPM_PIN_OPENDRAIN },
    CpmPin { port: CPM_PORTB, pin: 27, flags: CPM_PIN_INPUT | CPM_PIN_OPENDRAIN },
];

unsafe fn init_ioports() {
    for pin in MPC885ADS_PINS.iter() {
        cpm1_set_pin(pin.port, pin.pin, pin.flags);
    }

    cpm1_clk_setup(CPM_CLK_SMC1, CPM_BRG1, CPM_CLK_RTX);
    cpm1_clk_setup(CPM_CLK_SMC2, CPM_BRG2, CPM_CLK_RTX);
    cpm1_clk_setup(CPM_CLK_SCC3, CPM_CLK5, CPM_CLK_TX);
    cpm1_clk_setup(CPM_CLK_SCC3, CPM_CLK6, CPM_CLK_RX);

    /* Set FEC1 and FEC2 to MII mode */
    clrbits32(&mut (*mpc8xx_immr).im_cpm.cp_cptr as *mut _, 0x00000180);
}

unsafe fn mpc885ads_setup_arch() {
    let mut np: *mut device_node;

    cpm_reset();
    init_ioports();

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), c"fsl,mpc885ads-bcsr".as_ptr());
    if np.is_null() {
        printk(KERN_CRIT, c"Could not find fsl,mpc885ads-bcsr node\n".as_ptr());
        return;
    }

    BCSR = of_iomap(np, 0);
    BCSR5 = of_iomap(np, 1);
    of_node_put(np);

    if BCSR.is_null() || BCSR5.is_null() {
        printk(KERN_CRIT, c"Could not remap BCSR\n".as_ptr());
        return;
    }

    clrbits32(BCSR.add(1), BCSR1_RS232EN_1);
    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    setbits32(BCSR.add(1), BCSR1_RS232EN_2);
    #[cfg(not(CONFIG_MPC8XX_SECOND_ETH_FEC2))]
    clrbits32(BCSR.add(1), BCSR1_RS232EN_2);

    clrbits32(BCSR5, BCSR5_MII1_EN);
    setbits32(BCSR5, BCSR5_MII1_RST);
    udelay(1000);
    clrbits32(BCSR5, BCSR5_MII1_RST);

    #[cfg(CONFIG_MPC8XX_SECOND_ETH_FEC2)]
    {
        clrbits32(BCSR5, BCSR5_MII2_EN);
        setbits32(BCSR5, BCSR5_MII2_RST);
        udelay(1000);
        clrbits32(BCSR5, BCSR5_MII2_RST);
    }
    #[cfg(not(CONFIG_MPC8XX_SECOND_ETH_FEC2))]
    setbits32(BCSR5, BCSR5_MII2_EN);

    #[cfg(CONFIG_MPC8XX_SECOND_ETH_SCC3)]
    {
        clrbits32(BCSR.add(4), BCSR4_ETH10_RST);
        udelay(1000);
        setbits32(BCSR.add(4), BCSR4_ETH10_RST);
        setbits32(BCSR.add(1), BCSR1_ETHEN);
        np = of_find_node_by_path(c"/soc@ff000000/cpm@9c0/serial@a80".as_ptr());
    }
    #[cfg(not(CONFIG_MPC8XX_SECOND_ETH_SCC3))]
    {
        np = of_find_node_by_path(c"/soc@ff000000/cpm@9c0/ethernet@a40".as_ptr());
    }

    /* The SCC3 enet registers overlap the SMC1 registers, so
     * one of the two must be removed from the device tree.
     */
    if !np.is_null() {
        of_detach_node(np);
        of_node_put(np);
    }
}

#[repr(C)]
struct OfDeviceId {
    name: *const i8,
}

static OF_BUS_IDS: [OfDeviceId; 4] = [
    OfDeviceId { name: c"soc".as_ptr() },
    OfDeviceId { name: c"cpm".as_ptr() },
    OfDeviceId { name: c"localbus".as_ptr() },
    OfDeviceId { name: core::ptr::null() },
];

unsafe fn declare_of_platform_devices() -> i32 {
    /* Publish the QE devices */
    of_platform_bus_probe(core::ptr::null_mut(), OF_BUS_IDS.as_ptr(), core::ptr::null_mut());
    0
}

// machine_device_initcall(mpc885_ads, declare_of_platform_devices);
// define_machine(mpc885_ads) {
//     .name = "Freescale MPC885 ADS",
//     .compatible = "fsl,mpc885ads",
//     .setup_arch = mpc885ads_setup_arch,
//     .init_IRQ = mpc8xx_pic_init,
//     .get_irq = mpc8xx_get_irq,
//     .restart = mpc8xx_restart,
//     .calibrate_decr = mpc8xx_calibrate_decr,
//     .progress = udbg_progress,
// };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
