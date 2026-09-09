/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * V4L2 fwnode binding parsing library
 *
 * Copyright (c) 2016 Intel Corporation.
 * Author: Sakari Ailus <sakari.ailus@linux.intel.com>
 *
 * Copyright (C) 2012 - 2013 Samsung Electronics Co., Ltd.
 * Author: Sylwester Nawrocki <s.nawrocki@samsung.com>
 *
 * Copyright (C) 2012 Renesas Electronics Corp.
 * Author: Guennadi Liakhovetski <g.liakhovetski@gmx.de>
 */

#[repr(C)]
pub struct v4l2_fwnode_endpoint {
    pub base: fwnode_endpoint,
    pub bus_type: v4l2_mbus_type,
    pub bus: v4l2_fwnode_endpoint_bus,
    pub link_frequencies: *mut u64,
    pub nr_of_link_frequencies: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct v4l2_fwnode_endpoint_bus {
    pub parallel: v4l2_mbus_config_parallel,
    pub mipi_csi1: v4l2_mbus_config_mipi_csi1,
    pub mipi_csi2: v4l2_mbus_config_mipi_csi2,
}

pub const V4L2_FWNODE_PROPERTY_UNSET: ::core::ffi::c_uint = !0u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum v4l2_fwnode_orientation {
    V4L2_FWNODE_ORIENTATION_FRONT,
    V4L2_FWNODE_ORIENTATION_BACK,
    V4L2_FWNODE_ORIENTATION_EXTERNAL,
}

#[repr(C)]
pub struct v4l2_fwnode_device_properties {
    pub orientation: v4l2_fwnode_orientation,
    pub rotation: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct v4l2_fwnode_link {
    pub local_node: *mut fwnode_handle,
    pub local_port: ::core::ffi::c_uint,
    pub local_id: ::core::ffi::c_uint,
    pub remote_node: *mut fwnode_handle,
    pub remote_port: ::core::ffi::c_uint,
    pub remote_id: ::core::ffi::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum v4l2_connector_type {
    V4L2_CONN_UNKNOWN,
    V4L2_CONN_COMPOSITE,
    V4L2_CONN_SVIDEO,
}

#[repr(C)]
pub struct v4l2_connector_link {
    pub head: list_head,
    pub fwnode_link: v4l2_fwnode_link,
}

#[repr(C)]
pub struct v4l2_fwnode_connector_analog {
    pub sdtv_stds: v4l2_std_id,
}

#[repr(C)]
pub union v4l2_fwnode_connector_data {
    pub analog: v4l2_fwnode_connector_analog,
}

#[repr(C)]
pub struct v4l2_fwnode_connector {
    pub name: *const ::core::ffi::c_char,
    pub label: *const ::core::ffi::c_char,
    pub type_: v4l2_connector_type,
    pub links: list_head,
    pub nr_of_links: ::core::ffi::c_uint,
    pub connector: v4l2_fwnode_connector_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum v4l2_fwnode_bus_type {
    V4L2_FWNODE_BUS_TYPE_GUESS = 0,
    V4L2_FWNODE_BUS_TYPE_CSI2_CPHY,
    V4L2_FWNODE_BUS_TYPE_CSI1,
    V4L2_FWNODE_BUS_TYPE_CCP2,
    V4L2_FWNODE_BUS_TYPE_CSI2_DPHY,
    V4L2_FWNODE_BUS_TYPE_PARALLEL,
    V4L2_FWNODE_BUS_TYPE_BT656,
    V4L2_FWNODE_BUS_TYPE_DPI,
    NR_OF_V4L2_FWNODE_BUS_TYPE,
}

extern "C" {
    pub fn v4l2_fwnode_endpoint_parse(
        fwnode: *mut fwnode_handle,
        vep: *mut v4l2_fwnode_endpoint,
    ) -> ::core::ffi::c_int;
    pub fn v4l2_fwnode_endpoint_free(vep: *mut v4l2_fwnode_endpoint);
    pub fn v4l2_fwnode_endpoint_alloc_parse(
        fwnode: *mut fwnode_handle,
        vep: *mut v4l2_fwnode_endpoint,
    ) -> ::core::ffi::c_int;
    pub fn v4l2_fwnode_parse_link(
        fwnode: *mut fwnode_handle,
        link: *mut v4l2_fwnode_link,
    ) -> ::core::ffi::c_int;
    pub fn v4l2_fwnode_put_link(link: *mut v4l2_fwnode_link);
    pub fn v4l2_fwnode_connector_free(connector: *mut v4l2_fwnode_connector);
    pub fn v4l2_fwnode_connector_parse(
        fwnode: *mut fwnode_handle,
        connector: *mut v4l2_fwnode_connector,
    ) -> ::core::ffi::c_int;
    pub fn v4l2_fwnode_connector_add_link(
        fwnode: *mut fwnode_handle,
        connector: *mut v4l2_fwnode_connector,
    ) -> ::core::ffi::c_int;
    pub fn v4l2_fwnode_device_parse(
        dev: *mut device,
        props: *mut v4l2_fwnode_device_properties,
    ) -> ::core::ffi::c_int;
}

/* The following helpers correspond to the C list_first_entry_or_null and
 * list_last_entry macros; their list primitives are supplied externally. */
#[inline]
pub unsafe fn v4l2_connector_first_link(
    v4l2c: *mut v4l2_fwnode_connector,
) -> *mut v4l2_connector_link {
    list_first_entry_or_null(&mut (*v4l2c).links, v4l2_connector_link, head)
}

#[inline]
pub unsafe fn v4l2_connector_last_link(
    v4l2c: *mut v4l2_fwnode_connector,
) -> *mut v4l2_connector_link {
    list_last_entry(&mut (*v4l2c).links, v4l2_connector_link, head)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
