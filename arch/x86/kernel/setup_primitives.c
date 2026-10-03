// SPDX-License-Identifier: GPL-2.0-only
/* Architecture/compiler macros and initcall metadata, no setup.c algorithms. */
#include "setup_bindings.h"

unsigned long rust_setup_pa_symbol(unsigned long address) { return __pa_symbol(address); }
void __noreturn rust_setup_bug(void) { BUG(); unreachable(); }
void rust_setup_memzero_explicit(void *address, size_t size) { memzero_explicit(address, size); }
bool rust_setup_boot_cpu_has_nx(void) { return boot_cpu_has(X86_FEATURE_NX); }
unsigned long __init rust_setup_read_cr4(void) { return __read_cr4(); }
unsigned int __init rust_setup_max_physmem_bits(void) { return MAX_PHYSMEM_BITS; }
bool __init rust_setup_efi_enabled(unsigned int bit) { return efi_enabled(bit); }
void __init rust_setup_set_efi_flag(unsigned int bit) { set_bit(bit, &efi.flags); }
bool __init rust_setup_xen_pv_domain(void) { return xen_pv_domain(); }

extern int __init rust_setup_init_x86_sysctl(void);
extern int __init rust_setup_register_kernel_offset_dumper(void);
arch_initcall(rust_setup_init_x86_sysctl);
__initcall(rust_setup_register_kernel_offset_dumper);
