// SPDX-License-Identifier: LGPL-2.1-or-later
//! Sign a kernel module using an OpenSSL private key and X.509 certificate.
// Copyright © 2014-2016 Red Hat, Inc. All Rights Reserved.
// Copyright © 2015 Intel Corporation.
// Copyright © 2016 Hewlett Packard Enterprise Development LP.
// Original authors: David Howells <dhowells@redhat.com>,
// David Woodhouse <dwmw2@infradead.org>, Juerg Haefliger <juerg.haefliger@hpe.com>.
#![deny(unsafe_code)]

#[allow(unsafe_code, dead_code)]
#[path = "ssl-common_header.rs"]
mod crypto;
#[allow(non_camel_case_types, dead_code, missing_docs, unreachable_pub)]
#[path = "../tools/include/uapi/linux/module_signature_header.rs"]
mod module_signature;

use crypto::{Bio, Diagnostics, Signature};
use std::ffi::OsStr;
use std::io::{self, Write};
use std::os::unix::ffi::{OsStrExt, OsStringExt};

const USAGE: &[u8] = b"Usage: scripts/sign-file [-dp] <hash algo> <key> <x509> <module> [<dest>]\n       scripts/sign-file -s <raw sig> <hash algo> <x509> <module> [<dest>]\n";

#[derive(Default)]
struct Options {
    raw: bool,
    save: bool,
    only: bool,
    key_id: bool,
    operands: Vec<Vec<u8>>,
}

fn options(arguments: &[Vec<u8>], diagnostics: &mut Diagnostics) -> Result<Options, i32> {
    let mut options = Options::default();
    let mut ended = false;
    for argument in &arguments[1..] {
        if !ended && argument == b"--" {
            ended = true;
            continue;
        }
        if !ended && argument.len() > 1 && argument[0] == b'-' {
            for &flag in &argument[1..] {
                match flag {
                    b's' => options.raw = true,
                    b'p' => options.save = true,
                    b'd' => {
                        options.only = true;
                        options.save = true;
                    }
                    b'k' => options.key_id = true,
                    _ => {
                        diagnostics.output.extend_from_slice(&arguments[0]);
                        diagnostics
                            .output
                            .extend_from_slice(b": invalid option -- '");
                        diagnostics.output.push(flag);
                        diagnostics.output.extend_from_slice(b"'\n");
                        diagnostics.output.extend_from_slice(USAGE);
                        return Err(2);
                    }
                }
            }
        } else {
            options.operands.push(argument.clone());
            if std::env::var_os("POSIXLY_CORRECT").is_some() {
                ended = true;
            }
        }
    }
    if !(4..=5).contains(&options.operands.len()) {
        diagnostics.output.extend_from_slice(USAGE);
        return Err(2);
    }
    Ok(options)
}

fn sign(options: Options, diagnostics: &mut Diagnostics) -> Result<(), ()> {
    let operands = &options.operands;
    let module = &operands[3];
    let replace = operands
        .get(4)
        .is_none_or(|destination| destination == module);
    let destination = if replace {
        [module.as_slice(), b".~signed~"].concat()
    } else {
        operands[4].clone()
    };
    if replace {
        diagnostics.check(false, 240, b"asprintf")?;
    }
    let mut input = Bio::open(module, false, diagnostics, 247)?;
    let signature = if !options.raw {
        let password = std::env::var_os("KBUILD_SIGN_PIN").map(|value| value.into_vec());
        let signature = Signature::create(
            &operands[0],
            &operands[1],
            &operands[2],
            password,
            options.key_id,
            &mut input,
            diagnostics,
        )?;
        if options.save {
            let name = [module.as_slice(), b".p7s"].concat();
            diagnostics.check(false, 299, b"asprintf")?;
            let mut output = Bio::open(&name, true, diagnostics, 302)?;
            diagnostics.check(!signature.write(&mut output), 303, &name)?;
            diagnostics.check(!output.flush(), 303, &name)?;
        }
        if options.only {
            return Ok(());
        }
        Some(signature)
    } else {
        None
    };
    let mut output = Bio::open(&destination, true, diagnostics, 318)?;
    diagnostics.check(input.reset() < 0, 321, module)?;
    let mut buffer = [0u8; 4096];
    let result = loop {
        let read = input.read(&mut buffer);
        if read <= 0 {
            break read;
        }
        diagnostics.check(
            output.write(&buffer[..read as usize]) != read,
            324,
            &destination,
        )?;
    };
    drop(input);
    diagnostics.check(result < 0, 327, module)?;
    let module_size = output.written();
    if let Some(signature) = signature {
        diagnostics.check(!signature.write(&mut output), 331, &destination)?;
    } else {
        let mut raw = Bio::open(&operands[0], false, diagnostics, 339)?;
        loop {
            let read = raw.read(&mut buffer);
            if read <= 0 {
                break;
            }
            diagnostics.check(
                output.write(&buffer[..read as usize]) != read,
                341,
                &destination,
            )?;
        }
    }
    // The on-disk UAPI structure is serialized field-by-field: no host struct
    // layout, padding, or byte order is allowed to enter the signature trailer.
    let mut trailer = [0u8; 12];
    trailer[2] = module_signature::module_signature_type::MODULE_SIGNATURE_TYPE_PKCS7 as u8;
    trailer[8..]
        .copy_from_slice(&(output.written().wrapping_sub(module_size) as u32).to_be_bytes());
    diagnostics.check(
        output.write(&trailer) != trailer.len() as i32,
        347,
        &destination,
    )?;
    diagnostics.check(
        output.write(module_signature::MODULE_SIGNATURE_MARKER.as_bytes())
            != module_signature::MODULE_SIGNATURE_MARKER.len() as i32,
        348,
        &destination,
    )?;
    // A successful BIO_free does not imply that stdio flushed its buffer.
    // Never replace the original module after a deferred output failure.
    diagnostics.check(!output.flush(), 351, &destination)?;
    diagnostics.check(output.close() != 1, 351, &destination)?;
    if replace {
        diagnostics.check(
            std::fs::rename(OsStr::from_bytes(&destination), OsStr::from_bytes(module)).is_err(),
            355,
            &destination,
        )?;
    }
    Ok(())
}

fn main() {
    crypto::initialize();
    let arguments: Vec<Vec<u8>> = std::env::args_os().map(|value| value.into_vec()).collect();
    let program = arguments[0]
        .rsplit(|&byte| byte == b'/')
        .next()
        .unwrap_or(b"sign-file");
    let mut diagnostics = Diagnostics::new(program);
    let status = match options(&arguments, &mut diagnostics) {
        Ok(options) => i32::from(sign(options, &mut diagnostics).is_err()),
        Err(status) => status,
    };
    let _ = io::stderr().lock().write_all(&diagnostics.output);
    std::process::exit(status);
}
