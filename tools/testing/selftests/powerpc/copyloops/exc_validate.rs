use libc::{
    c_char, c_int, c_ulong, c_void, siginfo_t, ucontext_t, MAP_ANONYMOUS, MAP_FAILED,
    MAP_PRIVATE, PROT_NONE, PROT_READ, PROT_WRITE, SA_SIGINFO, SIGSEGV,
};
use std::ffi::CStr;
use std::mem;
use std::ptr;

unsafe extern "C" {
    static mut __start___ex_table: [c_char; 0];
    static mut __stop___ex_table: [c_char; 0];

    fn COPY_LOOP(to: *mut c_void, from: *const c_void, size: c_ulong) -> c_ulong;
    fn test_copy_tofrom_user_reference(
        to: *mut c_void,
        from: *const c_void,
        size: c_ulong,
    ) -> c_ulong;
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
}

#[cfg(target_arch = "powerpc64")]
unsafe fn ucontext_nia(uc: *mut ucontext_t) -> *mut c_ulong {
    unsafe { (*uc).uc_mcontext.gp_regs.as_mut_ptr().add(libc::PT_NIP as usize) as *mut c_ulong }
}

#[cfg(target_arch = "powerpc")]
unsafe fn ucontext_nia(uc: *mut ucontext_t) -> *mut c_ulong {
    unsafe { (*(*uc).uc_mcontext.uc_regs).gregs.as_mut_ptr().add(libc::PT_NIP as usize) as *mut c_ulong }
}

#[cfg(not(any(target_arch = "powerpc64", target_arch = "powerpc")))]
compile_error!("implement UCONTEXT_NIA");

unsafe extern "C" fn segv_handler(signr: c_int, info: *mut siginfo_t, ptr: *mut c_void) {
    let _ = signr;
    let uc = ptr as *mut ucontext_t;
    let addr = unsafe { (*info).si_addr() as c_ulong };
    let ip = unsafe { ucontext_nia(uc) };
    let mut ex_p = unsafe { __start___ex_table.as_mut_ptr() as *mut c_ulong };

    while ex_p < unsafe { __stop___ex_table.as_mut_ptr() as *mut c_ulong } {
        let insn: c_ulong;
        let fixup: c_ulong;

        insn = unsafe { *ex_p };
        ex_p = unsafe { ex_p.add(1) };
        fixup = unsafe { *ex_p };
        ex_p = unsafe { ex_p.add(1) };

        if insn == unsafe { *ip } {
            unsafe {
                *ip = fixup;
            }
            return;
        }
    }

    unsafe {
        libc::printf(
            c"No exception table match for NIA %lx ADDR %lx\n".as_ptr(),
            *ip,
            addr,
        );
        libc::abort();
    }
}

unsafe fn setup_segv_handler() {
    let mut action: libc::sigaction = unsafe { mem::zeroed() };

    unsafe {
        ptr::write_bytes(
            &mut action as *mut libc::sigaction as *mut u8,
            0,
            mem::size_of::<libc::sigaction>(),
        );
    }
    action.sa_sigaction = segv_handler as usize;
    action.sa_flags = SA_SIGINFO;
    unsafe {
        libc::sigaction(SIGSEGV, &action, ptr::null_mut());
    }
}

static mut TOTAL_PASSED: c_int = 0;
static mut TOTAL_FAILED: c_int = 0;

unsafe fn do_one_test(dstp: *mut c_char, srcp: *mut c_char, len: c_ulong) {
    let got: c_ulong;
    let expected: c_ulong;

    got = unsafe { COPY_LOOP(dstp as *mut c_void, srcp as *const c_void, len) };
    expected =
        unsafe { test_copy_tofrom_user_reference(dstp as *mut c_void, srcp as *const c_void, len) };

    if got != expected {
        unsafe {
            TOTAL_FAILED += 1;
            libc::printf(
                c"FAIL from=%p to=%p len=%ld returned %ld, expected %ld\n".as_ptr(),
                srcp,
                dstp,
                len,
                got,
                expected,
            );
        }
        //abort();
    } else {
        unsafe {
            TOTAL_PASSED += 1;
        }
    }
}

//#define MAX_LEN 512
const MAX_LEN: c_ulong = 16;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_copy_exception() -> c_int {
    let page_size: c_int;
    static mut P: *mut c_char = ptr::null_mut();
    static mut Q: *mut c_char = ptr::null_mut();
    let mut src: c_ulong;
    let mut dst: c_ulong;
    let mut len: c_ulong;

    page_size = unsafe { libc::getpagesize() };
    unsafe {
        P = libc::mmap(
            ptr::null_mut(),
            (page_size * 2) as usize,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            -1,
            0,
        ) as *mut c_char;
    }

    if unsafe { P == MAP_FAILED as *mut c_char } {
        unsafe {
            libc::perror(c"mmap".as_ptr());
            libc::exit(1);
        }
    }

    unsafe {
        ptr::write_bytes(P, 0, page_size as usize);
    }

    unsafe {
        setup_segv_handler();
    }

    if unsafe { libc::mprotect(P.add(page_size as usize) as *mut c_void, page_size as usize, PROT_NONE) } != 0 {
        unsafe {
            libc::perror(c"mprotect".as_ptr());
            libc::exit(1);
        }
    }

    unsafe {
        Q = P.add(page_size as usize).sub(MAX_LEN as usize);
    }

    src = 0;
    while src < MAX_LEN {
        dst = 0;
        while dst < MAX_LEN {
            len = 0;
            while len < MAX_LEN + 1 {
                // printf("from=%p to=%p len=%ld\n", q+dst, q+src, len);
                unsafe {
                    do_one_test(Q.add(dst as usize), Q.add(src as usize), len);
                }
                len += 1;
            }
            dst += 1;
        }
        src += 1;
    }

    unsafe {
        libc::printf(c"Totals:\n".as_ptr());
        libc::printf(c"  Pass: %d\n".as_ptr(), TOTAL_PASSED);
        libc::printf(c"  Fail: %d\n".as_ptr(), TOTAL_FAILED);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    unsafe { test_harness(test_copy_exception, CStr::from_bytes_with_nul_unchecked(b"COPY_LOOP\0").as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
