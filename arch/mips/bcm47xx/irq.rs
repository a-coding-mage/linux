/*
 *  Copyright (C) 2004 Florian Schirmer <jolt@tuxbox.org>
 *
 *  This program is free software; you can redistribute  it and/or modify it
 *  under  the terms of  the GNU General  Public License as published by the
 *  Free Software Foundation; either version 2 of the License, or (at your
 *  option) any later version.
 *
 *  This software is provided "as is", without warranty of any kind.
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn read_c0_cause() -> u32;
    fn read_c0_status() -> u32;
    fn clear_c0_status(value: u32);
    fn do_IRQ(irq: i32);
    fn bcm47xx_bus_setup();
    fn mips_cpu_irq_init();
    fn set_vi_handler(irq: i32, handler: unsafe extern "C" fn());
    fn pr_info(message: *const u8);
    static mut cp0_compare_irq: i32;
    static cpu_has_vint: bool;
}

const CAUSEF_IP: u32 = 0x0000_ff00;
const CAUSEF_IP2: u32 = 1 << 10;
const CAUSEF_IP3: u32 = 1 << 11;
const CAUSEF_IP4: u32 = 1 << 12;
const CAUSEF_IP5: u32 = 1 << 13;
const CAUSEF_IP6: u32 = 1 << 14;
const CAUSEF_IP7: u32 = 1 << 15;

pub unsafe extern "C" fn plat_irq_dispatch() {
    let cause: u32 = read_c0_cause() & read_c0_status() & CAUSEF_IP;

    clear_c0_status(cause);

    if cause & CAUSEF_IP7 != 0 {
        do_IRQ(7);
    }
    if cause & CAUSEF_IP2 != 0 {
        do_IRQ(2);
    }
    if cause & CAUSEF_IP3 != 0 {
        do_IRQ(3);
    }
    if cause & CAUSEF_IP4 != 0 {
        do_IRQ(4);
    }
    if cause & CAUSEF_IP5 != 0 {
        do_IRQ(5);
    }
    if cause & CAUSEF_IP6 != 0 {
        do_IRQ(6);
    }
}

macro_rules! define_hwx_irqdispatch {
    ($x:literal, $name:ident) => {
        unsafe extern "C" fn $name() {
            do_IRQ($x);
        }
    };
}

define_hwx_irqdispatch!(2, bcm47xx_hw2_irqdispatch);
define_hwx_irqdispatch!(3, bcm47xx_hw3_irqdispatch);
define_hwx_irqdispatch!(4, bcm47xx_hw4_irqdispatch);
define_hwx_irqdispatch!(5, bcm47xx_hw5_irqdispatch);
define_hwx_irqdispatch!(6, bcm47xx_hw6_irqdispatch);
define_hwx_irqdispatch!(7, bcm47xx_hw7_irqdispatch);

pub unsafe extern "C" fn arch_init_irq() {
    // This is the first arch callback after mm_init (we can use kmalloc),
    // so let's finish bus initialization now.
    bcm47xx_bus_setup();

    // CONFIG_BCM47XX_BCMA is a build-time condition from the original source.
    // When enabled, the original code writes the BCMA MIPS74K interrupt mask
    // for core 5, then routes the timer interrupt to IRQ 7.
    #[cfg(feature = "CONFIG_BCM47XX_BCMA")]
    {
        // External BCMA bus layout and register helpers are supplied elsewhere:
        // bcma_write32(bcm47xx_bus.bcma.bus.drv_mips.core,
        //              BCMA_MIPS_MIPS74K_INTMASK(5), 1 << 31);
        cp0_compare_irq = 7;
    }

    mips_cpu_irq_init();

    if cpu_has_vint {
        pr_info(b"Setting up vectored interrupts\n".as_ptr());
        set_vi_handler(2, bcm47xx_hw2_irqdispatch);
        set_vi_handler(3, bcm47xx_hw3_irqdispatch);
        set_vi_handler(4, bcm47xx_hw4_irqdispatch);
        set_vi_handler(5, bcm47xx_hw5_irqdispatch);
        set_vi_handler(6, bcm47xx_hw6_irqdispatch);
        set_vi_handler(7, bcm47xx_hw7_irqdispatch);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
