// Translated from bitmap.c.  The bitmap constants and masks correspond to
// the Linux bitmap.h definitions used by the original implementation.

const BITS_PER_LONG: usize = usize::BITS as usize;

#[inline]
fn bit_word(n: u32) -> usize {
    (n as usize) / BITS_PER_LONG
}

#[inline]
fn bitmap_first_word_mask(n: u32) -> usize {
    usize::MAX << ((n as usize) % BITS_PER_LONG)
}

#[inline]
fn bitmap_last_word_mask(n: u32) -> usize {
    let shift = (n as usize) % BITS_PER_LONG;
    if shift == 0 {
        usize::MAX
    } else {
        (1usize << shift).wrapping_sub(1)
    }
}

pub unsafe fn __bitmap_set(map: *mut usize, start: u32, mut len: i32) {
    let mut p = map.add(bit_word(start));
    let size = start.wrapping_add(len as u32);
    let mut bits_to_set = (BITS_PER_LONG - ((start as usize) % BITS_PER_LONG)) as i32;
    let mut mask_to_set = bitmap_first_word_mask(start);

    while len.wrapping_sub(bits_to_set) >= 0 {
        *p |= mask_to_set;
        len = len.wrapping_sub(bits_to_set);
        bits_to_set = BITS_PER_LONG as i32;
        mask_to_set = usize::MAX;
        p = p.add(1);
    }
    if len != 0 {
        mask_to_set &= bitmap_last_word_mask(size);
        *p |= mask_to_set;
    }
}

pub unsafe fn __bitmap_clear(map: *mut usize, start: u32, mut len: i32) {
    let mut p = map.add(bit_word(start));
    let size = start.wrapping_add(len as u32);
    let mut bits_to_clear = (BITS_PER_LONG - ((start as usize) % BITS_PER_LONG)) as i32;
    let mut mask_to_clear = bitmap_first_word_mask(start);

    while len.wrapping_sub(bits_to_clear) >= 0 {
        *p &= !mask_to_clear;
        len = len.wrapping_sub(bits_to_clear);
        bits_to_clear = BITS_PER_LONG as i32;
        mask_to_clear = usize::MAX;
        p = p.add(1);
    }
    if len != 0 {
        mask_to_clear &= bitmap_last_word_mask(size);
        *p &= !mask_to_clear;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
