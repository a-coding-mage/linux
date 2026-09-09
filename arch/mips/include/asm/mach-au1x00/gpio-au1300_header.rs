/* SPDX-License-Identifier: GPL-2.0 */
/*
 * gpio-au1300.h -- GPIO control for Au1300 GPIC and compatibles.
 *
 * Copyright (c) 2009-2011 Manuel Lauss <manuel.lauss@googlemail.com>
 */

use core::ffi::c_void;

/* Dependencies supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct gpio_chip;
#[repr(C)]
pub struct software_node;

unsafe extern "C" {
    pub static alchemy_gpic_node: software_node;
    fn KSEG1ADDR(addr: usize) -> usize;
    fn GPIC_GPIO_BANKOFF(gpio: u32) -> usize;
    fn GPIC_GPIO_TO_BIT(gpio: u32) -> usize;
    fn __raw_readl(addr: *mut c_void) -> u32;
    fn __raw_writel(value: usize, addr: *mut c_void);
    fn wmb();
    fn alchemy_get_cputype() -> i32;
}

/* with the current GPIC design, up to 128 GPIOs are possible.
 * The only implementation so far is in the Au1300, which has 75 externally
 * available GPIOs.
 */
pub const AU1300_GPIO_BASE: u32 = 0;
pub const AU1300_GPIO_NUM: u32 = 75;
pub const AU1300_GPIO_MAX: u32 = AU1300_GPIO_BASE + AU1300_GPIO_NUM - 1;

/* AU1300_GPIC_PHYS_ADDR is supplied by the surrounding translation. */
#[inline]
pub unsafe fn AU1300_GPIC_ADDR() -> *mut c_void {
    KSEG1ADDR(AU1300_GPIC_PHYS_ADDR as usize) as *mut c_void
}

#[inline]
pub unsafe fn au1300_gpio_get_value(mut gpio: u32) -> i32 {
    let mut roff = AU1300_GPIC_ADDR();
    gpio -= AU1300_GPIO_BASE;
    roff = (roff as usize + GPIC_GPIO_BANKOFF(gpio)) as *mut c_void;
    let bit = GPIC_GPIO_TO_BIT(gpio);
    (__raw_readl(roff.add(AU1300_GPIC_PINVAL)) as usize & bit) as i32
}

#[inline]
pub unsafe fn au1300_gpio_direction_input(mut gpio: u32) -> i32 {
    let mut roff = AU1300_GPIC_ADDR();
    gpio -= AU1300_GPIO_BASE;
    roff = (roff as usize + GPIC_GPIO_BANKOFF(gpio)) as *mut c_void;
    let bit = GPIC_GPIO_TO_BIT(gpio);
    __raw_writel(bit, roff.add(AU1300_GPIC_DEVCLR));
    wmb();
    0
}

#[inline]
pub unsafe fn au1300_gpio_set_value(mut gpio: u32, v: i32) -> i32 {
    let mut roff = AU1300_GPIC_ADDR();
    gpio -= AU1300_GPIO_BASE;
    roff = (roff as usize + GPIC_GPIO_BANKOFF(gpio)) as *mut c_void;
    let bit = GPIC_GPIO_TO_BIT(gpio);
    __raw_writel(bit, roff.add(if v != 0 { AU1300_GPIC_PINVAL } else { AU1300_GPIC_PINVALCLR }));
    wmb();
    0
}

#[inline]
pub unsafe fn au1300_gpio_direction_output(gpio: u32, v: i32) -> i32 {
    /* hw switches to output automatically */
    au1300_gpio_set_value(gpio, v)
}

#[inline]
pub fn au1300_gpio_to_irq(gpio: u32) -> i32 {
    AU1300_FIRST_INT + (gpio - AU1300_GPIO_BASE) as i32
}

#[inline]
pub fn au1300_irq_to_gpio(irq: i32) -> u32 {
    (irq - AU1300_FIRST_INT) as u32 + AU1300_GPIO_BASE
}

#[inline]
pub unsafe fn au1300_gpio_is_valid(gpio: u32) -> i32 {
    let ret;
    match alchemy_get_cputype() {
        ALCHEMY_CPU_AU1300 => {
            ret = (gpio >= AU1300_GPIO_BASE && gpio <= AU1300_GPIO_MAX) as i32;
        }
        _ => ret = 0,
    }
    ret
}

/* hardware remembers gpio 0-63 levels on powerup */
#[inline]
pub unsafe fn au1300_gpio_getinitlvl(mut gpio: u32) -> u32 {
    let mut roff = AU1300_GPIC_ADDR();
    let v;
    if gpio > 63 {
        return 0;
    } else if gpio > 31 {
        gpio -= 32;
        roff = roff.add(4);
    }
    v = __raw_readl(roff.add(AU1300_GPIC_RSTVAL));
    (v >> gpio) & 1
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
