// SPDX-License-Identifier: GPL-2.0-only
/*
 * Linux VM pressure
 *
 * Copyright 2012 Linaro Ltd.
 *		  Anton Vorontsov <anton.vorontsov@linaro.org>
 *
 * Based on ideas from Andrew Morton, David Rientjes, KOSAKI Motohiro,
 * Leonid Moiseichuk, Mel Gorman, Minchan Kim and Pekka Enberg.
 *
 * Tree-mode (cgroup v1 userspace eventfd) bookkeeping lives in
 * mm/memcontrol-v1.c; this file holds the shared code and the in-kernel
 * (tree=false) socket-pressure path that runs on cgroup v2.
 */

/* Kernel dependencies supplied by the surrounding translation unit. */

pub const vmpressure_win: ::core::ffi::c_ulong = SWAP_CLUSTER_MAX * 16;

static const vmpressure_level_med: ::core::ffi::c_uint = 60;
static const vmpressure_level_critical: ::core::ffi::c_uint = 95;

unsafe fn vmpressure_level(pressure: ::core::ffi::c_ulong) -> vmpressure_levels {
	if pressure >= vmpressure_level_critical as ::core::ffi::c_ulong {
		VMPRESSURE_CRITICAL
	} else if pressure >= vmpressure_level_med as ::core::ffi::c_ulong {
		VMPRESSURE_MEDIUM
	} else {
		VMPRESSURE_LOW
	}
}

pub unsafe fn vmpressure_calc_level(
	scanned: ::core::ffi::c_ulong,
	reclaimed: ::core::ffi::c_ulong,
) -> vmpressure_levels {
	let scale = scanned.wrapping_add(reclaimed);
	let mut pressure: ::core::ffi::c_ulong = 0;

	/*
	 * reclaimed can be greater than scanned for things such as reclaimed
	 * slab pages. shrink_node() just adds reclaimed pages without a
	 * related increment to scanned pages.
	 */
	if reclaimed < scanned {
		/*
		 * We calculate the ratio (in percents) of how many pages were
		 * scanned vs. reclaimed in a given time frame (window). Note that
		 * time is in VM reclaimer's "ticks", i.e. number of pages
		 * scanned. This makes it possible to set desired reaction time
		 * and serves as a ratelimit.
		 */
		pressure = scale.wrapping_sub(reclaimed.wrapping_mul(scale) / scanned);
		pressure = pressure.wrapping_mul(100) / scale;
	}

	pr_debug("%s: %3lu  (s: %lu  r: %lu)\n", "vmpressure_calc_level", pressure, scanned, reclaimed);

	vmpressure_level(pressure)
}

pub unsafe fn vmpressure(
	gfp: gfp_t,
	order: ::core::ffi::c_int,
	memcg: *mut mem_cgroup,
	tree: bool,
	mut scanned: ::core::ffi::c_ulong,
	mut reclaimed: ::core::ffi::c_ulong,
) {
	let vmpr: *mut vmpressure;

	if mem_cgroup_disabled() {
		return;
	}

	/*
	 * Only two combinations have a consumer:
	 *   cgroup v2 + tree=false -> in-kernel socket pressure
	 *   cgroup v1 + tree=true  -> userspace eventfds (memory.pressure_level)
	 * Skip the other two: nothing consumes the result.
	 */
	if cgroup_subsys_on_dfl(memory_cgrp_subsys) == tree {
		return;
	}

	vmpr = memcg_to_vmpressure(memcg);

	/*
	 * Here we only want to account pressure that userland is able to
	 * help us with. For example, suppose that DMA zone is under
	 * pressure; if we notify userland about that kind of pressure,
	 * then it will be mostly a waste as it will trigger unnecessary
	 * freeing of memory by userland (since userland is more likely to
	 * have HIGHMEM/MOVABLE pages instead of the DMA fallback). That
	 * is why we include only movable, highmem and FS/IO pages.
	 * Indirect reclaim (kswapd) sets sc->gfp_mask to GFP_KERNEL, so
	 * we account it too.
	 */
	if (gfp & (__GFP_HIGHMEM | __GFP_MOVABLE | __GFP_IO | __GFP_FS)) == 0 {
		return;
	}

	/*
	 * If we got here with no pages scanned, then that is an indicator
	 * that reclaimer was unable to find any shrinkable LRUs at the
	 * current scanning depth. But it does not mean that we should
	 * report the critical pressure, yet. If the scanning priority
	 * (scanning depth) goes too high (deep), we will be notified
	 * through vmpressure_prio(). But so far, keep calm.
	 */
	if scanned == 0 {
		return;
	}

	if tree {
		vmpressure_v1_account_tree(vmpr, scanned, reclaimed);
	} else {
		let level: vmpressure_levels;

		/* For now, no users for root-level efficiency */
		if memcg.is_null() || mem_cgroup_is_root(memcg) {
			return;
		}

		spin_lock(&mut (*vmpr).sr_lock);
		scanned = (*vmpr).scanned.wrapping_add(scanned);
		(*vmpr).scanned = scanned;
		reclaimed = (*vmpr).reclaimed.wrapping_add(reclaimed);
		(*vmpr).reclaimed = reclaimed;
		if scanned < vmpressure_win {
			spin_unlock(&mut (*vmpr).sr_lock);
			return;
		}
		(*vmpr).scanned = 0;
		(*vmpr).reclaimed = 0;
		spin_unlock(&mut (*vmpr).sr_lock);

		level = vmpressure_calc_level(scanned, reclaimed);

		/*
		 * Once we go above COSTLY_ORDER, reclaim relies heavily on
		 * compaction to make progress. Reclaim efficiency was never a
		 * great proxy for pressure to begin with, but it's outright
		 * misleading with these high orders. Don't throttle sockets
		 * because somebody is attempting something crazy like an order-7
		 * and predictably struggling.
		 */
		if level > VMPRESSURE_LOW && order <= PAGE_ALLOC_COSTLY_ORDER {
			/*
			 * Let the socket buffer allocator know that
			 * we are having trouble reclaiming LRU pages.
			 *
			 * For hysteresis keep the pressure state
			 * asserted for a second in which subsequent
			 * pressure events can occur.
			 */
			mem_cgroup_set_socket_pressure(memcg);
		}
	}
}

pub unsafe fn vmpressure_init(vmpr: *mut vmpressure) {
	spin_lock_init(&mut (*vmpr).sr_lock);
	vmpressure_v1_init(vmpr);
}

pub unsafe fn vmpressure_cleanup(vmpr: *mut vmpressure) {
	vmpressure_v1_cleanup(vmpr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
