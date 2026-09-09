/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2002 Integrated Device Technology, Inc.
 *	All rights reserved.
 *
 * GPIO register definition.
 *
 * Author : ryan.holmQVist@idt.com
 * Date	  : 20011005
 * Copyright (C) 2001, 2002 Ryan Holm <ryan.holmQVist@idt.com>
 * Copyright (C) 2008 Florian Fainelli <florian@openwrt.org>
 */

#[repr(C)]
pub struct rb532_gpio_reg {
	pub gpiofunc: u32,   /* GPIO Function Register
			       * gpiofunc[x]==0 bit = gpio
			       * func[x]==1	 bit = altfunc
			       */
	pub gpiocfg: u32,    /* GPIO Configuration Register
			       * gpiocfg[x]==0 bit = input
			       * gpiocfg[x]==1 bit = output
			       */
	pub gpiod: u32,      /* GPIO Data Register
			       * gpiod[x] read/write gpio pinX status
			       */
	pub gpioilevel: u32, /* GPIO Interrupt Status Register
			       * interrupt level (see gpioistat)
			       */
	pub gpioistat: u32,  /* Gpio Interrupt Status Register
			       * istat[x] = (gpiod[x] == level[x])
			       * cleared in ISR (STICKY bits)
			       */
	pub gpionmien: u32,  /* GPIO Non-maskable Interrupt Enable Register */
}

/* UART GPIO signals */
pub const RC32434_UART0_SOUT: u32 = 1 << 0;
pub const RC32434_UART0_SIN: u32 = 1 << 1;
pub const RC32434_UART0_RTS: u32 = 1 << 2;
pub const RC32434_UART0_CTS: u32 = 1 << 3;

/* M & P bus GPIO signals */
pub const RC32434_MP_BIT_22: u32 = 1 << 4;
pub const RC32434_MP_BIT_23: u32 = 1 << 5;
pub const RC32434_MP_BIT_24: u32 = 1 << 6;
pub const RC32434_MP_BIT_25: u32 = 1 << 7;

/* CPU GPIO signals */
pub const RC32434_CPU_GPIO: u32 = 1 << 8;

/* Reserved GPIO signals */
pub const RC32434_AF_SPARE_6: u32 = 1 << 9;
pub const RC32434_AF_SPARE_4: u32 = 1 << 10;
pub const RC32434_AF_SPARE_3: u32 = 1 << 11;
pub const RC32434_AF_SPARE_2: u32 = 1 << 12;

/* PCI messaging unit */
pub const RC32434_PCI_MSU_GPIO: u32 = 1 << 13;

/* NAND GPIO signals */
pub const GPIO_RDY: u32 = 8;
pub const GPIO_WPX: u32 = 9;
pub const GPIO_ALE: u32 = 10;
pub const GPIO_CLE: u32 = 11;

/* Compact Flash GPIO pin */
pub const CF_GPIO_NUM: u32 = 13;

/* S1 button GPIO (shared with UART0_SIN) */
pub const GPIO_BTN_S1: u32 = 1;

extern "C" {
	pub fn rb532_gpio_set_ilevel(bit: i32, gpio: u32);
	pub fn rb532_gpio_set_istat(bit: i32, gpio: u32);
	pub fn rb532_gpio_set_func(gpio: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
