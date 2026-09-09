/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of linux/init.h. C section, compiler, and preprocessor
// attributes are retained below as comments where Rust has no file-local
// equivalent.

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type initcall_t = unsafe extern "C" fn() -> c_int;
pub type exitcall_t = unsafe extern "C" fn();
pub type ctor_fn_t = unsafe extern "C" fn();

// __init, __initdata, __initconst, __exitdata, __exit_call,
// __ref, __refdata, __refconst, __exit, and __meminit* apply ELF sections
// and compiler attributes (.init.text, .init.data, .init.rodata, .exit.*,
// .ref.*). They have no direct declaration-only Rust equivalent.
// __HEAD, __INIT, __FINIT, __INITDATA, __INITRODATA, __FINITDATA, __REF,
// __REFDATA, and __REFCONST are assembly-only directives.

#[cfg(feature = "config-have-arch-prel32-relocations")]
pub type initcall_entry_t = c_int;
#[cfg(not(feature = "config-have-arch-prel32-relocations"))]
pub type initcall_entry_t = initcall_t;

#[cfg(feature = "config-have-arch-prel32-relocations")]
pub unsafe fn initcall_from_entry(entry: *mut initcall_entry_t) -> initcall_t {
    // C: offset_to_ptr(entry); the relocation helper is supplied externally.
    *(entry as *mut initcall_t)
}

#[cfg(not(feature = "config-have-arch-prel32-relocations"))]
pub unsafe fn initcall_from_entry(entry: *mut initcall_entry_t) -> initcall_t {
    *entry
}

extern "C" {
    pub static mut __con_initcall_start: initcall_entry_t;
    pub static mut __con_initcall_end: initcall_entry_t;

    pub fn do_one_initcall(fn_: initcall_t) -> c_int;
    pub static mut boot_command_line: c_char;
    pub static mut saved_command_line: *mut c_char;
    pub static mut saved_command_line_len: c_uint;
    pub static mut reset_devices: c_uint;

    pub fn setup_arch(command_line: *mut *mut c_char);
    pub fn prepare_namespace();
    pub fn init_rootfs();
    pub fn init_IRQ();
    pub fn time_init();
    pub fn poking_init();
    pub fn pgtable_cache_init();

    pub static mut __initcall_start: initcall_entry_t;
    pub static mut __initcall0_start: initcall_entry_t;
    pub static mut __initcall1_start: initcall_entry_t;
    pub static mut __initcall2_start: initcall_entry_t;
    pub static mut __initcall3_start: initcall_entry_t;
    pub static mut __initcall4_start: initcall_entry_t;
    pub static mut __initcall5_start: initcall_entry_t;
    pub static mut __initcall6_start: initcall_entry_t;
    pub static mut __initcall7_start: initcall_entry_t;
    pub static mut __initcall_end: initcall_entry_t;

    pub static mut rootfs_fs_type: file_system_type;
    pub static mut rodata_enabled: bool;
    pub fn mark_rodata_ro();
    pub static mut late_time_init: Option<unsafe extern "C" fn()>;
    pub static mut initcall_debug: bool;
}

#[repr(C)]
pub struct file_system_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[cfg(feature = "module")]
extern "C" {
    pub static mut __this_module: module;
}

#[cfg(feature = "module")]
pub const THIS_MODULE: *mut module = unsafe { &raw mut __this_module };
#[cfg(not(feature = "module"))]
pub const THIS_MODULE: *mut module = core::ptr::null_mut();

// The following __define_initcall/early_initcall/... macros place function
// pointers in linker sections (.initcall*, .con_initcall), optionally using
// PREL32 relocations, LTO stubs, unique __COUNTER__/__LINE__ symbol names,
// and compiler addressability attributes. Rust declarations cannot reproduce
// those preprocessor-generated symbols without the surrounding kernel build.

#[repr(C)]
pub struct obs_kernel_param {
    pub str_: *const c_char,
    pub setup_func: Option<unsafe extern "C" fn(*mut c_char) -> c_int>,
    pub early: c_int,
}

extern "C" {
    pub static __setup_start: obs_kernel_param;
    pub static __setup_end: obs_kernel_param;
}

// __setup_param, __setup, early_param, and early_param_on_off create aligned
// .init.setup records and parser functions. Their exact generated names and
// IS_ENABLED(config) expansion are build-time C behavior.

pub unsafe extern "C" fn parse_early_param();
pub unsafe extern "C" fn parse_early_options(cmdline: *mut c_char);

// __nosavedata selects .data..nosave; __exit_p(x) expands to x for modules
// and NULL for built-in code.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
