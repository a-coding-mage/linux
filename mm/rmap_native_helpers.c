// SPDX-License-Identifier: GPL-2.0
/* Native header/architecture/diagnostic leaves and original ELF metadata only.
 * No owner function from rmap.c is defined here. */
#undef KBUILD_MODNAME
#define KBUILD_MODNAME "rmap"
#include "rmap_native_bindings.h"
#define CREATE_TRACE_POINTS
#include <trace/events/migrate.h>
EXPORT_SYMBOL_GPL(folio_mkclean);
EXPORT_SYMBOL_GPL(mapping_wrprotect_range);
#ifdef CONFIG_DEVICE_PRIVATE
EXPORT_SYMBOL_GPL(make_device_exclusive);
#endif
