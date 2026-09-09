/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* V4L2 subdev userspace API. */
/* External Linux types and ioctl macros are supplied by other translated headers. */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum v4l2_subdev_format_whence {
    V4L2_SUBDEV_FORMAT_TRY = 0,
    V4L2_SUBDEV_FORMAT_ACTIVE = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_format {
    pub which: __u32,
    pub pad: __u32,
    pub format: v4l2_mbus_framefmt,
    pub stream: __u32,
    pub reserved: [__u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_crop {
    pub which: __u32,
    pub pad: __u32,
    pub rect: v4l2_rect,
    pub stream: __u32,
    pub reserved: [__u32; 7],
}

pub const V4L2_SUBDEV_MBUS_CODE_CSC_COLORSPACE: __u32 = 0x00000001;
pub const V4L2_SUBDEV_MBUS_CODE_CSC_XFER_FUNC: __u32 = 0x00000002;
pub const V4L2_SUBDEV_MBUS_CODE_CSC_YCBCR_ENC: __u32 = 0x00000004;
pub const V4L2_SUBDEV_MBUS_CODE_CSC_HSV_ENC: __u32 = V4L2_SUBDEV_MBUS_CODE_CSC_YCBCR_ENC;
pub const V4L2_SUBDEV_MBUS_CODE_CSC_QUANTIZATION: __u32 = 0x00000008;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_mbus_code_enum {
    pub pad: __u32,
    pub index: __u32,
    pub code: __u32,
    pub which: __u32,
    pub flags: __u32,
    pub stream: __u32,
    pub reserved: [__u32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_frame_size_enum {
    pub index: __u32,
    pub pad: __u32,
    pub code: __u32,
    pub min_width: __u32,
    pub max_width: __u32,
    pub min_height: __u32,
    pub max_height: __u32,
    pub which: __u32,
    pub stream: __u32,
    pub reserved: [__u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_frame_interval {
    pub pad: __u32,
    pub interval: v4l2_fract,
    pub stream: __u32,
    pub which: __u32,
    pub reserved: [__u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_frame_interval_enum {
    pub index: __u32,
    pub pad: __u32,
    pub code: __u32,
    pub width: __u32,
    pub height: __u32,
    pub interval: v4l2_fract,
    pub which: __u32,
    pub stream: __u32,
    pub reserved: [__u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_selection {
    pub which: __u32,
    pub pad: __u32,
    pub target: __u32,
    pub flags: __u32,
    pub r: v4l2_rect,
    pub stream: __u32,
    pub reserved: [__u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_capability {
    pub version: __u32,
    pub capabilities: __u32,
    pub reserved: [__u32; 14],
}

pub const V4L2_SUBDEV_CAP_RO_SUBDEV: __u32 = 0x00000001;
pub const V4L2_SUBDEV_CAP_STREAMS: __u32 = 0x00000002;
pub const V4L2_SUBDEV_ROUTE_FL_ACTIVE: __u32 = 1u32 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_route {
    pub sink_pad: __u32,
    pub sink_stream: __u32,
    pub source_pad: __u32,
    pub source_stream: __u32,
    pub flags: __u32,
    pub reserved: [__u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_routing {
    pub which: __u32,
    pub len_routes: __u32,
    pub routes: __u64,
    pub num_routes: __u32,
    pub reserved: [__u32; 11],
}

pub const V4L2_SUBDEV_CLIENT_CAP_STREAMS: __u64 = 1u64 << 0;
pub const V4L2_SUBDEV_CLIENT_CAP_INTERVAL_USES_WHICH: __u64 = 1u64 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_client_capability {
    pub capabilities: __u64,
}

/* Backwards compatibility define --- to be removed. */
pub use v4l2_edid as v4l2_subdev_edid;

/* Ioctl values retain the source header's external _IOR/_IOW/_IOWR definitions. */
pub const VIDIOC_SUBDEV_QUERYCAP: _ = _IOR!('V', 0, v4l2_subdev_capability);
pub const VIDIOC_SUBDEV_G_FMT: _ = _IOWR!('V', 4, v4l2_subdev_format);
pub const VIDIOC_SUBDEV_S_FMT: _ = _IOWR!('V', 5, v4l2_subdev_format);
pub const VIDIOC_SUBDEV_G_FRAME_INTERVAL: _ = _IOWR!('V', 21, v4l2_subdev_frame_interval);
pub const VIDIOC_SUBDEV_S_FRAME_INTERVAL: _ = _IOWR!('V', 22, v4l2_subdev_frame_interval);
pub const VIDIOC_SUBDEV_ENUM_MBUS_CODE: _ = _IOWR!('V', 2, v4l2_subdev_mbus_code_enum);
pub const VIDIOC_SUBDEV_ENUM_FRAME_SIZE: _ = _IOWR!('V', 74, v4l2_subdev_frame_size_enum);
pub const VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL: _ = _IOWR!('V', 75, v4l2_subdev_frame_interval_enum);
pub const VIDIOC_SUBDEV_G_CROP: _ = _IOWR!('V', 59, v4l2_subdev_crop);
pub const VIDIOC_SUBDEV_S_CROP: _ = _IOWR!('V', 60, v4l2_subdev_crop);
pub const VIDIOC_SUBDEV_G_SELECTION: _ = _IOWR!('V', 61, v4l2_subdev_selection);
pub const VIDIOC_SUBDEV_S_SELECTION: _ = _IOWR!('V', 62, v4l2_subdev_selection);
pub const VIDIOC_SUBDEV_G_ROUTING: _ = _IOWR!('V', 38, v4l2_subdev_routing);
pub const VIDIOC_SUBDEV_S_ROUTING: _ = _IOWR!('V', 39, v4l2_subdev_routing);
pub const VIDIOC_SUBDEV_G_CLIENT_CAP: _ = _IOR!('V', 101, v4l2_subdev_client_capability);
pub const VIDIOC_SUBDEV_S_CLIENT_CAP: _ = _IOWR!('V', 102, v4l2_subdev_client_capability);

pub const VIDIOC_SUBDEV_G_STD: _ = _IOR!('V', 23, v4l2_std_id);
pub const VIDIOC_SUBDEV_S_STD: _ = _IOW!('V', 24, v4l2_std_id);
pub const VIDIOC_SUBDEV_ENUMSTD: _ = _IOWR!('V', 25, v4l2_standard);
pub const VIDIOC_SUBDEV_G_EDID: _ = _IOWR!('V', 40, v4l2_edid);
pub const VIDIOC_SUBDEV_S_EDID: _ = _IOWR!('V', 41, v4l2_edid);
pub const VIDIOC_SUBDEV_QUERYSTD: _ = _IOR!('V', 63, v4l2_std_id);
pub const VIDIOC_SUBDEV_S_DV_TIMINGS: _ = _IOWR!('V', 87, v4l2_dv_timings);
pub const VIDIOC_SUBDEV_G_DV_TIMINGS: _ = _IOWR!('V', 88, v4l2_dv_timings);
pub const VIDIOC_SUBDEV_ENUM_DV_TIMINGS: _ = _IOWR!('V', 98, v4l2_enum_dv_timings);
pub const VIDIOC_SUBDEV_QUERY_DV_TIMINGS: _ = _IOR!('V', 99, v4l2_dv_timings);
pub const VIDIOC_SUBDEV_DV_TIMINGS_CAP: _ = _IOWR!('V', 100, v4l2_dv_timings_cap);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
