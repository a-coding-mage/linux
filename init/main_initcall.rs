// SPDX-License-Identifier: GPL-2.0-only
//! Original single-initcall ordering, diagnostics and context repair.

use super::{
    bindings, main_blacklist, main_initcall_context as context, main_initcall_trace,
    main_initcall_types::InitcallFn, main_warn::main_warn,
};
use kernel::ffi::{c_char, c_int, c_void};

/// Execute one permitted initcall and repair leaked preemption/IRQ state.
///
/// # Safety
/// `function` is a live initcall for the current boot/module phase. The caller
/// provides kernel current/per-CPU state and may re-enable local interrupts.
#[no_mangle]
#[cfg_attr(not(CONFIG_MODULES), link_section = ".init.text")]
pub unsafe extern "C" fn do_one_initcall(function: InitcallFn) -> c_int {
    // SAFETY: the initcall contract supplies all kernel services and storage.
    // Each repair and diagnostic retains the original C ordering.
    unsafe {
        let count = context::preempt_count();
        if main_blacklist::initcall_blacklisted(function) {
            return -(bindings::EPERM as c_int);
        }
        main_initcall_trace::do_trace_initcall_start(function);
        let result = function();
        main_initcall_trace::do_trace_initcall_finish(function, result);

        let mut message = [0 as c_char; 64];
        if context::preempt_count() != count {
            bindings::sprintf(
                message.as_mut_ptr(),
                b"preemption imbalance \0".as_ptr().cast(),
            );
            context::preempt_count_set(count);
        }
        if context::irqs_disabled() {
            bindings::strlcat(
                message.as_mut_ptr(),
                b"disabled interrupts \0".as_ptr().cast(),
                message.len(),
            );
            context::local_irq_enable();
        }
        main_warn!(
            message[0] != 0,
            b"initcall %pS returned with %s\n\0",
            function as *const c_void,
            message.as_ptr()
        );

        #[cfg(LATENT_ENTROPY_PLUGIN)]
        bindings::add_device_randomness(
            core::ptr::addr_of!(bindings::latent_entropy).cast(),
            core::mem::size_of_val(&bindings::latent_entropy),
        );
        #[cfg(not(LATENT_ENTROPY_PLUGIN))]
        bindings::add_device_randomness(core::ptr::null(), 0);
        result
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
