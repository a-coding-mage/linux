/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation: linux/workqueue.h

pub type SpiBbTxrxWordFn = unsafe extern "C" fn(
    *mut spi_device,
    ::core::ffi::c_uint,
    u32,
    u8,
    ::core::ffi::c_uint,
) -> u32;

#[repr(C)]
pub struct spi_bitbang {
    pub lock: mutex,
    pub busy: u8,
    pub use_dma: u8,
    pub flags: u16, /* extra spi->mode support */

    pub ctlr: *mut spi_controller,

    /* setup_transfer() changes clock and/or wordsize to match settings
     * for this transfer; zeroes restore defaults from spi_device.
     */
    pub setup_transfer: Option<unsafe extern "C" fn(
        *mut spi_device,
        *mut spi_transfer,
    ) -> ::core::ffi::c_int>,

    pub chipselect: Option<unsafe extern "C" fn(*mut spi_device, ::core::ffi::c_int)>,

    pub set_mosi_idle: Option<unsafe extern "C" fn(*mut spi_device)>,
    /* txrx_bufs() may handle dma mapping for transfers that don't
     * already have one (transfer.{tx,rx}_dma is zero), or use PIO
     */
    pub txrx_bufs: Option<unsafe extern "C" fn(
        *mut spi_device,
        *mut spi_transfer,
    ) -> ::core::ffi::c_int>,

    /* txrx_word[SPI_MODE_*]() just looks like a shift register */
    pub txrx_word: [Option<SpiBbTxrxWordFn>; SPI_MODE_X_MASK + 1],

    pub set_line_direction:
        Option<unsafe extern "C" fn(*mut spi_device, bool) -> ::core::ffi::c_int>,
}

pub const BITBANG_CS_ACTIVE: ::core::ffi::c_int = 1; /* normally nCS, active low */
pub const BITBANG_CS_INACTIVE: ::core::ffi::c_int = 0;

/* you can call these default bitbang->master methods from your custom
 * methods, if you like.
 */
extern "C" {
    pub fn spi_bitbang_setup(spi: *mut spi_device) -> ::core::ffi::c_int;
    pub fn spi_bitbang_cleanup(spi: *mut spi_device);
    pub fn spi_bitbang_setup_transfer(
        spi: *mut spi_device,
        t: *mut spi_transfer,
    ) -> ::core::ffi::c_int;

    /* start or stop queue processing */
    pub fn spi_bitbang_start(spi: *mut spi_bitbang) -> ::core::ffi::c_int;
    pub fn spi_bitbang_init(spi: *mut spi_bitbang) -> ::core::ffi::c_int;
    pub fn spi_bitbang_stop(spi: *mut spi_bitbang);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
