/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// __KERNEL__-only dependency: <linux/build_bug.h>
// Dependency: Linux integer types are represented by Rust fixed-width integers.

pub const BSG_PROTOCOL_SCSI: u32 = 0;

pub const BSG_SUB_PROTOCOL_SCSI_CMD: u32 = 0;
pub const BSG_SUB_PROTOCOL_SCSI_TMF: u32 = 1;
pub const BSG_SUB_PROTOCOL_SCSI_TRANSPORT: u32 = 2;

/*
 * For flag constants below:
 * sg.h sg_io_hdr also has bits defined for it's flags member. These
 * two flag values (0x10 and 0x20) have the same meaning in sg.h . For
 * bsg the BSG_FLAG_Q_AT_HEAD flag is ignored since it is the deafult.
 */
pub const BSG_FLAG_Q_AT_TAIL: u32 = 0x10; // default is Q_AT_HEAD
pub const BSG_FLAG_Q_AT_HEAD: u32 = 0x20;

#[repr(C)]
pub struct sg_io_v4 {
	pub guard: i32,             /* [i] 'Q' to differentiate from v3 */
	pub protocol: u32,          /* [i] 0 -> SCSI , .... */
	pub subprotocol: u32,       /* [i] 0 -> SCSI command, 1 -> SCSI task
	                              management function, .... */

	pub request_len: u32,       /* [i] in bytes */
	pub request: u64,           /* [i], [*i] {SCSI: cdb} */
	pub request_tag: u64,       /* [i] {SCSI: task tag (only if flagged)} */
	pub request_attr: u32,      /* [i] {SCSI: task attribute} */
	pub request_priority: u32,  /* [i] {SCSI: task priority} */
	pub request_extra: u32,     /* [i] {spare, for padding} */
	pub max_response_len: u32,  /* [i] in bytes */
	pub response: u64,          /* [i], [*o] {SCSI: (auto)sense data} */

	/* "dout_": data out (to device); "din_": data in (from device) */
	pub dout_iovec_count: u32,  /* [i] 0 -> "flat" dout transfer else
	                              dout_xfer points to array of iovec */
	pub dout_xfer_len: u32,     /* [i] bytes to be transferred to device */
	pub din_iovec_count: u32,   /* [i] 0 -> "flat" din transfer */
	pub din_xfer_len: u32,      /* [i] bytes to be transferred from device */
	pub dout_xferp: u64,        /* [i], [*i] */
	pub din_xferp: u64,         /* [i], [*o] */

	pub timeout: u32,            /* [i] units: millisecond */
	pub flags: u32,              /* [i] bit mask */
	pub usr_ptr: u64,            /* [i->o] unused internally */
	pub spare_in: u32,           /* [i] */

	pub driver_status: u32,      /* [o] 0 -> ok */
	pub transport_status: u32,   /* [o] 0 -> ok */
	pub device_status: u32,      /* [o] {SCSI: command completion status} */
	pub retry_delay: u32,        /* [o] {SCSI: status auxiliary information} */
	pub info: u32,               /* [o] additional information */
	pub duration: u32,           /* [o] time to complete, in milliseconds */
	pub response_len: u32,       /* [o] bytes of response actually written */
	pub din_resid: i32,          /* [o] din_xfer_len - actual_din_xfer_len */
	pub dout_resid: i32,         /* [o] dout_xfer_len - actual_dout_xfer_len */
	pub generated_tag: u64,      /* [o] {SCSI: transport generated task tag} */
	pub spare_out: u32,          /* [o] */

	pub padding: u32,
}

#[repr(C)]
pub struct bsg_uring_cmd {
	pub request: u64,            /* [i], [*i] command descriptor address */
	pub request_len: u32,        /* [i] command descriptor length in bytes */
	pub protocol: u32,           /* [i] protocol type (BSG_PROTOCOL_*) */
	pub subprotocol: u32,        /* [i] subprotocol type (BSG_SUB_PROTOCOL_*) */
	pub max_response_len: u32,   /* [i] response buffer size in bytes */

	pub response: u64,            /* [i], [*o] response data address */
	pub dout_xferp: u64,          /* [i], [*i] */
	pub dout_xfer_len: u32,       /* [i] bytes to be transferred to device */
	pub dout_iovec_count: u32,   /* [i] 0 -> "flat" dout transfer else
	                               * dout_xferp points to array of iovec
	                               */
	pub din_xferp: u64,           /* [i], [*o] */
	pub din_xfer_len: u32,        /* [i] bytes to be transferred from device */
	pub din_iovec_count: u32,     /* [i] 0 -> "flat" din transfer */

	pub timeout_ms: u32,          /* [i] timeout in milliseconds */
	pub reserved: [u8; 12],       /* reserved for future extension */
}

// __KERNEL__: must match IORING_OP_URING_CMD payload size (e.g. SQE128).

/*
 * SCSI BSG io_uring completion (res2, 64-bit)
 *
 * When using BSG_PROTOCOL_SCSI + BSG_SUB_PROTOCOL_SCSI_CMD with
 * IORING_OP_URING_CMD, the completion queue entry (CQE) contains:
 *   - result: errno (0 on success)
 *   - res2: packed SCSI status
 *
 * res2 bit layout:
 *   [0..7]   device_status  (SCSI status byte, e.g. CHECK_CONDITION)
 *   [8..15]  driver_status  (e.g. DRIVER_SENSE when sense data is valid)
 *   [16..23] host_status    (e.g. DID_OK, DID_TIME_OUT)
 *   [24..31] sense_len_wr   (bytes of sense data written to response buffer)
 *   [32..63] resid_len      (residual transfer length)
 */
#[inline]
pub fn bsg_scsi_res2_device_status(res2: u64) -> u8 {
	res2 as u8
}

#[inline]
pub fn bsg_scsi_res2_driver_status(res2: u64) -> u8 {
	(res2 >> 8) as u8
}

#[inline]
pub fn bsg_scsi_res2_host_status(res2: u64) -> u8 {
	(res2 >> 16) as u8
}

#[inline]
pub fn bsg_scsi_res2_sense_len(res2: u64) -> u8 {
	(res2 >> 24) as u8
}

#[inline]
pub fn bsg_scsi_res2_resid_len(res2: u64) -> u32 {
	(res2 >> 32) as u32
}

#[inline]
pub fn bsg_scsi_res2_build(
	device_status: u8,
	driver_status: u8,
	host_status: u8,
	sense_len_wr: u8,
	resid_len: u32,
) -> u64 {
	((resid_len as u64) << 32)
		| ((sense_len_wr as u64) << 24)
		| ((host_status as u64) << 16)
		| ((driver_status as u64) << 8)
		| (device_status as u64)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
