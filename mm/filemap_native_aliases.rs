// SPDX-License-Identifier: GPL-2.0-only
// Generated from exact typed native header leaves; no original C bodies.
use b::rust_filemap_ERR_PTR as ERR_PTR;
use b::rust_filemap_FGF_GET_ORDER as FGF_GET_ORDER;
use b::rust_filemap_IS_DAX as IS_DAX;
use b::rust_filemap_IS_ERR as IS_ERR;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_PageHWPoison as PageHWPoison;
use b::rust_filemap___add_wait_queue_entry_tail as __add_wait_queue_entry_tail;
use b::rust_filemap___ffs as __ffs;
use b::rust_filemap___filemap_get_folio as __filemap_get_folio;
#[cfg(all(CONFIG_NUMA))]
use b::rust_filemap___folio_alloc_node_noprof as __folio_alloc_node_noprof;
use b::rust_filemap___folio_clear_locked as __folio_clear_locked;
use b::rust_filemap___folio_set_dropbehind as __folio_set_dropbehind;
use b::rust_filemap___folio_set_locked as __folio_set_locked;
use b::rust_filemap___folio_set_referenced as __folio_set_referenced;
use b::rust_filemap___remove_wait_queue as __remove_wait_queue;
use b::rust_filemap_acct_reclaim_writeback as acct_reclaim_writeback;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_add_mm_counter as add_mm_counter;
use b::rust_filemap_alloc_folio_c2018 as filemap_alloc_folio_c2018;
use b::rust_filemap_alloc_folio_c2630 as filemap_alloc_folio_c2630;
use b::rust_filemap_alloc_folio_c4119 as filemap_alloc_folio_c4119;
use b::rust_filemap_atomic_set as atomic_set;
use b::rust_filemap_balance_dirty_pages_ratelimited as balance_dirty_pages_ratelimited;
use b::rust_filemap_clear_bit as clear_bit;
use b::rust_filemap_clear_bit_unlock as clear_bit_unlock;
use b::rust_filemap_cond_resched as cond_resched;
use b::rust_filemap_cond_resched_rcu as cond_resched_rcu;
use b::rust_filemap_copy_folio_to_iter as copy_folio_to_iter;
#[cfg(all(CONFIG_CACHESTAT_SYSCALL))]
use b::rust_filemap_copy_from_user as copy_from_user;
#[cfg(all(CONFIG_CACHESTAT_SYSCALL))]
use b::rust_filemap_copy_to_user as copy_to_user;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_count_memcg_event_mm as count_memcg_event_mm;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_count_vm_event as count_vm_event;
#[cfg(all(CONFIG_NUMA))]
use b::rust_filemap_cpuset_do_page_mem_spread as cpuset_do_page_mem_spread;
#[cfg(all(CONFIG_NUMA))]
use b::rust_filemap_cpuset_mem_spread_node as cpuset_mem_spread_node;
use b::rust_filemap_current as current;
use b::rust_filemap_delayacct_thrashing_end as delayacct_thrashing_end;
use b::rust_filemap_delayacct_thrashing_start as delayacct_thrashing_start;
use b::rust_filemap_down_write_nested as down_write_nested;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_exec_folio_order as exec_folio_order;
use b::rust_filemap_fatal_signal_pending as fatal_signal_pending;
use b::rust_filemap_fault_flag_allow_retry_first as fault_flag_allow_retry_first;
#[cfg(all(CONFIG_CACHESTAT_SYSCALL))]
use b::rust_filemap_fd_empty as fd_empty;
#[cfg(all(CONFIG_CACHESTAT_SYSCALL))]
use b::rust_filemap_fd_file as fd_file;
#[cfg(all(CONFIG_CACHESTAT_SYSCALL))]
use b::rust_filemap_fdget as fdget;
#[cfg(all(CONFIG_CACHESTAT_SYSCALL))]
use b::rust_filemap_fdput as fdput;
use b::rust_filemap_file_accessed as file_accessed;
use b::rust_filemap_file_lock_ptr as file_lock_ptr;
use b::rust_filemap_file_mapping as file_mapping;
use b::rust_filemap_file_mode as file_mode;
use b::rust_filemap_file_owner_or_capable as file_owner_or_capable;
use b::rust_filemap_file_ra as file_ra;
use b::rust_filemap_file_wb_err_ptr as file_wb_err_ptr;
use b::rust_filemap_filemap_get_folio as filemap_get_folio;
use b::rust_filemap_filemap_invalidate_lock as filemap_invalidate_lock;
use b::rust_filemap_filemap_invalidate_lock_shared as filemap_invalidate_lock_shared;
use b::rust_filemap_filemap_invalidate_trylock_shared as filemap_invalidate_trylock_shared;
use b::rust_filemap_filemap_invalidate_unlock as filemap_invalidate_unlock;
use b::rust_filemap_filemap_invalidate_unlock_shared as filemap_invalidate_unlock_shared;
use b::rust_filemap_filemap_range_needs_writeback as filemap_range_needs_writeback;
use b::rust_filemap_flush_dcache_folio as flush_dcache_folio;
#[cfg(all(CONFIG_NUMA))]
use b::rust_filemap_folio_alloc_noprof as folio_alloc_noprof;
use b::rust_filemap_folio_batch_add as folio_batch_add;
use b::rust_filemap_folio_batch_count as folio_batch_count;
use b::rust_filemap_folio_batch_init as folio_batch_init;
use b::rust_filemap_folio_batch_release as folio_batch_release;
use b::rust_filemap_folio_clear_idle as folio_clear_idle;
use b::rust_filemap_folio_clear_reclaim as folio_clear_reclaim;
use b::rust_filemap_folio_clear_waiters as folio_clear_waiters;
use b::rust_filemap_folio_contains as folio_contains;
use b::rust_filemap_folio_file_page as folio_file_page;
use b::rust_filemap_folio_flags as folio_flags;
use b::rust_filemap_folio_flags_field as folio_flags_field;
use b::rust_filemap_folio_get as folio_get;
use b::rust_filemap_folio_index as folio_index;
use b::rust_filemap_folio_lock as folio_lock;
use b::rust_filemap_folio_mapcount as folio_mapcount;
use b::rust_filemap_folio_mapcount_ptr as folio_mapcount_ptr;
use b::rust_filemap_folio_mapped as folio_mapped;
use b::rust_filemap_folio_mapping_field as folio_mapping_field;
use b::rust_filemap_folio_needs_release as folio_needs_release;
use b::rust_filemap_folio_next_index as folio_next_index;
use b::rust_filemap_folio_nr_pages as folio_nr_pages;
use b::rust_filemap_folio_order as folio_order;
use b::rust_filemap_folio_page as folio_page;
use b::rust_filemap_folio_page0 as folio_page0;
use b::rust_filemap_folio_pgdat as folio_pgdat;
use b::rust_filemap_folio_pos as folio_pos;
use b::rust_filemap_folio_put as folio_put;
use b::rust_filemap_folio_put_refs as folio_put_refs;
use b::rust_filemap_folio_ref_add as folio_ref_add;
use b::rust_filemap_folio_ref_count as folio_ref_count;
use b::rust_filemap_folio_ref_dec as folio_ref_dec;
use b::rust_filemap_folio_ref_sub as folio_ref_sub;
use b::rust_filemap_folio_set_index as folio_set_index;
use b::rust_filemap_folio_set_mapping as folio_set_mapping;
use b::rust_filemap_folio_set_waiters as folio_set_waiters;
use b::rust_filemap_folio_shift as folio_shift;
use b::rust_filemap_folio_size as folio_size;
use b::rust_filemap_folio_test_active as folio_test_active;
use b::rust_filemap_folio_test_clear_dropbehind as folio_test_clear_dropbehind;
use b::rust_filemap_folio_test_dirty as folio_test_dirty;
use b::rust_filemap_folio_test_dropbehind as folio_test_dropbehind;
use b::rust_filemap_folio_test_hugetlb as folio_test_hugetlb;
use b::rust_filemap_folio_test_idle as folio_test_idle;
use b::rust_filemap_folio_test_large as folio_test_large;
use b::rust_filemap_folio_test_locked as folio_test_locked;
use b::rust_filemap_folio_test_pmd_mappable as folio_test_pmd_mappable;
use b::rust_filemap_folio_test_private_2 as folio_test_private_2;
use b::rust_filemap_folio_test_readahead as folio_test_readahead;
use b::rust_filemap_folio_test_reclaim as folio_test_reclaim;
use b::rust_filemap_folio_test_swapbacked as folio_test_swapbacked;
use b::rust_filemap_folio_test_uptodate as folio_test_uptodate;
use b::rust_filemap_folio_test_workingset as folio_test_workingset;
use b::rust_filemap_folio_test_writeback as folio_test_writeback;
use b::rust_filemap_folio_try_get as folio_try_get;
use b::rust_filemap_folio_trylock as folio_trylock;
use b::rust_filemap_folio_wait_locked as folio_wait_locked;
use b::rust_filemap_folio_wait_locked_killable as folio_wait_locked_killable;
use b::rust_filemap_folio_wait_writeback as folio_wait_writeback;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_folio_within_vma as folio_within_vma;
use b::rust_filemap_folio_xor_flags_has_waiters as folio_xor_flags_has_waiters;
use b::rust_filemap_generic_write_sync as generic_write_sync;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_get_order as get_order;
use b::rust_filemap_hash_ptr as hash_ptr;
use b::rust_filemap_i_blocksize as i_blocksize;
use b::rust_filemap_i_size_read as i_size_read;
use b::rust_filemap_i_size_write as i_size_write;
use b::rust_filemap_in_task as in_task;
use b::rust_filemap_init_sync_kiocb as init_sync_kiocb;
use b::rust_filemap_init_wait as init_wait;
use b::rust_filemap_init_waitqueue_head as init_waitqueue_head;
use b::rust_filemap_inode_blkbits as inode_blkbits;
use b::rust_filemap_inode_is_blk as inode_is_blk;
use b::rust_filemap_inode_lock as inode_lock;
use b::rust_filemap_inode_lock_ptr as inode_lock_ptr;
use b::rust_filemap_inode_mapping as inode_mapping;
use b::rust_filemap_inode_maxbytes as inode_maxbytes;
use b::rust_filemap_inode_superblock as inode_superblock;
use b::rust_filemap_inode_to_wb as inode_to_wb;
use b::rust_filemap_inode_unlock as inode_unlock;
use b::rust_filemap_iov_iter_count as iov_iter_count;
use b::rust_filemap_iov_iter_truncate as iov_iter_truncate;
#[cfg(all(CONFIG_CACHESTAT_SYSCALL))]
use b::rust_filemap_is_file_hugepages as is_file_hugepages;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_is_shared_maywrite as is_shared_maywrite;
use b::rust_filemap_list_del_init_careful as list_del_init_careful;
use b::rust_filemap_lruvec_stat_add_folio as lruvec_stat_add_folio;
use b::rust_filemap_lruvec_stat_mod_folio as lruvec_stat_mod_folio;
use b::rust_filemap_lruvec_stat_sub_folio as lruvec_stat_sub_folio;
use b::rust_filemap_mapping_align_index as mapping_align_index;
use b::rust_filemap_mapping_aops as mapping_aops;
use b::rust_filemap_mapping_can_writeback as mapping_can_writeback;
use b::rust_filemap_mapping_exiting as mapping_exiting;
use b::rust_filemap_mapping_flags as mapping_flags;
use b::rust_filemap_mapping_gfp_constraint as mapping_gfp_constraint;
use b::rust_filemap_mapping_gfp_mask as mapping_gfp_mask;
use b::rust_filemap_mapping_host as mapping_host;
use b::rust_filemap_mapping_i_pages as mapping_i_pages;
use b::rust_filemap_mapping_invalidate_lock as mapping_invalidate_lock;
use b::rust_filemap_mapping_large_folio_support as mapping_large_folio_support;
use b::rust_filemap_mapping_max_folio_order as mapping_max_folio_order;
use b::rust_filemap_mapping_max_folio_size as mapping_max_folio_size;
use b::rust_filemap_mapping_min_folio_nrbytes as mapping_min_folio_nrbytes;
use b::rust_filemap_mapping_min_folio_order as mapping_min_folio_order;
use b::rust_filemap_mapping_nrpages as mapping_nrpages;
use b::rust_filemap_mapping_set_update as mapping_set_update;
use b::rust_filemap_mapping_shrinkable as mapping_shrinkable;
use b::rust_filemap_mapping_tagged as mapping_tagged;
use b::rust_filemap_mapping_wb_err as mapping_wb_err;
use b::rust_filemap_mapping_writably_mapped as mapping_writably_mapped;
use b::rust_filemap_mark_inode_dirty as mark_inode_dirty;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_maybe_unlock_mmap_for_io as maybe_unlock_mmap_for_io;
use b::rust_filemap_mem_cgroup_charge as mem_cgroup_charge;
#[cfg(all(CONFIG_CACHESTAT_SYSCALL))]
use b::rust_filemap_mem_cgroup_flush_stats_ratelimited as mem_cgroup_flush_stats_ratelimited;
use b::rust_filemap_mem_cgroup_replace_folio as mem_cgroup_replace_folio;
use b::rust_filemap_mem_cgroup_uncharge as mem_cgroup_uncharge;
use b::rust_filemap_memalloc_noio_restore as memalloc_noio_restore;
use b::rust_filemap_memalloc_noio_save as memalloc_noio_save;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_mm_counter_file as mm_counter_file;
use b::rust_filemap_mod_node_page_state as mod_node_page_state;
use b::rust_filemap_need_resched as need_resched;
#[cfg(all(CONFIG_NUMA))]
use b::rust_filemap_numa_node_id as numa_node_id;
use b::rust_filemap_offset_in_folio as offset_in_folio;
use b::rust_filemap_page_folio as page_folio;
use b::rust_filemap_page_nth as page_nth;
use b::rust_filemap_pipe_buf_usage as pipe_buf_usage;
use b::rust_filemap_pipe_head_buf as pipe_head_buf;
use b::rust_filemap_pipe_is_full as pipe_is_full;
use b::rust_filemap_pipe_max_usage as pipe_max_usage;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_pmd_install_vmf as pmd_install_vmf;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_pmd_none as pmd_none;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_pmd_trans_huge as pmd_trans_huge;
use b::rust_filemap_psi_memstall_enter as psi_memstall_enter;
use b::rust_filemap_psi_memstall_leave as psi_memstall_leave;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_pte_none as pte_none;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_pte_offset_map_lock as pte_offset_map_lock;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_pte_offset_map_ro_nolock as pte_offset_map_ro_nolock;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_pte_unmap as pte_unmap;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_pte_unmap_unlock as pte_unmap_unlock;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_ptep_get as ptep_get;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_ptep_get_lockless as ptep_get_lockless;
use b::rust_filemap_ra_mmap_miss_read_once as ra_mmap_miss_read_once;
use b::rust_filemap_ra_mmap_miss_write_once as ra_mmap_miss_write_once;
#[cfg(all(CONFIG_CACHESTAT_SYSCALL, CONFIG_SWAP))]
use b::rust_filemap_radix_to_swp_entry as radix_to_swp_entry;
use b::rust_filemap_rcu_read_lock as rcu_read_lock;
use b::rust_filemap_rcu_read_unlock as rcu_read_unlock;
#[cfg(all(CONFIG_NUMA))]
use b::rust_filemap_read_mems_allowed_begin as read_mems_allowed_begin;
#[cfg(all(CONFIG_NUMA))]
use b::rust_filemap_read_mems_allowed_retry as read_mems_allowed_retry;
use b::rust_filemap_readahead_set_dropbehind as readahead_set_dropbehind;
use b::rust_filemap_release_fault_lock as release_fault_lock;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_sb_end_pagefault as sb_end_pagefault;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_sb_start_pagefault as sb_start_pagefault;
use b::rust_filemap_set_active_memcg as set_active_memcg;
use b::rust_filemap_set_current_state as set_current_state;
use b::rust_filemap_shmem_mapping as shmem_mapping;
use b::rust_filemap_signal_pending_state as signal_pending_state;
#[cfg(all(CONFIG_CACHESTAT_SYSCALL, CONFIG_SWAP))]
use b::rust_filemap_softleaf_is_swap as softleaf_is_swap;
#[cfg(all(CONFIG_MIGRATION))]
use b::rust_filemap_softleaf_to_folio as softleaf_to_folio;
use b::rust_filemap_spin_lock as spin_lock;
use b::rust_filemap_spin_lock_irq as spin_lock_irq;
use b::rust_filemap_spin_lock_irqsave as spin_lock_irqsave;
use b::rust_filemap_spin_unlock as spin_unlock;
use b::rust_filemap_spin_unlock_irq as spin_unlock_irq;
use b::rust_filemap_spin_unlock_irqrestore as spin_unlock_irqrestore;
use b::rust_filemap_sysctl_set_data as sysctl_set_data;
use b::rust_filemap_sysctl_set_extra1 as sysctl_set_extra1;
use b::rust_filemap_sysctl_set_maxlen as sysctl_set_maxlen;
use b::rust_filemap_sysctl_set_mode as sysctl_set_mode;
use b::rust_filemap_sysctl_set_proc_handler as sysctl_set_proc_handler;
use b::rust_filemap_sysctl_set_procname as sysctl_set_procname;
use b::rust_filemap_test_and_clear_bit as test_and_clear_bit;
use b::rust_filemap_test_and_set_bit as test_and_set_bit;
use b::rust_filemap_test_bit as test_bit;
use b::rust_filemap_trace_file_check_and_advance_wb_err as trace_file_check_and_advance_wb_err;
use b::rust_filemap_trace_filemap_set_wb_err as trace_filemap_set_wb_err;
use b::rust_filemap_trace_mm_filemap_add_to_page_cache as trace_mm_filemap_add_to_page_cache;
use b::rust_filemap_trace_mm_filemap_delete_from_page_cache as trace_mm_filemap_delete_from_page_cache;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_trace_mm_filemap_fault as trace_mm_filemap_fault;
use b::rust_filemap_trace_mm_filemap_get_pages as trace_mm_filemap_get_pages;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_trace_mm_filemap_map_pages as trace_mm_filemap_map_pages;
use b::rust_filemap_try_to_free_buffers as try_to_free_buffers;
use b::rust_filemap_unlocked_inode_to_wb_begin as unlocked_inode_to_wb_begin;
use b::rust_filemap_unlocked_inode_to_wb_end as unlocked_inode_to_wb_end;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_vma_end_pgoff as vma_end_pgoff;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_vma_file as vma_file;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_vma_is_shared_maywrite as vma_is_shared_maywrite;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_vma_mm as vma_mm;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_vma_set_vm_ops as vma_set_vm_ops;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_vma_start as vma_start;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_vma_start_pgoff as vma_start_pgoff;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_vma_vm_flags as vma_vm_flags;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_vmf_address as vmf_address;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_vmf_gfp_mask as vmf_gfp_mask;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_vmf_has_prealloc_pte as vmf_has_prealloc_pte;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_vmf_pgoff as vmf_pgoff;
#[cfg(all(CONFIG_MMU))]
use b::rust_filemap_vmf_vma as vmf_vma;
use b::rust_filemap_waitqueue_active as waitqueue_active;
use b::rust_filemap_wake_page_match as wake_page_match;
use b::rust_filemap_wb_stat_mod as wb_stat_mod;
use b::rust_filemap_wbc_attach_fdatawrite_inode as wbc_attach_fdatawrite_inode;
use b::rust_filemap_wbc_detach_inode as wbc_detach_inode;
use b::rust_filemap_xa_get_order as xa_get_order;
use b::rust_filemap_xa_is_sibling as xa_is_sibling;
use b::rust_filemap_xa_is_value as xa_is_value;
use b::rust_filemap_xa_lock_irq as xa_lock_irq;
use b::rust_filemap_xa_unlock_irq as xa_unlock_irq;
use b::rust_filemap_xas_advance as xas_advance;
use b::rust_filemap_xas_error as xas_error;
use b::rust_filemap_xas_get_order as xas_get_order;
use b::rust_filemap_xas_lock_irq as xas_lock_irq;
use b::rust_filemap_xas_next as xas_next;
use b::rust_filemap_xas_next_entry as xas_next_entry;
use b::rust_filemap_xas_prev as xas_prev;
use b::rust_filemap_xas_reload as xas_reload;
use b::rust_filemap_xas_reset as xas_reset;
use b::rust_filemap_xas_retry as xas_retry;
use b::rust_filemap_xas_set as xas_set;
use b::rust_filemap_xas_set_err as xas_set_err;
use b::rust_filemap_xas_set_order as xas_set_order;
use b::rust_filemap_xas_try_split as xas_try_split;
use b::rust_filemap_xas_try_split_min_order as xas_try_split_min_order;
use b::rust_filemap_xas_unlock_irq as xas_unlock_irq;
