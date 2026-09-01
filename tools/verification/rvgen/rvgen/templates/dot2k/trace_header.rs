// SPDX-License-Identifier: GPL-2.0

/*
 * Snippet to be included in rv_trace.h
 */

/*
 * C preprocessor condition preserved from the source template:
 *
 * #ifdef CONFIG_RV_MON_%%MODEL_NAME_UP%%
 *
 * The original header declares trace events through external C trace macros:
 *
 * DEFINE_EVENT(event_%%MONITOR_CLASS%%, event_%%MODEL_NAME%%,
 * %%TRACEPOINT_ARGS_SKEL_EVENT%%);
 *
 * DEFINE_EVENT(error_%%MONITOR_CLASS%%, error_%%MODEL_NAME%%,
 * %%TRACEPOINT_ARGS_SKEL_ERROR%%);
 *
 * #endif
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
