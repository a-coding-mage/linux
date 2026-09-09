/* SPDX-License-Identifier: GPL-2.0 */

// In the kernel build, this corresponds to:
// typedef unsigned long kernel_ulong_t;
pub type kernel_ulong_t = std::os::raw::c_ulong;

/* EISA */

pub const EISA_SIG_LEN: usize = 8;

/* The EISA signature, in ASCII form, null terminated */
#[repr(C)]
pub struct eisa_device_id {
    pub sig: [std::os::raw::c_char; EISA_SIG_LEN],
    pub driver_data: kernel_ulong_t,
}

pub const EISA_DEVICE_MODALIAS_FMT: &str = "eisa:s%s";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
