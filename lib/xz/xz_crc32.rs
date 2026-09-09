// SPDX-License-Identifier: 0BSD

/*
 * CRC32 using the polynomial from IEEE-802.3
 *
 * Authors: Lasse Collin <lasse.collin@tukaani.org>
 *          Igor Pavlov <https://7-zip.org/>
 */

/*
 * This is not the fastest implementation, but it is pretty compact.
 * The fastest versions of xz_crc32() on modern CPUs without hardware
 * accelerated CRC instruction are 3-5 times as fast as this version,
 * but they are bigger and use more memory for the lookup table.
 */

/* Dependency: xz_private.h */

/*
 * STATIC_RW_DATA is used in the pre-boot environment on some architectures.
 * See <linux/decompress/mm.h> for details.
 *
 * The default C definition makes this table translation-unit local.
 */
static mut xz_crc32_table: [u32; 256] = [0; 256];

pub unsafe fn xz_crc32_init() {
    let poly: u32 = 0xEDB88320;

    let mut i: u32;
    let mut j: u32;
    let mut r: u32;

    i = 0;
    while i < 256 {
        r = i;
        j = 0;
        while j < 8 {
            r = (r >> 1) ^ (poly & !((r & 1).wrapping_sub(1)));
            j += 1;
        }

        xz_crc32_table[i as usize] = r;
        i += 1;
    }

    return;
}

pub unsafe fn xz_crc32(buf: *const u8, mut size: usize, mut crc: u32) -> u32 {
    crc = !crc;

    let mut current = buf;
    while size != 0 {
        let index = (*current as usize) ^ ((crc & 0xFF) as usize);
        crc = xz_crc32_table[index] ^ (crc >> 8);
        current = current.add(1);
        size -= 1;
    }

    !crc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
