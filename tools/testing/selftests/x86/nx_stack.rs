/*
 * Copyright (c) 2023 Alexey Dobriyan <adobriyan@gmail.com>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * Test that userspace stack is NX. Requires linking with -Wl,-z,noexecstack
 * because I don't want to bother with PT_GNU_STACK detection.
 *
 * Fill the stack with INT3's and then try to execute some of them:
 * SIGSEGV -- good, SIGTRAP -- bad.
 *
 * Regular stack is completely overwritten before testing.
 * Test doesn't exit SIGSEGV handler after first fault at INT3.
 */

use core::arch::{asm, global_asm};
use core::ffi::{c_int, c_ulong, c_void};
use core::mem::zeroed;
use core::ptr::{null, null_mut, read_volatile, write_volatile};

const PAGE_SIZE: c_ulong = 4096;

/*
 * This is memset(rsp, 0xcc, -1); but down.
 * It will SIGSEGV when bottom of the stack is reached.
 * Byte-size access is important! (see rdi tweak in the signal handler).
 */
unsafe extern "C" {
    fn make_stack1();
}

#[cfg(target_arch = "x86_64")]
global_asm!(
    r#"
.pushsection .text
.globl make_stack1
.align 16
make_stack1:
	mov $0xcc, %al
	mov %rsp, %rdi
	mov $-1, %rcx
	std
	rep stosb
	/* unreachable */
	hlt
.type make_stack1,@function
.size make_stack1,.-make_stack1
.popsection
"#
);

#[cfg(target_arch = "x86")]
global_asm!(
    r#"
.pushsection .text
.globl make_stack1
.align 16
make_stack1:
	mov $0xcc, %al
	mov %esp, %edi
	mov $-1, %ecx
	std
	rep stosb
	/* unreachable */
	hlt
.type make_stack1,@function
.size make_stack1,.-make_stack1
.popsection
"#
);

/*
 * memset(p, 0xcc, -1);
 * It will SIGSEGV when top of the stack is reached.
 */
unsafe extern "C" {
    fn make_stack2(p: u64);
}

#[cfg(target_arch = "x86_64")]
global_asm!(
    r#"
.pushsection .text
.globl make_stack2
.align 16
make_stack2:
	mov $0xcc, %al
	mov $-1, %rcx
	cld
	rep stosb
	/* unreachable */
	hlt
.type make_stack2,@function
.size make_stack2,.-make_stack2
.popsection
"#
);

#[cfg(target_arch = "x86")]
global_asm!(
    r#"
.pushsection .text
.globl make_stack2
.align 16
make_stack2:
	mov $0xcc, %al
	mov $-1, %ecx
	cld
	rep stosb
	/* unreachable */
	hlt
.type make_stack2,@function
.size make_stack2,.-make_stack2
.popsection
"#
);

static mut TEST_STATE: c_int = 0;
static mut STACK_MIN_ADDR: c_ulong = 0;

#[cfg(target_arch = "x86_64")]
const RDI: usize = libc::REG_RDI as usize;
#[cfg(target_arch = "x86_64")]
const RIP: usize = libc::REG_RIP as usize;
#[cfg(target_arch = "x86_64")]
const RIP_STRING: &str = "rip";

#[cfg(target_arch = "x86")]
const RDI: usize = libc::REG_EDI as usize;
#[cfg(target_arch = "x86")]
const RIP: usize = libc::REG_EIP as usize;
#[cfg(target_arch = "x86")]
const RIP_STRING: &str = "eip";

unsafe extern "C" fn sigsegv(_: c_int, __: *mut libc::siginfo_t, uc_: *mut c_void) {
    /*
     * Some Linux versions didn't clear DF before entering signal
     * handler. make_stack1() doesn't have a chance to clear DF
     * either so we clear it by hand here.
     */
    asm!("cld", options(nostack, preserves_flags));

    let uc = uc_ as *mut libc::ucontext_t;

    if read_volatile(&raw const TEST_STATE) == 0 {
        /* Stack is faulted and cleared from RSP to the lowest address. */
        (*uc).uc_mcontext.gregs[RDI] += 1;
        write_volatile(
            &raw mut STACK_MIN_ADDR,
            (*uc).uc_mcontext.gregs[RDI] as c_ulong,
        );
        if true {
            libc::printf(
                c"stack min %lx\n".as_ptr(),
                read_volatile(&raw const STACK_MIN_ADDR),
            );
        }
        (*uc).uc_mcontext.gregs[RIP] = make_stack2 as usize as libc::greg_t;
        write_volatile(&raw mut TEST_STATE, 1);
    } else if read_volatile(&raw const TEST_STATE) == 1 {
        /* Stack has been cleared from top to bottom. */
        let stack_max_addr = (*uc).uc_mcontext.gregs[RDI] as c_ulong;
        if true {
            libc::printf(c"stack max %lx\n".as_ptr(), stack_max_addr);
        }
        /* Start faulting pages on stack and see what happens. */
        (*uc).uc_mcontext.gregs[RIP] = (stack_max_addr - PAGE_SIZE) as libc::greg_t;
        write_volatile(&raw mut TEST_STATE, 2);
    } else if read_volatile(&raw const TEST_STATE) == 2 {
        /* Stack page is NX -- good, test next page. */
        (*uc).uc_mcontext.gregs[RIP] -= PAGE_SIZE as libc::greg_t;
        if (*uc).uc_mcontext.gregs[RIP] as c_ulong == read_volatile(&raw const STACK_MIN_ADDR) {
            /* One more SIGSEGV and test ends. */
            write_volatile(&raw mut TEST_STATE, 3);
        }
    } else {
        libc::printf(c"PASS\tAll stack pages are NX\n".as_ptr());
        libc::_exit(libc::EXIT_SUCCESS);
    }
}

unsafe extern "C" fn sigtrap(_: c_int, __: *mut libc::siginfo_t, uc_: *mut c_void) {
    let uc = uc_ as *const libc::ucontext_t;
    let rip = (*uc).uc_mcontext.gregs[RIP] as c_ulong;
    libc::printf(
        c"FAIL\texecutable page on the stack: %s %lx\n".as_ptr(),
        if RIP_STRING == "rip" {
            c"rip".as_ptr()
        } else {
            c"eip".as_ptr()
        },
        rip,
    );
    libc::_exit(libc::EXIT_FAILURE);
}

fn main() {
    unsafe {
        {
            let mut act: libc::sigaction = zeroed();
            libc::sigemptyset(&mut act.sa_mask);
            act.sa_flags = libc::SA_SIGINFO;
            act.sa_sigaction = sigsegv as usize;
            let rv = libc::sigaction(libc::SIGSEGV, &act, null_mut());
            assert!(rv == 0);
        }
        {
            let mut act: libc::sigaction = zeroed();
            libc::sigemptyset(&mut act.sa_mask);
            act.sa_flags = libc::SA_SIGINFO;
            act.sa_sigaction = sigtrap as usize;
            let rv = libc::sigaction(libc::SIGTRAP, &act, null_mut());
            assert!(rv == 0);
        }
        {
            let mut rlim: libc::rlimit = zeroed();
            let mut rv = libc::getrlimit(libc::RLIMIT_STACK, &mut rlim);
            assert!(rv == 0);
            /* Cap stack at time-honored 8 MiB value. */
            rlim.rlim_max = rlim.rlim_cur;
            if rlim.rlim_max > 8 * 1024 * 1024 {
                rlim.rlim_max = 8 * 1024 * 1024;
            }
            rv = libc::setrlimit(libc::RLIMIT_STACK, &rlim);
            assert!(rv == 0);
        }
        {
            /*
             * We don't know now much stack SIGSEGV handler uses.
             * Bump this by 1 page every time someone complains,
             * or rewrite it in assembly.
             */
            let len: usize = libc::SIGSTKSZ as usize;
            let p = libc::mmap(
                null_mut(),
                len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            );
            assert!(p != libc::MAP_FAILED);
            let mut ss: libc::stack_t = zeroed();
            ss.ss_sp = p;
            ss.ss_size = len;
            let rv = libc::sigaltstack(&ss, null());
            assert!(rv == 0);
        }
        make_stack1();
        /*
         * Unreachable, but if _this_ INT3 is ever reached, it's a bug somewhere.
         * Fold it into main SIGTRAP pathway.
         */
        core::intrinsics::abort();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
