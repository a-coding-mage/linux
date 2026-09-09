/* SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note */
/*
 * 1999 Copyright (C) Pavel Machek, pavel@ucw.cz. This code is GPL.
 * 1999/11/04 Copyright (C) 1999 VMware, Inc. (Regis "HPReg" Duchesne)
 *            Made nbd_end_request() use the io_request_lock
 * 2001 Copyright (C) Steven Whitehouse
 *            New nbd_end_request() for compatibility with new linux block
 *            layer code.
 * 2003/06/24 Louis D. Langholtz <ldl@aros.net>
 *            Removed unneeded blksize_bits field from nbd_device struct.
 *            Cleanup PARANOIA usage & code.
 * 2004/02/19 Paul Clements
 *            Removed PARANOIA, plus various cleanup and comments
 * 2023 Copyright Red Hat
 *            Link to userspace extensions, favor cookie over handle.
 */

// C dependency: <linux/types.h>
// The Linux _IO(type, nr) ioctl encoding is represented here for the
// no-argument commands used by this header.
const fn _io(ty: u32, nr: u32) -> u32 { (ty << 8) | nr }

pub const NBD_SET_SOCK: u32 = _io(0xab, 0);
pub const NBD_SET_BLKSIZE: u32 = _io(0xab, 1);
pub const NBD_SET_SIZE: u32 = _io(0xab, 2);
pub const NBD_DO_IT: u32 = _io(0xab, 3);
pub const NBD_CLEAR_SOCK: u32 = _io(0xab, 4);
pub const NBD_CLEAR_QUE: u32 = _io(0xab, 5);
pub const NBD_PRINT_DEBUG: u32 = _io(0xab, 6);
pub const NBD_SET_SIZE_BLOCKS: u32 = _io(0xab, 7);
pub const NBD_DISCONNECT: u32 = _io(0xab, 8);
pub const NBD_SET_TIMEOUT: u32 = _io(0xab, 9);
pub const NBD_SET_FLAGS: u32 = _io(0xab, 10);

/*
 * See also https://github.com/NetworkBlockDevice/nbd/blob/master/doc/proto.md
 * for additional userspace extensions not yet utilized in the kernel module.
 */

pub const NBD_CMD_READ: u32 = 0;
pub const NBD_CMD_WRITE: u32 = 1;
pub const NBD_CMD_DISC: u32 = 2;
pub const NBD_CMD_FLUSH: u32 = 3;
pub const NBD_CMD_TRIM: u32 = 4;
/* userspace defines additional extension commands */
pub const NBD_CMD_WRITE_ZEROES: u32 = 6;

/* values for flags field, these are server interaction specific. */
pub const NBD_FLAG_HAS_FLAGS: u32 = 1 << 0; /* nbd-server supports flags */
pub const NBD_FLAG_READ_ONLY: u32 = 1 << 1; /* device is read-only */
pub const NBD_FLAG_SEND_FLUSH: u32 = 1 << 2; /* can flush writeback cache */
pub const NBD_FLAG_SEND_FUA: u32 = 1 << 3; /* send FUA (forced unit access) */
pub const NBD_FLAG_ROTATIONAL: u32 = 1 << 4; /* device is rotational */
pub const NBD_FLAG_SEND_TRIM: u32 = 1 << 5; /* send trim/discard */
pub const NBD_FLAG_SEND_WRITE_ZEROES: u32 = 1 << 6; /* supports WRITE_ZEROES */
/* there is a gap here to match userspace */
pub const NBD_FLAG_CAN_MULTI_CONN: u32 = 1 << 8; /* Server supports multiple connections per export. */

/* values for cmd flags in the upper 16 bits of request type */
pub const NBD_CMD_FLAG_FUA: u32 = 1 << 16; /* FUA (forced unit access) op */
pub const NBD_CMD_FLAG_NO_HOLE: u32 = 1 << 17; /* Do not punch a hole for WRITE_ZEROES */

/* These are client behavior specific flags. */
pub const NBD_CFLAG_DESTROY_ON_DISCONNECT: u32 = 1 << 0; /* delete the nbd device on disconnect. */
pub const NBD_CFLAG_DISCONNECT_ON_CLOSE: u32 = 1 << 1; /* disconnect the nbd device on close by last opener. */

/* userspace doesn't need the nbd_device structure */

/* These are sent over the network in the request/reply magic fields */
pub const NBD_REQUEST_MAGIC: u32 = 0x25609513;
pub const NBD_REPLY_MAGIC: u32 = 0x67446698;
/* Do *not* use magics: 0x12560953 0x96744668. */
/* magic 0x668e33ef for structured reply not supported by kernel yet */

#[repr(C)]
pub union nbd_request_cookie {
    pub cookie: u64, /* Opaque identifier for request */
    pub handle: [u8; 8], /* older spelling of cookie */
}

#[repr(C, packed)]
pub struct nbd_request {
    pub magic: u32, /* NBD_REQUEST_MAGIC */
    pub type_: u32, /* See NBD_CMD_* */
    pub cookie: nbd_request_cookie,
    pub from: u64,
    pub len: u32,
}

#[repr(C)]
pub union nbd_reply_cookie {
    pub cookie: u64, /* Opaque identifier from request */
    pub handle: [u8; 8], /* older spelling of cookie */
}

#[repr(C)]
pub struct nbd_reply {
    pub magic: u32, /* NBD_REPLY_MAGIC */
    pub error: u32, /* 0 = ok, else error */
    pub cookie: nbd_reply_cookie,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
