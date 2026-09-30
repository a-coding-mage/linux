// SPDX-License-Identifier: GPL-2.0-only
//! Original random-stack-offset boot option, per-CPU state and late initcall.

#![cfg(CONFIG_RANDOMIZE_KSTACK_OFFSET)]

use super::bindings;
use core::ptr;
use kernel::ffi::{c_char, c_int};

#[cfg(CONFIG_RANDOMIZE_KSTACK_OFFSET_DEFAULT)]
type OffsetKey = bindings::static_key_true;
#[cfg(not(CONFIG_RANDOMIZE_KSTACK_OFFSET_DEFAULT))]
type OffsetKey = bindings::static_key_false;

const fn initial_key() -> OffsetKey {
    // SAFETY: the canonical static-key initializer zeroes all fields except
    // its integer enabled count and, with jump labels, the integer type tag.
    let mut key: OffsetKey = unsafe { core::mem::zeroed() };
    key.key.enabled.counter = cfg!(CONFIG_RANDOMIZE_KSTACK_OFFSET_DEFAULT) as c_int;
    #[cfg(CONFIG_JUMP_LABEL)]
    {
        key.key.__bindgen_anon_1.type_ = if cfg!(CONFIG_RANDOMIZE_KSTACK_OFFSET_DEFAULT) {
            bindings::JUMP_TYPE_TRUE as _
        } else {
            bindings::JUMP_TYPE_FALSE as _
        };
    }
    key
}

#[no_mangle]
#[link_section = ".data..ro_after_init"]
static mut randomize_kstack_offset: OffsetKey = initial_key();

// This is the linker-relative per-CPU base passed to the existing PRNG seed
// service. Keep the canonical type, alignment and exported object name.
#[no_mangle]
#[cfg_attr(CONFIG_SMP, link_section = ".data..percpu")]
#[cfg_attr(not(CONFIG_SMP), link_section = ".data")]
static mut kstack_rnd_state: bindings::rnd_state = bindings::rnd_state {
    s1: 0,
    s2: 0,
    s3: 0,
    s4: 0,
};

#[link_section = ".init.text"]
#[cfg_attr(
    all(CONFIG_LTO_CLANG, CONFIG_HAVE_ARCH_PREL32_RELOCATIONS),
    export_name = "__initstub__kmod_main__0_810_random_kstack_init7"
)]
unsafe extern "C" fn random_kstack_init() -> c_int {
    // SAFETY: late init runs after the per-CPU allocator is available. The
    // seed service consumes this base, not a single CPU's resolved pointer.
    unsafe { bindings::prandom_seed_full_state(ptr::addr_of_mut!(kstack_rnd_state)) };
    0
}

#[link_section = ".init.text"]
unsafe extern "C" fn early_randomize_kstack_offset(buffer: *mut c_char) -> c_int {
    let mut enabled = false;
    // SAFETY: the parser supplies its nullable, NUL-terminated option value;
    // kstrtobool checks null and writes the live local boolean on success.
    let result = unsafe { bindings::kstrtobool(buffer, &mut enabled) };
    if result != 0 {
        return result;
    }
    let key = unsafe { ptr::addr_of_mut!(randomize_kstack_offset.key) };
    // SAFETY: early boot owns this canonical key until ro_after_init is
    // protected. The existing static-key subsystem preserves its checks and
    // atomic updates in the non-jump-label configuration as well.
    unsafe {
        #[cfg(CONFIG_JUMP_LABEL)]
        if enabled {
            bindings::static_key_enable(key);
        } else {
            bindings::static_key_disable(key);
        }
        #[cfg(not(CONFIG_JUMP_LABEL))]
        if enabled {
            bindings::rust_helper_static_key_enable(key);
        } else {
            bindings::rust_helper_static_key_disable(key);
        }
    }
    0
}

super::main_setup::setup_param!(
    "randomize_kstack_offset",
    RANDOMIZE_KSTACK_OFFSET_SETUP,
    Some(early_randomize_kstack_offset),
    1
);

// This is main.c's sole initcall. Retain its original function/line identity
// and a unique counter so generate_initcall_order.pl can order the LTO section.
// PREL32 entries must remain four-byte relative relocations, not Rust pointers.
#[cfg(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS)]
#[used]
#[link_section = ".discard.addressable"]
static ADDRESSABLE: unsafe extern "C" fn() -> c_int = random_kstack_init;
#[cfg(all(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS, not(CONFIG_LTO_CLANG)))]
core::arch::global_asm!(
    ".pushsection .initcall7.init,\"a\"",
    "__initcall__kmod_main__0_810_random_kstack_init7:",
    ".long {callback} - .",
    ".popsection",
    callback = sym random_kstack_init,
);
#[cfg(all(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS, CONFIG_LTO_CLANG))]
core::arch::global_asm!(
    ".pushsection .initcall7.init..kmod_main__0_810_random_kstack_init,\"a\"",
    "__initcall__kmod_main__0_810_random_kstack_init7:",
    ".long {callback} - .",
    ".popsection",
    callback = sym random_kstack_init,
);

#[cfg(not(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS))]
#[used]
#[linkage = "internal"]
#[export_name = "__initcall__kmod_main__0_810_random_kstack_init7"]
#[cfg_attr(not(CONFIG_LTO_CLANG), link_section = ".initcall7.init")]
#[cfg_attr(
    CONFIG_LTO_CLANG,
    link_section = ".initcall7.init..kmod_main__0_810_random_kstack_init"
)]
static mut INITCALL: unsafe extern "C" fn() -> c_int = random_kstack_init;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
