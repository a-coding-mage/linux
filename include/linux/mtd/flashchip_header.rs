/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright © 2000      Red Hat UK Limited
 * Copyright © 2000-2010 David Woodhouse <dwmw2@infradead.org>
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum flstate_t {
    FL_READY,
    FL_STATUS,
    FL_CFI_QUERY,
    FL_JEDEC_QUERY,
    FL_ERASING,
    FL_ERASE_SUSPENDING,
    FL_ERASE_SUSPENDED,
    FL_WRITING,
    FL_WRITING_TO_BUFFER,
    FL_OTP_WRITE,
    FL_WRITE_SUSPENDING,
    FL_WRITE_SUSPENDED,
    FL_PM_SUSPENDED,
    FL_SYNCING,
    FL_UNLOADING,
    FL_LOCKING,
    FL_UNLOCKING,
    FL_POINT,
    FL_XIP_WHILE_ERASING,
    FL_XIP_WHILE_WRITING,
    FL_SHUTDOWN,
    /* These 2 come from nand_state_t, which has been unified here */
    FL_READING,
    FL_CACHEDPRG,
    /* These 4 come from onenand_state_t, which has been unified here */
    FL_RESETTING,
    FL_OTPING,
    FL_PREPARING_ERASE,
    FL_VERIFYING_ERASE,

    FL_UNKNOWN,
}

/* NOTE: confusingly, this can be used to refer to more than one chip at a time,
   if they're interleaved.  This can even refer to individual partitions on
   the same physical chip when present. */
#[repr(C)]
pub struct flchip {
    pub start: ::core::ffi::c_ulong, /* Offset within the map */
    // pub len: ::core::ffi::c_ulong;
    /* We omit len for now, because when we group them together
       we insist that they're all of the same size, and the chip size
       is held in the next level up. If we get more versatile later,
       it'll make it a damn sight harder to find which chip we want from
       a given offset, and we'll want to add the per-chip length field
       back in.
    */
    pub ref_point_counter: ::core::ffi::c_int,
    pub state: flstate_t,
    pub oldstate: flstate_t,

    /* C bit-fields; each flag occupies one bit in the original object. */
    pub write_suspended: ::core::ffi::c_uint,
    pub erase_suspended: ::core::ffi::c_uint,
    pub in_progress_block_addr: ::core::ffi::c_ulong,
    pub in_progress_block_mask: ::core::ffi::c_ulong,

    pub mutex: mutex,
    pub wq: wait_queue_head_t, /* Wait on here when we're waiting for the chip
                                  to be ready */
    pub word_write_time: ::core::ffi::c_int,
    pub buffer_write_time: ::core::ffi::c_int,
    pub erase_time: ::core::ffi::c_int,

    pub word_write_time_max: ::core::ffi::c_int,
    pub buffer_write_time_max: ::core::ffi::c_int,
    pub erase_time_max: ::core::ffi::c_int,

    pub priv_: *mut ::core::ffi::c_void,
}

/* This is used to handle contention on write/erase operations
   between partitions of the same physical chip. */
#[repr(C)]
pub struct flchip_shared {
    pub lock: mutex,
    pub writing: *mut flchip,
    pub erasing: *mut flchip,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
