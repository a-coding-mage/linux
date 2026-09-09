// SPDX-License-Identifier: GPL-2.0
// Dependency intent preserved from the C includes:
// ../../include/linux/crc32poly.h, ../../include/generated/autoconf.h

static mut crc32table_le: [u32; 256] = [0; 256];
static mut crc32table_be: [u32; 256] = [0; 256];
static mut crc32ctable_le: [u32; 256] = [0; 256];

/**
 * crc32init_le() - allocate and initialize LE table data
 *
 * crc is the crc of the byte i; other entries are filled in based on the
 * fact that crctable[i^j] = crctable[i] ^ crctable[j].
 */
unsafe fn crc32init_le_generic(polynomial: u32, tab: *mut u32) {
    let mut i: u32;
    let mut j: u32;
    let mut crc: u32 = 1;

    *tab.add(0) = 0;

    i = 128;
    while i != 0 {
        crc = (crc >> 1) ^ if (crc & 1) != 0 { polynomial } else { 0 };
        j = 0;
        while j < 256 {
            *tab.add((i + j) as usize) = crc ^ *tab.add(j as usize);
            j += 2 * i;
        }
        i >>= 1;
    }
}

unsafe fn crc32init_le() {
    crc32init_le_generic(CRC32_POLY_LE, crc32table_le.as_mut_ptr());
}

unsafe fn crc32cinit_le() {
    crc32init_le_generic(CRC32C_POLY_LE, crc32ctable_le.as_mut_ptr());
}

/**
 * crc32init_be() - allocate and initialize BE table data
 */
unsafe fn crc32init_be() {
    let mut i: u32;
    let mut j: u32;
    let mut crc: u32 = 0x80000000;

    crc32table_be[0] = 0;

    i = 1;
    while i < 256 {
        crc = (crc << 1) ^ if (crc & 0x80000000) != 0 { CRC32_POLY_BE } else { 0 };
        j = 0;
        while j < i {
            crc32table_be[(i + j) as usize] = crc ^ crc32table_be[j as usize];
            j += 1;
        }
        i <<= 1;
    }
}

unsafe fn output_table(table: *const u32) {
    let mut i: u32 = 0;

    while i < 256 {
        println!(
            "\t0x{:08x}, 0x{:08x}, 0x{:08x}, 0x{:08x},",
            *table.add(i as usize),
            *table.add((i + 1) as usize),
            *table.add((i + 2) as usize),
            *table.add((i + 3) as usize)
        );
        i += 4;
    }
}

fn main() {
    unsafe {
        println!("/* this file is generated - do not edit */\n");

        crc32init_le();
        println!("static const u32 ____cacheline_aligned crc32table_le[256] = {{");
        output_table(crc32table_le.as_ptr());
        println!("}};");

        crc32init_be();
        println!("static const u32 ____cacheline_aligned crc32table_be[256] = {{");
        output_table(crc32table_be.as_ptr());
        println!("}};");

        crc32cinit_le();
        println!("static const u32 ____cacheline_aligned crc32ctable_le[256] = {{");
        output_table(crc32ctable_le.as_ptr());
        println!("}};");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
