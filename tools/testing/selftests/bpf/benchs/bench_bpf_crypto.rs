// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C dependencies: <argp.h>, "bench.h", "crypto_bench.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of_val;
use core::ptr;

const MAX_CIPHER_LEN: usize = 32;

static mut INPUT: *mut c_char = ptr::null_mut();

#[repr(C)]
struct crypto_ctx {
    skel: *mut crypto_bench,
    pfd: c_int,
}

static mut ctx: crypto_ctx = crypto_ctx {
    skel: ptr::null_mut(),
    pfd: 0,
};

#[repr(C)]
struct crypto_args {
    crypto_len: u32,
    crypto_cipher: *mut c_char,
}

static mut args: crypto_args = crypto_args {
    crypto_len: 16,
    crypto_cipher: c"ecb(aes)".as_ptr() as *mut c_char,
};

const ARG_CRYPTO_LEN: c_int = 5000;
const ARG_CRYPTO_CIPHER: c_int = 5001;

static opts: [argp_option; 3] = [
    argp_option {
        name: c"crypto-len".as_ptr(),
        key: ARG_CRYPTO_LEN,
        arg: c"CRYPTO_LEN".as_ptr(),
        flags: 0,
        doc: c"Set the length of crypto buffer".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"crypto-cipher".as_ptr(),
        key: ARG_CRYPTO_CIPHER,
        arg: c"CRYPTO_CIPHER".as_ptr(),
        flags: 0,
        doc: c"Set the cipher to use (default:ecb(aes))".as_ptr(),
        group: 0,
    },
    argp_option {
        name: ptr::null(),
        key: 0,
        arg: ptr::null(),
        flags: 0,
        doc: ptr::null(),
        group: 0,
    },
];

unsafe extern "C" fn crypto_parse_arg(
    key: c_int,
    arg: *mut c_char,
    state: *mut argp_state,
) -> error_t {
    match key {
        ARG_CRYPTO_LEN => {
            args.crypto_len = strtoul(arg, ptr::null_mut(), 10) as u32;
            if args.crypto_len == 0
                || (args.crypto_len as usize) > size_of_val(&(*(*(*ctx.skel).bss).dst))
            {
                fprintf(
                    stderr,
                    c"Invalid crypto buffer len (limit %zu)\n".as_ptr(),
                    size_of_val(&(*(*(*ctx.skel).bss).dst)),
                );
                argp_usage(state);
            }
        }
        ARG_CRYPTO_CIPHER => {
            args.crypto_cipher = strdup(arg);
            if strlen(args.crypto_cipher) == 0 || strlen(args.crypto_cipher) > MAX_CIPHER_LEN {
                fprintf(
                    stderr,
                    c"Invalid crypto cipher len (limit %d)\n".as_ptr(),
                    MAX_CIPHER_LEN as c_int,
                );
                argp_usage(state);
            }
        }
        _ => {
            return ARGP_ERR_UNKNOWN;
        }
    }

    0
}

#[no_mangle]
pub static bench_crypto_argp: argp = argp {
    options: opts.as_ptr(),
    parser: Some(crypto_parse_arg),
};

unsafe extern "C" fn crypto_validate() {
    if env.consumer_cnt != 0 {
        fprintf(
            stderr,
            c"bpf crypto benchmark doesn't support consumer!\n".as_ptr(),
        );
        exit(1);
    }
}

unsafe extern "C" fn crypto_setup() {
    let mut opts: bpf_test_run_opts = bpf_test_run_opts::default();

    let mut err: c_int;
    let pfd: c_int;
    let mut i: usize;
    let sz: usize;

    sz = args.crypto_len as usize;
    if sz == 0 || sz > size_of_val(&(*(*(*ctx.skel).bss).dst)) {
        fprintf(
            stderr,
            c"invalid encrypt buffer size (source %zu, target %zu)\n".as_ptr(),
            sz,
            size_of_val(&(*(*(*ctx.skel).bss).dst)),
        );
        exit(1);
    }

    setup_libbpf();

    ctx.skel = crypto_bench__open();
    if ctx.skel.is_null() {
        fprintf(stderr, c"failed to open skeleton\n".as_ptr());
        exit(1);
    }

    snprintf(
        (*(*ctx.skel).bss).cipher.as_mut_ptr(),
        128,
        c"%s".as_ptr(),
        args.crypto_cipher,
    );
    memcpy(
        (*(*ctx.skel).bss).key.as_mut_ptr() as *mut c_void,
        c"12345678testtest".as_ptr() as *const c_void,
        16,
    );
    (*(*ctx.skel).bss).key_len = 16;
    (*(*ctx.skel).bss).authsize = 0;

    srandom(time(ptr::null_mut()));
    INPUT = malloc(sz) as *mut c_char;
    i = 0;
    while i < sz - 1 {
        *INPUT.add(i) = (b'1' as i64 + random() % 9) as c_char;
        i += 1;
    }
    *INPUT.add(sz - 1) = b'\0' as c_char;

    (*(*ctx.skel).rodata).len = args.crypto_len;

    err = crypto_bench__load(ctx.skel);
    if err != 0 {
        fprintf(stderr, c"failed to load skeleton\n".as_ptr());
        crypto_bench__destroy(ctx.skel);
        exit(1);
    }

    pfd = bpf_program__fd((*(*ctx.skel).progs).crypto_setup);
    if pfd < 0 {
        fprintf(stderr, c"failed to get fd for setup prog\n".as_ptr());
        crypto_bench__destroy(ctx.skel);
        exit(1);
    }

    err = bpf_prog_test_run_opts(pfd, &mut opts);
    if err != 0 || (*(*ctx.skel).bss).status != 0 {
        fprintf(
            stderr,
            c"failed to run setup prog: err %d, status %d\n".as_ptr(),
            err,
            (*(*ctx.skel).bss).status,
        );
        crypto_bench__destroy(ctx.skel);
        exit(1);
    }
}

unsafe extern "C" fn crypto_encrypt_setup() {
    crypto_setup();
    ctx.pfd = bpf_program__fd((*(*ctx.skel).progs).crypto_encrypt);
}

unsafe extern "C" fn crypto_decrypt_setup() {
    crypto_setup();
    ctx.pfd = bpf_program__fd((*(*ctx.skel).progs).crypto_decrypt);
}

unsafe extern "C" fn crypto_measure(res: *mut bench_res) {
    (*res).hits = atomic_swap(&mut (*(*ctx.skel).bss).hits, 0);
}

unsafe extern "C" fn crypto_producer(_unused: *mut c_void) -> *mut c_void {
    let mut opts: bpf_test_run_opts = bpf_test_run_opts {
        repeat: 64,
        data_in: INPUT as *mut c_void,
        data_size_in: args.crypto_len,
        ..bpf_test_run_opts::default()
    };

    loop {
        let _ = bpf_prog_test_run_opts(ctx.pfd, &mut opts);
    }
}

#[no_mangle]
pub static bench_crypto_encrypt: bench = bench {
    name: c"crypto-encrypt".as_ptr(),
    argp: &bench_crypto_argp,
    validate: Some(crypto_validate),
    setup: Some(crypto_encrypt_setup),
    producer_thread: Some(crypto_producer),
    measure: Some(crypto_measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
};

#[no_mangle]
pub static bench_crypto_decrypt: bench = bench {
    name: c"crypto-decrypt".as_ptr(),
    argp: &bench_crypto_argp,
    validate: Some(crypto_validate),
    setup: Some(crypto_decrypt_setup),
    producer_thread: Some(crypto_producer),
    measure: Some(crypto_measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
