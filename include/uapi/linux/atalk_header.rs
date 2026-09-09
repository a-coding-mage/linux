/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent from the C header: linux/types.h, asm/byteorder.h, and
// linux/socket.h provide the integer aliases, byte-order types, and socket
// constants used below.

/*
 * AppleTalk networking structures
 *
 * The following are directly referenced from the University Of Michigan
 * netatalk for compatibility reasons.
 */
pub const ATPORT_FIRST: u8 = 1;
pub const ATPORT_RESERVED: u8 = 128;
pub const ATPORT_LAST: u8 = 254; // 254 is only legal on localtalk
pub const ATADDR_ANYNET: u16 = 0;
pub const ATADDR_ANYNODE: u8 = 0;
pub const ATADDR_ANYPORT: u8 = 0;
pub const ATADDR_BCAST: u8 = 255;
pub const DDP_MAXSZ: usize = 587;
pub const DDP_MAXHOPS: u8 = 15; // 4 bits of hop counter

pub const SIOCATALKDIFADDR: u32 = SIOCPROTOPRIVATE + 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atalk_addr {
    pub s_net: u16,
    pub s_node: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_at {
    pub sat_family: __kernel_sa_family_t,
    pub sat_port: u8,
    pub sat_addr: atalk_addr,
    pub sat_zero: [std::ffi::c_char; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atalk_netrange {
    pub nr_phase: u8,
    pub nr_firstnet: u16,
    pub nr_lastnet: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
