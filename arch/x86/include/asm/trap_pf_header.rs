/* SPDX-License-Identifier: GPL-2.0 */

// Page fault error code bits:
//
//   bit 0 ==     0: no page found    1: protection fault
//   bit 1 ==     0: read access      1: write access
//   bit 2 ==     0: kernel-mode access       1: user-mode access
//   bit 3 ==                             1: use of reserved bit detected
//   bit 4 ==                             1: fault was an instruction fetch
//   bit 5 ==                             1: protection keys block access
//   bit 6 ==                             1: shadow stack access fault
//   bit 15 ==                            1: SGX MMU page-fault
//   bit 31 ==                            1: fault was due to RMP violation
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum x86_pf_error_code {
    X86_PF_PROT = 1u32 << 0,
    X86_PF_WRITE = 1u32 << 1,
    X86_PF_USER = 1u32 << 2,
    X86_PF_RSVD = 1u32 << 3,
    X86_PF_INSTR = 1u32 << 4,
    X86_PF_PK = 1u32 << 5,
    X86_PF_SHSTK = 1u32 << 6,
    X86_PF_SGX = 1u32 << 15,
    X86_PF_RMP = 1u32 << 31,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
