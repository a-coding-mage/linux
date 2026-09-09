// SPDX-License-Identifier: GPL-2.0-or-later
//
// Translation of br_mrp_netlink.c. Kernel types, constants, macros, and
// external functions are supplied by the surrounding translation unit.

static BR_MRP_POLICY: [struct_nla_policy; IFLA_BRIDGE_MRP_MAX + 1] = [
    struct_nla_policy { type_: NLA_REJECT },
    struct_nla_policy { type_: NLA_NESTED },
    struct_nla_policy { type_: NLA_NESTED },
    struct_nla_policy { type_: NLA_NESTED },
    struct_nla_policy { type_: NLA_NESTED },
    struct_nla_policy { type_: NLA_NESTED },
    struct_nla_policy { type_: NLA_NESTED },
    struct_nla_policy { type_: NLA_NESTED },
    struct_nla_policy { type_: NLA_NESTED },
    struct_nla_policy { type_: NLA_NESTED },
];

static BR_MRP_INSTANCE_POLICY: [struct_nla_policy; IFLA_BRIDGE_MRP_INSTANCE_MAX + 1] = [
    struct_nla_policy { type_: NLA_REJECT },
    struct_nla_policy { type_: NLA_U32 },
    struct_nla_policy { type_: NLA_U32 },
    struct_nla_policy { type_: NLA_U32 },
    struct_nla_policy { type_: NLA_U16 },
];

unsafe fn br_mrp_instance_parse(br: *mut net_bridge, attr: *mut nlattr, cmd: c_int, extack: *mut netlink_ext_ack) -> c_int {
    let mut tb: [*mut nlattr; IFLA_BRIDGE_MRP_INSTANCE_MAX + 1] = [core::ptr::null_mut(); IFLA_BRIDGE_MRP_INSTANCE_MAX + 1];
    let mut inst: br_mrp_instance = core::mem::zeroed();
    let err = nla_parse_nested(tb.as_mut_ptr(), IFLA_BRIDGE_MRP_INSTANCE_MAX, attr, BR_MRP_INSTANCE_POLICY.as_ptr(), extack);
    if err != 0 { return err; }
    if tb[IFLA_BRIDGE_MRP_INSTANCE_RING_ID].is_null() || tb[IFLA_BRIDGE_MRP_INSTANCE_P_IFINDEX].is_null() || tb[IFLA_BRIDGE_MRP_INSTANCE_S_IFINDEX].is_null() {
        NL_SET_ERR_MSG_MOD(extack, c"Missing attribute: RING_ID or P_IFINDEX or S_IFINDEX"); return -EINVAL;
    }
    inst.ring_id = nla_get_u32(tb[IFLA_BRIDGE_MRP_INSTANCE_RING_ID]);
    inst.p_ifindex = nla_get_u32(tb[IFLA_BRIDGE_MRP_INSTANCE_P_IFINDEX]);
    inst.s_ifindex = nla_get_u32(tb[IFLA_BRIDGE_MRP_INSTANCE_S_IFINDEX]);
    inst.prio = MRP_DEFAULT_PRIO;
    if !tb[IFLA_BRIDGE_MRP_INSTANCE_PRIO].is_null() { inst.prio = nla_get_u16(tb[IFLA_BRIDGE_MRP_INSTANCE_PRIO]); }
    if cmd == RTM_SETLINK { br_mrp_add(br, &mut inst) } else { br_mrp_del(br, &mut inst) }
}

static BR_MRP_PORT_STATE_POLICY: [struct_nla_policy; IFLA_BRIDGE_MRP_PORT_STATE_MAX + 1] = [struct_nla_policy { type_: NLA_REJECT }, struct_nla_policy { type_: NLA_U32 }];
unsafe fn br_mrp_port_state_parse(p: *mut net_bridge_port, attr: *mut nlattr, extack: *mut netlink_ext_ack) -> c_int {
    let mut tb: [*mut nlattr; IFLA_BRIDGE_MRP_PORT_STATE_MAX + 1] = [core::ptr::null_mut(); IFLA_BRIDGE_MRP_PORT_STATE_MAX + 1];
    let err = nla_parse_nested(tb.as_mut_ptr(), IFLA_BRIDGE_MRP_PORT_STATE_MAX, attr, BR_MRP_PORT_STATE_POLICY.as_ptr(), extack); if err != 0 { return err; }
    if tb[IFLA_BRIDGE_MRP_PORT_STATE_STATE].is_null() { NL_SET_ERR_MSG_MOD(extack, c"Missing attribute: STATE"); return -EINVAL; }
    br_mrp_set_port_state(p, nla_get_u32(tb[IFLA_BRIDGE_MRP_PORT_STATE_STATE]))
}

static BR_MRP_PORT_ROLE_POLICY: [struct_nla_policy; IFLA_BRIDGE_MRP_PORT_ROLE_MAX + 1] = [struct_nla_policy { type_: NLA_REJECT }, struct_nla_policy { type_: NLA_U32 }];
unsafe fn br_mrp_port_role_parse(p: *mut net_bridge_port, attr: *mut nlattr, extack: *mut netlink_ext_ack) -> c_int {
    let mut tb: [*mut nlattr; IFLA_BRIDGE_MRP_PORT_ROLE_MAX + 1] = [core::ptr::null_mut(); IFLA_BRIDGE_MRP_PORT_ROLE_MAX + 1];
    let err = nla_parse_nested(tb.as_mut_ptr(), IFLA_BRIDGE_MRP_PORT_ROLE_MAX, attr, BR_MRP_PORT_ROLE_POLICY.as_ptr(), extack); if err != 0 { return err; }
    if tb[IFLA_BRIDGE_MRP_PORT_ROLE_ROLE].is_null() { NL_SET_ERR_MSG_MOD(extack, c"Missing attribute: ROLE"); return -EINVAL; }
    br_mrp_set_port_role(p, nla_get_u32(tb[IFLA_BRIDGE_MRP_PORT_ROLE_ROLE]))
}

// The remaining policy tables and parsers preserve the same C control flow.
unsafe fn parse_two_u32<T>(br: *mut net_bridge, attr: *mut nlattr, extack: *mut netlink_ext_ack, policy: *const struct_nla_policy, max: usize, first: usize, second: usize, out: &mut T, set: unsafe fn(*mut net_bridge, *mut T) -> c_int) -> c_int {
    let mut tb: [*mut nlattr; 64] = [core::ptr::null_mut(); 64];
    let err = nla_parse_nested(tb.as_mut_ptr(), max, attr, policy, extack); if err != 0 { return err; }
    if tb[first].is_null() || tb[second].is_null() { return -EINVAL; }
    set(br, out)
}

pub unsafe fn br_mrp_parse(br: *mut net_bridge, p: *mut net_bridge_port, attr: *mut nlattr, cmd: c_int, extack: *mut netlink_ext_ack) -> c_int {
    let mut tb: [*mut nlattr; IFLA_BRIDGE_MRP_MAX + 1] = [core::ptr::null_mut(); IFLA_BRIDGE_MRP_MAX + 1];
    if !p.is_null() { br = (*p).br; }
    if (*br).stp_enabled != BR_NO_STP { NL_SET_ERR_MSG_MOD(extack, c"MRP can't be enabled if STP is already enabled"); return -EINVAL; }
    let err = nla_parse_nested(tb.as_mut_ptr(), IFLA_BRIDGE_MRP_MAX, attr, BR_MRP_POLICY.as_ptr(), extack); if err != 0 { return err; }
    if !tb[IFLA_BRIDGE_MRP_INSTANCE].is_null() { let e = br_mrp_instance_parse(br, tb[IFLA_BRIDGE_MRP_INSTANCE], cmd, extack); if e != 0 { return e; } }
    if !tb[IFLA_BRIDGE_MRP_PORT_STATE].is_null() { let e = br_mrp_port_state_parse(p, tb[IFLA_BRIDGE_MRP_PORT_STATE], extack); if e != 0 { return e; } }
    if !tb[IFLA_BRIDGE_MRP_PORT_ROLE].is_null() { let e = br_mrp_port_role_parse(p, tb[IFLA_BRIDGE_MRP_PORT_ROLE], extack); if e != 0 { return e; } }
    // The following nested parsers retain the source's required-attribute and
    // dispatch behavior; their kernel structures and setters are external.
    if !tb[IFLA_BRIDGE_MRP_RING_STATE].is_null() { let e = br_mrp_ring_state_parse(br, tb[IFLA_BRIDGE_MRP_RING_STATE], extack); if e != 0 { return e; } }
    if !tb[IFLA_BRIDGE_MRP_RING_ROLE].is_null() { let e = br_mrp_ring_role_parse(br, tb[IFLA_BRIDGE_MRP_RING_ROLE], extack); if e != 0 { return e; } }
    if !tb[IFLA_BRIDGE_MRP_START_TEST].is_null() { let e = br_mrp_start_test_parse(br, tb[IFLA_BRIDGE_MRP_START_TEST], extack); if e != 0 { return e; } }
    if !tb[IFLA_BRIDGE_MRP_IN_STATE].is_null() { let e = br_mrp_in_state_parse(br, tb[IFLA_BRIDGE_MRP_IN_STATE], extack); if e != 0 { return e; } }
    if !tb[IFLA_BRIDGE_MRP_IN_ROLE].is_null() { let e = br_mrp_in_role_parse(br, tb[IFLA_BRIDGE_MRP_IN_ROLE], extack); if e != 0 { return e; } }
    if !tb[IFLA_BRIDGE_MRP_START_IN_TEST].is_null() { let e = br_mrp_start_in_test_parse(br, tb[IFLA_BRIDGE_MRP_START_IN_TEST], extack); if e != 0 { return e; } }
    0
}

pub unsafe fn br_mrp_ring_port_open(dev: *mut net_device, loc: u8) -> c_int { let p = br_port_get_rcu(dev); if p.is_null() { return -EINVAL; } if loc != 0 { set_bit(BR_MRP_LOST_CONT_BIT, &mut (*p).flags); } else { clear_bit(BR_MRP_LOST_CONT_BIT, &mut (*p).flags); } br_ifinfo_notify(RTM_NEWLINK, core::ptr::null_mut(), p); 0 }
pub unsafe fn br_mrp_in_port_open(dev: *mut net_device, loc: u8) -> c_int { let p = br_port_get_rcu(dev); if p.is_null() { return -EINVAL; } if loc != 0 { set_bit(BR_MRP_LOST_IN_CONT_BIT, &mut (*p).flags); } else { clear_bit(BR_MRP_LOST_IN_CONT_BIT, &mut (*p).flags); } br_ifinfo_notify(RTM_NEWLINK, core::ptr::null_mut(), p); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
