// SPDX-License-Identifier: LGPL-2.1-or-later
//! Checked ownership and diagnostics around the OpenSSL host-library ABI.
// All foreign handles are opaque. The only borrowed C storage is a documented
// error string or the synchronous PEM callback's caller-owned buffer.
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(not(openssl_configured))]
compile_error!("OpenSSL ABI configuration is missing; use scripts/openssl_config.py with the host compiler and libcrypto header flags");

use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void, CStr, CString};
use std::ptr::{self, NonNull};

mod ffi {
    use super::*;
    macro_rules! opaque {
        ($($name:ident),*) => { $(pub(super) enum $name {})* };
    }
    opaque!(
        Bio, Key, Cert, Name, Digest, Cms, Signer, Stack, Init, Library, Provider, Store, Info, Ui,
        Engine
    );
    pub(super) type Password =
        Option<unsafe extern "C" fn(*mut c_char, c_int, c_int, *mut c_void) -> c_int>;
    #[link(name = "crypto")]
    extern "C" {
        pub(super) fn BIO_new_file(name: *const c_char, mode: *const c_char) -> *mut Bio;
        pub(super) fn BIO_free(bio: *mut Bio) -> c_int;
        pub(super) fn BIO_read(bio: *mut Bio, buffer: *mut c_void, len: c_int) -> c_int;
        pub(super) fn BIO_write(bio: *mut Bio, buffer: *const c_void, len: c_int) -> c_int;
        pub(super) fn BIO_ctrl(
            bio: *mut Bio,
            command: c_int,
            larg: c_long,
            parg: *mut c_void,
        ) -> c_long;
        pub(super) fn BIO_test_flags(bio: *const Bio, flags: c_int) -> c_int;
        #[cfg(ossl_bio_u64)]
        pub(super) fn BIO_number_written(bio: *mut Bio) -> u64;
        #[cfg(not(ossl_bio_u64))]
        pub(super) fn BIO_number_written(bio: *mut Bio) -> c_ulong;
        pub(super) fn PEM_read_bio_PrivateKey(
            bio: *mut Bio,
            key: *mut *mut Key,
            password: Password,
            data: *mut c_void,
        ) -> *mut Key;
        pub(super) fn PEM_read_bio_X509(
            bio: *mut Bio,
            cert: *mut *mut Cert,
            password: Password,
            data: *mut c_void,
        ) -> *mut Cert;
        pub(super) fn d2i_X509_bio(bio: *mut Bio, cert: *mut *mut Cert) -> *mut Cert;
        pub(super) fn i2d_X509_bio(bio: *mut Bio, cert: *mut Cert) -> c_int;
        pub(super) fn X509_get_subject_name(cert: *const Cert) -> *mut Name;
        pub(super) fn X509_NAME_oneline(
            name: *const Name,
            output: *mut c_char,
            len: c_int,
        ) -> *mut c_char;
        pub(super) fn EVP_PKEY_free(key: *mut Key);
        pub(super) fn X509_free(cert: *mut Cert);
        pub(super) fn EVP_get_digestbyname(name: *const c_char) -> *const Digest;
        pub(super) fn CMS_sign(
            cert: *mut Cert,
            key: *mut Key,
            certs: *mut Stack,
            data: *mut Bio,
            flags: c_uint,
        ) -> *mut Cms;
        pub(super) fn CMS_add1_signer(
            cms: *mut Cms,
            cert: *mut Cert,
            key: *mut Key,
            digest: *const Digest,
            flags: c_uint,
        ) -> *mut Signer;
        pub(super) fn CMS_final(
            cms: *mut Cms,
            data: *mut Bio,
            detached: *mut Bio,
            flags: c_uint,
        ) -> c_int;
        pub(super) fn i2d_CMS_bio_stream(
            output: *mut Bio,
            cms: *mut Cms,
            input: *mut Bio,
            flags: c_int,
        ) -> c_int;
        pub(super) fn CMS_ContentInfo_free(cms: *mut Cms);
        pub(super) fn ERR_peek_error() -> c_ulong;
        pub(super) fn ERR_peek_last_error() -> c_ulong;
        pub(super) fn ERR_peek_error_line(file: *mut *const c_char, line: *mut c_int) -> c_ulong;
        pub(super) fn ERR_error_string_n(code: c_ulong, buffer: *mut c_char, len: usize);
        pub(super) fn ERR_get_error() -> c_ulong;
        pub(super) fn ERR_clear_error();
        #[cfg(ossl110)]
        pub(super) fn OPENSSL_init_crypto(options: u64, settings: *const Init) -> c_int;
        #[cfg(not(ossl110))]
        pub(super) fn OPENSSL_add_all_algorithms_noconf();
        #[cfg(all(not(ossl110), ossl_load_conf))]
        pub(super) fn OPENSSL_add_all_algorithms_conf();
        #[cfg(not(ossl110))]
        pub(super) fn OpenSSL_add_all_digests();
        #[cfg(not(ossl110))]
        pub(super) fn ERR_load_crypto_strings();
        #[cfg(ossl3series)]
        pub(super) fn EVP_PKEY_is_a(key: *const Key, name: *const c_char) -> c_int;
        #[cfg(ossl300)]
        pub(super) fn OSSL_PROVIDER_try_load(
            library: *mut Library,
            name: *const c_char,
            retain_fallbacks: c_int,
        ) -> *mut Provider;
        #[cfg(ossl300)]
        pub(super) fn OSSL_PROVIDER_unload(provider: *mut Provider) -> c_int;
        #[cfg(ossl300)]
        pub(super) fn OSSL_STORE_open(
            uri: *const c_char,
            ui: *const Ui,
            ui_data: *mut c_void,
            post_process: Option<unsafe extern "C" fn(*mut Info, *mut c_void) -> *mut Info>,
            post_data: *mut c_void,
        ) -> *mut Store;
        #[cfg(ossl300)]
        pub(super) fn OSSL_STORE_eof(store: *mut Store) -> c_int;
        #[cfg(ossl300)]
        pub(super) fn OSSL_STORE_error(store: *mut Store) -> c_int;
        #[cfg(ossl300)]
        pub(super) fn OSSL_STORE_load(store: *mut Store) -> *mut Info;
        #[cfg(ossl300)]
        pub(super) fn OSSL_STORE_INFO_get_type(info: *const Info) -> c_int;
        #[cfg(ossl300)]
        pub(super) fn OSSL_STORE_INFO_get1_PKEY(info: *const Info) -> *mut Key;
        #[cfg(ossl300)]
        pub(super) fn OSSL_STORE_INFO_get1_CERT(info: *const Info) -> *mut Cert;
        #[cfg(ossl300)]
        pub(super) fn OSSL_STORE_INFO_free(info: *mut Info);
        #[cfg(ossl300)]
        pub(super) fn OSSL_STORE_close(store: *mut Store) -> c_int;
        #[cfg(ossl_engine)]
        pub(super) fn ENGINE_load_builtin_engines();
        #[cfg(ossl_engine)]
        pub(super) fn ENGINE_by_id(name: *const c_char) -> *mut Engine;
        #[cfg(ossl_engine)]
        pub(super) fn ENGINE_init(engine: *mut Engine) -> c_int;
        #[cfg(ossl_engine)]
        pub(super) fn ENGINE_finish(engine: *mut Engine) -> c_int;
        #[cfg(ossl_engine)]
        pub(super) fn ENGINE_free(engine: *mut Engine) -> c_int;
        #[cfg(ossl_engine)]
        pub(super) fn ENGINE_ctrl_cmd_string(
            engine: *mut Engine,
            command: *const c_char,
            argument: *const c_char,
            optional: c_int,
        ) -> c_int;
        #[cfg(ossl_engine)]
        pub(super) fn ENGINE_ctrl_cmd(
            engine: *mut Engine,
            command: *const c_char,
            larg: c_long,
            parg: *mut c_void,
            callback: Option<unsafe extern "C" fn()>,
            optional: c_int,
        ) -> c_int;
        #[cfg(ossl_engine)]
        pub(super) fn ENGINE_load_private_key(
            engine: *mut Engine,
            name: *const c_char,
            ui: *mut Ui,
            data: *mut c_void,
        ) -> *mut Key;
    }
}

/// Diagnostic collection preserves the original sign-file.c call-site numbers.
pub(crate) struct Diagnostics {
    program: Vec<u8>,
    pub(crate) output: Vec<u8>,
}

impl Diagnostics {
    pub(crate) fn new(program: &[u8]) -> Self {
        Self {
            program: program.to_vec(),
            output: Vec::new(),
        }
    }
    pub(crate) fn fail<T>(&mut self, message: &[u8]) -> Result<T, ()> {
        self.output.extend_from_slice(&self.program);
        self.output.extend_from_slice(b": ");
        self.output.extend_from_slice(message);
        self.output.push(b'\n');
        Err(())
    }
    pub(crate) fn check(&mut self, condition: bool, line: usize, message: &[u8]) -> Result<(), ()> {
        self.drain(line, false);
        if condition {
            self.fail(message)
        } else {
            Ok(())
        }
    }
    pub(crate) fn drain(&mut self, source_line: usize, silent: bool) {
        // SAFETY: the error queue is thread-local. Returned strings remain
        // valid until the next queue mutation, which occurs after copying.
        unsafe {
            if ffi::ERR_peek_error() == 0 {
                return;
            }
            if !silent {
                self.output
                    .extend_from_slice(format!("At main.c:{source_line}:\n").as_bytes());
            }
            loop {
                let mut file = ptr::null();
                let mut line = 0;
                let code = ffi::ERR_peek_error_line(&mut file, &mut line);
                if code == 0 {
                    break;
                }
                let mut buffer = [0 as c_char; 256];
                ffi::ERR_error_string_n(
                    (code as c_int) as c_ulong,
                    buffer.as_mut_ptr(),
                    buffer.len(),
                );
                if !silent {
                    self.output.extend_from_slice(b"- SSL ");
                    self.output
                        .extend_from_slice(CStr::from_ptr(buffer.as_ptr()).to_bytes());
                    self.output.extend_from_slice(b": ");
                    if !file.is_null() {
                        self.output
                            .extend_from_slice(CStr::from_ptr(file).to_bytes());
                    }
                    self.output
                        .extend_from_slice(format!(":{line}\n").as_bytes());
                }
                ffi::ERR_get_error();
            }
        }
    }
}

pub(crate) fn initialize() {
    // SAFETY: OpenSSL initialization functions have no pointer preconditions.
    unsafe {
        #[cfg(ossl110)]
        {
            let configuration = if cfg!(ossl_load_conf) { 0x40 } else { 0 };
            ffi::OPENSSL_init_crypto(0x4 | 0x8 | configuration, ptr::null());
            ffi::OPENSSL_init_crypto(0x2, ptr::null());
        }
        #[cfg(not(ossl110))]
        {
            #[cfg(ossl_load_conf)]
            ffi::OPENSSL_add_all_algorithms_conf();
            #[cfg(not(ossl_load_conf))]
            ffi::OPENSSL_add_all_algorithms_noconf();
            ffi::ERR_load_crypto_strings();
        }
        ffi::ERR_clear_error();
    }
}

pub(crate) fn clear_errors() {
    // SAFETY: clears only the current thread's OpenSSL error queue.
    unsafe {
        ffi::ERR_clear_error();
    }
}

pub(crate) fn no_start_line_error() -> bool {
    // SAFETY: reads only the current thread's error queue without mutation.
    let error = unsafe { ffi::ERR_peek_last_error() };
    let (shift, mask) = if cfg!(ossl300) {
        (23, 0x7fffff)
    } else {
        (24, 0xfff)
    };
    ((error >> shift) & 0xff) == 9 && (error & mask) == 108
}

pub(crate) struct Bio(Option<NonNull<ffi::Bio>>);

impl Bio {
    pub(crate) fn open(
        name: &[u8],
        write: bool,
        diagnostics: &mut Diagnostics,
        line: usize,
    ) -> Result<Self, ()> {
        let name_c = CString::new(name).map_err(|_| {
            let _ = diagnostics.fail::<()>(name);
        })?;
        // SAFETY: both arguments are NUL-terminated, and the resulting handle
        // has exactly one owning Bio, including on early-return paths.
        let pointer = unsafe {
            ffi::BIO_new_file(name_c.as_ptr(), if write { c"wb" } else { c"rb" }.as_ptr())
        };
        diagnostics.check(pointer.is_null(), line, name)?;
        Ok(Self(NonNull::new(pointer)))
    }
    fn pointer(&self) -> *mut ffi::Bio {
        self.0.map_or(ptr::null_mut(), NonNull::as_ptr)
    }
    pub(crate) fn read(&mut self, buffer: &mut [u8]) -> i32 {
        let length = buffer.len().min(c_int::MAX as usize) as c_int;
        // SAFETY: exclusive buffer borrow and its exact writable length.
        unsafe { ffi::BIO_read(self.pointer(), buffer.as_mut_ptr().cast(), length) }
    }
    pub(crate) fn write(&mut self, buffer: &[u8]) -> i32 {
        let length = buffer.len().min(c_int::MAX as usize) as c_int;
        // SAFETY: the input slice is readable for length bytes during the call.
        unsafe { ffi::BIO_write(self.pointer(), buffer.as_ptr().cast(), length) }
    }
    pub(crate) fn reset(&mut self) -> i64 {
        // SAFETY: BIO_CTRL_RESET takes no pointer argument.
        unsafe { ffi::BIO_ctrl(self.pointer(), 1, 0, ptr::null_mut()) as i64 }
    }
    pub(crate) fn retry(&self) -> bool {
        // SAFETY: the handle remains live throughout this shared query.
        unsafe { ffi::BIO_test_flags(self.pointer(), 0x08) != 0 }
    }
    pub(crate) fn written(&self) -> u64 {
        // SAFETY: selected return ABI is probed against the installed header.
        unsafe { ffi::BIO_number_written(self.pointer()) as u64 }
    }
    pub(crate) fn flush(&mut self) -> bool {
        // SAFETY: BIO_CTRL_FLUSH takes no pointer argument.
        unsafe { ffi::BIO_ctrl(self.pointer(), 11, 0, ptr::null_mut()) > 0 }
    }
    pub(crate) fn close(mut self) -> i32 {
        self.0.take().map_or(1, |pointer| {
            // SAFETY: ownership is removed before calling the destructor.
            unsafe { ffi::BIO_free(pointer.as_ptr()) }
        })
    }
}

impl Drop for Bio {
    fn drop(&mut self) {
        if let Some(pointer) = self.0.take() {
            // SAFETY: this is the sole owner of the live handle.
            unsafe {
                ffi::BIO_free(pointer.as_ptr());
            }
        }
    }
}

struct Password(Option<Vec<u8>>);

unsafe extern "C" fn password_callback(
    buffer: *mut c_char,
    length: c_int,
    _writing: c_int,
    data: *mut c_void,
) -> c_int {
    if buffer.is_null() || data.is_null() || length <= 0 {
        return -1;
    }
    // SAFETY: read_key passes a unique Password reference for this synchronous
    // callback. OpenSSL supplies a writable buffer of the stated length.
    let password = unsafe { &mut *data.cast::<Password>() };
    let Some(value) = password.0.as_ref() else {
        return -1;
    };
    if value.len() >= length as usize {
        return -1;
    }
    let size = value.len();
    unsafe {
        ptr::copy_nonoverlapping(value.as_ptr(), buffer.cast(), size);
        buffer.add(size).write(0);
    }
    password.0 = None;
    size as c_int
}

struct Key {
    pointer: NonNull<ffi::Key>,
    #[cfg(ossl300)]
    providers: Vec<NonNull<ffi::Provider>>,
    #[cfg(ossl_engine)]
    engine: Option<NonNull<ffi::Engine>>,
}

impl Key {
    fn new(pointer: NonNull<ffi::Key>) -> Self {
        Self {
            pointer,
            #[cfg(ossl300)]
            providers: Vec::new(),
            #[cfg(ossl_engine)]
            engine: None,
        }
    }
}

impl Drop for Key {
    fn drop(&mut self) {
        // SAFETY: keys are released before the providers/engine they rely on.
        unsafe {
            ffi::EVP_PKEY_free(self.pointer.as_ptr());
            #[cfg(ossl300)]
            for provider in &self.providers {
                ffi::OSSL_PROVIDER_unload(provider.as_ptr());
            }
            #[cfg(ossl_engine)]
            if let Some(engine) = self.engine {
                ffi::ENGINE_finish(engine.as_ptr());
                ffi::ENGINE_free(engine.as_ptr());
            }
        }
    }
}

fn read_key(
    name: &[u8],
    password: Option<Vec<u8>>,
    diagnostics: &mut Diagnostics,
) -> Result<Key, ()> {
    if name.starts_with(b"pkcs11:") {
        return read_pkcs11(name, password, diagnostics);
    }
    let bio = Bio::open(name, false, diagnostics, 137)?;
    let mut password = Password(password);
    // SAFETY: callback context lives until this synchronous read returns;
    // OpenSSL allocates a new key because the reuse pointer is NULL.
    let pointer = unsafe {
        ffi::PEM_read_bio_PrivateKey(
            bio.pointer(),
            ptr::null_mut(),
            Some(password_callback),
            (&mut password as *mut Password).cast(),
        )
    };
    diagnostics.check(pointer.is_null(), 140, name)?;
    Ok(Key::new(NonNull::new(pointer).ok_or(())?))
}

#[cfg(ossl300)]
fn read_pkcs11(
    name: &[u8],
    _password: Option<Vec<u8>>,
    diagnostics: &mut Diagnostics,
) -> Result<Key, ()> {
    // Provider references intentionally remain loaded for the process lifetime
    // on error, just as in sign-file.c; success transfers them to the key.
    let mut providers = Vec::new();
    for (name, line, message) in [
        (c"pkcs11", 83, b"OSSL_PROVIDER_try_load(pkcs11)".as_slice()),
        (
            c"default",
            85,
            b"OSSL_PROVIDER_try_load(default)".as_slice(),
        ),
    ] {
        // SAFETY: NULL selects the default OpenSSL context; the name is static.
        let provider = unsafe { ffi::OSSL_PROVIDER_try_load(ptr::null_mut(), name.as_ptr(), 1) };
        if provider.is_null() {
            diagnostics.check(true, line, message)?;
        }
        providers.push(NonNull::new(provider).ok_or(())?);
    }
    let uri = CString::new(name).map_err(|_| ())?;
    // SAFETY: URI stays alive for the call; all optional callbacks are NULL.
    let store = unsafe {
        ffi::OSSL_STORE_open(
            uri.as_ptr(),
            ptr::null(),
            ptr::null_mut(),
            None,
            ptr::null_mut(),
        )
    };
    diagnostics.check(store.is_null(), 88, b"OSSL_STORE_open")?;
    let mut key = None;
    // SAFETY: store is live and info objects are freed only after extracting
    // an independent owned key reference with the get1 API.
    unsafe {
        while ffi::OSSL_STORE_eof(store) == 0 {
            let info = ffi::OSSL_STORE_load(store);
            if info.is_null() {
                diagnostics.drain(94, false);
                if ffi::OSSL_STORE_error(store) != 0 {
                    break;
                }
                continue;
            }
            if ffi::OSSL_STORE_INFO_get_type(info) == 4 {
                key = NonNull::new(ffi::OSSL_STORE_INFO_get1_PKEY(info));
                ffi::OSSL_STORE_INFO_free(info);
                if key.is_none() {
                    ffi::OSSL_STORE_close(store);
                    diagnostics.check(true, 99, b"OSSL_STORE_INFO_get1_PKEY")?;
                }
                break;
            }
            ffi::OSSL_STORE_INFO_free(info);
        }
        ffi::OSSL_STORE_close(store);
    }
    let Some(pointer) = key else {
        return diagnostics.fail(b"OSSL_STORE_INFO_get1_PKEY");
    };
    let mut key = Key::new(pointer);
    key.providers = providers;
    Ok(key)
}

#[cfg(all(not(ossl300), ossl_engine))]
fn read_pkcs11(
    name: &[u8],
    password: Option<Vec<u8>>,
    diagnostics: &mut Diagnostics,
) -> Result<Key, ()> {
    // SAFETY: engine APIs use only opaque owned references and C strings;
    // the functional engine reference remains live until the key is dropped.
    unsafe {
        ffi::ENGINE_load_builtin_engines();
        diagnostics.drain(110, true);
        let engine = ffi::ENGINE_by_id(c"pkcs11".as_ptr());
        diagnostics.check(engine.is_null(), 112, b"Load PKCS#11 ENGINE")?;
        if ffi::ENGINE_init(engine) != 0 {
            diagnostics.drain(114, true);
        } else {
            ffi::ENGINE_free(engine);
            diagnostics.check(true, 116, b"ENGINE_init")?;
        }
        if let Some(password) = password {
            let password = CString::new(password).map_err(|_| ())?;
            diagnostics.check(
                ffi::ENGINE_ctrl_cmd_string(engine, c"PIN".as_ptr(), password.as_ptr(), 0) == 0,
                118,
                b"Set PKCS#11 PIN",
            )?;
        }
        let name_c = CString::new(name).map_err(|_| ())?;
        let pointer =
            ffi::ENGINE_load_private_key(engine, name_c.as_ptr(), ptr::null_mut(), ptr::null_mut());
        diagnostics.check(pointer.is_null(), 120, name)?;
        let mut key = Key::new(NonNull::new(pointer).ok_or(())?);
        key.engine = NonNull::new(engine);
        Ok(key)
    }
}

#[cfg(not(any(ossl300, ossl_engine)))]
fn read_pkcs11(
    _name: &[u8],
    _password: Option<Vec<u8>>,
    diagnostics: &mut Diagnostics,
) -> Result<Key, ()> {
    diagnostics
        .output
        .extend_from_slice(b"no pkcs11 engine/provider available\n");
    Err(())
}

pub(crate) struct Certificate(NonNull<ffi::Cert>);
impl Certificate {
    pub(crate) fn read_pem(bio: &mut Bio) -> Option<Self> {
        // SAFETY: a new owned certificate is allocated from the live BIO.
        NonNull::new(unsafe {
            ffi::PEM_read_bio_X509(bio.pointer(), ptr::null_mut(), None, ptr::null_mut())
        })
        .map(Self)
    }
    pub(crate) fn write_der(&self, bio: &mut Bio) -> bool {
        // SAFETY: both handles are live; DER serialization borrows the cert.
        unsafe { ffi::i2d_X509_bio(bio.pointer(), self.0.as_ptr()) != 0 }
    }
    pub(crate) fn subject_name(&self, max_len: usize) -> Vec<u8> {
        let length = max_len.min(c_int::MAX as usize);
        if length == 0 {
            return Vec::new();
        }
        let mut buffer = vec![0u8; length];
        // SAFETY: subject name is borrowed from the live certificate; the
        // supplied writable buffer prevents OpenSSL from allocating storage.
        let result = unsafe {
            ffi::X509_NAME_oneline(
                ffi::X509_get_subject_name(self.0.as_ptr()),
                buffer.as_mut_ptr().cast(),
                length as c_int,
            )
        };
        if result.is_null() {
            return Vec::new();
        }
        buffer.truncate(
            buffer
                .iter()
                .position(|&byte| byte == 0)
                .unwrap_or(buffer.len()),
        );
        buffer
    }
    pub(crate) fn from_pkcs11(
        name: &[u8],
        password: Option<Vec<u8>>,
        diagnostics: &mut Diagnostics,
    ) -> Result<Self, ()> {
        read_pkcs11_certificate(name, password, diagnostics)
    }
}
impl Drop for Certificate {
    fn drop(&mut self) {
        // SAFETY: the certificate pointer is an owned OpenSSL allocation.
        unsafe {
            ffi::X509_free(self.0.as_ptr());
        }
    }
}

#[cfg(ossl300)]
fn read_pkcs11_certificate(
    name: &[u8],
    _password: Option<Vec<u8>>,
    diagnostics: &mut Diagnostics,
) -> Result<Certificate, ()> {
    // SAFETY: all handles remain opaque; get1_CERT returns an independent
    // owned certificate. Providers stay loaded just as in extract-cert.c.
    unsafe {
        if ffi::OSSL_PROVIDER_try_load(ptr::null_mut(), c"pkcs11".as_ptr(), 1).is_null() {
            diagnostics.check(true, 72, b"OSSL_PROVIDER_try_load(pkcs11)")?;
        }
        if ffi::OSSL_PROVIDER_try_load(ptr::null_mut(), c"default".as_ptr(), 1).is_null() {
            diagnostics.check(true, 74, b"OSSL_PROVIDER_try_load(default)")?;
        }
        let name = CString::new(name).map_err(|_| ())?;
        let store = ffi::OSSL_STORE_open(
            name.as_ptr(),
            ptr::null(),
            ptr::null_mut(),
            None,
            ptr::null_mut(),
        );
        diagnostics.check(store.is_null(), 77, b"OSSL_STORE_open")?;
        let mut certificate = None;
        while ffi::OSSL_STORE_eof(store) == 0 {
            let info = ffi::OSSL_STORE_load(store);
            if info.is_null() {
                diagnostics.drain(83, false);
                if ffi::OSSL_STORE_error(store) != 0 {
                    break;
                }
                continue;
            }
            if ffi::OSSL_STORE_INFO_get_type(info) == 5 {
                certificate = NonNull::new(ffi::OSSL_STORE_INFO_get1_CERT(info));
                ffi::OSSL_STORE_INFO_free(info);
                if certificate.is_none() {
                    ffi::OSSL_STORE_close(store);
                    diagnostics.check(true, 88, b"OSSL_STORE_INFO_get1_CERT")?;
                }
                break;
            }
            ffi::OSSL_STORE_INFO_free(info);
        }
        ffi::OSSL_STORE_close(store);
        match certificate {
            Some(certificate) => Ok(Certificate(certificate)),
            None => diagnostics.fail(b"load_cert_pkcs11 failed"),
        }
    }
}

#[cfg(all(not(ossl300), ossl_engine))]
fn read_pkcs11_certificate(
    name: &[u8],
    password: Option<Vec<u8>>,
    diagnostics: &mut Diagnostics,
) -> Result<Certificate, ()> {
    // This is the two-pointer application message documented by the PKCS#11
    // engine's LOAD_CERT_CTRL command, not the layout of an OpenSSL object.
    #[repr(C)]
    struct CertificateRequest {
        identifier: *const c_char,
        certificate: *mut ffi::Cert,
    }
    let name = CString::new(name).map_err(|_| ())?;
    let mut request = CertificateRequest {
        identifier: name.as_ptr(),
        certificate: ptr::null_mut(),
    };
    // SAFETY: the engine receives the exact two-pointer command payload;
    // strings and payload remain live for every synchronous engine call.
    unsafe {
        ffi::ENGINE_load_builtin_engines();
        diagnostics.drain(106, true);
        let engine = ffi::ENGINE_by_id(c"pkcs11".as_ptr());
        diagnostics.check(engine.is_null(), 108, b"Load PKCS#11 ENGINE")?;
        if ffi::ENGINE_init(engine) != 0 {
            diagnostics.drain(110, true);
        } else {
            ffi::ENGINE_free(engine);
            diagnostics.check(true, 112, b"ENGINE_init")?;
        }
        if let Some(password) = password {
            let password = CString::new(password).map_err(|_| ())?;
            diagnostics.check(
                ffi::ENGINE_ctrl_cmd_string(engine, c"PIN".as_ptr(), password.as_ptr(), 0) == 0,
                114,
                b"Set PKCS#11 PIN",
            )?;
        }
        ffi::ENGINE_ctrl_cmd(
            engine,
            c"LOAD_CERT_CTRL".as_ptr(),
            0,
            (&mut request as *mut CertificateRequest).cast(),
            None,
            1,
        );
        diagnostics.check(
            request.certificate.is_null(),
            116,
            b"Get X.509 from PKCS#11",
        )?;
        ffi::ENGINE_finish(engine);
        ffi::ENGINE_free(engine);
    }
    Ok(Certificate(NonNull::new(request.certificate).ok_or(())?))
}

#[cfg(not(any(ossl300, ossl_engine)))]
fn read_pkcs11_certificate(
    _name: &[u8],
    _password: Option<Vec<u8>>,
    diagnostics: &mut Diagnostics,
) -> Result<Certificate, ()> {
    diagnostics
        .output
        .extend_from_slice(b"no pkcs11 engine/provider available\n");
    Err(())
}

fn read_certificate(name: &[u8], diagnostics: &mut Diagnostics) -> Result<Certificate, ()> {
    let mut bio = Bio::open(name, false, diagnostics, 155)?;
    let mut prefix = [0; 2];
    let count = bio.read(&mut prefix);
    if count != 2 {
        let message = if bio.retry() {
            Some(b": Read wanted retry\n".as_slice())
        } else if count >= 0 {
            Some(b": Short read\n".as_slice())
        } else {
            None
        };
        if let Some(message) = message {
            diagnostics.output.extend_from_slice(name);
            diagnostics.output.extend_from_slice(message);
            return Err(());
        }
        diagnostics.check(true, 168, name)?;
    }
    diagnostics.check(bio.reset() != 0, 171, name)?;
    // SAFETY: the live BIO is used synchronously and a new certificate is
    // allocated. NULL PEM callback/user data select OpenSSL's default callback.
    let pointer = unsafe {
        if prefix[0] == 0x30 && (0x81..=0x84).contains(&prefix[1]) {
            ffi::d2i_X509_bio(bio.pointer(), ptr::null_mut())
        } else {
            ffi::PEM_read_bio_X509(bio.pointer(), ptr::null_mut(), None, ptr::null_mut())
        }
    };
    drop(bio);
    diagnostics.check(pointer.is_null(), 181, name)?;
    Ok(Certificate(NonNull::new(pointer).ok_or(())?))
}

pub(crate) struct Signature {
    pointer: NonNull<ffi::Cms>,
    _key: Key,
    _certificate: Certificate,
}

impl Signature {
    pub(crate) fn create(
        hash: &[u8],
        key_name: &[u8],
        cert_name: &[u8],
        password: Option<Vec<u8>>,
        key_id: bool,
        data: &mut Bio,
        diagnostics: &mut Diagnostics,
    ) -> Result<Self, ()> {
        let key = read_key(key_name, password, diagnostics)?;
        let certificate = read_certificate(cert_name, diagnostics)?;
        let name = CString::new(hash).map_err(|_| ())?;
        // SAFETY: initialization has no pointer requirements; digest lookup
        // borrows static library storage for the lifetime of the signature.
        let digest = unsafe {
            #[cfg(ossl110)]
            ffi::OPENSSL_init_crypto(0x8, ptr::null());
            #[cfg(not(ossl110))]
            ffi::OpenSSL_add_all_digests();
            diagnostics.drain(258, false);
            ffi::EVP_get_digestbyname(name.as_ptr())
        };
        diagnostics.check(digest.is_null(), 260, b"EVP_get_digestbyname")?;
        // Public CMS flag values are stable across supported OpenSSL ABIs.
        let mut flags = 0x2 | 0x100 | 0x4000 | 0x80 | 0x40 | 0x1000 | 0x200;
        if key_id {
            flags |= 0x10000;
        }
        if cfg!(ossl_no_signing_time) {
            flags |= 0x400000;
        }
        #[cfg(ossl3series)]
        {
            // SAFETY: the algorithm-name query only borrows the key.
            if [c"ML-DSA-44", c"ML-DSA-65", c"ML-DSA-87"]
                .iter()
                .any(|name| unsafe { ffi::EVP_PKEY_is_a(key.pointer.as_ptr(), name.as_ptr()) != 0 })
            {
                flags &= !0x100;
            }
        }
        // SAFETY: CMS_PARTIAL requests an empty owned message; add1_signer
        // takes independent references to the still-live key and certificate.
        let pointer = unsafe {
            ffi::CMS_sign(
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                flags,
            )
        };
        diagnostics.check(pointer.is_null(), 288, b"CMS_sign")?;
        let signature = Self {
            pointer: NonNull::new(pointer).ok_or(())?,
            _key: key,
            _certificate: certificate,
        };
        let signer = unsafe {
            ffi::CMS_add1_signer(
                pointer,
                signature._certificate.0.as_ptr(),
                signature._key.pointer.as_ptr(),
                digest,
                flags,
            )
        };
        diagnostics.check(signer.is_null(), 290, b"CMS_add1_signer")?;
        let result = unsafe { ffi::CMS_final(pointer, data.pointer(), ptr::null_mut(), flags) };
        diagnostics.check(result != 1, 292, b"CMS_final")?;
        Ok(signature)
    }
    pub(crate) fn write(&self, output: &mut Bio) -> bool {
        // SAFETY: the owned message was finalized; flags=0 emits DER into the
        // exclusively borrowed BIO without consuming either object.
        unsafe {
            ffi::i2d_CMS_bio_stream(output.pointer(), self.pointer.as_ptr(), ptr::null_mut(), 0)
                == 1
        }
    }
}

impl Drop for Signature {
    fn drop(&mut self) {
        // SAFETY: release the CMS owner before its key/provider fields.
        unsafe {
            ffi::CMS_ContentInfo_free(self.pointer.as_ptr());
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
