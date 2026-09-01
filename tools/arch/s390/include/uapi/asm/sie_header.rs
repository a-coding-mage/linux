// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

#![allow(non_upper_case_globals)]

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct sie_code {
    pub code: u32,
    pub name: &'static str,
}

pub const diagnose_codes: &[sie_code] = &[
    sie_code { code: 0x10, name: "DIAG (0x10) release pages" },
    sie_code { code: 0x44, name: "DIAG (0x44) time slice end" },
    sie_code { code: 0x9c, name: "DIAG (0x9c) time slice end directed" },
    sie_code { code: 0x204, name: "DIAG (0x204) logical-cpu utilization" },
    sie_code { code: 0x258, name: "DIAG (0x258) page-reference services" },
    sie_code { code: 0x288, name: "DIAG (0x288) watchdog functions" },
    sie_code { code: 0x308, name: "DIAG (0x308) ipl functions" },
    sie_code { code: 0x500, name: "DIAG (0x500) KVM virtio functions" },
    sie_code { code: 0x501, name: "DIAG (0x501) KVM breakpoint" },
];

pub const sigp_order_codes: &[sie_code] = &[
    sie_code { code: 0x01, name: "SIGP sense" },
    sie_code { code: 0x02, name: "SIGP external call" },
    sie_code { code: 0x03, name: "SIGP emergency signal" },
    sie_code { code: 0x04, name: "SIGP start" },
    sie_code { code: 0x05, name: "SIGP stop" },
    sie_code { code: 0x06, name: "SIGP restart" },
    sie_code { code: 0x09, name: "SIGP stop and store status" },
    sie_code { code: 0x0b, name: "SIGP initial cpu reset" },
    sie_code { code: 0x0c, name: "SIGP cpu reset" },
    sie_code { code: 0x0d, name: "SIGP set prefix" },
    sie_code { code: 0x0e, name: "SIGP store status at address" },
    sie_code { code: 0x12, name: "SIGP set architecture" },
    sie_code { code: 0x13, name: "SIGP conditional emergency signal" },
    sie_code { code: 0x15, name: "SIGP sense running" },
    sie_code { code: 0x16, name: "SIGP set multithreading" },
    sie_code { code: 0x17, name: "SIGP store additional status at address" },
];

pub const icpt_prog_codes: &[sie_code] = &[
    sie_code { code: 0x0001, name: "Prog Operation" },
    sie_code { code: 0x0002, name: "Prog Privileged Operation" },
    sie_code { code: 0x0003, name: "Prog Execute" },
    sie_code { code: 0x0004, name: "Prog Protection" },
    sie_code { code: 0x0005, name: "Prog Addressing" },
    sie_code { code: 0x0006, name: "Prog Specification" },
    sie_code { code: 0x0007, name: "Prog Data" },
    sie_code { code: 0x0008, name: "Prog Fixedpoint overflow" },
    sie_code { code: 0x0009, name: "Prog Fixedpoint divide" },
    sie_code { code: 0x000A, name: "Prog Decimal overflow" },
    sie_code { code: 0x000B, name: "Prog Decimal divide" },
    sie_code { code: 0x000C, name: "Prog HFP exponent overflow" },
    sie_code { code: 0x000D, name: "Prog HFP exponent underflow" },
    sie_code { code: 0x000E, name: "Prog HFP significance" },
    sie_code { code: 0x000F, name: "Prog HFP divide" },
    sie_code { code: 0x0010, name: "Prog Segment translation" },
    sie_code { code: 0x0011, name: "Prog Page translation" },
    sie_code { code: 0x0012, name: "Prog Translation specification" },
    sie_code { code: 0x0013, name: "Prog Special operation" },
    sie_code { code: 0x0015, name: "Prog Operand" },
    sie_code { code: 0x0016, name: "Prog Trace table" },
    sie_code { code: 0x0017, name: "Prog ASNtranslation specification" },
    sie_code { code: 0x001C, name: "Prog Spaceswitch event" },
    sie_code { code: 0x001D, name: "Prog HFP square root" },
    sie_code { code: 0x001F, name: "Prog PCtranslation specification" },
    sie_code { code: 0x0020, name: "Prog AFX translation" },
    sie_code { code: 0x0021, name: "Prog ASX translation" },
    sie_code { code: 0x0022, name: "Prog LX translation" },
    sie_code { code: 0x0023, name: "Prog EX translation" },
    sie_code { code: 0x0024, name: "Prog Primary authority" },
    sie_code { code: 0x0025, name: "Prog Secondary authority" },
    sie_code { code: 0x0026, name: "Prog LFXtranslation exception" },
    sie_code { code: 0x0027, name: "Prog LSXtranslation exception" },
    sie_code { code: 0x0028, name: "Prog ALET specification" },
    sie_code { code: 0x0029, name: "Prog ALEN translation" },
    sie_code { code: 0x002A, name: "Prog ALE sequence" },
    sie_code { code: 0x002B, name: "Prog ASTE validity" },
    sie_code { code: 0x002C, name: "Prog ASTE sequence" },
    sie_code { code: 0x002D, name: "Prog Extended authority" },
    sie_code { code: 0x002E, name: "Prog LSTE sequence" },
    sie_code { code: 0x002F, name: "Prog ASTE instance" },
    sie_code { code: 0x0030, name: "Prog Stack full" },
    sie_code { code: 0x0031, name: "Prog Stack empty" },
    sie_code { code: 0x0032, name: "Prog Stack specification" },
    sie_code { code: 0x0033, name: "Prog Stack type" },
    sie_code { code: 0x0034, name: "Prog Stack operation" },
    sie_code { code: 0x0039, name: "Prog Region first translation" },
    sie_code { code: 0x003A, name: "Prog Region second translation" },
    sie_code { code: 0x003B, name: "Prog Region third translation" },
    sie_code { code: 0x0040, name: "Prog Monitor event" },
    sie_code { code: 0x0080, name: "Prog PER event" },
    sie_code { code: 0x0119, name: "Prog Crypto operation" },
];

pub const fn exit_code_ipa0(ipa0: u32, opcode: u32) -> u32 {
    (ipa0 << 8) | opcode
}

pub const fn exit_code(opcode: u32) -> u32 {
    opcode
}

pub const icpt_insn_codes: &[sie_code] = &[
    sie_code { code: exit_code_ipa0(0x01, 0x01), name: "0x01 PR" },
    sie_code { code: exit_code_ipa0(0x01, 0x04), name: "0x01 PTFF" },
    sie_code { code: exit_code_ipa0(0x01, 0x07), name: "0x01 SCKPF" },
    sie_code { code: exit_code_ipa0(0xAA, 0x00), name: "0xAA RINEXT" },
    sie_code { code: exit_code_ipa0(0xAA, 0x01), name: "0xAA RION" },
    sie_code { code: exit_code_ipa0(0xAA, 0x02), name: "0xAA TRIC" },
    sie_code { code: exit_code_ipa0(0xAA, 0x03), name: "0xAA RIOFF" },
    sie_code { code: exit_code_ipa0(0xAA, 0x04), name: "0xAA RIEMIT" },
    sie_code { code: exit_code_ipa0(0xB2, 0x02), name: "0xB2 STIDP" },
    sie_code { code: exit_code_ipa0(0xB2, 0x04), name: "0xB2 SCK" },
    sie_code { code: exit_code_ipa0(0xB2, 0x05), name: "0xB2 STCK" },
    sie_code { code: exit_code_ipa0(0xB2, 0x06), name: "0xB2 SCKC" },
    sie_code { code: exit_code_ipa0(0xB2, 0x07), name: "0xB2 STCKC" },
    sie_code { code: exit_code_ipa0(0xB2, 0x08), name: "0xB2 SPT" },
    sie_code { code: exit_code_ipa0(0xB2, 0x09), name: "0xB2 STPT" },
    sie_code { code: exit_code_ipa0(0xB2, 0x0d), name: "0xB2 PTLB" },
    sie_code { code: exit_code_ipa0(0xB2, 0x10), name: "0xB2 SPX" },
    sie_code { code: exit_code_ipa0(0xB2, 0x11), name: "0xB2 STPX" },
    sie_code { code: exit_code_ipa0(0xB2, 0x12), name: "0xB2 STAP" },
    sie_code { code: exit_code_ipa0(0xB2, 0x14), name: "0xB2 SIE" },
    sie_code { code: exit_code_ipa0(0xB2, 0x16), name: "0xB2 SETR" },
    sie_code { code: exit_code_ipa0(0xB2, 0x17), name: "0xB2 STETR" },
    sie_code { code: exit_code_ipa0(0xB2, 0x18), name: "0xB2 PC" },
    sie_code { code: exit_code_ipa0(0xB2, 0x20), name: "0xB2 SERVC" },
    sie_code { code: exit_code_ipa0(0xB2, 0x21), name: "0xB2 IPTE" },
    sie_code { code: exit_code_ipa0(0xB2, 0x28), name: "0xB2 PT" },
    sie_code { code: exit_code_ipa0(0xB2, 0x29), name: "0xB2 ISKE" },
    sie_code { code: exit_code_ipa0(0xB2, 0x2a), name: "0xB2 RRBE" },
    sie_code { code: exit_code_ipa0(0xB2, 0x2b), name: "0xB2 SSKE" },
    sie_code { code: exit_code_ipa0(0xB2, 0x2c), name: "0xB2 TB" },
    sie_code { code: exit_code_ipa0(0xB2, 0x2e), name: "0xB2 PGIN" },
    sie_code { code: exit_code_ipa0(0xB2, 0x2f), name: "0xB2 PGOUT" },
    sie_code { code: exit_code_ipa0(0xB2, 0x30), name: "0xB2 CSCH" },
    sie_code { code: exit_code_ipa0(0xB2, 0x31), name: "0xB2 HSCH" },
    sie_code { code: exit_code_ipa0(0xB2, 0x32), name: "0xB2 MSCH" },
    sie_code { code: exit_code_ipa0(0xB2, 0x33), name: "0xB2 SSCH" },
    sie_code { code: exit_code_ipa0(0xB2, 0x34), name: "0xB2 STSCH" },
    sie_code { code: exit_code_ipa0(0xB2, 0x35), name: "0xB2 TSCH" },
    sie_code { code: exit_code_ipa0(0xB2, 0x36), name: "0xB2 TPI" },
    sie_code { code: exit_code_ipa0(0xB2, 0x37), name: "0xB2 SAL" },
    sie_code { code: exit_code_ipa0(0xB2, 0x38), name: "0xB2 RSCH" },
    sie_code { code: exit_code_ipa0(0xB2, 0x39), name: "0xB2 STCRW" },
    sie_code { code: exit_code_ipa0(0xB2, 0x3a), name: "0xB2 STCPS" },
    sie_code { code: exit_code_ipa0(0xB2, 0x3b), name: "0xB2 RCHP" },
    sie_code { code: exit_code_ipa0(0xB2, 0x3c), name: "0xB2 SCHM" },
    sie_code { code: exit_code_ipa0(0xB2, 0x40), name: "0xB2 BAKR" },
    sie_code { code: exit_code_ipa0(0xB2, 0x48), name: "0xB2 PALB" },
    sie_code { code: exit_code_ipa0(0xB2, 0x4c), name: "0xB2 TAR" },
    sie_code { code: exit_code_ipa0(0xB2, 0x50), name: "0xB2 CSP" },
    sie_code { code: exit_code_ipa0(0xB2, 0x54), name: "0xB2 MVPG" },
    sie_code { code: exit_code_ipa0(0xB2, 0x56), name: "0xB2 STHYI" },
    sie_code { code: exit_code_ipa0(0xB2, 0x58), name: "0xB2 BSG" },
    sie_code { code: exit_code_ipa0(0xB2, 0x5a), name: "0xB2 BSA" },
    sie_code { code: exit_code_ipa0(0xB2, 0x5f), name: "0xB2 CHSC" },
    sie_code { code: exit_code_ipa0(0xB2, 0x74), name: "0xB2 SIGA" },
    sie_code { code: exit_code_ipa0(0xB2, 0x76), name: "0xB2 XSCH" },
    sie_code { code: exit_code_ipa0(0xB2, 0x78), name: "0xB2 STCKE" },
    sie_code { code: exit_code_ipa0(0xB2, 0x7c), name: "0xB2 STCKF" },
    sie_code { code: exit_code_ipa0(0xB2, 0x7d), name: "0xB2 STSI" },
    sie_code { code: exit_code_ipa0(0xB2, 0xb0), name: "0xB2 STFLE" },
    sie_code { code: exit_code_ipa0(0xB2, 0xb1), name: "0xB2 STFL" },
    sie_code { code: exit_code_ipa0(0xB2, 0xb2), name: "0xB2 LPSWE" },
    sie_code { code: exit_code_ipa0(0xB2, 0xf8), name: "0xB2 TEND" },
    sie_code { code: exit_code_ipa0(0xB2, 0xfc), name: "0xB2 TABORT" },
    sie_code { code: exit_code_ipa0(0xB9, 0x1e), name: "0xB9 KMAC" },
    sie_code { code: exit_code_ipa0(0xB9, 0x28), name: "0xB9 PCKMO" },
    sie_code { code: exit_code_ipa0(0xB9, 0x2a), name: "0xB9 KMF" },
    sie_code { code: exit_code_ipa0(0xB9, 0x2b), name: "0xB9 KMO" },
    sie_code { code: exit_code_ipa0(0xB9, 0x2d), name: "0xB9 KMCTR" },
    sie_code { code: exit_code_ipa0(0xB9, 0x2e), name: "0xB9 KM" },
    sie_code { code: exit_code_ipa0(0xB9, 0x2f), name: "0xB9 KMC" },
    sie_code { code: exit_code_ipa0(0xB9, 0x3e), name: "0xB9 KIMD" },
    sie_code { code: exit_code_ipa0(0xB9, 0x3f), name: "0xB9 KLMD" },
    sie_code { code: exit_code_ipa0(0xB9, 0x8a), name: "0xB9 CSPG" },
    sie_code { code: exit_code_ipa0(0xB9, 0x8d), name: "0xB9 EPSW" },
    sie_code { code: exit_code_ipa0(0xB9, 0x8e), name: "0xB9 IDTE" },
    sie_code { code: exit_code_ipa0(0xB9, 0x8f), name: "0xB9 CRDTE" },
    sie_code { code: exit_code_ipa0(0xB9, 0x9c), name: "0xB9 EQBS" },
    sie_code { code: exit_code_ipa0(0xB9, 0xa2), name: "0xB9 PTF" },
    sie_code { code: exit_code_ipa0(0xB9, 0xab), name: "0xB9 ESSA" },
    sie_code { code: exit_code_ipa0(0xB9, 0xae), name: "0xB9 RRBM" },
    sie_code { code: exit_code_ipa0(0xB9, 0xaf), name: "0xB9 PFMF" },
    sie_code { code: exit_code_ipa0(0xE3, 0x03), name: "0xE3 LRAG" },
    sie_code { code: exit_code_ipa0(0xE3, 0x13), name: "0xE3 LRAY" },
    sie_code { code: exit_code_ipa0(0xE3, 0x25), name: "0xE3 NTSTG" },
    sie_code { code: exit_code_ipa0(0xE5, 0x00), name: "0xE5 LASP" },
    sie_code { code: exit_code_ipa0(0xE5, 0x01), name: "0xE5 TPROT" },
    sie_code { code: exit_code_ipa0(0xE5, 0x60), name: "0xE5 TBEGIN" },
    sie_code { code: exit_code_ipa0(0xE5, 0x61), name: "0xE5 TBEGINC" },
    sie_code { code: exit_code_ipa0(0xEB, 0x25), name: "0xEB STCTG" },
    sie_code { code: exit_code_ipa0(0xEB, 0x2f), name: "0xEB LCTLG" },
    sie_code { code: exit_code_ipa0(0xEB, 0x60), name: "0xEB LRIC" },
    sie_code { code: exit_code_ipa0(0xEB, 0x61), name: "0xEB STRIC" },
    sie_code { code: exit_code_ipa0(0xEB, 0x62), name: "0xEB MRIC" },
    sie_code { code: exit_code_ipa0(0xEB, 0x8a), name: "0xEB SQBS" },
    sie_code { code: exit_code_ipa0(0xC8, 0x01), name: "0xC8 ECTG" },
    sie_code { code: exit_code(0x0a), name: "SVC" },
    sie_code { code: exit_code(0x80), name: "SSM" },
    sie_code { code: exit_code(0x82), name: "LPSW" },
    sie_code { code: exit_code(0x83), name: "DIAG" },
    sie_code { code: exit_code(0xae), name: "SIGP" },
    sie_code { code: exit_code(0xac), name: "STNSM" },
    sie_code { code: exit_code(0xad), name: "STOSM" },
    sie_code { code: exit_code(0xb1), name: "LRA" },
    sie_code { code: exit_code(0xb6), name: "STCTL" },
    sie_code { code: exit_code(0xb7), name: "LCTL" },
    sie_code { code: exit_code(0xee), name: "PLO" },
];

pub const sie_intercept_code: &[sie_code] = &[
    sie_code { code: 0x00, name: "Host interruption" },
    sie_code { code: 0x04, name: "Instruction" },
    sie_code { code: 0x08, name: "Program interruption" },
    sie_code { code: 0x0c, name: "Instruction and program interruption" },
    sie_code { code: 0x10, name: "External request" },
    sie_code { code: 0x14, name: "External interruption" },
    sie_code { code: 0x18, name: "I/O request" },
    sie_code { code: 0x1c, name: "Wait state" },
    sie_code { code: 0x20, name: "Validity" },
    sie_code { code: 0x28, name: "Stop request" },
    sie_code { code: 0x2c, name: "Operation exception" },
    sie_code { code: 0x38, name: "Partial-execution" },
    sie_code { code: 0x3c, name: "I/O interruption" },
    sie_code { code: 0x40, name: "I/O instruction" },
    sie_code { code: 0x48, name: "Timing subset" },
];

/*
 * This is the simple interceptable instructions decoder.
 *
 * It will be used as userspace interface and it can be used in places
 * that does not allow to use general decoder functions,
 * such as trace events declarations.
 *
 * Some userspace tools may want to parse this code
 * and would be confused by switch(), if() and other statements,
 * but they can understand conditional operator.
 */
pub const fn INSN_DECODE_IPA0(ipa0: u64, insn: u64, rshift: u32, mask: u64) -> Option<u32> {
    if (insn >> 56) == ipa0 {
        Some(((ipa0 << 8) | ((insn >> rshift) & mask)) as u32)
    } else {
        None
    }
}

pub const fn INSN_DECODE(insn: u64) -> u32 {
    (insn >> 56) as u32
}

/*
 * The macro icpt_insn_decoder() takes an intercepted instruction
 * and returns a key, which can be used to find a mnemonic name
 * of the instruction in the icpt_insn_codes table.
 */
pub const fn icpt_insn_decoder(insn: u64) -> u32 {
    if let Some(code) = INSN_DECODE_IPA0(0x01, insn, 48, 0xff) {
        code
    } else if let Some(code) = INSN_DECODE_IPA0(0xaa, insn, 48, 0x0f) {
        code
    } else if let Some(code) = INSN_DECODE_IPA0(0xb2, insn, 48, 0xff) {
        code
    } else if let Some(code) = INSN_DECODE_IPA0(0xb9, insn, 48, 0xff) {
        code
    } else if let Some(code) = INSN_DECODE_IPA0(0xe3, insn, 48, 0xff) {
        code
    } else if let Some(code) = INSN_DECODE_IPA0(0xe5, insn, 48, 0xff) {
        code
    } else if let Some(code) = INSN_DECODE_IPA0(0xeb, insn, 16, 0xff) {
        code
    } else if let Some(code) = INSN_DECODE_IPA0(0xc8, insn, 48, 0x0f) {
        code
    } else {
        INSN_DECODE(insn)
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
