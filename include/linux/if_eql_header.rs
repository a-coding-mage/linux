/*
 * Equalizer Load-balancer for serial network interfaces.
 *
 * (c) Copyright 1995 Simon "Guru Aleph-Null" Janes
 * NCM: Network and Communications Management, Inc.
 *
 *
 *	This software may be used and distributed according to the terms
 *	of the GNU General Public License, incorporated herein by reference.
 * 
 * The author may be reached as simon@ncm.com, or C/O
 *    NCM
 *    Attn: Simon Janes
 *    6803 Whittier Ave
 *    McLean VA 22101
 *    Phone: 1-703-847-0040 ext 103
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/timer.h, linux/spinlock.h, net/net_trackers.h, and
// uapi/linux/if_eql.h.

#[repr(C)]
pub struct slave {
    pub list: list_head,
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub priority: libc::c_long,
    pub priority_bps: libc::c_long,
    pub priority_Bps: libc::c_long,
    pub bytes_queued: libc::c_long,
}

pub type slave_t = slave;

#[repr(C)]
pub struct slave_queue {
    pub lock: spinlock_t,
    pub all_slaves: list_head,
    pub num_slaves: libc::c_int,
    pub master_dev: *mut net_device,
}

pub type slave_queue_t = slave_queue;

#[repr(C)]
pub struct equalizer {
    pub queue: slave_queue_t,
    pub min_slaves: libc::c_int,
    pub max_slaves: libc::c_int,
    pub timer: timer_list,
}

pub type equalizer_t = equalizer;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
