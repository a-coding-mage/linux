// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2025 Ventana Micro Systems Inc.
 */

// C dependencies supplied by the surrounding kernel translation.
use crate::asm::kvm_vcpu_sbi::kvm_vcpu_sbi_extension;
use crate::asm::sbi::{SBI_EXT_DBCN, SBI_EXT_EXPERIMENTAL_END,
    SBI_EXT_EXPERIMENTAL_START, SBI_EXT_MPXY, SBI_EXT_VENDOR_END,
    SBI_EXT_VENDOR_START};

extern "C" {
    pub fn kvm_riscv_vcpu_sbi_forward_handler();
}

pub static vcpu_sbi_ext_experimental: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: SBI_EXT_EXPERIMENTAL_START,
    extid_end: SBI_EXT_EXPERIMENTAL_END,
    handler: kvm_riscv_vcpu_sbi_forward_handler,
};

pub static vcpu_sbi_ext_vendor: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: SBI_EXT_VENDOR_START,
    extid_end: SBI_EXT_VENDOR_END,
    handler: kvm_riscv_vcpu_sbi_forward_handler,
};

pub static vcpu_sbi_ext_dbcn: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: SBI_EXT_DBCN,
    extid_end: SBI_EXT_DBCN,
    default_disabled: true,
    handler: kvm_riscv_vcpu_sbi_forward_handler,
};

pub static vcpu_sbi_ext_mpxy: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: SBI_EXT_MPXY,
    extid_end: SBI_EXT_MPXY,
    default_disabled: true,
    handler: kvm_riscv_vcpu_sbi_forward_handler,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
