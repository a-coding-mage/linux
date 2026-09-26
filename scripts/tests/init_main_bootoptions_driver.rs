// SPDX-License-Identifier: GPL-2.0-only
use ffi::{c_char, c_int};
use std::{ffi::CStr, ptr};

unsafe extern "C" {
    fn original_prepare(arguments: usize, environment: usize, replacement: usize, pending: bool);
    fn original_run(mode: u32, parameter: *mut c_char, value: *mut c_char) -> c_int;
    fn original_argument(index: usize) -> *const c_char;
    fn original_environment(index: usize) -> *const c_char;
    fn original_panic() -> *const c_char;
    fn original_panic_parameter() -> *const c_char;
}

static mut EVENTS: Vec<(u32, Vec<u8>)> = Vec::new();

#[no_mangle]
unsafe extern "C" fn bootoption_event(kind: u32, value: *const c_char) {
    unsafe {
        let value = CStr::from_ptr(value.cast()).to_bytes().to_vec();
        (*ptr::addr_of_mut!(EVENTS)).push((kind, value));
    }
}

#[no_mangle]
extern "C" fn bootoption_abort() -> ! { std::process::exit(86) }

unsafe fn bytes(value: *const c_char) -> Option<Vec<u8>> {
    if value.is_null() { None }
    else { Some(unsafe { CStr::from_ptr(value.cast()) }.to_bytes().to_vec()) }
}

unsafe fn prepare(arguments: usize, environment: usize, replacement: usize, pending: bool) {
    unsafe {
        original_prepare(arguments, environment, replacement, pending);
        let argv = &mut *ptr::addr_of_mut!(main_globals::argv_init);
        argv.fill(ptr::null());
        argv[..arguments].fill(c"seed".as_ptr().cast());
        let envp = &mut *ptr::addr_of_mut!(main_globals::envp_init);
        envp.fill(ptr::null());
        envp[..environment].fill(c"other=before".as_ptr().cast());
        if replacement < environment { envp[replacement] = c"replace=before".as_ptr().cast(); }
        main_globals::panic_later = if pending { c"pending".as_ptr().cast() } else { ptr::null() };
        main_globals::panic_param = if pending { c"older".as_ptr().cast() } else { ptr::null() };
    }
}

unsafe fn invoke(rust: bool, mode: u32, parameter: *mut c_char, value: *mut c_char) -> c_int {
    unsafe {
        if !rust { return original_run(mode, parameter, value); }
        match mode {
            0 => { main_bootoptions::repair_env_string(parameter, value); 0 },
            1 => main_bootoptions::set_init_arg(parameter, value, ptr::null(), ptr::null_mut()),
            2 => main_bootoptions::unknown_bootoption(parameter, value, ptr::null(), ptr::null_mut()),
            _ => c_int::from(main_bootoptions::obsolete_checksetup(parameter)),
        }
    }
}

fn token(key: &[u8], value: Option<&[u8]>, quoted: bool) -> (Vec<u8>, Option<usize>) {
    let mut buffer = key.to_vec();
    buffer.push(0);
    let offset = value.map(|value| {
        if quoted { buffer.push(b'"'); }
        let offset = buffer.len();
        buffer.extend(value);
        buffer.push(0);
        offset
    });
    (buffer, offset)
}

#[derive(Debug, PartialEq)]
struct Snapshot {
    result: c_int,
    token: Vec<u8>,
    argv: Vec<Option<Vec<u8>>>,
    envp: Vec<Option<Vec<u8>>>,
    borrowed_offsets: Vec<Option<usize>>,
    panic: Option<Vec<u8>>,
    panic_parameter: Option<Vec<u8>>,
    events: Vec<(u32, Vec<u8>)>,
}

unsafe fn snapshot(rust: bool, result: c_int, token: Vec<u8>) -> Snapshot {
    unsafe {
        let mut borrowed_offsets = Vec::new();
        let mut borrowed = |value: *const c_char| {
            let address = value as usize;
            let begin = token.as_ptr() as usize;
            borrowed_offsets.push(if address >= begin && address < begin + token.len() {
                Some(address - begin)
            } else { None });
            bytes(value)
        };
        let argv = (0..bindings::RUST_INIT_MAIN_MAX_INIT_ARGS as usize + 2)
            .map(|index| borrowed(if rust { main_globals::argv_init[index] } else { original_argument(index) }))
            .collect();
        let envp = (0..bindings::RUST_INIT_MAIN_MAX_INIT_ENVS as usize + 2)
            .map(|index| borrowed(if rust { main_globals::envp_init[index] } else { original_environment(index) }))
            .collect();
        let panic_parameter = borrowed(if rust { main_globals::panic_param } else { original_panic_parameter() });
        Snapshot { result, token, argv, envp, borrowed_offsets,
            panic: bytes(if rust { main_globals::panic_later } else { original_panic() }),
            panic_parameter,
            events: std::mem::take(&mut *ptr::addr_of_mut!(EVENTS)),
        }
    }
}

fn main() {
    let arguments: Vec<_> = std::env::args().collect();
    if arguments.len() > 1 {
        let rust = arguments[1] == "rust";
        let mode: u32 = arguments[2].parse().unwrap();
        let alias = arguments[3] == "alias";
        let pending = arguments[4] == "pending";
        let key = if alias { b"alias".as_slice() } else { b"key" };
        let (mut buffer, _) = token(key, Some(b"bad"), true);
        unsafe {
            prepare(1, 2, usize::MAX, pending);
            let value = buffer.as_mut_ptr().add(key.len() + 3); // deliberately invalid parser geometry
            let result = invoke(rust, mode, buffer.as_mut_ptr(), value);
            println!("RETURNED {result}");
        }
        return;
    }
    let limit = bindings::RUST_INIT_MAIN_MAX_INIT_ARGS as usize;
    let mut cases = 0usize;
    for mode in 0..4 {
        for key in [b"key".as_slice(), b"replace", b"with.dot", b"alias", b"alias.dot", b"BOOT_IMAGE",
                    b"BOOT_IMAGE=kernel", b"kexec", b"kexec_suffix", b"early", b"early_more", b"early=raw",
                    b"chain", b"chain=raw", b"obsolete", b"obsolete_suffix", b"dash_key", b"dash-key",
                    b"plain", b"", b"a\xff"] {
            for value in [None, Some(b"".as_slice()), Some(b"value"), Some(b"value.with.dot"), Some(b"two words")] {
                for quoted in [false, true] {
                    for count in [1, limit - 1, limit, limit + 1] {
                        for replacement in [usize::MAX, 0, count - 1] {
                            for pending in [false, true] {
                                let mut results = Vec::new();
                                for rust in [false, true] {
                                    let (mut buffer, offset) = token(key, value, quoted);
                                    unsafe {
                                        prepare(count, count, replacement, pending);
                                        let parameter = buffer.as_mut_ptr();
                                        let value = offset.map_or(ptr::null_mut(), |index| parameter.add(index));
                                        let result = invoke(rust, mode, parameter, value);
                                        results.push(snapshot(rust, result, buffer));
                                    }
                                }
                                assert_eq!(results[0], results[1], "mode={mode} key={key:?} value={value:?} quoted={quoted} count={count} replacement={replacement} pending={pending}");
                                cases += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("INIT_MAIN_BOOTOPTIONS_OK cases={cases}");
}
