/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 */

// C dependencies: <linux/errno.h>, <linux/types.h>, <linux/vmalloc.h>

#[cfg(CONFIG_BCM47XX_NVRAM)]
extern "C" {
    pub fn bcm47xx_nvram_init_from_iomem(
        nvram_start: *mut core::ffi::c_void,
        res_size: usize,
    ) -> core::ffi::c_int;
    pub fn bcm47xx_nvram_init_from_mem(base: u32, lim: u32) -> core::ffi::c_int;
    pub fn bcm47xx_nvram_getenv(
        name: *const core::ffi::c_char,
        val: *mut core::ffi::c_char,
        val_len: usize,
    ) -> core::ffi::c_int;
    pub fn bcm47xx_nvram_gpio_pin(name: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn bcm47xx_nvram_get_contents(val_len: *mut usize) -> *mut core::ffi::c_char;
    pub fn vfree(addr: *mut core::ffi::c_void);
}

#[cfg(CONFIG_BCM47XX_NVRAM)]
#[inline]
pub unsafe fn bcm47xx_nvram_release_contents(nvram: *mut core::ffi::c_char) {
    vfree(nvram.cast());
}

#[cfg(not(CONFIG_BCM47XX_NVRAM))]
#[inline]
pub unsafe fn bcm47xx_nvram_init_from_iomem(
    _nvram_start: *mut core::ffi::c_void,
    _res_size: usize,
) -> core::ffi::c_int {
    -ENOTSUPP
}

#[cfg(not(CONFIG_BCM47XX_NVRAM))]
#[inline]
pub unsafe fn bcm47xx_nvram_init_from_mem(_base: u32, _lim: u32) -> core::ffi::c_int {
    -ENOTSUPP
}

#[cfg(not(CONFIG_BCM47XX_NVRAM))]
#[inline]
pub unsafe fn bcm47xx_nvram_getenv(
    _name: *const core::ffi::c_char,
    _val: *mut core::ffi::c_char,
    _val_len: usize,
) -> core::ffi::c_int {
    -ENOTSUPP
}

#[cfg(not(CONFIG_BCM47XX_NVRAM))]
#[inline]
pub unsafe fn bcm47xx_nvram_gpio_pin(_name: *const core::ffi::c_char) -> core::ffi::c_int {
    -ENOTSUPP
}

#[cfg(not(CONFIG_BCM47XX_NVRAM))]
#[inline]
pub unsafe fn bcm47xx_nvram_get_contents(_val_len: *mut usize) -> *mut core::ffi::c_char {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_BCM47XX_NVRAM))]
#[inline]
pub unsafe fn bcm47xx_nvram_release_contents(_nvram: *mut core::ffi::c_char) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
