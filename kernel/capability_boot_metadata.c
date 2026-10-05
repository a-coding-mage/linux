// SPDX-License-Identifier: GPL-2.0
/* Native boot metadata only; capability.rs retains the policy body and state. */
#error "SOURCE ONLY HOLD: capability boot metadata is not admitted"

#include <linux/init.h>

#include "capability_boot_native.h"

/* Keep every configured __init attribute on the parser's native entry. */
static int __init file_caps_disable(char *str)
{
	return lupos_capability_file_caps_disable(str);
}
__setup("no_file_caps", file_caps_disable);
