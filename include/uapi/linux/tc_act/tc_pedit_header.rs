/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from the Linux UAPI header tc_pedit.h. */

/* Dependency supplied by linux/types.h and linux/pkt_cls.h. */

#[repr(i32)]
pub enum TcaPedit {
    TCA_PEDIT_UNSPEC,
    TCA_PEDIT_TM,
    TCA_PEDIT_PARMS,
    TCA_PEDIT_PAD,
    TCA_PEDIT_PARMS_EX,
    TCA_PEDIT_KEYS_EX,
    TCA_PEDIT_KEY_EX,
    __TCA_PEDIT_MAX,
}

pub const TCA_PEDIT_MAX: i32 = TcaPedit::__TCA_PEDIT_MAX as i32 - 1;

pub const TCA_PEDIT_KEY_EX_HTYPE: i32 = 1;
pub const TCA_PEDIT_KEY_EX_CMD: i32 = 2;
pub const __TCA_PEDIT_KEY_EX_MAX: i32 = 3;
pub const TCA_PEDIT_KEY_EX_MAX: i32 = __TCA_PEDIT_KEY_EX_MAX - 1;

/* TCA_PEDIT_KEY_EX_HDR_TYPE_NETWORK is a special case for legacy users. It
 * means no specific header type - offset is relative to the network layer.
 */
#[repr(i32)]
pub enum PeditHeaderType {
    TCA_PEDIT_KEY_EX_HDR_TYPE_NETWORK = 0,
    TCA_PEDIT_KEY_EX_HDR_TYPE_ETH = 1,
    TCA_PEDIT_KEY_EX_HDR_TYPE_IP4 = 2,
    TCA_PEDIT_KEY_EX_HDR_TYPE_IP6 = 3,
    TCA_PEDIT_KEY_EX_HDR_TYPE_TCP = 4,
    TCA_PEDIT_KEY_EX_HDR_TYPE_UDP = 5,
    __PEDIT_HDR_TYPE_MAX,
}

pub const TCA_PEDIT_HDR_TYPE_MAX: i32 = PeditHeaderType::__PEDIT_HDR_TYPE_MAX as i32 - 1;

#[repr(i32)]
pub enum PeditCmd {
    TCA_PEDIT_KEY_EX_CMD_SET = 0,
    TCA_PEDIT_KEY_EX_CMD_ADD = 1,
    __PEDIT_CMD_MAX,
}

pub const TCA_PEDIT_CMD_MAX: i32 = PeditCmd::__PEDIT_CMD_MAX as i32 - 1;

#[repr(C)]
pub struct tc_pedit_key {
    pub mask: u32, /* AND */
    pub val: u32,  /* XOR */
    pub off: u32,  /* offset */
    pub at: u32,
    pub offmask: u32,
    pub shift: u32,
}

#[repr(C)]
pub struct tc_pedit_sel {
    /* tc_gen; expanded by the dependent linux/pkt_cls.h definition. */
    pub tc_gen: tc_gen,
    pub nkeys: u8,
    pub flags: u8,
    /* Flexible array member, counted by nkeys. */
    pub keys: [tc_pedit_key; 0],
}

pub type tc_pedit = tc_pedit_sel;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
