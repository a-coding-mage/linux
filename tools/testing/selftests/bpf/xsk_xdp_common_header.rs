// SPDX-License-Identifier: GPL-2.0

pub const MAX_SOCKETS: u32 = 2;

// Depends on the C `struct ethhdr` definition supplied by another header.
pub const PKT_HDR_ALIGN: usize = core::mem::size_of::<ethhdr>() + 2; // Just to align the data in the packet

#[repr(C, align(32))]
pub struct xdp_info {
    pub count: u64,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
