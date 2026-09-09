/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2008 Simtec Electronics
 *	http://armlinux.simtec.co.uk/
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * S3C Platform - GPIO core
 */

// Dependencies supplied by the surrounding kernel translation.

pub const GPIOCON_OFF: u32 = 0x00;
pub const GPIODAT_OFF: u32 = 0x04;

#[inline]
pub const fn con_4bit_shift(off: u32) -> u32 {
    off * 4
}

pub struct samsung_gpio_chip;

pub struct samsung_gpio_pm {
    pub save: Option<unsafe extern "C" fn(chip: *mut samsung_gpio_chip)>,
    pub resume: Option<unsafe extern "C" fn(chip: *mut samsung_gpio_chip)>,
}

pub struct samsung_gpio_cfg;

#[repr(C)]
pub struct samsung_gpio_chip {
    pub chip: gpio_chip,
    pub config: *mut samsung_gpio_cfg,
    pub pm: *mut samsung_gpio_pm,
    pub base: *mut core::ffi::c_void,
    pub irq_base: i32,
    pub group: i32,
    pub lock: spinlock_t,
    // CONFIG_PM controls whether this field is present in the C build.
    #[cfg(feature = "CONFIG_PM")]
    pub pm_save: [u32; 4],
    pub bitmap_gpio_int: u32,
}

#[inline]
pub unsafe fn to_samsung_gpio(gpc: *mut gpio_chip) -> *mut samsung_gpio_chip {
    // Equivalent to container_of(gpc, struct samsung_gpio_chip, chip).
    (gpc as *mut u8).sub(core::mem::offset_of!(samsung_gpio_chip, chip))
        as *mut samsung_gpio_chip
}

extern "C" {
    pub fn samsung_gpiolib_to_irq(chip: *mut gpio_chip, offset: u32) -> i32;
}

// CONFIG_S3C_GPIO_TRACK selects the machine-specific GPIO tracking variant.
#[cfg(feature = "CONFIG_S3C_GPIO_TRACK")]
extern "C" {
    pub static mut s3c_gpios: *mut samsung_gpio_chip;
}

#[cfg(feature = "CONFIG_S3C_GPIO_TRACK")]
#[inline]
pub unsafe fn samsung_gpiolib_getchip(chip: u32) -> *mut samsung_gpio_chip {
    if chip < S3C_GPIO_END {
        s3c_gpios.add(chip as usize).read()
    } else {
        core::ptr::null_mut()
    }
}

#[cfg(not(feature = "CONFIG_S3C_GPIO_TRACK"))]
extern "C" {
    pub static mut s3c24xx_gpios: samsung_gpio_chip;
}

#[cfg(not(feature = "CONFIG_S3C_GPIO_TRACK"))]
#[inline]
pub unsafe fn samsung_gpiolib_getchip(pin: u32) -> *mut samsung_gpio_chip {
    if pin > S3C_GPIO_END {
        return core::ptr::null_mut();
    }
    let chip = (&raw mut s3c24xx_gpios).add((pin / 32) as usize);
    ((*chip).chip.base <= (pin as i32)
        && ((pin as i32) - (*chip).chip.base) < (*chip).chip.ngpio)
        .then_some(chip)
        .unwrap_or(core::ptr::null_mut())
}

#[cfg(not(feature = "CONFIG_S3C_GPIO_TRACK"))]
#[inline]
pub unsafe fn s3c_gpiolib_track(_chip: *mut samsung_gpio_chip) {}

// CONFIG_PM selects whether these power-management objects are external data
// or null pointers, matching the C preprocessor definitions.
#[cfg(feature = "CONFIG_PM")]
extern "C" {
    pub static mut samsung_gpio_pm_1bit: samsung_gpio_pm;
    pub static mut samsung_gpio_pm_2bit: samsung_gpio_pm;
    pub static mut samsung_gpio_pm_4bit: samsung_gpio_pm;
}

#[cfg(feature = "CONFIG_PM")]
#[inline]
pub const fn __gpio_pm(x: *mut samsung_gpio_pm) -> *mut samsung_gpio_pm { x }

#[cfg(not(feature = "CONFIG_PM"))]
pub const samsung_gpio_pm_1bit: *mut samsung_gpio_pm = core::ptr::null_mut();
#[cfg(not(feature = "CONFIG_PM"))]
pub const samsung_gpio_pm_2bit: *mut samsung_gpio_pm = core::ptr::null_mut();
#[cfg(not(feature = "CONFIG_PM"))]
pub const samsung_gpio_pm_4bit: *mut samsung_gpio_pm = core::ptr::null_mut();
#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub const fn __gpio_pm(_x: *mut samsung_gpio_pm) -> *mut samsung_gpio_pm {
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn samsung_gpio_lock(oc: *mut samsung_gpio_chip, fl: *mut irq_flags_t) {
    spin_lock_irqsave(&mut (*oc).lock, fl);
}

#[inline]
pub unsafe fn samsung_gpio_unlock(oc: *mut samsung_gpio_chip, fl: irq_flags_t) {
    spin_unlock_irqrestore(&mut (*oc).lock, fl);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
