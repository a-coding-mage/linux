/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the Linux kernel headers:
// linux/device.h, linux/fpga/fpga-mgr.h

use core::ffi::{c_char, c_int, c_void};

pub struct device;
pub struct device_node;
pub struct mutex;
pub struct attribute_group;
pub struct fpga_image_info;
pub struct list_head;
pub struct module;

pub struct fpga_bridge;

/**
 * struct fpga_bridge_ops - ops for low level FPGA bridge drivers
 * @enable_show: returns the FPGA bridge's status
 * @enable_set: set an FPGA bridge as enabled or disabled
 * @fpga_bridge_remove: set FPGA into a specific state during driver remove
 * @groups: optional attribute groups.
 */
#[repr(C)]
pub struct fpga_bridge_ops {
    pub enable_show: Option<unsafe extern "C" fn(bridge: *mut fpga_bridge) -> c_int>,
    pub enable_set:
        Option<unsafe extern "C" fn(bridge: *mut fpga_bridge, enable: bool) -> c_int>,
    pub fpga_bridge_remove: Option<unsafe extern "C" fn(bridge: *mut fpga_bridge)>,
    pub groups: *const *const attribute_group,
}

/**
 * struct fpga_bridge_info - collection of parameters an FPGA Bridge
 * @name: fpga bridge name
 * @br_ops: pointer to structure of fpga bridge ops
 * @priv: fpga bridge private data
 *
 * fpga_bridge_info contains parameters for the register function. These
 * are separated into an info structure because they some are optional
 * others could be added to in the future. The info structure facilitates
 * maintaining a stable API.
 */
#[repr(C)]
pub struct fpga_bridge_info {
    pub name: *const c_char,
    pub br_ops: *const fpga_bridge_ops,
    pub priv_: *mut c_void,
}

/**
 * struct fpga_bridge - FPGA bridge structure
 * @name: name of low level FPGA bridge
 * @dev: FPGA bridge device
 * @mutex: enforces exclusive reference to bridge
 * @br_ops: pointer to struct of FPGA bridge ops
 * @br_ops_owner: module containing the br_ops
 * @info: fpga image specific information
 * @node: FPGA bridge list node
 * @priv: low level driver private date
 */
#[repr(C)]
pub struct fpga_bridge {
    pub name: *const c_char,
    pub dev: device,
    pub mutex: mutex, /* for exclusive reference to bridge */
    pub br_ops: *const fpga_bridge_ops,
    pub br_ops_owner: *mut module,
    pub info: *mut fpga_image_info,
    pub node: list_head,
    pub priv_: *mut c_void,
}

// Equivalent of: container_of(d, struct fpga_bridge, dev)
#[macro_export]
macro_rules! to_fpga_bridge {
    ($d:expr) => {
        unsafe {
            &mut *((($d as *mut u8).sub(core::mem::offset_of!($crate::fpga_bridge, dev)))
                as *mut $crate::fpga_bridge)
        }
    };
}

extern "C" {
    pub fn of_fpga_bridge_get(
        node: *mut device_node,
        info: *mut fpga_image_info,
    ) -> *mut fpga_bridge;
    pub fn fpga_bridge_get(
        dev: *mut device,
        info: *mut fpga_image_info,
    ) -> *mut fpga_bridge;
    pub fn fpga_bridge_put(bridge: *mut fpga_bridge);
    pub fn fpga_bridge_enable(bridge: *mut fpga_bridge) -> c_int;
    pub fn fpga_bridge_disable(bridge: *mut fpga_bridge) -> c_int;

    pub fn fpga_bridges_enable(bridge_list: *mut list_head) -> c_int;
    pub fn fpga_bridges_disable(bridge_list: *mut list_head) -> c_int;
    pub fn fpga_bridges_put(bridge_list: *mut list_head);
    pub fn fpga_bridge_get_to_list(
        dev: *mut device,
        info: *mut fpga_image_info,
        bridge_list: *mut list_head,
    ) -> c_int;
    pub fn of_fpga_bridge_get_to_list(
        np: *mut device_node,
        info: *mut fpga_image_info,
        bridge_list: *mut list_head,
    ) -> c_int;

    pub fn __fpga_bridge_register(
        parent: *mut device,
        name: *const c_char,
        br_ops: *const fpga_bridge_ops,
        priv_: *mut c_void,
        owner: *mut module,
    ) -> *mut fpga_bridge;
    pub fn fpga_bridge_unregister(br: *mut fpga_bridge);
}

// The C macro passes THIS_MODULE as the final argument.
#[macro_export]
macro_rules! fpga_bridge_register {
    ($parent:expr, $name:expr, $br_ops:expr, $priv_:expr) => {
        unsafe {
            $crate::__fpga_bridge_register(
                $parent,
                $name,
                $br_ops,
                $priv_,
                $crate::THIS_MODULE,
            )
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
