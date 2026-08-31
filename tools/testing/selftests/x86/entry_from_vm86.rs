// SPDX-License-Identifier: GPL-2.0-only
/*
 * entry_from_vm86.c - tests kernel entries from vm86 mode
 * Copyright (c) 2014-2015 Andrew Lutomirski
 *
 * This exercises a few paths that need to special-case vm86 mode.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::arch::global_asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type sig_atomic_t = c_int;

const REG_EFL: usize = 17;
const REG_CS: usize = 18;

const SIGSEGV: c_int = 11;
const SIGILL: c_int = 4;

const ENOSYS: c_int = 38;
const EPERM: c_int = 1;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const PROT_EXEC: c_int = 0x4;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;

const VM86_ENTER: c_uint = 1;
const VM86_UNKNOWN: c_long = 0;
const VM86_INTx: c_long = 2;
const VM86_SIGNAL: c_long = 3;
const VM86_STI: c_long = 4;
const VM86_TRAP: c_long = 6;

const X86_EFLAGS_IF: c_ulong = 1 << 9;
const X86_EFLAGS_VM: c_ulong = 1 << 17;
const X86_EFLAGS_VIP: c_ulong = 1 << 20;

static mut load_addr: c_ulong = 0x10000;
static mut nerrs: c_int = 0;

static mut got_signal: sig_atomic_t = 0;

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mcontext_t {
    pub gregs: [c_long; 23],
}

#[repr(C)]
pub struct ucontext_t {
    pub uc_flags: c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: [u8; 0],
    pub uc_mcontext: mcontext_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm86_regs {
    pub ebx: c_long,
    pub ecx: c_long,
    pub edx: c_long,
    pub esi: c_long,
    pub edi: c_long,
    pub ebp: c_long,
    pub eax: c_long,
    pub __null_ds: c_long,
    pub __null_es: c_long,
    pub __null_fs: c_long,
    pub __null_gs: c_long,
    pub orig_eax: c_long,
    pub eip: c_long,
    pub cs: c_ushort,
    pub __csh: c_ushort,
    pub eflags: c_long,
    pub esp: c_long,
    pub ss: c_ushort,
    pub __ssh: c_ushort,
    pub es: c_ushort,
    pub __esh: c_ushort,
    pub ds: c_ushort,
    pub __dsh: c_ushort,
    pub fs: c_ushort,
    pub __fsh: c_ushort,
    pub gs: c_ushort,
    pub __gsh: c_ushort,
}

type c_ushort = u16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct revectored_struct {
    pub __map: [c_ulong; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm86plus_info_struct {
    pub force_return_for_pic: c_ulong,
    pub vm86dbg_active: c_ulong,
    pub vm86dbg_TFpendig: c_ulong,
    pub unused: c_ulong,
    pub is_vm86pus: c_ulong,
    pub vm86dbg_intxxtab: [c_uchar; 32],
}

type c_uchar = u8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm86plus_struct {
    pub regs: vm86_regs,
    pub flags: c_ulong,
    pub screen_bitmap: c_ulong,
    pub cpu_type: c_ulong,
    pub int_revectored: revectored_struct,
    pub int21_revectored: revectored_struct,
    pub vm86plus: vm86plus_info_struct,
}

unsafe extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn fork() -> c_int;
    fn vm86(cmd: c_uint, arg: *mut vm86plus_struct) -> c_int;
    fn __errno_location() -> *mut c_int;
    fn sethandler(sig: c_int, handler: extern "C" fn(c_int, *mut siginfo_t, *mut c_void), flags: c_int);
    fn clearhandler(sig: c_int);

    static vmcode: c_uchar;
    static end_vmcode: c_uchar;
    static vmcode_bound: c_uchar;
    static vmcode_sysenter: c_uchar;
    static vmcode_syscall: c_uchar;
    static vmcode_sti: c_uchar;
    static vmcode_int3: c_uchar;
    static vmcode_int80: c_uchar;
    static vmcode_popf_hlt: c_uchar;
    static vmcode_umip: c_uchar;
    static vmcode_umip_str: c_uchar;
    static vmcode_umip_sldt: c_uchar;
}

global_asm!(
    r#"
	.pushsection .rodata
	.type vmcode_bound, @object
vmcode:
vmcode_bound:
	.code16
	bound %ax, (2048)
	int3
vmcode_sysenter:
	sysenter
vmcode_syscall:
	syscall
vmcode_sti:
	sti
vmcode_int3:
	int3
vmcode_int80:
	int $0x80
vmcode_popf_hlt:
	push %ax
	popf
	hlt
vmcode_umip:
	/* addressing via displacements */
	smsw (2052)
	sidt (2054)
	sgdt (2060)
	/* addressing via registers */
	mov $2066, %bx
	smsw (%bx)
	mov $2068, %bx
	sidt (%bx)
	mov $2074, %bx
	sgdt (%bx)
	/* register operands, only for smsw */
	smsw %ax
	mov %ax, (2080)
	int3
vmcode_umip_str:
	str %eax
vmcode_umip_sldt:
	sldt %eax
	int3
	.size vmcode, . - vmcode
end_vmcode:
	.code32
	.popsection
"#,
    options(att_syntax)
);

fn VM86_TYPE(ret: c_long) -> c_long {
    ret & 0xff
}

fn VM86_ARG(ret: c_long) -> c_long {
    (ret >> 8) & 0xff
}

extern "C" fn sighandler(sig: c_int, _info: *mut siginfo_t, ctx_void: *mut c_void) {
    unsafe {
        let ctx = ctx_void as *mut ucontext_t;

        if ((*ctx).uc_mcontext.gregs[REG_EFL] as c_ulong & X86_EFLAGS_VM) != 0
            || ((*ctx).uc_mcontext.gregs[REG_CS] & 3) != 3
        {
            printf(c"[FAIL]\tSignal frame should not reflect vm86 mode\n".as_ptr());
            nerrs += 1;
        }

        let signame: *const c_char;
        if sig == SIGSEGV {
            signame = c"SIGSEGV".as_ptr();
        } else if sig == SIGILL {
            signame = c"SIGILL".as_ptr();
        } else {
            signame = c"unexpected signal".as_ptr();
        }

        printf(
            c"[INFO]\t%s: FLAGS = 0x%lx, CS = 0x%hx\n".as_ptr(),
            signame,
            (*ctx).uc_mcontext.gregs[REG_EFL] as c_ulong,
            (*ctx).uc_mcontext.gregs[REG_CS] as c_ushort as c_int,
        );

        got_signal = 1;
    }
}

/* Returns false if the test was skipped. */
unsafe fn do_test(
    v86: *mut vm86plus_struct,
    eip: c_ulong,
    rettype: c_uint,
    retarg: c_uint,
    text: *const c_char,
) -> bool {
    let ret: c_long;

    printf(c"[RUN]\t%s from vm86 mode\n".as_ptr(), text);
    (*v86).regs.eip = eip as c_long;
    ret = vm86(VM86_ENTER, v86) as c_long;

    if ret == -1 && (*__errno_location() == ENOSYS || *__errno_location() == EPERM) {
        printf(
            c"[SKIP]\tvm86 %s\n".as_ptr(),
            if *__errno_location() == ENOSYS {
                c"not supported".as_ptr()
            } else {
                c"not allowed".as_ptr()
            },
        );
        return false;
    }

    if VM86_TYPE(ret) == VM86_INTx {
        let mut trapname = [0 as c_char; 32];
        let trapno = VM86_ARG(ret) as c_int;
        if trapno == 13 {
            strcpy(trapname.as_mut_ptr(), c"GP".as_ptr());
        } else if trapno == 5 {
            strcpy(trapname.as_mut_ptr(), c"BR".as_ptr());
        } else if trapno == 14 {
            strcpy(trapname.as_mut_ptr(), c"PF".as_ptr());
        } else {
            sprintf(trapname.as_mut_ptr(), c"%d".as_ptr(), trapno);
        }

        printf(c"[INFO]\tExited vm86 mode due to #%s\n".as_ptr(), trapname.as_ptr());
    } else if VM86_TYPE(ret) == VM86_UNKNOWN {
        printf(c"[INFO]\tExited vm86 mode due to unhandled GP fault\n".as_ptr());
    } else if VM86_TYPE(ret) == VM86_TRAP {
        printf(
            c"[INFO]\tExited vm86 mode due to a trap (arg=%ld)\n".as_ptr(),
            VM86_ARG(ret),
        );
    } else if VM86_TYPE(ret) == VM86_SIGNAL {
        printf(c"[INFO]\tExited vm86 mode due to a signal\n".as_ptr());
    } else if VM86_TYPE(ret) == VM86_STI {
        printf(c"[INFO]\tExited vm86 mode due to STI\n".as_ptr());
    } else {
        printf(
            c"[INFO]\tExited vm86 mode due to type %ld, arg %ld\n".as_ptr(),
            VM86_TYPE(ret),
            VM86_ARG(ret),
        );
    }

    if rettype == (-1i32) as c_uint
        || (VM86_TYPE(ret) == rettype as c_long && VM86_ARG(ret) == retarg as c_long)
    {
        printf(c"[OK]\tReturned correctly\n".as_ptr());
    } else {
        printf(
            c"[FAIL]\tIncorrect return reason (started at eip = 0x%lx, ended at eip = 0x%lx)\n".as_ptr(),
            eip,
            (*v86).regs.eip as c_ulong,
        );
        nerrs += 1;
    }

    true
}

pub unsafe fn do_umip_tests(vm86_arg: *mut vm86plus_struct, test_mem: *mut c_uchar) {
    #[repr(C, packed)]
    #[derive(Copy, Clone)]
    struct table_desc {
        limit: c_ushort,
        base: c_ulong,
    }

    /* Initialize variables with arbitrary values */
    let mut gdt1 = table_desc {
        base: 0x3c3c3c3c,
        limit: 0x9999,
    };
    let mut gdt2 = table_desc {
        base: 0x1a1a1a1a,
        limit: 0xaeae,
    };
    let mut idt1 = table_desc {
        base: 0x7b7b7b7b,
        limit: 0xf1f1,
    };
    let mut idt2 = table_desc {
        base: 0x89898989,
        limit: 0x1313,
    };
    let mut msw1: c_ushort = 0x1414;
    let mut msw2: c_ushort = 0x2525;
    let mut msw3: c_ushort = 3737;

    /* UMIP -- exit with INT3 unless kernel emulation did not trap #GP */
    do_test(
        vm86_arg,
        (&vmcode_umip as *const c_uchar).offset_from(&vmcode as *const c_uchar) as c_ulong,
        VM86_TRAP as c_uint,
        3,
        c"UMIP tests".as_ptr(),
    );

    /* Results from displacement-only addressing */
    msw1 = ptr::read_unaligned(test_mem.add(2052) as *const c_ushort);
    memcpy(
        &mut idt1 as *mut table_desc as *mut c_void,
        test_mem.add(2054) as *const c_void,
        mem::size_of_val(&idt1),
    );
    memcpy(
        &mut gdt1 as *mut table_desc as *mut c_void,
        test_mem.add(2060) as *const c_void,
        mem::size_of_val(&gdt1),
    );

    /* Results from register-indirect addressing */
    msw2 = ptr::read_unaligned(test_mem.add(2066) as *const c_ushort);
    memcpy(
        &mut idt2 as *mut table_desc as *mut c_void,
        test_mem.add(2068) as *const c_void,
        mem::size_of_val(&idt2),
    );
    memcpy(
        &mut gdt2 as *mut table_desc as *mut c_void,
        test_mem.add(2074) as *const c_void,
        mem::size_of_val(&gdt2),
    );

    /* Results when using register operands */
    msw3 = ptr::read_unaligned(test_mem.add(2080) as *const c_ushort);

    printf(c"[INFO]\tResult from SMSW:[0x%04x]\n".as_ptr(), msw1 as c_int);
    printf(
        c"[INFO]\tResult from SIDT: limit[0x%04x]base[0x%08lx]\n".as_ptr(),
        idt1.limit as c_int,
        idt1.base,
    );
    printf(
        c"[INFO]\tResult from SGDT: limit[0x%04x]base[0x%08lx]\n".as_ptr(),
        gdt1.limit as c_int,
        gdt1.base,
    );

    if msw1 != msw2 || msw1 != msw3 {
        printf(c"[FAIL]\tAll the results of SMSW should be the same.\n".as_ptr());
    } else {
        printf(c"[PASS]\tAll the results from SMSW are identical.\n".as_ptr());
    }

    if memcmp(
        &gdt1 as *const table_desc as *const c_void,
        &gdt2 as *const table_desc as *const c_void,
        mem::size_of_val(&gdt1),
    ) != 0
    {
        printf(c"[FAIL]\tAll the results of SGDT should be the same.\n".as_ptr());
    } else {
        printf(c"[PASS]\tAll the results from SGDT are identical.\n".as_ptr());
    }

    if memcmp(
        &idt1 as *const table_desc as *const c_void,
        &idt2 as *const table_desc as *const c_void,
        mem::size_of_val(&idt1),
    ) != 0
    {
        printf(c"[FAIL]\tAll the results of SIDT should be the same.\n".as_ptr());
    } else {
        printf(c"[PASS]\tAll the results from SIDT are identical.\n".as_ptr());
    }

    sethandler(SIGILL, sighandler, 0);
    do_test(
        vm86_arg,
        (&vmcode_umip_str as *const c_uchar).offset_from(&vmcode as *const c_uchar) as c_ulong,
        VM86_SIGNAL as c_uint,
        0,
        c"STR instruction".as_ptr(),
    );
    clearhandler(SIGILL);

    sethandler(SIGILL, sighandler, 0);
    do_test(
        vm86_arg,
        (&vmcode_umip_sldt as *const c_uchar).offset_from(&vmcode as *const c_uchar) as c_ulong,
        VM86_SIGNAL as c_uint,
        0,
        c"SLDT instruction".as_ptr(),
    );
    clearhandler(SIGILL);
}

unsafe fn c_assert(cond: bool) {
    if !cond {
        core::intrinsics::abort();
    }
}

pub unsafe fn main() -> c_int {
    let mut v86: vm86plus_struct = mem::zeroed();
    let addr = mmap(
        load_addr as *mut c_void,
        4096,
        PROT_READ | PROT_WRITE | PROT_EXEC,
        MAP_ANONYMOUS | MAP_PRIVATE,
        -1,
        0,
    ) as *mut c_uchar;
    if addr != load_addr as *mut c_uchar {
        err(1, c"mmap".as_ptr());
    }

    memcpy(
        addr as *mut c_void,
        &vmcode as *const c_uchar as *const c_void,
        (&end_vmcode as *const c_uchar).offset_from(&vmcode as *const c_uchar) as size_t,
    );
    *addr.add(2048) = 2;
    *addr.add(2050) = 3;

    memset(
        &mut v86 as *mut vm86plus_struct as *mut c_void,
        0,
        mem::size_of_val(&v86),
    );

    v86.regs.cs = (load_addr / 16) as c_ushort;
    v86.regs.ss = (load_addr / 16) as c_ushort;
    v86.regs.ds = (load_addr / 16) as c_ushort;
    v86.regs.es = (load_addr / 16) as c_ushort;

    /* Use the end of the page as our stack. */
    v86.regs.esp = 4096;

    c_assert((v86.regs.cs & 3) == 0); /* Looks like RPL = 0 */

    /* #BR -- should deliver SIG??? */
    do_test(
        &mut v86,
        (&vmcode_bound as *const c_uchar).offset_from(&vmcode as *const c_uchar) as c_ulong,
        VM86_INTx as c_uint,
        5,
        c"#BR".as_ptr(),
    );

    /*
     * SYSENTER -- should cause #GP or #UD depending on CPU.
     * Expected return type -1 means that we shouldn't validate
     * the vm86 return value.  This will avoid problems on non-SEP
     * CPUs.
     */
    sethandler(SIGILL, sighandler, 0);
    do_test(
        &mut v86,
        (&vmcode_sysenter as *const c_uchar).offset_from(&vmcode as *const c_uchar) as c_ulong,
        (-1i32) as c_uint,
        0,
        c"SYSENTER".as_ptr(),
    );
    clearhandler(SIGILL);

    /*
     * SYSCALL would be a disaster in VM86 mode.  Fortunately,
     * there is no kernel that both enables SYSCALL and sets
     * EFER.SCE, so it's #UD on all systems.  But vm86 is
     * buggy (or has a "feature"), so the SIGILL will actually
     * be delivered.
     */
    sethandler(SIGILL, sighandler, 0);
    do_test(
        &mut v86,
        (&vmcode_syscall as *const c_uchar).offset_from(&vmcode as *const c_uchar) as c_ulong,
        VM86_SIGNAL as c_uint,
        0,
        c"SYSCALL".as_ptr(),
    );
    clearhandler(SIGILL);

    /* STI with VIP set */
    v86.regs.eflags |= X86_EFLAGS_VIP as c_long;
    v86.regs.eflags &= !(X86_EFLAGS_IF as c_long);
    do_test(
        &mut v86,
        (&vmcode_sti as *const c_uchar).offset_from(&vmcode as *const c_uchar) as c_ulong,
        VM86_STI as c_uint,
        0,
        c"STI with VIP set".as_ptr(),
    );

    /* POPF with VIP set but IF clear: should not trap */
    v86.regs.eflags = X86_EFLAGS_VIP as c_long;
    v86.regs.eax = 0;
    do_test(
        &mut v86,
        (&vmcode_popf_hlt as *const c_uchar).offset_from(&vmcode as *const c_uchar) as c_ulong,
        VM86_UNKNOWN as c_uint,
        0,
        c"POPF with VIP set and IF clear".as_ptr(),
    );

    /* POPF with VIP set and IF set: should trap */
    v86.regs.eflags = X86_EFLAGS_VIP as c_long;
    v86.regs.eax = X86_EFLAGS_IF as c_long;
    do_test(
        &mut v86,
        (&vmcode_popf_hlt as *const c_uchar).offset_from(&vmcode as *const c_uchar) as c_ulong,
        VM86_STI as c_uint,
        0,
        c"POPF with VIP and IF set".as_ptr(),
    );

    /* POPF with VIP clear and IF set: should not trap */
    v86.regs.eflags = 0;
    v86.regs.eax = X86_EFLAGS_IF as c_long;
    do_test(
        &mut v86,
        (&vmcode_popf_hlt as *const c_uchar).offset_from(&vmcode as *const c_uchar) as c_ulong,
        VM86_UNKNOWN as c_uint,
        0,
        c"POPF with VIP clear and IF set".as_ptr(),
    );

    v86.regs.eflags = 0;

    /* INT3 -- should cause #BP */
    do_test(
        &mut v86,
        (&vmcode_int3 as *const c_uchar).offset_from(&vmcode as *const c_uchar) as c_ulong,
        VM86_TRAP as c_uint,
        3,
        c"INT3".as_ptr(),
    );

    /* INT80 -- should exit with "INTx 0x80" */
    v86.regs.eax = (-1i32) as c_uint as c_long;
    do_test(
        &mut v86,
        (&vmcode_int80 as *const c_uchar).offset_from(&vmcode as *const c_uchar) as c_ulong,
        VM86_INTx as c_uint,
        0x80,
        c"int80".as_ptr(),
    );

    /* UMIP -- should exit with INTx 0x80 unless UMIP was not disabled */
    do_umip_tests(&mut v86, addr);

    /* Execute a null pointer */
    v86.regs.cs = 0;
    v86.regs.ss = 0;
    sethandler(SIGSEGV, sighandler, 0);
    got_signal = 0;
    if do_test(&mut v86, 0, VM86_SIGNAL as c_uint, 0, c"Execute null pointer".as_ptr())
        && got_signal == 0
    {
        printf(c"[FAIL]\tDid not receive SIGSEGV\n".as_ptr());
        nerrs += 1;
    }
    clearhandler(SIGSEGV);

    /* Make sure nothing explodes if we fork. */
    if fork() == 0 {
        return 0;
    }

    if nerrs == 0 { 0 } else { 1 }
}
