/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the Linux QRTR trace-event header.
// The original includes (<linux/qrtr.h>, <linux/tracepoint.h>, and
// <trace/define_trace.h>) provide external tracepoint infrastructure.

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct QrtrNsServiceEntry {
    pub service: ::core::ffi::c_uint,
    pub instance: ::core::ffi::c_uint,
    pub node: ::core::ffi::c_uint,
    pub port: ::core::ffi::c_uint,
}

impl QrtrNsServiceEntry {
    #[inline]
    pub const unsafe fn assign(
        service: ::core::ffi::c_uint,
        instance: ::core::ffi::c_uint,
        node: ::core::ffi::c_uint,
        port: ::core::ffi::c_uint,
    ) -> Self {
        Self {
            service,
            instance,
            node,
            port,
        }
    }
}

pub const QRTR_NS_SERVICE_ANNOUNCE_NEW: &str =
    "advertising new server [%d:%x]@[%d:%d]";
pub const QRTR_NS_SERVICE_ANNOUNCE_DEL: &str =
    "advertising removal of server [%d:%x]@[%d:%d]";
pub const QRTR_NS_SERVER_ADD: &str = "add server [%d:%x]@[%d:%d]";

#[inline]
pub fn qrtr_ns_service_announce_new_print(entry: &QrtrNsServiceEntry) -> String {
    format!(
        "advertising new server [{}:{:x}]@[{}:{}]",
        entry.service, entry.instance, entry.node, entry.port
    )
}

#[inline]
pub fn qrtr_ns_service_announce_del_print(entry: &QrtrNsServiceEntry) -> String {
    format!(
        "advertising removal of server [{}:{:x}]@[{}:{}]",
        entry.service, entry.instance, entry.node, entry.port
    )
}

#[inline]
pub fn qrtr_ns_server_add_print(entry: &QrtrNsServiceEntry) -> String {
    format!(
        "add server [{}:{:x}]@[{}:{}]",
        entry.service, entry.instance, entry.node, entry.port
    )
}

#[repr(C)]
#[derive(Debug)]
pub struct QrtrNsMessageEntry {
    pub ctrl_pkt_str: *mut ::core::ffi::c_char,
    pub sq_node: u32,
    pub sq_port: u32,
}

impl QrtrNsMessageEntry {
    #[inline]
    pub const unsafe fn assign(
        ctrl_pkt_str: *mut ::core::ffi::c_char,
        sq_node: u32,
        sq_port: u32,
    ) -> Self {
        Self {
            ctrl_pkt_str,
            sq_node,
            sq_port,
        }
    }

    #[inline]
    pub unsafe fn print(&self) -> String {
        let ctrl_pkt_str = ::core::ffi::CStr::from_ptr(self.ctrl_pkt_str).to_string_lossy();
        format!("{} from {}:{}", ctrl_pkt_str, self.sq_node, self.sq_port)
    }
}

pub const QRTR_NS_MESSAGE: &str = "%s from %d:%d";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
