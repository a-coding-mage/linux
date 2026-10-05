// SPDX-License-Identifier: GPL-2.0-only
/* Only native header/arch/atomic/diagnostic/trace/export/syscall ABI leaves.
 * No original mm/filemap.c algorithm remains in this translation unit. */
#define CREATE_TRACE_POINTS
#include "filemap_native_includes.h"
#include "filemap_native_primitives.h"
#include "filemap_native_diagnostics.h"

int __filemap_add_folio(struct address_space *mapping, struct folio *folio,
                      pgoff_t index, gfp_t gfp, void **shadowp);
ALLOW_ERROR_INJECTION(__filemap_add_folio, ERRNO);
EXPORT_SYMBOL(filemap_check_errors);
EXPORT_SYMBOL(filemap_fdatawrite_range);
EXPORT_SYMBOL(filemap_fdatawrite);
EXPORT_SYMBOL_GPL(filemap_flush_range);
EXPORT_SYMBOL(filemap_flush);
EXPORT_SYMBOL_FOR_MODULES(filemap_flush_nr, "btrfs");
EXPORT_SYMBOL(filemap_range_has_page);
EXPORT_SYMBOL(filemap_fdatawait_range);
EXPORT_SYMBOL(filemap_fdatawait_range_keep_errors);
EXPORT_SYMBOL(file_fdatawait_range);
EXPORT_SYMBOL(filemap_fdatawait_keep_errors);
EXPORT_SYMBOL_GPL(filemap_range_has_writeback);
EXPORT_SYMBOL(filemap_write_and_wait_range);
EXPORT_SYMBOL(__filemap_set_wb_err);
EXPORT_SYMBOL(file_check_and_advance_wb_err);
EXPORT_SYMBOL(file_write_and_wait_range);
EXPORT_SYMBOL_GPL(replace_page_cache_folio);
EXPORT_SYMBOL_GPL(filemap_add_folio);
#ifdef CONFIG_NUMA
EXPORT_SYMBOL(filemap_alloc_folio_noprof);
#endif
EXPORT_SYMBOL(filemap_invalidate_lock_two);
EXPORT_SYMBOL(filemap_invalidate_unlock_two);
EXPORT_SYMBOL(folio_wait_bit);
EXPORT_SYMBOL(folio_wait_bit_killable);
EXPORT_SYMBOL(folio_unlock);
EXPORT_SYMBOL(folio_end_read);
EXPORT_SYMBOL(folio_end_private_2);
EXPORT_SYMBOL(folio_wait_private_2);
EXPORT_SYMBOL(folio_wait_private_2_killable);
EXPORT_SYMBOL_GPL(folio_end_dropbehind);
EXPORT_SYMBOL_GPL(folio_end_writeback_no_dropbehind);
EXPORT_SYMBOL(folio_end_writeback);
EXPORT_SYMBOL(__folio_lock);
EXPORT_SYMBOL_GPL(__folio_lock_killable);
EXPORT_SYMBOL(page_cache_next_miss);
EXPORT_SYMBOL(page_cache_prev_miss);
EXPORT_SYMBOL(__filemap_get_folio_mpol);
EXPORT_SYMBOL(filemap_get_folios);
EXPORT_SYMBOL(filemap_get_folios_contig);
EXPORT_SYMBOL(filemap_get_folios_tag);
EXPORT_SYMBOL_GPL(filemap_read);
EXPORT_SYMBOL_GPL(kiocb_write_and_wait);
EXPORT_SYMBOL_GPL(kiocb_invalidate_pages);
EXPORT_SYMBOL(generic_file_read_iter);
EXPORT_SYMBOL(filemap_splice_read);
#ifdef CONFIG_MMU
EXPORT_SYMBOL(filemap_fault);
#endif
#ifdef CONFIG_MMU
EXPORT_SYMBOL(filemap_map_pages);
#endif
EXPORT_SYMBOL(filemap_page_mkwrite);
EXPORT_SYMBOL(generic_file_mmap);
EXPORT_SYMBOL(generic_file_mmap_prepare);
EXPORT_SYMBOL(generic_file_readonly_mmap);
EXPORT_SYMBOL(generic_file_readonly_mmap_prepare);
EXPORT_SYMBOL(read_cache_folio);
EXPORT_SYMBOL(mapping_read_folio_gfp);
EXPORT_SYMBOL(read_cache_page);
EXPORT_SYMBOL(read_cache_page_gfp);
EXPORT_SYMBOL(generic_file_direct_write);
EXPORT_SYMBOL(generic_perform_write);
EXPORT_SYMBOL(__generic_file_write_iter);
EXPORT_SYMBOL(generic_file_write_iter);
EXPORT_SYMBOL(filemap_release_folio);
EXPORT_SYMBOL_GPL(filemap_invalidate_inode);
#ifdef CONFIG_CACHESTAT_SYSCALL
long rust_filemap_cachestat_syscall(unsigned int fd,
    struct cachestat_range __user *cstat_range,
    struct cachestat __user *cstat, unsigned int flags);
SYSCALL_DEFINE4(cachestat, unsigned int, fd,
    struct cachestat_range __user *, cstat_range,
    struct cachestat __user *, cstat, unsigned int, flags)
{
    return rust_filemap_cachestat_syscall(fd, cstat_range, cstat, flags);
}
#endif
