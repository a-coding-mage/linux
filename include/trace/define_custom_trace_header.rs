/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Trace files that want to automate creation of all tracepoints defined
 * in their file should include this file. The following are macros that the
 * trace file may define:
 *
 * TRACE_SYSTEM defines the system the tracepoint is for
 *
 * TRACE_INCLUDE_FILE if the file name is something other than TRACE_SYSTEM.h
 *     This macro may be defined to tell define_trace.h what file to include.
 *     Note, leave off the ".h".
 *
 * TRACE_INCLUDE_PATH if the path is something other than core kernel include/trace
 *     then this macro can define the path to use. Note, the path is relative to
 *     define_trace.h, not the file including it. Full path names for out of tree
 *     modules must be used.
 */

/*
 * C preprocessor condition:
 *
 * #ifdef CREATE_CUSTOM_TRACE_EVENTS
 *
 * Prevent recursion:
 * #undef CREATE_CUSTOM_TRACE_EVENTS
 *
 * The following C macros are intentionally represented as comments because
 * they control preprocessing and have no direct executable Rust equivalent:
 *
 * #undef TRACE_CUSTOM_EVENT
 * #define TRACE_CUSTOM_EVENT(name, proto, args, tstruct, assign, print)
 *
 * #undef DEFINE_CUSTOM_EVENT
 * #define DEFINE_CUSTOM_EVENT(template, name, proto, args)
 *
 * #undef TRACE_INCLUDE
 * #undef __TRACE_INCLUDE
 *
 * #ifndef TRACE_INCLUDE_FILE
 * # define TRACE_INCLUDE_FILE TRACE_SYSTEM
 * # define UNDEF_TRACE_INCLUDE_FILE
 * #endif
 *
 * #ifndef TRACE_INCLUDE_PATH
 * # define __TRACE_INCLUDE(system) <trace/events/system.h>
 * # define UNDEF_TRACE_INCLUDE_PATH
 * #else
 * # define __TRACE_INCLUDE(system) __stringify(TRACE_INCLUDE_PATH/system.h)
 * #endif
 *
 * # define TRACE_INCLUDE(system) __TRACE_INCLUDE(system)
 *
 * Let the trace headers be reread:
 * #define TRACE_CUSTOM_MULTI_READ
 *
 * The source then includes TRACE_INCLUDE(TRACE_INCLUDE_FILE), and, when
 * TRACEPOINTS_ENABLED is defined, includes <trace/trace_custom_events.h>.
 * Those are external dependencies and are intentionally not implemented here.
 *
 * Cleanup performed by the C source:
 * #undef TRACE_CUSTOM_EVENT
 * #undef DECLARE_CUSTOM_EVENT_CLASS
 * #undef DEFINE_CUSTOM_EVENT
 * #undef TRACE_CUSTOM_MULTI_READ
 *
 * Only symbols defined in this file are undefined:
 * #ifdef UNDEF_TRACE_INCLUDE_FILE
 * # undef TRACE_INCLUDE_FILE
 * # undef UNDEF_TRACE_INCLUDE_FILE
 * #endif
 *
 * #ifdef UNDEF_TRACE_INCLUDE_PATH
 * # undef TRACE_INCLUDE_PATH
 * # undef UNDEF_TRACE_INCLUDE_PATH
 * #endif
 *
 * More files may be processed:
 * #define CREATE_CUSTOM_TRACE_POINTS
 *
 * #endif /* CREATE_CUSTOM_TRACE_POINTS */
 */

// Rust has no file-local equivalent for the C preprocessor state manipulated
// by this header; the complete conditional and include intent is preserved
// above for the eventual trace-event integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
