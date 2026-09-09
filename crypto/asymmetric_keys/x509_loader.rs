// SPDX-License-Identifier: GPL-2.0-or-later

// Dependencies supplied by the corresponding kernel headers:
// linux/kernel.h, linux/key.h, and keys/asymmetric-type.h

pub unsafe fn x509_load_certificate_list(
    cert_list: *const u8,
    list_size: ::core::ffi::c_ulong,
    keyring: *const key,
) -> ::core::ffi::c_int {
    let mut key: key_ref_t;
    let mut p: *const u8;
    let end: *const u8;
    let mut plen: usize;

    p = cert_list;
    end = p.add(list_size as usize);
    while p < end {
        /* Each cert begins with an ASN.1 SEQUENCE tag and must be more
         * than 256 bytes in size.
         */
        if end.offset_from(p) < 4 {
            goto_dodgy_cert!();
        }
        if *p != 0x30 || *p.add(1) != 0x82 {
            goto_dodgy_cert!();
        }
        plen = ((*p.add(2) as usize) << 8) | (*p.add(3) as usize);
        plen += 4;
        if plen > end.offset_from(p) as usize {
            goto_dodgy_cert!();
        }

        key = key_create_or_update(
            make_key_ref(keyring, 1),
            b"asymmetric\0".as_ptr() as *const _,
            ::core::ptr::null(),
            p,
            plen,
            (KEY_POS_ALL & !KEY_POS_SETATTR) | KEY_USR_VIEW | KEY_USR_READ,
            KEY_ALLOC_NOT_IN_QUOTA | KEY_ALLOC_BUILT_IN | KEY_ALLOC_BYPASS_RESTRICTION,
        );
        if IS_ERR(key) {
            pr_err!(
                b"Problem loading in-kernel X.509 certificate (%ld)\n\0".as_ptr(),
                PTR_ERR(key)
            );
        } else {
            pr_notice!(
                b"Loaded X.509 cert '%s'\n\0".as_ptr(),
                (*key_ref_to_ptr(key)).description
            );
            key_ref_put(key);
        }
        p = p.add(plen);
    }

    return 0;

    // C label `dodgy_cert`; this macro preserves the original branch target.
    #[allow(unreachable_code)]
    fn dodgy_cert() -> ::core::ffi::c_int {
        pr_err!(b"Problem parsing in-kernel X.509 certificate list\n\0".as_ptr());
        0
    }

    // The source-level translation above uses this dependency-provided control-flow hook.
    macro_rules! goto_dodgy_cert {
        () => {{ return dodgy_cert(); }};
    }
}

// EXPORT_SYMBOL_GPL(x509_load_certificate_list)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
