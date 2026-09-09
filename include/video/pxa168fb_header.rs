/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (C) 2009 Marvell International Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

/* Dumb interface */
pub const PIN_MODE_DUMB_24: u32 = 0;
pub const PIN_MODE_DUMB_18_SPI: u32 = 1;
pub const PIN_MODE_DUMB_18_GPIO: u32 = 2;
pub const PIN_MODE_DUMB_16_SPI: u32 = 3;
pub const PIN_MODE_DUMB_16_GPIO: u32 = 4;
pub const PIN_MODE_DUMB_12_SPI_GPIO: u32 = 5;
pub const PIN_MODE_SMART_18_SPI: u32 = 6;
pub const PIN_MODE_SMART_16_SPI: u32 = 7;
pub const PIN_MODE_SMART_8_SPI_GPIO: u32 = 8;

/* Dumb interface pin allocation */
pub const DUMB_MODE_RGB565: u32 = 0;
pub const DUMB_MODE_RGB565_UPPER: u32 = 1;
pub const DUMB_MODE_RGB666: u32 = 2;
pub const DUMB_MODE_RGB666_UPPER: u32 = 3;
pub const DUMB_MODE_RGB444: u32 = 4;
pub const DUMB_MODE_RGB444_UPPER: u32 = 5;
pub const DUMB_MODE_RGB888: u32 = 6;

/* default fb buffer size WVGA-32bits */
pub const DEFAULT_FB_SIZE: usize = 800 * 480 * 4;

/*
 * Buffer pixel format
 * bit0 is for rb swap.
 * bit12 is for Y UorV swap
 */
pub const PIX_FMT_RGB565: u32 = 0;
pub const PIX_FMT_BGR565: u32 = 1;
pub const PIX_FMT_RGB1555: u32 = 2;
pub const PIX_FMT_BGR1555: u32 = 3;
pub const PIX_FMT_RGB888PACK: u32 = 4;
pub const PIX_FMT_BGR888PACK: u32 = 5;
pub const PIX_FMT_RGB888UNPACK: u32 = 6;
pub const PIX_FMT_BGR888UNPACK: u32 = 7;
pub const PIX_FMT_RGBA888: u32 = 8;
pub const PIX_FMT_BGRA888: u32 = 9;
pub const PIX_FMT_YUV422PACK: u32 = 10;
pub const PIX_FMT_YVU422PACK: u32 = 11;
pub const PIX_FMT_YUV422PLANAR: u32 = 12;
pub const PIX_FMT_YVU422PLANAR: u32 = 13;
pub const PIX_FMT_YUV420PLANAR: u32 = 14;
pub const PIX_FMT_YVU420PLANAR: u32 = 15;
pub const PIX_FMT_PSEUDOCOLOR: u32 = 20;
pub const PIX_FMT_UYVY422PACK: u32 = 0x1000 | PIX_FMT_YUV422PACK;

/*
 * PXA LCD controller private state.
 */
#[repr(C)]
pub struct pxa168fb_info {
    pub dev: *mut device,
    pub clk: *mut clk,
    pub info: *mut fb_info,

    pub reg_base: *mut core::ffi::c_void,
    pub fb_start_dma: dma_addr_t,
    pub pseudo_palette: [u32; 16],

    pub pix_fmt: i32,
    // C bit-fields: is_blanked:1, panel_rbswap:1, active:1.
    pub is_blanked: u32,
    pub panel_rbswap: u32,
    pub active: u32,
}

/*
 * PXA fb machine information
 */
#[repr(C)]
pub struct pxa168fb_mach_info {
    pub id: [core::ffi::c_char; 16],

    pub num_modes: i32,
    pub modes: *mut fb_videomode,

    /*
     * Pix_fmt
     */
    pub pix_fmt: u32,

    /*
     * I/O pin allocation.
     */
    // C bit-field: io_pin_allocation_mode:4.
    pub io_pin_allocation_mode: u32,

    /*
     * Dumb panel -- assignment of R/G/B component info to the 24
     * available external data lanes.
     */
    // C bit-fields: dumb_mode:4, panel_rgb_reverse_lanes:1.
    pub dumb_mode: u32,
    pub panel_rgb_reverse_lanes: u32,

    /*
     * Dumb panel -- GPIO output data.
     */
    // C bit-fields: gpio_output_mask:8, gpio_output_data:8.
    pub gpio_output_mask: u32,
    pub gpio_output_data: u32,

    /*
     * Dumb panel -- configurable output signal polarity.
     */
    // C bit-fields: invert_composite_blank:1, invert_pix_val_ena:1,
    // invert_pixclock:1, panel_rbswap:1, active:1, enable_lcd:1.
    pub invert_composite_blank: u32,
    pub invert_pix_val_ena: u32,
    pub invert_pixclock: u32,
    pub panel_rbswap: u32,
    pub active: u32,
    pub enable_lcd: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
