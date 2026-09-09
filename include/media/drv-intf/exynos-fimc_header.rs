/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Samsung S5P/Exynos4 SoC series camera interface driver header
 *
 * Copyright (C) 2010 - 2013 Samsung Electronics Co., Ltd.
 * Sylwester Nawrocki <s.nawrocki@samsung.com>
 */

/* Dependencies: media-entity.h, v4l2-dev.h, and v4l2-mediabus.h. */

/* Enumeration of data inputs to the camera subsystem. */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fimc_input {
    FIMC_INPUT_PARALLEL_0 = 1,
    FIMC_INPUT_PARALLEL_1,
    FIMC_INPUT_MIPI_CSI2_0 = 3,
    FIMC_INPUT_MIPI_CSI2_1,
    FIMC_INPUT_WRITEBACK_A = 5,
    FIMC_INPUT_WRITEBACK_B,
    FIMC_INPUT_WRITEBACK_ISP = 5,
}

/* Enumeration of the FIMC data bus types. */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fimc_bus_type {
    /* Camera parallel bus */
    FIMC_BUS_TYPE_ITU_601 = 1,
    /* Camera parallel bus with embedded synchronization */
    FIMC_BUS_TYPE_ITU_656,
    /* Camera MIPI-CSI2 serial bus */
    FIMC_BUS_TYPE_MIPI_CSI2,
    /* FIFO link from LCD controller (WriteBack A) */
    FIMC_BUS_TYPE_LCD_WRITEBACK_A,
    /* FIFO link from LCD controller (WriteBack B) */
    FIMC_BUS_TYPE_LCD_WRITEBACK_B,
    /* FIFO link from FIMC-IS */
    FIMC_BUS_TYPE_ISP_WRITEBACK = FIMC_BUS_TYPE_LCD_WRITEBACK_B as isize,
}

#[inline]
pub const fn fimc_input_is_parallel(x: u32) -> bool { x == 1 || x == 2 }
#[inline]
pub const fn fimc_input_is_mipi_csi(x: u32) -> bool { x == 3 || x == 4 }

/* The subdevices' group IDs. */
pub const GRP_ID_SENSOR: u32 = 1 << 8;
pub const GRP_ID_FIMC_IS_SENSOR: u32 = 1 << 9;
pub const GRP_ID_WRITEBACK: u32 = 1 << 10;
pub const GRP_ID_CSIS: u32 = 1 << 11;
pub const GRP_ID_FIMC: u32 = 1 << 12;
pub const GRP_ID_FLITE: u32 = 1 << 13;
pub const GRP_ID_FIMC_IS: u32 = 1 << 14;

#[repr(C)]
pub struct fimc_source_info {
    pub fimc_bus_type: fimc_bus_type,
    pub sensor_bus_type: fimc_bus_type,
    pub flags: u16,
    pub mux_id: u16,
}

/* v4l2_device notification id. This is only for internal use in the kernel.
 * Sensor subdevs should issue S5P_FIMC_TX_END_NOTIFY notification in single
 * frame capture mode when there is only one VSYNC pulse issued by the sensor
 * at beginning of the frame transmission.
 */
/* _IO('e', 0), supplied by the dependent ioctl definitions. */
pub const S5P_FIMC_TX_END_NOTIFY: u32 = 0;

pub const FIMC_MAX_PLANES: usize = 3;

#[repr(C)]
pub struct fimc_fmt {
    pub mbus_code: u32,
    pub fourcc: u32,
    pub color: u32,
    pub memplanes: u16,
    pub colplanes: u16,
    pub colorspace: u8,
    pub depth: [u8; FIMC_MAX_PLANES],
    pub mdataplanes: u16,
    pub flags: u16,
}

pub const FMT_FLAGS_CAM: u16 = 1 << 0;
pub const FMT_FLAGS_M2M_IN: u16 = 1 << 1;
pub const FMT_FLAGS_M2M_OUT: u16 = 1 << 2;
pub const FMT_FLAGS_M2M: u16 = (1 << 1) | (1 << 2);
pub const FMT_HAS_ALPHA: u16 = 1 << 3;
pub const FMT_FLAGS_COMPRESSED: u16 = 1 << 4;
pub const FMT_FLAGS_WRITEBACK: u16 = 1 << 5;
pub const FMT_FLAGS_RAW_BAYER: u16 = 1 << 6;
pub const FMT_FLAGS_YUV: u16 = 1 << 7;

pub enum media_entity {}
pub enum video_device {}
pub enum media_pipeline {}

#[repr(C)]
pub struct exynos_media_pipeline_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut exynos_media_pipeline, *mut media_entity) -> i32>,
    pub unprepare: Option<unsafe extern "C" fn(*mut exynos_media_pipeline) -> i32>,
    pub open: Option<unsafe extern "C" fn(*mut exynos_media_pipeline, *mut media_entity, bool) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut exynos_media_pipeline) -> i32>,
    pub set_stream: Option<unsafe extern "C" fn(*mut exynos_media_pipeline, bool) -> i32>,
}

#[repr(C)]
pub struct exynos_video_entity {
    pub vdev: video_device,
    pub pipe: *mut exynos_media_pipeline,
}

#[repr(C)]
pub struct exynos_media_pipeline {
    pub mp: media_pipeline,
    pub ops: *const exynos_media_pipeline_ops,
}

#[inline]
pub unsafe fn vdev_to_exynos_video_entity(vdev: *mut video_device) -> *mut exynos_video_entity {
    vdev as *mut exynos_video_entity
}

/* Implemented by the related media device driver; -ENOENT and -ENOIOCTLCMD
 * are supplied by the dependent kernel errno definitions. */
#[inline]
pub unsafe fn fimc_pipeline_call(
    ent: *mut exynos_video_entity,
    op: fn(&exynos_media_pipeline_ops) -> Option<unsafe extern "C" fn(*mut exynos_media_pipeline) -> i32>,
) -> i32 {
    if ent.is_null() || (*ent).pipe.is_null() {
        return -2;
    }
    match op(&*(*(*ent).pipe).ops) {
        Some(call) => call((*ent).pipe),
        None => -515,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
