/* SPDX-License-Identifier: GPL-2.0-only */
/* Original mm/memory.c late-unit export and debugfs registration metadata. */
EXPORT_SYMBOL_GPL(unmap_mapping_pages);
EXPORT_SYMBOL(unmap_mapping_range);
EXPORT_SYMBOL_GPL(handle_mm_fault);
EXPORT_SYMBOL_GPL(follow_pfnmap_start);
EXPORT_SYMBOL_GPL(follow_pfnmap_end);
#ifdef CONFIG_HAVE_IOREMAP_PROT
EXPORT_SYMBOL_GPL(generic_access_phys);
#endif
EXPORT_SYMBOL_GPL(access_process_vm);
#ifdef CONFIG_BPF_SYSCALL
EXPORT_SYMBOL_GPL(copy_remote_vm_str);
#endif
#if defined(CONFIG_PROVE_LOCKING) || defined(CONFIG_DEBUG_ATOMIC_SLEEP)
EXPORT_SYMBOL(__might_fault);
#endif
#ifdef CONFIG_DEBUG_FS
extern int rust_memory_fault_around_bytes_get(void *, u64 *);
extern int rust_memory_fault_around_bytes_set(void *, u64);
extern int rust_memory_fault_around_debugfs(void);
DEFINE_DEBUGFS_ATTRIBUTE(rust_memory_fault_around_bytes_fops,
    rust_memory_fault_around_bytes_get, rust_memory_fault_around_bytes_set, "%llu\n");
void rust_memory_fault_around_debugfs_create(void)
{
    debugfs_create_file_unsafe("fault_around_bytes", 0644, NULL, NULL,
        &rust_memory_fault_around_bytes_fops);
}
late_initcall(rust_memory_fault_around_debugfs);
#endif
