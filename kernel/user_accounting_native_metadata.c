// SPDX-License-Identifier: GPL-2.0-only
/* Native export/init metadata. The user-cache decision body stays in Rust. */
#error "SOURCE ONLY HOLD: user-accounting native metadata is not admitted"

#include "user_accounting_native_bindings.h"
#include <linux/export.h>

EXPORT_SYMBOL_GPL(free_uid);

static int __init uid_cache_init(void)
{
	return lupos_user_accounting_uid_cache_init();
}
subsys_initcall(uid_cache_init);
