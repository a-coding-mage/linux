/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* ila.h - ILA Interface */

/* NETLINK_GENERIC related info */
pub const ILA_GENL_NAME: &str = "ila";
pub const ILA_GENL_VERSION: u32 = 0x1;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IlaAttr {
    Unspec = 0,
    Locator,
    Identifier,
    LocatorMatch,
    Ifindex,
    Dir,
    Pad,
    CsumMode,
    IdentType,
    HookType,
    Max,
}

pub const ILA_ATTR_UNSPEC: u32 = IlaAttr::Unspec as u32;
pub const ILA_ATTR_LOCATOR: u32 = IlaAttr::Locator as u32; /* u64 */
pub const ILA_ATTR_IDENTIFIER: u32 = IlaAttr::Identifier as u32; /* u64 */
pub const ILA_ATTR_LOCATOR_MATCH: u32 = IlaAttr::LocatorMatch as u32; /* u64 */
pub const ILA_ATTR_IFINDEX: u32 = IlaAttr::Ifindex as u32; /* s32 */
pub const ILA_ATTR_DIR: u32 = IlaAttr::Dir as u32; /* u32 */
pub const ILA_ATTR_PAD: u32 = IlaAttr::Pad as u32;
pub const ILA_ATTR_CSUM_MODE: u32 = IlaAttr::CsumMode as u32; /* u8 */
pub const ILA_ATTR_IDENT_TYPE: u32 = IlaAttr::IdentType as u32; /* u8 */
pub const ILA_ATTR_HOOK_TYPE: u32 = IlaAttr::HookType as u32; /* u8 */
pub const __ILA_ATTR_MAX: u32 = IlaAttr::Max as u32;
pub const ILA_ATTR_MAX: u32 = __ILA_ATTR_MAX - 1;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IlaCmd {
    Unspec = 0,
    Add,
    Del,
    Get,
    Flush,
    Max,
}

pub const ILA_CMD_UNSPEC: u32 = IlaCmd::Unspec as u32;
pub const ILA_CMD_ADD: u32 = IlaCmd::Add as u32;
pub const ILA_CMD_DEL: u32 = IlaCmd::Del as u32;
pub const ILA_CMD_GET: u32 = IlaCmd::Get as u32;
pub const ILA_CMD_FLUSH: u32 = IlaCmd::Flush as u32;
pub const __ILA_CMD_MAX: u32 = IlaCmd::Max as u32;
pub const ILA_CMD_MAX: u32 = __ILA_CMD_MAX - 1;

pub const ILA_DIR_IN: u32 = 1 << 0;
pub const ILA_DIR_OUT: u32 = 1 << 1;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IlaCsumMode {
    AdjustTransport = 0,
    NeutralMap,
    NoAction,
    NeutralMapAuto,
}

pub const ILA_CSUM_ADJUST_TRANSPORT: u32 = IlaCsumMode::AdjustTransport as u32;
pub const ILA_CSUM_NEUTRAL_MAP: u32 = IlaCsumMode::NeutralMap as u32;
pub const ILA_CSUM_NO_ACTION: u32 = IlaCsumMode::NoAction as u32;
pub const ILA_CSUM_NEUTRAL_MAP_AUTO: u32 = IlaCsumMode::NeutralMapAuto as u32;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IlaAtype {
    Iid = 0,
    Luid,
    VirtV4,
    VirtUniV6,
    VirtMultiV6,
    NonlocalAddr,
    Rsvd1,
    Rsvd2,
    UseFormat = 32, /* Get type from type field in identifier */
}

pub const ILA_ATYPE_IID: u32 = IlaAtype::Iid as u32;
pub const ILA_ATYPE_LUID: u32 = IlaAtype::Luid as u32;
pub const ILA_ATYPE_VIRT_V4: u32 = IlaAtype::VirtV4 as u32;
pub const ILA_ATYPE_VIRT_UNI_V6: u32 = IlaAtype::VirtUniV6 as u32;
pub const ILA_ATYPE_VIRT_MULTI_V6: u32 = IlaAtype::VirtMultiV6 as u32;
pub const ILA_ATYPE_NONLOCAL_ADDR: u32 = IlaAtype::NonlocalAddr as u32;
pub const ILA_ATYPE_RSVD_1: u32 = IlaAtype::Rsvd1 as u32;
pub const ILA_ATYPE_RSVD_2: u32 = IlaAtype::Rsvd2 as u32;
pub const ILA_ATYPE_USE_FORMAT: u32 = IlaAtype::UseFormat as u32;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IlaHookType {
    RouteOutput = 0,
    RouteInput,
}

pub const ILA_HOOK_ROUTE_OUTPUT: u32 = IlaHookType::RouteOutput as u32;
pub const ILA_HOOK_ROUTE_INPUT: u32 = IlaHookType::RouteInput as u32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
