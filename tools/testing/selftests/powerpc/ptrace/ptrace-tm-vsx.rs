// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Ptrace test for VMX/VSX registers in the TM context
 *
 * Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
 */

// C dependencies: "ptrace.h", "tm.h", "ptrace-vsx.h".

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem;
use core::ptr;

type pid_t = c_int;

const VEC_MAX: usize = 128;
const VSX_MAX: usize = 64;
const VMX_MAX: usize = 32;
const SPRN_TEXASR: c_int = 0x82;
const IPC_PRIVATE: c_int = 0;
const IPC_CREAT: c_int = 0o1000;
const IPC_RMID: c_int = 0;
const SIGKILL: c_int = 9;
const TEST_PASS: c_int = 0;
const TEST_FAIL: c_int = 1;

static mut shm_id: c_int = 0;
static mut cptr: *mut c_ulong = ptr::null_mut();
static mut pptr: *mut c_ulong = ptr::null_mut();

static mut fp_load: [c_ulong; VEC_MAX] = [0; VEC_MAX];
static mut fp_store: [c_ulong; VEC_MAX] = [0; VEC_MAX];
static mut fp_load_ckpt: [c_ulong; VEC_MAX] = [0; VEC_MAX];
static mut fp_load_ckpt_new: [c_ulong; VEC_MAX] = [0; VEC_MAX];

unsafe extern "C" {
    fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void;
    fn shmdt(shmaddr: *const c_void) -> c_int;
    fn shmget(key: c_int, size: usize, shmflg: c_int) -> c_int;
    fn shmctl(shmid: c_int, cmd: c_int, buf: *mut c_void) -> c_int;
    fn fork() -> pid_t;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn wait(status: *mut c_int) -> pid_t;
    fn printf(format: *const c_char, ...) -> c_int;
    fn rand() -> c_int;

    fn loadvsx(ptr: *mut c_ulong, offset: c_int);
    fn storevsx(ptr: *mut c_ulong, offset: c_int);
    fn compare_vsx_vmx(vsx: *mut c_ulong, vmx: *mut c_ulong) -> c_int;
    fn start_trace(child: pid_t) -> c_int;
    fn stop_trace(child: pid_t) -> c_int;
    fn show_vsx(child: pid_t, vsx: *mut c_ulong) -> c_int;
    fn validate_vsx(vsx: *mut c_ulong, expected: *mut c_ulong) -> c_int;
    fn show_vmx(child: pid_t, vmx: *mut [[c_ulong; 2]; VMX_MAX + 2]) -> c_int;
    fn validate_vmx(vmx: *mut [[c_ulong; 2]; VMX_MAX + 2], expected: *mut c_ulong) -> c_int;
    fn show_vsx_ckpt(child: pid_t, vsx: *mut c_ulong) -> c_int;
    fn show_vmx_ckpt(child: pid_t, vmx: *mut [[c_ulong; 2]; VMX_MAX + 2]) -> c_int;
    fn load_vsx_vmx(
        load: *mut c_ulong,
        vsx: *mut c_ulong,
        vmx: *mut [[c_ulong; 2]; VMX_MAX + 2],
    );
    fn write_vsx_ckpt(child: pid_t, vsx: *mut c_ulong) -> c_int;
    fn write_vmx_ckpt(child: pid_t, vmx: *mut [[c_ulong; 2]; VMX_MAX + 2]) -> c_int;
    fn have_htm() -> c_int;
    fn htm_is_synthetic() -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

unsafe fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

#[used]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_vsx() {
    unsafe {
        loadvsx((&raw mut fp_load).cast::<c_ulong>(), 0);
    }
}

#[used]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_vsx_ckpt() {
    unsafe {
        loadvsx((&raw mut fp_load_ckpt).cast::<c_ulong>(), 0);
    }
}

unsafe extern "C" fn tm_vsx() {
    let mut result: c_ulong;
    let mut texasr: c_ulong;
    let ret: c_int;

    unsafe {
        cptr = shmat(shm_id, ptr::null(), 0).cast::<c_ulong>();
    }

    loop {
        unsafe {
            *cptr.add(1) = 0;
            asm!(
                "bl load_vsx_ckpt",
                "1:",
                "tbegin.",
                "beq 2f",
                "bl load_vsx",
                "tsuspend.",
                "li 7, 1",
                "stw 7, 0({cptr1})",
                "tresume.",
                "b .",
                "tend.",
                "li 0, 0",
                "ori {res}, 0, 0",
                "b 3f",
                "2:",
                "li 0, 1",
                "ori {res}, 0, 0",
                "mfspr {texasr}, {sprn_texasr}",
                "3:",
                res = lateout(reg) result,
                texasr = lateout(reg) texasr,
                sprn_texasr = const SPRN_TEXASR,
                cptr1 = in(reg) cptr.add(1),
                out("r0") _,
                out("r3") _,
                out("r4") _,
                out("r7") _,
                out("r8") _,
                out("r9") _,
                out("r10") _,
                out("r11") _,
                out("lr") _,
            );

            if result != 0 {
                if *cptr.add(0) == 0 {
                    continue;
                }

                shmdt(cptr.cast::<c_void>());
                storevsx((&raw mut fp_store).cast::<c_ulong>(), 0);
                ret = compare_vsx_vmx(
                    (&raw mut fp_store).cast::<c_ulong>(),
                    (&raw mut fp_load_ckpt_new).cast::<c_ulong>(),
                );
                if ret != 0 {
                    exit(1);
                }
                exit(0);
            }
            shmdt(cptr.cast::<c_void>());
            exit(1);
        }
    }
}

unsafe extern "C" fn trace_tm_vsx(child: pid_t) -> c_int {
    let mut vsx: [c_ulong; VSX_MAX] = [0; VSX_MAX];
    let mut vmx: [[c_ulong; 2]; VMX_MAX + 2] = [[0; 2]; VMX_MAX + 2];

    unsafe {
        if start_trace(child) != 0 {
            return TEST_FAIL;
        }
        if show_vsx(child, vsx.as_mut_ptr()) != 0 {
            return TEST_FAIL;
        }
        if validate_vsx(vsx.as_mut_ptr(), (&raw mut fp_load).cast::<c_ulong>()) != 0 {
            return TEST_FAIL;
        }
        if show_vmx(child, &mut vmx) != 0 {
            return TEST_FAIL;
        }
        if validate_vmx(&mut vmx, (&raw mut fp_load).cast::<c_ulong>()) != 0 {
            return TEST_FAIL;
        }
        if show_vsx_ckpt(child, vsx.as_mut_ptr()) != 0 {
            return TEST_FAIL;
        }
        if validate_vsx(vsx.as_mut_ptr(), (&raw mut fp_load_ckpt).cast::<c_ulong>()) != 0 {
            return TEST_FAIL;
        }
        if show_vmx_ckpt(child, &mut vmx) != 0 {
            return TEST_FAIL;
        }
        if validate_vmx(&mut vmx, (&raw mut fp_load_ckpt).cast::<c_ulong>()) != 0 {
            return TEST_FAIL;
        }

        vsx = [0; VSX_MAX];
        vmx = [[0; 2]; VMX_MAX + 2];

        load_vsx_vmx(
            (&raw mut fp_load_ckpt_new).cast::<c_ulong>(),
            vsx.as_mut_ptr(),
            &mut vmx,
        );

        if write_vsx_ckpt(child, vsx.as_mut_ptr()) != 0 {
            return TEST_FAIL;
        }
        if write_vmx_ckpt(child, &mut vmx) != 0 {
            return TEST_FAIL;
        }
        *pptr.add(0) = 1;
        if stop_trace(child) != 0 {
            return TEST_FAIL;
        }
    }

    TEST_PASS
}

unsafe extern "C" fn ptrace_tm_vsx() -> c_int {
    let pid: pid_t;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let mut i: c_int;

    unsafe {
        if have_htm() == 0 {
            return TEST_PASS;
        }
        if htm_is_synthetic() != 0 {
            return TEST_PASS;
        }
        shm_id = shmget(IPC_PRIVATE, mem::size_of::<c_int>() * 2, 0o777 | IPC_CREAT);

        i = 0;
        while i < 128 {
            fp_load[i as usize] = 1u64.wrapping_add(rand() as c_ulong);
            fp_load_ckpt[i as usize] = 1u64.wrapping_add(2u64.wrapping_mul(rand() as c_ulong));
            fp_load_ckpt_new[i as usize] =
                1u64.wrapping_add(3u64.wrapping_mul(rand() as c_ulong));
            i += 1;
        }

        pid = fork();
        if pid < 0 {
            perror(c"fork() failed".as_ptr());
            return TEST_FAIL;
        }

        if pid == 0 {
            tm_vsx();
        }

        if pid != 0 {
            pptr = shmat(shm_id, ptr::null(), 0).cast::<c_ulong>();
            while *pptr.add(1) == 0 {
                asm!("", options(nostack, preserves_flags));
            }

            ret = trace_tm_vsx(pid);
            if ret != 0 {
                kill(pid, SIGKILL);
                shmdt(pptr.cast::<c_void>());
                shmctl(shm_id, IPC_RMID, ptr::null_mut());
                return TEST_FAIL;
            }

            shmdt(pptr.cast::<c_void>());
            ret = wait(&mut status);
            shmctl(shm_id, IPC_RMID, ptr::null_mut());
            if ret != pid {
                printf(c"Child's exit status not captured\n".as_ptr());
                return TEST_FAIL;
            }

            return if wifexited(status) && wexitstatus(status) != 0 {
                TEST_FAIL
            } else {
                TEST_PASS
            };
        }
    }

    TEST_PASS
}

fn main() {
    unsafe {
        core::process::exit(test_harness(ptrace_tm_vsx, c"ptrace_tm_vsx".as_ptr()));
    }
}
