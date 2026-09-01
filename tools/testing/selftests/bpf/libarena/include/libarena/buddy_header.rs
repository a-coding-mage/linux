// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/*
 * C header dependencies intentionally not implemented here:
 * - u8/u32/u64 map to Rust primitive integer types.
 * - size_t maps to usize.
 * - void * maps to core::ffi::c_void.
 * - __arena is a BPF address-space annotation with no direct Rust item here.
 * - arena_spinlock_t is provided by another header.
 */

/*
 * Minimum allocation is 1 << BUDDY_MIN_ALLOC_SHIFT.
 * Larger sizes increase internal fragmentation, but smaller
 * sizes increase the space overhead of the block metadata.
 */
pub const BUDDY_MIN_ALLOC_SHIFT: usize = 4;
pub const BUDDY_MIN_ALLOC_BYTES: usize = 1 << BUDDY_MIN_ALLOC_SHIFT;

/*
 * How many orders the buddy allocator can serve. Minimum block
 * size is 1 << BUDDY_MIN_ALLOC_SHIFT, maximum block size is
 * 1 << (BUDDY_MIN_ALLOC_SHIFT + BUDDY_CHUNK_NUM_ORDERS - 1):
 * Each block has size 1 << BUDDY_MIN_ALLOC_SHIFT, and the
 * allocation orders are in [0, BUDDY_CHUNK_NUM_ORDERS).
 * We keep two blocks of the maximum size to retain the
 * property in the code that all blocks have a buddy.
 * Higher values increase the maximum allocation size,
 * but also the size of the metadata for each block.
 */
pub const BUDDY_CHUNK_NUM_ORDERS: usize = 1 << 4;
pub const BUDDY_CHUNK_BYTES: usize = BUDDY_MIN_ALLOC_BYTES << BUDDY_CHUNK_NUM_ORDERS;

/* Offset of the buddy header within a free block, see buddy.bpf.c for details */
pub const BUDDY_HEADER_OFF: usize = 8;

/* The maximum number of blocks a chunk may have to track. */
pub const BUDDY_CHUNK_ITEMS: usize = 1 << BUDDY_CHUNK_NUM_ORDERS;
pub const BUDDY_CHUNK_OFFSET_MASK: usize = BUDDY_CHUNK_BYTES - 1;

/*
 * Alignment for chunk allocations based on bpf_arena_alloc_pages.
 * The arena allocation kfunc does not have an alignment argument,
 * but that is required for all block calculations in the chunk to
 * work.
 */
pub const BUDDY_VADDR_OFFSET: usize = BUDDY_CHUNK_BYTES;

/* Total arena virtual address space the allocator can consume. */
pub const BUDDY_VADDR_SIZE: usize = BUDDY_CHUNK_BYTES << 10;

#[repr(C)]
pub struct buddy_header {
    pub prev_index: u32, /* "Pointer" to the previous available allocation of the same size. */
    pub next_index: u32, /* Same for the next allocation. */
}

/*
 * We bring memory into the allocator 1 MiB at a time.
 */
#[repr(C)]
pub struct buddy_chunk {
    /* The order of the current allocation for a item. 4 bits per order. */
    pub orders: [u8; BUDDY_CHUNK_ITEMS / 2],
    /*
     * Bit to denote whether chunk is allocated. Size of the allocated/free
     * chunk found from the orders array.
     */
    pub allocated: [u8; BUDDY_CHUNK_ITEMS / 8],
    /* Freelists for O(1) allocation. */
    pub freelists: [u64; BUDDY_CHUNK_NUM_ORDERS],
    pub next: *mut buddy_chunk,
}

#[repr(C)]
pub struct buddy {
    pub first_chunk: *mut buddy_chunk, /* Pointer to the chunk linked list. */
    pub lock: arena_spinlock_t,        /* Allocator lock */
    pub vaddr: u64,                    /* Allocation into reserved vaddr */
}

/* Original declarations are gated by #ifdef __BPF__. */
#[cfg(__BPF__)]
unsafe extern "C" {
    pub fn buddy_init(buddy: *mut buddy) -> i32;
    pub fn buddy_destroy(buddy: *mut buddy) -> i32;
    pub fn buddy_free(buddy: *mut buddy, free: *mut core::ffi::c_void) -> i32;
    pub fn buddy_alloc(buddy: *mut buddy, size: usize) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
