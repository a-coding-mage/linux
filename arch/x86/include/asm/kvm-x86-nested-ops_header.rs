/* SPDX-License-Identifier: GPL-2.0 */

// This header is intended to be included with the KVM_X86_NESTED_OP,
// KVM_X86_NESTED_OP_OPTIONAL, and KVM_X86_NESTED_OP_OPTIONAL_RET0 macros
// defined by the including translation unit.
//
// The original C preprocessor guard emits an error when one or more of these
// macros are missing. Rust macro availability is determined at expansion time.

/*
 * KVM_X86_NESTED_OP() and KVM_X86_NESTED_OP_OPTIONAL() are used to help
 * generate both DECLARE/DEFINE_STATIC_CALL() invocations and
 * "static_call_update()" calls.
 *
 * KVM_X86_NESTED_OP_OPTIONAL() can be used for those functions that can have
 * a NULL definition. KVM_X86_NESTED_OP_OPTIONAL_RET0() can be used likewise
 * to make a definition optional, but in this case the default will be
 * __static_call_return0.
 */
KVM_X86_NESTED_OP!(leave_nested);
KVM_X86_NESTED_OP!(is_exception_vmexit);
KVM_X86_NESTED_OP!(check_events);
KVM_X86_NESTED_OP_OPTIONAL_RET0!(has_events);
KVM_X86_NESTED_OP!(triple_fault);
KVM_X86_NESTED_OP!(get_state);
KVM_X86_NESTED_OP!(set_state);
KVM_X86_NESTED_OP!(get_nested_state_pages);
KVM_X86_NESTED_OP_OPTIONAL_RET0!(write_log_dirty);
KVM_X86_NESTED_OP!(translate_nested_gpa);

// Corresponds to the C CONFIG_KVM_HYPERV build-time condition.
#[cfg(feature = "CONFIG_KVM_HYPERV")]
mod config_kvm_hyperv {
    KVM_X86_NESTED_OP_OPTIONAL!(enable_evmcs);
    KVM_X86_NESTED_OP_OPTIONAL_RET0!(get_evmcs_version);
    KVM_X86_NESTED_OP!(hv_inject_synthetic_vmexit_post_tlb_flush);
}

// The C source undefines the temporary invocation macros after inclusion.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
