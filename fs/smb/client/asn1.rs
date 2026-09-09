// SPDX-License-Identifier: GPL-2.0-or-later

// Kernel and CIFS declarations are supplied by the surrounding translation.
// The ASN.1 decoder declaration is supplied by cifs_spnego_negtokeninit.asn1.h.

extern "C" {
    fn asn1_ber_decoder(
        decoder: *const core::ffi::c_void,
        context: *mut TCP_Server_Info,
        data: *mut u8,
        length: isize,
    ) -> i32;
    fn look_up_OID(value: *const core::ffi::c_void, vlen: usize) -> OID;
    fn sprint_oid(
        value: *const core::ffi::c_void,
        vlen: usize,
        buf: *mut i8,
        buflen: usize,
    );
    fn cifs_dbg(level: i32, fmt: *const i8, ...);
    static cifs_spnego_negtokeninit_decoder: core::ffi::c_void;
}

// These types, constants, and the error value are provided by the CIFS
// headers translated elsewhere.
type OID = crate::OID;
type TCP_Server_Info = crate::TCP_Server_Info;

extern "C" {
    static OID_spnego: OID;
    static OID_mskrb5: OID;
    static OID_krb5u2u: OID;
    static OID_krb5: OID;
    static OID_ntlmssp: OID;
    static OID_IAKerb: OID;
}

#[no_mangle]
pub unsafe extern "C" fn decode_negTokenInit(
    security_blob: *mut u8,
    length: i32,
    server: *mut TCP_Server_Info,
) -> i32 {
    if asn1_ber_decoder(
        &cifs_spnego_negtokeninit_decoder,
        server,
        security_blob,
        length as isize,
    ) == 0
    {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn cifs_gssapi_this_mech(
    _context: *mut core::ffi::c_void,
    _hdrlen: usize,
    _tag: u8,
    value: *const core::ffi::c_void,
    vlen: usize,
) -> i32 {
    let oid = look_up_OID(value, vlen);
    if oid != OID_spnego {
        let mut buf = [0i8; 50];

        sprint_oid(value, vlen, buf.as_mut_ptr(), buf.len());
        cifs_dbg(
            crate::FYI,
            b"Error decoding negTokenInit header: unexpected OID %s\n\0".as_ptr()
                as *const i8,
            buf.as_ptr(),
        );
        return -crate::EBADMSG;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn cifs_neg_token_init_mech_type(
    context: *mut core::ffi::c_void,
    _hdrlen: usize,
    _tag: u8,
    value: *const core::ffi::c_void,
    vlen: usize,
) -> i32 {
    let server = context as *mut TCP_Server_Info;
    let oid = look_up_OID(value, vlen);

    if oid == OID_mskrb5 {
        (*server).sec_mskerberos = true;
    } else if oid == OID_krb5u2u {
        (*server).sec_kerberosu2u = true;
    } else if oid == OID_krb5 {
        (*server).sec_kerberos = true;
    } else if oid == OID_ntlmssp {
        (*server).sec_ntlmssp = true;
    } else if oid == OID_IAKerb {
        (*server).sec_iakerb = true;
    } else {
        let mut buf = [0i8; 50];

        sprint_oid(value, vlen, buf.as_mut_ptr(), buf.len());
        cifs_dbg(
            crate::FYI,
            b"Decoding negTokenInit: unsupported OID %s\n\0".as_ptr() as *const i8,
            buf.as_ptr(),
        );
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
