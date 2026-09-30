// SPDX-License-Identifier: GPL-2.0-only
// Common observer boundaries for already separate boot components.
mod main_initcall_levels {
    extern "C" {
        fn fixture_pre_smp();
        fn fixture_basic_setup();
    }
    pub(super) unsafe fn do_pre_smp_initcalls() { unsafe { fixture_pre_smp(); } }
    pub(super) unsafe fn do_basic_setup() { unsafe { fixture_basic_setup(); } }
}
mod main_console {
    extern "C" { fn fixture_console(); }
    pub(super) unsafe fn console_on_rootfs() { unsafe { fixture_console(); } }
}

#[no_mangle]
pub unsafe extern "C" fn rust_freeable(value: *mut kernel::ffi::c_char, explicit: bool) -> *mut kernel::ffi::c_char {
    unsafe {
        main_globals::ramdisk_execute_command = value;
        main_globals::ramdisk_execute_command_set = explicit;
        main_freeable::kernel_init_freeable();
        main_globals::ramdisk_execute_command
    }
}
