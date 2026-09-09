/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by other translation units.
use core::ffi::c_void;

#[repr(C)]
pub struct component_master_ops;
#[repr(C)]
pub struct component_match;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct drm_device;
#[repr(C)]
pub struct drm_encoder;
#[repr(C)]
pub struct drm_panel;
#[repr(C)]
pub struct drm_bridge;
#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct mipi_dsi_device_info;
#[repr(C)]
pub struct mipi_dsi_host;
#[repr(C)]
pub struct of_endpoint {
    pub port: i32,
    pub id: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panel_orientation {
    _Unused = 0,
}

/**
 * enum drm_lvds_dual_link_pixels - Pixel order of an LVDS dual-link connection
 * @DRM_LVDS_DUAL_LINK_EVEN_ODD_PIXELS: Even pixels are expected to be generated
 *    from the first port, odd pixels from the second port
 * @DRM_LVDS_DUAL_LINK_ODD_EVEN_PIXELS: Odd pixels are expected to be generated
 *    from the first port, even pixels from the second port
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_lvds_dual_link_pixels {
    DRM_LVDS_DUAL_LINK_EVEN_ODD_PIXELS = 0,
    DRM_LVDS_DUAL_LINK_ODD_EVEN_PIXELS = 1,
}

// Under CONFIG_OF these are external functions; otherwise the inline fallbacks apply.
#[cfg(feature = "CONFIG_OF")]
extern "C" {
    pub fn drm_of_crtc_port_mask(dev: *mut drm_device, port: *mut device_node) -> u32;
    pub fn drm_of_find_possible_crtcs(dev: *mut drm_device, port: *mut device_node) -> u32;
    pub fn drm_of_component_match_add(master: *mut device, matchptr: *mut *mut component_match,
        compare: Option<unsafe extern "C" fn(*mut device, *mut c_void) -> i32>, node: *mut device_node);
    pub fn drm_of_component_probe(dev: *mut device,
        compare_of: Option<unsafe extern "C" fn(*mut device, *mut c_void) -> i32>,
        m_ops: *const component_master_ops) -> i32;
    pub fn drm_of_encoder_active_endpoint(node: *mut device_node, encoder: *mut drm_encoder,
        endpoint: *mut of_endpoint) -> i32;
    pub fn drm_of_get_panel_orientation(np: *const device_node,
        orientation: *mut drm_panel_orientation) -> i32;
    pub fn drm_of_find_panel_or_bridge(np: *const device_node, port: i32, endpoint: i32,
        panel: *mut *mut drm_panel, bridge: *mut *mut drm_bridge) -> i32;
    pub fn drm_of_lvds_get_dual_link_pixel_order(port1: *const device_node, port2: *const device_node) -> i32;
    pub fn drm_of_lvds_get_dual_link_pixel_order_sink(port1: *mut device_node, port2: *mut device_node) -> i32;
    pub fn drm_of_lvds_get_data_mapping(port: *const device_node) -> i32;
    pub fn drm_of_get_data_lanes_count(endpoint: *const device_node, min: u32, max: u32) -> i32;
    pub fn drm_of_get_data_lanes_count_ep(port: *const device_node, port_reg: i32, reg: i32, min: u32, max: u32) -> i32;
    pub fn drm_of_get_data_lanes_count_remote(port: *const device_node, port_reg: i32, reg: i32, min: u32, max: u32) -> i32;
}

#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn drm_of_crtc_port_mask(_dev: *mut drm_device, _port: *mut device_node) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn drm_of_find_possible_crtcs(_dev: *mut drm_device, _port: *mut device_node) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn drm_of_component_match_add(_master: *mut device, _matchptr: *mut *mut component_match,
    _compare: Option<unsafe extern "C" fn(*mut device, *mut c_void) -> i32>, _node: *mut device_node) {}
#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn drm_of_component_probe(_dev: *mut device,
    _compare_of: Option<unsafe extern "C" fn(*mut device, *mut c_void) -> i32>,
    _m_ops: *const component_master_ops) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn drm_of_encoder_active_endpoint(_node: *mut device_node, _encoder: *mut drm_encoder, _endpoint: *mut of_endpoint) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn drm_of_get_panel_orientation(_np: *const device_node, _orientation: *mut drm_panel_orientation) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn drm_of_find_panel_or_bridge(_np: *const device_node, _port: i32, _endpoint: i32, _panel: *mut *mut drm_panel, _bridge: *mut *mut drm_bridge) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn drm_of_lvds_get_dual_link_pixel_order(_port1: *const device_node, _port2: *const device_node) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn drm_of_lvds_get_dual_link_pixel_order_sink(_port1: *mut device_node, _port2: *mut device_node) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn drm_of_lvds_get_data_mapping(_port: *const device_node) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn drm_of_get_data_lanes_count(_endpoint: *const device_node, _min: u32, _max: u32) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn drm_of_get_data_lanes_count_ep(_port: *const device_node, _port_reg: i32, _reg: i32, _min: u32, _max: u32) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn drm_of_get_data_lanes_count_remote(_port: *const device_node, _port_reg: i32, _reg: i32, _min: u32, _max: u32) -> i32 { -22 }

#[cfg(all(feature = "CONFIG_OF", feature = "CONFIG_DRM_MIPI_DSI"))]
extern "C" { pub fn drm_of_get_dsi_bus(dev: *mut device) -> *mut mipi_dsi_host; }
#[cfg(not(all(feature = "CONFIG_OF", feature = "CONFIG_DRM_MIPI_DSI")))]
pub unsafe fn drm_of_get_dsi_bus(_dev: *mut device) -> *mut mipi_dsi_host { core::ptr::without_provenance_mut((-22isize) as usize) }

// CONFIG_OF && CONFIG_DRM_PANEL_BRIDGE supplies these external functions.
extern "C" {
    fn of_graph_get_remote_node(np: *const device_node, port: i32, endpoint: i32) -> *mut device_node;
    fn of_drm_find_and_get_bridge(np: *mut device_node) -> *mut drm_bridge;
    fn drm_panel_bridge_remove(bridge: *mut drm_bridge);
    fn drm_bridge_put(bridge: *mut drm_bridge);
    fn of_node_put(node: *mut device_node);
}

pub unsafe fn drm_of_panel_bridge_remove(np: *const device_node, port: i32, endpoint: i32) -> i32 {
    // The CONFIG_OF && CONFIG_DRM_PANEL_BRIDGE branch is preserved by these calls.
    let remote = of_graph_get_remote_node(np, port, endpoint);
    if remote.is_null() { return -19; }
    let bridge = of_drm_find_and_get_bridge(remote);
    drm_panel_bridge_remove(bridge);
    drm_bridge_put(bridge);
    of_node_put(remote);
    0
}

pub unsafe fn drm_of_encoder_active_endpoint_id(node: *mut device_node, encoder: *mut drm_encoder) -> i32 {
    let mut endpoint = of_endpoint { port: 0, id: 0 };
    let ret = drm_of_encoder_active_endpoint(node, encoder, &mut endpoint);
    if ret != 0 { ret } else { endpoint.id }
}

pub unsafe fn drm_of_encoder_active_port_id(node: *mut device_node, encoder: *mut drm_encoder) -> i32 {
    let mut endpoint = of_endpoint { port: 0, id: 0 };
    let ret = drm_of_encoder_active_endpoint(node, encoder, &mut endpoint);
    if ret != 0 { ret } else { endpoint.port }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
