// SPDX-License-Identifier: GPL-2.0
/*
 * This host program runs at kernel build time and generates the lookup tables
 * used by the generic CRC64 code.
 *
 * Copyright 2018 SUSE Linux.
 *   Author: Coly Li <colyli@suse.de>
 */

const CRC64_ECMA182_POLY: u64 = 0x42F0E1EBA9EA3693;
const CRC64_NVME_POLY: u64 = 0x9A6C9329AC4BC9B5;

static mut CRC64_TABLE: [u64; 256] = [0; 256];
static mut CRC64_NVME_TABLE: [u64; 256] = [0; 256];

unsafe fn generate_reflected_crc64_table(table: *mut u64, poly: u64) {
    let mut i: u64;
    let mut j: u64;
    let mut c: u64;
    let mut crc: u64;

    i = 0;
    while i < 256 {
        crc = 0;
        c = i;

        j = 0;
        while j < 8 {
            if ((crc ^ (c >> j)) & 1) != 0 {
                crc = (crc >> 1) ^ poly;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        *table.add(i as usize) = crc;
        i += 1;
    }
}

unsafe fn generate_crc64_table(table: *mut u64, poly: u64) {
    let mut i: u64;
    let mut j: u64;
    let mut c: u64;
    let mut crc: u64;

    i = 0;
    while i < 256 {
        crc = 0;
        c = i << 56;

        j = 0;
        while j < 8 {
            if ((crc ^ c) & 0x8000000000000000) != 0 {
                crc = (crc << 1) ^ poly;
            } else {
                crc <<= 1;
            }
            c <<= 1;
            j += 1;
        }

        *table.add(i as usize) = crc;
        i += 1;
    }
}

unsafe fn output_table(table: *const u64) {
    let mut i: i32;

    i = 0;
    while i < 256 {
        print!("\t0x{:016x}ULL", *table.add(i as usize));
        if (i & 0x1) != 0 {
            println!();
        } else {
            print!(", ");
        }
        i += 1;
    }
    println!("}};");
}

unsafe fn print_crc64_tables() {
    println!("/* this file is generated - do not edit */\n");
    println!("#include <linux/types.h>");
    println!("#include <linux/cache.h>\n");
    println!("static const u64 ____cacheline_aligned crc64table[256] = {{");
    output_table(CRC64_TABLE.as_ptr());

    println!("\nstatic const u64 ____cacheline_aligned crc64nvmetable[256] = {{");
    output_table(CRC64_NVME_TABLE.as_ptr());
}

fn main() {
    unsafe {
        generate_crc64_table(CRC64_TABLE.as_mut_ptr(), CRC64_ECMA182_POLY);
        generate_reflected_crc64_table(CRC64_NVME_TABLE.as_mut_ptr(), CRC64_NVME_POLY);
        print_crc64_tables();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
