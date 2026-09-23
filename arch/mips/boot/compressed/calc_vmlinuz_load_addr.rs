// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2010 "Wu Zhangjin" <wuzhangjin@gmail.com>
//! Compute the compressed kernel's load address using the original 64-KiB gap.

#[path = "../host_tool.rs"]
mod host_tool;

use std::io::{self, Write};

fn hexadecimal(mut text: &[u8]) -> Option<u64> {
    while text
        .first()
        .is_some_and(|b| matches!(b, b' ' | b'\t'..=b'\r'))
    {
        text = &text[1..];
    }
    let negative = text.first() == Some(&b'-');
    if matches!(text.first(), Some(b'+' | b'-')) {
        text = &text[1..];
    }
    let mut found = false;
    if text.starts_with(b"0x") || text.starts_with(b"0X") {
        // sscanf("%llx") accepts a bare hexadecimal prefix as zero.
        found = true;
        text = &text[2..];
    }
    let mut value = 0u64;
    let mut overflow = false;
    for &byte in text {
        let digit = match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            b'A'..=b'F' => byte - b'A' + 10,
            _ => break,
        };
        found = true;
        if let Some(next) = value
            .checked_mul(16)
            .and_then(|n| n.checked_add(u64::from(digit)))
        {
            value = next;
        } else {
            overflow = true;
        }
    }
    found.then(|| {
        if overflow {
            u64::MAX
        } else if negative {
            value.wrapping_neg()
        } else {
            value
        }
    })
}

fn run() -> Result<(), Vec<u8>> {
    let arguments: Vec<_> = std::env::args_os().collect();
    if arguments.len() != 3 {
        let mut message = b"Usage: ".to_vec();
        message.extend_from_slice(arguments[0].as_encoded_bytes());
        message.extend_from_slice(b" <pathname> <vmlinux_load_addr>\n");
        return Err(message);
    }
    let size = std::fs::metadata(&arguments[1])
        .map_err(|error| host_tool::perror("stat", error))?
        .len();
    let load = hexadecimal(arguments[2].as_encoded_bytes())
        .ok_or_else(|| b"No matching characters\n".to_vec())?;
    let address = load.wrapping_add(size).wrapping_add(65536 - size % 65536);
    writeln!(io::stdout().lock(), "0x{address:x}")
        .map_err(|error| host_tool::perror("write", error))?;
    Ok(())
}
fn main() {
    host_tool::finish(run());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
