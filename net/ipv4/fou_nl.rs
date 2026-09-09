// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
/* Do not edit directly, auto-generated from: */
/* Documentation/netlink/specs/fou.yaml */
/* YNL-GEN kernel source */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

// C dependencies: <net/netlink.h>, <net/genetlink.h>, "fou_nl.h",
// and <uapi/linux/fou.h>.

#[repr(C)]
pub struct nla_policy {
    pub type_: u16,
}

#[repr(C)]
pub struct genl_small_ops {
    pub cmd: u8,
    pub validate: u32,
    pub doit: Option<unsafe extern "C" fn()>,
    pub dumpit: Option<unsafe extern "C" fn()>,
    pub flags: u32,
}

// External declarations supplied by fou_nl.h and the kernel headers.
unsafe extern "C" {
    fn fou_nl_add_doit();
    fn fou_nl_del_doit();
    fn fou_nl_get_doit();
    fn fou_nl_get_dumpit();
}

// Global operation policy for fou
pub static fou_nl_policy: [nla_policy; FOU_ATTR_IFINDEX as usize + 1] = [
    /* FOU_ATTR_UNSPEC */ nla_policy { type_: 0 },
    /* FOU_ATTR_PORT */ nla_policy { type_: NLA_BE16 },
    /* FOU_ATTR_AF */ nla_policy { type_: NLA_U8 },
    /* FOU_ATTR_IPPROTO: NLA_POLICY_MIN(NLA_U8, 1) */ nla_policy { type_: NLA_U8 },
    /* FOU_ATTR_TYPE */ nla_policy { type_: NLA_U8 },
    /* FOU_ATTR_REMCSUM_NOPARTIAL */ nla_policy { type_: NLA_FLAG },
    /* FOU_ATTR_LOCAL_V4 */ nla_policy { type_: NLA_U32 },
    /* FOU_ATTR_LOCAL_V6: NLA_POLICY_EXACT_LEN(16) */ nla_policy { type_: 0 },
    /* FOU_ATTR_PEER_V4 */ nla_policy { type_: NLA_U32 },
    /* FOU_ATTR_PEER_V6: NLA_POLICY_EXACT_LEN(16) */ nla_policy { type_: 0 },
    /* FOU_ATTR_PEER_PORT */ nla_policy { type_: NLA_BE16 },
    /* FOU_ATTR_IFINDEX */ nla_policy { type_: NLA_S32 },
];

// Ops table for fou
pub static fou_nl_ops: [genl_small_ops; 3] = [
    genl_small_ops {
        cmd: FOU_CMD_ADD,
        validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP,
        doit: Some(fou_nl_add_doit),
        dumpit: None,
        flags: GENL_ADMIN_PERM,
    },
    genl_small_ops {
        cmd: FOU_CMD_DEL,
        validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP,
        doit: Some(fou_nl_del_doit),
        dumpit: None,
        flags: GENL_ADMIN_PERM,
    },
    genl_small_ops {
        cmd: FOU_CMD_GET,
        validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP,
        doit: Some(fou_nl_get_doit),
        dumpit: Some(fou_nl_get_dumpit),
        flags: 0,
    },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
