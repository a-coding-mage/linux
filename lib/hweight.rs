// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/export.h>, <linux/bitops.h>, <asm/types.h>

/**
 * DOC: __sw_hweightN - returns the hamming weight of a N-bit word
 * @w: the word to weigh
 *
 * The Hamming Weight of a number is the total number of bits set in it.
 */

pub fn __sw_hweight32(mut w: u32) -> u32 {
    // CONFIG_ARCH_HAS_FAST_MULTIPLIER selects the corresponding C branch.
    #[cfg(CONFIG_ARCH_HAS_FAST_MULTIPLIER)]
    {
        w = w.wrapping_sub((w >> 1) & 0x5555_5555);
        w = (w & 0x3333_3333).wrapping_add((w >> 2) & 0x3333_3333);
        w = w.wrapping_add(w >> 4) & 0x0f0f_0f0f;
        return w.wrapping_mul(0x0101_0101) >> 24;
    }

    #[cfg(not(CONFIG_ARCH_HAS_FAST_MULTIPLIER))]
    {
        let mut res = w.wrapping_sub((w >> 1) & 0x5555_5555);
        res = (res & 0x3333_3333).wrapping_add((res >> 2) & 0x3333_3333);
        res = res.wrapping_add(res >> 4) & 0x0f0f_0f0f;
        res = res.wrapping_add(res >> 8);
        return res.wrapping_add(res >> 16) & 0x0000_00ff;
    }
}

// EXPORT_SYMBOL(__sw_hweight32);

pub fn __sw_hweight16(w: u32) -> u32 {
    let mut res = w.wrapping_sub((w >> 1) & 0x5555);
    res = (res & 0x3333).wrapping_add((res >> 2) & 0x3333);
    res = res.wrapping_add(res >> 4) & 0x0f0f;
    (res.wrapping_add(res >> 8)) & 0x00ff
}

// EXPORT_SYMBOL(__sw_hweight16);

pub fn __sw_hweight8(w: u32) -> u32 {
    let mut res = w.wrapping_sub((w >> 1) & 0x55);
    res = (res & 0x33).wrapping_add((res >> 2) & 0x33);
    (res.wrapping_add(res >> 4)) & 0x0f
}

// EXPORT_SYMBOL(__sw_hweight8);

pub fn __sw_hweight64(w: u64) -> usize {
    // BITS_PER_LONG == 32
    #[cfg(target_pointer_width = "32")]
    {
        return (__sw_hweight32((w >> 32) as u32)
            .wrapping_add(__sw_hweight32(w as u32))) as usize;
    }

    // BITS_PER_LONG == 64
    #[cfg(target_pointer_width = "64")]
    {
        // CONFIG_ARCH_HAS_FAST_MULTIPLIER selects the corresponding C branch.
        #[cfg(CONFIG_ARCH_HAS_FAST_MULTIPLIER)]
        {
            let mut value = w.wrapping_sub((w >> 1) & 0x5555_5555_5555_5555);
            value = (value & 0x3333_3333_3333_3333)
                .wrapping_add((value >> 2) & 0x3333_3333_3333_3333);
            value = value.wrapping_add(value >> 4) & 0x0f0f_0f0f_0f0f_0f0f;
            return value.wrapping_mul(0x0101_0101_0101_0101) as usize >> 56;
        }

        #[cfg(not(CONFIG_ARCH_HAS_FAST_MULTIPLIER))]
        {
            let mut res = w.wrapping_sub((w >> 1) & 0x5555_5555_5555_5555);
            res = (res & 0x3333_3333_3333_3333)
                .wrapping_add((res >> 2) & 0x3333_3333_3333_3333);
            res = res.wrapping_add(res >> 4) & 0x0f0f_0f0f_0f0f_0f0f;
            res = res.wrapping_add(res >> 8);
            res = res.wrapping_add(res >> 16);
            return (res.wrapping_add(res >> 32) & 0x0000_0000_0000_00ff) as usize;
        }
    }

    unreachable!()
}

// EXPORT_SYMBOL(__sw_hweight64);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
