// SPDX-License-Identifier: GPL-2.0
/* Native syscall ABI/metadata and named primitives only. */
#undef KBUILD_MODNAME
#define KBUILD_MODNAME "mremap"
#include "mremap_native_bindings.h"
extern long rust_mremap_sys_mremap(unsigned long, unsigned long, unsigned long,
                                  unsigned long, unsigned long);
SYSCALL_DEFINE5(mremap, unsigned long, addr, unsigned long, old_len,
               unsigned long, new_len, unsigned long, flags,
               unsigned long, new_addr)
{
 return rust_mremap_sys_mremap(addr, old_len, new_len, flags, new_addr);
}
