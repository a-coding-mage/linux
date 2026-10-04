// SPDX-License-Identifier: GPL-2.0-only
// Generated native-header primitive aliases.
use b::rust_vs_current as current;
use b::rust_vs_node_data as node_data;
use b::rust_vs_folio_lru_ptr as folio_lru_ptr;
use b::rust_vs_folio_flags_ptr as folio_flags_ptr;
use b::rust_vs_folio_deferred_ptr as folio_deferred_ptr;
use b::rust_vs_folio_swap as folio_swap;
use b::rust_vs_folio_page as folio_page;
use b::rust_vs_folio_from_lru as folio_from_lru;
use b::rust_vs_lru_to_folio as lru_to_folio;
use b::rust_vs_read_ulong as read_ulong;
use b::rust_vs_write_ulong as write_ulong;
use b::rust_vs_read_int as read_int;
use b::rust_vs_write_int as write_int;
use b::rust_vs_first_online_node as first_online_node;
use b::rust_vs_next_online_node as next_online_node;
use b::rust_vs_first_memory_node as first_memory_node;
use b::rust_vs_next_memory_node as next_memory_node;
use b::rust_vs_nodes_empty as nodes_empty;
use b::rust_vs_node_state as node_state;
use b::rust_vs_numa_node_id as numa_node_id;
use b::rust_vs_native_pageblock_order as native_pageblock_order;
use b::rust_vs_init_list_head as init_list_head;
use b::rust_vs_list_add as list_add;
use b::rust_vs_list_add_tail as list_add_tail;
use b::rust_vs_list_del as list_del;
use b::rust_vs_list_empty as list_empty;
use b::rust_vs_list_move as list_move;
use b::rust_vs_list_splice as list_splice;
use b::rust_vs_list_splice_init as list_splice_init;
use b::rust_vs_spin_lock as spin_lock;
use b::rust_vs_spin_unlock as spin_unlock;
use b::rust_vs_spin_lock_irqsave as spin_lock_irqsave;
use b::rust_vs_spin_unlock_irqrestore as spin_unlock_irqrestore;
use b::rust_vs_xa_lock_irq as xa_lock_irq;
use b::rust_vs_xa_unlock_irq as xa_unlock_irq;
use b::rust_vs_test_bit as test_bit;
use b::rust_vs_set_bit as set_bit;
use b::rust_vs_clear_bit as clear_bit;
use b::rust_vs_test_and_set_bit_lock as test_and_set_bit_lock;
use b::rust_vs_clear_bit_unlock as clear_bit_unlock;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_set_mask_bits as set_mask_bits;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_folio_lru_refs as folio_lru_refs;
use b::rust_vs_atomic_dec as atomic_dec;
use b::rust_vs_atomic_inc_return as atomic_inc_return;
use b::rust_vs_atomic_read as atomic_read;
use b::rust_vs_atomic_set as atomic_set;
use b::rust_vs_atomic_xchg as atomic_xchg;
use b::rust_vs_init_wait as init_wait;
use b::rust_vs_waitqueue_active as waitqueue_active;
use b::rust_vs_wake_up as wake_up;
use b::rust_vs_wake_up_all as wake_up_all;
use b::rust_vs_wake_up_interruptible as wake_up_interruptible;
use b::rust_vs_wait_pfmemalloc_interruptible_timeout as wait_pfmemalloc_interruptible_timeout;
use b::rust_vs_wait_pfmemalloc_killable as wait_pfmemalloc_killable;
use b::rust_vs_cond_resched as cond_resched;
use b::rust_vs_cond_resched_tasks_rcu_qs as cond_resched_tasks_rcu_qs;
use b::rust_vs_current_is_kswapd as current_is_kswapd;
use b::rust_vs_current_is_khugepaged as current_is_khugepaged;
use b::rust_vs_fatal_signal_pending as fatal_signal_pending;
use b::rust_vs_signal_pending as signal_pending;
use b::rust_vs_freezing as freezing;
use b::rust_vs_set_freezable as set_freezable;
use b::rust_vs_memalloc_noreclaim_save as memalloc_noreclaim_save;
use b::rust_vs_memalloc_noreclaim_restore as memalloc_noreclaim_restore;
use b::rust_vs_current_gfp_context as current_gfp_context;
use b::rust_vs_delayacct_freepages_start as delayacct_freepages_start;
use b::rust_vs_delayacct_freepages_end as delayacct_freepages_end;
use b::rust_vs_fs_reclaim_acquire as fs_reclaim_acquire;
use b::rust_vs_fs_reclaim_release as fs_reclaim_release;
use b::rust_vs_fs_reclaim_acquire_balance as fs_reclaim_acquire_balance;
use b::rust_vs_fs_reclaim_release_balance as fs_reclaim_release_balance;
use b::rust_vs_fs_reclaim_acquire_freeze as fs_reclaim_acquire_freeze;
use b::rust_vs_fs_reclaim_release_freeze as fs_reclaim_release_freeze;
use b::rust_vs_psi_memstall_enter as psi_memstall_enter;
use b::rust_vs_psi_memstall_leave as psi_memstall_leave;
use b::rust_vs_is_err as is_err;
use b::rust_vs_create_kswapd as create_kswapd;
use b::rust_vs_pgdat_kswapd_lock as pgdat_kswapd_lock;
use b::rust_vs_pgdat_kswapd_unlock as pgdat_kswapd_unlock;
use b::rust_vs_error_kswapd_start as error_kswapd_start;
use b::rust_vs_warn_reclaim_overwrite as warn_reclaim_overwrite;
use b::rust_vs_warn_reclaim_null as warn_reclaim_null;
use b::rust_vs_warn_throttle_reason as warn_throttle_reason;
use b::rust_vs_warn_anon_only as warn_anon_only;
#[cfg(CONFIG_MEMCG)]
use b::rust_vs_warn_softlimit_reclaim_state as warn_softlimit_reclaim_state;
use b::rust_vs_bug_remove_unlocked as bug_remove_unlocked;
use b::rust_vs_bug_remove_mapping as bug_remove_mapping;
use b::rust_vs_bug_kswapd_boot as bug_kswapd_boot;
use b::rust_vs_bug_shrink_active as bug_shrink_active;
use b::rust_vs_bug_activate_active as bug_activate_active;
use b::rust_vs_bug_keep_lru as bug_keep_lru;
use b::rust_vs_bug_isolate_ref as bug_isolate_ref;
use b::rust_vs_bug_move_lru as bug_move_lru;
use b::rust_vs_prefetch_prev_lru_flags as prefetch_prev_lru_flags;
use b::rust_vs_vma_flags_test as vma_flags_test;
use b::rust_vs_folio_is_file_lru as folio_is_file_lru;
use b::rust_vs_folio_test_active as folio_test_active;
use b::rust_vs_folio_test_anon as folio_test_anon;
use b::rust_vs_folio_test_dirty as folio_test_dirty;
use b::rust_vs_folio_test_hugetlb as folio_test_hugetlb;
use b::rust_vs_folio_test_large as folio_test_large;
use b::rust_vs_folio_test_lazyfree as folio_test_lazyfree;
use b::rust_vs_folio_test_locked as folio_test_locked;
use b::rust_vs_folio_test_lru as folio_test_lru;
use b::rust_vs_folio_test_mlocked as folio_test_mlocked;
use b::rust_vs_folio_test_pmd_mappable as folio_test_pmd_mappable;
use b::rust_vs_folio_test_private as folio_test_private;
use b::rust_vs_folio_test_reclaim as folio_test_reclaim;
use b::rust_vs_folio_test_referenced as folio_test_referenced;
use b::rust_vs_folio_test_swapbacked as folio_test_swapbacked;
use b::rust_vs_folio_test_swapcache as folio_test_swapcache;
use b::rust_vs_folio_test_unevictable as folio_test_unevictable;
use b::rust_vs_folio_test_workingset as folio_test_workingset;
use b::rust_vs_folio_test_writeback as folio_test_writeback;
use b::rust_vs_folio_test_clear_lru as folio_test_clear_lru;
use b::rust_vs_folio_test_clear_referenced as folio_test_clear_referenced;
use b::rust_vs_folio_set_active as folio_set_active;
use b::rust_vs_folio_set_lru as folio_set_lru;
use b::rust_vs_folio_set_reclaim as folio_set_reclaim;
use b::rust_vs_folio_set_referenced as folio_set_referenced;
use b::rust_vs_folio_set_workingset as folio_set_workingset;
use b::rust_vs_folio_clear_active as folio_clear_active;
use b::rust_vs_folio_clear_reclaim as folio_clear_reclaim;
use b::rust_vs_folio_clear_unevictable as folio_clear_unevictable;
use b::rust_vs___folio_clear_lru_flags as __folio_clear_lru_flags;
use b::rust_vs_folio_nr_pages as folio_nr_pages;
use b::rust_vs_folio_order as folio_order;
use b::rust_vs_folio_nid as folio_nid;
use b::rust_vs_folio_zonenum as folio_zonenum;
use b::rust_vs_folio_pfn as folio_pfn;
use b::rust_vs_folio_ref_count as folio_ref_count;
use b::rust_vs_folio_expected_ref_count as folio_expected_ref_count;
use b::rust_vs_folio_ref_freeze as folio_ref_freeze;
use b::rust_vs_folio_ref_unfreeze as folio_ref_unfreeze;
use b::rust_vs_folio_try_get as folio_try_get;
use b::rust_vs_folio_get as folio_get;
use b::rust_vs_folio_put as folio_put;
use b::rust_vs_folio_put_testzero as folio_put_testzero;
use b::rust_vs_folio_maybe_dma_pinned as folio_maybe_dma_pinned;
use b::rust_vs_folio_mapped as folio_mapped;
use b::rust_vs_folio_trylock as folio_trylock;
use b::rust_vs_folio_lock as folio_lock;
use b::rust_vs_folio_unlock as folio_unlock;
use b::rust_vs_folio_wait_writeback as folio_wait_writeback;
use b::rust_vs_folio_contain_hwpoisoned_page as folio_contain_hwpoisoned_page;
use b::rust_vs_folio_evictable as folio_evictable;
use b::rust_vs_folio_needs_release as folio_needs_release;
use b::rust_vs_folio_deferred_partially_mapped as folio_deferred_partially_mapped;
use b::rust_vs_folio_unqueue_deferred_split as folio_unqueue_deferred_split;
use b::rust_vs_folio_mapping as folio_mapping;
use b::rust_vs_folio_batch_init as folio_batch_init;
use b::rust_vs_folio_batch_add as folio_batch_add;
use b::rust_vs_folio_free_swap as folio_free_swap;
use b::rust_vs_folio_alloc_swap as folio_alloc_swap;
use b::rust_vs_page_has_movable_ops as page_has_movable_ops;
use b::rust_vs_split_folio_to_list as split_folio_to_list;
use b::rust_vs_thp_migration_supported as thp_migration_supported;
use b::rust_vs_try_to_unmap_flush as try_to_unmap_flush;
use b::rust_vs_try_to_unmap_flush_dirty as try_to_unmap_flush_dirty;
use b::rust_vs___swap_entry_to_info as __swap_entry_to_info;
use b::rust_vs_swap_cluster_get_and_lock_irq as swap_cluster_get_and_lock_irq;
use b::rust_vs_swap_cluster_unlock_irq as swap_cluster_unlock_irq;
use b::rust_vs___memcg1_swapout as __memcg1_swapout;
use b::rust_vs___swap_cache_del_folio as __swap_cache_del_folio;
use b::rust_vs_swap_writeout as swap_writeout;
use b::rust_vs_swap_write_submit as swap_write_submit;
use b::rust_vs_shmem_mapping as shmem_mapping;
use b::rust_vs_dax_mapping as dax_mapping;
use b::rust_vs_mapping_exiting as mapping_exiting;
use b::rust_vs_mapping_shrinkable as mapping_shrinkable;
use b::rust_vs_mapping_writeback_may_deadlock_on_reclaim as mapping_writeback_may_deadlock_on_reclaim;
use b::rust_vs_mapping_set_error as mapping_set_error;
use b::rust_vs_gfp_has_io_fs as gfp_has_io_fs;
use b::rust_vs_gfp_compaction_allowed as gfp_compaction_allowed;
use b::rust_vs_gfpflags_allow_blocking as gfpflags_allow_blocking;
use b::rust_vs_gfp_zone as gfp_zone;
use b::rust_vs_compact_gap as compact_gap;
use b::rust_vs_compaction_suitable as compaction_suitable;
use b::rust_vs_reset_isolation_suitable as reset_isolation_suitable;
use b::rust_vs_wakeup_kcompactd as wakeup_kcompactd;
use b::rust_vs_cpuset_zone_allowed as cpuset_zone_allowed;
use b::rust_vs_managed_zone as managed_zone;
use b::rust_vs_zone_idx as zone_idx;
use b::rust_vs_zone_to_nid as zone_to_nid;
use b::rust_vs_min_wmark_pages as min_wmark_pages;
use b::rust_vs_high_wmark_pages as high_wmark_pages;
use b::rust_vs_promo_wmark_pages as promo_wmark_pages;
use b::rust_vs_zone_page_state as zone_page_state;
use b::rust_vs_zone_page_state_snapshot as zone_page_state_snapshot;
use b::rust_vs_sum_zone_node_page_state as sum_zone_node_page_state;
use b::rust_vs_zone_watermark_ok as zone_watermark_ok;
use b::rust_vs_first_zones_zonelist as first_zones_zonelist;
use b::rust_vs_next_zones_zonelist as next_zones_zonelist;
use b::rust_vs_node_zonelist as node_zonelist;
use b::rust_vs_node_page_state as node_page_state;
use b::rust_vs_node_page_state_pages as node_page_state_pages;
use b::rust_vs_node_stat_add_folio as node_stat_add_folio;
use b::rust_vs_node_stat_mod_folio as node_stat_mod_folio;
use b::rust_vs___mod_node_page_state as __mod_node_page_state;
use b::rust_vs_mod_node_page_state as mod_node_page_state;
use b::rust_vs_count_vm_event as count_vm_event;
use b::rust_vs_count_vm_events as count_vm_events;
use b::rust_vs___count_vm_events as __count_vm_events;
use b::rust_vs___count_zid_vm_events as __count_zid_vm_events;
use b::rust_vs_count_mthp_stat as count_mthp_stat;
use b::rust_vs_set_pgdat_normal_threshold as set_pgdat_normal_threshold;
use b::rust_vs_set_pgdat_pressure_threshold as set_pgdat_pressure_threshold;
use b::rust_vs_is_active_lru as is_active_lru;
use b::rust_vs_is_file_lru as is_file_lru;
use b::rust_vs_lru_gen_enabled as lru_gen_enabled;
use b::rust_vs_lru_gen_switching as lru_gen_switching;
use b::rust_vs_lruvec_pgdat as lruvec_pgdat;
use b::rust_vs_lruvec_memcg as lruvec_memcg;
use b::rust_vs_lruvec_page_state as lruvec_page_state;
use b::rust_vs_lruvec_page_state_monotonic as lruvec_page_state_monotonic;
use b::rust_vs_lruvec_stat_mod_folio as lruvec_stat_mod_folio;
use b::rust_vs_mod_lruvec_state as mod_lruvec_state;
use b::rust_vs_update_lru_size as update_lru_size;
use b::rust_vs_lruvec_lock_irq as lruvec_lock_irq;
use b::rust_vs_lruvec_unlock_irq as lruvec_unlock_irq;
use b::rust_vs_folio_lruvec_lock_irq as folio_lruvec_lock_irq;
use b::rust_vs_folio_lruvec_relock_irq as folio_lruvec_relock_irq;
use b::rust_vs_lruvec_add_folio as lruvec_add_folio;
use b::rust_vs_lruvec_del_folio as lruvec_del_folio;
use b::rust_vs_wake_throttle_isolated as wake_throttle_isolated;
use b::rust_vs_get_nr_swap_pages as get_nr_swap_pages;
use b::rust_vs_mem_cgroup_disabled as mem_cgroup_disabled;
use b::rust_vs_mem_cgroup_is_root as mem_cgroup_is_root;
#[cfg(CONFIG_CGROUP_WRITEBACK)]
use b::rust_vs_memory_cgroup_on_dfl as memory_cgroup_on_dfl;
use b::rust_vs_mem_cgroup_online as mem_cgroup_online;
use b::rust_vs_memcg_is_dying as memcg_is_dying;
use b::rust_vs_mem_cgroup_swappiness as mem_cgroup_swappiness;
use b::rust_vs_mem_cgroup_get_nr_swap_pages as mem_cgroup_get_nr_swap_pages;
use b::rust_vs_mem_cgroup_get_zone_lru_size as mem_cgroup_get_zone_lru_size;
use b::rust_vs_mem_cgroup_iter as mem_cgroup_iter;
use b::rust_vs_mem_cgroup_iter_break as mem_cgroup_iter_break;
use b::rust_vs_mem_cgroup_lruvec as mem_cgroup_lruvec;
use b::rust_vs_mem_cgroup_flush_stats_ratelimited as mem_cgroup_flush_stats_ratelimited;
use b::rust_vs_mem_cgroup_node_filter_allowed as mem_cgroup_node_filter_allowed;
use b::rust_vs_mem_cgroup_protection as mem_cgroup_protection;
use b::rust_vs_mem_cgroup_calculate_protection as mem_cgroup_calculate_protection;
use b::rust_vs_mem_cgroup_below_min as mem_cgroup_below_min;
use b::rust_vs_mem_cgroup_below_low as mem_cgroup_below_low;
use b::rust_vs_mem_cgroup_swap_full as mem_cgroup_swap_full;
use b::rust_vs_mem_cgroup_uncharge_folios as mem_cgroup_uncharge_folios;
use b::rust_vs_count_memcg_events as count_memcg_events;
use b::rust_vs_count_memcg_folio_events as count_memcg_folio_events;
use b::rust_vs_memcg_memory_event as memcg_memory_event;
use b::rust_vs_memcg1_soft_limit_reclaim as memcg1_soft_limit_reclaim;
use b::rust_vs_node_get_allowed_targets as node_get_allowed_targets;
use b::rust_vs_next_demotion_node as next_demotion_node;
use b::rust_vs_vmpressure as vmpressure;
use b::rust_vs_vmpressure_prio as vmpressure_prio;
use b::rust_vs_blk_start_plug as blk_start_plug;
use b::rust_vs_blk_finish_plug as blk_finish_plug;
use b::rust_vs_jiffies_to_usecs as jiffies_to_usecs;
use b::rust_vs_div64_u64 as div64_u64;
use b::rust_vs_div64_u64_round_up as div64_u64_round_up;
use b::rust_vs_try_to_unmap as try_to_unmap;
use b::rust_vs_register_vmscan_sysctl as register_vmscan_sysctl;
use b::rust_vs_match_reclaim_token as match_reclaim_token;
#[cfg(all(CONFIG_SYSFS, CONFIG_NUMA))]
use b::rust_vs_device_create_reclaim_file as device_create_reclaim_file;
#[cfg(all(CONFIG_SYSFS, CONFIG_NUMA))]
use b::rust_vs_device_remove_reclaim_file as device_remove_reclaim_file;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_atomic_long_read as atomic_long_read;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_atomic_long_set as atomic_long_set;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_bitmap_clear as bitmap_clear;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_bitmap_free as bitmap_free;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_bitmap_zalloc as bitmap_zalloc;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_cgroup_lock as cgroup_lock;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_cgroup_unlock as cgroup_unlock;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_cpus_read_lock as cpus_read_lock;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_cpus_read_unlock as cpus_read_unlock;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_folio_activate as folio_activate;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_folio_lru_gen as folio_lru_gen;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_folio_memcg as folio_memcg;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_folio_pgdat as folio_pgdat;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_get_mem_cgroup_from_folio as get_mem_cgroup_from_folio;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_get_mem_cgroup_from_mm as get_mem_cgroup_from_mm;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_get_online_mems as get_online_mems;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_put_online_mems as put_online_mems;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_get_random_u32_below as get_random_u32_below;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_is_vm_hugetlb_page as is_vm_hugetlb_page;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_kvmalloc as kvmalloc;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_list_del_init as list_del_init;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_list_move_tail as list_move_tail;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_list_splice_tail_init as list_splice_tail_init;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_lru_gen_from_seq as lru_gen_from_seq;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_lru_hist_from_seq as lru_hist_from_seq;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_lru_tier_from_refs as lru_tier_from_refs;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_lru_gen_folio_seq as lru_gen_folio_seq;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_lru_gen_is_active as lru_gen_is_active;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_lru_gen_update_size as lru_gen_update_size;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_lru_gen_add_folio as lru_gen_add_folio;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_lru_gen_del_folio as lru_gen_del_folio;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs___update_lru_size as __update_lru_size;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_lruvec_live_lock_irq as lruvec_live_lock_irq;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_mapping_unevictable as mapping_unevictable;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_mem_cgroup_from_task as mem_cgroup_from_task;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_mem_cgroup_get_from_id as mem_cgroup_get_from_id;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_mem_cgroup_id as mem_cgroup_id;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_mem_cgroup_put as mem_cgroup_put;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_mem_cgroup_tryget as mem_cgroup_tryget;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_mm_has_notifiers as mm_has_notifiers;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_mmap_read_trylock as mmap_read_trylock;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_mmap_read_unlock as mmap_read_unlock;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_mmgrab as mmgrab;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_mmdrop as mmdrop;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_mutex_lock as mutex_lock;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_mutex_trylock as mutex_trylock;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_mutex_unlock as mutex_unlock;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_need_resched as need_resched;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_pgdat_end_pfn as pgdat_end_pfn;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_rcu_read_lock as rcu_read_lock;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_rcu_read_unlock as rcu_read_unlock;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_spin_is_contended as spin_is_contended;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_spin_lock_irq as spin_lock_irq;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_spin_unlock_irq as spin_unlock_irq;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_spin_trylock as spin_trylock;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_vma_has_recency as vma_has_recency;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_vma_is_accessible as vma_is_accessible;
#[cfg(CONFIG_LRU_GEN)]
use b::rust_vs_vma_is_anonymous as vma_is_anonymous;
#[cfg(CONFIG_MEMCG)]
use b::rust_vs_cgroup_path as cgroup_path;
#[cfg(CONFIG_MEMCG)]
use b::rust_vs_mem_cgroup_update_lru_size as mem_cgroup_update_lru_size;
use b::rust_vs_native_numa_demotion_enabled as native_numa_demotion_enabled;
use b::rust_vs_native_total_swap_pages as native_total_swap_pages;
use b::rust_vs_native_numa_balancing_mode as native_numa_balancing_mode;
use b::rust_vs_native_buffer_heads_over_limit as native_buffer_heads_over_limit;
use b::rust_vs_folio_referenced as folio_referenced;
use b::rust_vs_alloc_migration_target as alloc_migration_target;
use b::rust_vs_migrate_pages as migrate_pages;
use b::rust_vs_unmap_poisoned_folio as unmap_poisoned_folio;
use b::rust_vs_read_zone_type as read_zone_type;
use b::rust_vs_write_zone_type as write_zone_type;
use b::rust_vs_trace_mm_vmscan_kswapd_sleep as trace_mm_vmscan_kswapd_sleep;
use b::rust_vs_trace_mm_vmscan_kswapd_wake as trace_mm_vmscan_kswapd_wake;
use b::rust_vs_trace_mm_vmscan_balance_pgdat_begin as trace_mm_vmscan_balance_pgdat_begin;
use b::rust_vs_trace_mm_vmscan_balance_pgdat_end as trace_mm_vmscan_balance_pgdat_end;
use b::rust_vs_trace_mm_vmscan_wakeup_kswapd as trace_mm_vmscan_wakeup_kswapd;
use b::rust_vs_trace_mm_vmscan_direct_reclaim_begin as trace_mm_vmscan_direct_reclaim_begin;
#[cfg(CONFIG_MEMCG)]
use b::rust_vs_trace_mm_vmscan_memcg_reclaim_begin as trace_mm_vmscan_memcg_reclaim_begin;
#[cfg(CONFIG_MEMCG)]
use b::rust_vs_trace_mm_vmscan_memcg_softlimit_reclaim_begin as trace_mm_vmscan_memcg_softlimit_reclaim_begin;
use b::rust_vs_trace_mm_vmscan_direct_reclaim_end as trace_mm_vmscan_direct_reclaim_end;
#[cfg(CONFIG_MEMCG)]
use b::rust_vs_trace_mm_vmscan_memcg_reclaim_end as trace_mm_vmscan_memcg_reclaim_end;
#[cfg(CONFIG_MEMCG)]
use b::rust_vs_trace_mm_vmscan_memcg_softlimit_reclaim_end as trace_mm_vmscan_memcg_softlimit_reclaim_end;
use b::rust_vs_trace_mm_vmscan_lru_isolate as trace_mm_vmscan_lru_isolate;
use b::rust_vs_trace_mm_vmscan_write_folio as trace_mm_vmscan_write_folio;
use b::rust_vs_trace_mm_vmscan_reclaim_pages as trace_mm_vmscan_reclaim_pages;
use b::rust_vs_trace_mm_vmscan_lru_shrink_inactive as trace_mm_vmscan_lru_shrink_inactive;
use b::rust_vs_trace_mm_vmscan_lru_shrink_active as trace_mm_vmscan_lru_shrink_active;
use b::rust_vs_trace_mm_vmscan_node_reclaim_begin as trace_mm_vmscan_node_reclaim_begin;
use b::rust_vs_trace_mm_vmscan_node_reclaim_end as trace_mm_vmscan_node_reclaim_end;
use b::rust_vs_trace_mm_vmscan_throttled as trace_mm_vmscan_throttled;
use b::rust_vs_trace_mm_vmscan_kswapd_reclaim_fail as trace_mm_vmscan_kswapd_reclaim_fail;
use b::rust_vs_trace_mm_vmscan_kswapd_clear_hopeless as trace_mm_vmscan_kswapd_clear_hopeless;
