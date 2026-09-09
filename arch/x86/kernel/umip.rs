/*
 * umip.rs - Rust translation of umip.c
 * Emulation for instructions protected by User-Mode Instruction Prevention.
 */

const UMIP_DUMMY_GDT_BASE: u64 = 0xfffffffffffe0000;
const UMIP_DUMMY_IDT_BASE: u64 = 0xffffffffffff0000;

const UMIP_GDT_IDT_BASE_SIZE_64BIT: usize = 8;
const UMIP_GDT_IDT_BASE_SIZE_32BIT: usize = 4;
const UMIP_GDT_IDT_LIMIT_SIZE: usize = 2;

const UMIP_INST_SGDT: i32 = 0;
const UMIP_INST_SIDT: i32 = 1;
const UMIP_INST_SMSW: i32 = 2;
const UMIP_INST_SLDT: i32 = 3;
const UMIP_INST_STR: i32 = 4;

static UMIP_INSNS: [&[u8]; 5] = [b"SGDT", b"SIDT", b"SMSW", b"SLDT", b"STR"];

/* External kernel types, constants, globals, and functions supplied elsewhere. */
extern "C" {
    fn insn_get_modrm(insn: *mut insn);
    fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn insn_fetch_from_user(regs: *mut pt_regs, buf: *mut u8) -> i32;
    fn insn_decode_from_regs(insn: *mut insn, regs: *mut pt_regs, buf: *mut u8, n: i32) -> bool;
    fn insn_get_modrm_rm_off(insn: *mut insn, regs: *mut pt_regs) -> i32;
    fn insn_get_addr_ref(insn: *mut insn, regs: *mut pt_regs) -> *mut core::ffi::c_void;
    fn copy_to_user(to: *mut core::ffi::c_void, from: *const u8, n: usize) -> usize;
    fn force_sig_fault(sig: i32, code: i32, addr: *mut core::ffi::c_void);
    fn cpu_feature_enabled(feature: i32) -> bool;
    fn user_64bit_mode(regs: *mut pt_regs) -> bool;
    fn unhandled_signal(tsk: *mut task_struct, sig: i32) -> bool;
    fn umip_printk(regs: *const pt_regs, level: *const u8, fmt: *const u8, ...);
    static mut current: *mut task_struct;
    static show_unhandled_signals: bool;
    static CR0_STATE: usize;
    static GDT_ENTRY_TSS: usize;
    static GDT_ENTRY_LDT: usize;
}

#[repr(C)]
pub struct insn {
    pub modrm: modrm,
    pub opcode: opcode,
    pub opnd_bytes: i32,
    pub length: i32,
}

#[repr(C)]
pub struct modrm { pub nbytes: u8, pub value: u8 }
#[repr(C)]
pub struct opcode { pub nbytes: u8, pub bytes: [u8; 4] }
#[repr(C)]
pub struct pt_regs { pub ip: usize, pub sp: usize }
#[repr(C)]
pub struct task_struct { pub comm: [u8; 16], pub thread: thread_struct }
#[repr(C)]
pub struct thread_struct { pub cr2: usize, pub error_code: usize, pub trap_nr: usize }

const EINVAL: i32 = 22;
const X86_FEATURE_UMIP: i32 = 0;
const X86_PF_USER: usize = 4;
const X86_PF_WRITE: usize = 2;
const X86_TRAP_PF: usize = 14;
const SIGSEGV: i32 = 11;
const SEGV_MAPERR: i32 = 1;

#[inline]
unsafe fn x86_modrm_reg(v: u8) -> u8 { (v >> 3) & 7 }
#[inline]
unsafe fn x86_modrm_mod(v: u8) -> u8 { v >> 6 }

unsafe fn identify_insn(insn: *mut insn) -> i32 {
    insn_get_modrm(insn);
    if (*insn).modrm.nbytes == 0 { return -EINVAL; }
    if (*insn).opcode.nbytes < 2 || (*insn).opcode.bytes[0] != 0xf { return -EINVAL; }
    if (*insn).opcode.bytes[1] == 1 {
        match x86_modrm_reg((*insn).modrm.value) {
            0 => if x86_modrm_mod((*insn).modrm.value) == 3 { -EINVAL } else { UMIP_INST_SGDT },
            1 => if x86_modrm_mod((*insn).modrm.value) == 3 { -EINVAL } else { UMIP_INST_SIDT },
            4 => UMIP_INST_SMSW,
            _ => -EINVAL,
        }
    } else if (*insn).opcode.bytes[1] == 0 {
        match x86_modrm_reg((*insn).modrm.value) {
            0 => UMIP_INST_SLDT,
            1 => UMIP_INST_STR,
            _ => -EINVAL,
        }
    } else { -EINVAL }
}

unsafe fn emulate_umip_insn(insn: *mut insn, umip_inst: i32, data: *mut u8,
                            data_size: *mut i32, x86_64: bool) -> i32 {
    if data.is_null() || data_size.is_null() || insn.is_null() { return -EINVAL; }
    if umip_inst == UMIP_INST_SGDT || umip_inst == UMIP_INST_SIDT {
        if x86_modrm_mod((*insn).modrm.value) == 3 { return -EINVAL; }
        let base = if umip_inst == UMIP_INST_SGDT { UMIP_DUMMY_GDT_BASE } else { UMIP_DUMMY_IDT_BASE };
        let base_size = if x86_64 { UMIP_GDT_IDT_BASE_SIZE_64BIT } else { UMIP_GDT_IDT_BASE_SIZE_32BIT };
        *data_size = (base_size + UMIP_GDT_IDT_LIMIT_SIZE) as i32;
        memcpy(data.add(2), (&base as *const u64).cast::<u8>(), base_size);
        let limit: u16 = 0;
        memcpy(data, (&limit as *const u16).cast::<u8>(), UMIP_GDT_IDT_LIMIT_SIZE);
    } else if umip_inst == UMIP_INST_SMSW || umip_inst == UMIP_INST_SLDT || umip_inst == UMIP_INST_STR {
        let dummy_value: usize = if umip_inst == UMIP_INST_SMSW { CR0_STATE }
            else if umip_inst == UMIP_INST_STR { GDT_ENTRY_TSS * 8 }
            else { GDT_ENTRY_LDT * 8 };
        *data_size = if x86_modrm_mod((*insn).modrm.value) == 3 { (*insn).opnd_bytes } else { 2 };
        memcpy(data, (&dummy_value as *const usize).cast::<u8>(), *data_size as usize);
    } else { return -EINVAL; }
    0
}

unsafe fn force_sig_info_umip_fault(addr: *mut core::ffi::c_void, regs: *mut pt_regs) {
    let tsk = current;
    (*tsk).thread.cr2 = addr as usize;
    (*tsk).thread.error_code = X86_PF_USER | X86_PF_WRITE;
    (*tsk).thread.trap_nr = X86_TRAP_PF;
    force_sig_fault(SIGSEGV, SEGV_MAPERR, addr);
    if show_unhandled_signals && unhandled_signal(tsk, SIGSEGV) {
        umip_printk(regs, b"KERN_ERR\0".as_ptr(), b"segfault in emulation. error%x\n\0".as_ptr(), X86_PF_USER | X86_PF_WRITE);
    }
}

pub unsafe fn fixup_umip_exception(regs: *mut pt_regs) -> bool {
    let mut dummy_data = [0u8; 10];
    let mut buf = [0u8; 64];
    let mut insn = core::mem::MaybeUninit::<insn>::uninit();
    if !cpu_feature_enabled(X86_FEATURE_UMIP) || regs.is_null() { return false; }
    let nr_copied = insn_fetch_from_user(regs, buf.as_mut_ptr());
    if nr_copied <= 0 || !insn_decode_from_regs(insn.as_mut_ptr(), regs, buf.as_mut_ptr(), nr_copied) { return false; }
    let insn = insn.as_mut_ptr();
    let umip_inst = identify_insn(insn);
    if umip_inst < 0 { return false; }
    let mut data_size = 0;
    if emulate_umip_insn(insn, umip_inst, dummy_data.as_mut_ptr(), &mut data_size, user_64bit_mode(regs)) != 0 { return false; }
    if x86_modrm_mod((*insn).modrm.value) == 3 {
        let offset = insn_get_modrm_rm_off(insn, regs);
        if offset < 0 { return false; }
        let reg_addr = (regs as *mut u8).offset(offset as isize);
        memcpy(reg_addr, dummy_data.as_ptr(), data_size as usize);
    } else {
        let uaddr = insn_get_addr_ref(insn, regs);
        if uaddr as isize == -1 { return false; }
        if copy_to_user(uaddr, dummy_data.as_ptr(), data_size as usize) > 0 {
            force_sig_info_umip_fault(uaddr, regs);
            return true;
        }
    }
    (*regs).ip = (*regs).ip.wrapping_add((*insn).length as usize);
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
