// SPDX-License-Identifier: GPL-2.0-only
/* Compiler macros and registration metadata only. All e820.c algorithms are Rust. */
#include "e820_bindings.h"

void __noreturn __init rust_e820_bug(void)
{
	BUG();
	unreachable();
}

/* Ordinary kernel cpufeature.h semantics include disabled-feature masks,
 * __builtin_constant_p and ALTERNATIVE_TERNARY asm-goto patching. The translated
 * cpufeature_header.rs does not implement that alternative machinery; do not
 * replace this with boot_cpu_has() or the compressed-boot mode variable.
 */
bool __init rust_e820_cpu_has_la57(void)
{
	return cpu_feature_enabled(X86_FEATURE_LA57);
}

/* kmemdup is an alloc_hooks/FORTIFY compiler macro. Keep one expansion for
 * each original allocation site rather than collapsing profiling records.
 */
void *__init rust_e820_kmemdup_main(const void *src, size_t len)
{
	return kmemdup(src, len, GFP_KERNEL);
}
void *__init rust_e820_kmemdup_kexec(const void *src, size_t len)
{
	return kmemdup(src, len, GFP_KERNEL);
}
void *__init rust_e820_kmemdup_firmware(const void *src, size_t len)
{
	return kmemdup(src, len, GFP_KERNEL);
}

/* Native macros preserve configured PREL32/LTO initcall and KVM namespaces. */
#ifdef CONFIG_ACPI
extern int __init e820__register_nvs_regions(void);
core_initcall(e820__register_nvs_regions);
#endif
EXPORT_SYMBOL_FOR_KVM(e820__mapped_raw_any);
