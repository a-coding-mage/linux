// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Scatterlist Cryptographic API.
 *
 * Procfs information.
 *
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 * Copyright (c) 2005 Herbert Xu <herbert@gondor.apana.org.au>
 */

// External kernel declarations supplied by the surrounding translation unit.

unsafe fn c_start(m: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    down_read(&raw mut crypto_alg_sem);
    seq_list_start(&raw mut crypto_alg_list, unsafe { *pos })
}

unsafe fn c_next(
    m: *mut seq_file,
    p: *mut core::ffi::c_void,
    pos: *mut loff_t,
) -> *mut core::ffi::c_void {
    seq_list_next(p, &raw mut crypto_alg_list, pos)
}

unsafe fn c_stop(m: *mut seq_file, p: *mut core::ffi::c_void) {
    up_read(&raw mut crypto_alg_sem);
}

unsafe fn c_show(m: *mut seq_file, p: *mut core::ffi::c_void) -> i32 {
    let alg: *mut crypto_alg = list_entry(p, crypto_alg, cra_list);

    seq_printf(m, c"name         : %s\n", (*alg).cra_name);
    seq_printf(m, c"driver       : %s\n", (*alg).cra_driver_name);
    seq_printf(m, c"module       : %s\n", module_name((*alg).cra_module));
    seq_printf(m, c"priority     : %d\n", (*alg).cra_priority);
    seq_printf(m, c"refcnt       : %u\n", refcount_read(&raw mut (*alg).cra_refcnt));
    seq_printf(
        m,
        c"selftest     : %s\n",
        if (*alg).cra_flags & CRYPTO_ALG_TESTED != 0 { c"passed" } else { c"unknown" },
    );
    seq_printf(
        m,
        c"internal     : %s\n",
        str_yes_no((*alg).cra_flags & CRYPTO_ALG_INTERNAL != 0),
    );
    if fips_enabled {
        seq_printf(
            m,
            c"fips         : %s\n",
            str_no_yes((*alg).cra_flags & CRYPTO_ALG_FIPS_INTERNAL != 0),
        );
    }

    if (*alg).cra_flags & CRYPTO_ALG_LARVAL != 0 {
        seq_printf(m, c"type         : larval\n");
        seq_printf(m, c"flags        : 0x%x\n", (*alg).cra_flags);
        seq_putc(m, b'\n' as i32);
        return 0;
    }

    if !(*alg).cra_type.is_null() && (*(*alg).cra_type).show.is_some() {
        ((*(*alg).cra_type).show.unwrap())(m, alg);
        seq_putc(m, b'\n' as i32);
        return 0;
    }

    match (*alg).cra_flags & CRYPTO_ALG_TYPE_MASK {
        CRYPTO_ALG_TYPE_CIPHER => {
            seq_printf(m, c"type         : cipher\n");
            seq_printf(m, c"blocksize    : %u\n", (*alg).cra_blocksize);
            seq_printf(m, c"min keysize  : %u\n", (*alg).cra_cipher.cia_min_keysize);
            seq_printf(m, c"max keysize  : %u\n", (*alg).cra_cipher.cia_max_keysize);
        }
        _ => {
            seq_printf(m, c"type         : unknown\n");
        }
    }

    seq_putc(m, b'\n' as i32);
    0
}

static mut crypto_seq_ops: seq_operations = seq_operations {
    start: Some(c_start),
    next: Some(c_next),
    stop: Some(c_stop),
    show: Some(c_show),
};

pub unsafe fn crypto_init_proc() {
    proc_create_seq(c"crypto", 0, core::ptr::null_mut(), &raw mut crypto_seq_ops);
}

pub unsafe fn crypto_exit_proc() {
    remove_proc_entry(c"crypto", core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
