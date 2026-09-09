// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Translated from the Linux kernel implementation. The following symbols are
// supplied by the surrounding kernel dependencies.

/*
 * ARCH specific callbacks for generic noncoherent DMA ops
 *  - hardware IOC not available (or "dma-coherent" not set for device in DT)
 *  - But still handle both coherent and non-coherent requests from caller
 *
 * For DMA coherent hardware (IOC) generic code suffices
 */

pub unsafe fn arch_dma_prep_coherent(page: *mut page, size: usize) {
	/*
	 * Evict any existing L1 and/or L2 lines for the backing page
	 * in case it was used earlier as a normal "cached" page.
	 * Yeah this bit us - STAR 9000898266
	 *
	 * Although core does call flush_cache_vmap(), it gets kvaddr hence
	 * can't be used to efficiently flush L1 and/or L2 which need paddr
	 * Currently flush_cache_vmap nukes the L1 cache completely which
	 * will be optimized as a separate commit
	 */
	dma_cache_wback_inv(page_to_phys(page), size);
}

/*
 * Cache operations depending on function and direction argument, inspired by
 * https://lore.kernel.org/lkml/20180518175004.GF17671@n2100.armlinux.org.uk
 * "dma_sync_*_for_cpu and direction=TO_DEVICE (was Re: [PATCH 02/20]
 * dma-mapping: provide a generic dma-noncoherent implementation)"
 *
 *          |   map          ==  for_device     |   unmap     ==  for_cpu
 *          |----------------------------------------------------------------
 * TO_DEV   |   writeback        writeback      |   none          none
 * FROM_DEV |   invalidate       invalidate     |   invalidate*   invalidate*
 * BIDIR    |   writeback+inv    writeback+inv  |   invalidate    invalidate
 *
 *     [*] needed for CPU speculative prefetches
 *
 * NOTE: we don't check the validity of direction argument as it is done in
 * upper layer functions (in include/linux/dma-mapping.h)
 */

pub unsafe fn arch_sync_dma_for_device(
	paddr: phys_addr_t,
	size: usize,
	dir: dma_data_direction,
) {
	match dir {
		DMA_TO_DEVICE => {
			dma_cache_wback(paddr, size);
		}
		DMA_FROM_DEVICE => {
			dma_cache_inv(paddr, size);
		}
		DMA_BIDIRECTIONAL => {
			dma_cache_wback_inv(paddr, size);
		}
		_ => {}
	}
}

pub unsafe fn arch_sync_dma_for_cpu(
	paddr: phys_addr_t,
	size: usize,
	dir: dma_data_direction,
) {
	match dir {
		DMA_TO_DEVICE => {}

		/* FROM_DEVICE invalidate needed if speculative CPU prefetch only */
		DMA_FROM_DEVICE | DMA_BIDIRECTIONAL => {
			dma_cache_inv(paddr, size);
		}
		_ => {}
	}
}

/*
 * Plug in direct dma map ops.
 */
pub unsafe fn arch_setup_dma_ops(dev: *mut device, coherent: bool) {
	/*
	 * IOC hardware snoops all DMA traffic keeping the caches consistent
	 * with memory - eliding need for any explicit cache maintenance of
	 * DMA buffers.
	 */
	if is_isa_arcv2() && ioc_enable && coherent {
		dev_set_dma_coherent(dev);
	}

	dev_info(
		dev,
		if dev_dma_coherent(dev) {
			"use %scoherent DMA ops\n"
		} else {
			"use %scoherent DMA ops\n"
		},
		if dev_dma_coherent(dev) { "" } else { "non" },
	);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
