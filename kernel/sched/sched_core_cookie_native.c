// SPDX-License-Identifier: GPL-2.0-only
/* Real native leaves for the existing core_sched.rs algorithms. This file
 * does not include or forward any original core_sched.c owner body. */
#include "sched_core_cookie_bindings.h"

#ifdef CONFIG_RUST_SCHED_CORE_COOKIE
#error "SOURCE ONLY HOLD: scheduler core-cookie owner is not admitted"
#endif

#ifdef CONFIG_SCHED_CORE
#define LUPOS_CORE_COOKIE_VALUE(type, name, args, ...) \
	type name args { return (__VA_ARGS__); }
#define LUPOS_CORE_COOKIE_VOID(name, args, ...) \
	void name args { __VA_ARGS__; }
#define LUPOS_CORE_COOKIE_BODY(type, name, args, ...) \
	type name args { __VA_ARGS__ }
#include "sched_core_cookie_native_leaves.def"
#undef LUPOS_CORE_COOKIE_VALUE
#undef LUPOS_CORE_COOKIE_VOID
#undef LUPOS_CORE_COOKIE_BODY
#endif /* CONFIG_SCHED_CORE */
