// SPDX-License-Identifier: GPL-2.0
/*
 * UEFI Common Platform Error Record (CPER) support
 *
 * Copyright (C) 2017, The Linux Foundation. All rights reserved.
 */

use core::ffi::{c_char, c_int, c_long, c_void};

extern "C" {
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ... ) -> c_int;
    fn cper_bits_to_str(buf: *mut c_char, size: usize, value: u32,
                        strings: *const *const c_char, count: usize);
    fn print_hex_dump(prefix: *const c_char, ascii: *const c_char,
                      dump_prefix: c_int, rowsize: c_int, groupsize: c_int,
                      buf: *const c_void, len: usize, ascii2: bool);
}

#[repr(C)]
pub struct cper_sec_proc_arm {
    pub midr: u64,
    pub section_length: u32,
    pub err_info_num: u32,
    pub validation_bits: u32,
    pub mpidr: u64,
    pub affinity_level: u8,
    pub running_state: u8,
    pub psci_state: u8,
}
#[repr(C)]
pub struct cper_arm_err_info {
    pub validation_bits: u8,
    pub flags: u8,
    pub multiple_error: u8,
    pub type_: u8,
    pub error_info: u64,
    pub virt_fault_addr: u64,
    pub physical_fault_addr: u64,
}
#[repr(C)]
pub struct cper_arm_ctx_info { pub type_: u16, pub size: u16 }

extern "C" {
    static cper_proc_error_type_strs: *const *const c_char;
}

static ARM_REG_CTX_STRS: [&[u8]; 9] = [
    b"AArch32 general purpose registers\0", b"AArch32 EL1 context registers\0",
    b"AArch32 EL2 context registers\0", b"AArch32 secure context registers\0",
    b"AArch64 general purpose registers\0", b"AArch64 EL1 context registers\0",
    b"AArch64 EL2 context registers\0", b"AArch64 EL3 context registers\0",
    b"Misc. system register structure\0",
];
static ARM_ERR_TRANS_TYPE_STRS: [&[u8]; 3] = [b"Instruction\0", b"Data Access\0", b"Generic\0"];
static ARM_BUS_ERR_OP_STRS: [&[u8]; 7] = [
    b"Generic error (type cannot be determined)\0", b"Generic read (type of instruction or data request cannot be determined)\0",
    b"Generic write (type of instruction of data request cannot be determined)\0", b"Data read\0",
    b"Data write\0", b"Instruction fetch\0", b"Prefetch\0",
];
static ARM_CACHE_ERR_OP_STRS: [&[u8]; 11] = [
    b"Generic error (type cannot be determined)\0", b"Generic read (type of instruction or data request cannot be determined)\0",
    b"Generic write (type of instruction of data request cannot be determined)\0", b"Data read\0", b"Data write\0",
    b"Instruction fetch\0", b"Prefetch\0", b"Eviction\0", b"Snooping (processor initiated a cache snoop that resulted in an error)\0",
    b"Snooped (processor raised a cache error caused by another processor or device snooping its cache)\0", b"Management\0",
];
static ARM_TLB_ERR_OP_STRS: [&[u8]; 9] = [
    b"Generic error (type cannot be determined)\0", b"Generic read (type of instruction or data request cannot be determined)\0",
    b"Generic write (type of instruction of data request cannot be determined)\0", b"Data read\0", b"Data write\0",
    b"Instruction fetch\0", b"Prefetch\0", b"Local management operation (processor initiated a TLB management operation that resulted in an error)\0",
    b"External management operation (processor raised a TLB error caused by another processor or device broadcasting TLB operations)\0",
];
static ARM_BUS_ERR_PART_TYPE_STRS: [&[u8]; 4] = [b"Local processor originated request\0", b"Local processor responded to request\0", b"Local processor observed\0", b"Generic\0"];
static ARM_BUS_ERR_ADDR_SPACE_STRS: [&[u8]; 4] = [b"External Memory Access\0", b"Internal Memory Access\0", b"Unknown\0", b"Device Memory Access\0"];

unsafe fn cper_print_arm_err_info(pfx: *const c_char, type_: u32, error_info: u64) {
    if type_ & CPER_ARM_VENDOR_ERROR != 0 { return; }
    if error_info & CPER_ARM_ERR_VALID_TRANSACTION_TYPE != 0 {
        let v = ((error_info >> CPER_ARM_ERR_TRANSACTION_SHIFT) & CPER_ARM_ERR_TRANSACTION_MASK) as usize;
        if v < ARM_ERR_TRANS_TYPE_STRS.len() { printk(b"%stransaction type: %s\n\0".as_ptr() as _, pfx, ARM_ERR_TRANS_TYPE_STRS[v].as_ptr()); }
    }
    if error_info & CPER_ARM_ERR_VALID_OPERATION_TYPE != 0 {
        let v = ((error_info >> CPER_ARM_ERR_OPERATION_SHIFT) & CPER_ARM_ERR_OPERATION_MASK) as usize;
        if type_ & CPER_ARM_CACHE_ERROR != 0 && v < ARM_CACHE_ERR_OP_STRS.len() { printk(b"%scache error, operation type: %s\n\0".as_ptr() as _, pfx, ARM_CACHE_ERR_OP_STRS[v].as_ptr()); }
        if type_ & CPER_ARM_TLB_ERROR != 0 && v < ARM_TLB_ERR_OP_STRS.len() { printk(b"%sTLB error, operation type: %s\n\0".as_ptr() as _, pfx, ARM_TLB_ERR_OP_STRS[v].as_ptr()); }
        if type_ & CPER_ARM_BUS_ERROR != 0 && v < ARM_BUS_ERR_OP_STRS.len() { printk(b"%sbus error, operation type: %s\n\0".as_ptr() as _, pfx, ARM_BUS_ERR_OP_STRS[v].as_ptr()); }
    }
    if error_info & CPER_ARM_ERR_VALID_LEVEL != 0 {
        let v = ((error_info >> CPER_ARM_ERR_LEVEL_SHIFT) & CPER_ARM_ERR_LEVEL_MASK) as u8;
        if type_ & CPER_ARM_CACHE_ERROR != 0 { printk(b"%scache level: %d\n\0".as_ptr() as _, pfx, v as c_int); }
        if type_ & CPER_ARM_TLB_ERROR != 0 { printk(b"%sTLB level: %d\n\0".as_ptr() as _, pfx, v as c_int); }
        if type_ & CPER_ARM_BUS_ERROR != 0 { printk(b"%saffinity level at which the bus error occurred: %d\n\0".as_ptr() as _, pfx, v as c_int); }
    }
    macro_rules! bit_message { ($valid:ident, $shift:ident, $mask:ident, $yes:expr, $no:expr) => { if error_info & $valid != 0 { if ((error_info >> $shift) & $mask) != 0 { printk($yes.as_ptr() as _, pfx); } else { printk($no.as_ptr() as _, pfx); } } }; }
    bit_message!(CPER_ARM_ERR_VALID_PROC_CONTEXT_CORRUPT, CPER_ARM_ERR_PC_CORRUPT_SHIFT, CPER_ARM_ERR_PC_CORRUPT_MASK, b"%sprocessor context corrupted\n\0", b"%sprocessor context not corrupted\n\0");
    bit_message!(CPER_ARM_ERR_VALID_CORRECTED, CPER_ARM_ERR_CORRECTED_SHIFT, CPER_ARM_ERR_CORRECTED_MASK, b"%sthe error has been corrected\n\0", b"%sthe error has not been corrected\n\0");
    bit_message!(CPER_ARM_ERR_VALID_PRECISE_PC, CPER_ARM_ERR_PRECISE_PC_SHIFT, CPER_ARM_ERR_PRECISE_PC_MASK, b"%sPC is precise\n\0", b"%sPC is imprecise\n\0");
    if error_info & CPER_ARM_ERR_VALID_RESTARTABLE_PC != 0 && ((error_info >> CPER_ARM_ERR_RESTARTABLE_PC_SHIFT) & CPER_ARM_ERR_RESTARTABLE_PC_MASK) != 0 { printk(b"%sProgram execution can be restarted reliably at the PC associated with the error.\n\0".as_ptr() as _, pfx); }
    if type_ != CPER_ARM_BUS_ERROR { return; }
    if error_info & CPER_ARM_ERR_VALID_PARTICIPATION_TYPE != 0 { let v = ((error_info >> CPER_ARM_ERR_PARTICIPATION_TYPE_SHIFT) & CPER_ARM_ERR_PARTICIPATION_TYPE_MASK) as usize; if v < 4 { printk(b"%sparticipation type: %s\n\0".as_ptr() as _, pfx, ARM_BUS_ERR_PART_TYPE_STRS[v].as_ptr()); } }
    if error_info & CPER_ARM_ERR_VALID_TIME_OUT != 0 && ((error_info >> CPER_ARM_ERR_TIME_OUT_SHIFT) & CPER_ARM_ERR_TIME_OUT_MASK) != 0 { printk(b"%srequest timed out\n\0".as_ptr() as _, pfx); }
    if error_info & CPER_ARM_ERR_VALID_ADDRESS_SPACE != 0 { let v = ((error_info >> CPER_ARM_ERR_ADDRESS_SPACE_SHIFT) & CPER_ARM_ERR_ADDRESS_SPACE_MASK) as usize; if v < 4 { printk(b"%saddress space: %s\n\0".as_ptr() as _, pfx, ARM_BUS_ERR_ADDR_SPACE_STRS[v].as_ptr()); } }
    if error_info & CPER_ARM_ERR_VALID_MEM_ATTRIBUTES != 0 { let v = (error_info >> CPER_ARM_ERR_MEM_ATTRIBUTES_SHIFT) & CPER_ARM_ERR_MEM_ATTRIBUTES_MASK; printk(b"%smemory access attributes:0x%x\n\0".as_ptr() as _, pfx, v as u32); }
    if error_info & CPER_ARM_ERR_VALID_ACCESS_MODE != 0 { if ((error_info >> CPER_ARM_ERR_ACCESS_MODE_SHIFT) & CPER_ARM_ERR_ACCESS_MODE_MASK) != 0 { printk(b"%saccess mode: normal\n\0".as_ptr() as _, pfx); } else { printk(b"%saccess mode: secure\n\0".as_ptr() as _, pfx); } }
}

pub unsafe fn cper_print_proc_arm(pfx: *const c_char, proc: *const cper_sec_proc_arm, length: u32) {
    let p = &*proc;
    printk(b"%sMIDR: 0x%016llx\n\0".as_ptr() as _, pfx, p.midr);
    let mut len = p.section_length as isize - (core::mem::size_of::<cper_sec_proc_arm>() + p.err_info_num as usize * core::mem::size_of::<cper_arm_err_info>()) as isize;
    if len < 0 || p.section_length > length { printk(b"%ssection length: %d, CPER size: %d\n\0".as_ptr() as _, pfx, p.section_length, length); printk(b"%ssection length is too %s\n\0".as_ptr() as _, pfx, if len < 0 { b"small\0".as_ptr() } else { b"big\0".as_ptr() }); printk(b"%sfirmware-generated error record is incorrect\n\0".as_ptr() as _, pfx); printk(b"%sERR_INFO_NUM is %d\n\0".as_ptr() as _, pfx, p.err_info_num); return; }
    if p.validation_bits & CPER_ARM_VALID_MPIDR != 0 { printk(b"%sMultiprocessor Affinity Register (MPIDR): 0x%016llx\n\0".as_ptr() as _, pfx, p.mpidr); }
    if p.validation_bits & CPER_ARM_VALID_AFFINITY_LEVEL != 0 { printk(b"%serror affinity level: %d\n\0".as_ptr() as _, pfx, p.affinity_level); }
    if p.validation_bits & CPER_ARM_VALID_RUNNING_STATE != 0 { printk(b"%srunning state: 0x%x\n\0".as_ptr() as _, pfx, p.running_state); printk(b"%sPower State Coordination Interface state: %d\n\0".as_ptr() as _, pfx, p.psci_state); }
    let mut newpfx = [0i8; 64]; let mut infopfx = [0i8; 65]; let mut error_type = [0i8; 120]; snprintf(newpfx.as_mut_ptr(), 64, b"%s \0".as_ptr() as _, pfx);
    let mut err_info = (proc as *mut u8).add(core::mem::size_of::<cper_sec_proc_arm>()) as *mut cper_arm_err_info;
    for i in 0..p.err_info_num { let e = &*err_info; printk(b"%sError info structure %d:\n\0".as_ptr() as _, pfx, i); printk(b"%snum errors: %d\n\0".as_ptr() as _, pfx, e.multiple_error as c_int + 1); if e.validation_bits & CPER_ARM_INFO_VALID_FLAGS != 0 { if e.flags & CPER_ARM_INFO_FLAGS_FIRST != 0 { printk(b"%sfirst error captured\n\0".as_ptr() as _, newpfx.as_ptr()); } if e.flags & CPER_ARM_INFO_FLAGS_LAST != 0 { printk(b"%slast error captured\n\0".as_ptr() as _, newpfx.as_ptr()); } if e.flags & CPER_ARM_INFO_FLAGS_PROPAGATED != 0 { printk(b"%spropagated error captured\n\0".as_ptr() as _, newpfx.as_ptr()); } if e.flags & CPER_ARM_INFO_FLAGS_OVERFLOW != 0 { printk(b"%soverflow occurred, error info is incomplete\n\0".as_ptr() as _, newpfx.as_ptr()); } } cper_bits_to_str(error_type.as_mut_ptr(), 120, ((e.type_ as u32) & CPER_ARM_ERR_TYPE_MASK) as u32, cper_proc_error_type_strs, 0); printk(b"%serror_type: 0x%02x: %s%s\n\0".as_ptr() as _, newpfx.as_ptr(), e.type_, error_type.as_ptr(), if e.type_ as u32 & !CPER_ARM_ERR_TYPE_MASK != 0 { b" with reserved bit(s)\0".as_ptr() } else { b"\0".as_ptr() }); if e.validation_bits & CPER_ARM_INFO_VALID_ERR_INFO != 0 { printk(b"%serror_info: 0x%016llx\n\0".as_ptr() as _, newpfx.as_ptr(), e.error_info); snprintf(infopfx.as_mut_ptr(), 65, b"%s \0".as_ptr() as _, newpfx.as_ptr()); cper_print_arm_err_info(infopfx.as_ptr(), e.type_ as u32, e.error_info); } if e.validation_bits & CPER_ARM_INFO_VALID_VIRT_ADDR != 0 { printk(b"%svirtual fault address: 0x%016llx\n\0".as_ptr() as _, newpfx.as_ptr(), e.virt_fault_addr); } if e.validation_bits & CPER_ARM_INFO_VALID_PHYSICAL_ADDR != 0 { printk(b"%sphysical fault address: 0x%016llx\n\0".as_ptr() as _, newpfx.as_ptr(), e.physical_fault_addr); } err_info = err_info.add(1); }
    let mut ctx_info = err_info as *mut cper_arm_ctx_info; let max_ctx_type = ARM_REG_CTX_STRS.len() - 1;
    for i in 0..p.context_info_num { let c = &*ctx_info; let size = (core::mem::size_of::<cper_arm_ctx_info>() + c.size as usize + 15) & !15; printk(b"%sContext info structure %d:\n\0".as_ptr() as _, pfx, i); if len < size as isize { printk(b"%ssection length is too small\n\0".as_ptr() as _, newpfx.as_ptr()); printk(b"%sfirmware-generated error record is incorrect\n\0".as_ptr() as _, pfx); return; } if c.type_ as usize > max_ctx_type { printk(b"%sInvalid context type: %d (max: %d)\n\0".as_ptr() as _, newpfx.as_ptr(), c.type_, max_ctx_type); return; } printk(b"%sregister context type: %s\n\0".as_ptr() as _, newpfx.as_ptr(), ARM_REG_CTX_STRS[c.type_ as usize].as_ptr()); print_hex_dump(newpfx.as_ptr(), b"\0".as_ptr() as _, 0, 16, 4, ctx_info.add(1) as *const c_void, c.size as usize, false); len -= size as isize; ctx_info = (ctx_info as *mut u8).add(size) as *mut cper_arm_ctx_info; }
    if len > 0 { printk(b"%sVendor specific error info has %u bytes:\n\0".as_ptr() as _, pfx, len); print_hex_dump(newpfx.as_ptr(), b"\0".as_ptr() as _, 0, 16, 4, ctx_info as *const c_void, len as usize, true); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
