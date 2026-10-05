// SPDX-License-Identifier: GPL-2.0
/* Native header/architecture/diagnostic leaves and original ELF metadata only.
 * No algorithm from mm/migrate.c is implemented in this translation unit. */
#undef KBUILD_MODNAME
#define KBUILD_MODNAME "migrate"
#include "migrate_native_bindings.h"
#define RM_RET(type, name, args, expr) type rust_migrate_##name args { return (expr); }
#define RM_VOID(name, args, expr) void rust_migrate_##name args { expr; }
#include "migrate_native_primitives.def"
#include "migrate_native_diagnostics.def"
#undef RM_RET
#undef RM_VOID
EXPORT_SYMBOL_GPL(set_movable_ops);
EXPORT_SYMBOL(folio_migrate_mapping);
EXPORT_SYMBOL(folio_migrate_flags);
EXPORT_SYMBOL(migrate_folio);
#ifdef CONFIG_BUFFER_HEAD
EXPORT_SYMBOL(buffer_migrate_folio);
EXPORT_SYMBOL_GPL(buffer_migrate_folio_norefs);
#endif
EXPORT_SYMBOL_GPL(filemap_migrate_folio);
#ifdef CONFIG_NUMA_MIGRATION
extern int rust_migrate_kernel_move_pages(pid_t pid, unsigned long nr_pages,
        const void __user * __user *pages, const int __user *nodes,
        int __user *status, int flags);
/* Native syscall ABI table, argument conversion, tracing and metadata only. */
SYSCALL_DEFINE6(move_pages, pid_t, pid, unsigned long, nr_pages,
        const void __user * __user *, pages, const int __user *, nodes,
        int __user *, status, int, flags)
{
        return rust_migrate_kernel_move_pages(pid, nr_pages, pages, nodes, status, flags);
}
#endif
