// SPDX-License-Identifier: GPL-2.0-only
//! Original weak boot defaults, superseded by architecture strong definitions.

use super::bindings;
use kernel::ffi::c_int;

macro_rules! empty_init_default {
    ($($name:ident),* $(,)?) => {$(
        // These are the original empty architecture defaults, not replacement
        // boot services. Real strong definitions retain normal linker priority.
        #[no_mangle]
        #[linkage = "weak"]
        #[link_section = ".init.text"]
        pub(super) unsafe extern "C" fn $name() {}
    )*};
}

empty_init_default!(
    arch_post_acpi_subsys_init,
    smp_setup_processor_id,
    smp_prepare_boot_cpu,
    poking_init,
    pgtable_cache_init,
    trap_init,
);

// Rust cfg attributes cannot inspect canonical generated integer constants.
// Let the assembler apply the original THREAD_SIZE >= PAGE_SIZE condition to
// a weak alias of a real, protected C-ABI Rust function. This emits no default
// public symbol on architectures whose stacks are smaller than a page.
#[export_name = "__rust_main_thread_stack_cache_init"]
#[linkage = "internal"]
#[link_section = ".init.text"]
unsafe extern "C" fn thread_stack_cache_default() {}

core::arch::global_asm!(
    ".if {thread_size} >= {page_size}",
    ".weak thread_stack_cache_init",
    ".set thread_stack_cache_init, {implementation}",
    ".endif",
    thread_size = const bindings::RUST_INIT_MAIN_THREAD_SIZE,
    page_size = const bindings::RUST_INIT_MAIN_PAGE_SIZE,
    implementation = sym thread_stack_cache_default,
);

/// Release the original init-memory range with its original poison/name.
///
/// # Safety
///
/// The caller has completed all init users and the original memory/lifetime
/// transitions. Architecture overrides may impose additional requirements.
#[no_mangle]
#[linkage = "weak"]
pub(super) unsafe extern "C" fn free_initmem() {
    // SAFETY: the caller establishes the same release phase as C's weak
    // free_initmem(). This is the exact inline free_initmem_default operation.
    unsafe {
        bindings::free_reserved_area(
            core::ptr::addr_of_mut!(bindings::__init_begin).cast(),
            core::ptr::addr_of_mut!(bindings::__init_end).cast(),
            bindings::POISON_FREE_INITMEM as c_int,
            c"unused kernel image (initmem)".as_ptr().cast(),
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
