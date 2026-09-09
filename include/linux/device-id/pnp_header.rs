/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard and include directives have no executable Rust equivalent.
// Under __KERNEL__, linux/types.h supplies __u8 and kernel_ulong_t; those names
// remain external dependencies here.

pub const PNP_ID_LEN: usize = 8;
pub const PNP_MAX_DEVICES: usize = 8;

#[repr(C)]
pub struct pnp_device_id {
    pub id: [__u8; PNP_ID_LEN],
    pub driver_data: kernel_ulong_t,
}

#[repr(C)]
pub struct pnp_card_device_id {
    pub id: [__u8; PNP_ID_LEN],
    pub driver_data: kernel_ulong_t,
    pub devs: [pnp_card_device_id_devs; PNP_MAX_DEVICES],
}

#[repr(C)]
pub struct pnp_card_device_id_devs {
    pub id: [__u8; PNP_ID_LEN],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
