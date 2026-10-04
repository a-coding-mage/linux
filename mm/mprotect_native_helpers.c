// SPDX-License-Identifier: GPL-2.0
/* Native primitive leaves and syscall registration/ABI metadata only.
 * All four syscall bodies and all original mprotect.c algorithms are Rust. */
#undef KBUILD_MODNAME
#define KBUILD_MODNAME "mprotect"
#include "mprotect_native_bindings.h"
extern long rust_mprotect_sys_mprotect(unsigned long, size_t, unsigned long);
SYSCALL_DEFINE3(mprotect, unsigned long, start, size_t, len, unsigned long, prot)
{ return rust_mprotect_sys_mprotect(start, len, prot); }
#ifdef CONFIG_ARCH_HAS_PKEYS
extern long rust_mprotect_sys_pkey_mprotect(unsigned long, size_t, unsigned long, int);
extern long rust_mprotect_sys_pkey_alloc(unsigned long, unsigned long);
extern long rust_mprotect_sys_pkey_free(int);
SYSCALL_DEFINE4(pkey_mprotect, unsigned long, start, size_t, len,
               unsigned long, prot, int, pkey)
{ return rust_mprotect_sys_pkey_mprotect(start, len, prot, pkey); }
SYSCALL_DEFINE2(pkey_alloc, unsigned long, flags, unsigned long, init_val)
{ return rust_mprotect_sys_pkey_alloc(flags, init_val); }
SYSCALL_DEFINE1(pkey_free, int, pkey)
{ return rust_mprotect_sys_pkey_free(pkey); }
#endif
