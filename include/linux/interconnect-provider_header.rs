/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018, Linaro Ltd.
 * Author: Georgi Djakov <georgi.djakov@linaro.org>
 */

// Dependency: linux/interconnect.h

#[inline]
pub const fn icc_units_to_bps(bw: u64) -> u64 {
    bw.wrapping_mul(1000u64)
}

#[repr(C)]
pub struct icc_node;
#[repr(C)]
pub struct of_phandle_args;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct list_head;
#[repr(C)]
pub struct hlist_head;

#[repr(C)]
pub struct icc_node_data {
    pub node: *mut icc_node,
    pub tag: u32,
}

#[repr(C)]
pub struct icc_onecell_data {
    pub num_nodes: ::core::ffi::c_uint,
    pub nodes: [*mut icc_node; 0],
}

unsafe extern "C" {
    pub fn of_icc_xlate_onecell(
        spec: *const of_phandle_args,
        data: *mut ::core::ffi::c_void,
    ) -> *mut icc_node;
}

#[repr(C)]
pub struct icc_provider {
    pub provider_list: list_head,
    pub nodes: list_head,
    pub set: Option<unsafe extern "C" fn(src: *mut icc_node, dst: *mut icc_node) -> i32>,
    pub aggregate: Option<unsafe extern "C" fn(
        node: *mut icc_node,
        tag: u32,
        avg_bw: u32,
        peak_bw: u32,
        agg_avg: *mut u32,
        agg_peak: *mut u32,
    ) -> i32>,
    pub pre_aggregate: Option<unsafe extern "C" fn(node: *mut icc_node)>,
    pub get_bw: Option<unsafe extern "C" fn(node: *mut icc_node, avg: *mut u32, peak: *mut u32) -> i32>,
    pub xlate: Option<unsafe extern "C" fn(spec: *const of_phandle_args, data: *mut ::core::ffi::c_void) -> *mut icc_node>,
    pub xlate_extended: Option<unsafe extern "C" fn(spec: *const of_phandle_args, data: *mut ::core::ffi::c_void) -> *mut icc_node_data>,
    pub dev: *mut device,
    pub users: i32,
    pub inter_set: bool,
    pub data: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct icc_node {
    pub id: i32,
    pub name: *const ::core::ffi::c_char,
    pub links: *mut *mut icc_node,
    pub num_links: usize,
    pub provider: *mut icc_provider,
    pub node_list: list_head,
    pub search_list: list_head,
    pub reverse: *mut icc_node,
    pub is_traversed: u8,
    pub req_list: hlist_head,
    pub avg_bw: u32,
    pub peak_bw: u32,
    pub init_avg: u32,
    pub init_peak: u32,
    pub data: *mut ::core::ffi::c_void,
}

// The following declarations are enabled when CONFIG_INTERCONNECT is enabled.
#[cfg(feature = "CONFIG_INTERCONNECT")]
unsafe extern "C" {
    pub fn icc_std_aggregate(node: *mut icc_node, tag: u32, avg_bw: u32, peak_bw: u32, agg_avg: *mut u32, agg_peak: *mut u32) -> i32;
    pub fn icc_node_create_dyn() -> *mut icc_node;
    pub fn icc_node_create(id: i32) -> *mut icc_node;
    pub fn icc_node_destroy(id: i32);
    pub fn icc_node_set_name(node: *mut icc_node, provider: *const icc_provider, name: *const ::core::ffi::c_char) -> i32;
    pub fn icc_link_nodes(src_node: *mut icc_node, dst_node: *mut *mut icc_node) -> i32;
    pub fn icc_link_create(node: *mut icc_node, dst_id: i32) -> i32;
    pub fn icc_node_add(node: *mut icc_node, provider: *mut icc_provider);
    pub fn icc_node_del(node: *mut icc_node);
    pub fn icc_nodes_remove(provider: *mut icc_provider) -> i32;
    pub fn icc_provider_init(provider: *mut icc_provider);
    pub fn icc_provider_register(provider: *mut icc_provider) -> i32;
    pub fn icc_provider_deregister(provider: *mut icc_provider);
    pub fn of_icc_get_from_provider(spec: *const of_phandle_args) -> *mut icc_node_data;
    pub fn icc_sync_state(dev: *mut device);
}

// CONFIG_INTERCONNECT disabled equivalents. Error constants are supplied by dependencies.
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
#[inline]
pub unsafe fn icc_std_aggregate(_node: *mut icc_node, _tag: u32, _avg_bw: u32, _peak_bw: u32, _agg_avg: *mut u32, _agg_peak: *mut u32) -> i32 { -ENOTSUPP }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
#[inline]
pub unsafe fn icc_node_create_dyn() -> *mut icc_node { ERR_PTR(-EOPNOTSUPP) }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
#[inline]
pub unsafe fn icc_node_create(_id: i32) -> *mut icc_node { ERR_PTR(-EOPNOTSUPP) }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
#[inline]
pub unsafe fn icc_node_destroy(_id: i32) {}
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
#[inline]
pub unsafe fn icc_node_set_name(_node: *mut icc_node, _provider: *const icc_provider, _name: *const ::core::ffi::c_char) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
#[inline]
pub unsafe fn icc_link_nodes(_src_node: *mut icc_node, _dst_node: *mut *mut icc_node) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
#[inline]
pub unsafe fn icc_link_create(_node: *mut icc_node, _dst_id: i32) -> i32 { -ENOTSUPP }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
#[inline]
pub unsafe fn icc_node_add(_node: *mut icc_node, _provider: *mut icc_provider) {}
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
#[inline]
pub unsafe fn icc_node_del(_node: *mut icc_node) {}
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
#[inline]
pub unsafe fn icc_nodes_remove(_provider: *mut icc_provider) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
#[inline]
pub unsafe fn icc_provider_init(_provider: *mut icc_provider) {}
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
#[inline]
pub unsafe fn icc_provider_register(_provider: *mut icc_provider) -> i32 { -ENOTSUPP }
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
#[inline]
pub unsafe fn icc_provider_deregister(_provider: *mut icc_provider) {}
#[cfg(not(feature = "CONFIG_INTERCONNECT"))]
#[inline]
pub unsafe fn of_icc_get_from_provider(_spec: *const of_phandle_args) -> *mut icc_node_data { ERR_PTR(-ENOTSUPP) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
