/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * v4l2-mc.h - Media Controller V4L2 types and prototypes
 *
 * Copyright (C) 2016 Mauro Carvalho Chehab <mchehab@kernel.org>
 * Copyright (C) 2006-2010 Nokia Corporation
 * Copyright (c) 2016 Intel Corporation.
 */

/* We don't need to include pci.h or usb.h here. */

use core::ffi::c_int;

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usb_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct media_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct video_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vb2_queue {
    _private: [u8; 0],
}

#[repr(C)]
pub struct v4l2_subdev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct media_pad {
    _private: [u8; 0],
}

#[repr(C)]
pub struct media_entity {
    _private: [u8; 0],
}

#[repr(C)]
pub struct media_link {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_MEDIA_CONTROLLER")]
extern "C" {
    /** v4l2_mc_create_media_graph() - create Media Controller links at the graph. */
    pub fn v4l2_mc_create_media_graph(mdev: *mut media_device) -> c_int;

    /** v4l_enable_media_source() - Hold media source for exclusive use if free. */
    pub fn v4l_enable_media_source(vdev: *mut video_device) -> c_int;

    /** v4l_disable_media_source() - Release media source. */
    pub fn v4l_disable_media_source(vdev: *mut video_device);

    /** v4l_vb2q_enable_media_tuner - Hold media source for exclusive use if free. */
    pub fn v4l_vb2q_enable_media_source(q: *mut vb2_queue) -> c_int;

    /** v4l2_create_fwnode_links_to_pad - Create fwnode-based links to a sink pad. */
    pub fn v4l2_create_fwnode_links_to_pad(
        src_sd: *mut v4l2_subdev,
        sink: *mut media_pad,
        flags: u32,
    ) -> c_int;

    /** v4l2_create_fwnode_links - Create fwnode-based links from source to sink. */
    pub fn v4l2_create_fwnode_links(
        src_sd: *mut v4l2_subdev,
        sink_sd: *mut v4l2_subdev,
    ) -> c_int;

    /** Deprecated: increase the use count of a pipeline. */
    pub fn v4l2_pipeline_pm_get(entity: *mut media_entity) -> c_int;

    /** Deprecated: decrease the use count of a pipeline. */
    pub fn v4l2_pipeline_pm_put(entity: *mut media_entity);

    /** Deprecated: link management notification callback. */
    pub fn v4l2_pipeline_link_notify(
        link: *mut media_link,
        flags: u32,
        notification: u32,
    ) -> c_int;
}

#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn v4l2_mc_create_media_graph(_mdev: *mut media_device) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn v4l_enable_media_source(_vdev: *mut video_device) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn v4l_disable_media_source(_vdev: *mut video_device) {}

#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn v4l_vb2q_enable_media_source(_q: *mut vb2_queue) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn v4l2_pipeline_pm_get(_entity: *mut media_entity) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn v4l2_pipeline_pm_put(_entity: *mut media_entity) {}

#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn v4l2_pipeline_link_notify(
    _link: *mut media_link,
    _flags: u32,
    _notification: u32,
) -> c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
