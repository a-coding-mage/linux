// SPDX-License-Identifier: GPL-2.0-only
#error "SOURCE ONLY HOLD: usermode-helper init metadata is unqualified"
/* Inactive native entry/metadata; initialization policy remains in umh.rs. */
#include "umh_native_bindings.h"

#ifdef CONFIG_SYSCTL
static int __init init_umh_sysctls(void)
{
	return lupos_umh_init_sysctls();
}
early_initcall(init_umh_sysctls);
#endif
