/* include/linux/aio_abi.h
 *
 * Copyright 2000,2001,2002 Red Hat.
 *
 * Written by Benjamin LaHaise <bcrl@kvack.org>
 *
 * Distribute under the terms of the GPLv2 (see ../../COPYING) or under
 * the following terms.
 *
 * Permission to use, copy, modify, and distribute this software and its
 * documentation is hereby granted, provided that the above copyright
 * notice appears in all copies.  This software is provided without any
 * warranty, express or implied.  Red Hat makes no representations about
 * the suitability of this software for any purpose.
 *
 * IN NO EVENT SHALL RED HAT BE LIABLE TO ANY PARTY FOR DIRECT, INDIRECT,
 * SPECIAL, INCIDENTAL, OR CONSEQUENTIAL DAMAGES ARISING OUT OF THE USE OF
 * THIS SOFTWARE AND ITS DOCUMENTATION, EVEN IF RED HAT HAS BEEN ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * RED HAT DISCLAIMS ANY WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE.  THE SOFTWARE PROVIDED HEREUNDER IS ON AN "AS IS" BASIS, AND
 * RED HAT HAS NO OBLIGATION TO PROVIDE MAINTENANCE, SUPPORT, UPDATES,
 * ENHANCEMENTS, OR MODIFICATIONS.
 */

pub type aio_context_t = __kernel_ulong_t;

#[repr(u32)]
pub enum __UnnamedEnumAioAbi {
    IOCB_CMD_PREAD = 0,
    IOCB_CMD_PWRITE = 1,
    IOCB_CMD_FSYNC = 2,
    IOCB_CMD_FDSYNC = 3,
    /* 4 was the experimental IOCB_CMD_PREADX */
    IOCB_CMD_POLL = 5,
    IOCB_CMD_NOOP = 6,
    IOCB_CMD_PREADV = 7,
    IOCB_CMD_PWRITEV = 8,
}

pub const IOCB_FLAG_RESFD: u32 = 1 << 0;
pub const IOCB_FLAG_IOPRIO: u32 = 1 << 1;

/* Valid flags for the "aio_flags" member of the "struct iocb".
 *
 * IOCB_FLAG_RESFD - Set if the "aio_resfd" member of the "struct iocb"
 *                   is valid.
 * IOCB_FLAG_IOPRIO - Set if the "aio_reqprio" member of the "struct iocb"
 *                    is valid.
 */

/* read() from /dev/aio returns these structures. */
#[repr(C)]
pub struct io_event {
    pub data: __u64, /* the data field from the iocb */
    pub obj: __u64,  /* what iocb this event came from */
    pub res: __s64,  /* result code for this event */
    pub res2: __s64, /* secondary result */
}

/*
 * we always use a 64bit off_t when communicating
 * with userland.  its up to libraries to do the
 * proper padding and aio_error abstraction
 */

#[repr(C)]
pub struct iocb {
    /* these are internal to the kernel/libc. */
    pub aio_data: __u64, /* data to be returned in event's data */

    /* The C header selects this order according to the target byte order. */
    #[cfg(target_endian = "little")]
    pub aio_key: __u32, /* the kernel sets aio_key to the req # */
    #[cfg(target_endian = "little")]
    pub aio_rw_flags: __kernel_rwf_t, /* RWF_* flags */
    #[cfg(target_endian = "big")]
    pub aio_rw_flags: __kernel_rwf_t, /* RWF_* flags */
    #[cfg(target_endian = "big")]
    pub aio_key: __u32, /* the kernel sets aio_key to the req # */

    /* common fields */
    pub aio_lio_opcode: __u16, /* see IOCB_CMD_ above */
    pub aio_reqprio: __s16,
    pub aio_fildes: __u32,

    pub aio_buf: __u64,
    pub aio_nbytes: __u64,
    pub aio_offset: __s64,

    /* extra parameters */
    pub aio_reserved2: __u64, /* TODO: use this for a (struct sigevent *) */

    /* flags for the "struct iocb" */
    pub aio_flags: __u32,

    /*
     * if the IOCB_FLAG_RESFD flag of "aio_flags" is set, this is an
     * eventfd to signal AIO readiness to
     */
    pub aio_resfd: __u32,
} /* 64 bytes */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
