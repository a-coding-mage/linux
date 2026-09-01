// SPDX-License-Identifier: GPL-2.0-only
/*
 * fsgsbase_restore.c, test ptrace vs fsgsbase
 * Copyright (c) 2020 Andy Lutomirski
 *
 * This test case simulates a tracer redirecting tracee execution to
 * a function and then restoring tracee state using PTRACE_GETREGS and
 * PTRACE_SETREGS.  This is similar to what gdb does when doing
 * 'p func()'.  The catch is that this test has the called function
 * modify a segment register.  This makes sure that ptrace correctly
 * restores segment state when using PTRACE_SETREGS.
 *
 * This is not part of fsgsbase.c, because that test is 64-bit only.
 */

use core::arch::asm;
use core::ffi::{c_long, c_void};
use core::mem;
use core::ptr;

const EXPECTED_VALUE: u32 = 0x1337f00d;

/*
 * Defined in clang_helpers_[32|64].S, because unlike gcc, clang inline asm does
 * not support segmentation prefixes.
 */
unsafe extern "C" {
    fn dereference_seg_base() -> u32;
}

fn err(status: i32, msg: &str) -> ! {
    eprintln!("{}: {}", msg, std::io::Error::last_os_error());
    std::process::exit(status);
}

unsafe fn init_seg() {
    let target = unsafe {
        libc::mmap(
            ptr::null_mut(),
            mem::size_of::<u32>(),
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_32BIT,
            -1,
            0,
        )
    } as *mut u32;
    if target == libc::MAP_FAILED as *mut u32 {
        err(1, "mmap");
    }

    unsafe {
        *target = EXPECTED_VALUE;
    }

    println!("\tsegment base address = 0x{:x}", target as libc::c_ulong);

    let mut desc: libc::user_desc = unsafe { mem::zeroed() };
    desc.entry_number = 0;
    desc.base_addr = target as usize as u32;
    desc.limit = mem::size_of::<u32>() as u32 - 1;
    desc.seg_32bit = 1;
    desc.contents = 0; /* Data, grow-up */
    desc.read_exec_only = 0;
    desc.limit_in_pages = 0;
    desc.seg_not_present = 0;
    desc.useable = 0;

    if unsafe {
        libc::syscall(
            libc::SYS_modify_ldt,
            1,
            &desc as *const libc::user_desc,
            mem::size_of_val(&desc),
        )
    } == 0
    {
        println!("\tusing LDT slot 0");
        #[cfg(target_arch = "x86_64")]
        unsafe {
            asm!("mov {0:x}, gs", in(reg) 0x7_u16, options(att_syntax));
        }
        #[cfg(target_arch = "x86")]
        unsafe {
            asm!("mov {0:x}, fs", in(reg) 0x7_u16, options(att_syntax));
        }
    } else {
        /* No modify_ldt for us (configured out, perhaps) */

        let low_desc = unsafe {
            libc::mmap(
                ptr::null_mut(),
                mem::size_of_val(&desc),
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_32BIT,
                -1,
                0,
            )
        } as *mut libc::user_desc;
        unsafe {
            ptr::copy_nonoverlapping(&desc, low_desc, 1);
        }

        unsafe {
            (*low_desc).entry_number = -1_i32 as u32;
        }

        /* 32-bit set_thread_area */
        let ret: c_long;
        #[cfg(target_arch = "x86_64")]
        unsafe {
            asm!(
                "int $0x80",
                lateout("rax") ret,
                in("rax") 243_c_long,
                in("rbx") low_desc,
                inout("m") *low_desc => _,
                lateout("r8") _,
                lateout("r9") _,
                lateout("r10") _,
                lateout("r11") _,
                options(att_syntax)
            );
        }
        #[cfg(target_arch = "x86")]
        unsafe {
            asm!(
                "int $0x80",
                lateout("eax") ret,
                in("eax") 243_c_long,
                in("ebx") low_desc,
                inout("m") *low_desc => _,
                options(att_syntax)
            );
        }
        unsafe {
            ptr::copy_nonoverlapping(low_desc, &mut desc, 1);
            libc::munmap(low_desc as *mut c_void, mem::size_of_val(&desc));
        }

        if ret != 0 {
            println!("[NOTE]\tcould not create a segment -- can't test anything");
            std::process::exit(0);
        }
        println!("\tusing GDT slot {}", desc.entry_number);

        let sel: u16 = ((desc.entry_number << 3) | 0x3) as u16;
        #[cfg(target_arch = "x86_64")]
        unsafe {
            asm!("mov {0:x}, gs", in(reg) sel, options(att_syntax));
        }
        #[cfg(target_arch = "x86")]
        unsafe {
            asm!("mov {0:x}, fs", in(reg) sel, options(att_syntax));
        }
    }
}

unsafe extern "C" fn tracee_zap_segment() {
    /*
     * The tracer will redirect execution here.  This is meant to
     * work like gdb's 'p func()' feature.  The tricky bit is that
     * we modify a segment register in order to make sure that ptrace
     * can correctly restore segment registers.
     */
    println!("\tTracee: in tracee_zap_segment()");

    /*
     * Write a nonzero selector with base zero to the segment register.
     * Using a null selector would defeat the test on AMD pre-Zen2
     * CPUs, as such CPUs don't clear the base when loading a null
     * selector.
     */
    let sel: u16;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!(
            "mov %ss, {0:x}",
            "mov {0:x}, %gs",
            lateout(reg) sel,
            options(att_syntax)
        );
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        asm!(
            "mov %ss, {0:x}",
            "mov {0:x}, %fs",
            lateout(reg) sel,
            options(att_syntax)
        );
    }

    let pid: libc::pid_t = unsafe { libc::getpid() };
    let tid: libc::pid_t = unsafe { libc::syscall(libc::SYS_gettid) as libc::pid_t };

    println!("\tTracee is going back to sleep");
    unsafe {
        libc::syscall(libc::SYS_tgkill, pid, tid, libc::SIGSTOP);
    }

    /* Should not get here. */
    loop {
        println!("[FAIL]\tTracee hit unreachable code");
        unsafe {
            libc::pause();
        }
    }
}

fn main() {
    println!("\tSetting up a segment");
    unsafe {
        init_seg();
    }

    let mut val = unsafe { dereference_seg_base() };
    if val != EXPECTED_VALUE {
        println!(
            "[FAIL]\tseg[0] == {:x}; should be {:x}",
            val, EXPECTED_VALUE
        );
        std::process::exit(1);
    }
    println!("[OK]\tThe segment points to the right place.");

    let chld = unsafe { libc::fork() };
    if chld < 0 {
        err(1, "fork");
    }

    if chld == 0 {
        unsafe {
            libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL, 0, 0, 0, 0);
        }

        if unsafe {
            libc::ptrace(
                libc::PTRACE_TRACEME,
                0,
                ptr::null_mut::<c_void>(),
                ptr::null_mut::<c_void>(),
            )
        } != 0
        {
            err(1, "PTRACE_TRACEME");
        }

        let pid: libc::pid_t = unsafe { libc::getpid() };
        let tid: libc::pid_t = unsafe { libc::syscall(libc::SYS_gettid) as libc::pid_t };

        println!("\tTracee will take a nap until signaled");
        unsafe {
            libc::syscall(libc::SYS_tgkill, pid, tid, libc::SIGSTOP);
        }

        println!("\tTracee was resumed.  Will re-check segment.");

        val = unsafe { dereference_seg_base() };
        if val != EXPECTED_VALUE {
            println!(
                "[FAIL]\tseg[0] == {:x}; should be {:x}",
                val, EXPECTED_VALUE
            );
            std::process::exit(1);
        }

        println!("[OK]\tThe segment points to the right place.");
        std::process::exit(0);
    }

    let mut status: i32 = 0;

    /* Wait for SIGSTOP. */
    if unsafe { libc::waitpid(chld, &mut status, 0) } != chld || !libc::WIFSTOPPED(status) {
        err(1, "waitpid");
    }

    let mut regs: libc::user_regs_struct = unsafe { mem::zeroed() };

    if unsafe {
        libc::ptrace(
            libc::PTRACE_GETREGS,
            chld,
            ptr::null_mut::<c_void>(),
            &mut regs as *mut libc::user_regs_struct as *mut c_void,
        )
    } != 0
    {
        err(1, "PTRACE_GETREGS");
    }

    #[cfg(target_arch = "x86_64")]
    println!(
        "\tChild GS=0x{:x}, GSBASE=0x{:x}",
        regs.gs as libc::c_ulong,
        regs.gs_base as libc::c_ulong
    );
    #[cfg(target_arch = "x86")]
    println!("\tChild FS=0x{:x}", regs.xfs as libc::c_ulong);

    let mut regs2 = regs;
    #[cfg(target_arch = "x86_64")]
    {
        regs2.rip = tracee_zap_segment as usize as u64;
        regs2.rsp -= 128; /* Don't clobber the redzone. */
    }
    #[cfg(target_arch = "x86")]
    {
        regs2.eip = tracee_zap_segment as usize as u32;
    }

    println!("\tTracer: redirecting tracee to tracee_zap_segment()");
    if unsafe {
        libc::ptrace(
            libc::PTRACE_SETREGS,
            chld,
            ptr::null_mut::<c_void>(),
            &mut regs2 as *mut libc::user_regs_struct as *mut c_void,
        )
    } != 0
    {
        err(1, "PTRACE_GETREGS");
    }
    if unsafe {
        libc::ptrace(
            libc::PTRACE_CONT,
            chld,
            ptr::null_mut::<c_void>(),
            ptr::null_mut::<c_void>(),
        )
    } != 0
    {
        err(1, "PTRACE_GETREGS");
    }

    /* Wait for SIGSTOP. */
    if unsafe { libc::waitpid(chld, &mut status, 0) } != chld || !libc::WIFSTOPPED(status) {
        err(1, "waitpid");
    }

    println!("\tTracer: restoring tracee state");
    if unsafe {
        libc::ptrace(
            libc::PTRACE_SETREGS,
            chld,
            ptr::null_mut::<c_void>(),
            &mut regs as *mut libc::user_regs_struct as *mut c_void,
        )
    } != 0
    {
        err(1, "PTRACE_GETREGS");
    }
    if unsafe {
        libc::ptrace(
            libc::PTRACE_DETACH,
            chld,
            ptr::null_mut::<c_void>(),
            ptr::null_mut::<c_void>(),
        )
    } != 0
    {
        err(1, "PTRACE_GETREGS");
    }

    /* Wait for SIGSTOP. */
    if unsafe { libc::waitpid(chld, &mut status, 0) } != chld {
        err(1, "waitpid");
    }

    if libc::WIFSIGNALED(status) {
        println!("[FAIL]\tTracee crashed");
        std::process::exit(1);
    }

    if !libc::WIFEXITED(status) {
        println!(
            "[FAIL]\tTracee stopped for an unexpected reason: {}",
            status
        );
        std::process::exit(1);
    }

    let exitcode = libc::WEXITSTATUS(status);
    if exitcode != 0 {
        println!("[FAIL]\tTracee reported failure");
        std::process::exit(1);
    }

    println!("[OK]\tAll is well.");
    std::process::exit(0);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
