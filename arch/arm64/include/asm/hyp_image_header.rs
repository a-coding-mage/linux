/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020 Google LLC.
 * Written by David Brazdil <dbrazdil@google.com>
 */

//! Rust translation of `asm/hyp_image.h`.

/*
 * The C token-pasting helpers are represented as string-producing macros here;
 * linker/build users requiring symbol token construction should apply the same
 * concatenation at the consuming boundary.
 */
#[macro_export]
macro_rules! __hyp_concat {
    ($a:ident, $b:ident) => {
        concat!(stringify!($a), stringify!($b))
    };
}

#[macro_export]
macro_rules! hyp_concat {
    ($a:ident, $b:ident) => {
        $crate::__hyp_concat!($a, $b)
    };
}

/*
 * KVM nVHE code has its own symbol namespace prefixed with __kvm_nvhe_,
 * to separate it from the kernel proper. In Rust, retain the selected symbol
 * spelling as a string for consumers which cannot use C token pasting.
 */
#[cfg(not(kvm_nvhe_hypervisor))]
#[macro_export]
macro_rules! kvm_nvhe_sym {
    ($sym:ident) => {
        concat!("__kvm_nvhe_", stringify!($sym))
    };
}

#[cfg(kvm_nvhe_hypervisor)]
#[macro_export]
macro_rules! kvm_nvhe_sym {
    ($sym:ident) => {
        stringify!($sym)
    };
}

/*
 * Under LINKER_SCRIPT the original header emits GNU linker-script syntax.
 * That syntax has no executable Rust equivalent; the complete source-level
 * intent is preserved below for the linker-script generation boundary.
 *
 * HYP_SECTION_NAME(NAME): .hyp##NAME
 * HYP_SECTION_SYMBOL_NAME(NAME): __hyp_section_##HYP_SECTION_NAME(NAME)
 * BEGIN_HYP_SECTION(NAME):
 *     HYP_SECTION_NAME(NAME) : {
 *         HYP_SECTION_SYMBOL_NAME(NAME) = .;
 * END_HYP_SECTION: }
 * HYP_SECTION(NAME):
 *     BEGIN_HYP_SECTION(NAME) *(NAME NAME##.*) END_HYP_SECTION
 * KVM_NVHE_ALIAS(sym): kvm_nvhe_sym(sym) = sym;
 * KVM_NVHE_ALIAS_HYP(first, sec):
 *     kvm_nvhe_sym(first) = kvm_nvhe_sym(sec);
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
