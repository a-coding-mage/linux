/* SPDX-License-Identifier: GPL-2.0 */

// Under the C kernel build, this header includes <linux/types.h> and defines
// kernel_ulong_t as unsigned long. Rust's usize preserves that pointer-sized
// integer intent.
pub type kernel_ulong_t = usize;

/* s390 css bus devices (subchannels) */
#[repr(C)]
pub struct css_device_id {
    pub match_flags: u8,
    pub r#type: u8, /* subchannel type */
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
