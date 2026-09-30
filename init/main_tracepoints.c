// SPDX-License-Identifier: GPL-2.0-only
/*
 * Shared tracing infrastructure for the staged Rust init/main owner.
 *
 * Keep the original CREATE_TRACE_POINTS expansion: tracepoint storage, event
 * and perf metadata, static calls, SRCU dispatch and LOCKDEP checks are still
 * supplied by the shared C tracing headers. Boot ordering, registration and
 * probe callbacks live in Rust. This file is not a tracing-subsystem rewrite.
 *
 * RUST_INIT_MAIN selects this owner instead of init/main.c's tracepoint
 * definitions; linking both owners is an error.
 */
#include <linux/init.h>

#define CREATE_TRACE_POINTS
#include <trace/events/initcall.h>

#ifdef TRACEPOINTS_ENABLED

void __init_or_module rust_main_trace_initcall_start(initcall_t function);
void __init_or_module rust_main_trace_initcall_finish(initcall_t function, int result);
void __init rust_main_trace_initcall_level(const char *level);

void __init_or_module rust_main_trace_initcall_start(initcall_t function)
{
	trace_initcall_start(function);
}

void __init_or_module rust_main_trace_initcall_finish(initcall_t function, int result)
{
	trace_initcall_finish(function, result);
}

void __init rust_main_trace_initcall_level(const char *level)
{
	trace_initcall_level(level);
}

#endif
