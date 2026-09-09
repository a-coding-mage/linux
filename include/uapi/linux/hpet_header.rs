/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// The ioctl encoding helpers below are supplied by the surrounding UAPI
// environment; the original header includes the compiler support for them.

#[repr(C)]
pub struct hpet_info {
    pub hi_ireqfreq: ::core::ffi::c_ulong, /* Hz */
    pub hi_flags: ::core::ffi::c_ulong,    /* information */
    pub hi_hpet: ::core::ffi::c_ushort,
    pub hi_timer: ::core::ffi::c_ushort,
}

pub const HPET_INFO_PERIODIC: u32 = 0x0010; /* periodic-capable comparator */

pub const HPET_IE_ON: u32 = _IO(b'h' as _, 0x01); /* interrupt on */
pub const HPET_IE_OFF: u32 = _IO(b'h' as _, 0x02); /* interrupt off */
pub const HPET_INFO: u32 = _IOR(b'h' as _, 0x03, hpet_info); /* information */
pub const HPET_EPI: u32 = _IO(b'h' as _, 0x04); /* enable periodic */
pub const HPET_DPI: u32 = _IO(b'h' as _, 0x05); /* disable periodic */
pub const HPET_IRQFREQ: u32 = _IOW(b'h' as _, 0x6, ::core::ffi::c_ulong); /* IRQFREQ usec */

pub const MAX_HPET_TBS: u32 = 8; /* maximum hpet timer blocks */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
