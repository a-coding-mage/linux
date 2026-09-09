/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Defines for the BPQETHER pseudo device driver
 *
 * Dependency: `SIOCDEVPRIVATE` and `ETH_ALEN` are supplied by the Linux
 * interface headers corresponding to the original include.
 */

pub const SIOCSBPQETHOPT: usize = SIOCDEVPRIVATE + 0; /* reserved */
pub const SIOCSBPQETHADDR: usize = SIOCDEVPRIVATE + 1;

#[repr(C)]
pub struct bpq_ethaddr {
    pub destination: [u8; ETH_ALEN],
    pub accept: [u8; ETH_ALEN],
}

/*
 * For SIOCSBPQETHOPT - this is compatible with PI2/PacketTwin card drivers,
 * currently not implemented, though. If someone wants to hook a radio
 * to his Ethernet card he may find this useful. ;-)
 */

pub const SIOCGBPQETHPARAM: usize = 0x5000; /* get Level 1 parameters */
pub const SIOCSBPQETHPARAM: usize = 0x5001; /* set */

#[repr(C)]
pub struct bpq_req {
    pub cmd: i32,
    pub speed: i32,       /* unused */
    pub clockmode: i32,   /* unused */
    pub txdelay: i32,
    pub persist: u8,      /* unused */
    pub slotime: i32,     /* unused */
    pub squeldelay: i32,
    pub dmachan: i32,     /* unused */
    pub irq: i32,         /* unused */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
