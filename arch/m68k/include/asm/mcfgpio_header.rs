/* SPDX-License-Identifier: GPL-2.0-only */
/* Coldfire generic GPIO support. */

extern "C" {
    pub fn __mcfgpio_get_value(gpio: u32) -> i32;
    pub fn __mcfgpio_set_value(gpio: u32, value: i32);
    pub fn __mcfgpio_direction_input(gpio: u32) -> i32;
    pub fn __mcfgpio_direction_output(gpio: u32, value: i32) -> i32;
    pub fn __mcfgpio_request(gpio: u32) -> i32;
    pub fn __mcfgpio_free(gpio: u32);
}

/* CONFIG_GPIOLIB selects the kernel legacy GPIO implementation. */
#[cfg(not(feature = "CONFIG_GPIOLIB"))]
pub unsafe fn __gpio_get_value(gpio: u32) -> i32 {
    if gpio < MCFGPIO_PIN_MAX { __mcfgpio_get_value(gpio) } else { -EINVAL }
}

#[cfg(not(feature = "CONFIG_GPIOLIB"))]
pub unsafe fn __gpio_set_value(gpio: u32, value: i32) {
    if gpio < MCFGPIO_PIN_MAX { __mcfgpio_set_value(gpio, value); }
}

#[cfg(not(feature = "CONFIG_GPIOLIB"))]
pub unsafe fn __gpio_to_irq(_gpio: u32) -> i32 { -EINVAL }

#[cfg(not(feature = "CONFIG_GPIOLIB"))]
pub unsafe fn gpio_direction_input(gpio: u32) -> i32 {
    if gpio < MCFGPIO_PIN_MAX { __mcfgpio_direction_input(gpio) } else { -EINVAL }
}

#[cfg(not(feature = "CONFIG_GPIOLIB"))]
pub unsafe fn gpio_direction_output(gpio: u32, value: i32) -> i32 {
    if gpio < MCFGPIO_PIN_MAX { __mcfgpio_direction_output(gpio, value) } else { -EINVAL }
}

#[cfg(not(feature = "CONFIG_GPIOLIB"))]
pub unsafe fn gpio_request(gpio: u32, _label: *const core::ffi::c_char) -> i32 {
    if gpio < MCFGPIO_PIN_MAX { __mcfgpio_request(gpio) } else { -EINVAL }
}

#[cfg(not(feature = "CONFIG_GPIOLIB"))]
pub unsafe fn gpio_free(gpio: u32) {
    if gpio < MCFGPIO_PIN_MAX { __mcfgpio_free(gpio); }
}

/* Port organization follows the selected ColdFire family. */
#[cfg(any(feature="CONFIG_M5206", feature="CONFIG_M5206e", feature="CONFIG_M520x", feature="CONFIG_M523x", feature="CONFIG_M527x", feature="CONFIG_M528x", feature="CONFIG_M53xx", feature="CONFIG_M54xx", feature="CONFIG_M5441x"))]
pub type MCFGPIO_PORTTYPE = u8;
#[cfg(any(feature="CONFIG_M5206", feature="CONFIG_M5206e", feature="CONFIG_M520x", feature="CONFIG_M523x", feature="CONFIG_M527x", feature="CONFIG_M528x", feature="CONFIG_M53xx", feature="CONFIG_M54xx", feature="CONFIG_M5441x"))]
pub const MCFGPIO_PORTSIZE: u32 = 8;
#[cfg(any(feature="CONFIG_M5307", feature="CONFIG_M5407", feature="CONFIG_M5272"))]
pub type MCFGPIO_PORTTYPE = u16;
#[cfg(any(feature="CONFIG_M5307", feature="CONFIG_M5407", feature="CONFIG_M5272"))]
pub const MCFGPIO_PORTSIZE: u32 = 16;
#[cfg(any(feature="CONFIG_M5249", feature="CONFIG_M525x"))]
pub type MCFGPIO_PORTTYPE = u32;
#[cfg(any(feature="CONFIG_M5249", feature="CONFIG_M525x"))]
pub const MCFGPIO_PORTSIZE: u32 = 32;

#[inline] pub const fn mcfgpio_bit(gpio: u32) -> u32 { 1u32 << (gpio % MCFGPIO_PORTSIZE) }
#[inline] pub const fn mcfgpio_port(gpio: u32) -> u32 { gpio / MCFGPIO_PORTSIZE }

#[cfg(feature="CONFIG_M528x")] pub const MCFGPIO_SCR_START: u32 = 40;
#[cfg(all(not(feature="CONFIG_M528x"), feature="CONFIG_M5441x"))] pub const MCFGPIO_SCR_START: u32 = 0;
#[cfg(all(not(feature="CONFIG_M528x"), not(feature="CONFIG_M5441x"), any(feature="CONFIG_M520x", feature="CONFIG_M523x", feature="CONFIG_M527x", feature="CONFIG_M53xx", feature="CONFIG_M54xx")))] pub const MCFGPIO_SCR_START: u32 = 8;
#[cfg(not(any(feature="CONFIG_M520x", feature="CONFIG_M523x", feature="CONFIG_M527x", feature="CONFIG_M528x", feature="CONFIG_M53xx", feature="CONFIG_M54xx", feature="CONFIG_M5441x")))] pub const MCFGPIO_SCR_START: u32 = MCFGPIO_PIN_MAX;

#[cfg(any(feature="CONFIG_M520x", feature="CONFIG_M523x", feature="CONFIG_M527x", feature="CONFIG_M528x", feature="CONFIG_M53xx", feature="CONFIG_M54xx", feature="CONFIG_M5441x"))]
#[inline] pub const fn MCFGPIO_SETR_PORT(gpio: u32) -> u32 { MCFGPIO_SETR + mcfgpio_port(gpio - MCFGPIO_SCR_START) }
#[cfg(any(feature="CONFIG_M520x", feature="CONFIG_M523x", feature="CONFIG_M527x", feature="CONFIG_M528x", feature="CONFIG_M53xx", feature="CONFIG_M54xx", feature="CONFIG_M5441x"))]
#[inline] pub const fn MCFGPIO_CLRR_PORT(gpio: u32) -> u32 { MCFGPIO_CLRR + mcfgpio_port(gpio - MCFGPIO_SCR_START) }
#[cfg(not(any(feature="CONFIG_M520x", feature="CONFIG_M523x", feature="CONFIG_M527x", feature="CONFIG_M528x", feature="CONFIG_M53xx", feature="CONFIG_M54xx", feature="CONFIG_M5441x")))]
#[inline] pub const fn MCFGPIO_SETR_PORT(_gpio: u32) -> u32 { 0 }
#[cfg(not(any(feature="CONFIG_M520x", feature="CONFIG_M523x", feature="CONFIG_M527x", feature="CONFIG_M528x", feature="CONFIG_M53xx", feature="CONFIG_M54xx", feature="CONFIG_M5441x")))]
#[inline] pub const fn MCFGPIO_CLRR_PORT(_gpio: u32) -> u32 { 0 }

/* Register-selection helpers. The register constants are supplied externally. */
#[inline]
pub unsafe fn __mcfgpio_ppdr(gpio: u32) -> u32 {
    #[cfg(any(feature="CONFIG_M5206", feature="CONFIG_M5206e", feature="CONFIG_M5307", feature="CONFIG_M5407"))] { return MCFSIM_PADAT; }
    #[cfg(feature="CONFIG_M5272")] { return if gpio < 16 { MCFSIM_PADAT } else if gpio < 32 { MCFSIM_PBDAT } else { MCFSIM_PCDAT }; }
    #[cfg(any(feature="CONFIG_M5249", feature="CONFIG_M525x"))] { return if gpio < 32 { MCFSIM2_GPIOREAD } else { MCFSIM2_GPIO1READ }; }
    #[cfg(feature="CONFIG_M528x")] { if gpio < 8 { return MCFEPORT_EPPDR; } else if gpio < 16 { return MCFGPTA_GPTPORT; } else if gpio < 24 { return MCFGPTB_GPTPORT; } else if gpio < 32 { return MCFQADC_PORTQA; } else if gpio < 40 { return MCFQADC_PORTQB; } }
    #[cfg(any(feature="CONFIG_M520x", feature="CONFIG_M523x", feature="CONFIG_M527x", feature="CONFIG_M528x", feature="CONFIG_M53xx", feature="CONFIG_M54xx", feature="CONFIG_M5441x"))] { return MCFGPIO_PPDR + mcfgpio_port(gpio - MCFGPIO_SCR_START); }
    0
}

#[inline]
pub unsafe fn __mcfgpio_podr(gpio: u32) -> u32 {
    #[cfg(any(feature="CONFIG_M5206", feature="CONFIG_M5206e", feature="CONFIG_M5307", feature="CONFIG_M5407"))] { return MCFSIM_PADAT; }
    #[cfg(feature="CONFIG_M5272")] { return if gpio < 16 { MCFSIM_PADAT } else if gpio < 32 { MCFSIM_PBDAT } else { MCFSIM_PCDAT }; }
    #[cfg(any(feature="CONFIG_M5249", feature="CONFIG_M525x"))] { return if gpio < 32 { MCFSIM2_GPIOWRITE } else { MCFSIM2_GPIO1WRITE }; }
    #[cfg(feature="CONFIG_M528x")] { if gpio < 8 { return MCFEPORT_EPDR; } else if gpio < 16 { return MCFGPTA_GPTPORT; } else if gpio < 24 { return MCFGPTB_GPTPORT; } else if gpio < 32 { return MCFQADC_PORTQA; } else if gpio < 40 { return MCFQADC_PORTQB; } }
    #[cfg(any(feature="CONFIG_M520x", feature="CONFIG_M523x", feature="CONFIG_M527x", feature="CONFIG_M528x", feature="CONFIG_M53xx", feature="CONFIG_M54xx", feature="CONFIG_M5441x"))] { return MCFGPIO_PODR + mcfgpio_port(gpio - MCFGPIO_SCR_START); }
    0
}

#[inline]
pub unsafe fn __mcfgpio_pddr(gpio: u32) -> u32 {
    #[cfg(any(feature="CONFIG_M5206", feature="CONFIG_M5206e", feature="CONFIG_M5307", feature="CONFIG_M5407"))] { return MCFSIM_PADDR; }
    #[cfg(feature="CONFIG_M5272")] { return if gpio < 16 { MCFSIM_PADDR } else if gpio < 32 { MCFSIM_PBDDR } else { MCFSIM_PCDDR }; }
    #[cfg(any(feature="CONFIG_M5249", feature="CONFIG_M525x"))] { return if gpio < 32 { MCFSIM2_GPIOENABLE } else { MCFSIM2_GPIO1ENABLE }; }
    #[cfg(feature="CONFIG_M528x")] { if gpio < 8 { return MCFEPORT_EPDDR; } else if gpio < 16 { return MCFGPTA_GPTDDR; } else if gpio < 24 { return MCFGPTB_GPTDDR; } else if gpio < 32 { return MCFQADC_DDRQA; } else if gpio < 40 { return MCFQADC_DDRQB; } }
    #[cfg(any(feature="CONFIG_M520x", feature="CONFIG_M523x", feature="CONFIG_M527x", feature="CONFIG_M528x", feature="CONFIG_M53xx", feature="CONFIG_M54xx", feature="CONFIG_M5441x"))] { return MCFGPIO_PDDR + mcfgpio_port(gpio - MCFGPIO_SCR_START); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
