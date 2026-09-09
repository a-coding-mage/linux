/* SPDX-License-Identifier: GPL-2.0 */

// Translated from irq_stack.h. C includes and build-system supplied symbols
// are intentionally omitted; they remain external dependencies of this file.

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! call_on_stack {
    ($stack:expr, $func:expr, $asm_call:expr $(, $argconstr:tt)*) => {{
        // The C macro uses a register variable in r11 and volatile inline
        // assembly to switch stacks, invoke the supplied call sequence, and
        // restore the original stack pointer.
        let mut tos = ($stack) as *mut core::ffi::c_void;
        unsafe {
            core::arch::asm!(
                "movq %rsp, ({tos})",
                "movq {tos}, %rsp",
                $asm_call,
                "popq %rsp",
                tos = inout("r11") tos,
                __func = const $func,
                options(nostack)
            );
        }
    }};
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! ASM_CALL_ARG0 {
    () => { "1: call {__func}\n" };
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! ASM_CALL_ARG1 {
    () => { concat!("movq {arg1}, %rdi\n", $crate::ASM_CALL_ARG0!()) };
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! ASM_CALL_ARG2 {
    () => { concat!("movq {arg2}, %rsi\n", $crate::ASM_CALL_ARG1!()) };
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! ASM_CALL_ARG3 {
    () => { concat!("movq {arg3}, %rdx\n", $crate::ASM_CALL_ARG2!()) };
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! call_on_irqstack {
    ($func:expr, $asm_call:expr $(, $argconstr:tt)*) => {
        $crate::call_on_stack!(__this_cpu_read!(hardirq_stack_ptr), $func, $asm_call $(, $argconstr)*)
    };
}

// C's __builtin_types_compatible_p/static_assert checks are represented by
// Rust's compile-time type assertions at each call site.
#[macro_export]
macro_rules! assert_function_type {
    ($func:expr, $proto:ty) => {{ let _: $proto = $func; }};
}

#[macro_export]
macro_rules! assert_arg_type {
    ($arg:expr, $proto:ty) => {{ let _: $proto = $arg; }};
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! call_on_irqstack_cond {
    ($func:expr, $regs:expr, $asm_call:expr, $constr:expr $(, $c_args:expr)*) => {{
        if user_mode($regs) || __this_cpu_read!(hardirq_stack_inuse) {
            irq_enter_rcu();
            $func($($c_args),*);
            irq_exit_rcu();
        } else {
            __this_cpu_write!(hardirq_stack_inuse, true);
            $crate::call_on_irqstack!($func, $asm_call, $constr);
            __this_cpu_write!(hardirq_stack_inuse, false);
        }
    }};
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! ASM_CALL_SYSVEC { () => {
    concat!("call irq_enter_rcu\n", $crate::ASM_CALL_ARG1!(), "call irq_exit_rcu\n")
}; }

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! SYSVEC_CONSTRAINTS { ($regs:expr) => { $regs }; }

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! run_sysvec_on_irqstack_cond {
    ($func:expr, $regs:expr) => {{
        $crate::assert_function_type!($func, unsafe extern "C" fn(*mut pt_regs));
        $crate::assert_arg_type!($regs, *mut pt_regs);
        $crate::call_on_irqstack_cond!($func, $regs, $crate::ASM_CALL_SYSVEC!(), $crate::SYSVEC_CONSTRAINTS!($regs), $regs);
    }};
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! ASM_CALL_IRQ { () => {
    concat!("call irq_enter_rcu\n", $crate::ASM_CALL_ARG2!(), "call irq_exit_rcu\n")
}; }

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! IRQ_CONSTRAINTS { ($regs:expr, $vector:expr) => { ($regs, $vector as u64) }; }

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! run_irq_on_irqstack_cond {
    ($func:expr, $regs:expr, $vector:expr) => {{
        $crate::assert_function_type!($func, unsafe extern "C" fn(*mut pt_regs, u32));
        $crate::assert_arg_type!($regs, *mut pt_regs);
        $crate::assert_arg_type!($vector, u32);
        $crate::call_on_irqstack_cond!($func, $regs, $crate::ASM_CALL_IRQ!(), $crate::IRQ_CONSTRAINTS!($regs, $vector), $regs, $vector);
    }};
}

#[cfg(all(target_arch = "x86_64", feature = "softirq_on_own_stack"))]
#[macro_export]
macro_rules! do_softirq_own_stack {
    () => {{
        __this_cpu_write!(hardirq_stack_inuse, true);
        $crate::call_on_irqstack!(__do_softirq, $crate::ASM_CALL_ARG0!());
        __this_cpu_write!(hardirq_stack_inuse, false);
    }};
}

#[cfg(not(target_arch = "x86_64"))]
#[macro_export]
macro_rules! run_sysvec_on_irqstack_cond {
    ($func:expr, $regs:expr) => {{ irq_enter_rcu(); $func($regs); irq_exit_rcu(); }};
}

#[cfg(not(target_arch = "x86_64"))]
#[macro_export]
macro_rules! run_irq_on_irqstack_cond {
    ($func:expr, $regs:expr, $vector:expr) => {{ irq_enter_rcu(); $func($regs, $vector); irq_exit_rcu(); }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
