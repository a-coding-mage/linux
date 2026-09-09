/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Framework for buffer objects that can be shared across devices/subsystems.
 *
 * Copyright(C) 2015 Intel Ltd
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 as published by
 * the Free Software Foundation.
 */

// C dependencies: linux/ioctl.h and linux/types.h.

/**
 * struct dma_buf_sync - Synchronize with CPU access.
 *
 * When a DMA buffer is accessed from the CPU via mmap, it is not always
 * possible to guarantee coherency between the CPU-visible map and underlying
 * memory.  To manage coherency, DMA_BUF_IOCTL_SYNC must be used to bracket
 * any CPU access to give the kernel the chance to shuffle memory around if
 * needed.
 *
 * Prior to accessing the map, the client must call DMA_BUF_IOCTL_SYNC
 * with DMA_BUF_SYNC_START and the appropriate read/write flags.  Once the
 * access is complete, the client should call DMA_BUF_IOCTL_SYNC with
 * DMA_BUF_SYNC_END and the same read/write flags.
 *
 * The synchronization provided via DMA_BUF_IOCTL_SYNC only provides cache
 * coherency.  It does not prevent other processes or devices from accessing
 * the memory at the same time.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_buf_sync {
    /// Set of access flags.
    pub flags: u64,
}

pub const DMA_BUF_SYNC_READ: u64 = 1 << 0;
pub const DMA_BUF_SYNC_WRITE: u64 = 2 << 0;
pub const DMA_BUF_SYNC_RW: u64 = DMA_BUF_SYNC_READ | DMA_BUF_SYNC_WRITE;
pub const DMA_BUF_SYNC_START: u64 = 0 << 2;
pub const DMA_BUF_SYNC_END: u64 = 1 << 2;
pub const DMA_BUF_SYNC_VALID_FLAGS_MASK: u64 = DMA_BUF_SYNC_RW | DMA_BUF_SYNC_END;

pub const DMA_BUF_NAME_LEN: usize = 32;

/** Get a sync_file from a dma-buf. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_buf_export_sync_file {
    /// Read/write flags.
    pub flags: u32,
    /// Returned sync file descriptor.
    pub fd: i32,
}

/** Insert a sync_file into a dma-buf. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_buf_import_sync_file {
    /// Read/write flags.
    pub flags: u32,
    /// Sync file descriptor.
    pub fd: i32,
}

pub const DMA_BUF_BASE: u8 = b'b';

// These ioctl values are architecture-dependent _IOW/_IOWR encodings from
// linux/ioctl.h. The ioctl encoding macros are supplied by the surrounding
// UAPI environment.
pub const DMA_BUF_IOCTL_SYNC: u64 = _IOW!(DMA_BUF_BASE, 0, dma_buf_sync);

/* 32/64bitness of this uapi was botched in android, there's no difference
 * between them in actual uapi, they're just different numbers.
 */
pub const DMA_BUF_SET_NAME: u64 = _IOW!(DMA_BUF_BASE, 1, *const i8);
pub const DMA_BUF_SET_NAME_A: u64 = _IOW!(DMA_BUF_BASE, 1, u32);
pub const DMA_BUF_SET_NAME_B: u64 = _IOW!(DMA_BUF_BASE, 1, u64);
pub const DMA_BUF_IOCTL_EXPORT_SYNC_FILE: u64 =
    _IOWR!(DMA_BUF_BASE, 2, dma_buf_export_sync_file);
pub const DMA_BUF_IOCTL_IMPORT_SYNC_FILE: u64 =
    _IOW!(DMA_BUF_BASE, 3, dma_buf_import_sync_file);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
