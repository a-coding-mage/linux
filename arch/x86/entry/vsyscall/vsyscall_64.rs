// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2012-2014 Andy Lutomirski <luto@amacapital.net>
 *
 * Based on the original implementation which is:
 *  Copyright (C) 2001 Andrea Arcangeli <andrea@suse.de> SuSE
 *  Copyright 2003 Andi Kleen, SuSE Labs.
 *
 *  Parts of the original code have been moved to arch/x86/vdso/vma.c
 *
 * This file implements vsyscall emulation.
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct PtRegs {
    pub ip: c_ulong,
    pub cs: c_uint,
    pub sp: c_ulong,
    pub ax: c_ulong,
    pub si: c_ulong,
    pub di: c_ulong,
    pub dx: c_ulong,
    pub orig_ax: c_long,
}

#[repr(C)]
pub struct MmStruct {
    pub context: MmContext,
}

#[repr(C)]
pub struct MmContext {
    pub flags: c_ulong,
}

#[repr(C)]
pub struct VmAreaStruct {
    pub vm_start: c_ulong,
    pub vm_end: c_ulong,
    pub vm_page_prot: c_ulong,
    pub vm_flags: c_ulong,
    pub vm_ops: *const VmOperationsStruct,
}

#[repr(C)]
pub struct VmOperationsStruct {
    pub name: Option<unsafe extern "C" fn(*mut VmAreaStruct) -> *const c_char>,
}

type c_char = i8;
type c_int = i32;
type c_uint = u32;
type c_long = i64;
type c_ulong = u64;
type size_t = usize;

const EMULATE: c_int = 0;
const XONLY: c_int = 1;
const NONE: c_int = 2;

extern "C" {
    static mut vsyscall_mode: c_int;
    static mut show_unhandled_signals: bool;
    static mut current: *mut TaskStruct;
    static mut swapper_pg_dir: *mut PgdT;

    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn cpu_feature_enabled(feature: c_int) -> bool;
    fn setup_clear_cpu_cap(feature: c_int);
    fn pr_warn_once(fmt: *const c_char, ...);
    fn printk_ratelimited(fmt: *const c_char, ...);
    fn task_pid_nr(task: *mut TaskStruct) -> c_int;
    fn access_ok(ptr: *const core::ffi::c_void, size: size_t) -> bool;
    fn force_sig_fault(sig: c_int, code: c_int, addr: *mut core::ffi::c_void);
    fn force_exit_sig(sig: c_int);
    fn force_sig(sig: c_int);
    fn get_user(dst: *mut c_ulong, src: *const c_ulong) -> c_int;
    fn user_64bit_mode(regs: *mut PtRegs) -> bool;
    fn seccomp_permit_syscall() -> bool;
    fn trace_emulate_vsyscall(nr: c_int);
    fn is_vsyscall_vaddr(addr: c_ulong) -> bool;
    fn WARN_ON_ONCE(condition: bool);
    fn pgd_offset_pgd(root: *mut PgdT, addr: c_ulong) -> *mut PgdT;
    fn set_pgd(p: *mut PgdT, v: PgdT);
    fn p4d_offset(p: *mut PgdT, addr: c_ulong) -> *mut P4dT;
    fn set_p4d(p: *mut P4dT, v: P4dT);
    fn pud_offset(p: *mut P4dT, addr: c_ulong) -> *mut PudT;
    fn set_pud(p: *mut PudT, v: PudT);
    fn pmd_offset(p: *mut PudT, addr: c_ulong) -> *mut PmdT;
    fn set_pmd(p: *mut PmdT, v: PmdT);
    fn pgd_val(p: PgdT) -> c_ulong;
    fn p4d_val(p: P4dT) -> c_ulong;
    fn pud_val(p: PudT) -> c_ulong;
    fn pmd_val(p: PmdT) -> c_ulong;
    fn __pgd(v: c_ulong) -> PgdT;
    fn __p4d(v: c_ulong) -> P4dT;
    fn __pud(v: c_ulong) -> PudT;
    fn __pmd(v: c_ulong) -> PmdT;
    fn __set_fixmap(page: c_ulong, phys: c_ulong, prot: c_ulong);
    fn __pa_symbol(p: *const core::ffi::c_void) -> c_ulong;
    fn vm_flags_init(vma: *mut VmAreaStruct, flags: c_ulong);
    fn build_bug_on(condition: bool);
    fn __fix_to_virt(page: c_ulong) -> c_ulong;
    fn __x64_sys_gettimeofday(regs: *mut PtRegs) -> c_long;
    fn __x64_sys_time(regs: *mut PtRegs) -> c_long;
    fn __x64_sys_getcpu(regs: *mut PtRegs) -> c_long;
}

#[repr(C)] pub struct TaskStruct { pub comm: [c_char; 16], pub thread: ThreadStruct }
#[repr(C)] pub struct ThreadStruct { pub error_code: c_ulong, pub cr2: c_ulong, pub trap_nr: c_ulong }
#[repr(C)] pub struct PgdT { _private: [u8; 0] }
#[repr(C)] pub struct P4dT { _private: [u8; 0] }
#[repr(C)] pub struct PudT { _private: [u8; 0] }
#[repr(C)] pub struct PmdT { _private: [u8; 0] }

const VSYSCALL_ADDR: c_ulong = 0xffffffffff600000;
const PAGE_SIZE: c_ulong = 4096;
const PAGE_MASK: c_ulong = !(PAGE_SIZE - 1);
const X86_PF_WRITE: c_ulong = 2;
const X86_PF_USER: c_ulong = 4;
const X86_PF_INSTR: c_ulong = 1 << 4;
const X86_TRAP_PF: c_ulong = 14;
const SIGSEGV: c_int = 11;
const SIGSYS: c_int = 31;
const SEGV_MAPERR: c_int = 1;
const EINVAL: c_long = 22;
const EFAULT: c_long = 14;
const ENOSYS: c_long = 38;
const __NR_gettimeofday: c_int = 96;
const __NR_time: c_int = 201;
const __NR_getcpu: c_int = 309;
const X86_FEATURE_LASS: c_int = 0;
const X86_FEATURE_NX: c_int = 1;
const MM_CONTEXT_HAS_VSYSCALL: c_ulong = 0;
const VM_READ: c_ulong = 1;
const VM_EXEC: c_ulong = 4;
const PAGE_READONLY_EXEC: c_ulong = 0;
const PAGE_KERNEL_VVAR: c_ulong = 0;
const VSYSCALL_PAGE: c_ulong = 0;
const _PAGE_USER: c_ulong = 1;

static mut GATE_VMA_OPS: VmOperationsStruct = VmOperationsStruct { name: Some(gate_vma_name) };
static mut GATE_VMA: VmAreaStruct = VmAreaStruct {
    vm_start: VSYSCALL_ADDR,
    vm_end: VSYSCALL_ADDR + PAGE_SIZE,
    vm_page_prot: PAGE_READONLY_EXEC,
    vm_flags: VM_READ | VM_EXEC,
    vm_ops: core::ptr::addr_of!(GATE_VMA_OPS),
};

unsafe extern "C" fn warn_bad_vsyscall(_level: *const c_char, _regs: *mut PtRegs, _message: *const c_char) {}

unsafe fn addr_to_vsyscall_nr(addr: c_ulong) -> c_int {
    if (addr & !0xc00) != VSYSCALL_ADDR { return -EINVAL as c_int; }
    let nr = ((addr & 0xc00) >> 10) as c_int;
    if nr >= 3 { return -EINVAL as c_int; }
    nr
}

unsafe fn write_ok_or_segv(ptr: c_ulong, size: size_t) -> bool {
    if !access_ok(ptr as *const _, size) {
        (*current).thread.error_code = X86_PF_USER | X86_PF_WRITE;
        (*current).thread.cr2 = ptr;
        (*current).thread.trap_nr = X86_TRAP_PF;
        force_sig_fault(SIGSEGV, SEGV_MAPERR, ptr as *mut _);
        false
    } else { true }
}

unsafe fn __emulate_vsyscall(regs: *mut PtRegs, address: c_ulong) -> bool {
    if !user_64bit_mode(regs) || vsyscall_mode == NONE { return false; }
    let nr = addr_to_vsyscall_nr(address);
    trace_emulate_vsyscall(nr);
    if nr < 0 || get_user(&mut (*regs).sp as *mut c_ulong, (*regs).sp as *const c_ulong) != 0 { force_sig(SIGSEGV); return true; }
    let syscall_nr = match nr { 0 => __NR_gettimeofday, 1 => __NR_time, 2 => __NR_getcpu, _ => return false };
    (*regs).orig_ax = syscall_nr as c_long;
    (*regs).ax = -ENOSYS as c_ulong;
    if !seccomp_permit_syscall() { (*regs).ip = (*regs).sp; (*regs).sp += 8; return true; }
    let ret = match nr { 0 => __x64_sys_gettimeofday(regs), 1 => __x64_sys_time(regs), 2 => { let dx = (*regs).dx; (*regs).dx = 0; let r = __x64_sys_getcpu(regs); (*regs).dx = dx; r }, _ => -EFAULT };
    if ret == -EFAULT { force_sig(SIGSEGV); return true; }
    (*regs).ax = ret as c_ulong;
    (*regs).ip = (*regs).sp;
    (*regs).sp += 8;
    true
}

pub unsafe extern "C" fn emulate_vsyscall_pf(error_code: c_ulong, regs: *mut PtRegs, address: c_ulong) -> bool {
    if (error_code & (X86_PF_WRITE | X86_PF_USER)) != X86_PF_USER || address != (*regs).ip { return false; }
    if cpu_feature_enabled(X86_FEATURE_NX) { WARN_ON_ONCE(error_code & X86_PF_INSTR == 0); }
    __emulate_vsyscall(regs, address)
}

pub unsafe extern "C" fn emulate_vsyscall_gp(regs: *mut PtRegs) -> bool {
    if !cpu_feature_enabled(X86_FEATURE_LASS) || !is_vsyscall_vaddr((*regs).ip) { return false; }
    __emulate_vsyscall(regs, (*regs).ip)
}

unsafe extern "C" fn gate_vma_name(_vma: *mut VmAreaStruct) -> *const c_char { b"[vsyscall]\0".as_ptr() as *const c_char }

pub unsafe extern "C" fn get_gate_vma(mm: *mut MmStruct) -> *mut VmAreaStruct {
    if vsyscall_mode == NONE { return core::ptr::null_mut(); }
    core::ptr::addr_of_mut!(GATE_VMA)
}

pub unsafe extern "C" fn in_gate_area(mm: *mut MmStruct, addr: c_ulong) -> c_int {
    let vma = get_gate_vma(mm);
    if vma.is_null() { return 0; }
    ((*vma).vm_start <= addr && addr < (*vma).vm_end) as c_int
}

pub unsafe extern "C" fn in_gate_area_no_mm(addr: c_ulong) -> c_int {
    (vsyscall_mode != NONE && (addr & PAGE_MASK) == VSYSCALL_ADDR) as c_int
}

pub unsafe extern "C" fn set_vsyscall_pgtable_user_bits(root: *mut PgdT) {
    let pgd = pgd_offset_pgd(root, VSYSCALL_ADDR); set_pgd(pgd, __pgd(pgd_val(*pgd) | _PAGE_USER));
    let p4d = p4d_offset(pgd, VSYSCALL_ADDR); set_p4d(p4d, __p4d(p4d_val(*p4d) | _PAGE_USER));
    let pud = pud_offset(p4d, VSYSCALL_ADDR); set_pud(pud, __pud(pud_val(*pud) | _PAGE_USER));
    let pmd = pmd_offset(pud, VSYSCALL_ADDR); set_pmd(pmd, __pmd(pmd_val(*pmd) | _PAGE_USER));
}

pub unsafe extern "C" fn map_vsyscall() {
    extern "C" { static __vsyscall_page: u8; }
    let physaddr_vsyscall = __pa_symbol(&__vsyscall_page);
    if vsyscall_mode == EMULATE { __set_fixmap(VSYSCALL_PAGE, physaddr_vsyscall, PAGE_KERNEL_VVAR); set_vsyscall_pgtable_user_bits(swapper_pg_dir); }
    if vsyscall_mode == XONLY { vm_flags_init(core::ptr::addr_of_mut!(GATE_VMA), VM_EXEC); }
    build_bug_on(__fix_to_virt(VSYSCALL_PAGE) != VSYSCALL_ADDR);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
