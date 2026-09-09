// SPDX-License-Identifier: GPL-2.0

/* Do not include this file directly. */

/*
 * In order to keep track of what pids to trace, a tree is created much
 * like page tables are used. This creates a sparse bit map, where
 * the tree is filled in when needed. A PID is at most 30 bits (see
 * linux/thread.h), and is broken up into 3 sections based on the bit map
 * of the bits. The 8 MSB is the "upper1" section. The next 8 MSB is
 * the "upper2" section and the 14 LSB is the "lower" section.
 *
 * A trace_pid_list structure holds the "upper1" section, in an
 * array of 256 pointers (1 or 2K in size) to "upper_chunk" unions, where
 * each has an array of 256 pointers (1 or 2K in size) to the "lower_chunk"
 * structures, where each has an array of size 2K bytes representing a bitmask
 * of the 14 LSB of the PID (256 * 8 = 2048)
 *
 * When a trace_pid_list is allocated, it includes the 256 pointer array
 * of the upper1 unions. Then a "cache" of upper and lower is allocated
 * where these will be assigned as needed.
 *
 * When a bit is set in the pid_list bitmask, the pid to use has
 * the 8 MSB masked, and this is used to index the array in the
 * pid_list to find the next upper union. If the element is NULL,
 * then one is retrieved from the upper_list cache. If none is
 * available, then -ENOMEM is returned.
 *
 * The next 8 MSB is used to index into the "upper2" section. If this
 * element is NULL, then it is retrieved from the lower_list cache.
 * Again, if one is not available -ENOMEM is returned.
 *
 * Finally the 14 LSB of the PID is used to set the bit in the 16384
 * bitmask (made up of 2K bytes).
 *
 * When the second upper section or the lower section has their last
 * bit cleared, they are added back to the free list to be reused
 * when needed.
 */

pub const UPPER_BITS: usize = 8;
pub const UPPER_MAX: usize = 1usize << UPPER_BITS;
pub const UPPER1_SIZE: usize = 1usize << UPPER_BITS;
pub const UPPER2_SIZE: usize = 1usize << UPPER_BITS;

pub const LOWER_BITS: usize = 14;
pub const LOWER_MAX: usize = 1usize << LOWER_BITS;
pub const LOWER_SIZE: usize = LOWER_MAX / BITS_PER_LONG;

pub const UPPER1_SHIFT: usize = LOWER_BITS + UPPER_BITS;
pub const UPPER2_SHIFT: usize = LOWER_BITS;
pub const LOWER_MASK: usize = LOWER_MAX - 1;

pub const UPPER_MASK: usize = UPPER_MAX - 1;

/* According to linux/thread.h pids can not be bigger than or equal to 1 << 30 */
pub const MAX_PID: usize = 1usize << 30;

/* Just keep 6 chunks of both upper and lower in the cache on alloc */
pub const CHUNK_ALLOC: i32 = 6;

/* Have 2 chunks free, trigger a refill of the cache */
pub const CHUNK_REALLOC: i32 = 2;

#[repr(C)]
pub union lower_chunk {
    pub next: *mut lower_chunk,
    pub data: [::core::ffi::c_ulong; LOWER_SIZE], // 2K in size
}

#[repr(C)]
pub union upper_chunk {
    pub next: *mut upper_chunk,
    pub data: [*mut lower_chunk; UPPER2_SIZE], // 1 or 2K in size
}

#[repr(C)]
pub struct trace_pid_list {
    pub seqcount: seqcount_raw_spinlock_t,
    pub lock: raw_spinlock_t,
    pub refill_irqwork: irq_work,
    pub upper: [*mut upper_chunk; UPPER1_SIZE], // 1 or 2K in size
    pub upper_list: *mut upper_chunk,
    pub lower_list: *mut lower_chunk,
    pub free_upper_chunks: i32,
    pub free_lower_chunks: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
