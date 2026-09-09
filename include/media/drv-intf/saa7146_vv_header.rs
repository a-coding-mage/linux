/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations translated from saa7146_vv.h. External kernel types and symbols
 * are intentionally left as dependencies of the surrounding translation. */

pub const MAX_SAA7146_CAPTURE_BUFFERS: u32 = 32; /* arbitrary */
pub const BUFFER_TIMEOUT: u32 = HZ / 2; /* 0.5 seconds */

macro_rules! WRITE_RPS0 {
    ($dev:expr, $count:expr, $x:expr) => {{
        $dev.d_rps0.cpu_addr[$count] = cpu_to_le32($x);
        $count += 1;
    }};
}

macro_rules! WRITE_RPS1 {
    ($dev:expr, $count:expr, $x:expr) => {{
        $dev.d_rps1.cpu_addr[$count] = cpu_to_le32($x);
        $count += 1;
    }};
}

#[repr(C)]
pub struct saa7146_video_dma {
    pub base_odd: u32,
    pub base_even: u32,
    pub prot_addr: u32,
    pub pitch: u32,
    pub base_page: u32,
    pub num_line_byte: u32,
}

pub const FORMAT_BYTE_SWAP: u32 = 0x1;
pub const FORMAT_IS_PLANAR: u32 = 0x2;

#[repr(C)]
pub struct saa7146_format {
    pub pixelformat: u32,
    pub trans: u32,
    pub depth: u8,
    pub flags: u8,
    pub swap: u8,
}

#[repr(C)]
pub struct saa7146_standard {
    pub name: *mut core::ffi::c_char,
    pub id: v4l2_std_id,
    pub v_offset: core::ffi::c_int,
    pub v_field: core::ffi::c_int,
    pub h_offset: core::ffi::c_int,
    pub h_pixels: core::ffi::c_int,
    pub v_max_out: core::ffi::c_int,
    pub h_max_out: core::ffi::c_int,
}

#[repr(C)]
pub struct saa7146_buf {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub activate: Option<unsafe extern "C" fn(
        *mut saa7146_dev,
        *mut saa7146_buf,
        *mut saa7146_buf,
    ) -> core::ffi::c_int>,
    pub pt: [saa7146_pgtable; 3],
}

#[repr(C)]
pub struct saa7146_dmaqueue {
    pub dev: *mut saa7146_dev,
    pub curr: *mut saa7146_buf,
    pub queue: list_head,
    pub timeout: timer_list,
    pub q: vb2_queue,
}

#[repr(C)]
pub struct saa7146_vv {
    pub vbi_dmaq: saa7146_dmaqueue,
    pub vbi_fmt: v4l2_vbi_format,
    pub vbi_read_timeout: timer_list,
    pub vbi_wq: wait_queue_head_t,
    pub video_dmaq: saa7146_dmaqueue,
    pub video_fmt: v4l2_pix_format,
    pub last_field: v4l2_field,
    pub seqnr: u32,
    pub standard: *mut saa7146_standard,
    pub vflip: core::ffi::c_int,
    pub hflip: core::ffi::c_int,
    pub current_hps_source: core::ffi::c_int,
    pub current_hps_sync: core::ffi::c_int,
    pub resources: u32,
}

pub const SAA7146_USE_PORT_B_FOR_VBI: u32 = 0x2;

#[repr(C)]
pub struct saa7146_ext_vv {
    pub inputs: core::ffi::c_int,
    pub audios: core::ffi::c_int,
    pub capabilities: u32,
    pub flags: core::ffi::c_int,
    pub stds: *mut saa7146_standard,
    pub num_stds: core::ffi::c_int,
    pub std_callback: Option<unsafe extern "C" fn(*mut saa7146_dev, *mut saa7146_standard) -> core::ffi::c_int>,
    pub vid_ops: v4l2_ioctl_ops,
    pub vbi_ops: v4l2_ioctl_ops,
    pub core_ops: *const v4l2_ioctl_ops,
    pub vbi_write: Option<unsafe extern "C" fn(*mut file, *const core::ffi::c_char, usize, *mut loff_t) -> isize>,
}

#[repr(C)]
pub struct saa7146_use_ops {
    pub init: Option<unsafe extern "C" fn(*mut saa7146_dev, *mut saa7146_vv)>,
    pub irq_done: Option<unsafe extern "C" fn(*mut saa7146_dev, u64)>,
}

extern "C" {
    pub static saa7146_video_ioctl_ops: v4l2_ioctl_ops;
    pub static saa7146_vbi_ioctl_ops: v4l2_ioctl_ops;
    pub static saa7146_video_uops: saa7146_use_ops;
    pub static video_qops: vb2_ops;
    pub static saa7146_vbi_uops: saa7146_use_ops;
    pub static vbi_qops: vb2_ops;
    pub fn saa7146_register_device(*mut video_device, *mut saa7146_dev, *mut core::ffi::c_char, core::ffi::c_int) -> core::ffi::c_int;
    pub fn saa7146_unregister_device(*mut video_device, *mut saa7146_dev) -> core::ffi::c_int;
    pub fn saa7146_buffer_finish(*mut saa7146_dev, *mut saa7146_dmaqueue, core::ffi::c_int);
    pub fn saa7146_buffer_next(*mut saa7146_dev, *mut saa7146_dmaqueue, core::ffi::c_int);
    pub fn saa7146_buffer_queue(*mut saa7146_dev, *mut saa7146_dmaqueue, *mut saa7146_buf) -> core::ffi::c_int;
    pub fn saa7146_buffer_timeout(*mut timer_list);
    pub fn saa7146_vv_init(*mut saa7146_dev, *mut saa7146_ext_vv) -> core::ffi::c_int;
    pub fn saa7146_vv_release(*mut saa7146_dev) -> core::ffi::c_int;
    pub fn saa7146_set_capture(*mut saa7146_dev, *mut saa7146_buf, *mut saa7146_buf);
    pub fn saa7146_write_out_dma(*mut saa7146_dev, core::ffi::c_int, *mut saa7146_video_dma);
    pub fn saa7146_set_hps_source_and_sync(*mut saa7146_dev, core::ffi::c_int, core::ffi::c_int);
    pub fn saa7146_set_gpio(*mut saa7146_dev, u8, u8);
    pub fn saa7146_video_do_ioctl(*mut file, u32, *mut core::ffi::c_void) -> isize;
    pub fn saa7146_s_ctrl(*mut v4l2_ctrl) -> core::ffi::c_int;
    pub fn saa7146_res_get(*mut saa7146_dev, u32) -> core::ffi::c_int;
    pub fn saa7146_res_free(*mut saa7146_dev, u32);
}

pub const RESOURCE_DMA1_HPS: u32 = 0x1;
pub const RESOURCE_DMA2_CLP: u32 = 0x2;
pub const RESOURCE_DMA3_BRS: u32 = 0x4;
pub const SAA7146_HPS_SOURCE_PORT_A: u32 = 0x00;
pub const SAA7146_HPS_SOURCE_PORT_B: u32 = 0x01;
pub const SAA7146_HPS_SOURCE_YPB_CPA: u32 = 0x02;
pub const SAA7146_HPS_SOURCE_YPA_CPB: u32 = 0x03;
pub const SAA7146_HPS_SYNC_PORT_A: u32 = 0x00;
pub const SAA7146_HPS_SYNC_PORT_B: u32 = 0x01;
pub const SAA7146_CLIPPING_MEM: usize = 16 * 4 * core::mem::size_of::<u32>();
pub const SAA7146_CLIPPING_RECT: u32 = 0x4;
pub const SAA7146_CLIPPING_RECT_INVERTED: u32 = 0x5;
pub const SAA7146_CLIPPING_MASK: u32 = 0x6;
pub const SAA7146_CLIPPING_MASK_INVERTED: u32 = 0x7;
pub const RGB08_COMPOSED: u32 = 0x0217;
pub const RGB15_COMPOSED: u32 = 0x0213;
pub const RGB16_COMPOSED: u32 = 0x0210;
pub const RGB24_COMPOSED: u32 = 0x0201;
pub const RGB32_COMPOSED: u32 = 0x0202;
pub const Y8: u32 = 0x0006;
pub const YUV411_COMPOSED: u32 = 0x0003;
pub const YUV422_COMPOSED: u32 = 0x0000;
pub const YUV411_DECOMPOSED: u32 = 0x100b;
pub const YUV422_DECOMPOSED: u32 = 0x1009;
pub const YUV420_DECOMPOSED: u32 = 0x100a;

#[inline]
pub const fn IS_PLANAR(x: u32) -> u32 { x & 0xf000 }
pub const SAA7146_NO_SWAP: u32 = 0x0;
pub const SAA7146_TWO_BYTE_SWAP: u32 = 0x1;
pub const SAA7146_FOUR_BYTE_SWAP: u32 = 0x2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
