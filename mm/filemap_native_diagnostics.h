/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_FILEMAP_NATIVE_DIAGNOSTICS_H
#define RUST_FILEMAP_NATIVE_DIAGNOSTICS_H
/* Diagnostic leaves retain kernel BUG/taint/once/ratelimit machinery. Each
 * WARN_ON_ONCE source site owns a separate native once flag. */
void rust_filemap_vm_bug_folio(bool condition, struct folio *folio);
void rust_filemap_vm_bug_folio(bool condition, struct folio *folio)
{
    VM_BUG_ON_FOLIO(condition, folio);
}
void rust_filemap_bug(bool condition);
void rust_filemap_bug(bool condition) { BUG_ON(condition); }
#define RF_WARN(name) \
 bool rust_filemap_##name(bool c); \
 bool rust_filemap_##name(bool c) { return WARN_ON_ONCE(c); }
RF_WARN(warn_dirty_unaccount)
RF_WARN(warn_delete_unlocked)
RF_WARN(warn_add_active)
RF_WARN(warn_get_unlocked)
#undef RF_WARN
#ifndef CONFIG_DEBUG_VM
void rust_filemap_bad_page_cache(struct folio *folio);
void rust_filemap_bad_page_cache(struct folio *folio)
{
    pr_alert("BUG: Bad page cache in process %s  pfn:%05lx\n",
             current->comm, folio_pfn(folio));
    dump_page(&folio->page, "still mapped when deleted");
    dump_stack();
    add_taint(TAINT_BAD_PAGE, LOCKDEP_NOW_UNRELIABLE);
}
#endif
bool rust_filemap_dio_ratelimit(void);
bool rust_filemap_dio_ratelimit(void)
{
    static DEFINE_RATELIMIT_STATE(_rs, 86400 * HZ, DEFAULT_RATELIMIT_BURST);
    return __ratelimit(&_rs);
}
void rust_filemap_dio_print(const char *path);
void rust_filemap_dio_print(const char *path)
{
    pr_crit("Page cache invalidation failure on direct I/O.  Possible data corruption due to collision with buffered I/O!\n");
    pr_crit("File: %s PID: %d Comm: %.20s\n", path, current->pid, current->comm);
}
#endif
