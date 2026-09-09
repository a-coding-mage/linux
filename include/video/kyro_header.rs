/*
 *  linux/drivers/video/kyro/kryo.h
 *
 *  Copyright (C) 2002 STMicroelectronics
 *  Copyright (C) 2004 Paul Mundt
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

#[repr(C)]
pub struct kyrofb_info {
    pub regbase: *mut core::ffi::c_void,

    pub palette: [u32; 16],
    pub HTot: u32, /* Hor Total Time    */
    pub HFP: u32,  /* Hor Front Porch   */
    pub HST: u32,  /* Hor Sync Time     */
    pub HBP: u32,  /* Hor Back Porch    */
    pub HSP: i32,  /* Hor Sync Polarity */
    pub VTot: u32, /* Ver Total Time    */
    pub VFP: u32,  /* Ver Front Porch   */
    pub VST: u32,  /* Ver Sync Time     */
    pub VBP: u32,  /* Ver Back Porch    */
    pub VSP: i32,  /* Ver Sync Polarity */
    pub XRES: u32, /* X Resolution      */
    pub YRES: u32, /* Y Resolution      */
    pub VFREQ: u32, /* Ver Frequency     */
    pub PIXCLK: u32, /* Pixel Clock       */
    pub HCLK: u32,  /* Hor Clock         */

    /* Useful to hold depth here for Linux */
    pub PIXDEPTH: u8,

    pub wc_cookie: i32,
}

/*
 * benedict.gaster@superh.com
 * Added the follow IOCTLS for the creation of overlay services...
 */
pub const KYRO_IOC_MAGIC: u8 = b'k';

/* Linux _IO(type, nr): no encoded data payload. */
pub const KYRO_IOCTL_OVERLAY_CREATE: u32 = ((KYRO_IOC_MAGIC as u32) << 8) | 0;
pub const KYRO_IOCTL_OVERLAY_VIEWPORT_SET: u32 = ((KYRO_IOC_MAGIC as u32) << 8) | 1;
pub const KYRO_IOCTL_SET_VIDEO_MODE: u32 = ((KYRO_IOC_MAGIC as u32) << 8) | 2;
pub const KYRO_IOCTL_UVSTRIDE: u32 = ((KYRO_IOC_MAGIC as u32) << 8) | 3;
pub const KYRO_IOCTL_OVERLAY_OFFSET: u32 = ((KYRO_IOC_MAGIC as u32) << 8) | 4;
pub const KYRO_IOCTL_STRIDE: u32 = ((KYRO_IOC_MAGIC as u32) << 8) | 5;

/*
 * The follow 3 structures are used to pass data from user space into the kernel
 * for the creation of overlay surfaces and setting the video mode.
 */
#[repr(C)]
pub struct _OVERLAY_CREATE {
    pub ulWidth: u32,
    pub ulHeight: u32,
    pub bLinear: i32,
}
pub type overlay_create = _OVERLAY_CREATE;

#[repr(C)]
pub struct _OVERLAY_VIEWPORT_SET {
    pub xOrgin: u32,
    pub yOrgin: u32,
    pub xSize: u32,
    pub ySize: u32,
}
pub type overlay_viewport_set = _OVERLAY_VIEWPORT_SET;

#[repr(C)]
pub struct _SET_VIDEO_MODE {
    pub ulWidth: u32,
    pub ulHeight: u32,
    pub ulScan: u32,
    pub displayDepth: u8,
    pub bLinear: i32,
}
pub type set_video_mode = _SET_VIDEO_MODE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
