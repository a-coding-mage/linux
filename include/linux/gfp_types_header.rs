/* SPDX-License-Identifier: GPL-2.0 */

/* The typedef is in types.h but the documentation belongs here. */
/*
 * typedef unsigned int __bitwise gfp_t;
 *
 * GFP flags are memory allocation flags; the GFP acronym stands for
 * get_free_pages().
 */

/* In case of changes, also update include/trace/events/mmflags.h and
 * tools/perf/builtin-kmem.c. */

const fn bit(n: u32) -> u32 { 1u32 << n }

pub const ___GFP_DMA_BIT: u32 = 0;
pub const ___GFP_HIGHMEM_BIT: u32 = 1;
pub const ___GFP_DMA32_BIT: u32 = 2;
pub const ___GFP_MOVABLE_BIT: u32 = 3;
pub const ___GFP_RECLAIMABLE_BIT: u32 = 4;
pub const ___GFP_HIGH_BIT: u32 = 5;
pub const ___GFP_IO_BIT: u32 = 6;
pub const ___GFP_FS_BIT: u32 = 7;
pub const ___GFP_ZERO_BIT: u32 = 8;
pub const ___GFP_UNUSED_BIT: u32 = 9; /* 0x200u unused */
pub const ___GFP_DIRECT_RECLAIM_BIT: u32 = 10;
pub const ___GFP_KSWAPD_RECLAIM_BIT: u32 = 11;
pub const ___GFP_WRITE_BIT: u32 = 12;
pub const ___GFP_NOWARN_BIT: u32 = 13;
pub const ___GFP_RETRY_MAYFAIL_BIT: u32 = 14;
pub const ___GFP_NOFAIL_BIT: u32 = 15;
pub const ___GFP_NORETRY_BIT: u32 = 16;
pub const ___GFP_MEMALLOC_BIT: u32 = 17;
pub const ___GFP_COMP_BIT: u32 = 18;
pub const ___GFP_NOMEMALLOC_BIT: u32 = 19;
pub const ___GFP_HARDWALL_BIT: u32 = 20;
pub const ___GFP_THISNODE_BIT: u32 = 21;
pub const ___GFP_ACCOUNT_BIT: u32 = 22;
pub const ___GFP_ZEROTAGS_BIT: u32 = 23;
/* CONFIG_KASAN_HW_TAGS conditionally adds bits 24 and 25. */
#[cfg(CONFIG_KASAN_HW_TAGS)]
pub const ___GFP_SKIP_ZERO_BIT: u32 = 24;
#[cfg(CONFIG_KASAN_HW_TAGS)]
pub const ___GFP_SKIP_KASAN_BIT: u32 = 25;
/* CONFIG_LOCKDEP conditionally adds the next bit. */
#[cfg(CONFIG_LOCKDEP)]
pub const ___GFP_NOLOCKDEP_BIT: u32 = 26;
#[cfg(all(CONFIG_KASAN_HW_TAGS, CONFIG_LOCKDEP))]
pub const ___GFP_LAST_BIT: u32 = 27;
#[cfg(all(CONFIG_KASAN_HW_TAGS, not(CONFIG_LOCKDEP)))]
pub const ___GFP_LAST_BIT: u32 = 26;
#[cfg(all(not(CONFIG_KASAN_HW_TAGS), CONFIG_LOCKDEP))]
pub const ___GFP_LAST_BIT: u32 = 26;
#[cfg(all(not(CONFIG_KASAN_HW_TAGS), not(CONFIG_LOCKDEP)))]
pub const ___GFP_LAST_BIT: u32 = 24;

pub const ___GFP_DMA: u32 = bit(___GFP_DMA_BIT);
pub const ___GFP_HIGHMEM: u32 = bit(___GFP_HIGHMEM_BIT);
pub const ___GFP_DMA32: u32 = bit(___GFP_DMA32_BIT);
pub const ___GFP_MOVABLE: u32 = bit(___GFP_MOVABLE_BIT);
pub const ___GFP_RECLAIMABLE: u32 = bit(___GFP_RECLAIMABLE_BIT);
pub const ___GFP_HIGH: u32 = bit(___GFP_HIGH_BIT);
pub const ___GFP_IO: u32 = bit(___GFP_IO_BIT);
pub const ___GFP_FS: u32 = bit(___GFP_FS_BIT);
pub const ___GFP_ZERO: u32 = bit(___GFP_ZERO_BIT);
pub const ___GFP_DIRECT_RECLAIM: u32 = bit(___GFP_DIRECT_RECLAIM_BIT);
pub const ___GFP_KSWAPD_RECLAIM: u32 = bit(___GFP_KSWAPD_RECLAIM_BIT);
pub const ___GFP_WRITE: u32 = bit(___GFP_WRITE_BIT);
pub const ___GFP_NOWARN: u32 = bit(___GFP_NOWARN_BIT);
pub const ___GFP_RETRY_MAYFAIL: u32 = bit(___GFP_RETRY_MAYFAIL_BIT);
pub const ___GFP_NOFAIL: u32 = bit(___GFP_NOFAIL_BIT);
pub const ___GFP_NORETRY: u32 = bit(___GFP_NORETRY_BIT);
pub const ___GFP_MEMALLOC: u32 = bit(___GFP_MEMALLOC_BIT);
pub const ___GFP_COMP: u32 = bit(___GFP_COMP_BIT);
pub const ___GFP_NOMEMALLOC: u32 = bit(___GFP_NOMEMALLOC_BIT);
pub const ___GFP_HARDWALL: u32 = bit(___GFP_HARDWALL_BIT);
pub const ___GFP_THISNODE: u32 = bit(___GFP_THISNODE_BIT);
pub const ___GFP_ACCOUNT: u32 = bit(___GFP_ACCOUNT_BIT);
pub const ___GFP_ZEROTAGS: u32 = bit(___GFP_ZEROTAGS_BIT);
#[cfg(CONFIG_KASAN_HW_TAGS)]
pub const ___GFP_SKIP_ZERO: u32 = bit(___GFP_SKIP_ZERO_BIT);
#[cfg(not(CONFIG_KASAN_HW_TAGS))]
pub const ___GFP_SKIP_ZERO: u32 = 0;
#[cfg(CONFIG_KASAN_HW_TAGS)]
pub const ___GFP_SKIP_KASAN: u32 = bit(___GFP_SKIP_KASAN_BIT);
#[cfg(not(CONFIG_KASAN_HW_TAGS))]
pub const ___GFP_SKIP_KASAN: u32 = 0;
#[cfg(CONFIG_LOCKDEP)]
pub const ___GFP_NOLOCKDEP: u32 = bit(___GFP_NOLOCKDEP_BIT);
#[cfg(not(CONFIG_LOCKDEP))]
pub const ___GFP_NOLOCKDEP: u32 = 0;

/* Physical address zone modifiers; the casts model C's __force gfp_t casts. */
pub const __GFP_DMA: u32 = ___GFP_DMA;
pub const __GFP_HIGHMEM: u32 = ___GFP_HIGHMEM;
pub const __GFP_DMA32: u32 = ___GFP_DMA32;
pub const __GFP_MOVABLE: u32 = ___GFP_MOVABLE;
pub const GFP_ZONEMASK: u32 = __GFP_DMA | __GFP_HIGHMEM | __GFP_DMA32 | __GFP_MOVABLE;

pub const __GFP_RECLAIMABLE: u32 = ___GFP_RECLAIMABLE;
pub const __GFP_WRITE: u32 = ___GFP_WRITE;
pub const __GFP_HARDWALL: u32 = ___GFP_HARDWALL;
pub const __GFP_THISNODE: u32 = ___GFP_THISNODE;
pub const __GFP_ACCOUNT: u32 = ___GFP_ACCOUNT;

pub const __GFP_HIGH: u32 = ___GFP_HIGH;
pub const __GFP_MEMALLOC: u32 = ___GFP_MEMALLOC;
pub const __GFP_NOMEMALLOC: u32 = ___GFP_NOMEMALLOC;

pub const __GFP_IO: u32 = ___GFP_IO;
pub const __GFP_FS: u32 = ___GFP_FS;
pub const __GFP_DIRECT_RECLAIM: u32 = ___GFP_DIRECT_RECLAIM;
pub const __GFP_KSWAPD_RECLAIM: u32 = ___GFP_KSWAPD_RECLAIM;
pub const __GFP_RECLAIM: u32 = ___GFP_DIRECT_RECLAIM | ___GFP_KSWAPD_RECLAIM;
pub const __GFP_RETRY_MAYFAIL: u32 = ___GFP_RETRY_MAYFAIL;
pub const __GFP_NOFAIL: u32 = ___GFP_NOFAIL;
pub const __GFP_NORETRY: u32 = ___GFP_NORETRY;

pub const __GFP_NOWARN: u32 = ___GFP_NOWARN;
pub const __GFP_COMP: u32 = ___GFP_COMP;
pub const __GFP_ZERO: u32 = ___GFP_ZERO;
pub const __GFP_ZEROTAGS: u32 = ___GFP_ZEROTAGS;
pub const __GFP_SKIP_ZERO: u32 = ___GFP_SKIP_ZERO;
pub const __GFP_SKIP_KASAN: u32 = ___GFP_SKIP_KASAN;
pub const __GFP_NOLOCKDEP: u32 = ___GFP_NOLOCKDEP;

pub const __GFP_BITS_SHIFT: u32 = ___GFP_LAST_BIT;
pub const __GFP_BITS_MASK: u32 = (1u32 << __GFP_BITS_SHIFT) - 1;

pub const GFP_ATOMIC: u32 = __GFP_HIGH | __GFP_KSWAPD_RECLAIM;
pub const GFP_KERNEL: u32 = __GFP_RECLAIM | __GFP_IO | __GFP_FS;
pub const GFP_KERNEL_ACCOUNT: u32 = GFP_KERNEL | __GFP_ACCOUNT;
pub const GFP_NOWAIT: u32 = __GFP_KSWAPD_RECLAIM | __GFP_NOWARN;
pub const GFP_NOIO: u32 = __GFP_RECLAIM;
pub const GFP_NOFS: u32 = __GFP_RECLAIM | __GFP_IO;
pub const GFP_USER: u32 = __GFP_RECLAIM | __GFP_IO | __GFP_FS | __GFP_HARDWALL;
pub const GFP_DMA: u32 = __GFP_DMA;
pub const GFP_DMA32: u32 = __GFP_DMA32;
pub const GFP_HIGHUSER: u32 = GFP_USER | __GFP_HIGHMEM;
pub const GFP_HIGHUSER_MOVABLE: u32 = GFP_HIGHUSER | __GFP_MOVABLE | __GFP_SKIP_KASAN;
pub const GFP_TRANSHUGE_LIGHT: u32 =
    (GFP_HIGHUSER_MOVABLE | __GFP_COMP | __GFP_NOMEMALLOC | __GFP_NOWARN) & !__GFP_RECLAIM;
pub const GFP_TRANSHUGE: u32 = GFP_TRANSHUGE_LIGHT | __GFP_DIRECT_RECLAIM;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
