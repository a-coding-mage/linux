/* SPDX-License-Identifier: GPL-2.0 */
/*
 * AMD SEV header common between the guest and the hypervisor.
 *
 * Author: Brijesh Singh <brijesh.singh@amd.com>
 */

pub const GHCB_MSR_INFO_POS: u64 = 0;
pub const GHCB_DATA_LOW: u64 = 12;
pub const GHCB_MSR_INFO_MASK: u64 = (1u64 << GHCB_DATA_LOW) - 1;

pub const fn ghcb_data(v: u64) -> u64 { (v & !GHCB_MSR_INFO_MASK) >> GHCB_DATA_LOW }

/* SEV Information Request/Response */
pub const GHCB_MSR_SEV_INFO_RESP: u64 = 0x001;
pub const GHCB_MSR_SEV_INFO_REQ: u64 = 0x002;
pub const fn ghcb_msr_sev_info(max: u64, min: u64, cbit: u64) -> u64 {
    ((max & 0xffff) << 48) | ((min & 0xffff) << 32) |
        ((cbit & 0xff) << 24) | GHCB_MSR_SEV_INFO_RESP
}
pub const fn ghcb_msr_info(v: u64) -> u64 { v & 0xfff }
pub const fn ghcb_msr_proto_max(v: u64) -> u64 { (v >> 48) & 0xffff }
pub const fn ghcb_msr_proto_min(v: u64) -> u64 { (v >> 32) & 0xffff }

/* CPUID Request/Response */
pub const GHCB_MSR_CPUID_REQ: u64 = 0x004;
pub const GHCB_MSR_CPUID_RESP: u64 = 0x005;
pub const GHCB_MSR_CPUID_FUNC_POS: u64 = 32;
pub const GHCB_MSR_CPUID_FUNC_MASK: u64 = 0xffffffff;
pub const GHCB_MSR_CPUID_VALUE_POS: u64 = 32;
pub const GHCB_MSR_CPUID_VALUE_MASK: u64 = 0xffffffff;
pub const GHCB_MSR_CPUID_REG_POS: u64 = 30;
pub const GHCB_MSR_CPUID_REG_MASK: u64 = 0x3;
pub const GHCB_CPUID_REQ_EAX: u64 = 0;
pub const GHCB_CPUID_REQ_EBX: u64 = 1;
pub const GHCB_CPUID_REQ_ECX: u64 = 2;
pub const GHCB_CPUID_REQ_EDX: u64 = 3;
pub const fn ghcb_cpuid_req(fn_: u64, reg: u64) -> u64 {
    GHCB_MSR_CPUID_REQ | ((reg & 0x3) << 30) | (fn_ << 32)
}

/* AP Reset Hold */
pub const GHCB_MSR_AP_RESET_HOLD_REQ: u64 = 0x006;
pub const GHCB_MSR_AP_RESET_HOLD_RESP: u64 = 0x007;
pub const GHCB_MSR_AP_RESET_HOLD_RESULT_POS: u64 = 12;
pub const GHCB_MSR_AP_RESET_HOLD_RESULT_MASK: u64 = (1u64 << 52) - 1;

/* Preferred GHCB GPA Request */
pub const GHCB_MSR_PREF_GPA_REQ: u64 = 0x010;
pub const GHCB_MSR_GPA_VALUE_POS: u64 = 12;
pub const GHCB_MSR_GPA_VALUE_MASK: u64 = (1u64 << 52) - 1;
pub const GHCB_MSR_PREF_GPA_RESP: u64 = 0x011;
pub const GHCB_MSR_PREF_GPA_NONE: u64 = 0xfffffffffffff;

/* GHCB GPA Register */
pub const GHCB_MSR_REG_GPA_REQ: u64 = 0x012;
pub const fn ghcb_msr_reg_gpa_req_val(v: u64) -> u64 {
    ((v & ((1u64 << 52) - 1)) << 12) | GHCB_MSR_REG_GPA_REQ
}
pub const GHCB_MSR_REG_GPA_RESP: u64 = 0x013;
pub const fn ghcb_msr_reg_gpa_resp_val(v: u64) -> u64 { (v & (!0u64 << 12)) >> 12 }

#[repr(u32)]
pub enum PscOp { SnpPageStatePrivate = 1, SnpPageStateShared }

pub const GHCB_MSR_PSC_REQ: u64 = 0x014;
pub const fn ghcb_msr_psc_req_gfn(gfn: u64, op: u64) -> u64 {
    ((op & 0xf) << 52) | ((gfn & ((1u64 << 40) - 1)) << 12) | GHCB_MSR_PSC_REQ
}
pub const fn ghcb_msr_psc_req_to_gfn(msr: u64) -> u64 { (msr & (!0u64 >> 12 << 12)) >> 12 }
pub const fn ghcb_msr_psc_req_to_op(msr: u64) -> u64 { (msr & (0xfu64 << 52)) >> 52 }
pub const GHCB_MSR_PSC_RESP: u64 = 0x015;
pub const fn ghcb_msr_psc_resp_val(val: u64) -> u64 { (val & (!0u64 << 32)) >> 32 }
pub const GHCB_MSR_PSC_RESP_ERROR: u64 = (1u64 << 63) | GHCB_MSR_PSC_RESP;

/* GHCB Run at VMPL Request/Response */
pub const GHCB_MSR_VMPL_REQ: u64 = 0x016;
pub const fn ghcb_msr_vmpl_req_level(v: u64) -> u64 { ((v & 0xff) << 32) | GHCB_MSR_VMPL_REQ }
pub const GHCB_MSR_VMPL_RESP: u64 = 0x017;
pub const fn ghcb_msr_vmpl_resp_val(v: u64) -> u64 { (v & (!0u64 << 32)) >> 32 }

/* GHCB Hypervisor Feature Request/Response */
pub const GHCB_MSR_HV_FT_REQ: u64 = 0x080;
pub const GHCB_MSR_HV_FT_RESP: u64 = 0x081;
pub const GHCB_MSR_HV_FT_POS: u64 = 12;
pub const GHCB_MSR_HV_FT_MASK: u64 = (1u64 << 52) - 1;
pub const fn ghcb_msr_hv_ft_resp_val(v: u64) -> u64 { (v & (!0u64 << 12)) >> 12 }
pub const GHCB_HV_FT_SNP: u64 = 1u64 << 0;
pub const GHCB_HV_FT_SNP_AP_CREATION: u64 = 1u64 << 1;
pub const GHCB_HV_FT_SNP_MULTI_VMPL: u64 = 1u64 << 5;

/* SNP Page State Change NAE event */
pub const VMGEXIT_PSC_MAX_ENTRY: usize = 64;
pub const VMGEXIT_PSC_MAX_COUNT: usize = 253;
pub const VMGEXIT_PSC_ERROR_GENERIC: u64 = 0x100u64 << 32;
pub const VMGEXIT_PSC_ERROR_INVALID_HDR: u64 = (1u64 << 32) | 1;
pub const VMGEXIT_PSC_ERROR_INVALID_ENTRY: u64 = (1u64 << 32) | 2;
pub const VMGEXIT_PSC_OP_PRIVATE: u64 = 1;
pub const VMGEXIT_PSC_OP_SHARED: u64 = 2;

#[repr(C, packed)]
pub struct PscHdr { pub cur_entry: u16, pub end_entry: u16, pub reserved: u32 }

/* C bit-field layout: cur_page[11:0], gfn[51:12], operation[55:52], pagesize[56], reserved[63:57]. */
#[repr(transparent)]
pub struct PscEntry(pub u64);

#[repr(C, packed)]
pub struct SnpPscDesc {
    pub hdr: PscHdr,
    pub entries: [PscEntry; VMGEXIT_PSC_MAX_ENTRY],
}

pub const GHCB_MSR_TERM_REQ: u64 = 0x100;
pub const GHCB_MSR_TERM_REASON_SET_POS: u64 = 12;
pub const GHCB_MSR_TERM_REASON_SET_MASK: u64 = 0xf;
pub const GHCB_MSR_TERM_REASON_POS: u64 = 16;
pub const GHCB_MSR_TERM_REASON_MASK: u64 = 0xff;
pub const fn ghcb_sev_term_reason(reason_set: u64, reason_val: u64) -> u64 {
    ((reason_set & 0xf) << 12) | ((reason_val & 0xff) << 16)
}

/* Error codes from reason set 0 */
pub const SEV_TERM_SET_GEN: u64 = 0;
pub const GHCB_SEV_ES_GEN_REQ: u64 = 0;
pub const GHCB_SEV_ES_PROT_UNSUPPORTED: u64 = 1;
pub const GHCB_SNP_UNSUPPORTED: u64 = 2;
/* Linux-specific reason codes (used with reason set 1) */
pub const SEV_TERM_SET_LINUX: u64 = 1;
pub const GHCB_TERM_REGISTER: u64 = 0;
pub const GHCB_TERM_PSC: u64 = 1;
pub const GHCB_TERM_PVALIDATE: u64 = 2;
pub const GHCB_TERM_NOT_VMPL0: u64 = 3;
pub const GHCB_TERM_CPUID: u64 = 4;
pub const GHCB_TERM_CPUID_HV: u64 = 5;
pub const GHCB_TERM_SECRETS_PAGE: u64 = 6;
pub const GHCB_TERM_NO_SVSM: u64 = 7;
pub const GHCB_TERM_SVSM_VMPL0: u64 = 8;
pub const GHCB_TERM_SVSM_CAA: u64 = 9;
pub const GHCB_TERM_SECURE_TSC: u64 = 10;
pub const GHCB_TERM_SVSM_CA_REMAP_FAIL: u64 = 11;
pub const GHCB_TERM_SAVIC_FAIL: u64 = 12;
pub const fn ghcb_resp_code(v: u64) -> u64 { v & GHCB_MSR_INFO_MASK }

pub const GHCB_HV_RESP_NO_ACTION: u64 = 0;
pub const GHCB_HV_RESP_ISSUE_EXCEPTION: u64 = 1;
pub const GHCB_HV_RESP_MALFORMED_INPUT: u64 = 2;
pub const GHCB_ERR_NOT_REGISTERED: u64 = 1;
pub const GHCB_ERR_INVALID_USAGE: u64 = 2;
pub const GHCB_ERR_INVALID_SCRATCH_AREA: u64 = 3;
pub const GHCB_ERR_MISSING_INPUT: u64 = 4;
pub const GHCB_ERR_INVALID_INPUT: u64 = 5;
pub const GHCB_ERR_INVALID_EVENT: u64 = 6;

#[repr(C)]
pub struct SevConfig { pub bits: u64 }

extern "C" {
    pub static mut sev_cfg: SevConfig;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
