/* SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause) */
/*
 * Copyright(c) 2018 Intel Corporation.
 *
 */

/* C dependency: <rdma/ib_pack.h> */

#[repr(C)]
pub struct tid_rdma_read_req {
    pub kdeth0: __le32,
    pub kdeth1: __le32,
    pub reth: ib_reth,
    pub tid_flow_psn: __be32,
    pub tid_flow_qp: __be32,
    pub verbs_qp: __be32,
}

#[repr(C)]
pub struct tid_rdma_read_resp {
    pub kdeth0: __le32,
    pub kdeth1: __le32,
    pub aeth: __be32,
    pub reserved: [__be32; 4],
    pub verbs_psn: __be32,
    pub verbs_qp: __be32,
}

#[repr(C)]
pub struct tid_rdma_write_req {
    pub kdeth0: __le32,
    pub kdeth1: __le32,
    pub reth: ib_reth,
    pub reserved: [__be32; 2],
    pub verbs_qp: __be32,
}

#[repr(C)]
pub struct tid_rdma_write_resp {
    pub kdeth0: __le32,
    pub kdeth1: __le32,
    pub aeth: __be32,
    pub reserved: [__be32; 3],
    pub tid_flow_psn: __be32,
    pub tid_flow_qp: __be32,
    pub verbs_qp: __be32,
}

#[repr(C)]
pub struct tid_rdma_write_data {
    pub kdeth0: __le32,
    pub kdeth1: __le32,
    pub reserved: [__be32; 6],
    pub verbs_qp: __be32,
}

#[repr(C)]
pub struct tid_rdma_resync {
    pub kdeth0: __le32,
    pub kdeth1: __le32,
    pub reserved: [__be32; 6],
    pub verbs_qp: __be32,
}

#[repr(C)]
pub struct tid_rdma_ack {
    pub kdeth0: __le32,
    pub kdeth1: __le32,
    pub aeth: __be32,
    pub reserved: [__be32; 2],
    pub tid_flow_psn: __be32,
    pub verbs_psn: __be32,
    pub tid_flow_qp: __be32,
    pub verbs_qp: __be32,
}

/*
 * TID RDMA Opcodes
 */
pub const IB_OPCODE_TID_RDMA: u8 = 0xe0;

pub const IB_OPCODE_WRITE_REQ: u8 = 0x0;
pub const IB_OPCODE_WRITE_RESP: u8 = 0x1;
pub const IB_OPCODE_WRITE_DATA: u8 = 0x2;
pub const IB_OPCODE_WRITE_DATA_LAST: u8 = 0x3;
pub const IB_OPCODE_READ_REQ: u8 = 0x4;
pub const IB_OPCODE_READ_RESP: u8 = 0x5;
pub const IB_OPCODE_RESYNC: u8 = 0x6;
pub const IB_OPCODE_ACK: u8 = 0x7;

/* Expansion of IB_OPCODE(TID_RDMA, x), dependent on ib_pack.h's macro. */
pub const IB_OPCODE_TID_RDMA_WRITE_REQ: u16 = ((IB_OPCODE_TID_RDMA as u16) << 4) | IB_OPCODE_WRITE_REQ as u16;
pub const IB_OPCODE_TID_RDMA_WRITE_RESP: u16 = ((IB_OPCODE_TID_RDMA as u16) << 4) | IB_OPCODE_WRITE_RESP as u16;
pub const IB_OPCODE_TID_RDMA_WRITE_DATA: u16 = ((IB_OPCODE_TID_RDMA as u16) << 4) | IB_OPCODE_WRITE_DATA as u16;
pub const IB_OPCODE_TID_RDMA_WRITE_DATA_LAST: u16 = ((IB_OPCODE_TID_RDMA as u16) << 4) | IB_OPCODE_WRITE_DATA_LAST as u16;
pub const IB_OPCODE_TID_RDMA_READ_REQ: u16 = ((IB_OPCODE_TID_RDMA as u16) << 4) | IB_OPCODE_READ_REQ as u16;
pub const IB_OPCODE_TID_RDMA_READ_RESP: u16 = ((IB_OPCODE_TID_RDMA as u16) << 4) | IB_OPCODE_READ_RESP as u16;
pub const IB_OPCODE_TID_RDMA_RESYNC: u16 = ((IB_OPCODE_TID_RDMA as u16) << 4) | IB_OPCODE_RESYNC as u16;
pub const IB_OPCODE_TID_RDMA_ACK: u16 = ((IB_OPCODE_TID_RDMA as u16) << 4) | IB_OPCODE_ACK as u16;

pub const TID_OP_WRITE_REQ: u16 = IB_OPCODE_TID_RDMA_WRITE_REQ;
pub const TID_OP_WRITE_RESP: u16 = IB_OPCODE_TID_RDMA_WRITE_RESP;
pub const TID_OP_WRITE_DATA: u16 = IB_OPCODE_TID_RDMA_WRITE_DATA;
pub const TID_OP_WRITE_DATA_LAST: u16 = IB_OPCODE_TID_RDMA_WRITE_DATA_LAST;
pub const TID_OP_READ_REQ: u16 = IB_OPCODE_TID_RDMA_READ_REQ;
pub const TID_OP_READ_RESP: u16 = IB_OPCODE_TID_RDMA_READ_RESP;
pub const TID_OP_RESYNC: u16 = IB_OPCODE_TID_RDMA_RESYNC;
pub const TID_OP_ACK: u16 = IB_OPCODE_TID_RDMA_ACK;

/*
 * Define TID RDMA specific WR opcodes. The ib_wr_opcode
 * enum already provides some reserved values for use by
 * low level drivers. Two of those are used but renamed
 * to be more descriptive.
 */
pub const IB_WR_TID_RDMA_WRITE: u32 = IB_WR_RESERVED1;
pub const IB_WR_TID_RDMA_READ: u32 = IB_WR_RESERVED2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
