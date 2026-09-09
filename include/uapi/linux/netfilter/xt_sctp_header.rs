/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the surrounding Linux UAPI translation: __u8, __u16,
// __u32, and related type definitions.

pub const XT_SCTP_SRC_PORTS: u32 = 0x01;
pub const XT_SCTP_DEST_PORTS: u32 = 0x02;
pub const XT_SCTP_CHUNK_TYPES: u32 = 0x04;

pub const XT_SCTP_VALID_FLAGS: u32 = 0x07;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_sctp_flag_info {
    pub chunktype: __u8,
    pub flag: __u8,
    pub flag_mask: __u8,
}

pub const XT_NUM_SCTP_FLAGS: usize = 4;

#[repr(C)]
pub struct xt_sctp_info {
    pub dpts: [__u16; 2], // Min, Max
    pub spts: [__u16; 2], // Min, Max
    pub chunkmap: [__u32; 256 / core::mem::size_of::<__u32>()], // Bit mask of chunks to be matched according to RFC 2960
    pub chunk_match_type: __u32,
    pub flag_info: [xt_sctp_flag_info; XT_NUM_SCTP_FLAGS],
    pub flag_count: core::ffi::c_int,
    pub flags: __u32,
    pub invflags: __u32,
}

pub const SCTP_CHUNK_MATCH_ANY: __u32 = 0x01; // Match if any of the chunk types are present
pub const SCTP_CHUNK_MATCH_ALL: __u32 = 0x02; // Match if all of the chunk types are present
pub const SCTP_CHUNK_MATCH_ONLY: __u32 = 0x04; // Match if these are the only chunk types present

#[macro_export]
macro_rules! bytes {
    ($type:ty) => { core::mem::size_of::<$type>() * 8 };
}

#[macro_export]
macro_rules! SCTP_CHUNKMAP_SET {
    ($chunkmap:expr, $type:expr) => {{
        ($chunkmap)[$type / bytes!(__u32)] |= 1u32 << ($type % bytes!(__u32));
    }};
}

#[macro_export]
macro_rules! SCTP_CHUNKMAP_CLEAR {
    ($chunkmap:expr, $type:expr) => {{
        ($chunkmap)[$type / bytes!(__u32)] &= !(1u32 << ($type % bytes!(__u32)));
    }};
}

#[macro_export]
macro_rules! SCTP_CHUNKMAP_IS_SET {
    ($chunkmap:expr, $type:expr) => {{
        if (($chunkmap)[$type / bytes!(__u32)] & (1u32 << ($type % bytes!(__u32)))) != 0 {
            1
        } else {
            0
        }
    }};
}

#[macro_export]
macro_rules! SCTP_CHUNKMAP_RESET {
    ($chunkmap:expr) => {{
        unsafe {
            core::ptr::write_bytes(($chunkmap).as_mut_ptr(), 0, ($chunkmap).len());
        }
    }};
}

#[macro_export]
macro_rules! SCTP_CHUNKMAP_SET_ALL {
    ($chunkmap:expr) => {{
        unsafe {
            core::ptr::write_bytes(($chunkmap).as_mut_ptr(), u8::MAX, ($chunkmap).len());
        }
    }};
}

#[macro_export]
macro_rules! SCTP_CHUNKMAP_COPY {
    ($destmap:expr, $srcmap:expr) => {{
        unsafe {
            core::ptr::copy_nonoverlapping(
                ($srcmap).as_ptr(),
                ($destmap).as_mut_ptr(),
                ($srcmap).len(),
            );
        }
    }};
}

#[inline]
pub unsafe fn __sctp_chunkmap_is_clear(chunkmap: *const __u32, n: usize) -> bool {
    let mut i = 0;
    while i < n {
        if *chunkmap.add(i) != 0 {
            return false;
        }
        i += 1;
    }
    true
}

#[macro_export]
macro_rules! SCTP_CHUNKMAP_IS_CLEAR {
    ($chunkmap:expr) => {{
        unsafe { $crate::__sctp_chunkmap_is_clear(($chunkmap).as_ptr(), ($chunkmap).len()) }
    }};
}

#[inline]
pub unsafe fn __sctp_chunkmap_is_all_set(chunkmap: *const __u32, n: usize) -> bool {
    let mut i = 0;
    while i < n {
        if *chunkmap.add(i) != __u32::MAX {
            return false;
        }
        i += 1;
    }
    true
}

#[macro_export]
macro_rules! SCTP_CHUNKMAP_IS_ALL_SET {
    ($chunkmap:expr) => {{
        unsafe { $crate::__sctp_chunkmap_is_all_set(($chunkmap).as_ptr(), ($chunkmap).len()) }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
