// SPDX-License-Identifier: GPL-2.0-only
#include "freezer_native_bindings.h"

/* Exact original non-GPL export classes; the functions are sole Rust bodies. */
EXPORT_SYMBOL(freezer_active);
EXPORT_SYMBOL(freezing_slow_path);
EXPORT_SYMBOL(__refrigerator);
EXPORT_SYMBOL(set_freezable);
