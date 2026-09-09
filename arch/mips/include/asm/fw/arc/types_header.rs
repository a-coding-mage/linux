/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright 1999 Ralf Baechle (ralf@gnu.org)
 * Copyright 1999 Silicon Graphics, Inc.
 */

/* C build-time condition: CONFIG_FW_ARC32. */
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type CHAR = i8;
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type SHORT = i16;
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type LARGE_INTEGER = i64;
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type LONG = i32;
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type UCHAR = u8;
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type USHORT = u16;
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type ULONG = u32;
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type VOID = core::ffi::c_void;

/* The ARC structures contain 32-bit pointer representations under ARC32. */
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type _PCHAR = LONG;
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type _PSHORT = LONG;
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type _PLARGE_INTEGER = LONG;
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type _PLONG = LONG;
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type _PUCHAR = LONG;
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type _PUSHORT = LONG;
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type _PULONG = LONG;
#[cfg(feature = "CONFIG_FW_ARC32")]
pub type _PVOID = LONG;

/* C build-time condition: CONFIG_FW_ARC64. */
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type CHAR = i8;
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type SHORT = i16;
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type LARGE_INTEGER = i64;
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type LONG = i64;
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type UCHAR = u8;
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type USHORT = u16;
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type ULONG = u64;
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type VOID = core::ffi::c_void;

/* The firmware is 64-bit under ARC64, so pointer types are native pointers. */
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type _PCHAR = *mut CHAR;
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type _PSHORT = *mut SHORT;
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type _PLARGE_INTEGER = *mut LARGE_INTEGER;
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type _PLONG = *mut LONG;
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type _PUCHAR = *mut UCHAR;
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type _PUSHORT = *mut USHORT;
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type _PULONG = *mut ULONG;
#[cfg(feature = "CONFIG_FW_ARC64")]
pub type _PVOID = *mut VOID;

pub type PCHAR = *mut CHAR;
pub type PSHORT = *mut SHORT;
pub type PLARGE_INTEGER = *mut LARGE_INTEGER;
pub type PLONG = *mut LONG;
pub type PUCHAR = *mut UCHAR;
pub type PUSHORT = *mut USHORT;
pub type PULONG = *mut ULONG;
pub type PVOID = *mut VOID;

/* Return type of ArcGetDisplayStatus(). */
#[repr(C)]
pub struct DISPLAY_STATUS {
    pub CursorXPosition: USHORT,
    pub CursorYPosition: USHORT,
    pub CursorMaxXPosition: USHORT,
    pub CursorMaxYPosition: USHORT,
    pub ForegroundColor: USHORT,
    pub BackgroundColor: USHORT,
    pub HighIntensity: UCHAR,
    pub Underscored: UCHAR,
    pub ReverseVideo: UCHAR,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
