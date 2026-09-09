/* SPDX-License-Identifier: GPL-2.0 */
/*
 * OF graph binding parsing helpers
 *
 * Translated from the corresponding C header. External types and functions
 * supplied by other headers are intentionally left as dependencies.
 */

#[repr(C)]
pub struct of_endpoint {
    pub port: ::core::ffi::c_uint,
    pub id: ::core::ffi::c_uint,
    pub local_node: *const device_node,
}

/* Iterate over every endpoint in a device node. On loop break, the current
 * node must be released manually, as in the original C macro. */
#[macro_export]
macro_rules! for_each_endpoint_of_node {
    ($parent:expr, $child:ident) => {
        for $child in unsafe { of_graph_get_next_endpoint($parent, core::ptr::null()) };
             !$child.is_null();
             $child = unsafe { of_graph_get_next_endpoint($parent, $child) }
        {}
    };
}

/* Iterate over every port in a device or ports node. The C __free cleanup
 * behavior is represented only by this loop's ownership-preserving pointer. */
#[macro_export]
macro_rules! for_each_of_graph_port {
    ($parent:expr, $child:ident) => {
        for $child = unsafe { of_graph_get_next_port($parent, core::ptr::null()) };
             !$child.is_null();
             $child = unsafe { of_graph_get_next_port($parent, $child) }
        {}
    };
}

/* Iterate over every endpoint in a port node. */
#[macro_export]
macro_rules! for_each_of_graph_port_endpoint {
    ($parent:expr, $child:ident) => {
        for $child = unsafe { of_graph_get_next_port_endpoint($parent, core::ptr::null()) };
             !$child.is_null();
             $child = unsafe { of_graph_get_next_port_endpoint($parent, $child) }
        {}
    };
}

#[cfg(feature = "CONFIG_OF")]
unsafe extern "C" {
    pub fn of_graph_is_present(node: *const device_node) -> bool;
    pub fn of_graph_parse_endpoint(
        node: *const device_node,
        endpoint: *mut of_endpoint,
    ) -> ::core::ffi::c_int;
    pub fn of_graph_get_endpoint_count(np: *const device_node) -> ::core::ffi::c_uint;
    pub fn of_graph_get_port_count(np: *mut device_node) -> ::core::ffi::c_uint;
    pub fn of_graph_get_port_by_id(node: *mut device_node, id: u32) -> *mut device_node;
    pub fn of_graph_get_next_endpoint(
        parent: *const device_node,
        previous: *mut device_node,
    ) -> *mut device_node;
    pub fn of_graph_get_next_port(
        parent: *const device_node,
        port: *mut device_node,
    ) -> *mut device_node;
    pub fn of_graph_get_next_port_endpoint(
        port: *const device_node,
        prev: *mut device_node,
    ) -> *mut device_node;
    pub fn of_graph_get_endpoint_by_regs(
        parent: *const device_node,
        port_reg: ::core::ffi::c_int,
        reg: ::core::ffi::c_int,
    ) -> *mut device_node;
    pub fn of_graph_get_remote_endpoint(node: *const device_node) -> *mut device_node;
    pub fn of_graph_get_port_parent(node: *mut device_node) -> *mut device_node;
    pub fn of_graph_get_remote_port_parent(node: *const device_node) -> *mut device_node;
    pub fn of_graph_get_remote_port(node: *const device_node) -> *mut device_node;
    pub fn of_graph_get_remote_node(
        node: *const device_node,
        port: u32,
        endpoint: u32,
    ) -> *mut device_node;
}

#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn of_graph_is_present(_node: *const device_node) -> bool { false }

#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn of_graph_parse_endpoint(
    _node: *const device_node,
    _endpoint: *mut of_endpoint,
) -> ::core::ffi::c_int { -ENOSYS }

#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn of_graph_get_endpoint_count(_np: *const device_node) -> ::core::ffi::c_uint { 0 }

#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn of_graph_get_port_count(_np: *mut device_node) -> ::core::ffi::c_uint { 0 }

#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn of_graph_get_port_by_id(_node: *mut device_node, _id: u32) -> *mut device_node { core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn of_graph_get_next_endpoint(_parent: *const device_node, _previous: *mut device_node) -> *mut device_node { core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn of_graph_get_next_port(_parent: *const device_node, _previous: *mut device_node) -> *mut device_node { core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn of_graph_get_next_port_endpoint(_parent: *const device_node, _previous: *mut device_node) -> *mut device_node { core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn of_graph_get_endpoint_by_regs(_parent: *const device_node, _port_reg: ::core::ffi::c_int, _reg: ::core::ffi::c_int) -> *mut device_node { core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn of_graph_get_remote_endpoint(_node: *const device_node) -> *mut device_node { core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn of_graph_get_port_parent(_node: *mut device_node) -> *mut device_node { core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn of_graph_get_remote_port_parent(_node: *const device_node) -> *mut device_node { core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn of_graph_get_remote_port(_node: *const device_node) -> *mut device_node { core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_OF"))]
pub unsafe fn of_graph_get_remote_node(_node: *const device_node, _port: u32, _endpoint: u32) -> *mut device_node { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
