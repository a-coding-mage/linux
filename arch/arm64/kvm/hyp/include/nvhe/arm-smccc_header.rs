/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency intent from the C header:
//   <asm/kvm_hypevents.h>
//   <linux/arm-smccc.h>
// The symbols used below are supplied by those dependencies.

macro_rules! hyp_smccc_1_1_smc {
    ($($arg:expr),* $(,)?) => {{
        trace_hyp_exit(core::ptr::null_mut(), HYP_REASON_SMC);
        arm_smccc_1_1_smc!($($arg),*);
        trace_hyp_enter(core::ptr::null_mut(), HYP_REASON_SMC);
    }};
}

macro_rules! hyp_smccc_1_2_smc {
    ($($arg:expr),* $(,)?) => {{
        trace_hyp_exit(core::ptr::null_mut(), HYP_REASON_SMC);
        arm_smccc_1_2_smc!($($arg),*);
        trace_hyp_enter(core::ptr::null_mut(), HYP_REASON_SMC);
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
