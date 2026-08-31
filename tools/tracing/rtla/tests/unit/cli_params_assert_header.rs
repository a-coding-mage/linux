// SPDX-License-Identifier: GPL-2.0

// Depends on declarations from "../../src/timerlat.h" in the original C header.

/* Tracing Options */

macro_rules! CLI_ASSERT_SINGLE_EVENT {
    ($params:expr, $_system:expr, $_event:expr) => {{
        ck_assert_ptr_nonnull(($params).events);
        ck_assert_str_eq(unsafe { (*($params).events).system }, $_system);
        ck_assert_str_eq(unsafe { (*($params).events).event }, $_event);
        ck_assert_ptr_null(unsafe { (*($params).events).next });
    }};
}

macro_rules! CLI_ASSERT_SINGLE_FILTER {
    ($params:expr, $_filter:expr) => {{
        ck_assert_ptr_nonnull(($params).events);
        ck_assert_str_eq(unsafe { (*($params).events).filter }, $_filter);
        ck_assert_ptr_null(unsafe { (*($params).events).next });
    }};
}

macro_rules! CLI_ASSERT_SINGLE_TRIGGER {
    ($params:expr, $_trigger:expr) => {{
        ck_assert_ptr_nonnull(($params).events);
        ck_assert_str_eq(unsafe { (*($params).events).trigger }, $_trigger);
        ck_assert_ptr_null(unsafe { (*($params).events).next });
    }};
}

/* CPU Configuration */

macro_rules! CLI_ASSERT_CPUSET {
    ($params:expr, $_field:ident, $($cpu:expr),* $(,)?) => {{
        let cpus: &[i32] = &[$($cpu),*];
        let mut n: usize = 0;
        while n < cpus.len() {
            ck_assert(CPU_ISSET(cpus[n], &($params).$_field));
            n += 1;
        }
        ck_assert_int_eq(CPU_COUNT(&($params).$_field), n as _);
    }};
}

/* Auto Analysis and Actions */

macro_rules! CLI_OSNOISE_ASSERT_AUTO {
    ($params:expr, $osn_params:expr, $_stop:expr) => {{
        ck_assert_int_eq(($params).stop_us, $_stop);
        ck_assert_int_eq(($osn_params).threshold, 1);
        ck_assert_int_eq(($params).threshold_actions.len, 1);
        ck_assert_int_eq(($params).threshold_actions.list[0].type, ACTION_TRACE_OUTPUT);
        ck_assert_str_eq(
            ($params).threshold_actions.list[0].trace_output,
            "osnoise_trace.txt",
        );
    }};
}

macro_rules! CLI_TIMERLAT_ASSERT_AUTO {
    ($params:expr, $tlat_params:expr, $_threshold:expr) => {{
        ck_assert_int_eq(($params).stop_us, $_threshold);
        ck_assert_int_eq(($params).stop_total_us, $_threshold);
        ck_assert_int_eq(($tlat_params).print_stack, $_threshold);
        ck_assert_int_eq(($params).threshold_actions.len, 1);
        ck_assert_int_eq(($params).threshold_actions.list[0].type, ACTION_TRACE_OUTPUT);
        ck_assert_str_eq(
            ($params).threshold_actions.list[0].trace_output,
            "timerlat_trace.txt",
        );
    }};
}

macro_rules! CLI_TIMERLAT_ASSERT_AA_ONLY {
    ($params:expr, $tlat_params:expr, $_threshold:expr) => {{
        ck_assert_int_eq(($params).stop_us, $_threshold);
        ck_assert_int_eq(($params).stop_total_us, $_threshold);
        ck_assert_int_eq(($tlat_params).print_stack, $_threshold);
        ck_assert_int_eq(($params).threshold_actions.len, 0);
        ck_assert(($params).aa_only);
    }};
}

macro_rules! CLI_ASSERT_SINGLE_ACTION {
    ($params:expr, $_actions:ident, $_type:expr, $_arg:ident, int, $_value:expr) => {{
        ck_assert_int_eq(($params).$_actions.len, 1);
        ck_assert_int_eq(($params).$_actions.list[0].type, $_type);
        ck_assert_int_eq(($params).$_actions.list[0].$_arg, $_value);
    }};
    ($params:expr, $_actions:ident, $_type:expr, $_arg:ident, str, $_value:expr) => {{
        ck_assert_int_eq(($params).$_actions.len, 1);
        ck_assert_int_eq(($params).$_actions.list[0].type, $_type);
        ck_assert_str_eq(($params).$_actions.list[0].$_arg, $_value);
    }};
    ($params:expr, $_actions:ident, $_type:expr, $_arg:ident, ptr, $_value:expr) => {{
        ck_assert_int_eq(($params).$_actions.len, 1);
        ck_assert_int_eq(($params).$_actions.list[0].type, $_type);
        ck_assert_ptr_eq(($params).$_actions.list[0].$_arg, $_value);
    }};
    ($params:expr, $_actions:ident, $_type:expr, $_arg:ident, $_valtype:ident, $_value:expr) => {{
        ck_assert_int_eq(($params).$_actions.len, 1);
        ck_assert_int_eq(($params).$_actions.list[0].type, $_type);
        compile_error!("unsupported ck_assert value type in CLI_ASSERT_SINGLE_ACTION");
    }};
}
