/* SPDX-License-Identifier: GPL-2.0 */
#ifndef RUST_MIGRATE_NATIVE_BINDINGS_H
#define RUST_MIGRATE_NATIVE_BINDINGS_H
#include "migrate_native_types.h"
#include "migrate_native_constants.h"
#define RM_RET(type, name, args, expr) type rust_migrate_##name args;
#define RM_VOID(name, args, expr) void rust_migrate_##name args;
#include "migrate_native_primitives.def"
#include "migrate_native_diagnostics.def"
#undef RM_RET
#undef RM_VOID
#endif
