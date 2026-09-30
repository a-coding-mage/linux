// SPDX-License-Identifier: GPL-2.0-only
// Already validated siblings remain observable boundaries for ordering tests.
#[cfg(fixture_host)]
mod main_initcall_context {
    unsafe extern "C" {
        fn fixture_irq_disable(); fn fixture_irq_enable();
        fn fixture_irqs_disabled() -> bool;
    }
    pub(super) unsafe fn local_irq_disable() { unsafe { fixture_irq_disable(); } }
    pub(super) unsafe fn local_irq_enable() { unsafe { fixture_irq_enable(); } }
    pub(super) unsafe fn irqs_disabled() -> bool { unsafe { fixture_irqs_disabled() } }
}
#[cfg(fixture_host)]
mod main_start_arch {
    unsafe extern "C" { fn fixture_early_numa(); fn fixture_canary(); }
    pub(super) unsafe fn early_numa_node_init() { unsafe { fixture_early_numa(); } }
    pub(super) unsafe fn boot_init_stack_canary() { unsafe { fixture_canary(); } }
}
#[cfg(fixture_host)]
mod main_warn {
    unsafe extern "C" { pub(crate) fn fixture_warn(value: bool, format: *const kernel::ffi::c_char) -> bool; }
    macro_rules! main_warn { ($value:expr, $format:expr) => {
        $crate::main_warn::fixture_warn($value, $format.as_ptr().cast())
    }; }
    pub(crate) use main_warn;
}
#[cfg(fixture_host)]
#[no_mangle]
pub unsafe extern "C" fn fixture_rust_private(command: *mut kernel::ffi::c_char,
    extra: *mut kernel::ffi::c_char, panic: *const kernel::ffi::c_char,
    parameter: *const kernel::ffi::c_char) {
    unsafe {
        main_globals::static_command_line = command;
        main_globals::extra_init_args = extra;
        main_globals::panic_later = panic;
        main_globals::panic_param = parameter;
    }
}
mod main_bootconfig {
    extern "C" { fn fixture_setup_boot_config(); }
    pub(super) unsafe fn setup_boot_config() { unsafe { fixture_setup_boot_config(); } }
}
mod main_command_line {
    extern "C" { fn fixture_setup_command_line(value: *mut kernel::ffi::c_char); }
    pub(super) unsafe fn setup_command_line(value: *mut kernel::ffi::c_char) {
        unsafe { fixture_setup_command_line(value); }
    }
}
mod main_print {
    extern "C" { fn fixture_print_kernel_cmdline(value: *const kernel::ffi::c_char); }
    pub(super) unsafe fn print_kernel_cmdline(value: *const kernel::ffi::c_char) {
        unsafe { fixture_print_kernel_cmdline(value); }
    }
}
mod main_early {
    extern "C" { fn fixture_parse_early_param(); }
    pub(super) unsafe fn parse_early_param() { unsafe { fixture_parse_early_param(); } }
}
mod main_unknown {
    extern "C" { fn fixture_print_unknown_bootoptions(); }
    pub(super) unsafe fn print_unknown_bootoptions() { unsafe { fixture_print_unknown_bootoptions(); } }
}
mod main_initcall_trace {
    extern "C" { fn fixture_initcall_debug_enable(); }
    pub(super) unsafe fn initcall_debug_enable() { unsafe { fixture_initcall_debug_enable(); } }
}
mod main_rest {
    extern "C" { fn fixture_rest_init() -> !; }
    pub(super) unsafe fn rest_init() -> ! { unsafe { fixture_rest_init() } }
}
mod main_bootoptions {
    use kernel::ffi::{c_char, c_int, c_void};
    extern "C" {
        fn fixture_unknown_bootoption(parameter: *mut c_char, value: *mut c_char,
            doing: *const c_char, argument: *mut c_void) -> c_int;
        fn fixture_set_init_arg(parameter: *mut c_char, value: *mut c_char,
            doing: *const c_char, argument: *mut c_void) -> c_int;
    }
    pub(super) unsafe extern "C" fn unknown_bootoption(parameter: *mut c_char, value: *mut c_char,
        doing: *const c_char, argument: *mut c_void) -> c_int {
        unsafe { fixture_unknown_bootoption(parameter, value, doing, argument) }
    }
    pub(super) unsafe extern "C" fn set_init_arg(parameter: *mut c_char, value: *mut c_char,
        doing: *const c_char, argument: *mut c_void) -> c_int {
        unsafe { fixture_set_init_arg(parameter, value, doing, argument) }
    }
}
