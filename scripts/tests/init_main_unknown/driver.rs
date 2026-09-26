// SPDX-License-Identifier: GPL-2.0-only
use ffi::{c_char, c_int};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
struct Snapshot {
    size: u64, align: u64, low: u64, high: u64, nid: c_int,
    allocated: u32, formatted: u32, logged: u32, freed: u32,
    free_size: u64, logged_size: u64,
    format: [c_char; 160], argument: [c_char; 4096], memory: [c_char; 4096],
}

unsafe extern "C" {
    fn capture_reset(fail: bool);
    fn capture_result() -> *const Snapshot;
    fn original_unknown_prepare(args: *const *const c_char, envs: *const *const c_char, pending: bool);
    fn original_unknown_notice();
}

fn main() {
    let limit = bindings::RUST_INIT_MAIN_MAX_INIT_ARGS as usize;
    let mut cases = 0;
    unsafe {
        for text in [b"\0".as_slice(), b"word\0", b"x='a b'\0", b"\x80=\xff\0"] {
            for arguments in [0, 1, 2, limit - 1, limit] {
                for environment in [0, 1, 2, limit - 2, limit - 1] {
                    for pending in [false, true] {
                        for fail in [false, true] {
                            let mut args = [core::ptr::null(); bindings::RUST_INIT_MAIN_MAX_INIT_ARGS as usize + 2];
                            let mut envs = [core::ptr::null(); bindings::RUST_INIT_MAIN_MAX_INIT_ENVS as usize + 2];
                            args[0] = c"init".as_ptr().cast();
                            envs[0] = c"HOME=/".as_ptr().cast();
                            envs[1] = c"TERM=linux".as_ptr().cast();
                            args[1..1 + arguments].fill(text.as_ptr());
                            envs[2..2 + environment].fill(text.as_ptr());
                            main_globals::argv_init = args;
                            main_globals::envp_init = envs;
                            main_globals::panic_later = if pending { c"pending".as_ptr().cast() } else { core::ptr::null() };
                            original_unknown_prepare(args.as_ptr(), envs.as_ptr(), pending);
                            capture_reset(fail);
                            original_unknown_notice();
                            let expected = (*capture_result()).clone();
                            capture_reset(fail);
                            unknown_notice_fixture();
                            assert_eq!(*capture_result(), expected);
                            assert_eq!({ main_globals::argv_init }, args);
                            assert_eq!({ main_globals::envp_init }, envs);
                            cases += 1;
                        }
                    }
                }
            }
        }
    }
    println!("INIT_MAIN_UNKNOWN_OK cases={cases}");
}
