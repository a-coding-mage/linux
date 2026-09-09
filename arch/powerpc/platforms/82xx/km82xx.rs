// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Keymile km82xx support
 * Copyright 2008-2011 DENX Software Engineering GmbH
 * Author: Heiko Schocher <hs@denx.de>
 *
 * based on code from:
 * Copyright 2007 Freescale Semiconductor, Inc.
 * Author: Scott Wood <scottwood@freescale.com>
 */

// Linux and platform dependencies supplied by the surrounding repository.

unsafe fn km82xx_pic_init() {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), "fsl,pq2-pic");

    if np.is_null() {
        pr_err!("PIC init: can not find cpm-pic node\n");
        return;
    }

    cpm2_pic_init(np);
}

#[repr(C)]
struct cpm_pin {
    port: i32,
    pin: i32,
    flags: i32,
}

static mut KM82XX_PINS: &[cpm_pin] = &[
    /* SMC1 */
    cpm_pin { port: 2, pin: 4, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 5, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },

    /* SMC2 */
    cpm_pin { port: 0, pin: 8, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 9, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },

    /* SCC1 */
    cpm_pin { port: 2, pin: 21, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 15, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 3, pin: 31, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 3, pin: 30, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },

    /* SCC4 */
    cpm_pin { port: 2, pin: 25, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 24, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 9, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 8, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 3, pin: 22, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 3, pin: 21, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },

    /* FCC1 */
    cpm_pin { port: 0, pin: 14, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 15, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 16, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 17, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 18, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 19, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 20, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 21, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 26, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 0, pin: 27, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 0, pin: 28, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 0, pin: 29, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 0, pin: 30, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 0, pin: 31, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 2, pin: 22, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 23, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },

    /* FCC2 */
    cpm_pin { port: 1, pin: 18, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 19, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 20, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 21, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 22, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 23, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 24, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 25, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 26, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 27, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 28, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 29, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 1, pin: 30, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 31, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 18, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 19, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },

    /* MDC */
    cpm_pin { port: 0, pin: 13, flags: CPM_PIN_OUTPUT | CPM_PIN_GPIO },

    // CONFIG_I2C_CPM conditional entries preserved below.
    #[cfg(CONFIG_I2C_CPM)]
    cpm_pin { port: 3, pin: 14, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_OPENDRAIN },
    #[cfg(CONFIG_I2C_CPM)]
    cpm_pin { port: 3, pin: 15, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_OPENDRAIN },

    /* USB */
    cpm_pin { port: 0, pin: 10, flags: CPM_PIN_OUTPUT | CPM_PIN_GPIO }, // FULL_SPEED
    cpm_pin { port: 0, pin: 11, flags: CPM_PIN_OUTPUT | CPM_PIN_GPIO }, // /SLAVE
    cpm_pin { port: 2, pin: 10, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY }, // RXN
    cpm_pin { port: 2, pin: 11, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY }, // RXP
    cpm_pin { port: 2, pin: 20, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY }, // /OE
    cpm_pin { port: 2, pin: 27, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY }, // RXCLK
    cpm_pin { port: 3, pin: 23, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY }, // TXP
    cpm_pin { port: 3, pin: 24, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY }, // TXN
    cpm_pin { port: 3, pin: 25, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY }, // RXD

    /* SPI */
    cpm_pin { port: 3, pin: 16, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY }, // SPI_MISO PD16
    cpm_pin { port: 3, pin: 17, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY }, // SPI_MOSI PD17
    cpm_pin { port: 3, pin: 18, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY }, // SPI_CLK PD18
];

unsafe fn init_ioports() {
    let mut i = 0usize;
    while i < KM82XX_PINS.len() {
        let pin = &KM82XX_PINS[i];
        cpm2_set_pin(pin.port, pin.pin, pin.flags);
        i += 1;
    }

    cpm2_smc_clk_setup(CPM_CLK_SMC2, CPM_BRG8);
    cpm2_smc_clk_setup(CPM_CLK_SMC1, CPM_BRG7);
    cpm2_clk_setup(CPM_CLK_SCC1, CPM_CLK11, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_SCC1, CPM_CLK11, CPM_CLK_TX);
    cpm2_clk_setup(CPM_CLK_SCC3, CPM_CLK5, CPM_CLK_RTX);
    cpm2_clk_setup(CPM_CLK_SCC4, CPM_CLK7, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_SCC4, CPM_CLK8, CPM_CLK_TX);
    cpm2_clk_setup(CPM_CLK_FCC1, CPM_CLK10, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_FCC1, CPM_CLK9, CPM_CLK_TX);
    cpm2_clk_setup(CPM_CLK_FCC2, CPM_CLK13, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_FCC2, CPM_CLK14, CPM_CLK_TX);

    /* Force USB FULL SPEED bit to '1' */
    setbits32(&mut (*cpm2_immr).im_ioport.iop_pdata, 1 << (31 - 10));
    /* clear USB_SLAVE */
    clrbits32(&mut (*cpm2_immr).im_ioport.iop_pdata, 1 << (31 - 11));
}

unsafe fn km82xx_setup_arch() {
    if let Some(progress) = ppc_md.progress {
        progress("km82xx_setup_arch()", 0);
    }

    cpm2_reset();

    /* When this is set, snooping CPM DMA from RAM causes
     * machine checks.  See erratum SIU18.
     */
    clrbits32(&mut (*cpm2_immr).im_siu_conf.siu_82xx.sc_bcr, MPC82XX_BCR_PLDP);

    init_ioports();

    if let Some(progress) = ppc_md.progress {
        progress("km82xx_setup_arch(), finish", 0);
    }
}

static OF_BUS_IDS: &[of_device_id] = &[
    of_device_id { compatible: "simple-bus" },
    of_device_id::default(),
];

unsafe fn declare_of_platform_devices() -> i32 {
    of_platform_bus_probe(core::ptr::null_mut(), OF_BUS_IDS, core::ptr::null_mut());
    0
}

// machine_device_initcall(km82xx, declare_of_platform_devices);

static KM82XX_MACHINE: machine_desc = machine_desc {
    name: "Keymile km82xx",
    compatible: "keymile,km82xx",
    setup_arch: Some(km82xx_setup_arch),
    init_IRQ: Some(km82xx_pic_init),
    get_irq: Some(cpm2_get_irq),
    restart: Some(pq2_restart),
    progress: Some(udbg_progress),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
