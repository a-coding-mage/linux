// SPDX-License-Identifier: GPL-2.0-only
// Empty linker ranges satisfy retained, unused initcall-table relocations.
macro_rules! boundary {
    ($($name:ident),*) => {$(
        #[no_mangle] static mut $name: [bindings::initcall_entry_t; 0] = [];
    )*};
}
boundary!(__initcall0_start, __initcall1_start, __initcall2_start, __initcall3_start,
          __initcall4_start, __initcall5_start, __initcall6_start, __initcall7_start,
          __initcall_end);

unsafe extern "C" {
    fn prepare_exec_case(which: ffi::c_int, argv: *mut *const ffi::c_char,
        envp: *mut *const ffi::c_char, execute: *mut *mut ffi::c_char,
        ramdisk: *mut *mut ffi::c_char);
    fn c_fixture(which: ffi::c_int) -> ffi::c_int;
    fn exercise_exec(which: ffi::c_int,
        callback: unsafe extern "C" fn(ffi::c_int) -> ffi::c_int) -> ffi::c_int;
}

unsafe extern "C" fn rust_fixture(which: ffi::c_int) -> ffi::c_int {
    unsafe {
        prepare_exec_case(which, core::ptr::addr_of_mut!(main_globals::argv_init).cast(),
            core::ptr::addr_of_mut!(main_globals::envp_init).cast(),
            core::ptr::addr_of_mut!(main_globals::execute_command),
            core::ptr::addr_of_mut!(main_globals::ramdisk_execute_command));
        let name = b"/test/init\0".as_ptr().cast();
        if which == 0 || (15..=18).contains(&which) {
            main_exec::run_init_process(name)
        } else if which < 4 || which >= 19 {
            main_exec::try_to_run_init_process(name)
        } else {
            main_exec::execute_init_processes()
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let which: ffi::c_int = args[2].parse().unwrap();
    let callback = if args[1] == "c" { c_fixture } else { rust_fixture };
    let result = unsafe { exercise_exec(which, callback) };
    std::process::exit(result);
}
