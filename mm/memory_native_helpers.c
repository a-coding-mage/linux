// SPDX-License-Identifier: GPL-2.0-only
/* Original registration/export metadata and narrow native header boundaries.
 * All memory.c algorithm and decision bodies are defined in memory.rs. */
#undef KBUILD_MODNAME
#define KBUILD_MODNAME "memory"
#include "memory_native_bindings.h"
extern int randomize_va_space;
/* Original ctl_table object: native static metadata and function-pointer ABI. */
static const struct ctl_table mmu_sysctl_table[] = {
    {
        .procname = "randomize_va_space",
        .data = &randomize_va_space,
        .maxlen = sizeof(int),
        .mode = 0644,
        .proc_handler = proc_dointvec,
    },
};
void __init rust_memory_register_mmu_sysctl(void)
{
    register_sysctl_init("kernel", mmu_sysctl_table);
}
extern int rust_memory_init_mm_sysctl(void);
subsys_initcall(rust_memory_init_mm_sysctl);
extern int rust_memory_disable_randmaps(char *s);
__setup("norandmaps", rust_memory_disable_randmaps);
EXPORT_SYMBOL_GPL(zap_special_vma_range);
EXPORT_SYMBOL(vm_insert_pages);
EXPORT_SYMBOL(map_kernel_pages_prepare);
EXPORT_SYMBOL(map_kernel_pages_complete);
EXPORT_SYMBOL(vm_insert_page);
EXPORT_SYMBOL(vm_map_pages);
EXPORT_SYMBOL(vm_map_pages_zero);
EXPORT_SYMBOL(vmf_insert_pfn_prot);
EXPORT_SYMBOL(vmf_insert_pfn);
EXPORT_SYMBOL_GPL(vmf_insert_page_mkwrite);
EXPORT_SYMBOL(vmf_insert_mixed);
EXPORT_SYMBOL(remap_pfn_range);
EXPORT_SYMBOL(vm_iomap_memory);
EXPORT_SYMBOL_GPL(apply_to_page_range);
#include "memory_faults_metadata.inc"
