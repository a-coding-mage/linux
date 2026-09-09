// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2010 "Wu Zhangjin" <wuzhangjin@gmail.com>
 */

use std::env;
use std::fs;
use std::io::{self, Write};

const SZ_64K: u64 = 64 * 1024;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    let failure = 1;
    let success = 0;

    if argc != 3 {
        eprintln!("Usage: {} <pathname> <vmlinux_load_addr>", argv[0]);
        std::process::exit(failure);
    }

    let sb = match fs::metadata(&argv[1]) {
        Ok(metadata) => metadata,
        Err(error) => {
            eprintln!("stat: {}", error);
            std::process::exit(failure);
        }
    };

    /* Convert hex characters to dec number */
    let address_text = argv[2].trim_start();
    let address_text = address_text
        .strip_prefix("0x")
        .or_else(|| address_text.strip_prefix("0X"))
        .unwrap_or(address_text);
    let vmlinux_load_addr = match u64::from_str_radix(address_text, 16) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("No matching characters");
            std::process::exit(failure);
        }
    };

    let vmlinux_size = sb.len();
    let mut vmlinuz_load_addr = vmlinux_load_addr.wrapping_add(vmlinux_size);

    /*
     * Align with 64KB: KEXEC needs load sections to be aligned to PAGE_SIZE,
     * which may be as large as 64KB depending on the kernel configuration.
     */

    vmlinuz_load_addr = vmlinuz_load_addr
        .wrapping_add(SZ_64K.wrapping_sub(vmlinux_size % SZ_64K));

    println!("0x{:x}", vmlinuz_load_addr);

    // Keep the C program's explicit success return status.
    let _ = io::stdout().flush();
    std::process::exit(success);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
