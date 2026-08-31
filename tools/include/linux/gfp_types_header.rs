/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency intent from C header: #include <linux/bits.h> */

/* The typedef is in types.h but we want the documentation here */
/*
/**
 * typedef gfp_t - Memory allocation flags.
 *
 * GFP flags are commonly used throughout Linux to indicate how memory
 * should be allocated.  The GFP acronym stands for get_free_pages(),
 * the underlying memory allocation function.  Not every GFP flag is
 * supported by every function which may allocate memory.  Most users
 * will want to use a plain ``GFP_KERNEL``.
 */
typedef unsigned int __bitwise gfp_t;
*/

/*
 * In case of changes, please don't forget to update
 * include/trace/events/mmflags.h and tools/perf/builtin-kmem.c
 */

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
/* CONFIG_KASAN_HW_TAGS adds ___GFP_SKIP_ZERO_BIT and ___GFP_SKIP_KASAN_BIT here. */
#[cfg(CONFIG_KASAN_HW_TAGS)]
pub const ___GFP_SKIP_ZERO_BIT: u32 = 24;
#[cfg(CONFIG_KASAN_HW_TAGS)]
pub const ___GFP_SKIP_KASAN_BIT: u32 = 25;
/* CONFIG_LOCKDEP adds ___GFP_NOLOCKDEP_BIT after the preceding entries. */
#[cfg(all(CONFIG_LOCKDEP, CONFIG_KASAN_HW_TAGS))]
pub const ___GFP_NOLOCKDEP_BIT: u32 = 26;
#[cfg(all(CONFIG_LOCKDEP, not(CONFIG_KASAN_HW_TAGS)))]
pub const ___GFP_NOLOCKDEP_BIT: u32 = 24;
#[cfg(all(CONFIG_LOCKDEP, CONFIG_KASAN_HW_TAGS))]
pub const ___GFP_LAST_BIT: u32 = 27;
#[cfg(all(not(CONFIG_LOCKDEP), CONFIG_KASAN_HW_TAGS))]
pub const ___GFP_LAST_BIT: u32 = 26;
#[cfg(all(CONFIG_LOCKDEP, not(CONFIG_KASAN_HW_TAGS)))]
pub const ___GFP_LAST_BIT: u32 = 25;
#[cfg(all(not(CONFIG_LOCKDEP), not(CONFIG_KASAN_HW_TAGS)))]
pub const ___GFP_LAST_BIT: u32 = 24;

/* Plain integer GFP bitmasks. Do not use this directly. */
pub const ___GFP_DMA: u32 = 1u32 << ___GFP_DMA_BIT;
pub const ___GFP_HIGHMEM: u32 = 1u32 << ___GFP_HIGHMEM_BIT;
pub const ___GFP_DMA32: u32 = 1u32 << ___GFP_DMA32_BIT;
pub const ___GFP_MOVABLE: u32 = 1u32 << ___GFP_MOVABLE_BIT;
pub const ___GFP_RECLAIMABLE: u32 = 1u32 << ___GFP_RECLAIMABLE_BIT;
pub const ___GFP_HIGH: u32 = 1u32 << ___GFP_HIGH_BIT;
pub const ___GFP_IO: u32 = 1u32 << ___GFP_IO_BIT;
pub const ___GFP_FS: u32 = 1u32 << ___GFP_FS_BIT;
pub const ___GFP_ZERO: u32 = 1u32 << ___GFP_ZERO_BIT;
/* 0x200u unused */
pub const ___GFP_DIRECT_RECLAIM: u32 = 1u32 << ___GFP_DIRECT_RECLAIM_BIT;
pub const ___GFP_KSWAPD_RECLAIM: u32 = 1u32 << ___GFP_KSWAPD_RECLAIM_BIT;
pub const ___GFP_WRITE: u32 = 1u32 << ___GFP_WRITE_BIT;
pub const ___GFP_NOWARN: u32 = 1u32 << ___GFP_NOWARN_BIT;
pub const ___GFP_RETRY_MAYFAIL: u32 = 1u32 << ___GFP_RETRY_MAYFAIL_BIT;
pub const ___GFP_NOFAIL: u32 = 1u32 << ___GFP_NOFAIL_BIT;
pub const ___GFP_NORETRY: u32 = 1u32 << ___GFP_NORETRY_BIT;
pub const ___GFP_MEMALLOC: u32 = 1u32 << ___GFP_MEMALLOC_BIT;
pub const ___GFP_COMP: u32 = 1u32 << ___GFP_COMP_BIT;
pub const ___GFP_NOMEMALLOC: u32 = 1u32 << ___GFP_NOMEMALLOC_BIT;
pub const ___GFP_HARDWALL: u32 = 1u32 << ___GFP_HARDWALL_BIT;
pub const ___GFP_THISNODE: u32 = 1u32 << ___GFP_THISNODE_BIT;
pub const ___GFP_ACCOUNT: u32 = 1u32 << ___GFP_ACCOUNT_BIT;
pub const ___GFP_ZEROTAGS: u32 = 1u32 << ___GFP_ZEROTAGS_BIT;
#[cfg(CONFIG_KASAN_HW_TAGS)]
pub const ___GFP_SKIP_ZERO: u32 = 1u32 << ___GFP_SKIP_ZERO_BIT;
#[cfg(CONFIG_KASAN_HW_TAGS)]
pub const ___GFP_SKIP_KASAN: u32 = 1u32 << ___GFP_SKIP_KASAN_BIT;
#[cfg(not(CONFIG_KASAN_HW_TAGS))]
pub const ___GFP_SKIP_ZERO: u32 = 0;
#[cfg(not(CONFIG_KASAN_HW_TAGS))]
pub const ___GFP_SKIP_KASAN: u32 = 0;
#[cfg(CONFIG_LOCKDEP)]
pub const ___GFP_NOLOCKDEP: u32 = 1u32 << ___GFP_NOLOCKDEP_BIT;
#[cfg(not(CONFIG_LOCKDEP))]
pub const ___GFP_NOLOCKDEP: u32 = 0;

/*
 * Physical address zone modifiers (see linux/mmzone.h - low four bits)
 *
 * Do not put any conditional on these. If necessary modify the definitions
 * without the underscores and use them consistently. The definitions here may
 * be used in bit comparisons.
 */
pub const __GFP_DMA: gfp_t = ___GFP_DMA as gfp_t;
pub const __GFP_HIGHMEM: gfp_t = ___GFP_HIGHMEM as gfp_t;
pub const __GFP_DMA32: gfp_t = ___GFP_DMA32 as gfp_t;
pub const __GFP_MOVABLE: gfp_t = ___GFP_MOVABLE as gfp_t; /* ZONE_MOVABLE allowed */
pub const GFP_ZONEMASK: gfp_t = __GFP_DMA | __GFP_HIGHMEM | __GFP_DMA32 | __GFP_MOVABLE;

/**
 * DOC: Page mobility and placement hints
 *
 * Page mobility and placement hints
 * ---------------------------------
 *
 * These flags provide hints about how mobile the page is. Pages with similar
 * mobility are placed within the same pageblocks to minimise problems due
 * to external fragmentation.
 *
 * %__GFP_MOVABLE (also a zone modifier) indicates that the page can be
 * moved by page migration during memory compaction or can be reclaimed.
 *
 * %__GFP_RECLAIMABLE is used for slab allocations that specify
 * SLAB_RECLAIM_ACCOUNT and whose pages can be freed via shrinkers.
 *
 * %__GFP_WRITE indicates the caller intends to dirty the page. Where possible,
 * these pages will be spread between local zones to avoid all the dirty
 * pages being in one zone (fair zone allocation policy).
 *
 * %__GFP_HARDWALL enforces the cpuset memory allocation policy.
 *
 * %__GFP_THISNODE forces the allocation to be satisfied from the requested
 * node with no fallbacks or placement policy enforcements.
 *
 * %__GFP_ACCOUNT causes the allocation to be accounted to kmemcg.
 */
pub const __GFP_RECLAIMABLE: gfp_t = ___GFP_RECLAIMABLE as gfp_t;
pub const __GFP_WRITE: gfp_t = ___GFP_WRITE as gfp_t;
pub const __GFP_HARDWALL: gfp_t = ___GFP_HARDWALL as gfp_t;
pub const __GFP_THISNODE: gfp_t = ___GFP_THISNODE as gfp_t;
pub const __GFP_ACCOUNT: gfp_t = ___GFP_ACCOUNT as gfp_t;

/**
 * DOC: Watermark modifiers
 *
 * Watermark modifiers -- controls access to emergency reserves
 * ------------------------------------------------------------
 *
 * %__GFP_HIGH indicates that the caller is high-priority and that granting
 * the request is necessary before the system can make forward progress.
 * For example creating an IO context to clean pages and requests
 * from atomic context.
 *
 * %__GFP_MEMALLOC allows access to all memory. This should only be used when
 * the caller guarantees the allocation will allow more memory to be freed
 * very shortly e.g. process exiting or swapping. Users either should
 * be the MM or co-ordinating closely with the VM (e.g. swap over NFS).
 * Users of this flag have to be extremely careful to not deplete the reserve
 * completely and implement a throttling mechanism which controls the
 * consumption of the reserve based on the amount of freed memory.
 * Usage of a pre-allocated pool (e.g. mempool) should be always considered
 * before using this flag.
 *
 * %__GFP_NOMEMALLOC is used to explicitly forbid access to emergency reserves.
 * This takes precedence over the %__GFP_MEMALLOC flag if both are set.
 */
pub const __GFP_HIGH: gfp_t = ___GFP_HIGH as gfp_t;
pub const __GFP_MEMALLOC: gfp_t = ___GFP_MEMALLOC as gfp_t;
pub const __GFP_NOMEMALLOC: gfp_t = ___GFP_NOMEMALLOC as gfp_t;

/**
 * DOC: Reclaim modifiers
 *
 * Reclaim modifiers
 * -----------------
 * Please note that all the following flags are only applicable to sleepable
 * allocations (e.g. %GFP_NOWAIT and %GFP_ATOMIC will ignore them).
 *
 * %__GFP_IO can start physical IO.
 *
 * %__GFP_FS can call down to the low-level FS. Clearing the flag avoids the
 * allocator recursing into the filesystem which might already be holding
 * locks.
 *
 * %__GFP_DIRECT_RECLAIM indicates that the caller may enter direct reclaim.
 * This flag can be cleared to avoid unnecessary delays when a fallback
 * option is available.
 *
 * %__GFP_KSWAPD_RECLAIM indicates that the caller wants to wake kswapd when
 * the low watermark is reached and have it reclaim pages until the high
 * watermark is reached. A caller may wish to clear this flag when fallback
 * options are available and the reclaim is likely to disrupt the system. The
 * canonical example is THP allocation where a fallback is cheap but
 * reclaim/compaction may cause indirect stalls.
 *
 * %__GFP_RECLAIM is shorthand to allow/forbid both direct and kswapd reclaim.
 *
 * The default allocator behavior depends on the request size. We have a concept
 * of so-called costly allocations (with order > %PAGE_ALLOC_COSTLY_ORDER).
 * !costly allocations are too essential to fail so they are implicitly
 * non-failing by default (with some exceptions like OOM victims might fail so
 * the caller still has to check for failures) while costly requests try to be
 * not disruptive and back off even without invoking the OOM killer.
 * The following three modifiers might be used to override some of these
 * implicit rules. Please note that all of them must be used along with
 * %__GFP_DIRECT_RECLAIM flag.
 *
 * %__GFP_NORETRY: The VM implementation will try only very lightweight
 * memory direct reclaim to get some memory under memory pressure (thus
 * it can sleep). It will avoid disruptive actions like OOM killer. The
 * caller must handle the failure which is quite likely to happen under
 * heavy memory pressure. The flag is suitable when failure can easily be
 * handled at small cost, such as reduced throughput.
 *
 * %__GFP_RETRY_MAYFAIL: The VM implementation will retry memory reclaim
 * procedures that have previously failed if there is some indication
 * that progress has been made elsewhere.  It can wait for other
 * tasks to attempt high-level approaches to freeing memory such as
 * compaction (which removes fragmentation) and page-out.
 * There is still a definite limit to the number of retries, but it is
 * a larger limit than with %__GFP_NORETRY.
 * Allocations with this flag may fail, but only when there is
 * genuinely little unused memory. While these allocations do not
 * directly trigger the OOM killer, their failure indicates that
 * the system is likely to need to use the OOM killer soon.  The
 * caller must handle failure, but can reasonably do so by failing
 * a higher-level request, or completing it only in a much less
 * efficient manner.
 * If the allocation does fail, and the caller is in a position to
 * free some non-essential memory, doing so could benefit the system
 * as a whole.
 *
 * %__GFP_NOFAIL: The VM implementation _must_ retry infinitely: the caller
 * cannot handle allocation failures. The allocation could block
 * indefinitely but will never return with failure. Testing for
 * failure is pointless.
 * It _must_ be blockable and used together with __GFP_DIRECT_RECLAIM.
 * It should _never_ be used in non-sleepable contexts.
 * New users should be evaluated carefully (and the flag should be
 * used only when there is no reasonable failure policy) but it is
 * definitely preferable to use the flag rather than opencode endless
 * loop around allocator.
 * Allocating pages from the buddy with __GFP_NOFAIL and order > 1 is
 * not supported. Please consider using kvmalloc() instead.
 */
pub const __GFP_IO: gfp_t = ___GFP_IO as gfp_t;
pub const __GFP_FS: gfp_t = ___GFP_FS as gfp_t;
pub const __GFP_DIRECT_RECLAIM: gfp_t = ___GFP_DIRECT_RECLAIM as gfp_t; /* Caller can reclaim */
pub const __GFP_KSWAPD_RECLAIM: gfp_t = ___GFP_KSWAPD_RECLAIM as gfp_t; /* kswapd can wake */
pub const __GFP_RECLAIM: gfp_t = (___GFP_DIRECT_RECLAIM | ___GFP_KSWAPD_RECLAIM) as gfp_t;
pub const __GFP_RETRY_MAYFAIL: gfp_t = ___GFP_RETRY_MAYFAIL as gfp_t;
pub const __GFP_NOFAIL: gfp_t = ___GFP_NOFAIL as gfp_t;
pub const __GFP_NORETRY: gfp_t = ___GFP_NORETRY as gfp_t;

/**
 * DOC: Action modifiers
 *
 * Action modifiers
 * ----------------
 *
 * %__GFP_NOWARN suppresses allocation failure reports.
 *
 * %__GFP_COMP address compound page metadata.
 *
 * %__GFP_ZERO returns a zeroed page on success.
 *
 * %__GFP_ZEROTAGS zeroes memory tags at allocation time. Setting memory tags at
 * the same time as zeroing memory (e.g., with __GFP_ZERO) has minimal
 * additional performance impact. However, __GFP_ZEROTAGS also zeroes the tags
 * even if memory is not getting zeroed at allocation time (e.g.,
 * with init_on_free).
 *
 * %__GFP_SKIP_KASAN makes KASAN skip unpoisoning on page allocation.
 * Used for userspace and vmalloc pages; the latter are unpoisoned by
 * kasan_unpoison_vmalloc instead. If passed to vmalloc, kasan_unpoison_vmalloc
 * is skipped too. For userspace pages, results in poisoning being skipped as
 * well, see should_skip_kasan_poison for details. Only effective in HW_TAGS mode.
 */
pub const __GFP_NOWARN: gfp_t = ___GFP_NOWARN as gfp_t;
pub const __GFP_COMP: gfp_t = ___GFP_COMP as gfp_t;
pub const __GFP_ZERO: gfp_t = ___GFP_ZERO as gfp_t;
pub const __GFP_ZEROTAGS: gfp_t = ___GFP_ZEROTAGS as gfp_t;
pub const __GFP_SKIP_ZERO: gfp_t = ___GFP_SKIP_ZERO as gfp_t;
pub const __GFP_SKIP_KASAN: gfp_t = ___GFP_SKIP_KASAN as gfp_t;

/* Disable lockdep for GFP context tracking */
pub const __GFP_NOLOCKDEP: gfp_t = ___GFP_NOLOCKDEP as gfp_t;

/* Room for N __GFP_FOO bits */
pub const __GFP_BITS_SHIFT: u32 = ___GFP_LAST_BIT;
pub const __GFP_BITS_MASK: gfp_t = ((1u32 << __GFP_BITS_SHIFT) - 1) as gfp_t;

/**
 * DOC: Useful GFP flag combinations
 *
 * Useful GFP flag combinations
 * ----------------------------
 *
 * Useful GFP flag combinations that are commonly used. It is recommended
 * that subsystems start with one of these combinations and then set/clear
 * %__GFP_FOO flags as necessary.
 *
 * %GFP_ATOMIC users can not sleep and need the allocation to succeed. A lower
 * watermark is applied to allow access to "atomic reserves".
 * The current implementation doesn't support NMI, nor contexts that disable
 * preemption under PREEMPT_RT. This includes raw_spin_lock() and plain
 * preempt_disable() - see "Memory allocation" in
 * Documentation/core-api/real-time/differences.rst for more info.
 *
 * %GFP_KERNEL is typical for kernel-internal allocations. The caller requires
 * %ZONE_NORMAL or a lower zone for direct access but can direct reclaim.
 *
 * %GFP_KERNEL_ACCOUNT is the same as GFP_KERNEL, except the allocation is
 * accounted to kmemcg.
 *
 * %GFP_NOWAIT is for kernel allocations that should not stall for direct
 * reclaim, start physical IO or use any filesystem callback.  It is very
 * likely to fail to allocate memory, even for very small allocations.
 * The same restrictions on calling contexts apply as for %GFP_ATOMIC.
 *
 * %GFP_NOIO will use direct reclaim to discard clean pages or slab pages
 * that do not require the starting of any physical IO.
 * Please try to avoid using this flag directly and instead use
 * memalloc_noio_{save,restore} to mark the whole scope which cannot
 * perform any IO with a short explanation why. All allocation requests
 * will inherit GFP_NOIO implicitly.
 *
 * %GFP_NOFS will use direct reclaim but will not use any filesystem interfaces.
 * Please try to avoid using this flag directly and instead use
 * memalloc_nofs_{save,restore} to mark the whole scope which cannot/shouldn't
 * recurse into the FS layer with a short explanation why. All allocation
 * requests will inherit GFP_NOFS implicitly.
 *
 * %GFP_USER is for userspace allocations that also need to be directly
 * accessibly by the kernel or hardware. It is typically used by hardware
 * for buffers that are mapped to userspace (e.g. graphics) that hardware
 * still must DMA to. cpuset limits are enforced for these allocations.
 *
 * %GFP_DMA exists for historical reasons and should be avoided where possible.
 * The flags indicates that the caller requires that the lowest zone be
 * used (%ZONE_DMA or 16M on x86-64). Ideally, this would be removed but
 * it would require careful auditing as some users really require it and
 * others use the flag to avoid lowmem reserves in %ZONE_DMA and treat the
 * lowest zone as a type of emergency reserve.
 *
 * %GFP_DMA32 is similar to %GFP_DMA except that the caller requires a 32-bit
 * address. Note that kmalloc(..., GFP_DMA32) does not return DMA32 memory
 * because the DMA32 kmalloc cache array is not implemented.
 * (Reason: there is no such user in kernel).
 *
 * %GFP_HIGHUSER is for userspace allocations that may be mapped to userspace,
 * do not need to be directly accessible by the kernel but that cannot
 * move once in use. An example may be a hardware allocation that maps
 * data directly into userspace but has no addressing limitations.
 *
 * %GFP_HIGHUSER_MOVABLE is for userspace allocations that the kernel does not
 * need direct access to but can use kmap() when access is required. They
 * are expected to be movable via page reclaim or page migration. Typically,
 * pages on the LRU would also be allocated with %GFP_HIGHUSER_MOVABLE.
 *
 * %GFP_TRANSHUGE and %GFP_TRANSHUGE_LIGHT are used for THP allocations. They
 * are compound allocations that will generally fail quickly if memory is not
 * available and will not wake kswapd/kcompactd on failure. The _LIGHT
 * version does not attempt reclaim/compaction at all and is by default used
 * in page fault path, while the non-light is used by khugepaged.
 */
pub const GFP_ATOMIC: gfp_t = __GFP_HIGH | __GFP_KSWAPD_RECLAIM;
pub const GFP_KERNEL: gfp_t = __GFP_RECLAIM | __GFP_IO | __GFP_FS;
pub const GFP_KERNEL_ACCOUNT: gfp_t = GFP_KERNEL | __GFP_ACCOUNT;
pub const GFP_NOWAIT: gfp_t = __GFP_KSWAPD_RECLAIM | __GFP_NOWARN;
pub const GFP_NOIO: gfp_t = __GFP_RECLAIM;
pub const GFP_NOFS: gfp_t = __GFP_RECLAIM | __GFP_IO;
pub const GFP_USER: gfp_t = __GFP_RECLAIM | __GFP_IO | __GFP_FS | __GFP_HARDWALL;
pub const GFP_DMA: gfp_t = __GFP_DMA;
pub const GFP_DMA32: gfp_t = __GFP_DMA32;
pub const GFP_HIGHUSER: gfp_t = GFP_USER | __GFP_HIGHMEM;
pub const GFP_HIGHUSER_MOVABLE: gfp_t = GFP_HIGHUSER | __GFP_MOVABLE | __GFP_SKIP_KASAN;
pub const GFP_TRANSHUGE_LIGHT: gfp_t =
    (GFP_HIGHUSER_MOVABLE | __GFP_COMP | __GFP_NOMEMALLOC | __GFP_NOWARN) & !__GFP_RECLAIM;
pub const GFP_TRANSHUGE: gfp_t = GFP_TRANSHUGE_LIGHT | __GFP_DIRECT_RECLAIM;
