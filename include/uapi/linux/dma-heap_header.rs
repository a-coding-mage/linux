/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * DMABUF Heaps Userspace API
 *
 * Copyright (C) 2011 Google, Inc.
 * Copyright (C) 2019 Linaro Ltd.
 */

// Dependency intent from the C header:
// #include <linux/ioctl.h>
// #include <linux/types.h>

/**
 * DOC: DMABUF Heaps Userspace API
 */

/* Valid FD_FLAGS are O_CLOEXEC, O_RDONLY, O_WRONLY, O_RDWR */
pub const DMA_HEAP_VALID_FD_FLAGS: _ = O_CLOEXEC | O_ACCMODE;

/* Currently no heap flags */
pub const DMA_HEAP_VALID_HEAP_FLAGS: u64 = 0_u64;

/**
 * struct dma_heap_allocation_data - metadata passed from userspace for
 *                                      allocations
 * @len:                size of the allocation
 * @fd:                 will be populated with a fd which provides the
 *                      handle to the allocated dma-buf
 * @fd_flags:           file descriptor flags used when allocating
 * @heap_flags:         flags passed to heap
 *
 * Provided by userspace as an argument to the ioctl
 */
#[repr(C)]
pub struct dma_heap_allocation_data {
    pub len: u64,
    pub fd: u32,
    pub fd_flags: u32,
    pub heap_flags: u64,
}

pub const DMA_HEAP_IOC_MAGIC: u8 = b'H';

/**
 * DOC: DMA_HEAP_IOCTL_ALLOC - allocate memory from pool
 *
 * Takes a dma_heap_allocation_data struct and returns it with the fd field
 * populated with the dmabuf handle of the allocation.
 */
pub const DMA_HEAP_IOCTL_ALLOC: _ = _IOWR(
    DMA_HEAP_IOC_MAGIC,
    0x0,
    dma_heap_allocation_data,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
