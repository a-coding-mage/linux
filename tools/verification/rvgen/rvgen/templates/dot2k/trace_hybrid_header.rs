/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Snippet to be included in rv_trace.h
 */

/*
 * C preprocessor condition preserved from the source:
 * #ifdef CONFIG_RV_MON_%%MODEL_NAME_UP%%
 *
 * The following C tracepoint template declarations depend on external
 * preprocessor macros and template placeholders:
 *
 * DEFINE_EVENT(event_%%MONITOR_CLASS%%, event_%%MODEL_NAME%%,
 * %%TRACEPOINT_ARGS_SKEL_EVENT%%);
 *
 * DEFINE_EVENT(error_%%MONITOR_CLASS%%, error_%%MODEL_NAME%%,
 * %%TRACEPOINT_ARGS_SKEL_ERROR%%);
 *
 * DEFINE_EVENT(error_env_%%MONITOR_CLASS%%, error_env_%%MODEL_NAME%%,
 * %%TRACEPOINT_ARGS_SKEL_ERROR_ENV%%);
 *
 * #endif // CONFIG_RV_MON_%%MODEL_NAME_UP%%
 */
