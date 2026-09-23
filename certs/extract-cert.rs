// SPDX-License-Identifier: LGPL-2.1-or-later
// Copyright (C) 2014-2015 Red Hat, Inc. All Rights Reserved.
// Copyright (C) 2015 Intel Corporation.
// Original authors: David Howells and David Woodhouse.

//! Extract X.509 certificates in DER form from a PEM file or PKCS#11 URI.

use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::os::unix::ffi::OsStrExt;

#[allow(dead_code)] // The shared module also implements module signing.
#[path = "../scripts/ssl-common_header.rs"]
mod ssl;
use ssl::{Bio, Certificate, Diagnostics};

fn write_certificate(
    certificate: &Certificate,
    destination: &[u8],
    output: &mut Option<Bio>,
    verbose: bool,
    diagnostics: &mut Diagnostics,
) -> Result<(), ()> {
    if output.is_none() {
        *output = Some(Bio::open(destination, true, diagnostics, 57)?);
    }
    let subject = certificate.subject_name(200);
    diagnostics.check(
        !certificate.write_der(output.as_mut().ok_or(())?),
        60,
        destination,
    )?;
    if verbose {
        diagnostics.output.extend_from_slice(b"Extracted cert: ");
        diagnostics.output.extend_from_slice(&subject);
        diagnostics.output.push(b'\n');
    }
    Ok(())
}

fn extract(source: &[u8], destination: &[u8], diagnostics: &mut Diagnostics) -> Result<(), ()> {
    let verbose =
        env::var_os("KBUILD_VERBOSE").is_some_and(|value| value.as_bytes().contains(&b'1'));
    if source.is_empty() {
        // No source means an empty trusted-key list, not a PEM parse failure.
        let file = File::create(std::ffi::OsStr::from_bytes(destination));
        diagnostics.check(file.is_err(), 151, destination)?;
        return Ok(());
    }
    let mut output = None;
    if source.starts_with(b"pkcs11:") {
        let pin = env::var_os("KBUILD_SIGN_PIN").map(|value| value.as_bytes().to_vec());
        let certificate = Certificate::from_pkcs11(source, pin, diagnostics)?;
        write_certificate(&certificate, destination, &mut output, verbose, diagnostics)?;
    } else {
        let mut input = Bio::open(source, false, diagnostics, 164)?;
        loop {
            let certificate = Certificate::read_pem(&mut input);
            if output.is_some() && certificate.is_none() && ssl::no_start_line_error() {
                ssl::clear_errors();
                break;
            }
            diagnostics.check(certificate.is_none(), 176, source)?;
            write_certificate(
                &certificate.ok_or(())?,
                destination,
                &mut output,
                verbose,
                diagnostics,
            )?;
        }
    }
    if let Some(mut output) = output {
        // BIO_free may hide a buffered fclose failure. Explicitly flushing
        // prevents a successful exit after writing an incomplete certificate.
        diagnostics.check(!output.flush(), 181, destination)?;
        diagnostics.check(output.close() != 1, 181, destination)?;
    }
    Ok(())
}

fn main() {
    ssl::initialize();
    let arguments: Vec<_> = env::args_os().collect();
    if arguments.len() != 3 {
        let _ = io::stderr()
            .lock()
            .write_all(b"Usage: extract-cert <source> <dest>\n");
        std::process::exit(2);
    }
    let name = arguments[0]
        .as_bytes()
        .rsplit(|&byte| byte == b'/')
        .next()
        .unwrap_or(b"extract-cert");
    let mut diagnostics = Diagnostics::new(name);
    let result = extract(
        arguments[1].as_bytes(),
        arguments[2].as_bytes(),
        &mut diagnostics,
    );
    let written = io::stderr().lock().write_all(&diagnostics.output);
    if result.is_err() || written.is_err() {
        std::process::exit(1);
    }
}
