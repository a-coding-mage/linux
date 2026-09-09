/* SPDX-License-Identifier: GPL-2.0 */

// Dependency declarations supplied by the corresponding Linux headers.
use core::ffi::{c_int, c_ulong, c_uint, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mmc_host {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    _private: [u8; 0],
}

// Supplied by linux/interrupt.h.
pub type irqreturn_t = c_int;

/* Put this in platform_data of a device being used to manage an MMC/SD
 * card slot.  (Modeled after PXA mmc glue; see that for usage examples.)
 *
 * REVISIT This is not a spi-specific notion.  Any card slot should be
 * able to handle it.  If the MMC core doesn't adopt this kind of notion,
 * switch the "struct device *" parameters over to "struct spi_device *".
 */
#[repr(C)]
pub struct mmc_spi_platform_data {
    /* driver activation and (optional) card detect irq hookup */
    pub init: Option<unsafe extern "C" fn(
        *mut device,
        Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        *mut c_void,
    ) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut device, *mut c_void)>,

    /* Capabilities to pass into mmc core (e.g. MMC_CAP_NEEDS_POLL). */
    pub caps: c_ulong,
    pub caps2: c_ulong,

    /* how long to debounce card detect, in msecs */
    pub detect_delay: u16,

    /* power management */
    pub powerup_msecs: u16, /* delay of up to 250 msec */
    pub ocr_mask: u32,      /* available voltages */
    pub setpower: Option<unsafe extern "C" fn(*mut device, c_uint)>,
}

unsafe extern "C" {
    pub fn mmc_spi_get_pdata(spi: *mut spi_device) -> *mut mmc_spi_platform_data;
    pub fn mmc_spi_put_pdata(spi: *mut spi_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
