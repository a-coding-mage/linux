/* SPDX-License-Identifier: ((GPL-2.0-only WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Translated from linux/can.h. */

/* Dependencies supplied by the Linux UAPI translation environment. */

/* controller area network (CAN) kernel definitions */

pub const CAN_EFF_FLAG: u32 = 0x80000000u32; /* EFF/SFF is set in the MSB */
pub const CAN_RTR_FLAG: u32 = 0x40000000u32; /* remote transmission request */
pub const CAN_ERR_FLAG: u32 = 0x20000000u32; /* error message frame */

pub const CAN_SFF_MASK: u32 = 0x000007FFu32; /* standard frame format (SFF) */
pub const CAN_EFF_MASK: u32 = 0x1FFFFFFFu32; /* extended frame format (EFF) */
pub const CAN_ERR_MASK: u32 = 0x1FFFFFFFu32; /* omit EFF, RTR, ERR flags */
pub const CANXL_PRIO_MASK: u32 = CAN_SFF_MASK; /* 11 bit priority mask */

pub type canid_t = u32;

pub const CAN_SFF_ID_BITS: u32 = 11;
pub const CAN_EFF_ID_BITS: u32 = 29;
pub const CANXL_PRIO_BITS: u32 = CAN_SFF_ID_BITS;

pub type can_err_mask_t = u32;

pub const CAN_MAX_DLC: usize = 8;
pub const CAN_MAX_RAW_DLC: usize = 15;
pub const CAN_MAX_DLEN: usize = 8;
pub const CANFD_MAX_DLC: usize = 15;
pub const CANFD_MAX_DLEN: usize = 64;

pub const CANXL_MIN_DLC: usize = 0;
pub const CANXL_MAX_DLC: usize = 2047;
pub const CANXL_MAX_DLC_MASK: u16 = 0x07FF;
pub const CANXL_MIN_DLEN: usize = 1;
pub const CANXL_MAX_DLEN: usize = 2048;

#[repr(C)]
pub union can_frame_len {
    pub len: u8,
    pub can_dlc: u8,
}

#[repr(C, align(8))]
pub struct can_frame {
    pub can_id: canid_t,
    pub __bindgen_anon_1: can_frame_len,
    pub __pad: u8,
    pub __res0: u8,
    pub len8_dlc: u8,
    pub data: [u8; CAN_MAX_DLEN],
}

pub const CANFD_BRS: u8 = 0x01;
pub const CANFD_ESI: u8 = 0x02;
pub const CANFD_FDF: u8 = 0x04;

#[repr(C, align(8))]
pub struct canfd_frame {
    pub can_id: canid_t,
    pub len: u8,
    pub flags: u8,
    pub __res0: u8,
    pub __res1: u8,
    pub data: [u8; CANFD_MAX_DLEN],
}

pub const CANXL_XLF: u8 = 0x80;
pub const CANXL_SEC: u8 = 0x01;
pub const CANXL_RRS: u8 = 0x02;
pub const CANXL_VCID_OFFSET: u32 = 16;
pub const CANXL_VCID_VAL_MASK: u64 = 0xFFu64;
pub const CANXL_VCID_MASK: u64 = CANXL_VCID_VAL_MASK << CANXL_VCID_OFFSET;

#[repr(C)]
pub struct canxl_frame {
    pub prio: canid_t,
    pub flags: u8,
    pub sdt: u8,
    pub len: u16,
    pub af: u32,
    pub data: [u8; CANXL_MAX_DLEN],
}

pub const CAN_MTU: usize = core::mem::size_of::<can_frame>();
pub const CANFD_MTU: usize = core::mem::size_of::<canfd_frame>();
pub const CANXL_MTU: usize = core::mem::size_of::<canxl_frame>();
pub const CANXL_HDR_SIZE: usize = core::mem::offset_of!(canxl_frame, data);
pub const CANXL_MIN_MTU: usize = CANXL_HDR_SIZE + 64;
pub const CANXL_MAX_MTU: usize = CANXL_MTU;

pub const CAN_RAW: u32 = 1;
pub const CAN_BCM: u32 = 2;
pub const CAN_TP16: u32 = 3;
pub const CAN_TP20: u32 = 4;
pub const CAN_MCNET: u32 = 5;
pub const CAN_ISOTP: u32 = 6;
pub const CAN_J1939: u32 = 7;
pub const CAN_NPROTO: u32 = 8;

pub const SOL_CAN_BASE: u32 = 100;

#[repr(C)]
pub struct can_sockaddr_tp {
    pub rx_id: canid_t,
    pub tx_id: canid_t,
}

#[repr(C)]
pub struct can_sockaddr_j1939 {
    pub name: u64,
    pub pgn: u32,
    pub addr: u8,
}

#[repr(C)]
pub union can_addr {
    pub tp: can_sockaddr_tp,
    pub j1939: can_sockaddr_j1939,
}

#[repr(C)]
pub struct sockaddr_can {
    pub can_family: __kernel_sa_family_t,
    pub can_ifindex: i32,
    pub can_addr: can_addr,
}

#[repr(C)]
pub struct can_filter {
    pub can_id: canid_t,
    pub can_mask: canid_t,
}

pub const CAN_INV_FILTER: u32 = 0x20000000u32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
