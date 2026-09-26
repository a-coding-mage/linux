/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of linux/init.h. C section, compiler, and preprocessor
// attributes are retained below as comments where Rust has no file-local
// equivalent.

use kernel::ffi::{c_char, c_int, c_uint};

pub type initcall_t = Option<unsafe extern "C" fn() -> c_int>;
pub type exitcall_t = Option<unsafe extern "C" fn()>;
pub type ctor_fn_t = Option<unsafe extern "C" fn()>;

// __init, __initdata, __initconst, __exitdata, __exit_call,
// __ref, __refdata, __refconst, __exit, and __meminit* apply ELF sections
// and compiler attributes (.init.text, .init.data, .init.rodata, .exit.*,
// .ref.*). They have no direct declaration-only Rust equivalent.
// __HEAD, __INIT, __FINIT, __INITDATA, __INITRODATA, __FINITDATA, __REF,
// __REFDATA, and __REFCONST are assembly-only directives.

#[cfg(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS)]
pub type initcall_entry_t = c_int;
#[cfg(not(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS))]
pub type initcall_entry_t = initcall_t;

/// Decodes an initcall entry using the architecture's linker representation.
///
/// # Safety
///
/// `entry` must point to a live, aligned, readable entry. Its decoded address
/// must be null or name an `unsafe extern "C" fn() -> c_int` that remains live
/// whenever called. Init sections cannot be used after they have been freed.
#[cfg(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS)]
#[inline]
pub unsafe fn initcall_from_entry(entry: *mut initcall_entry_t) -> initcall_t {
    // The signed displacement is relative to the entry itself, not the next
    // entry. Keep the native unsigned-long wrapping used by offset_to_ptr.
    // SAFETY: the caller supplies one readable entry and a valid decoded target.
    unsafe {
        let address = entry
            .expose_provenance()
            .wrapping_add_signed(entry.read() as isize);
        core::mem::transmute::<usize, initcall_t>(address)
    }
}

/// Reads a native function-pointer initcall entry.
///
/// # Safety
///
/// `entry` must be live, aligned and readable, and contain a valid nullable C
/// function pointer. Any target must remain live whenever called.
#[cfg(not(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS))]
#[inline]
pub unsafe fn initcall_from_entry(entry: *mut initcall_entry_t) -> initcall_t {
    // SAFETY: the caller supplies a readable, initialized function pointer.
    unsafe { entry.read() }
}

extern "C" {
    pub static mut __con_initcall_start: [initcall_entry_t; 0];
    pub static mut __con_initcall_end: [initcall_entry_t; 0];

    pub fn do_one_initcall(fn_: initcall_t) -> c_int;
    pub static mut boot_command_line: [c_char; 0];
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

    pub static mut __initcall_start: [initcall_entry_t; 0];
    pub static mut __initcall0_start: [initcall_entry_t; 0];
    pub static mut __initcall1_start: [initcall_entry_t; 0];
    pub static mut __initcall2_start: [initcall_entry_t; 0];
    pub static mut __initcall3_start: [initcall_entry_t; 0];
    pub static mut __initcall4_start: [initcall_entry_t; 0];
    pub static mut __initcall5_start: [initcall_entry_t; 0];
    pub static mut __initcall6_start: [initcall_entry_t; 0];
    pub static mut __initcall7_start: [initcall_entry_t; 0];
    pub static mut __initcall_end: [initcall_entry_t; 0];

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

#[cfg(MODULE)]
extern "C" {
    pub static mut __this_module: module;
}

#[cfg(MODULE)]
pub const THIS_MODULE: *mut module = core::ptr::addr_of_mut!(__this_module);
#[cfg(not(MODULE))]
pub const THIS_MODULE: *mut module = core::ptr::null_mut();

// The following __define_initcall/early_initcall/... macros place function
// pointers in linker sections (.initcall*, .con_initcall), optionally using
// PREL32 relocations, LTO stubs, unique __COUNTER__/__LINE__ symbol names,
// and compiler addressability attributes. Rust declarations cannot reproduce
// those preprocessor-generated symbols without the surrounding kernel build.

#[repr(C)]
#[cfg(not(MODULE))]
pub struct obs_kernel_param {
    pub str_: *const c_char,
    pub setup_func: Option<unsafe extern "C" fn(*mut c_char) -> c_int>,
    pub early: c_int,
}

#[cfg(not(MODULE))]
extern "C" {
    pub static __setup_start: [obs_kernel_param; 0];
    pub static __setup_end: [obs_kernel_param; 0];
}

// __setup_param, __setup, early_param, and early_param_on_off create aligned
// .init.setup records and parser functions. Their exact generated names and
// IS_ENABLED(config) expansion are build-time C behavior.

#[cfg(not(MODULE))]
extern "C" {
    pub fn parse_early_param();
    pub fn parse_early_options(cmdline: *mut c_char);
}

// __nosavedata selects .data..nosave; __exit_p(x) expands to x for modules
// and NULL for built-in code.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
