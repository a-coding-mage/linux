/*
 * Compressed RAM block device
 *
 * Copyright (C) 2008, 2009, 2010  Nitin Gupta
 *               2012, 2013 Minchan Kim
 *
 * This code is released using a dual license strategy: BSD/GPL
 * You can choose the licence that better fits your requirements.
 *
 * Released under the terms of 3-clause BSD License
 * Released under the terms of GNU General Public License Version 2.0
 *
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub const SECTORS_PER_PAGE_SHIFT: u32 = PAGE_SHIFT - SECTOR_SHIFT;
pub const SECTORS_PER_PAGE: u32 = 1u32 << SECTORS_PER_PAGE_SHIFT;
pub const ZRAM_LOGICAL_BLOCK_SHIFT: u32 = 12;
pub const ZRAM_LOGICAL_BLOCK_SIZE: u32 = 1u32 << ZRAM_LOGICAL_BLOCK_SHIFT;
pub const ZRAM_SECTOR_PER_LOGICAL_BLOCK: u32 =
    1u32 << (ZRAM_LOGICAL_BLOCK_SHIFT - SECTOR_SHIFT);

/*
 * ZRAM is mainly used for memory efficiency so we want to keep memory
 * footprint small and thus squeeze size and zram pageflags into a flags
 * member. The lower ZRAM_FLAG_SHIFT bits is for object size (excluding
 * header), which cannot be larger than PAGE_SIZE (requiring PAGE_SHIFT
 * bits), the higher bits are for zram_pageflags.
 *
 * We use BUILD_BUG_ON() to make sure that zram pageflags don't overflow.
 */
pub const ZRAM_FLAG_SHIFT: u32 = PAGE_SHIFT + 1;

/* Only 2 bits are allowed for comp priority index */
pub const ZRAM_COMP_PRIORITY_MASK: u32 = 0x3;

/* Flags for zram pages (table[page_no].flags) */
#[repr(u32)]
pub enum ZramPageflags {
    ZRAM_SAME = ZRAM_FLAG_SHIFT, /* Page consists the same element */
    ZRAM_ENTRY_LOCK, /* entry access lock bit */
    ZRAM_WB, /* page is stored on backing_device */
    ZRAM_PP_SLOT, /* Selected for post-processing */
    ZRAM_HUGE, /* Incompressible page */
    ZRAM_IDLE, /* not accessed page since last idle marking */
    ZRAM_INCOMPRESSIBLE, /* none of the algorithms could compress it */
    ZRAM_COMP_PRIORITY_BIT1, /* First bit of comp priority index */
    ZRAM_COMP_PRIORITY_BIT2, /* Second bit of comp priority index */
    __NR_ZRAM_PAGEFLAGS,
}

/* On 64-bit big-endian builds the lock bit is shifted into the upper half. */
#[cfg(all(target_pointer_width = "64", target_endian = "big"))]
pub const ZRAM_ENTRY_LOCK_BIT: u32 = ZRAM_ENTRY_LOCK as u32 + 32;
#[cfg(not(all(target_pointer_width = "64", target_endian = "big")))]
pub const ZRAM_ENTRY_LOCK_BIT: u32 = ZRAM_ENTRY_LOCK as u32;

#[repr(C)]
pub struct ZramTableEntryAttr {
    pub flags: u32,
    #[cfg(CONFIG_ZRAM_TRACK_ENTRY_ACTIME)]
    pub ac_time: u32,
}

#[repr(C)]
pub union ZramTableEntryUnion {
    pub __lock: usize,
    pub attr: ZramTableEntryAttr,
}

/*
 * Allocated for each disk page.  We use bit-lock (ZRAM_ENTRY_LOCK bit
 * of flags) to save memory.  There can be plenty of entries and standard
 * locking primitives (e.g. mutex) will significantly increase sizeof()
 * of each entry and hence of the meta table.
 */
#[repr(C)]
pub struct ZramTableEntry {
    pub handle: usize,
    pub lock_or_attr: ZramTableEntryUnion,
}

#[repr(C)]
pub struct ZramStats {
    pub compr_data_size: atomic64_t, /* compressed size of pages stored */
    pub failed_reads: atomic64_t, /* can happen when memory is too low */
    pub failed_writes: atomic64_t, /* can happen when memory is too low */
    pub notify_free: atomic64_t, /* no. of swap slot free notifications */
    pub same_pages: atomic64_t, /* no. of same element filled pages */
    pub huge_pages: atomic64_t, /* no. of huge pages */
    pub huge_pages_since: atomic64_t, /* no. of huge pages since zram set up */
    pub pages_stored: atomic64_t, /* no. of pages currently stored */
    pub max_used_pages: atomic_long_t, /* no. of maximum pages stored */
    pub miss_free: atomic64_t, /* no. of missed free */
    #[cfg(CONFIG_ZRAM_WRITEBACK)]
    pub bd_count: atomic64_t, /* no. of pages in backing device */
    #[cfg(CONFIG_ZRAM_WRITEBACK)]
    pub bd_reads: atomic64_t, /* no. of reads from backing device */
    #[cfg(CONFIG_ZRAM_WRITEBACK)]
    pub bd_writes: atomic64_t, /* no. of writes from backing device */
}

#[cfg(CONFIG_ZRAM_MULTI_COMP)]
pub const ZRAM_PRIMARY_COMP: u32 = 0;
#[cfg(CONFIG_ZRAM_MULTI_COMP)]
pub const ZRAM_SECONDARY_COMP: u32 = 1;
#[cfg(CONFIG_ZRAM_MULTI_COMP)]
pub const ZRAM_MAX_COMPS: usize = 4;
#[cfg(not(CONFIG_ZRAM_MULTI_COMP))]
pub const ZRAM_PRIMARY_COMP: u32 = 0;
#[cfg(not(CONFIG_ZRAM_MULTI_COMP))]
pub const ZRAM_SECONDARY_COMP: u32 = 0;
#[cfg(not(CONFIG_ZRAM_MULTI_COMP))]
pub const ZRAM_MAX_COMPS: usize = 1;

#[repr(C)]
pub struct Zram {
    pub table: *mut ZramTableEntry,
    pub table_lock_map: lockdep_map,
    pub table_lock_key: lock_class_key,
    pub mem_pool: *mut zs_pool,
    pub comps: [*mut zcomp; ZRAM_MAX_COMPS],
    pub params: [zcomp_params; ZRAM_MAX_COMPS],
    pub disk: *mut gendisk,
    /* Locks the device either in exclusive or in shared mode */
    pub dev_lock: rw_semaphore,
    /* the number of pages zram can consume for storing compressed data */
    pub limit_pages: usize,
    pub stats: ZramStats,
    /* This is the limit on amount of *uncompressed* worth of data
     * we can store in a disk. */
    pub disksize: u64, /* bytes */
    pub comp_algs: [*const core::ffi::c_char; ZRAM_MAX_COMPS],
    /* zram is claimed so open request will be failed */
    pub claim: bool, /* Protected by disk->open_mutex */
    #[cfg(CONFIG_ZRAM_WRITEBACK)]
    pub backing_dev: *mut file,
    #[cfg(CONFIG_ZRAM_WRITEBACK)]
    pub wb_limit_enable: bool,
    #[cfg(CONFIG_ZRAM_WRITEBACK)]
    pub compressed_wb: bool,
    #[cfg(CONFIG_ZRAM_WRITEBACK)]
    pub wb_batch_size: u32,
    #[cfg(CONFIG_ZRAM_WRITEBACK)]
    pub bd_wb_limit: u64,
    #[cfg(CONFIG_ZRAM_WRITEBACK)]
    pub bdev: *mut block_device,
    #[cfg(CONFIG_ZRAM_WRITEBACK)]
    pub bitmap: *mut usize,
    #[cfg(CONFIG_ZRAM_WRITEBACK)]
    pub nr_pages: usize,
    #[cfg(CONFIG_ZRAM_MEMORY_TRACKING)]
    pub debugfs_dir: *mut dentry,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
