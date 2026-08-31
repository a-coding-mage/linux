/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * if_xdp: XDP socket user-space interface
 * Copyright(c) 2018 Intel Corporation.
 *
 * Author(s): Bjorn Topel <bjorn.topel@intel.com>
 *	      Magnus Karlsson <magnus.karlsson@intel.com>
 */

/* C source included <linux/types.h> for these fixed-width integer aliases. */
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;

/* Options for the sxdp_flags field */
pub const XDP_SHARED_UMEM: u32 = 1 << 0;
pub const XDP_COPY: u32 = 1 << 1; /* Force copy-mode */
pub const XDP_ZEROCOPY: u32 = 1 << 2; /* Force zero-copy mode */
/* If this option is set, the driver might go sleep and in that case
 * the XDP_RING_NEED_WAKEUP flag in the fill and/or Tx rings will be
 * set. If it is set, the application need to explicitly wake up the
 * driver with a poll() (Rx and Tx) or sendto() (Tx only). If you are
 * running the driver and the application on the same core, you should
 * use this option so that the kernel will yield to the user space
 * application.
 */
pub const XDP_USE_NEED_WAKEUP: u32 = 1 << 3;
/* By setting this option, userspace application indicates that it can
 * handle multiple descriptors per packet thus enabling AF_XDP to split
 * multi-buffer XDP frames into multiple Rx descriptors. Without this set
 * such frames will be dropped.
 */
pub const XDP_USE_SG: u32 = 1 << 4;

/* Flags for xsk_umem_config flags */
pub const XDP_UMEM_UNALIGNED_CHUNK_FLAG: u32 = 1 << 0;

/* Force checksum calculation in software. Can be used for testing or
 * working around potential HW issues. This option causes performance
 * degradation and only works in XDP_COPY mode.
 */
pub const XDP_UMEM_TX_SW_CSUM: u32 = 1 << 1;

/* Request to reserve tx_metadata_len bytes of per-chunk metadata.
 */
pub const XDP_UMEM_TX_METADATA_LEN: u32 = 1 << 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct sockaddr_xdp {
    pub sxdp_family: __u16,
    pub sxdp_flags: __u16,
    pub sxdp_ifindex: __u32,
    pub sxdp_queue_id: __u32,
    pub sxdp_shared_umem_fd: __u32,
}

/* XDP_RING flags */
pub const XDP_RING_NEED_WAKEUP: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct xdp_ring_offset {
    pub producer: __u64,
    pub consumer: __u64,
    pub desc: __u64,
    pub flags: __u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct xdp_mmap_offsets {
    pub rx: xdp_ring_offset,
    pub tx: xdp_ring_offset,
    pub fr: xdp_ring_offset, /* Fill */
    pub cr: xdp_ring_offset, /* Completion */
}

/* XDP socket options */
pub const XDP_MMAP_OFFSETS: u32 = 1;
pub const XDP_RX_RING: u32 = 2;
pub const XDP_TX_RING: u32 = 3;
pub const XDP_UMEM_REG: u32 = 4;
pub const XDP_UMEM_FILL_RING: u32 = 5;
pub const XDP_UMEM_COMPLETION_RING: u32 = 6;
pub const XDP_STATISTICS: u32 = 7;
pub const XDP_OPTIONS: u32 = 8;
pub const XDP_MAX_TX_SKB_BUDGET: u32 = 9;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct xdp_umem_reg {
    pub addr: __u64, /* Start of packet data area */
    pub len: __u64, /* Length of packet data area */
    pub chunk_size: __u32,
    pub headroom: __u32,
    pub flags: __u32,
    pub tx_metadata_len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct xdp_statistics {
    pub rx_dropped: __u64, /* Dropped for other reasons */
    pub rx_invalid_descs: __u64, /* Dropped due to invalid descriptor */
    pub tx_invalid_descs: __u64, /* Dropped due to invalid descriptor */
    pub rx_ring_full: __u64, /* Dropped due to rx ring being full */
    pub rx_fill_ring_empty_descs: __u64, /* Failed to retrieve item from fill ring */
    pub tx_ring_empty_descs: __u64, /* Failed to retrieve item from tx ring */
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct xdp_options {
    pub flags: __u32,
}

/* Flags for the flags field of struct xdp_options */
pub const XDP_OPTIONS_ZEROCOPY: u32 = 1 << 0;

/* Pgoff for mmaping the rings */
pub const XDP_PGOFF_RX_RING: u64 = 0;
pub const XDP_PGOFF_TX_RING: u64 = 0x80000000;
pub const XDP_UMEM_PGOFF_FILL_RING: u64 = 0x100000000;
pub const XDP_UMEM_PGOFF_COMPLETION_RING: u64 = 0x180000000;

/* Masks for unaligned chunks mode */
pub const XSK_UNALIGNED_BUF_OFFSET_SHIFT: u32 = 48;
pub const XSK_UNALIGNED_BUF_ADDR_MASK: u64 = (1u64 << XSK_UNALIGNED_BUF_OFFSET_SHIFT) - 1;

/* Request transmit timestamp. Upon completion, put it into tx_timestamp
 * field of struct xsk_tx_metadata.
 */
pub const XDP_TXMD_FLAGS_TIMESTAMP: u32 = 1 << 0;

/* Request transmit checksum offload. Checksum start position and offset
 * are communicated via csum_start and csum_offset fields of struct
 * xsk_tx_metadata.
 */
pub const XDP_TXMD_FLAGS_CHECKSUM: u32 = 1 << 1;

/* Request launch time hardware offload. The device will schedule the packet for
 * transmission at a pre-determined time called launch time. The value of
 * launch time is communicated via launch_time field of struct xsk_tx_metadata.
 */
pub const XDP_TXMD_FLAGS_LAUNCH_TIME: u32 = 1 << 2;

/* AF_XDP offloads request. 'request' union member is consumed by the driver
 * when the packet is being transmitted. 'completion' union member is
 * filled by the driver when the transmit completion arrives.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_tx_metadata {
    pub flags: __u64,
    pub metadata: xsk_tx_metadata_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xsk_tx_metadata_union {
    pub request: xsk_tx_metadata_request,
    pub completion: xsk_tx_metadata_completion,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct xsk_tx_metadata_request {
    /* XDP_TXMD_FLAGS_CHECKSUM */

    /* Offset from desc->addr where checksumming should start. */
    pub csum_start: __u16,
    /* Offset from csum_start where checksum should be stored. */
    pub csum_offset: __u16,
    pub reserved: __u32,

    /* XDP_TXMD_FLAGS_LAUNCH_TIME */
    /* Launch time in nanosecond against the PTP HW Clock */
    pub launch_time: __u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct xsk_tx_metadata_completion {
    /* XDP_TXMD_FLAGS_TIMESTAMP */
    pub tx_timestamp: __u64,
}

/* Rx/Tx descriptor */
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct xdp_desc {
    pub addr: __u64,
    pub len: __u32,
    pub options: __u32,
}

/* UMEM descriptor is __u64 */

/* Flag indicating that the packet continues with the buffer pointed out by the
 * next frame in the ring. The end of the packet is signalled by setting this
 * bit to zero. For single buffer packets, every descriptor has 'options' set
 * to 0 and this maintains backward compatibility.
 */
pub const XDP_PKT_CONTD: u32 = 1 << 0;

/* TX packet carries valid metadata. */
pub const XDP_TX_METADATA: u32 = 1 << 1;
