// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/
/*
 * m5249.c -- platform support for ColdFire 5249 based boards
 *
 * Copyright (C) 2002, Greg Ungerer (gerg@snapgear.com)
 */
/***************************************************************************/

// Dependencies supplied by the surrounding kernel translation.

#[allow(non_upper_case_globals)]
static mut clk_pll: clk = clk { _private: 0 };
#[allow(non_upper_case_globals)]
static mut clk_sys: clk = clk { _private: 0 };

#[no_mangle]
pub static mut m5249_clk_lookup: [clk_lookup; 9] = [
    CLKDEV_INIT(core::ptr::null(), b"pll.0\0".as_ptr() as *const i8, unsafe { &mut clk_pll }),
    CLKDEV_INIT(core::ptr::null(), b"sys.0\0".as_ptr() as *const i8, unsafe { &mut clk_sys }),
    CLKDEV_INIT(b"mcftmr.0\0".as_ptr() as *const i8, core::ptr::null(), unsafe { &mut clk_sys }),
    CLKDEV_INIT(b"mcftmr.1\0".as_ptr() as *const i8, core::ptr::null(), unsafe { &mut clk_sys }),
    CLKDEV_INIT(b"mcfuart.0\0".as_ptr() as *const i8, core::ptr::null(), unsafe { &mut clk_sys }),
    CLKDEV_INIT(b"mcfuart.1\0".as_ptr() as *const i8, core::ptr::null(), unsafe { &mut clk_sys }),
    CLKDEV_INIT(b"mcfqspi.0\0".as_ptr() as *const i8, core::ptr::null(), unsafe { &mut clk_sys }),
    CLKDEV_INIT(b"imx1-i2c.0\0".as_ptr() as *const i8, core::ptr::null(), unsafe { &mut clk_sys }),
    CLKDEV_INIT(b"imx1-i2c.1\0".as_ptr() as *const i8, core::ptr::null(), unsafe { &mut clk_sys }),
];

#[cfg(CONFIG_M5249C3)]
static mut m5249_smc91x_resources: [resource; 2] = [
    resource { start: 0xe0000300, end: 0xe0000300 + 0x100, flags: IORESOURCE_MEM },
    resource { start: MCF_IRQ_GPIO6, end: MCF_IRQ_GPIO6, flags: IORESOURCE_IRQ },
];

#[cfg(CONFIG_M5249C3)]
static mut m5249_smc91x: platform_device = platform_device {
    name: b"smc91x\0".as_ptr() as *const i8,
    id: 0,
    num_resources: 2,
    resource: unsafe { m5249_smc91x_resources.as_mut_ptr() },
};

#[cfg(CONFIG_M5249C3)]
static mut m5249_devices: [*mut platform_device; 1] = [unsafe { &mut m5249_smc91x }];
#[cfg(not(CONFIG_M5249C3))]
static mut m5249_devices: [*mut platform_device; 0] = [];

unsafe fn m5249_qspi_init() {
    // QSPI irq setup
    #[cfg(IS_ENABLED_CONFIG_SPI_COLDFIRE_QSPI)]
    {
        mcf_write8(MCFSIM_ICR_AUTOVEC | MCFSIM_ICR_LEVEL4 | MCFSIM_ICR_PRI0, MCFSIM_QSPIICR);
        mcf_mapirq2imr(MCF_IRQ_QSPI, MCFINTC_QSPI);
    }
}

unsafe fn m5249_i2c_init() {
    #[cfg(IS_ENABLED_CONFIG_I2C_IMX)]
    {
        let mut r: u32;
        // first I2C controller uses regular irq setup
        mcf_write8(MCFSIM_ICR_AUTOVEC | MCFSIM_ICR_LEVEL5 | MCFSIM_ICR_PRI0, MCFSIM_I2CICR);
        mcf_mapirq2imr(MCF_IRQ_I2C0, MCFINTC_I2C);
        // second I2C controller is completely different
        r = mcf_read32(MCFINTC2_INTPRI_REG(MCF_IRQ_I2C1));
        r &= !MCFINTC2_INTPRI_BITS(0xf, MCF_IRQ_I2C1);
        r |= MCFINTC2_INTPRI_BITS(0x5, MCF_IRQ_I2C1);
        mcf_write32(r, MCFINTC2_INTPRI_REG(MCF_IRQ_I2C1));
    }
}

#[cfg(CONFIG_M5249C3)]
unsafe fn m5249_smc91x_init() {
    let mut gpio: u32;
    // Set the GPIO line as interrupt source for smc91x device
    gpio = mcf_read32(MCFSIM2_GPIOINTENABLE);
    mcf_write32(gpio | 0x40, MCFSIM2_GPIOINTENABLE);
    gpio = mcf_read32(MCFINTC2_INTPRI5);
    mcf_write32(gpio | 0x04000000, MCFINTC2_INTPRI5);
}

#[no_mangle]
pub unsafe extern "C" fn config_BSP(_commandp: *mut i8, _size: i32) {
    mach_sched_init = hw_timer_init;
    #[cfg(CONFIG_M5249C3)]
    m5249_smc91x_init();
    m5249_qspi_init();
    m5249_i2c_init();
    clkdev_add_table(m5249_clk_lookup.as_mut_ptr(), 9);
}

unsafe fn init_BSP() -> i32 {
    platform_add_devices(m5249_devices.as_mut_ptr(), m5249_devices.len());
    0
}

arch_initcall!(init_BSP);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
