/* SPDX-License-Identifier: GPL-2.0 */

// S3C_FB_MAX_WIN: maximum number of windows supported by any hardware.
pub const S3C_FB_MAX_WIN: usize = 5;

/**
 * struct s3c_fb_pd_win - per window setup data
 * @xres     : The window X size.
 * @yres     : The window Y size.
 * @virtual_x: The virtual X size.
 * @virtual_y: The virtual Y size.
 */
#[repr(C)]
pub struct s3c_fb_pd_win {
    pub default_bpp: u16,
    pub max_bpp: u16,
    pub xres: u16,
    pub yres: u16,
    pub virtual_x: u16,
    pub virtual_y: u16,
}

/**
 * struct s3c_fb_platdata - S3C driver platform specific information
 * @setup_gpio: Setup the external GPIO pins to the right state to transfer
 *      the data from the display system to the connected display device.
 * @vidcon0: The base vidcon0 values to control the panel data format.
 * @vidcon1: The base vidcon1 values to control the panel data output.
 * @vtiming: Video timing when connected to a RGB type panel.
 * @win: The setup data for each hardware window, or NULL for unused.
 * @display_mode: The LCD output display mode.
 *
 * The platform data supplies the video driver with all the information
 * it requires to work with the display(s) attached to the machine. It
 * controls the initial mode, the number of display windows (0 is always
 * the base framebuffer) that are initialised etc.
 */
#[repr(C)]
pub struct s3c_fb_platdata {
    pub setup_gpio: Option<unsafe extern "C" fn()>,
    pub win: [*mut s3c_fb_pd_win; S3C_FB_MAX_WIN],
    pub vtiming: *mut fb_videomode,
    pub vidcon0: u32,
    pub vidcon1: u32,
}

// `fb_videomode` is supplied by the framebuffer subsystem.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
