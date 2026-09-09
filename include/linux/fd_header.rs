/* SPDX-License-Identifier: GPL-2.0 */

// The declarations referenced by the original UAPI include are supplied by
// other translated headers.

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct compat_floppy_struct {
    pub size: compat_uint_t,
    pub sect: compat_uint_t,
    pub head: compat_uint_t,
    pub track: compat_uint_t,
    pub stretch: compat_uint_t,
    pub gap: u8,
    pub rate: u8,
    pub spec1: u8,
    pub fmt_gap: u8,
    pub name: compat_caddr_t,
}

// #define FDGETPRM32 _IOR(2, 0x04, struct compat_floppy_struct)
// The _IOR ioctl encoding is retained here as the corresponding Rust value.
#[cfg(CONFIG_COMPAT)]
pub const FDGETPRM32: u64 =
    (2u64 << 30) | ((core::mem::size_of::<compat_floppy_struct>() as u64) << 16)
        | (2u64 << 8) | 0x04u64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
