/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: math.h, signal.h, stdlib.h, unistd.h, sys/wait.h,
// linux/compiler.h, and ../tests.h.

type sig_atomic_t = i32;
type pid_t = i32;
type sighandler_t = Option<unsafe extern "C" fn(i32)>;

const SIGALRM: i32 = 14;

unsafe extern "C" {
    fn signal(signum: i32, handler: sighandler_t) -> sighandler_t;
    fn alarm(seconds: u32) -> u32;
    fn sqrt(x: f64) -> f64;
    fn rand() -> i32;
    fn atoi(nptr: *const i8) -> i32;
    fn fork() -> pid_t;
    fn wait(wstatus: *mut i32) -> pid_t;
}

static mut done: sig_atomic_t = 0;

unsafe extern "C" fn sighandler(_sig: i32) {
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!(done), 1);
    }
}

unsafe fn __sqrtloop(sec: i32) -> i32 {
    unsafe {
        signal(SIGALRM, Some(sighandler));
        alarm(sec as u32);

        while core::ptr::read_volatile(core::ptr::addr_of!(done)) == 0 {
            let _ = sqrt(rand() as f64);
        }
    }
    0
}

unsafe fn sqrtloop(argc: i32, argv: *const *const i8) -> i32 {
    let mut sec: i32 = 1;

    unsafe {
        if argc > 0 {
            sec = atoi(*argv);
        }

        match fork() {
            0 => return __sqrtloop(sec),
            -1 => return -1,
            _ => {
                wait(core::ptr::null_mut());
            }
        }
    }
    0
}

// DEFINE_WORKLOAD(sqrtloop);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
