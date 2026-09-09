/* SPDX-License-Identifier: LGPL-2.1
 *
 * Based on Paul Hsieh's (LGPG 2.1) hash function
 * From: http://www.azillionmonkeys.com/qed/hash.html
 */

#[inline(always)]
unsafe fn get16bits(d: *const std::ffi::c_char) -> u16 {
    std::ptr::read_unaligned(d as *const u16)
}

#[inline(always)]
pub unsafe fn SuperFastHash(
    mut data: *const std::ffi::c_char,
    mut len: i32,
    initval: u32,
) -> u32 {
    let mut hash = initval;
    let mut tmp: u32;
    let rem: i32;

    if len <= 0 || data.is_null() {
        return 0;
    }

    rem = len & 3;
    len >>= 2;

    /* Main loop */
    for _ in 0..len {
        hash = hash.wrapping_add(get16bits(data) as u32);
        tmp = (u32::from(get16bits(data.add(2))) << 11) ^ hash;
        hash = (hash << 16) ^ tmp;
        data = data.add(2 * std::mem::size_of::<u16>());
        hash = hash.wrapping_add(hash >> 11);
    }

    /* Handle end cases */
    match rem {
        3 => {
            hash = hash.wrapping_add(get16bits(data) as u32);
            hash ^= hash << 16;
            hash ^= ((*(data.add(std::mem::size_of::<u16>()) as *const i8)) as i32 as u32) << 18;
            hash = hash.wrapping_add(hash >> 11);
        }
        2 => {
            hash = hash.wrapping_add(get16bits(data) as u32);
            hash ^= hash << 11;
            hash = hash.wrapping_add(hash >> 17);
        }
        1 => {
            hash = hash.wrapping_add((*data as i8 as i32) as u32);
            hash ^= hash << 10;
            hash = hash.wrapping_add(hash >> 1);
        }
        _ => {}
    }

    /* Force "avalanching" of final 127 bits */
    hash ^= hash << 3;
    hash = hash.wrapping_add(hash >> 5);
    hash ^= hash << 4;
    hash = hash.wrapping_add(hash >> 17);
    hash ^= hash << 25;
    hash = hash.wrapping_add(hash >> 6);

    hash
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
