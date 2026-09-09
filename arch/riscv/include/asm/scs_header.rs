/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header exposes assembler macros.  The assembler-only include and
 * header guard have no executable Rust equivalent.
 */

/* __ASSEMBLER__ */
/* asm/asm-offsets.h supplies TASK_TI_SCS_SP and related assembler symbols. */

/* CONFIG_SHADOW_CALL_STACK */

/* Load init_shadow_call_stack to gp. */
#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
macro_rules! scs_load_init_stack {
    () => {
        unsafe {
            core::arch::asm!("la gp, init_shadow_call_stack");
        }
    };
}

/* Load the per-CPU IRQ shadow call stack to gp. */
#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
macro_rules! scs_load_irq_stack {
    ($tmp:tt) => {
        unsafe {
            core::arch::asm!("load_per_cpu gp, irq_shadow_call_stack_ptr, {tmp}", tmp = in(reg) $tmp);
        }
    };
}

/* Load task_scs_sp(current) to gp. */
#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
macro_rules! scs_load_current {
    () => {
        unsafe {
            core::arch::asm!("REG_L gp, TASK_TI_SCS_SP(tp)");
        }
    };
}

/* Load task_scs_sp(current) to gp, but only if tp has changed. */
#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
macro_rules! scs_load_current_if_task_changed {
    ($prev:tt) => {
        unsafe {
            core::arch::asm!("beq {prev}, tp, _skip_scs", prev = in(reg) $prev);
        }
        scs_load_current!();
        /* _skip_scs: */
    };
}

/* Save gp to task_scs_sp(current). */
#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
macro_rules! scs_save_current {
    () => {
        unsafe {
            core::arch::asm!("REG_S gp, TASK_TI_SCS_SP(tp)");
        }
    };
}

/* !CONFIG_SHADOW_CALL_STACK */

#[cfg(not(feature = "CONFIG_SHADOW_CALL_STACK"))]
macro_rules! scs_load_init_stack {
    () => {};
}

#[cfg(not(feature = "CONFIG_SHADOW_CALL_STACK"))]
macro_rules! scs_load_irq_stack {
    ($tmp:tt) => {};
}

#[cfg(not(feature = "CONFIG_SHADOW_CALL_STACK"))]
macro_rules! scs_load_current {
    () => {};
}

#[cfg(not(feature = "CONFIG_SHADOW_CALL_STACK"))]
macro_rules! scs_load_current_if_task_changed {
    ($prev:tt) => {};
}

#[cfg(not(feature = "CONFIG_SHADOW_CALL_STACK"))]
macro_rules! scs_save_current {
    () => {};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
