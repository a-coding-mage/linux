/* SPDX-License-Identifier: GPL-2.0 */
#ifndef RUST_RMAP_NATIVE_DIAGNOSTICS_H
#define RUST_RMAP_NATIVE_DIAGNOSTICS_H
/* Original diagnostic expressions remain in their native macros: this retains
 * CONFIG_DEBUG_VM's non-evaluation and distinct WARN_ON_ONCE state per site. */
#define RR_DIAG(name, args, body) \
 void rust_rmap_##name args; void rust_rmap_##name args { body; }
RR_DIAG(diag_118, (struct anon_vma *anon_vma), VM_BUG_ON(atomic_read(&anon_vma->refcount)))
RR_DIAG(diag_clone_1, (struct vm_area_struct *dst, struct vm_area_struct *src, enum vma_operation operation), VM_WARN_ON_ONCE(operation != VMA_OP_FORK && dst->vm_mm != src->vm_mm))
RR_DIAG(diag_clone_2, (struct vm_area_struct *dst, struct vm_area_struct *src, enum vma_operation operation), VM_WARN_ON_ONCE(!src->anon_vma && !list_empty(&src->anon_vma_chain)))
RR_DIAG(diag_clone_3, (struct vm_area_struct *dst, struct vm_area_struct *src, enum vma_operation operation), VM_WARN_ON_ONCE(!src->anon_vma && dst->anon_vma))
RR_DIAG(diag_clone_4, (struct vm_area_struct *dst, struct vm_area_struct *src, enum vma_operation operation), VM_WARN_ON_ONCE(!list_empty(&dst->anon_vma_chain)))
RR_DIAG(diag_clone_5, (struct vm_area_struct *dst, struct vm_area_struct *src, enum vma_operation operation), VM_WARN_ON_ONCE(dst->anon_vma && dst->anon_vma != src->anon_vma))
RR_DIAG(diag_clone_6, (struct vm_area_struct *dst, struct vm_area_struct *src, enum vma_operation operation), VM_WARN_ON_ONCE(operation != VMA_OP_FORK && src->anon_vma && !dst->anon_vma))
RR_DIAG(diag_clone_7, (struct vm_area_struct *dst, struct vm_area_struct *src, enum vma_operation operation), VM_WARN_ON_ONCE(operation == VMA_OP_MERGE_UNFAULTED && !list_is_singular(&src->anon_vma_chain)))
#ifdef CONFIG_PER_VMA_LOCK
RR_DIAG(diag_clone_attached, (struct vm_area_struct *dst, enum vma_operation operation), VM_WARN_ON_ONCE(operation != VMA_OP_MERGE_UNFAULTED && vma_is_attached(dst)))
#endif
RR_DIAG(diag_unlink_empty, (struct vm_area_struct *vma), VM_WARN_ON_ONCE(!list_empty(&vma->anon_vma_chain)))
RR_DIAG(diag_unlink_counts_1, (struct anon_vma *anon_vma), VM_WARN_ON(anon_vma->num_children))
RR_DIAG(diag_unlink_counts_2, (struct anon_vma *anon_vma), VM_WARN_ON(anon_vma->num_active_vmas))
RR_DIAG(diag_get_anon_locked, (const struct folio *folio), VM_WARN_ON_FOLIO(!folio_test_locked(folio),folio))
RR_DIAG(diag_lock_anon_locked, (const struct folio *folio), VM_WARN_ON_FOLIO(!folio_test_locked(folio),folio))
RR_DIAG(diag_referenced_pmd, (void), WARN_ON_ONCE(1))
RR_DIAG(diag_referenced_device, (struct folio *folio), VM_WARN_ON_ONCE_FOLIO(folio_is_zone_device(folio),folio))
RR_DIAG(diag_mkclean_pmd, (void), WARN_ON_ONCE(1))
RR_DIAG(diag_mkclean_locked, (struct folio *folio), BUG_ON(!folio_test_locked(folio)))
RR_DIAG(diag_pfn_mkclean_address, (unsigned long address, struct vm_area_struct *vma), VM_BUG_ON_VMA(address == -EFAULT,vma))
RR_DIAG(diag_walk_anon_locked, (struct folio *folio), VM_WARN_ON_FOLIO(!folio_test_locked(folio),folio))
RR_DIAG(diag_walk_anon_present, (struct folio *folio, struct anon_vma *anon_vma), VM_BUG_ON_FOLIO(!anon_vma,folio))
RR_DIAG(diag_walk_anon_address, (unsigned long address, struct vm_area_struct *vma), VM_WARN_ON_ONCE_VMA(address == -EFAULT,vma))
RR_DIAG(diag_walk_file_mapping_1, (struct folio *folio, struct address_space *mapping, pgoff_t pgoff_start, unsigned long nr_pages), VM_WARN_ON_FOLIO(folio && mapping != folio_mapping(folio),folio))
RR_DIAG(diag_walk_file_mapping_2, (struct folio *folio, struct address_space *mapping, pgoff_t pgoff_start, unsigned long nr_pages), VM_WARN_ON_FOLIO(folio && pgoff_start != folio_pgoff(folio),folio))
RR_DIAG(diag_walk_file_mapping_3, (struct folio *folio, struct address_space *mapping, pgoff_t pgoff_start, unsigned long nr_pages), VM_WARN_ON_FOLIO(folio && nr_pages != folio_nr_pages(folio),folio))
RR_DIAG(diag_walk_file_address, (unsigned long address, struct vm_area_struct *vma), VM_BUG_ON_VMA(address == -EFAULT,vma))
RR_DIAG(diag_walk_file_locked, (struct folio *folio), VM_BUG_ON_FOLIO(!folio_test_locked(folio),folio))
RR_DIAG(diag_walk_locked_ksm, (struct folio *folio), VM_BUG_ON_FOLIO(folio_test_ksm(folio),folio))
RR_DIAG(diag_move_anon_1, (struct folio *folio, struct vm_area_struct *vma, struct anon_vma *anon_vma), VM_BUG_ON_FOLIO(!folio_test_locked(folio),folio))
RR_DIAG(diag_move_anon_2, (struct folio *folio, struct vm_area_struct *vma, struct anon_vma *anon_vma), VM_BUG_ON_VMA(!anon_vma,vma))
RR_DIAG(diag_set_anon, (struct anon_vma *anon_vma), BUG_ON(!anon_vma))
RR_DIAG(diag_check_anon_1, (const struct folio *folio, const struct page *page, struct vm_area_struct *vma, unsigned long address), VM_BUG_ON_FOLIO(folio_anon_vma(folio)->root != vma->anon_vma->root,folio))
RR_DIAG(diag_check_anon_2, (const struct folio *folio, const struct page *page, struct vm_area_struct *vma, unsigned long address), VM_BUG_ON_PAGE(page_pgoff(folio,page) != linear_anon_page_index(vma,address),page))
RR_DIAG(diag_add_anon, (struct folio *folio), VM_WARN_ON_FOLIO(!folio_test_anon(folio),folio))
RR_DIAG(diag_add_anon_pud, (void), WARN_ON_ONCE(1))
RR_DIAG(diag_add_anon_small_count, (struct folio *folio, struct page *page), VM_WARN_ON_FOLIO(!folio_test_large(folio) && PageAnonExclusive(page) && atomic_read(&folio->_mapcount) > 0,folio))
RR_DIAG(diag_add_anon_entire_count, (struct folio *folio, struct page *cur_page), VM_WARN_ON_FOLIO(folio_test_large(folio) && folio_entire_mapcount(folio) > 1 && PageAnonExclusive(cur_page),folio))
RR_DIAG(diag_add_anon_page_count, (struct folio *folio, struct page *cur_page), VM_WARN_ON_FOLIO(atomic_read(&cur_page->_mapcount) > 0 && PageAnonExclusive(cur_page),folio))
RR_DIAG(diag_add_anon_pmd_unsupported, (void), WARN_ON_ONCE(true))
RR_DIAG(diag_add_new_anon_1, (struct folio *folio, bool exclusive), VM_WARN_ON_FOLIO(folio_test_hugetlb(folio),folio))
RR_DIAG(diag_add_new_anon_2, (struct folio *folio, bool exclusive), VM_WARN_ON_FOLIO(!exclusive && !folio_test_locked(folio),folio))
RR_DIAG(diag_add_new_anon_range, (unsigned long address, int nr, struct vm_area_struct *vma), VM_WARN_ON_ONCE(address < vma->vm_start || address + (nr << PAGE_SHIFT) > vma->vm_end))
RR_DIAG(diag_add_file, (struct folio *folio), VM_WARN_ON_FOLIO(folio_test_anon(folio),folio))
RR_DIAG(diag_add_file_pmd_unsupported, (void), WARN_ON_ONCE(true))
RR_DIAG(diag_add_file_pud_unsupported, (void), WARN_ON_ONCE(true))
RR_DIAG(diag_remove_pmd_unsupported, (void), WARN_ON_ONCE(true))
RR_DIAG(diag_remove_pud_unsupported, (void), WARN_ON_ONCE(true))
RR_DIAG(diag_poisoned_hugetlb_1, (struct folio *folio, enum ttu_flags flags), VM_WARN_ON_ONCE_FOLIO(!folio_test_hwpoison(folio),folio))
RR_DIAG(diag_poisoned_hugetlb_2, (struct folio *folio, enum ttu_flags flags), VM_WARN_ON_ONCE(!(flags & TTU_HWPOISON)))
RR_DIAG(diag_poisoned_address, (unsigned long address, unsigned long walked_address), VM_WARN_ON_ONCE(address != walked_address))
RR_DIAG(diag_poisoned_pte_1, (pte_t pteval, struct folio *folio), VM_WARN_ON_ONCE(!pte_present(pteval)))
RR_DIAG(diag_poisoned_pte_2, (pte_t pteval, struct folio *folio), VM_WARN_ON_ONCE(pte_pfn(pteval) != folio_pfn(folio)))
RR_DIAG(diag_poisoned_rmap_locked, (enum ttu_flags flags), VM_WARN_ON_ONCE(!(flags & TTU_RMAP_LOCKED)))
RR_DIAG(diag_unmap_pte, (struct folio *folio, pte_t *pte), VM_BUG_ON_FOLIO(!pte,folio))
bool rust_rmap_diag_swapbacked_swapcache(struct folio *folio);
bool rust_rmap_diag_swapbacked_swapcache(struct folio *folio)
{ return WARN_ON_ONCE(folio_test_swapbacked(folio) != folio_test_swapcache(folio)); }
RR_DIAG(diag_migrate_pmd, (struct folio *folio), VM_BUG_ON_FOLIO(folio_test_hugetlb(folio) || !folio_test_pmd_mappable(folio),folio))
RR_DIAG(diag_migrate_pte, (struct folio *folio, pte_t *pte), VM_BUG_ON_FOLIO(!pte,folio))
RR_DIAG(diag_migrate_nonpresent_huge, (struct folio *folio), VM_WARN_ON_FOLIO(folio_test_hugetlb(folio),folio))
RR_DIAG(diag_migrate_rmap_locked, (enum ttu_flags flags), VM_BUG_ON(!(flags & TTU_RMAP_LOCKED)))
RR_DIAG(diag_migrate_writable, (struct folio *folio, bool writable, bool anon_exclusive), VM_WARN_ON_FOLIO(writable && folio_test_anon(folio) && !anon_exclusive,folio))
RR_DIAG(diag_migrate_poison_private, (struct folio *folio), VM_WARN_ON_FOLIO(folio_is_device_private(folio),folio))
bool rust_rmap_diag_migrate_flags(enum ttu_flags flags);
bool rust_rmap_diag_migrate_flags(enum ttu_flags flags)
{ return WARN_ON_ONCE(flags & ~(TTU_RMAP_LOCKED | TTU_SPLIT_HUGE_PMD | TTU_SYNC | TTU_BATCH_FLUSH)); }
#ifdef CONFIG_HUGETLB_PAGE
RR_DIAG(diag_hugetlb_add_1, (struct folio *folio), VM_WARN_ON_FOLIO(!folio_test_hugetlb(folio),folio))
RR_DIAG(diag_hugetlb_add_2, (struct folio *folio), VM_WARN_ON_FOLIO(!folio_test_anon(folio),folio))
RR_DIAG(diag_hugetlb_add_exclusive, (struct folio *folio), VM_WARN_ON_FOLIO(folio_entire_mapcount(folio) > 1 && PageAnonExclusive(&folio->page),folio))
RR_DIAG(diag_hugetlb_new_1, (struct folio *folio, struct vm_area_struct *vma, unsigned long address), VM_WARN_ON_FOLIO(!folio_test_hugetlb(folio),folio))
RR_DIAG(diag_hugetlb_new_2, (struct folio *folio, struct vm_area_struct *vma, unsigned long address), BUG_ON(address < vma->vm_start || address >= vma->vm_end))
#endif
#undef RR_DIAG
#endif
