// SPDX-License-Identifier: GPL-2.0
/* Header-only configured expression boundary; no translated C body lives here. */
#include "boot_bindings.h"
unsigned long lupos_boot_maxmem(void)
{
	return MAXMEM;
}
