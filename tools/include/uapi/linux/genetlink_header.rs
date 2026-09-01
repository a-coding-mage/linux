/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependencies from C includes:
 * <linux/types.h>
 * <linux/netlink.h>
 */

pub const GENL_NAMSIZ: usize = 16; /* length of family name */

pub const GENL_MIN_ID: u32 = NLMSG_MIN_TYPE;
pub const GENL_MAX_ID: u32 = 1023;

#[repr(C)]
pub struct genlmsghdr {
    pub cmd: __u8,
    pub version: __u8,
    pub reserved: __u16,
}

pub const GENL_HDRLEN: usize = NLMSG_ALIGN(core::mem::size_of::<genlmsghdr>());

pub const GENL_ADMIN_PERM: u32 = 0x01;
pub const GENL_CMD_CAP_DO: u32 = 0x02;
pub const GENL_CMD_CAP_DUMP: u32 = 0x04;
pub const GENL_CMD_CAP_HASPOL: u32 = 0x08;
pub const GENL_UNS_ADMIN_PERM: u32 = 0x10;

/*
 * List of reserved static generic netlink identifiers:
 */
pub const GENL_ID_CTRL: u32 = NLMSG_MIN_TYPE;
pub const GENL_ID_VFS_DQUOT: u32 = NLMSG_MIN_TYPE + 1;
pub const GENL_ID_PMCRAID: u32 = NLMSG_MIN_TYPE + 2;
/* must be last reserved + 1 */
pub const GENL_START_ALLOC: u32 = NLMSG_MIN_TYPE + 3;

/**************************************************************************
 * Controller
 **************************************************************************/

pub const CTRL_CMD_UNSPEC: u32 = 0;
pub const CTRL_CMD_NEWFAMILY: u32 = 1;
pub const CTRL_CMD_DELFAMILY: u32 = 2;
pub const CTRL_CMD_GETFAMILY: u32 = 3;
pub const CTRL_CMD_NEWOPS: u32 = 4;
pub const CTRL_CMD_DELOPS: u32 = 5;
pub const CTRL_CMD_GETOPS: u32 = 6;
pub const CTRL_CMD_NEWMCAST_GRP: u32 = 7;
pub const CTRL_CMD_DELMCAST_GRP: u32 = 8;
pub const CTRL_CMD_GETMCAST_GRP: u32 = 9; /* unused */
pub const CTRL_CMD_GETPOLICY: u32 = 10;
pub const __CTRL_CMD_MAX: u32 = 11;

pub const CTRL_CMD_MAX: u32 = __CTRL_CMD_MAX - 1;

pub const CTRL_ATTR_UNSPEC: u32 = 0;
pub const CTRL_ATTR_FAMILY_ID: u32 = 1;
pub const CTRL_ATTR_FAMILY_NAME: u32 = 2;
pub const CTRL_ATTR_VERSION: u32 = 3;
pub const CTRL_ATTR_HDRSIZE: u32 = 4;
pub const CTRL_ATTR_MAXATTR: u32 = 5;
pub const CTRL_ATTR_OPS: u32 = 6;
pub const CTRL_ATTR_MCAST_GROUPS: u32 = 7;
pub const CTRL_ATTR_POLICY: u32 = 8;
pub const CTRL_ATTR_OP_POLICY: u32 = 9;
pub const CTRL_ATTR_OP: u32 = 10;
pub const __CTRL_ATTR_MAX: u32 = 11;

pub const CTRL_ATTR_MAX: u32 = __CTRL_ATTR_MAX - 1;

pub const CTRL_ATTR_OP_UNSPEC: u32 = 0;
pub const CTRL_ATTR_OP_ID: u32 = 1;
pub const CTRL_ATTR_OP_FLAGS: u32 = 2;
pub const __CTRL_ATTR_OP_MAX: u32 = 3;

pub const CTRL_ATTR_OP_MAX: u32 = __CTRL_ATTR_OP_MAX - 1;

pub const CTRL_ATTR_MCAST_GRP_UNSPEC: u32 = 0;
pub const CTRL_ATTR_MCAST_GRP_NAME: u32 = 1;
pub const CTRL_ATTR_MCAST_GRP_ID: u32 = 2;
pub const __CTRL_ATTR_MCAST_GRP_MAX: u32 = 3;

pub const CTRL_ATTR_MCAST_GRP_MAX: u32 = __CTRL_ATTR_MCAST_GRP_MAX - 1;

pub const CTRL_ATTR_POLICY_UNSPEC: u32 = 0;
pub const CTRL_ATTR_POLICY_DO: u32 = 1;
pub const CTRL_ATTR_POLICY_DUMP: u32 = 2;

pub const __CTRL_ATTR_POLICY_DUMP_MAX: u32 = 3;
pub const CTRL_ATTR_POLICY_DUMP_MAX: u32 = __CTRL_ATTR_POLICY_DUMP_MAX - 1;

pub const CTRL_ATTR_POLICY_MAX: u32 = __CTRL_ATTR_POLICY_DUMP_MAX - 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
