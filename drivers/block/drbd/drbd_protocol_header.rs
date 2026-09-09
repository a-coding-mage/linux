/* SPDX-License-Identifier: GPL-2.0-only */

#[repr(i32)]
pub enum drbd_packet {
    /* receiver (data socket) */
    P_DATA = 0x00,
    P_DATA_REPLY = 0x01, /* Response to P_DATA_REQUEST */
    P_RS_DATA_REPLY = 0x02, /* Response to P_RS_DATA_REQUEST */
    P_BARRIER = 0x03,
    P_BITMAP = 0x04,
    P_BECOME_SYNC_TARGET = 0x05,
    P_BECOME_SYNC_SOURCE = 0x06,
    P_UNPLUG_REMOTE = 0x07, /* Used at various times to hint the peer */
    P_DATA_REQUEST = 0x08, /* Used to ask for a data block */
    P_RS_DATA_REQUEST = 0x09, /* Used to ask for a data block for resync */
    P_SYNC_PARAM = 0x0a,
    P_PROTOCOL = 0x0b,
    P_UUIDS = 0x0c,
    P_SIZES = 0x0d,
    P_STATE = 0x0e,
    P_SYNC_UUID = 0x0f,
    P_AUTH_CHALLENGE = 0x10,
    P_AUTH_RESPONSE = 0x11,
    P_STATE_CHG_REQ = 0x12,

    /* (meta socket) */
    P_PING = 0x13,
    P_PING_ACK = 0x14,
    P_RECV_ACK = 0x15, /* Used in protocol B */
    P_WRITE_ACK = 0x16, /* Used in protocol C */
    P_RS_WRITE_ACK = 0x17, /* Is a P_WRITE_ACK, additionally call set_in_sync(). */
    P_SUPERSEDED = 0x18, /* Used in proto C, two-primaries conflict detection */
    P_NEG_ACK = 0x19, /* Sent if local disk is unusable */
    P_NEG_DREPLY = 0x1a, /* Local disk is broken... */
    P_NEG_RS_DREPLY = 0x1b, /* Local disk is broken... */
    P_BARRIER_ACK = 0x1c,
    P_STATE_CHG_REPLY = 0x1d,

    /* "new" commands, no longer fitting into the ordering scheme above */
    P_OV_REQUEST = 0x1e, /* data socket */
    P_OV_REPLY = 0x1f,
    P_OV_RESULT = 0x20, /* meta socket */
    P_CSUM_RS_REQUEST = 0x21, /* data socket */
    P_RS_IS_IN_SYNC = 0x22, /* meta socket */
    P_SYNC_PARAM89 = 0x23, /* data socket, protocol version 89 replacement for P_SYNC_PARAM */
    P_COMPRESSED_BITMAP = 0x24, /* compressed or otherwise encoded bitmap transfer */
    /* P_CKPT_FENCE_REQ = 0x25, * currently reserved for protocol D */
    /* P_CKPT_DISABLE_REQ = 0x26, * currently reserved for protocol D */
    P_DELAY_PROBE = 0x27, /* is used on BOTH sockets */
    P_OUT_OF_SYNC = 0x28, /* Mark as out of sync (Outrunning), data socket */
    P_RS_CANCEL = 0x29, /* meta: Used to cancel RS_DATA_REQUEST packet by SyncSource */
    P_CONN_ST_CHG_REQ = 0x2a, /* data sock: Connection wide state request */
    P_CONN_ST_CHG_REPLY = 0x2b, /* meta sock: Connection side state req reply */
    P_RETRY_WRITE = 0x2c, /* Protocol C: retry conflicting write request */
    P_PROTOCOL_UPDATE = 0x2d, /* data sock: is used in established connections */
    /* 0x2e to 0x30 reserved, used in drbd 9 */
    P_TRIM = 0x31,
    P_RS_THIN_REQ = 0x32, /* Request a block for resync or reply P_RS_DEALLOCATED */
    P_RS_DEALLOCATED = 0x33, /* Contains only zeros on sync source node */
    P_WSAME = 0x34,
    /* 0x35 already claimed in DRBD 9 */
    P_ZEROES = 0x36, /* data sock: zero-out, WRITE_ZEROES */
    /* 0x40 .. 0x48 already claimed in DRBD 9 */
    P_MAY_IGNORE = 0x100, /* Flag to test if (cmd > P_MAY_IGNORE) ... */
    P_MAX_OPT_CMD = 0x101,
    /* special command ids for handshake */
    P_INITIAL_META = 0xfff1, /* First Packet on the MetaSock */
    P_INITIAL_DATA = 0xfff2, /* First Packet on the Socket */
    P_CONNECTION_FEATURES = 0xfffe, /* FIXED for the next century! */
}

#[repr(C, packed)]
pub struct p_header80 { pub magic: u32, pub command: u16, pub length: u16 }
#[repr(C, packed)]
pub struct p_header95 { pub magic: u16, pub command: u16, pub length: u32 }
#[repr(C, packed)]
pub struct p_header100 { pub magic: u32, pub volume: u16, pub command: u16, pub length: u32, pub pad: u32 }

pub const DP_HARDBARRIER: u32 = 1;
pub const DP_RW_SYNC: u32 = 2;
pub const DP_MAY_SET_IN_SYNC: u32 = 4;
pub const DP_UNPLUG: u32 = 8;
pub const DP_FUA: u32 = 16;
pub const DP_FLUSH: u32 = 32;
pub const DP_DISCARD: u32 = 64;
pub const DP_SEND_RECEIVE_ACK: u32 = 128;
pub const DP_SEND_WRITE_ACK: u32 = 256;
pub const DP_WSAME: u32 = 512;
pub const DP_ZEROES: u32 = 1024;

#[repr(C, packed)]
pub struct p_data { pub sector: u64, pub block_id: u64, pub seq_num: u32, pub dp_flags: u32 }
#[repr(C, packed)]
pub struct p_trim { pub p_data: p_data, pub size: u32 }
#[repr(C, packed)]
pub struct p_wsame { pub p_data: p_data, pub size: u32 }
#[repr(C, packed)]
pub struct p_block_ack { pub sector: u64, pub block_id: u64, pub blksize: u32, pub seq_num: u32 }
#[repr(C, packed)]
pub struct p_block_req { pub sector: u64, pub block_id: u64, pub blksize: u32, pub pad: u32 }

pub const DRBD_FF_TRIM: u32 = 1;
pub const DRBD_FF_THIN_RESYNC: u32 = 2;
pub const DRBD_FF_WSAME: u32 = 4;
pub const DRBD_FF_WZEROES: u32 = 8;

#[repr(C, packed)]
pub struct p_connection_features { pub protocol_min: u32, pub feature_flags: u32, pub protocol_max: u32, pub _pad: u32, pub reserved: [u64; 7] }
#[repr(C, packed)]
pub struct p_barrier { pub barrier: u32, pub pad: u32 }
#[repr(C, packed)]
pub struct p_barrier_ack { pub barrier: u32, pub set_size: u32 }
#[repr(C, packed)]
pub struct p_rs_param { pub resync_rate: u32, pub verify_alg: [core::ffi::c_char; 0] }
#[repr(C, packed)]
pub struct p_rs_param_89 { pub resync_rate: u32, pub verify_alg: [core::ffi::c_char; SHARED_SECRET_MAX], pub csums_alg: [core::ffi::c_char; SHARED_SECRET_MAX] }
#[repr(C, packed)]
pub struct p_rs_param_95 { pub resync_rate: u32, pub verify_alg: [core::ffi::c_char; SHARED_SECRET_MAX], pub csums_alg: [core::ffi::c_char; SHARED_SECRET_MAX], pub c_plan_ahead: u32, pub c_delay_target: u32, pub c_fill_target: u32, pub c_max_rate: u32 }

#[repr(i32)]
pub enum drbd_conn_flags { CF_DISCARD_MY_DATA = 1, CF_DRY_RUN = 2 }

#[repr(C, packed)]
pub struct p_protocol { pub protocol: u32, pub after_sb_0p: u32, pub after_sb_1p: u32, pub after_sb_2p: u32, pub conn_flags: u32, pub two_primaries: u32, pub integrity_alg: [core::ffi::c_char; 0] }
#[repr(C, packed)]
pub struct p_uuids { pub uuid: [u64; UI_EXTENDED_SIZE] }
#[repr(C, packed)]
pub struct p_rs_uuid { pub uuid: u64 }

#[repr(C, packed)]
pub struct o_qlim { pub physical_block_size: u32, pub logical_block_size: u32, pub alignment_offset: u32, pub io_min: u32, pub io_opt: u32, pub discard_enabled: u8, pub discard_zeroes_data: u8, pub write_same_capable: u8, pub _pad: u8 }
#[repr(C, packed)]
pub struct p_sizes { pub d_size: u64, pub u_size: u64, pub c_size: u64, pub max_bio_size: u32, pub queue_order_type: u16, pub dds_flags: u16, pub qlim: [o_qlim; 0] }
#[repr(C, packed)]
pub struct p_state { pub state: u32 }
#[repr(C, packed)]
pub struct p_req_state { pub mask: u32, pub val: u32 }
#[repr(C, packed)]
pub struct p_req_state_reply { pub retcode: u32 }
#[repr(C, packed)]
pub struct p_drbd06_param { pub size: u64, pub state: u32, pub blksize: u32, pub protocol: u32, pub version: u32, pub gen_cnt: [u32; 5], pub bit_map_gen: [u32; 5] }
#[repr(C, packed)]
pub struct p_block_desc { pub sector: u64, pub blksize: u32, pub pad: u32 }

#[repr(i32)]
pub enum drbd_bitmap_code { RLE_VLI_Bits = 2 }
#[repr(C, packed)]
pub struct p_compressed_bm { pub encoding: u8, pub code: [u8; 0] }
#[repr(C, packed)]
pub struct p_delay_probe93 { pub seq_num: u32, pub offset: u32 }

pub const DRBD_SOCKET_BUFFER_SIZE: usize = 4096;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
