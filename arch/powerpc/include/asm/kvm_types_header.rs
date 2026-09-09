/* SPDX-License-Identifier: GPL-2.0 */

// Build-time configuration equivalent of the original KVM_SUB_MODULES macro:
//
// - when CONFIG_KVM_BOOK3S_64_PR and CONFIG_KVM_BOOK3S_64_HV are both modules,
//   KVM_SUB_MODULES is "kvm-pr,kvm-hv";
// - when only CONFIG_KVM_BOOK3S_64_PR is a module, it is "kvm-pr";
// - when only CONFIG_KVM_BOOK3S_64_HV is a module, it is "kvm-hv";
// - otherwise KVM_SUB_MODULES is undefined.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
