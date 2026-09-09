/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * RCU segmented callback lists
 *
 * This seemingly RCU-private file must be available to SRCU users
 * because the size of the TREE SRCU srcu_struct structure depends
 * on these definitions.
 *
 * Copyright IBM Corporation, 2017
 *
 * Authors: Paul E. McKenney <paulmck@linux.net.ibm.com>
 */

// C dependencies: linux/types.h and linux/atomic.h.

/* Simple unsegmented callback lists. */
#[repr(C)]
pub struct RcuCblist {
    pub head: *mut RcuHead,
    pub tail: *mut *mut RcuHead,
    pub len: core::ffi::c_long,
}

#[macro_export]
macro_rules! RCU_CBLIST_INITIALIZER {
    ($n:ident) => {
        RcuCblist {
            head: core::ptr::null_mut(),
            tail: core::ptr::addr_of_mut!($n.head),
            len: 0,
        }
    };
}

/* Complicated segmented callback lists.  ;-) */

/*
 * Index values for segments in rcu_segcblist structure.
 *
 * The segments are as follows:
 *
 * [head, *tails[RCU_DONE_TAIL]):
 *  Callbacks whose grace period has elapsed, and thus can be invoked.
 * [*tails[RCU_DONE_TAIL], *tails[RCU_WAIT_TAIL]):
 *  Callbacks waiting for the current GP from the current CPU's viewpoint.
 * [*tails[RCU_WAIT_TAIL], *tails[RCU_NEXT_READY_TAIL]):
 *  Callbacks that arrived before the next GP started, again from
 *  the current CPU's viewpoint.  These can be handled by the next GP.
 * [*tails[RCU_NEXT_READY_TAIL], *tails[RCU_NEXT_TAIL]):
 *  Callbacks that might have arrived after the next GP started.
 *  There is some uncertainty as to when a given GP starts and
 *  ends, but a CPU knows the exact times if it is the one starting
 *  or ending the GP.  Other CPUs know that the previous GP ends
 *  before the next one starts.
 *
 * Note that RCU_WAIT_TAIL cannot be empty unless RCU_NEXT_READY_TAIL is also
 * empty.
 *
 * The ->gp_seq[] array contains the grace-period state at which the
 * corresponding segment of callbacks will be ready to invoke.  This tracks
 * both normal and expedited grace periods, allowing callbacks to complete
 * when either type of GP finishes.  A given element of this array is
 * meaningful only when the corresponding segment is non-empty, and it is
 * never valid for RCU_DONE_TAIL (whose callbacks are already ready to
 * invoke) or for RCU_NEXT_TAIL (whose callbacks have not yet been assigned
 * a grace-period state).
 */
pub const RCU_DONE_TAIL: usize = 0; /* Also RCU_WAIT head. */
pub const RCU_WAIT_TAIL: usize = 1; /* Also RCU_NEXT_READY head. */
pub const RCU_NEXT_READY_TAIL: usize = 2; /* Also RCU_NEXT head. */
pub const RCU_NEXT_TAIL: usize = 3;
pub const RCU_CBLIST_NSEGS: usize = 4;

/* NOCB offloading and de-offloading state-machine diagrams are preserved in
 * the source comments above; they describe the flag transitions. */
pub const SEGCBLIST_ENABLED: u32 = 1u32 << 0;
pub const SEGCBLIST_OFFLOADED: u32 = 1u32 << 1;

#[repr(C)]
pub struct RcuSegcblist {
    pub head: *mut RcuHead,
    pub tails: [*mut *mut RcuHead; RCU_CBLIST_NSEGS],
    pub gp_seq: [RcuGpSeq; RCU_CBLIST_NSEGS],
    // Under CONFIG_RCU_NOCB_CPU this field is atomic_long_t; otherwise long.
    #[cfg(CONFIG_RCU_NOCB_CPU)]
    pub len: AtomicLong,
    #[cfg(not(CONFIG_RCU_NOCB_CPU))]
    pub len: core::ffi::c_long,
    pub seglen: [core::ffi::c_long; RCU_CBLIST_NSEGS],
    pub flags: u8,
}

#[macro_export]
macro_rules! RCU_SEGCBLIST_INITIALIZER {
    ($n:ident) => {
        RcuSegcblist {
            head: core::ptr::null_mut(),
            tails: [
                core::ptr::addr_of_mut!($n.head),
                core::ptr::addr_of_mut!($n.head),
                core::ptr::addr_of_mut!($n.head),
                core::ptr::addr_of_mut!($n.head),
            ],
            gp_seq: [core::mem::zeroed(); RCU_CBLIST_NSEGS],
            len: core::mem::zeroed(),
            seglen: [0; RCU_CBLIST_NSEGS],
            flags: 0,
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
