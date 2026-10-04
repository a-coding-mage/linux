// SPDX-License-Identifier: GPL-2.0-only
/* Syscall ABI, registration/export/trace metadata, and native primitive leaves.
 * mmap.c algorithms and its eleven explicit named globals are Rust-owned.
 * Macro-created once-state, lock keys, and registration/trace/export metadata
 * remain native; DATA.tsv inventories that storage boundary explicitly. */
#define RUST_MMAP_NATIVE_CREATE_TRACE_POINTS
#undef KBUILD_MODNAME
#define KBUILD_MODNAME "mmap"
#include "mmap_native_bindings.h"
extern bool ignore_rlimit_data;
core_param(ignore_rlimit_data, ignore_rlimit_data, bool, 0644);
extern int cmdline_parse_stack_guard_gap(char *p);
__setup("stack_guard_gap=", cmdline_parse_stack_guard_gap);
extern int init_user_reserve(void);
extern int init_admin_reserve(void);
extern int init_reserve_notifier(void);
subsys_initcall(init_user_reserve);
subsys_initcall(init_admin_reserve);
subsys_initcall(init_reserve_notifier);
extern long rust_mmap_sys_brk(unsigned long);
extern long rust_mmap_sys_mmap_pgoff(unsigned long, unsigned long, unsigned long,
                                  unsigned long, unsigned long, unsigned long);
extern long rust_mmap_sys_munmap(unsigned long, size_t);
extern long rust_mmap_sys_remap_file_pages(unsigned long, unsigned long,
                                         unsigned long, unsigned long, unsigned long);
SYSCALL_DEFINE1(brk, unsigned long, brk) { return rust_mmap_sys_brk(brk); }
SYSCALL_DEFINE6(mmap_pgoff, unsigned long, addr, unsigned long, len,
               unsigned long, prot, unsigned long, flags,
               unsigned long, fd, unsigned long, pgoff)
{ return rust_mmap_sys_mmap_pgoff(addr, len, prot, flags, fd, pgoff); }
#ifdef __ARCH_WANT_SYS_OLD_MMAP
extern long rust_mmap_sys_old_mmap(struct mmap_arg_struct __user *);
SYSCALL_DEFINE1(old_mmap, struct mmap_arg_struct __user *, arg)
{ return rust_mmap_sys_old_mmap(arg); }
#endif
SYSCALL_DEFINE2(munmap, unsigned long, addr, size_t, len)
{ return rust_mmap_sys_munmap(addr, len); }
SYSCALL_DEFINE5(remap_file_pages, unsigned long, start, unsigned long, size,
               unsigned long, prot, unsigned long, pgoff, unsigned long, flags)
{ return rust_mmap_sys_remap_file_pages(start, size, prot, pgoff, flags); }
EXPORT_SYMBOL(mm_get_unmapped_area);
EXPORT_SYMBOL(find_vma_intersection);
EXPORT_SYMBOL(find_vma);
EXPORT_SYMBOL(vm_munmap);
