/* SPDX-License-Identifier: GPL-2.0 */

// The original header requires KVM_X86_PMU_OP, KVM_X86_PMU_OP_OPTIONAL, and
// KVM_X86_PMU_OP_OPTIONAL_RET0 to be defined by the including translation
// unit.  They are represented here as externally supplied Rust macros.

/*
 * KVM_X86_PMU_OP!() and KVM_X86_PMU_OP_OPTIONAL!() are used to help generate
 * both declarations/definitions of static calls and static-call update calls.
 *
 * KVM_X86_PMU_OP_OPTIONAL!() can be used for functions that can have a null
 * definition.
 */
KVM_X86_PMU_OP!(rdpmc_ecx_to_pmc);
KVM_X86_PMU_OP!(msr_idx_to_pmc);
KVM_X86_PMU_OP_OPTIONAL!(check_rdpmc_early);
KVM_X86_PMU_OP!(is_valid_msr);
KVM_X86_PMU_OP!(get_msr);
KVM_X86_PMU_OP!(set_msr);
KVM_X86_PMU_OP!(refresh);
KVM_X86_PMU_OP!(init);
KVM_X86_PMU_OP_OPTIONAL!(reset);
KVM_X86_PMU_OP_OPTIONAL!(deliver_pmi);
KVM_X86_PMU_OP_OPTIONAL!(cleanup);
KVM_X86_PMU_OP_OPTIONAL_RET0!(pmc_is_disabled_in_current_mode);

KVM_X86_PMU_OP_OPTIONAL!(write_global_ctrl);
KVM_X86_PMU_OP!(mediated_load);
KVM_X86_PMU_OP!(mediated_put);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
