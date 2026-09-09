/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * The C header guard and include directives have no direct Rust equivalent.
 * Types referenced from other headers are intentionally left as external
 * dependencies.
 */

pub const HVPIPE_HMC_ID_MASK: u32 = 0x0200_0000; /* 02-HMC, 00-reserved and HMC ID */
pub const HVPIPE_MAX_WRITE_BUFFER_SIZE: usize = 4048;

/*
 * hvpipe specific RTAS return values
 */
pub const RTAS_HVPIPE_CLOSED: i32 = -4;

pub const HVPIPE_HDR_LEN: usize = core::mem::size_of::<papr_hvpipe_hdr>();

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hvpipe_migrate_action {
    HVPIPE_SUSPEND,
    HVPIPE_RESUME,
}

#[repr(C)]
pub struct hvpipe_source_info {
    pub list: list_head, /* list of sources */
    pub srcID: u32,
    pub hvpipe_status: u32,
    pub recv_wqh: wait_queue_head_t, /* wake up poll() waitq */
}

/*
 * Source ID Format 0xCCRRQQQQ
 * CC = indicating value is source type (ex: 0x02 for HMC)
 * RR = 0x00 (reserved)
 * QQQQ = 0x0000 – 0xFFFF indicating the source index indetifier
 */
#[repr(C)]
pub struct hvpipe_event_buf {
    pub srcID: __be32, /* Source ID */
    pub event_type: u8, /* 0x01 for hvpipe message available */
    /* from specified src ID */
    /* 0x02 for loss of pipe connection */
    /* with specified src ID */
}

unsafe extern "C" {
    pub fn hvpipe_migration_handler(action: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
