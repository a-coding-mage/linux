// C dependencies removed from executable Rust:
// stdlib.h, signal.h, unistd.h, linux/compiler.h, ../tests.h

use core::ffi::{c_char, c_int};
use core::ptr;

const SIGINT: c_int = 2;
const SIGALRM: c_int = 14;

type sig_atomic_t = c_int;
type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

unsafe extern "C" {
    fn atoi(nptr: *const c_char) -> c_int;
    fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    fn alarm(seconds: c_int) -> c_int;
}

#[repr(C, align(64))]
struct buf {
    data1: c_char,
    reserved: [c_char; 55],
    data2: c_char,
}

/* volatile to try to avoid the compiler seeing reserved as unused. */
static mut workload_datasym_buf1: buf = buf {
    data1: 0,
    reserved: {
        let mut reserved = [0; 55];
        /* to have this in the data section */
        reserved[0] = 1;
        reserved
    },
    data2: 0,
};

static mut done: sig_atomic_t = 0;

unsafe extern "C" fn sighandler(_sig: c_int) {
    unsafe {
        ptr::write_volatile(ptr::addr_of_mut!(done), 1);
    }
}

unsafe extern "C" fn datasym(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut sec: c_int = 1;

    if argc > 0 {
        unsafe {
            sec = atoi(*argv);
        }
    }

    unsafe {
        signal(SIGINT, Some(sighandler));
        signal(SIGALRM, Some(sighandler));
        alarm(sec);
    }

    while unsafe { ptr::read_volatile(ptr::addr_of!(done)) } == 0 {
        unsafe {
            let data1 = ptr::addr_of_mut!(workload_datasym_buf1.data1);
            ptr::write_volatile(data1, ptr::read_volatile(data1).wrapping_add(1));

            if ptr::read_volatile(data1) == 123 {
                /*
                 * Add some 'noise' in the loop to work around errata
                 * 1694299 on Arm N1.
                 *
                 * Bias exists in SPE sampling which can cause the load
                 * and store instructions to be skipped entirely. This
                 * comes and goes randomly depending on the offset the
                 * linker places the datasym loop at in the Perf binary.
                 * With an extra branch in the middle of the loop that
                 * isn't always taken, the instruction stream is no
                 * longer a continuous repeating pattern that interacts
                 * badly with the bias.
                 */
                ptr::write_volatile(data1, ptr::read_volatile(data1).wrapping_add(1));
            }

            let data2 = ptr::addr_of_mut!(workload_datasym_buf1.data2);
            ptr::write_volatile(
                data2,
                ptr::read_volatile(data2).wrapping_add(ptr::read_volatile(data1)),
            );
        }
    }
    0
}

DEFINE_WORKLOAD!(datasym);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
