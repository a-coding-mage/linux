// SPDX-License-Identifier: GPL-2.0-only
// Exact designated initializer inventory transcribed from frozen vmstat.c.
#[cfg(CONFIG_PROC_FS)]
const NR_VMSTAT_ITEMS: usize = NR_VM_ZONE_STAT_ITEMS as usize
    + NR_VM_NUMA_EVENT_ITEMS as usize
    + NR_VM_NODE_STAT_ITEMS as usize
    + NR_VM_STAT_ITEMS as usize
    + if cfg!(CONFIG_VM_EVENT_COUNTERS) {
        NR_VM_EVENT_ITEMS as usize
    } else {
        0
    };
// A single configured designated-initializer inventory supplies both the
// inferred C array length and its values. Neither pass adds runtime code.
macro_rules! vmstat_entries {
    ($entry:ident, $target:ident) => {{
        {
            $entry!(
                $target,
                (0 + NR_FREE_PAGES) as usize,
                b"nr_free_pages\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (0 + NR_FREE_PAGES_BLOCKS) as usize,
                b"nr_free_pages_blocks\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (0 + NR_ZONE_INACTIVE_ANON) as usize,
                b"nr_zone_inactive_anon\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (0 + NR_ZONE_ACTIVE_ANON) as usize,
                b"nr_zone_active_anon\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (0 + NR_ZONE_INACTIVE_FILE) as usize,
                b"nr_zone_inactive_file\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (0 + NR_ZONE_ACTIVE_FILE) as usize,
                b"nr_zone_active_file\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (0 + NR_ZONE_UNEVICTABLE) as usize,
                b"nr_zone_unevictable\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (0 + NR_ZONE_WRITE_PENDING) as usize,
                b"nr_zone_write_pending\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (0 + NR_MLOCK) as usize,
                b"nr_mlock\0".as_ptr().cast()
            );
        }
        #[cfg(all(RUST_VMSTAT_ENABLED_ZSMALLOC))]
        {
            $entry!(
                $target,
                (0 + NR_ZSPAGES) as usize,
                b"nr_zspages\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (0 + NR_FREE_CMA_PAGES) as usize,
                b"nr_free_cma\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_UNACCEPTED_MEMORY))]
        {
            $entry!(
                $target,
                (0 + NR_UNACCEPTED) as usize,
                b"nr_unaccepted\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_NUMA))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + 0 + NUMA_HIT) as usize,
                b"numa_hit\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_NUMA))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + 0 + NUMA_MISS) as usize,
                b"numa_miss\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_NUMA))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + 0 + NUMA_FOREIGN) as usize,
                b"numa_foreign\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_NUMA))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + 0 + NUMA_INTERLEAVE_HIT) as usize,
                b"numa_interleave\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_NUMA))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + 0 + NUMA_LOCAL) as usize,
                b"numa_local\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_NUMA))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + 0 + NUMA_OTHER) as usize,
                b"numa_other\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_INACTIVE_ANON) as usize,
                b"nr_inactive_anon\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_ACTIVE_ANON) as usize,
                b"nr_active_anon\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_INACTIVE_FILE) as usize,
                b"nr_inactive_file\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_ACTIVE_FILE) as usize,
                b"nr_active_file\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_UNEVICTABLE) as usize,
                b"nr_unevictable\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_SLAB_RECLAIMABLE_B)
                    as usize,
                b"nr_slab_reclaimable\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_SLAB_UNRECLAIMABLE_B)
                    as usize,
                b"nr_slab_unreclaimable\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_ISOLATED_ANON) as usize,
                b"nr_isolated_anon\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_ISOLATED_FILE) as usize,
                b"nr_isolated_file\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + WORKINGSET_NODES) as usize,
                b"workingset_nodes\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + WORKINGSET_REFAULT_ANON)
                    as usize,
                b"workingset_refault_anon\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + WORKINGSET_REFAULT_FILE)
                    as usize,
                b"workingset_refault_file\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + WORKINGSET_ACTIVATE_ANON)
                    as usize,
                b"workingset_activate_anon\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + WORKINGSET_ACTIVATE_FILE)
                    as usize,
                b"workingset_activate_file\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + WORKINGSET_RESTORE_ANON)
                    as usize,
                b"workingset_restore_anon\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + WORKINGSET_RESTORE_FILE)
                    as usize,
                b"workingset_restore_file\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + WORKINGSET_NODERECLAIM)
                    as usize,
                b"workingset_nodereclaim\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_ANON_MAPPED) as usize,
                b"nr_anon_pages\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_FILE_MAPPED) as usize,
                b"nr_mapped\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_FILE_PAGES) as usize,
                b"nr_file_pages\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_FILE_DIRTY) as usize,
                b"nr_dirty\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_WRITEBACK) as usize,
                b"nr_writeback\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_SHMEM) as usize,
                b"nr_shmem\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_SHMEM_THPS) as usize,
                b"nr_shmem_hugepages\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_SHMEM_PMDMAPPED) as usize,
                b"nr_shmem_pmdmapped\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_FILE_THPS) as usize,
                b"nr_file_hugepages\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_FILE_PMDMAPPED) as usize,
                b"nr_file_pmdmapped\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_ANON_THPS) as usize,
                b"nr_anon_transparent_hugepages\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_VMSCAN_WRITE) as usize,
                b"nr_vmscan_write\0".as_ptr().cast()
            );
        }
        {
            names[(NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_VMSCAN_IMMEDIATE)
                as usize] = b"nr_vmscan_immediate_reclaim\0".as_ptr().cast();
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_DIRTIED) as usize,
                b"nr_dirtied\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_WRITTEN) as usize,
                b"nr_written\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_THROTTLED_WRITTEN)
                    as usize,
                b"nr_throttled_written\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_KERNEL_MISC_RECLAIMABLE)
                    as usize,
                b"nr_kernel_misc_reclaimable\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_FOLL_PIN_ACQUIRED)
                    as usize,
                b"nr_foll_pin_acquired\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_FOLL_PIN_RELEASED)
                    as usize,
                b"nr_foll_pin_released\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_VMALLOC) as usize,
                b"nr_vmalloc\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_KERNEL_STACK_KB) as usize,
                b"nr_kernel_stack\0".as_ptr().cast()
            );
        }
        #[cfg(all(RUST_VMSTAT_ENABLED_SHADOW_CALL_STACK))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_KERNEL_SCS_KB) as usize,
                b"nr_shadow_call_stack\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_PAGETABLE) as usize,
                b"nr_page_table_pages\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_SECONDARY_PAGETABLE)
                    as usize,
                b"nr_sec_page_table_pages\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_IOMMU_SUPPORT))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_IOMMU_PAGES) as usize,
                b"nr_iommu_pages\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_SWAP))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_SWAPCACHE) as usize,
                b"nr_swapcached\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_NUMA_BALANCING))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGPROMOTE_SUCCESS) as usize,
                b"pgpromote_success\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_NUMA_BALANCING))]
        {
            names[(NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGPROMOTE_CANDIDATE)
                as usize] = b"pgpromote_candidate\0".as_ptr().cast();
        }
        #[cfg(all(CONFIG_NUMA_BALANCING))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGPROMOTE_CANDIDATE_NRL)
                    as usize,
                b"pgpromote_candidate_nrl\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGDEMOTE_KSWAPD) as usize,
                b"pgdemote_kswapd\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGDEMOTE_DIRECT) as usize,
                b"pgdemote_direct\0".as_ptr().cast()
            );
        }
        {
            names[(NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGDEMOTE_KHUGEPAGED)
                as usize] = b"pgdemote_khugepaged\0".as_ptr().cast();
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGDEMOTE_PROACTIVE) as usize,
                b"pgdemote_proactive\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGSTEAL_KSWAPD) as usize,
                b"pgsteal_kswapd\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGSTEAL_DIRECT) as usize,
                b"pgsteal_direct\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGSTEAL_KHUGEPAGED) as usize,
                b"pgsteal_khugepaged\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGSTEAL_PROACTIVE) as usize,
                b"pgsteal_proactive\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGSTEAL_ANON) as usize,
                b"pgsteal_anon\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGSTEAL_FILE) as usize,
                b"pgsteal_file\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGSCAN_KSWAPD) as usize,
                b"pgscan_kswapd\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGSCAN_DIRECT) as usize,
                b"pgscan_direct\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGSCAN_KHUGEPAGED) as usize,
                b"pgscan_khugepaged\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGSCAN_PROACTIVE) as usize,
                b"pgscan_proactive\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGSCAN_ANON) as usize,
                b"pgscan_anon\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGSCAN_FILE) as usize,
                b"pgscan_file\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGROTATE_ANON) as usize,
                b"pgrotate_anon\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGROTATE_FILE) as usize,
                b"pgrotate_file\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + PGREFILL) as usize,
                b"pgrefill\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_HUGETLB_PAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_HUGETLB) as usize,
                b"nr_hugetlb\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_BALLOON_PAGES) as usize,
                b"nr_balloon_pages\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_KERNEL_FILE_PAGES)
                    as usize,
                b"nr_kernel_file_pages\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_GPU_ACTIVE) as usize,
                b"nr_gpu_active\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS + NR_VM_NUMA_EVENT_ITEMS + 0 + NR_GPU_RECLAIM) as usize,
                b"nr_gpu_reclaim\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + 0
                    + NR_DIRTY_THRESHOLD) as usize,
                b"nr_dirty_threshold\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + 0
                    + NR_DIRTY_BG_THRESHOLD) as usize,
                b"nr_dirty_background_threshold\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + 0
                    + NR_MEMMAP_PAGES) as usize,
                b"nr_memmap_pages\0".as_ptr().cast()
            );
        }
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + 0
                    + NR_MEMMAP_BOOT_PAGES) as usize,
                b"nr_memmap_boot_pages\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGPGIN) as usize,
                b"pgpgin\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGPGOUT) as usize,
                b"pgpgout\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PSWPIN) as usize,
                b"pswpin\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PSWPOUT) as usize,
                b"pswpout\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_ZONE_DMA))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + PGALLOC_DMA) as usize,
                b"pgalloc_dma\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_ZONE_DMA32))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + PGALLOC_DMA32) as usize,
                b"pgalloc_dma32\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + PGALLOC_NORMAL) as usize,
                b"pgalloc_normal\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_HIGHMEM))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + PGALLOC_HIGH) as usize,
                b"pgalloc_high\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + PGALLOC_MOVABLE) as usize,
                b"pgalloc_movable\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_ZONE_DEVICE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + PGALLOC_DEVICE) as usize,
                b"pgalloc_device\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_ZONE_DMA))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + ALLOCSTALL_DMA) as usize,
                b"allocstall_dma\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_ZONE_DMA32))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + ALLOCSTALL_DMA32) as usize,
                b"allocstall_dma32\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + ALLOCSTALL_NORMAL) as usize,
                b"allocstall_normal\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_HIGHMEM))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + ALLOCSTALL_HIGH) as usize,
                b"allocstall_high\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + ALLOCSTALL_MOVABLE) as usize,
                b"allocstall_movable\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_ZONE_DEVICE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + ALLOCSTALL_DEVICE) as usize,
                b"allocstall_device\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_ZONE_DMA))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + PGSCAN_SKIP_DMA) as usize,
                b"pgskip_dma\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_ZONE_DMA32))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + PGSCAN_SKIP_DMA32) as usize,
                b"pgskip_dma32\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + PGSCAN_SKIP_NORMAL) as usize,
                b"pgskip_normal\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_HIGHMEM))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + PGSCAN_SKIP_HIGH) as usize,
                b"pgskip_high\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + PGSCAN_SKIP_MOVABLE) as usize,
                b"pgskip_movable\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_ZONE_DEVICE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + PGSCAN_SKIP_DEVICE) as usize,
                b"pgskip_device\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGFREE) as usize,
                b"pgfree\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGACTIVATE) as usize,
                b"pgactivate\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGDEACTIVATE) as usize,
                b"pgdeactivate\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGLAZYFREE) as usize,
                b"pglazyfree\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGFAULT) as usize,
                b"pgfault\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGMAJFAULT) as usize,
                b"pgmajfault\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGLAZYFREED) as usize,
                b"pglazyfreed\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGREUSE) as usize,
                b"pgreuse\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGSCAN_DIRECT_THROTTLE) as usize,
                b"pgscan_direct_throttle\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_NUMA))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGSCAN_ZONE_RECLAIM_SUCCESS) as usize,
                b"zone_reclaim_success\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_NUMA))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGSCAN_ZONE_RECLAIM_FAILED) as usize,
                b"zone_reclaim_failed\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGINODESTEAL) as usize,
                b"pginodesteal\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + SLABS_SCANNED) as usize,
                b"slabs_scanned\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + KSWAPD_INODESTEAL) as usize,
                b"kswapd_inodesteal\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + KSWAPD_LOW_WMARK_HIT_QUICKLY) as usize,
                b"kswapd_low_wmark_hit_quickly\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + KSWAPD_HIGH_WMARK_HIT_QUICKLY) as usize,
                b"kswapd_high_wmark_hit_quickly\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PAGEOUTRUN) as usize,
                b"pageoutrun\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGROTATED) as usize,
                b"pgrotated\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + DROP_PAGECACHE) as usize,
                b"drop_pagecache\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + DROP_SLAB) as usize,
                b"drop_slab\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + OOM_KILL) as usize,
                b"oom_kill\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_NUMA_BALANCING))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + NUMA_PTE_UPDATES) as usize,
                b"numa_pte_updates\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_NUMA_BALANCING))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + NUMA_HUGE_PTE_UPDATES) as usize,
                b"numa_huge_pte_updates\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_NUMA_BALANCING))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + NUMA_HINT_FAULTS) as usize,
                b"numa_hint_faults\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_NUMA_BALANCING))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + NUMA_HINT_FAULTS_LOCAL) as usize,
                b"numa_hint_faults_local\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_NUMA_BALANCING))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + NUMA_PAGE_MIGRATE) as usize,
                b"numa_pages_migrated\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_MIGRATION))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGMIGRATE_SUCCESS) as usize,
                b"pgmigrate_success\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_MIGRATION))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + PGMIGRATE_FAIL) as usize,
                b"pgmigrate_fail\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_MIGRATION))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_MIGRATION_SUCCESS) as usize,
                b"thp_migration_success\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_MIGRATION))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_MIGRATION_FAIL) as usize,
                b"thp_migration_fail\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_MIGRATION))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_MIGRATION_SPLIT) as usize,
                b"thp_migration_split\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_COMPACTION))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + COMPACTMIGRATE_SCANNED) as usize,
                b"compact_migrate_scanned\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_COMPACTION))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + COMPACTFREE_SCANNED) as usize,
                b"compact_free_scanned\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_COMPACTION))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + COMPACTISOLATED) as usize,
                b"compact_isolated\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_COMPACTION))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + COMPACTSTALL) as usize,
                b"compact_stall\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_COMPACTION))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + COMPACTFAIL) as usize,
                b"compact_fail\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_COMPACTION))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + COMPACTSUCCESS) as usize,
                b"compact_success\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_COMPACTION))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + KCOMPACTD_WAKE) as usize,
                b"compact_daemon_wake\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_COMPACTION))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + KCOMPACTD_MIGRATE_SCANNED) as usize,
                b"compact_daemon_migrate_scanned\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_COMPACTION))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + KCOMPACTD_FREE_SCANNED) as usize,
                b"compact_daemon_free_scanned\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_HUGETLB_PAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + HTLB_BUDDY_PGALLOC) as usize,
                b"htlb_buddy_alloc_success\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_HUGETLB_PAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + HTLB_BUDDY_PGALLOC_FAIL) as usize,
                b"htlb_buddy_alloc_fail\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_CMA))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + CMA_ALLOC_SUCCESS) as usize,
                b"cma_alloc_success\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_CMA))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + CMA_ALLOC_FAIL) as usize,
                b"cma_alloc_fail\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + UNEVICTABLE_PGCULLED) as usize,
                b"unevictable_pgs_culled\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + UNEVICTABLE_PGSCANNED) as usize,
                b"unevictable_pgs_scanned\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + UNEVICTABLE_PGRESCUED) as usize,
                b"unevictable_pgs_rescued\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + UNEVICTABLE_PGMLOCKED) as usize,
                b"unevictable_pgs_mlocked\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + UNEVICTABLE_PGMUNLOCKED) as usize,
                b"unevictable_pgs_munlocked\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + UNEVICTABLE_PGCLEARED) as usize,
                b"unevictable_pgs_cleared\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + UNEVICTABLE_PGSTRANDED) as usize,
                b"unevictable_pgs_stranded\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_FAULT_ALLOC) as usize,
                b"thp_fault_alloc\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_FAULT_FALLBACK) as usize,
                b"thp_fault_fallback\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_FAULT_FALLBACK_CHARGE) as usize,
                b"thp_fault_fallback_charge\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_COLLAPSE_ALLOC) as usize,
                b"thp_collapse_alloc\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_COLLAPSE_ALLOC_FAILED) as usize,
                b"thp_collapse_alloc_failed\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_FILE_ALLOC) as usize,
                b"thp_file_alloc\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_FILE_FALLBACK) as usize,
                b"thp_file_fallback\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_FILE_FALLBACK_CHARGE) as usize,
                b"thp_file_fallback_charge\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_FILE_MAPPED) as usize,
                b"thp_file_mapped\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_SPLIT_PAGE) as usize,
                b"thp_split_page\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_SPLIT_PAGE_FAILED) as usize,
                b"thp_split_page_failed\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_DEFERRED_SPLIT_PAGE) as usize,
                b"thp_deferred_split_page\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_UNDERUSED_SPLIT_PAGE) as usize,
                b"thp_underused_split_page\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_SPLIT_PMD) as usize,
                b"thp_split_pmd\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_SCAN_EXCEED_NONE_PTE) as usize,
                b"thp_scan_exceed_none_pte\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_SCAN_EXCEED_SWAP_PTE) as usize,
                b"thp_scan_exceed_swap_pte\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_SCAN_EXCEED_SHARED_PTE) as usize,
                b"thp_scan_exceed_share_pte\0".as_ptr().cast()
            );
        }
        #[cfg(all(
            CONFIG_VM_EVENT_COUNTERS,
            CONFIG_TRANSPARENT_HUGEPAGE,
            CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD
        ))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_SPLIT_PUD) as usize,
                b"thp_split_pud\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_ZERO_PAGE_ALLOC) as usize,
                b"thp_zero_page_alloc\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_ZERO_PAGE_ALLOC_FAILED) as usize,
                b"thp_zero_page_alloc_failed\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_SWPOUT) as usize,
                b"thp_swpout\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_TRANSPARENT_HUGEPAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + THP_SWPOUT_FALLBACK) as usize,
                b"thp_swpout_fallback\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_BALLOON))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + BALLOON_INFLATE) as usize,
                b"balloon_inflate\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_BALLOON))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + BALLOON_DEFLATE) as usize,
                b"balloon_deflate\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_BALLOON, CONFIG_BALLOON_MIGRATION))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + BALLOON_MIGRATE) as usize,
                b"balloon_migrate\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_DEBUG_TLBFLUSH))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + NR_TLB_REMOTE_FLUSH) as usize,
                b"nr_tlb_remote_flush\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_DEBUG_TLBFLUSH))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + NR_TLB_REMOTE_FLUSH_RECEIVED) as usize,
                b"nr_tlb_remote_flush_received\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_DEBUG_TLBFLUSH))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + NR_TLB_LOCAL_FLUSH_ALL) as usize,
                b"nr_tlb_local_flush_all\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_DEBUG_TLBFLUSH))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + NR_TLB_LOCAL_FLUSH_ONE) as usize,
                b"nr_tlb_local_flush_one\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_SWAP))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + SWAP_RA) as usize,
                b"swap_ra\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_SWAP))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + SWAP_RA_HIT) as usize,
                b"swap_ra_hit\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_SWAP))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + SWPIN_ZERO) as usize,
                b"swpin_zero\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_SWAP))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + SWPOUT_ZERO) as usize,
                b"swpout_zero\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_SWAP, CONFIG_KSM))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + KSM_SWPIN_COPY) as usize,
                b"ksm_swpin_copy\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_KSM))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + COW_KSM) as usize,
                b"cow_ksm\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_ZSWAP))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + ZSWPIN) as usize,
                b"zswpin\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_ZSWAP))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + ZSWPOUT) as usize,
                b"zswpout\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_ZSWAP))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + ZSWPWB) as usize,
                b"zswpwb\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_X86))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + DIRECT_MAP_LEVEL2_SPLIT) as usize,
                b"direct_map_level2_splits\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_X86))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + DIRECT_MAP_LEVEL3_SPLIT) as usize,
                b"direct_map_level3_splits\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_X86))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + DIRECT_MAP_LEVEL2_COLLAPSE) as usize,
                b"direct_map_level2_collapses\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_X86))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + DIRECT_MAP_LEVEL3_COLLAPSE) as usize,
                b"direct_map_level3_collapses\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_PER_VMA_LOCK_STATS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + VMA_LOCK_SUCCESS) as usize,
                b"vma_lock_success\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_PER_VMA_LOCK_STATS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + VMA_LOCK_ABORT) as usize,
                b"vma_lock_abort\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_PER_VMA_LOCK_STATS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + VMA_LOCK_RETRY) as usize,
                b"vma_lock_retry\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_PER_VMA_LOCK_STATS))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + VMA_LOCK_MISS) as usize,
                b"vma_lock_miss\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_DEBUG_STACK_USAGE))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + KSTACK_1K) as usize,
                b"kstack_1k\0".as_ptr().cast()
            );
        }
        #[cfg(all(
            CONFIG_VM_EVENT_COUNTERS,
            CONFIG_DEBUG_STACK_USAGE,
            RUST_VMSTAT_THREAD_GT_1024
        ))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + KSTACK_2K) as usize,
                b"kstack_2k\0".as_ptr().cast()
            );
        }
        #[cfg(all(
            CONFIG_VM_EVENT_COUNTERS,
            CONFIG_DEBUG_STACK_USAGE,
            RUST_VMSTAT_THREAD_GT_2048
        ))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + KSTACK_4K) as usize,
                b"kstack_4k\0".as_ptr().cast()
            );
        }
        #[cfg(all(
            CONFIG_VM_EVENT_COUNTERS,
            CONFIG_DEBUG_STACK_USAGE,
            RUST_VMSTAT_THREAD_GT_4096
        ))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + KSTACK_8K) as usize,
                b"kstack_8k\0".as_ptr().cast()
            );
        }
        #[cfg(all(
            CONFIG_VM_EVENT_COUNTERS,
            CONFIG_DEBUG_STACK_USAGE,
            RUST_VMSTAT_THREAD_GT_8192
        ))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + KSTACK_16K) as usize,
                b"kstack_16k\0".as_ptr().cast()
            );
        }
        #[cfg(all(
            CONFIG_VM_EVENT_COUNTERS,
            CONFIG_DEBUG_STACK_USAGE,
            RUST_VMSTAT_THREAD_GT_16384
        ))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + KSTACK_32K) as usize,
                b"kstack_32k\0".as_ptr().cast()
            );
        }
        #[cfg(all(
            CONFIG_VM_EVENT_COUNTERS,
            CONFIG_DEBUG_STACK_USAGE,
            RUST_VMSTAT_THREAD_GT_32768
        ))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + KSTACK_64K) as usize,
                b"kstack_64k\0".as_ptr().cast()
            );
        }
        #[cfg(all(
            CONFIG_VM_EVENT_COUNTERS,
            CONFIG_DEBUG_STACK_USAGE,
            RUST_VMSTAT_THREAD_GT_65536
        ))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + KSTACK_REST) as usize,
                b"kstack_rest\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_SWAP))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + NRSWPIN) as usize,
                b"nrswpin\0".as_ptr().cast()
            );
        }
        #[cfg(all(CONFIG_VM_EVENT_COUNTERS, CONFIG_SWAP))]
        {
            $entry!(
                $target,
                (NR_VM_ZONE_STAT_ITEMS
                    + NR_VM_NUMA_EVENT_ITEMS
                    + NR_VM_NODE_STAT_ITEMS
                    + NR_VM_STAT_ITEMS
                    + 0
                    + NRSWPOUT) as usize,
                b"nrswpout\0".as_ptr().cast()
            );
        }
    }};
}
const fn vmstat_inferred_length() -> usize {
    let mut length: usize = 0;
    macro_rules! count_name {
        ($target:ident, $index:expr, $value:expr) => {{
            let index: usize = $index;
            if index + 1 > $target {
                $target = index + 1;
            }
        }};
    }
    vmstat_entries!(count_name, length);
    length
}
const VMSTAT_TEXT_LENGTH: usize = vmstat_inferred_length();
// vmstat_start's original BUILD_BUG_ON exists only with PROC_FS enabled.
#[cfg(CONFIG_PROC_FS)]
const _: () = assert!(VMSTAT_TEXT_LENGTH == NR_VMSTAT_ITEMS);
#[repr(transparent)]
struct VmstatText([*const c_char; VMSTAT_TEXT_LENGTH]);
// SAFETY: every pointer names an immutable NUL-terminated static string.
unsafe impl Sync for VmstatText {}
#[export_name = "vmstat_text"]
static VMSTAT_TEXT: VmstatText = VmstatText({
    let mut names = [core::ptr::null(); VMSTAT_TEXT_LENGTH];
    macro_rules! assign_name {
        ($target:ident, $index:expr, $value:expr) => {{
            $target[$index] = $value;
        }};
    }
    vmstat_entries!(assign_name, names);
    names
});
#[cfg(CONFIG_PROC_FS)]
fn zone_stat_name(item: usize) -> *const c_char {
    VMSTAT_TEXT.0[item]
}
#[cfg(CONFIG_PROC_FS)]
fn node_stat_name(item: usize) -> *const c_char {
    VMSTAT_TEXT.0[NR_VM_ZONE_STAT_ITEMS as usize + NR_VM_NUMA_EVENT_ITEMS as usize + item]
}
#[cfg(all(CONFIG_NUMA, CONFIG_PROC_FS))]
fn numa_stat_name(item: usize) -> *const c_char {
    VMSTAT_TEXT.0[NR_VM_ZONE_STAT_ITEMS as usize + item]
}
