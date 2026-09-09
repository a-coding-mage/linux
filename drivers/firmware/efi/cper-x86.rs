// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018, Advanced Micro Devices, Inc.

// Translated from cper-x86.c. The kernel types and functions referenced here
// are supplied by the surrounding implementation.

const VALID_LAPIC_ID: u64 = 1 << 0;
const VALID_CPUID_INFO: u64 = 1 << 1;
const fn valid_proc_err_info_num(bits: u64) -> u64 { (bits & 0xfc) >> 2 }
const fn valid_proc_cxt_info_num(bits: u64) -> u64 { (bits & 0x3f00) >> 8 }

const INFO_VALID_CHECK_INFO: u64 = 1 << 0;
const INFO_VALID_TARGET_ID: u64 = 1 << 1;
const INFO_VALID_REQUESTOR_ID: u64 = 1 << 2;
const INFO_VALID_RESPONDER_ID: u64 = 1 << 3;
const INFO_VALID_IP: u64 = 1 << 4;

const CHECK_VALID_TRANS_TYPE: u16 = 1 << 0;
const CHECK_VALID_OPERATION: u16 = 1 << 1;
const CHECK_VALID_LEVEL: u16 = 1 << 2;
const CHECK_VALID_PCC: u16 = 1 << 3;
const CHECK_VALID_UNCORRECTED: u16 = 1 << 4;
const CHECK_VALID_PRECISE_IP: u16 = 1 << 5;
const CHECK_VALID_RESTARTABLE_IP: u16 = 1 << 6;
const CHECK_VALID_OVERFLOW: u16 = 1 << 7;
const CHECK_VALID_BUS_PART_TYPE: u16 = 1 << 8;
const CHECK_VALID_BUS_TIME_OUT: u16 = 1 << 9;
const CHECK_VALID_BUS_ADDR_SPACE: u16 = 1 << 10;
const fn check_valid_bits(check: u64) -> u16 { (check & 0xffff) as u16 }
const fn check_trans_type(check: u64) -> u8 { ((check >> 16) & 3) as u8 }
const fn check_operation(check: u64) -> u8 { ((check >> 18) & 0xf) as u8 }
const fn check_level(check: u64) -> u8 { ((check >> 22) & 7) as u8 }
const CHECK_PCC: u64 = 1 << 25;
const CHECK_UNCORRECTED: u64 = 1 << 26;
const CHECK_PRECISE_IP: u64 = 1 << 27;
const CHECK_RESTARTABLE_IP: u64 = 1 << 28;
const CHECK_OVERFLOW: u64 = 1 << 29;
const fn check_bus_part_type(check: u64) -> u8 { ((check >> 30) & 3) as u8 }
const CHECK_BUS_TIME_OUT: u64 = 1 << 32;
const fn check_bus_addr_space(check: u64) -> u8 { ((check >> 33) & 3) as u8 }

const CHECK_VALID_MS_ERR_TYPE: u16 = 1 << 0;
const CHECK_VALID_MS_PCC: u16 = 1 << 1;
const CHECK_VALID_MS_UNCORRECTED: u16 = 1 << 2;
const CHECK_VALID_MS_PRECISE_IP: u16 = 1 << 3;
const CHECK_VALID_MS_RESTARTABLE_IP: u16 = 1 << 4;
const CHECK_VALID_MS_OVERFLOW: u16 = 1 << 5;
const fn check_ms_err_type(check: u64) -> u8 { ((check >> 16) & 7) as u8 }
const CHECK_MS_PCC: u64 = 1 << 19;
const CHECK_MS_UNCORRECTED: u64 = 1 << 20;
const CHECK_MS_PRECISE_IP: u64 = 1 << 21;
const CHECK_MS_RESTARTABLE_IP: u64 = 1 << 22;
const CHECK_MS_OVERFLOW: u64 = 1 << 23;
const CTX_TYPE_MSR: u8 = 1;
const CTX_TYPE_MMREG: u8 = 7;

#[repr(i32)]
enum ErrTypes { ErrTypeCache = 0, ErrTypeTlb, ErrTypeBus, ErrTypeMs, NErrTypes }

extern "C" {
    fn guid_equal(a: *const guid_t, b: *const guid_t) -> bool;
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn snprintf(buf: *mut core::ffi::c_char, size: usize, fmt: *const core::ffi::c_char, ... ) -> i32;
    fn print_hex_dump(prefix: *const core::ffi::c_char, ...);
    fn arch_apei_report_x86_error(ctx: *const cper_ia_proc_ctx, lapic_id: u64) -> bool;
    static cper_proc_error_type_strs: [*const core::ffi::c_char; 4];
}

unsafe fn cper_get_err_type(err_type: *const guid_t) -> ErrTypes {
    if guid_equal(err_type, &INFO_ERR_STRUCT_TYPE_CACHE) { ErrTypes::ErrTypeCache }
    else if guid_equal(err_type, &INFO_ERR_STRUCT_TYPE_TLB) { ErrTypes::ErrTypeTlb }
    else if guid_equal(err_type, &INFO_ERR_STRUCT_TYPE_BUS) { ErrTypes::ErrTypeBus }
    else if guid_equal(err_type, &INFO_ERR_STRUCT_TYPE_MS) { ErrTypes::ErrTypeMs }
    else { ErrTypes::NErrTypes }
}

static IA_CHECK_TRANS_TYPE_STRS: [&[u8]; 3] = [b"Instruction\0", b"Data Access\0", b"Generic\0"];
static IA_CHECK_OP_STRS: [&[u8]; 9] = [b"generic error\0", b"generic read\0", b"generic write\0", b"data read\0", b"data write\0", b"instruction fetch\0", b"prefetch\0", b"eviction\0", b"snoop\0"];
static IA_CHECK_BUS_PART_TYPE_STRS: [&[u8]; 4] = [b"Local Processor originated request\0", b"Local Processor responded to request\0", b"Local Processor observed\0", b"Generic\0"];
static IA_CHECK_BUS_ADDR_SPACE_STRS: [&[u8]; 4] = [b"Memory Access\0", b"Reserved\0", b"I/O\0", b"Other Transaction\0"];
static IA_CHECK_MS_ERROR_TYPE_STRS: [&[u8]; 6] = [b"No Error\0", b"Unclassified\0", b"Microcode ROM Parity Error\0", b"External Error\0", b"FRC Error\0", b"Internal Unclassified\0"];
static IA_REG_CTX_STRS: [&[u8]; 8] = [b"Unclassified Data\0", b"MSR Registers (Machine Check and other MSRs)\0", b"32-bit Mode Execution Context\0", b"64-bit Mode Execution Context\0", b"FXSAVE Context\0", b"32-bit Mode Debug Registers (DR0-DR7)\0", b"64-bit Mode Debug Registers (DR0-DR7)\0", b"Memory Mapped Registers\0"];

unsafe fn print_bool(str_: *const core::ffi::c_char, pfx: *const core::ffi::c_char, check: u64, bit: u64) {
    printk(b"%s%s: %s\n\0".as_ptr() as _, pfx, str_, if check & bit != 0 { b"true\0".as_ptr() } else { b"false\0".as_ptr() });
}

// The remaining decoding routines retain the C field-level behavior through
// the supplied kernel structs and helpers.
unsafe fn print_err_info_ms(pfx: *const core::ffi::c_char, validation_bits: u16, check: u64) {
    if validation_bits & CHECK_VALID_MS_ERR_TYPE != 0 { let err_type = check_ms_err_type(check); printk(b"%sError Type: %u\n\0".as_ptr() as _, pfx, err_type); }
    if validation_bits & CHECK_VALID_MS_PCC != 0 { print_bool(b"Processor Context Corrupt\0".as_ptr() as _, pfx, check, CHECK_MS_PCC); }
    if validation_bits & CHECK_VALID_MS_UNCORRECTED != 0 { print_bool(b"Uncorrected\0".as_ptr() as _, pfx, check, CHECK_MS_UNCORRECTED); }
    if validation_bits & CHECK_VALID_MS_PRECISE_IP != 0 { print_bool(b"Precise IP\0".as_ptr() as _, pfx, check, CHECK_MS_PRECISE_IP); }
    if validation_bits & CHECK_VALID_MS_RESTARTABLE_IP != 0 { print_bool(b"Restartable IP\0".as_ptr() as _, pfx, check, CHECK_MS_RESTARTABLE_IP); }
    if validation_bits & CHECK_VALID_MS_OVERFLOW != 0 { print_bool(b"Overflow\0".as_ptr() as _, pfx, check, CHECK_MS_OVERFLOW); }
}

unsafe fn print_err_info(pfx: *const core::ffi::c_char, err_type: ErrTypes, check: u64) {
    let validation_bits = check_valid_bits(check);
    if matches!(err_type, ErrTypes::ErrTypeMs) { return print_err_info_ms(pfx, validation_bits, check); }
    if validation_bits & CHECK_VALID_TRANS_TYPE != 0 { printk(b"%sTransaction Type: %u\n\0".as_ptr() as _, pfx, check_trans_type(check)); }
    if validation_bits & CHECK_VALID_OPERATION != 0 { printk(b"%sOperation: %u\n\0".as_ptr() as _, pfx, check_operation(check)); }
    if validation_bits & CHECK_VALID_LEVEL != 0 { printk(b"%sLevel: %llu\n\0".as_ptr() as _, pfx, check_level(check)); }
    if validation_bits & CHECK_VALID_PCC != 0 { print_bool(b"Processor Context Corrupt\0".as_ptr() as _, pfx, check, CHECK_PCC); }
    if validation_bits & CHECK_VALID_UNCORRECTED != 0 { print_bool(b"Uncorrected\0".as_ptr() as _, pfx, check, CHECK_UNCORRECTED); }
    if validation_bits & CHECK_VALID_PRECISE_IP != 0 { print_bool(b"Precise IP\0".as_ptr() as _, pfx, check, CHECK_PRECISE_IP); }
    if validation_bits & CHECK_VALID_RESTARTABLE_IP != 0 { print_bool(b"Restartable IP\0".as_ptr() as _, pfx, check, CHECK_RESTARTABLE_IP); }
    if validation_bits & CHECK_VALID_OVERFLOW != 0 { print_bool(b"Overflow\0".as_ptr() as _, pfx, check, CHECK_OVERFLOW); }
    if !matches!(err_type, ErrTypes::ErrTypeBus) { return; }
    if validation_bits & CHECK_VALID_BUS_PART_TYPE != 0 { printk(b"%sParticipation Type: %u\n\0".as_ptr() as _, pfx, check_bus_part_type(check)); }
    if validation_bits & CHECK_VALID_BUS_TIME_OUT != 0 { print_bool(b"Time Out\0".as_ptr() as _, pfx, check, CHECK_BUS_TIME_OUT); }
    if validation_bits & CHECK_VALID_BUS_ADDR_SPACE != 0 { printk(b"%sAddress Space: %u\n\0".as_ptr() as _, pfx, check_bus_addr_space(check)); }
}

// Struct definitions and GUID constants are external declarations from linux/cper.h.
unsafe extern "C" {
    static INFO_ERR_STRUCT_TYPE_CACHE: guid_t;
    static INFO_ERR_STRUCT_TYPE_TLB: guid_t;
    static INFO_ERR_STRUCT_TYPE_BUS: guid_t;
    static INFO_ERR_STRUCT_TYPE_MS: guid_t;
}

pub unsafe fn cper_print_proc_ia(pfx: *const core::ffi::c_char, proc: *const cper_sec_proc_ia) {
    let mut newpfx = [0i8; 64];
    let mut infopfx = [0i8; 64];
    if (*proc).validation_bits & VALID_LAPIC_ID != 0 { printk(b"%sLocal APIC_ID: 0x%llx\n\0".as_ptr() as _, pfx, (*proc).lapic_id); }
    if (*proc).validation_bits & VALID_CPUID_INFO != 0 {
        printk(b"%sCPUID Info:\n\0".as_ptr() as _, pfx);
        print_hex_dump(pfx, b"\0".as_ptr(), 0, 16, 4, (*proc).cpuid.as_ptr(), core::mem::size_of_val(&(*proc).cpuid), 0);
    }
    snprintf(newpfx.as_mut_ptr(), newpfx.len(), b"%s \0".as_ptr() as _, pfx);
    let mut err_info = (proc.add(1)) as *mut cper_ia_err_info;
    for i in 0..valid_proc_err_info_num((*proc).validation_bits) {
        printk(b"%sError Information Structure %d:\n\0".as_ptr() as _, pfx, i);
        let err_type = cper_get_err_type(&(*err_info).err_type);
        printk(b"%sError Structure Type: %s\n\0".as_ptr() as _, newpfx.as_ptr(), cper_proc_error_type_strs[err_type as usize]);
        if (*err_info).validation_bits & INFO_VALID_CHECK_INFO != 0 {
            printk(b"%sCheck Information: 0x%016llx\n\0".as_ptr() as _, newpfx.as_ptr(), (*err_info).check_info);
            if (err_type as i32) < (ErrTypes::NErrTypes as i32) { snprintf(infopfx.as_mut_ptr(), infopfx.len(), b"%s \0".as_ptr() as _, newpfx.as_ptr()); print_err_info(infopfx.as_ptr(), err_type, (*err_info).check_info); }
        }
        if (*err_info).validation_bits & INFO_VALID_TARGET_ID != 0 { printk(b"%sTarget Identifier: 0x%016llx\n\0".as_ptr() as _, newpfx.as_ptr(), (*err_info).target_id); }
        if (*err_info).validation_bits & INFO_VALID_REQUESTOR_ID != 0 { printk(b"%sRequestor Identifier: 0x%016llx\n\0".as_ptr() as _, newpfx.as_ptr(), (*err_info).requestor_id); }
        if (*err_info).validation_bits & INFO_VALID_RESPONDER_ID != 0 { printk(b"%sResponder Identifier: 0x%016llx\n\0".as_ptr() as _, newpfx.as_ptr(), (*err_info).responder_id); }
        if (*err_info).validation_bits & INFO_VALID_IP != 0 { printk(b"%sInstruction Pointer: 0x%016llx\n\0".as_ptr() as _, newpfx.as_ptr(), (*err_info).ip); }
        err_info = err_info.add(1);
    }
    let mut ctx_info = err_info as *mut cper_ia_proc_ctx;
    for i in 0..valid_proc_cxt_info_num((*proc).validation_bits) {
        let size = (core::mem::size_of::<cper_ia_proc_ctx>() + (*ctx_info).reg_arr_size as usize + 15) & !15;
        let mut groupsize = 4;
        printk(b"%sContext Information Structure %d:\n\0".as_ptr() as _, pfx, i);
        printk(b"%sRegister Context Type: %s\n\0".as_ptr() as _, newpfx.as_ptr(), IA_REG_CTX_STRS[(*ctx_info).reg_ctx_type as usize].as_ptr());
        printk(b"%sRegister Array Size: 0x%04x\n\0".as_ptr() as _, newpfx.as_ptr(), (*ctx_info).reg_arr_size);
        if (*ctx_info).reg_ctx_type == CTX_TYPE_MSR { groupsize = 8; printk(b"%sMSR Address: 0x%08x\n\0".as_ptr() as _, newpfx.as_ptr(), (*ctx_info).msr_addr); }
        if (*ctx_info).reg_ctx_type == CTX_TYPE_MMREG { printk(b"%sMM Register Address: 0x%016llx\n\0".as_ptr() as _, newpfx.as_ptr(), (*ctx_info).mm_reg_addr); }
        if (*ctx_info).reg_ctx_type != CTX_TYPE_MSR || arch_apei_report_x86_error(ctx_info, (*proc).lapic_id) { printk(b"%sRegister Array:\n\0".as_ptr() as _, newpfx.as_ptr()); print_hex_dump(newpfx.as_ptr(), b"\0".as_ptr(), 0, 16, groupsize, ctx_info.add(1), (*ctx_info).reg_arr_size, 0); }
        ctx_info = ((ctx_info as *mut u8).add(size)) as *mut cper_ia_proc_ctx;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
