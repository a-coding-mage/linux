/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent: __u32 and __u64 correspond to the Linux fixed-width
// unsigned integer types supplied by <linux/types.h>.

#[repr(C)]
pub struct trace_buffer_meta {
    pub meta_page_size: __u32,
    pub meta_struct_len: __u32,

    pub subbuf_size: __u32,
    pub nr_subbufs: __u32,

    pub reader: trace_buffer_meta_reader,

    pub flags: __u64,

    pub entries: __u64,
    pub overrun: __u64,
    pub read: __u64,

    pub pages_lost: __u64,
    pub pages_touched: __u64,
}

#[repr(C)]
pub struct trace_buffer_meta_reader {
    pub lost_events: __u64,
    pub id: __u32,
    pub read: __u32,
}

// TRACE_MMAP_IOCTL_GET_READER = _IO('R', 0x20)
pub const TRACE_MMAP_IOCTL_GET_READER: ::core::ffi::c_ulong =
    _IO(b'R' as ::core::ffi::c_ulong, 0x20);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
