// SPDX-License-Identifier: GPL-2.0-only
/*
 * sigreturn.c - tests for x86 sigreturn(2) and exit-to-userspace
 * Copyright (c) 2014-2015 Andrew Lutomirski
 *
 * This is a series of tests that exercises the sigreturn(2) syscall and
 * the IRET / SYSRET paths in the kernel.
 *
 * For now, this focuses on the effects of unusual CS and SS values,
 * and it has a bunch of tests to make sure that ESP/RSP is restored
 * properly.
 *
 * The basic idea behind these tests is to raise(SIGUSR1) to create a
 * sigcontext frame, plug in the values to be tested, and then return,
 * which implicitly invokes sigreturn(2) and programs the user context
 * as desired.
 *
 * For tests for which we expect sigreturn and the subsequent return to
 * user mode to succeed, we return to a short trampoline that generates
 * SIGTRAP so that the meat of the tests can be ordinary C code in a
 * SIGTRAP handler.
 *
 * The inner workings of each test is documented below.
 *
 * Do not run on outdated, unpatched kernels at risk of nasty crashes.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::arch::{asm, global_asm};
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type u32 = c_uint;
type u16 = u16;
type greg_t = c_long;
type gregset_t = [greg_t; NGREG as usize];
type sig_atomic_t = c_int;

// Pull in AR_xyz defines.
// From "../../../../arch/x86/include/asm/desc_defs.h".
const AR_TYPE_MASK: u32 = 0x0000_0f00;
const AR_TYPE_RWDATA: u32 = 0x0000_0200;
const AR_TYPE_RWDATA_EXPDOWN: u32 = 0x0000_0600;
const AR_P: u32 = 0x0000_8000;

/*
 * Copied from asm/ucontext.h, as asm/ucontext.h conflicts badly with the glibc
 * headers.
 */
#[cfg(target_arch = "x86_64")]
const UC_SIGCONTEXT_SS: c_ulong = 0x2;
#[cfg(target_arch = "x86_64")]
const UC_STRICT_RESTORE_SS: c_ulong = 0x4;

/*
 * In principle, this test can run on Linux emulation layers (e.g.
 * Illumos "LX branded zones").  Solaris-based kernels reserve LDT
 * entries 0-5 for their own internal purposes, so start our LDT
 * allocations above that reservation.  (The tests don't pass on LX
 * branded zones, but at least this lets them run.)
 */
const LDT_OFFSET: c_int = 6;

const SYS_modify_ldt: c_long = 154;
#[cfg(target_arch = "x86_64")]
const SYS_set_thread_area: c_long = 205;
#[cfg(target_arch = "x86")]
const SYS_set_thread_area: c_long = 243;

const SIGUSR1: c_int = 10;
const SIGUSR2: c_int = 12;
const SIGSEGV: c_int = 11;
const SIGBUS: c_int = 7;
const SIGILL: c_int = 4;
const SIGTRAP: c_int = 5;
const SA_ONSTACK: c_int = 0x0800_0000;
const SIGSTKSZ: usize = 8192;

#[cfg(target_arch = "x86_64")]
const NGREG: c_int = 23;
#[cfg(target_arch = "x86_64")]
const REG_RIP: usize = 16;
#[cfg(target_arch = "x86_64")]
const REG_RSP: usize = 15;
#[cfg(target_arch = "x86_64")]
const REG_RCX: usize = 14;
#[cfg(target_arch = "x86_64")]
const REG_CSGSFS: usize = 18;
#[cfg(target_arch = "x86_64")]
const REG_ERR: usize = 19;
#[cfg(target_arch = "x86_64")]
const REG_TRAPNO: usize = 20;

#[cfg(target_arch = "x86")]
const NGREG: c_int = 19;
#[cfg(target_arch = "x86")]
const REG_EIP: usize = 14;
#[cfg(target_arch = "x86")]
const REG_ESP: usize = 7;
#[cfg(target_arch = "x86")]
const REG_ECX: usize = 1;
#[cfg(target_arch = "x86")]
const REG_SS: usize = 18;
#[cfg(target_arch = "x86")]
const REG_CS: usize = 15;
#[cfg(target_arch = "x86")]
const REG_DS: usize = 3;
#[cfg(target_arch = "x86")]
const REG_ES: usize = 4;
#[cfg(target_arch = "x86")]
const REG_UESP: usize = 17;
#[cfg(target_arch = "x86")]
const REG_ERR: usize = 13;
#[cfg(target_arch = "x86")]
const REG_TRAPNO: usize = 12;

#[cfg(target_arch = "x86_64")]
const REG_IP: usize = REG_RIP;
#[cfg(target_arch = "x86_64")]
const REG_SP: usize = REG_RSP;
#[cfg(target_arch = "x86_64")]
const REG_CX: usize = REG_RCX;

#[cfg(target_arch = "x86")]
const REG_IP: usize = REG_EIP;
#[cfg(target_arch = "x86")]
const REG_SP: usize = REG_ESP;
#[cfg(target_arch = "x86")]
const REG_CX: usize = REG_ECX;

#[repr(C)]
struct user_desc {
    entry_number: c_uint,
    base_addr: c_uint,
    limit: c_uint,
    seg_32bit: c_uint,
    contents: c_uint,
    read_exec_only: c_uint,
    limit_in_pages: c_uint,
    seg_not_present: c_uint,
    useable: c_uint,
}

#[repr(C)]
struct mcontext_t {
    gregs: gregset_t,
}

#[repr(C)]
struct ucontext_t {
    uc_flags: c_ulong,
    uc_link: *mut ucontext_t,
    uc_stack: stack_t,
    uc_mcontext: mcontext_t,
}

#[repr(C)]
struct siginfo_t {
    _private: [u8; 128],
}

#[repr(C)]
struct stack_t {
    ss_sp: *mut c_void,
    ss_flags: c_int,
    ss_size: usize,
}

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strsignal(sig: c_int) -> *mut c_char;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn raise(sig: c_int) -> c_int;
    fn sigaltstack(ss: *const stack_t, old_ss: *mut stack_t) -> c_int;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn errx(eval: c_int, fmt: *const c_char, ...) -> !;

    // From "helpers.h".
    fn sethandler(sig: c_int, handler: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void), flags: c_int);
    fn clearhandler(sig: c_int);
}

/* An aligned stack accessible through some of our segments. */
#[repr(align(4096))]
struct AlignedStack16([u8; 65536]);
static mut stack16: AlignedStack16 = AlignedStack16([0; 65536]);

/*
 * An aligned int3 instruction used as a trampoline.  Some of the tests
 * want to fish out their ss values, so this trampoline copies ss to eax
 * before the int3.
 */
global_asm!(
    ".pushsection .text",
    ".type int3, @function",
    ".align 4096",
    "int3:",
    "mov %ss,%ecx",
    "int3",
    ".size int3, . - int3",
    ".align 4096, 0xcc",
    ".popsection",
);

unsafe extern "C" {
    static int3: [c_char; 4096];
}

/*
 * At startup, we prepapre:
 *
 * - ldt_nonexistent_sel: An LDT entry that doesn't exist (all-zero
 *   descriptor or out of bounds).
 * - code16_sel: A 16-bit LDT code segment pointing to int3.
 * - data16_sel: A 16-bit LDT data segment pointing to stack16.
 * - npcode32_sel: A 32-bit not-present LDT code segment pointing to int3.
 * - npdata32_sel: A 32-bit not-present LDT data segment pointing to stack16.
 * - gdt_data16_idx: A 16-bit GDT data segment pointing to stack16.
 * - gdt_npdata32_idx: A 32-bit not-present GDT data segment pointing to
 *   stack16.
 *
 * For no particularly good reason, xyz_sel is a selector value with the
 * RPL and LDT bits filled in, whereas xyz_idx is just an index into the
 * descriptor table.  These variables will be zero if their respective
 * segments could not be allocated.
 */
static mut ldt_nonexistent_sel: u16 = 0;
static mut code16_sel: u16 = 0;
static mut data16_sel: u16 = 0;
static mut npcode32_sel: u16 = 0;
static mut npdata32_sel: u16 = 0;

static mut gdt_data16_idx: u16 = 0;
static mut gdt_npdata32_idx: u16 = 0;

unsafe fn GDT3(idx: c_int) -> u16 {
    ((idx << 3) | 3) as u16
}

unsafe fn LDT3(idx: c_int) -> u16 {
    ((idx << 3) | 7) as u16
}

unsafe fn add_ldt(desc: *const user_desc, var: *mut u16, name: *const c_char) {
    if syscall(SYS_modify_ldt, 1 as c_int, desc, size_of::<user_desc>()) == 0 {
        *var = LDT3((*desc).entry_number as c_int);
    } else {
        printf(c"[NOTE]\tFailed to create %s segment\n".as_ptr(), name);
        *var = 0;
    }
}

unsafe fn setup_ldt() {
    if (&raw const stack16 as c_ulong) > ((1u64 << 32) - size_of::<AlignedStack16>() as u64) as c_ulong {
        errx(1, c"stack16 is too high\n".as_ptr());
    }
    if (&raw const int3 as c_ulong) > ((1u64 << 32) - size_of::<[c_char; 4096]>() as u64) as c_ulong {
        errx(1, c"int3 is too high\n".as_ptr());
    }

    ldt_nonexistent_sel = LDT3(LDT_OFFSET + 2);

    let code16_desc = user_desc {
        entry_number: (LDT_OFFSET + 0) as c_uint,
        base_addr: &raw const int3 as c_uint,
        limit: 4095,
        seg_32bit: 0,
        contents: 2, /* Code, not conforming */
        read_exec_only: 0,
        limit_in_pages: 0,
        seg_not_present: 0,
        useable: 0,
    };
    add_ldt(&code16_desc, &raw mut code16_sel, c"code16".as_ptr());

    let data16_desc = user_desc {
        entry_number: (LDT_OFFSET + 1) as c_uint,
        base_addr: &raw const stack16 as c_uint,
        limit: 0xffff,
        seg_32bit: 0,
        contents: 0, /* Data, grow-up */
        read_exec_only: 0,
        limit_in_pages: 0,
        seg_not_present: 0,
        useable: 0,
    };
    add_ldt(&data16_desc, &raw mut data16_sel, c"data16".as_ptr());

    let npcode32_desc = user_desc {
        entry_number: (LDT_OFFSET + 3) as c_uint,
        base_addr: &raw const int3 as c_uint,
        limit: 4095,
        seg_32bit: 1,
        contents: 2, /* Code, not conforming */
        read_exec_only: 0,
        limit_in_pages: 0,
        seg_not_present: 1,
        useable: 0,
    };
    add_ldt(&npcode32_desc, &raw mut npcode32_sel, c"npcode32".as_ptr());

    let npdata32_desc = user_desc {
        entry_number: (LDT_OFFSET + 4) as c_uint,
        base_addr: &raw const stack16 as c_uint,
        limit: 0xffff,
        seg_32bit: 1,
        contents: 0, /* Data, grow-up */
        read_exec_only: 0,
        limit_in_pages: 0,
        seg_not_present: 1,
        useable: 0,
    };
    add_ldt(&npdata32_desc, &raw mut npdata32_sel, c"npdata32".as_ptr());

    let mut gdt_data16_desc = user_desc {
        entry_number: (-1_i32) as c_uint,
        base_addr: &raw const stack16 as c_uint,
        limit: 0xffff,
        seg_32bit: 0,
        contents: 0, /* Data, grow-up */
        read_exec_only: 0,
        limit_in_pages: 0,
        seg_not_present: 0,
        useable: 0,
    };

    if syscall(SYS_set_thread_area, &mut gdt_data16_desc as *mut user_desc) == 0 {
        /*
         * This probably indicates vulnerability to CVE-2014-8133.
         * Merely getting here isn't definitive, though, and we'll
         * diagnose the problem for real later on.
         */
        printf(
            c"[WARN]\tset_thread_area allocated data16 at index %d\n".as_ptr(),
            gdt_data16_desc.entry_number as c_int,
        );
        gdt_data16_idx = gdt_data16_desc.entry_number as u16;
    } else {
        printf(c"[OK]\tset_thread_area refused 16-bit data\n".as_ptr());
    }

    let mut gdt_npdata32_desc = user_desc {
        entry_number: (-1_i32) as c_uint,
        base_addr: &raw const stack16 as c_uint,
        limit: 0xffff,
        seg_32bit: 1,
        contents: 0, /* Data, grow-up */
        read_exec_only: 0,
        limit_in_pages: 0,
        seg_not_present: 1,
        useable: 0,
    };

    if syscall(SYS_set_thread_area, &mut gdt_npdata32_desc as *mut user_desc) == 0 {
        /*
         * As a hardening measure, newer kernels don't allow this.
         */
        printf(
            c"[WARN]\tset_thread_area allocated npdata32 at index %d\n".as_ptr(),
            gdt_npdata32_desc.entry_number as c_int,
        );
        gdt_npdata32_idx = gdt_npdata32_desc.entry_number as u16;
    } else {
        printf(c"[OK]\tset_thread_area refused 16-bit data\n".as_ptr());
    }
}

/* State used by our signal handlers. */
static mut initial_regs: gregset_t = [0; NGREG as usize];
static mut requested_regs: gregset_t = [0; NGREG as usize];
static mut resulting_regs: gregset_t = [0; NGREG as usize];

/* Instructions for the SIGUSR1 handler. */
static mut sig_cs: u16 = 0;
static mut sig_ss: u16 = 0;
static mut sig_trapped: sig_atomic_t = 0;
static mut sig_err: sig_atomic_t = 0;
static mut sig_trapno: sig_atomic_t = 0;
#[cfg(target_arch = "x86_64")]
static mut sig_corrupt_final_ss: sig_atomic_t = 0;

/* Abstractions for some 32-bit vs 64-bit differences. */
#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct selectors {
    cs: u16,
    gs: u16,
    fs: u16,
    ss: u16,
}

#[cfg(target_arch = "x86_64")]
unsafe fn ssptr(ctx: *mut ucontext_t) -> *mut u16 {
    let sels = (&mut (*ctx).uc_mcontext.gregs[REG_CSGSFS] as *mut greg_t).cast::<selectors>();
    &mut (*sels).ss
}

#[cfg(target_arch = "x86_64")]
unsafe fn csptr(ctx: *mut ucontext_t) -> *mut u16 {
    let sels = (&mut (*ctx).uc_mcontext.gregs[REG_CSGSFS] as *mut greg_t).cast::<selectors>();
    &mut (*sels).cs
}

#[cfg(target_arch = "x86")]
unsafe fn ssptr(ctx: *mut ucontext_t) -> *mut greg_t {
    &mut (*ctx).uc_mcontext.gregs[REG_SS]
}

#[cfg(target_arch = "x86")]
unsafe fn csptr(ctx: *mut ucontext_t) -> *mut greg_t {
    &mut (*ctx).uc_mcontext.gregs[REG_CS]
}

/*
 * Checks a given selector for its code bitness or returns -1 if it's not
 * a usable code segment selector.
 */
unsafe extern "C" fn cs_bitness(cs: u16) -> c_int {
    let mut valid: u32 = 0;
    let ar: u32;
    asm!(
        "lar {cs:x}, {ar:e}",
        "jnz 2f",
        "mov {valid:e}, 1",
        "2:",
        cs = in(reg) cs,
        ar = lateout(reg) ar,
        valid = inout(reg) valid,
        options(nostack),
    );

    if valid == 0 {
        return -1;
    }

    let db = (ar & (1 << 22)) != 0;
    let l = (ar & (1 << 21)) != 0;

    if (ar & (1 << 11)) == 0 {
        return -1; /* Not code. */
    }

    if l && !db {
        64
    } else if !l && db {
        32
    } else if !l && !db {
        16
    } else {
        -1 /* Unknown bitness. */
    }
}

/*
 * Checks a given selector for its code bitness or returns -1 if it's not
 * a usable code segment selector.
 */
unsafe extern "C" fn is_valid_ss(cs: u16) -> bool {
    let mut valid: u32 = 0;
    let ar: u32;
    asm!(
        "lar {cs:x}, {ar:e}",
        "jnz 2f",
        "mov {valid:e}, 1",
        "2:",
        cs = in(reg) cs,
        ar = lateout(reg) ar,
        valid = inout(reg) valid,
        options(nostack),
    );

    if valid == 0 {
        return false;
    }

    if (ar & AR_TYPE_MASK) != AR_TYPE_RWDATA && (ar & AR_TYPE_MASK) != AR_TYPE_RWDATA_EXPDOWN {
        return false;
    }

    (ar & AR_P) != 0
}

/* Number of errors in the current test case. */
static mut nerrs: sig_atomic_t = 0;

unsafe fn validate_signal_ss(sig: c_int, ctx: *mut ucontext_t) {
    #[cfg(target_arch = "x86_64")]
    {
        let was_64bit = cs_bitness(*csptr(ctx)) == 64;

        if ((*ctx).uc_flags & UC_SIGCONTEXT_SS) == 0 {
            printf(c"[FAIL]\tUC_SIGCONTEXT_SS was not set\n".as_ptr());
            nerrs += 1;

            /*
             * This happens on Linux 4.1.  The rest will fail, too, so
             * return now to reduce the noise.
             */
            return;
        }

        /* UC_STRICT_RESTORE_SS is set iff we came from 64-bit mode. */
        if (((*ctx).uc_flags & UC_STRICT_RESTORE_SS) != 0) != was_64bit {
            printf(c"[FAIL]\tUC_STRICT_RESTORE_SS was wrong in signal %d\n".as_ptr(), sig);
            nerrs += 1;
        }

        if is_valid_ss(*ssptr(ctx)) {
            /*
             * DOSEMU was written before 64-bit sigcontext had SS, and
             * it tries to figure out the signal source SS by looking at
             * the physical register.  Make sure that keeps working.
             */
            let hw_ss: u16;
            asm!("mov {0:x}, ss", out(reg) hw_ss, options(nostack));
            if hw_ss != *ssptr(ctx) {
                printf(c"[FAIL]\tHW SS didn't match saved SS\n".as_ptr());
                nerrs += 1;
            }
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = (sig, ctx);
    }
}

/*
 * SIGUSR1 handler.  Sets CS and SS as requested and points IP to the
 * int3 trampoline.  Sets SP to a large known value so that we can see
 * whether the value round-trips back to user mode correctly.
 */
unsafe extern "C" fn sigusr1(sig: c_int, _info: *mut siginfo_t, ctx_void: *mut c_void) {
    let ctx = ctx_void.cast::<ucontext_t>();

    validate_signal_ss(sig, ctx);

    memcpy(
        (&raw mut initial_regs).cast::<c_void>(),
        (&raw const (*ctx).uc_mcontext.gregs).cast::<c_void>(),
        size_of::<gregset_t>(),
    );

    *csptr(ctx) = sig_cs as _;
    *ssptr(ctx) = sig_ss as _;

    (*ctx).uc_mcontext.gregs[REG_IP] = if sig_cs == code16_sel {
        0
    } else {
        &raw const int3 as c_ulong as greg_t
    };
    (*ctx).uc_mcontext.gregs[REG_SP] = 0x8bad_f00d_5aad_c0de_u64 as greg_t;
    (*ctx).uc_mcontext.gregs[REG_CX] = 0;

    #[cfg(target_arch = "x86")]
    {
        /*
         * Make sure the kernel doesn't inadvertently use DS or ES-relative
         * accesses in a region where user DS or ES is loaded.
         *
         * Skip this for 64-bit builds because long mode doesn't care about
         * DS and ES and skipping it increases test coverage a little bit,
         * since 64-bit kernels can still run the 32-bit build.
         */
        (*ctx).uc_mcontext.gregs[REG_DS] = 0;
        (*ctx).uc_mcontext.gregs[REG_ES] = 0;
    }

    memcpy(
        (&raw mut requested_regs).cast::<c_void>(),
        (&raw const (*ctx).uc_mcontext.gregs).cast::<c_void>(),
        size_of::<gregset_t>(),
    );
    requested_regs[REG_CX] = *ssptr(ctx) as greg_t; /* The asm code does this. */
}

/*
 * Called after a successful sigreturn (via int3) or from a failed
 * sigreturn (directly by kernel).  Restores our state so that the
 * original raise(SIGUSR1) returns.
 */
unsafe extern "C" fn sigtrap(sig: c_int, _info: *mut siginfo_t, ctx_void: *mut c_void) {
    let ctx = ctx_void.cast::<ucontext_t>();

    validate_signal_ss(sig, ctx);

    sig_err = (*ctx).uc_mcontext.gregs[REG_ERR] as sig_atomic_t;
    sig_trapno = (*ctx).uc_mcontext.gregs[REG_TRAPNO] as sig_atomic_t;

    let ss: u16;
    asm!("mov {0:x}, ss", out(reg) ss, options(nostack));

    let asm_ss = (*ctx).uc_mcontext.gregs[REG_CX];
    if asm_ss != sig_ss as greg_t && sig == SIGTRAP {
        /* Sanity check failure. */
        printf(
            c"[FAIL]\tSIGTRAP: ss = %hx, frame ss = %x, ax = %llx\n".as_ptr(),
            ss as c_int,
            *ssptr(ctx) as c_int,
            asm_ss as u64,
        );
        nerrs += 1;
    }

    memcpy(
        (&raw mut resulting_regs).cast::<c_void>(),
        (&raw const (*ctx).uc_mcontext.gregs).cast::<c_void>(),
        size_of::<gregset_t>(),
    );
    memcpy(
        (&raw mut (*ctx).uc_mcontext.gregs).cast::<c_void>(),
        (&raw const initial_regs).cast::<c_void>(),
        size_of::<gregset_t>(),
    );

    #[cfg(target_arch = "x86_64")]
    {
        if sig_corrupt_final_ss != 0 {
            if ((*ctx).uc_flags & UC_STRICT_RESTORE_SS) != 0 {
                printf(c"[FAIL]\tUC_STRICT_RESTORE_SS was set inappropriately\n".as_ptr());
                nerrs += 1;
            } else {
                /*
                 * DOSEMU transitions from 32-bit to 64-bit mode by
                 * adjusting sigcontext, and it requires that this work
                 * even if the saved SS is bogus.
                 */
                printf(c"\tCorrupting SS on return to 64-bit mode\n".as_ptr());
                *ssptr(ctx) = 0;
            }
        }
    }

    sig_trapped = sig;
}

#[cfg(target_arch = "x86_64")]
/* Tests recovery if !UC_STRICT_RESTORE_SS */
unsafe extern "C" fn sigusr2(_sig: c_int, _info: *mut siginfo_t, ctx_void: *mut c_void) {
    let ctx = ctx_void.cast::<ucontext_t>();

    if ((*ctx).uc_flags & UC_STRICT_RESTORE_SS) == 0 {
        printf(c"[FAIL]\traise(2) didn't set UC_STRICT_RESTORE_SS\n".as_ptr());
        nerrs += 1;
        return; /* We can't do the rest. */
    }

    (*ctx).uc_flags &= !UC_STRICT_RESTORE_SS;
    *ssptr(ctx) = 0;

    /* Return.  The kernel should recover without sending another signal. */
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_nonstrict_ss() -> c_int {
    clearhandler(SIGUSR1);
    clearhandler(SIGTRAP);
    clearhandler(SIGSEGV);
    clearhandler(SIGILL);
    sethandler(SIGUSR2, sigusr2, 0);

    nerrs = 0;

    printf(c"[RUN]\tClear UC_STRICT_RESTORE_SS and corrupt SS\n".as_ptr());
    raise(SIGUSR2);
    if nerrs == 0 {
        printf(c"[OK]\tIt worked\n".as_ptr());
    }

    nerrs
}

/* Finds a usable code segment of the requested bitness. */
unsafe extern "C" fn find_cs(bitness: c_int) -> c_int {
    let my_cs: u16;

    asm!("mov {0:x}, cs", out(reg) my_cs, options(nostack));

    if cs_bitness(my_cs) == bitness {
        return my_cs as c_int;
    }
    if cs_bitness(my_cs.wrapping_add(2 << 3)) == bitness {
        return my_cs.wrapping_add(2 << 3) as c_int;
    }
    if my_cs > (2 << 3) && cs_bitness(my_cs.wrapping_sub(2 << 3)) == bitness {
        return my_cs.wrapping_sub(2 << 3) as c_int;
    }
    if cs_bitness(code16_sel) == bitness {
        return code16_sel as c_int;
    }

    printf(c"[WARN]\tCould not find %d-bit CS\n".as_ptr(), bitness);
    -1
}

unsafe fn test_valid_sigreturn(cs_bits: c_int, use_16bit_ss: bool, force_ss: c_int) -> c_int {
    let cs = find_cs(cs_bits);
    if cs == -1 {
        printf(
            c"[SKIP]\tCode segment unavailable for %d-bit CS, %d-bit SS\n".as_ptr(),
            cs_bits,
            if use_16bit_ss { 16 } else { 32 },
        );
        return 0;
    }

    if force_ss != -1 {
        sig_ss = force_ss as u16;
    } else if use_16bit_ss {
        if data16_sel == 0 {
            printf(
                c"[SKIP]\tData segment unavailable for %d-bit CS, 16-bit SS\n".as_ptr(),
                cs_bits,
            );
            return 0;
        }
        sig_ss = data16_sel;
    } else {
        asm!("mov {0:x}, ss", out(reg) sig_ss, options(nostack));
    }

    sig_cs = cs as u16;

    printf(
        c"[RUN]\tValid sigreturn: %d-bit CS (%hx), %d-bit SS (%hx%s)\n".as_ptr(),
        cs_bits,
        sig_cs as c_int,
        if use_16bit_ss { 16 } else { 32 },
        sig_ss as c_int,
        if (sig_ss & 4) != 0 { c"".as_ptr() } else { c", GDT".as_ptr() },
    );

    raise(SIGUSR1);

    nerrs = 0;

    /*
     * Check that each register had an acceptable value when the
     * int3 trampoline was invoked.
     */
    for i in 0..NGREG as usize {
        let req = requested_regs[i];
        let res = resulting_regs[i];

        if i == REG_TRAPNO || i == REG_IP {
            continue; /* don't care */
        }

        if i == REG_SP {
            /*
             * If we were using a 16-bit stack segment, then
             * the kernel is a bit stuck: IRET only restores
             * the low 16 bits of ESP/RSP if SS is 16-bit.
             * The kernel uses a hack to restore bits 31:16,
             * but that hack doesn't help with bits 63:32.
             * On Intel CPUs, bits 63:32 end up zeroed, and, on
             * AMD CPUs, they leak the high bits of the kernel
             * espfix64 stack pointer.  There's very little that
             * the kernel can do about it.
             *
             * Similarly, if we are returning to a 32-bit context,
             * the CPU will often lose the high 32 bits of RSP.
             */

            if res == req {
                continue;
            }

            if cs_bits != 64 && (((res ^ req) as u64) & 0xffff_ffff) == 0 {
                printf(c"[NOTE]\tSP: %llx -> %llx\n".as_ptr(), req as u64, res as u64);
                continue;
            }

            printf(
                c"[FAIL]\tSP mismatch: requested 0x%llx; got 0x%llx\n".as_ptr(),
                requested_regs[i] as u64,
                resulting_regs[i] as u64,
            );
            nerrs += 1;
            continue;
        }

        let mut ignore_reg = false;
        #[cfg(target_arch = "x86")]
        {
            if i == REG_UESP {
                ignore_reg = true;
            }
        }
        #[cfg(not(target_arch = "x86"))]
        {
            if i == REG_CSGSFS {
                let req_sels = (&raw const requested_regs[REG_CSGSFS]).cast::<selectors>();
                let res_sels = (&raw const resulting_regs[REG_CSGSFS]).cast::<selectors>();
                if (*req_sels).cs != (*res_sels).cs {
                    printf(
                        c"[FAIL]\tCS mismatch: requested 0x%hx; got 0x%hx\n".as_ptr(),
                        (*req_sels).cs as c_int,
                        (*res_sels).cs as c_int,
                    );
                    nerrs += 1;
                }

                if (*req_sels).ss != (*res_sels).ss {
                    printf(
                        c"[FAIL]\tSS mismatch: requested 0x%hx; got 0x%hx\n".as_ptr(),
                        (*req_sels).ss as c_int,
                        (*res_sels).ss as c_int,
                    );
                    nerrs += 1;
                }

                continue;
            }
        }

        /* Sanity check on the kernel */
        if i == REG_CX && req != res {
            printf(
                c"[FAIL]\tCX (saved SP) mismatch: requested 0x%llx; got 0x%llx\n".as_ptr(),
                req as u64,
                res as u64,
            );
            nerrs += 1;
            continue;
        }

        if req != res && !ignore_reg {
            printf(
                c"[FAIL]\tReg %d mismatch: requested 0x%llx; got 0x%llx\n".as_ptr(),
                i as c_int,
                req as u64,
                res as u64,
            );
            nerrs += 1;
        }
    }

    if nerrs == 0 {
        printf(c"[OK]\tall registers okay\n".as_ptr());
    }

    nerrs
}

unsafe fn test_bad_iret(cs_bits: c_int, ss: u16, force_cs: c_int) -> c_int {
    let cs = if force_cs == -1 { find_cs(cs_bits) } else { force_cs };
    if cs == -1 {
        return 0;
    }

    sig_cs = cs as u16;
    sig_ss = ss;

    printf(
        c"[RUN]\t%d-bit CS (%hx), bogus SS (%hx)\n".as_ptr(),
        cs_bits,
        sig_cs as c_int,
        sig_ss as c_int,
    );

    sig_trapped = 0;
    raise(SIGUSR1);
    if sig_trapped != 0 {
        let mut errdesc: [c_char; 32] = [0; 32];
        if sig_err != 0 {
            let src = if (sig_err & 1) != 0 { c" EXT".as_ptr() } else { c"".as_ptr() };
            let table = if (sig_err & 0x6) == 0x0 {
                c"GDT".as_ptr()
            } else if (sig_err & 0x6) == 0x4 {
                c"LDT".as_ptr()
            } else if (sig_err & 0x6) == 0x2 {
                c"IDT".as_ptr()
            } else {
                c"???".as_ptr()
            };

            sprintf(
                errdesc.as_mut_ptr(),
                c"%s%s index %d, ".as_ptr(),
                table,
                src,
                sig_err >> 3,
            );
        }

        let mut trapname: [c_char; 32] = [0; 32];
        if sig_trapno == 13 {
            strcpy(trapname.as_mut_ptr(), c"GP".as_ptr());
        } else if sig_trapno == 11 {
            strcpy(trapname.as_mut_ptr(), c"NP".as_ptr());
        } else if sig_trapno == 12 {
            strcpy(trapname.as_mut_ptr(), c"SS".as_ptr());
        } else if sig_trapno == 32 {
            strcpy(trapname.as_mut_ptr(), c"IRET".as_ptr()); /* X86_TRAP_IRET */
        } else {
            sprintf(trapname.as_mut_ptr(), c"%d".as_ptr(), sig_trapno);
        }

        printf(
            c"[OK]\tGot #%s(0x%lx) (i.e. %s%s)\n".as_ptr(),
            trapname.as_ptr(),
            sig_err as c_ulong,
            errdesc.as_ptr(),
            strsignal(sig_trapped),
        );
        0
    } else {
        /*
         * This also implicitly tests UC_STRICT_RESTORE_SS:
         * We check that these signals set UC_STRICT_RESTORE_SS and,
         * if UC_STRICT_RESTORE_SS doesn't cause strict behavior,
         * then we won't get SIGSEGV.
         */
        printf(c"[FAIL]\tDid not get SIGSEGV\n".as_ptr());
        1
    }
}

unsafe fn c_main() -> c_int {
    let mut total_nerrs: c_int = 0;
    let my_cs: u16;
    let my_ss: u16;

    asm!("mov {0:x}, cs", out(reg) my_cs, options(nostack));
    asm!("mov {0:x}, ss", out(reg) my_ss, options(nostack));
    setup_ldt();

    let stack = stack_t {
        /* Our sigaltstack scratch space. */
        ss_sp: malloc(size_of::<c_char>() * SIGSTKSZ),
        ss_flags: 0,
        ss_size: SIGSTKSZ,
    };
    if sigaltstack(&stack, ptr::null_mut()) != 0 {
        err(1, c"sigaltstack".as_ptr());
    }

    sethandler(SIGUSR1, sigusr1, 0);
    sethandler(SIGTRAP, sigtrap, SA_ONSTACK);

    /* Easy cases: return to a 32-bit SS in each possible CS bitness. */
    total_nerrs += test_valid_sigreturn(64, false, -1);
    total_nerrs += test_valid_sigreturn(32, false, -1);
    total_nerrs += test_valid_sigreturn(16, false, -1);

    /*
     * Test easy espfix cases: return to a 16-bit LDT SS in each possible
     * CS bitness.  NB: with a long mode CS, the SS bitness is irrelevant.
     *
     * This catches the original missing-espfix-on-64-bit-kernels issue
     * as well as CVE-2014-8134.
     */
    total_nerrs += test_valid_sigreturn(64, true, -1);
    total_nerrs += test_valid_sigreturn(32, true, -1);
    total_nerrs += test_valid_sigreturn(16, true, -1);

    if gdt_data16_idx != 0 {
        /*
         * For performance reasons, Linux skips espfix if SS points
         * to the GDT.  If we were able to allocate a 16-bit SS in
         * the GDT, see if it leaks parts of the kernel stack pointer.
         *
         * This tests for CVE-2014-8133.
         */
        total_nerrs += test_valid_sigreturn(64, true, GDT3(gdt_data16_idx as c_int) as c_int);
        total_nerrs += test_valid_sigreturn(32, true, GDT3(gdt_data16_idx as c_int) as c_int);
        total_nerrs += test_valid_sigreturn(16, true, GDT3(gdt_data16_idx as c_int) as c_int);
    }

    #[cfg(target_arch = "x86_64")]
    {
        /* Nasty ABI case: check SS corruption handling. */
        sig_corrupt_final_ss = 1;
        total_nerrs += test_valid_sigreturn(32, false, -1);
        total_nerrs += test_valid_sigreturn(32, true, -1);
        sig_corrupt_final_ss = 0;
    }

    /*
     * We're done testing valid sigreturn cases.  Now we test states
     * for which sigreturn itself will succeed but the subsequent
     * entry to user mode will fail.
     *
     * Depending on the failure mode and the kernel bitness, these
     * entry failures can generate SIGSEGV, SIGBUS, or SIGILL.
     */
    clearhandler(SIGTRAP);
    sethandler(SIGSEGV, sigtrap, SA_ONSTACK);
    sethandler(SIGBUS, sigtrap, SA_ONSTACK);
    sethandler(SIGILL, sigtrap, SA_ONSTACK); /* 32-bit kernels do this */

    /* Easy failures: invalid SS, resulting in #GP(0) */
    test_bad_iret(64, ldt_nonexistent_sel, -1);
    test_bad_iret(32, ldt_nonexistent_sel, -1);
    test_bad_iret(16, ldt_nonexistent_sel, -1);

    /* These fail because SS isn't a data segment, resulting in #GP(SS) */
    test_bad_iret(64, my_cs, -1);
    test_bad_iret(32, my_cs, -1);
    test_bad_iret(16, my_cs, -1);

    /* Try to return to a not-present code segment, triggering #NP(SS). */
    test_bad_iret(32, my_ss, npcode32_sel as c_int);

    /*
     * Try to return to a not-present but otherwise valid data segment.
     * This will cause IRET to fail with #SS on the espfix stack.  This
     * exercises CVE-2014-9322.
     *
     * Note that, if espfix is enabled, 64-bit Linux will lose track
     * of the actual cause of failure and report #GP(0) instead.
     * This would be very difficult for Linux to avoid, because
     * espfix64 causes IRET failures to be promoted to #DF, so the
     * original exception frame is never pushed onto the stack.
     */
    test_bad_iret(32, npdata32_sel, -1);

    /*
     * Try to return to a not-present but otherwise valid data
     * segment without invoking espfix.  Newer kernels don't allow
     * this to happen in the first place.  On older kernels, though,
     * this can trigger CVE-2014-9322.
     */
    if gdt_npdata32_idx != 0 {
        test_bad_iret(32, GDT3(gdt_npdata32_idx as c_int), -1);
    }

    #[cfg(target_arch = "x86_64")]
    {
        total_nerrs += test_nonstrict_ss();
    }

    free(stack.ss_sp);
    if total_nerrs != 0 { 1 } else { 0 }
}

fn main() {
    unsafe {
        std::process::exit(c_main());
    }
}
