/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// linux/skbuff.h, linux/slab.h, linux/netdevice.h

#[repr(C)]
pub struct gro_cell {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gro_cells {
    // C declaration: struct gro_cell __percpu *cells;
    pub cells: *mut gro_cell,
}

extern "C" {
    pub fn gro_cells_receive(gcells: *mut gro_cells, skb: *mut sk_buff) -> ::core::ffi::c_int;
    pub fn gro_cells_init(gcells: *mut gro_cells, dev: *mut net_device) -> ::core::ffi::c_int;
    pub fn gro_cells_destroy(gcells: *mut gro_cells);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
