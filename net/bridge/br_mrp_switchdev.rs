// SPDX-License-Identifier: GPL-2.0-or-later

// Dependencies supplied by the kernel switchdev and bridge-private headers.

use core::ffi::c_int;

extern "C" {
    fn switchdev_port_obj_add(dev: *mut net_device, obj: *mut switchdev_obj, extack: *mut core::ffi::c_void) -> c_int;
    fn switchdev_port_obj_del(dev: *mut net_device, obj: *mut switchdev_obj) -> c_int;
    fn switchdev_port_attr_set(dev: *mut net_device, attr: *mut switchdev_attr, extack: *mut core::ffi::c_void) -> c_int;
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_bridge {
    pub dev: *mut net_device,
}

#[repr(C)]
pub struct net_bridge_port {
    pub dev: *mut net_device,
}

#[repr(C)]
pub struct br_mrp {
    pub p_port: *mut net_bridge_port,
    pub s_port: *mut net_bridge_port,
    pub i_port: *mut net_bridge_port,
    pub ring_id: u32,
    pub prio: u32,
    pub in_id: u16,
}

#[repr(C)]
pub struct switchdev_obj {
    pub orig_dev: *mut net_device,
    pub id: u32,
}

#[repr(C)]
pub struct switchdev_obj_mrp {
    pub obj: switchdev_obj,
    pub p_port: *mut net_device,
    pub s_port: *mut net_device,
    pub ring_id: u32,
    pub prio: u32,
}

#[repr(C)]
pub struct switchdev_obj_ring_role_mrp {
    pub obj: switchdev_obj,
    pub ring_role: br_mrp_ring_role_type,
    pub ring_id: u32,
    pub sw_backup: bool,
}

#[repr(C)]
pub struct switchdev_obj_ring_test_mrp {
    pub obj: switchdev_obj,
    pub interval: u32,
    pub max_miss: u8,
    pub ring_id: u32,
    pub period: u32,
    pub monitor: bool,
}

#[repr(C)]
pub struct switchdev_obj_ring_state_mrp {
    pub obj: switchdev_obj,
    pub ring_state: br_mrp_ring_state_type,
    pub ring_id: u32,
}

#[repr(C)]
pub struct switchdev_obj_in_role_mrp {
    pub obj: switchdev_obj,
    pub in_role: br_mrp_in_role_type,
    pub in_id: u16,
    pub ring_id: u32,
    pub i_port: *mut net_device,
    pub sw_backup: bool,
}

#[repr(C)]
pub struct switchdev_obj_in_state_mrp {
    pub obj: switchdev_obj,
    pub in_state: br_mrp_in_state_type,
    pub in_id: u16,
}

#[repr(C)]
pub struct switchdev_obj_in_test_mrp {
    pub obj: switchdev_obj,
    pub interval: u32,
    pub max_miss: u8,
    pub in_id: u16,
    pub period: u32,
}

#[repr(C)]
pub union switchdev_attr_u {
    pub stp_state: u32,
    pub mrp_port_role: br_mrp_port_role_type,
}

#[repr(C)]
pub struct switchdev_attr {
    pub orig_dev: *mut net_device,
    pub id: u32,
    pub u: switchdev_attr_u,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_hw_support { BR_MRP_NONE = 0, BR_MRP_HW = 1, BR_MRP_SW = 2 }

pub type br_mrp_ring_role_type = u32;
pub type br_mrp_ring_state_type = u32;
pub type br_mrp_in_role_type = u32;
pub type br_mrp_in_state_type = u32;
pub type br_mrp_port_role_type = u32;

pub const SWITCHDEV_OBJ_ID_MRP: u32 = 0;
pub const SWITCHDEV_OBJ_ID_RING_ROLE_MRP: u32 = 1;
pub const SWITCHDEV_OBJ_ID_RING_TEST_MRP: u32 = 2;
pub const SWITCHDEV_OBJ_ID_RING_STATE_MRP: u32 = 3;
pub const SWITCHDEV_OBJ_ID_IN_ROLE_MRP: u32 = 4;
pub const SWITCHDEV_OBJ_ID_IN_STATE_MRP: u32 = 5;
pub const SWITCHDEV_OBJ_ID_IN_TEST_MRP: u32 = 6;
pub const SWITCHDEV_ATTR_ID_PORT_STP_STATE: u32 = 0;
pub const SWITCHDEV_ATTR_ID_MRP_PORT_ROLE: u32 = 1;
pub const BR_MRP_RING_ROLE_DISABLED: br_mrp_ring_role_type = 0;
pub const BR_MRP_IN_ROLE_DISABLED: br_mrp_in_role_type = 0;
pub const EOPNOTSUPP: c_int = 95;

unsafe fn br_mrp_switchdev_port_obj(br: *mut net_bridge, obj: *mut switchdev_obj, add: bool) -> br_mrp_hw_support {
    let err = if add {
        switchdev_port_obj_add((*br).dev, obj, core::ptr::null_mut())
    } else {
        switchdev_port_obj_del((*br).dev, obj)
    };
    if err == 0 { return br_mrp_hw_support::BR_MRP_HW; }
    if err != -EOPNOTSUPP { return br_mrp_hw_support::BR_MRP_NONE; }
    br_mrp_hw_support::BR_MRP_SW
}

pub unsafe fn br_mrp_switchdev_add(br: *mut net_bridge, mrp: *mut br_mrp) -> c_int {
    let mrp_obj = switchdev_obj_mrp { obj: switchdev_obj { orig_dev: (*br).dev, id: SWITCHDEV_OBJ_ID_MRP }, p_port: (*(*mrp).p_port).dev, s_port: (*(*mrp).s_port).dev, ring_id: (*mrp).ring_id, prio: (*mrp).prio };
    switchdev_port_obj_add((*br).dev, &mrp_obj.obj as *const _ as *mut _, core::ptr::null_mut())
}

pub unsafe fn br_mrp_switchdev_del(br: *mut net_bridge, mrp: *mut br_mrp) -> c_int {
    let mrp_obj = switchdev_obj_mrp { obj: switchdev_obj { orig_dev: (*br).dev, id: SWITCHDEV_OBJ_ID_MRP }, p_port: core::ptr::null_mut(), s_port: core::ptr::null_mut(), ring_id: (*mrp).ring_id, prio: 0 };
    switchdev_port_obj_del((*br).dev, &mrp_obj.obj as *const _ as *mut _)
}

pub unsafe fn br_mrp_switchdev_set_ring_role(br: *mut net_bridge, mrp: *mut br_mrp, role: br_mrp_ring_role_type) -> br_mrp_hw_support {
    let mut mrp_role = switchdev_obj_ring_role_mrp { obj: switchdev_obj { orig_dev: (*br).dev, id: SWITCHDEV_OBJ_ID_RING_ROLE_MRP }, ring_role: role, ring_id: (*mrp).ring_id, sw_backup: false };
    let support = br_mrp_switchdev_port_obj(br, &mut mrp_role.obj, role != BR_MRP_RING_ROLE_DISABLED);
    if support != br_mrp_hw_support::BR_MRP_SW { return support; }
    mrp_role.sw_backup = true;
    let err = if role != BR_MRP_RING_ROLE_DISABLED { switchdev_port_obj_add((*br).dev, &mut mrp_role.obj, core::ptr::null_mut()) } else { switchdev_port_obj_del((*br).dev, &mut mrp_role.obj) };
    if err == 0 { br_mrp_hw_support::BR_MRP_SW } else { br_mrp_hw_support::BR_MRP_NONE }
}

pub unsafe fn br_mrp_switchdev_send_ring_test(br: *mut net_bridge, mrp: *mut br_mrp, interval: u32, max_miss: u8, period: u32, monitor: bool) -> br_mrp_hw_support {
    let mut test = switchdev_obj_ring_test_mrp { obj: switchdev_obj { orig_dev: (*br).dev, id: SWITCHDEV_OBJ_ID_RING_TEST_MRP }, interval, max_miss, ring_id: (*mrp).ring_id, period, monitor };
    br_mrp_switchdev_port_obj(br, &mut test.obj, interval != 0)
}

pub unsafe fn br_mrp_switchdev_set_ring_state(br: *mut net_bridge, mrp: *mut br_mrp, state: br_mrp_ring_state_type) -> c_int {
    let mut mrp_state = switchdev_obj_ring_state_mrp { obj: switchdev_obj { orig_dev: (*br).dev, id: SWITCHDEV_OBJ_ID_RING_STATE_MRP }, ring_state: state, ring_id: (*mrp).ring_id };
    switchdev_port_obj_add((*br).dev, &mut mrp_state.obj, core::ptr::null_mut())
}

pub unsafe fn br_mrp_switchdev_set_in_role(br: *mut net_bridge, mrp: *mut br_mrp, _in_id: u16, _ring_id: u32, role: br_mrp_in_role_type) -> br_mrp_hw_support {
    let mut mrp_role = switchdev_obj_in_role_mrp { obj: switchdev_obj { orig_dev: (*br).dev, id: SWITCHDEV_OBJ_ID_IN_ROLE_MRP }, in_role: role, in_id: (*mrp).in_id, ring_id: (*mrp).ring_id, i_port: (*(*mrp).i_port).dev, sw_backup: false };
    let support = br_mrp_switchdev_port_obj(br, &mut mrp_role.obj, role != BR_MRP_IN_ROLE_DISABLED);
    if support != br_mrp_hw_support::BR_MRP_NONE { return support; }
    mrp_role.sw_backup = true;
    let err = if role != BR_MRP_IN_ROLE_DISABLED { switchdev_port_obj_add((*br).dev, &mut mrp_role.obj, core::ptr::null_mut()) } else { switchdev_port_obj_del((*br).dev, &mut mrp_role.obj) };
    if err == 0 { br_mrp_hw_support::BR_MRP_SW } else { br_mrp_hw_support::BR_MRP_NONE }
}

pub unsafe fn br_mrp_switchdev_set_in_state(br: *mut net_bridge, mrp: *mut br_mrp, state: br_mrp_in_state_type) -> c_int {
    let mut mrp_state = switchdev_obj_in_state_mrp { obj: switchdev_obj { orig_dev: (*br).dev, id: SWITCHDEV_OBJ_ID_IN_STATE_MRP }, in_state: state, in_id: (*mrp).in_id };
    switchdev_port_obj_add((*br).dev, &mut mrp_state.obj, core::ptr::null_mut())
}

pub unsafe fn br_mrp_switchdev_send_in_test(br: *mut net_bridge, mrp: *mut br_mrp, interval: u32, max_miss: u8, period: u32) -> br_mrp_hw_support {
    let mut test = switchdev_obj_in_test_mrp { obj: switchdev_obj { orig_dev: (*br).dev, id: SWITCHDEV_OBJ_ID_IN_TEST_MRP }, interval, max_miss, in_id: (*mrp).in_id, period };
    br_mrp_switchdev_port_obj(br, &mut test.obj, interval != 0)
}

pub unsafe fn br_mrp_port_switchdev_set_state(p: *mut net_bridge_port, state: u32) -> c_int {
    let attr = switchdev_attr { orig_dev: (*p).dev, id: SWITCHDEV_ATTR_ID_PORT_STP_STATE, u: switchdev_attr_u { stp_state: state } };
    switchdev_port_attr_set((*p).dev, &attr as *const _ as *mut _, core::ptr::null_mut())
}

pub unsafe fn br_mrp_port_switchdev_set_role(p: *mut net_bridge_port, role: br_mrp_port_role_type) -> c_int {
    let attr = switchdev_attr { orig_dev: (*p).dev, id: SWITCHDEV_ATTR_ID_MRP_PORT_ROLE, u: switchdev_attr_u { mrp_port_role: role } };
    switchdev_port_attr_set((*p).dev, &attr as *const _ as *mut _, core::ptr::null_mut())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
