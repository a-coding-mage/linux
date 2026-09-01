// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// Translated from C source depending on:
// <libarena/common.h>, <libarena/asan.h>, <libarena/buddy.h>

/*
 * Buddy allocator arena-based implementation.
 *
 * Memory is organized into chunks. These chunks
 * cannot be coalesced or split. Allocating
 * chunks allocates their memory eagerly.
 *
 * Internally, each chunk is organized into blocks.
 * Blocks _can_ be coalesced/split, but only inside
 * the chunk. Each block can be allocated or
 * unallocated. If allocated, the entire block holds
 * user data. If unallocated, the block is mostly
 * invalid memory, with the exception of a header
 * used for freelist tracking.
 *
 * The header is placed at an offset inside the block
 * to prevent off-by-one errors from the previous block
 * from trivially overwriting the header. Such an error
 * is also not catchable by ASAN, since the header remains
 * valid memory even after the block is freed. It is still
 * theoretically possible for the header to be corrupted
 * without being caught by ASAN, but harder.
 *
 * Since the allocator needs to track order information for
 * both allocated and free blocks, and allocated blocks cannot
 * store a header, the allocator also stores per-chunk order
 * information in a reserved region at the beginning of the
 * chunk. The header includes a bitmap with the order of blocks
 * and their allocation state. It also includes the freelist
 * heads for the allocation itself.
 */

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;
use core::sync::atomic::{AtomicU64, Ordering};

const BUDDY_POISONED: i8 = 0xef_u8 as i8;

/* Number of pages to be allocated per chunk. */
const BUDDY_CHUNK_PAGES: u64 = BUDDY_CHUNK_BYTES / __PAGE_SIZE;

extern "C" {
    static mut arena: c_void;
    static can_loop: bool;
    static zero: u64;

    fn arena_spin_lock(lock: *mut c_void) -> i32;
    fn arena_spin_unlock(lock: *mut c_void);
    fn bpf_arena_reserve_pages(arena: *mut c_void, addr: *mut c_void, pages: u64) -> i32;
    fn bpf_arena_free_pages(arena: *mut c_void, addr: *mut c_void, pages: u64);
    fn bpf_arena_alloc_pages(
        arena: *mut c_void,
        addr: *mut c_void,
        pages: u64,
        numa_node: i32,
        flags: u64,
    ) -> *mut buddy_chunk;
    fn arena_fls(n: u64) -> i32;
    fn asan_ready() -> bool;
    fn asan_poison(addr: *mut c_void, val: i8, size: u64);
    fn asan_unpoison(addr: *mut c_void, size: u64);
    fn arena_stderr(fmt: *const u8, ...);
}

extern "Rust" {
    static BUDDY_CHUNK_BYTES: u64;
    static __PAGE_SIZE: u64;
    static BUDDY_VADDR_OFFSET: u64;
    static BUDDY_VADDR_SIZE: u64;
    static BUDDY_CHUNK_ITEMS: u64;
    static BUDDY_CHUNK_NUM_ORDERS: u64;
    static BUDDY_MIN_ALLOC_BYTES: u64;
    static BUDDY_HEADER_OFF: u64;
    static BUDDY_MIN_ALLOC_SHIFT: u64;
    static BUDDY_CHUNK_OFFSET_MASK: u64;
    static NUMA_NO_NODE: i32;
    static EINVAL: i32;
    static ENOMEM: i32;
}

#[repr(C)]
pub struct buddy {
    pub lock: c_void,
    pub vaddr: u64,
    pub first_chunk: *mut buddy_chunk,
}

#[repr(C)]
pub struct buddy_header {
    pub next_index: u64,
    pub prev_index: u64,
}

#[repr(C)]
pub struct buddy_chunk {
    pub next: *mut buddy_chunk,
    pub freelists: [u64; BUDDY_CHUNK_NUM_ORDERS as usize],
    pub allocated: [u8; (BUDDY_CHUNK_ITEMS as usize + 7) / 8],
    pub orders: [u8; (BUDDY_CHUNK_ITEMS as usize + 1) / 2],
}

#[inline(always)]
unsafe fn buddy_lock(buddy: *mut buddy) -> i32 {
    arena_spin_lock(&mut (*buddy).lock)
}

#[inline(always)]
unsafe fn buddy_unlock(buddy: *mut buddy) {
    arena_spin_unlock(&mut (*buddy).lock);
}

/*
 * Reserve part of the arena address space for the allocator. We use
 * this to get aligned addresses for the chunks, since the arena
 * page alloc kfuncs do not support aligning to a boundary (in this
 * case 1 MiB, see buddy.h on how this is derived).
 */
unsafe fn buddy_reserve_arena_vaddr(buddy: *mut buddy) -> i32 {
    (*buddy).vaddr = 0;

    bpf_arena_reserve_pages(
        &mut arena,
        BUDDY_VADDR_OFFSET as *mut c_void,
        BUDDY_VADDR_SIZE / __PAGE_SIZE,
    )
}

/*
 * Free up any unused address space. Used only during teardown.
 */
unsafe fn buddy_unreserve_arena_vaddr(buddy: *mut buddy) {
    bpf_arena_free_pages(
        &mut arena,
        (BUDDY_VADDR_OFFSET + (*buddy).vaddr) as *mut c_void,
        (BUDDY_VADDR_SIZE - (*buddy).vaddr) / __PAGE_SIZE,
    );

    (*buddy).vaddr = 0;
}

/*
 * Carve out part of the reserved address space and hand it over
 * to the buddy allocator.
 *
 * We are assuming the buddy allocator is the only allocator in the
 * system, so there is no race between this function reserving a
 * page range and some other allocator actually making the BPF call
 * to really create and reserve it.
 *
 * However, bump allocation must still be atomic because this function
 * is called without the buddy lock from multiple threads concurrently.
 */
#[no_mangle]
pub unsafe extern "C" fn buddy_alloc_arena_vaddr(buddy: *mut buddy, vaddrp: *mut u64) -> i32 {
    let mut vaddr: u64;
    let mut old: u64;
    let new: u64;

    if buddy.is_null() || vaddrp.is_null() {
        return -EINVAL;
    }

    loop {
        vaddr = (*buddy).vaddr;
        new = vaddr.wrapping_add(BUDDY_CHUNK_BYTES);

        if new > BUDDY_VADDR_SIZE {
            return -EINVAL;
        }

        old = (*(ptr::addr_of_mut!((*buddy).vaddr) as *mut AtomicU64))
            .compare_exchange(vaddr, new, Ordering::SeqCst, Ordering::SeqCst)
            .err()
            .unwrap_or(vaddr);
        if !(old != vaddr && can_loop) {
            break;
        }
    }

    if old != vaddr {
        return -EINVAL;
    }

    *vaddrp = BUDDY_VADDR_OFFSET + vaddr;

    0
}

unsafe fn arena_next_pow2(mut n: u64) -> u64 {
    n = n.wrapping_sub(1);
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n |= n >> 32;
    n = n.wrapping_add(1);

    n
}

#[no_mangle]
pub unsafe extern "C" fn idx_set_allocated(
    chunk: *mut buddy_chunk,
    idx: u64,
    allocated: bool,
) -> i32 {
    let already_allocated: bool;

    if idx >= BUDDY_CHUNK_ITEMS {
        arena_stderr(
            b"setting state of invalid idx (%ld, max %d)\n\0".as_ptr(),
            idx,
            BUDDY_CHUNK_ITEMS,
        );
        return -EINVAL;
    }

    already_allocated =
        ((*chunk).allocated[(idx / 8) as usize] & (1u8 << (idx % 8) as u32)) != 0;
    if already_allocated == allocated {
        arena_stderr(
            b"Double %s of idx %ld for chunk %p\0".as_ptr(),
            if allocated {
                b"alloc\0".as_ptr()
            } else {
                b"free\0".as_ptr()
            },
            idx,
            chunk,
        );
        return -EINVAL;
    }

    if allocated {
        (*chunk).allocated[(idx / 8) as usize] |= 1u8 << (idx % 8) as u32;
    } else {
        (*chunk).allocated[(idx / 8) as usize] &= !(1u8 << (idx % 8) as u32);
    }

    0
}

unsafe fn idx_is_allocated(chunk: *mut buddy_chunk, idx: u64, allocated: *mut bool) -> i32 {
    if idx >= BUDDY_CHUNK_ITEMS {
        arena_stderr(
            b"getting state of invalid idx (%llu, max %d)\n\0".as_ptr(),
            idx,
            BUDDY_CHUNK_ITEMS,
        );
        return -EINVAL;
    }

    *allocated = ((*chunk).allocated[(idx / 8) as usize] & (1u8 << (idx % 8) as u32)) != 0;
    0
}

#[no_mangle]
pub unsafe extern "C" fn idx_set_order(chunk: *mut buddy_chunk, idx: u64, mut order: u8) -> i32 {
    let prev_order: u8;

    if (order as u64) >= BUDDY_CHUNK_NUM_ORDERS {
        arena_stderr(b"setting invalid order %u\n\0".as_ptr(), order as u32);
        return -EINVAL;
    }

    if idx >= BUDDY_CHUNK_ITEMS {
        arena_stderr(
            b"setting order of invalid idx (%d, max %d)\n\0".as_ptr(),
            idx,
            BUDDY_CHUNK_ITEMS,
        );
        return -EINVAL;
    }

    /*
     * We store two order instances per byte, one per nibble.
     * Retain the existing nibble.
     */
    prev_order = (*chunk).orders[(idx / 2) as usize];
    if (idx & 0x1) != 0 {
        order &= 0xf;
        order |= prev_order & 0xf0;
    } else {
        order <<= 4;
        order |= prev_order & 0xf;
    }

    (*chunk).orders[(idx / 2) as usize] = order;

    0
}

unsafe fn idx_get_order(chunk: *mut buddy_chunk, idx: u64) -> u8 {
    let result: u8;

    // Static assertion from C: BUDDY_CHUNK_NUM_ORDERS <= 16, order must fit in 4 bits.

    if idx >= BUDDY_CHUNK_ITEMS {
        arena_stderr(b"getting order of invalid idx %u\n\0".as_ptr(), idx);
        return BUDDY_CHUNK_NUM_ORDERS as u8;
    }

    result = (*chunk).orders[(idx / 2) as usize];

    if (idx & 0x1) != 0 {
        result & 0xf
    } else {
        result >> 4
    }
}

unsafe fn idx_to_addr(chunk: *mut buddy_chunk, idx: usize) -> *mut c_void {
    let address: u64;

    if (idx as u64) >= BUDDY_CHUNK_ITEMS {
        arena_stderr(b"translating invalid idx %u\n\0".as_ptr(), idx);
        return ptr::null_mut();
    }

    /*
     * The data blocks start in the chunk after the metadata block.
     * We find the actual address by indexing into the region at an
     * BUDDY_MIN_ALLOC_BYTES granularity, the minimum allowed.
     * The index number already accounts for the fact that the first
     * blocks in the chunk are occupied by the metadata, so we do
     * not need to offset it.
     */

    address = (chunk as u64).wrapping_add((idx as u64).wrapping_mul(BUDDY_MIN_ALLOC_BYTES));

    address as *mut c_void
}

unsafe fn idx_to_header(chunk: *mut buddy_chunk, idx: usize) -> *mut buddy_header {
    let mut allocated = false;
    let address: u64;

    if idx_is_allocated(chunk, idx as u64, &mut allocated) != 0 {
        arena_stderr(b"accessing invalid idx 0x%lx\n\0".as_ptr(), idx);
        return ptr::null_mut();
    }

    if allocated {
        arena_stderr(b"accessing allocated idx 0x%lx as header\n\0".as_ptr(), idx);
        return ptr::null_mut();
    }

    address = idx_to_addr(chunk, idx) as u64;
    if address == 0 {
        return ptr::null_mut();
    }

    /*
     * Offset the header within the block. This avoids accidental overwrites
     * to the header because of off-by-one errors when using adjacent blocks.
     *
     * The offset has been chosen as a compromise between ASAN effectiveness
     * and allocator granularity:
     * 1) ASAN dictates valid data runs are 8-byte aligned.
     * 2) We want to keep a low minimum allocation size (currently 16).
     *
     * As a result, we have only two possible positions for the header: Bytes
     * 0 and 8. Keeping the header in byte 0 means off-by-ones from the previous
     * block touch the header, and, since the header must be accessible, ASAN
     * will not trigger. Keeping the header on byte 8 means off-by-one errors from
     * the previous block are caught by ASAN. Negative offsets are rarer, so
     * while accesses into the block from the next block are possible, they are
     * less probable.
     */

    address.wrapping_add(BUDDY_HEADER_OFF) as *mut buddy_header
}

unsafe fn header_add_freelist(
    chunk: *mut buddy_chunk,
    header: *mut buddy_header,
    idx: u64,
    order: u8,
) {
    let tmp_header: *mut buddy_header;

    idx_set_order(chunk, idx, order);

    (*header).next_index = (*chunk).freelists[order as usize];
    (*header).prev_index = BUDDY_CHUNK_ITEMS;

    if (*header).next_index != BUDDY_CHUNK_ITEMS {
        tmp_header = idx_to_header(chunk, (*header).next_index as usize);
        (*tmp_header).prev_index = idx;
    }

    (*chunk).freelists[order as usize] = idx;
}

unsafe fn header_remove_freelist(
    chunk: *mut buddy_chunk,
    header: *mut buddy_header,
    order: u8,
) {
    let tmp_header: *mut buddy_header;

    if (*header).prev_index != BUDDY_CHUNK_ITEMS {
        tmp_header = idx_to_header(chunk, (*header).prev_index as usize);
        (*tmp_header).next_index = (*header).next_index;
    }

    if (*header).next_index != BUDDY_CHUNK_ITEMS {
        tmp_header = idx_to_header(chunk, (*header).next_index as usize);
        (*tmp_header).prev_index = (*header).prev_index;
    }

    /* Pop off the list head if necessary. */
    if idx_to_header(chunk, (*chunk).freelists[order as usize] as usize) == header {
        (*chunk).freelists[order as usize] = (*header).next_index;
    }

    (*header).prev_index = BUDDY_CHUNK_ITEMS;
    (*header).next_index = BUDDY_CHUNK_ITEMS;
}

unsafe fn size_to_order(size: usize) -> u64 {
    let order: u64;

    /*
     * Legal sizes are [1, 4GiB] (the biggest possible arena).
     * Of course, sizes close to GiB are practically impossible
     * to fulfill and allocation will fail, but that's taken care
     * of by the caller.
     */

    if size == 0 || size > (1usize << 32) {
        arena_stderr(b"illegal size request %lu\n\0".as_ptr(), size);
        return 64;
    }
    /*
     * To find the order of the allocation we find the first power of two
     * >= the requested size, take the log2, then adjust it for the minimum
     * allocation size by removing the minimum shift from it. Requests
     * smaller than the minimum allocation size are rounded up.
     */
    order = (arena_fls(arena_next_pow2(size as u64)) - 1) as u64;
    if order < BUDDY_MIN_ALLOC_SHIFT {
        return 0;
    }

    order - BUDDY_MIN_ALLOC_SHIFT
}

#[no_mangle]
pub unsafe extern "C" fn add_leftovers_to_freelist(
    chunk: *mut buddy_chunk,
    cur_idx: u32,
    min_order: u64,
    max_order: u64,
) -> i32 {
    let mut header: *mut buddy_header;
    let mut ord: u64;
    let mut idx: u32;

    ord = min_order;
    while ord < max_order && can_loop {
        /* Mark the buddy as free and add it to the freelists. */
        idx = cur_idx.wrapping_add(1u32 << ord as u32);

        header = idx_to_header(chunk, idx as usize);
        if header.is_null() {
            arena_stderr(b"idx %u has no header\0".as_ptr(), idx);
            return -EINVAL;
        }

        asan_unpoison(header as *mut c_void, size_of::<buddy_header>() as u64);

        header_add_freelist(chunk, header, idx as u64, ord as u8);
        ord = ord.wrapping_add(1);
    }

    0
}

unsafe fn buddy_chunk_get(buddy: *mut buddy) -> *mut buddy_chunk {
    let mut order: u64;
    let mut ord: u64;
    let mut min_order: u64;
    let mut max_order: u64;
    let chunk: *mut buddy_chunk;
    let mut left: usize;
    let power2: i32;
    let mut vaddr: u64 = 0;
    let mut idx: u32;
    let ret: i32;

    /*
     * Step 1:  Allocate a properly aligned chunk, and
     * prep it for insertion into the buddy allocator.
     * We don't need the allocator lock until step 2.
     */

    ret = buddy_alloc_arena_vaddr(buddy, &mut vaddr);
    if ret != 0 {
        return ptr::null_mut();
    }

    /* Addresses must be aligned to the chunk boundary. */
    if vaddr % BUDDY_CHUNK_BYTES != 0 {
        return ptr::null_mut();
    }

    /* Unreserve the address space. */
    bpf_arena_free_pages(&mut arena, vaddr as *mut c_void, BUDDY_CHUNK_PAGES);

    chunk = bpf_arena_alloc_pages(
        &mut arena,
        vaddr as *mut c_void,
        BUDDY_CHUNK_PAGES,
        NUMA_NO_NODE,
        0,
    );
    if chunk.is_null() {
        arena_stderr(b"[ALLOC FAILED]\0".as_ptr());
        return ptr::null_mut();
    }

    if buddy_lock(buddy) != 0 {
        /*
         * We cannot reclaim the vaddr space, but that is ok - this
         * operation should always succeed. The error path is to catch
         * accidental deadlocks that will cause -ENOMEMs to the program as
         * the allocator fails to refill itself, in which case vaddr usage
         * is the least of our worries.
         */
        bpf_arena_free_pages(&mut arena, vaddr as *mut c_void, BUDDY_CHUNK_PAGES);
        return ptr::null_mut();
    }

    asan_poison(
        chunk as *mut c_void,
        BUDDY_POISONED,
        BUDDY_CHUNK_PAGES * __PAGE_SIZE,
    );

    /* Unpoison the chunk itself. */
    asan_unpoison(chunk as *mut c_void, size_of::<buddy_chunk>() as u64);

    /* Mark all freelists as empty. */
    ord = zero;
    while ord < BUDDY_CHUNK_NUM_ORDERS && can_loop {
        (*chunk).freelists[ord as usize] = BUDDY_CHUNK_ITEMS;
        ord = ord.wrapping_add(1);
    }

    /*
     * Initialize the chunk by carving out a page range to hold the metadata
     * struct above, then dumping the rest of the pages into the allocator.
     */

    // Static assertion from C: BUDDY_CHUNK_PAGES * __PAGE_SIZE >=
    // BUDDY_MIN_ALLOC_BYTES * BUDDY_CHUNK_ITEMS.

    /*
     * Step 2: Reserve a chunk for the chunk metadata, then breaks
     * the rest of the full allocation into the different buckets.
     * We allocating the memory by grabbing blocks of progressively
     * smaller sizes from the allocator, which are guaranteed to be
     * continuous.
     *
     * This operation also populates the allocator.
     *
     * Algorithm:
     *
     * - max_order: The last order allocation we made
     * - left: How many bytes are left to allocate
     * - cur_index: Current index into the top-level block we are
     * allocating from.
     *
     * Step 3:
     * - Find the largest power-of-2 allocation still smaller than left (infimum)
     * - Reserve a chunk of that size, along with its buddy
     * - For every order from [infimum + 1, last order), carve out a block
     *   and put it into the allocator.
     *
     *  Example: Chunk size 0b1010000 (80 bytes)
     *
     *  Step 1:
     *
     *   idx  infimum                             1 << max_order
     *   0        64        128                    1 << 20
     *   |________|_________|______________________|
     *
     *   Blocks set aside:
     *   	[0, 64)         - Completely allocated
     *   	[64, 128)       - Will be further split in the next iteration
     *
     *   Blocks added to the allocator:
     *   	[128, 256)
     *   	[256, 512)
     *   	...
     *   	[1 << 18, 1 << 19)
     *   	[1 << 19, 1 << 20)
     *
     *  Step 2:
     *
     *   idx  infimum			   idx + 1 << max_order
     *   64	      80	96		   	64 + 1 << 6 = 128
     *   |________|_________|______________________|
     *
     *   Blocks set aside:
     *   	[64, 80)	- Completely allocated
     *
     *   Blocks added to the allocator:
     *      [80, 96) - left == 0 so the buddy is unused and marked as freed
     *   	[96, 128)
     */
    max_order = BUDDY_CHUNK_NUM_ORDERS;
    left = size_of::<buddy_chunk>();
    idx = 0;
    while left != 0 && can_loop {
        power2 = arena_fls(left as u64) - 1;
        /*
         * Note: The condition below only triggers to catch serious bugs
         * early. There is no sane way to undo any block insertions from
         * the allocated chunk, so just leak any leftover allocations,
         * emit a diagnostic, unlock and exit.
         *
         */
        if (power2 as u64) >= BUDDY_CHUNK_NUM_ORDERS {
            arena_stderr(
                b"buddy chunk metadata require allocation of order %d\n\0".as_ptr(),
                power2,
            );
            arena_stderr(
                b"chunk has size of 0x%lx bytes (left %lx bytes)\n\0".as_ptr(),
                size_of::<buddy_chunk>(),
                left,
            );
            buddy_unlock(buddy);

            return ptr::null_mut();
        }

        /* Round up allocations that are too small. */

        left = left.wrapping_sub(if (power2 as u64) >= BUDDY_MIN_ALLOC_SHIFT {
            1usize << power2 as u32
        } else {
            left
        });
        order = if (power2 as u64) >= BUDDY_MIN_ALLOC_SHIFT {
            power2 as u64 - BUDDY_MIN_ALLOC_SHIFT
        } else {
            0
        };

        if idx_set_allocated(chunk, idx as u64, true) != 0 {
            buddy_unlock(buddy);
            return ptr::null_mut();
        }

        /*
         * Starting an order above the one we allocated, populate
         * the allocator with free blocks. If this is the last
         * allocation (left == 0), also mark the buddy as free.
         *
         * See comment above about error handling: The error path
         * is only there as a way to mitigate deeply buggy allocator
         * states by emitting a diagnostic in add_leftovers_to_freelist()
         * and leaking any memory not added in the freelists.
         */
        min_order = if left != 0 { order + 1 } else { order };
        if add_leftovers_to_freelist(chunk, idx, min_order, max_order) != 0 {
            buddy_unlock(buddy);
            return ptr::null_mut();
        }

        /* Adjust the index. */
        idx = idx.wrapping_add(1u32 << order as u32);
        max_order = order;
    }

    buddy_unlock(buddy);

    chunk
}

#[no_mangle]
pub unsafe extern "C" fn buddy_init(buddy: *mut buddy) -> i32 {
    let chunk: *mut buddy_chunk;
    let ret: i32;

    if !asan_ready() {
        return -EINVAL;
    }

    /* Reserve enough address space to ensure allocations are aligned. */
    ret = buddy_reserve_arena_vaddr(buddy);
    if ret != 0 {
        return ret;
    }

    // Static assertion from C: BUDDY_CHUNK_PAGES > 0.

    chunk = buddy_chunk_get(buddy);

    if buddy_lock(buddy) != 0 {
        bpf_arena_free_pages(&mut arena, chunk as *mut c_void, BUDDY_CHUNK_PAGES);
        return -EINVAL;
    }

    /* Chunk is already properly unpoisoned if allocated. */
    if !chunk.is_null() {
        (*chunk).next = (*buddy).first_chunk;
    }

    /* Put the chunk at the beginning of the list. */
    (*buddy).first_chunk = chunk;

    buddy_unlock(buddy);

    if !chunk.is_null() {
        0
    } else {
        -ENOMEM
    }
}

/*
 * Destroy the allocator. This does not check whether there are any allocations
 * currently in use, so any pages being accessed will start taking arena faults.
 * We do not take a lock because we are freeing arena pages, and nobody should
 * be using the allocator at that point in the execution.
 */
#[no_mangle]
pub unsafe extern "C" fn buddy_destroy(buddy: *mut buddy) -> i32 {
    let mut chunk: *mut buddy_chunk;
    let mut next: *mut buddy_chunk;

    if buddy.is_null() {
        return -EINVAL;
    }

    /*
     * Traverse all buddy chunks and free them back to the arena
     * with the same granularity they were allocated with.
     */
    chunk = (*buddy).first_chunk;
    while !chunk.is_null() && can_loop {
        next = (*chunk).next;

        /* Wholesale poison the entire block. */
        asan_poison(
            chunk as *mut c_void,
            BUDDY_POISONED,
            BUDDY_CHUNK_PAGES * __PAGE_SIZE,
        );
        bpf_arena_free_pages(&mut arena, chunk as *mut c_void, BUDDY_CHUNK_PAGES);
        chunk = next;
    }

    /* Free up any part of the address space that did not get used. */
    buddy_unreserve_arena_vaddr(buddy);

    /* Clear all fields. */
    (*buddy).first_chunk = ptr::null_mut();

    0
}

#[no_mangle]
pub unsafe extern "C" fn buddy_chunk_alloc(chunk: *mut buddy_chunk, order_req: i32) -> u64 {
    let mut header: *mut buddy_header;
    let mut tmp_header: *mut buddy_header;
    let mut next_header: *mut buddy_header;
    let mut idx: u32;
    let tmpidx: u32;
    let retidx: u32;
    let address: u64;
    let mut order: u64 = 0;
    let mut i: u64;

    order = order_req as u64;
    while order < BUDDY_CHUNK_NUM_ORDERS && can_loop {
        if (*chunk).freelists[order as usize] != BUDDY_CHUNK_ITEMS {
            break;
        }
        order = order.wrapping_add(1);
    }

    if order >= BUDDY_CHUNK_NUM_ORDERS {
        return ptr::null::<c_void>() as u64;
    }

    retidx = (*chunk).freelists[order as usize] as u32;
    header = idx_to_header(chunk, retidx as usize);
    if header.is_null() {
        return ptr::null::<c_void>() as u64;
    }

    (*chunk).freelists[order as usize] = (*header).next_index;

    if (*header).next_index != BUDDY_CHUNK_ITEMS {
        next_header = idx_to_header(chunk, (*header).next_index as usize);
        (*next_header).prev_index = BUDDY_CHUNK_ITEMS;
    }

    (*header).prev_index = BUDDY_CHUNK_ITEMS;
    (*header).next_index = BUDDY_CHUNK_ITEMS;
    if idx_set_order(chunk, retidx as u64, order_req as u8) != 0 {
        return ptr::null::<c_void>() as u64;
    }

    if idx_set_allocated(chunk, retidx as u64, true) != 0 {
        return ptr::null::<c_void>() as u64;
    }

    /*
     * Do not unpoison the address yet, will be done by the caller
     * because the caller has the exact allocation size requested.
     */
    address = idx_to_addr(chunk, retidx as usize) as u64;
    if address == 0 {
        return ptr::null::<c_void>() as u64;
    }

    /* If we allocated from a larger-order chunk, split the buddies. */
    i = order_req as u64;
    while i < order && can_loop {
        /*
         * Flip the bit for the current order (the bit is guaranteed
         * to be 0, so just add 1 << i).
         */
        idx = retidx.wrapping_add(1u32 << i as u32);

        /* Add the buddy of the allocation to the free list. */
        header = idx_to_header(chunk, idx as usize);
        /* Unpoison the buddy header */
        asan_unpoison(header as *mut c_void, size_of::<buddy_header>() as u64);

        if idx_set_order(chunk, idx as u64, i as u8) != 0 {
            return ptr::null::<c_void>() as u64;
        }

        /* Push the header to the beginning of the freelists list. */
        tmpidx = (*chunk).freelists[i as usize] as u32;

        (*header).prev_index = BUDDY_CHUNK_ITEMS;
        (*header).next_index = tmpidx as u64;

        if tmpidx as u64 != BUDDY_CHUNK_ITEMS {
            tmp_header = idx_to_header(chunk, tmpidx as usize);
            (*tmp_header).prev_index = idx as u64;
        }

        (*chunk).freelists[i as usize] = idx as u64;
        i = i.wrapping_add(1);
    }

    address
}

/* Scan the existing chunks for available memory. */
unsafe fn buddy_alloc_from_existing_chunks(buddy: *mut buddy, order: i32) -> u64 {
    let mut chunk: *mut buddy_chunk;
    let mut address: u64;

    chunk = (*buddy).first_chunk;
    while !chunk.is_null() && can_loop {
        address = buddy_chunk_alloc(chunk, order);
        if address != 0 {
            return address;
        }
        chunk = (*chunk).next;
    }

    ptr::null::<c_void>() as u64
}

/*
 * Try an allocation from a newly allocated chunk. Also
 * incorporate the chunk into the linked list.
 */
unsafe fn buddy_alloc_from_new_chunk(
    buddy: *mut buddy,
    chunk: *mut buddy_chunk,
    order: i32,
) -> u64 {
    let address: u64;

    if buddy_lock(buddy) != 0 {
        return ptr::null::<c_void>() as u64;
    }

    /*
     * Add the chunk into the allocator and try
     * to allocate specifically from that chunk.
     */
    (*chunk).next = (*buddy).first_chunk;
    (*buddy).first_chunk = chunk;

    address = buddy_chunk_alloc((*buddy).first_chunk, order);

    buddy_unlock(buddy);

    address
}

#[no_mangle]
pub unsafe extern "C" fn buddy_alloc(buddy: *mut buddy, size: usize) -> *mut c_void {
    let mut address: *mut c_void = ptr::null_mut();
    let chunk: *mut buddy_chunk;
    let order: i32;

    if buddy.is_null() {
        return ptr::null_mut();
    }

    order = size_to_order(size) as i32;
    if (order as u64) >= BUDDY_CHUNK_NUM_ORDERS || order < 0 {
        arena_stderr(b"invalid order %d (sz %lu)\n\0".as_ptr(), order, size);
        return ptr::null_mut();
    }

    if buddy_lock(buddy) != 0 {
        return ptr::null_mut();
    }

    address = buddy_alloc_from_existing_chunks(buddy, order) as *mut c_void;
    buddy_unlock(buddy);
    if !address.is_null() {
        // goto done
    } else {
        /* Get a new chunk. */
        chunk = buddy_chunk_get(buddy);
        if !chunk.is_null() {
            address = buddy_alloc_from_new_chunk(buddy, chunk, order) as *mut c_void;
        }
    }

    /* If we failed to allocate memory, return NULL. */
    if address.is_null() {
        return ptr::null_mut();
    }

    /*
     * Unpoison exactly the amount of bytes requested. If the
     * data is smaller than the header, we must poison any
     * unused bytes that were part of the header.
     */
    if (size as u64) < BUDDY_HEADER_OFF + size_of::<buddy_header>() as u64 {
        asan_poison(
            (address as u64).wrapping_add(BUDDY_HEADER_OFF) as *mut c_void,
            BUDDY_POISONED,
            size_of::<buddy_header>() as u64,
        );
    }

    asan_unpoison(address, size as u64);

    address
}

#[inline(always)]
unsafe fn buddy_free_unlocked(buddy: *mut buddy, addr: u64) -> i32 {
    let mut header: *mut buddy_header;
    let mut buddy_header: *mut buddy_header;
    let mut idx: u64;
    let mut buddy_idx: u64;
    let tmp_idx: u64;
    let chunk: *mut buddy_chunk;
    let mut allocated = false;
    let mut order: u8;
    let ret: i32;

    if buddy.is_null() {
        return -EINVAL;
    }

    if (addr & (BUDDY_MIN_ALLOC_BYTES - 1)) != 0 {
        arena_stderr(b"Freeing unaligned address %llx\n\0".as_ptr(), addr);
        return -EINVAL;
    }

    /* Get (chunk, idx) out of the address. */
    chunk = (addr & !BUDDY_CHUNK_OFFSET_MASK) as *mut buddy_chunk;
    idx = (addr & BUDDY_CHUNK_OFFSET_MASK) / BUDDY_MIN_ALLOC_BYTES;

    /* Mark the block as unallocated so we can access the header. */
    ret = idx_set_allocated(chunk, idx, false);
    if ret != 0 {
        return ret;
    }

    order = idx_get_order(chunk, idx);
    header = idx_to_header(chunk, idx as usize);

    /* The header is in the block itself, keep it unpoisoned. */
    asan_poison(
        addr as *mut c_void,
        BUDDY_POISONED,
        BUDDY_MIN_ALLOC_BYTES << order as u32,
    );
    asan_unpoison(header as *mut c_void, size_of::<buddy_header>() as u64);

    /*
     * Coalescing loop. Merge with free buddies of equal order.
     * For every coalescing step, keep the left buddy and
     * drop the right buddy's header.
     */
    while (order as u64) < BUDDY_CHUNK_NUM_ORDERS && can_loop {
        buddy_idx = idx ^ (1u64 << order as u32);

        /* Check if the buddy is actually free. */
        idx_is_allocated(chunk, buddy_idx, &mut allocated);
        if allocated {
            break;
        }

        /*
         * If buddy is not the same order as the chunk
         * being freed, then we're done coalescing.
         */
        if idx_get_order(chunk, buddy_idx) != order {
            break;
        }

        buddy_header = idx_to_header(chunk, buddy_idx as usize);
        header_remove_freelist(chunk, buddy_header, order);

        /* Keep the left header out of the two buddies, drop the other one. */
        if buddy_idx < idx {
            tmp_idx = idx;
            idx = buddy_idx;
            buddy_idx = tmp_idx;
        }

        /* Remove the buddy from the freelists so that we can merge it. */
        idx_set_order(chunk, buddy_idx, order);

        buddy_header = idx_to_header(chunk, buddy_idx as usize);
        asan_poison(
            buddy_header as *mut c_void,
            BUDDY_POISONED,
            size_of::<buddy_header>() as u64,
        );
        order = order.wrapping_add(1);
    }

    /* Header properly freed but not in any freelists yet .*/
    idx_set_order(chunk, idx, order);

    header = idx_to_header(chunk, idx as usize);
    header_add_freelist(chunk, header, idx, order);

    0
}

#[no_mangle]
pub unsafe extern "C" fn buddy_free(buddy: *mut buddy, addr: *mut c_void) -> i32 {
    let mut ret: i32;

    if buddy.is_null() {
        return -EINVAL;
    }

    /* Freeing NULL is a valid no-op. */
    if addr.is_null() {
        return 0;
    }

    ret = buddy_lock(buddy);
    if ret != 0 {
        return ret;
    }

    ret = buddy_free_unlocked(buddy, addr as u64);

    buddy_unlock(buddy);

    ret
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
