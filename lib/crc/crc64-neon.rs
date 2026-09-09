// SPDX-License-Identifier: GPL-2.0-only
/*
 * Accelerated CRC64 (NVMe) using ARM NEON C intrinsics
 */

// Dependency intent preserved from <linux/types.h>, <asm/neon-intrinsics.h>,
// and "crc64-neon.h". Their Rust declarations are supplied externally.

pub fn crc64_nvme_neon(crc: u64, mut p: *const u8, mut len: usize) -> u64 {
    let fold_consts: uint64x2_t = unsafe { vld1q_u64(fold_consts_val.as_ptr()) };
    let mut v0: uint64x2_t = [crc, 0];
    let zero: uint64x2_t = [0, 0];

    loop {
        v0 ^= unsafe { vreinterpretq_u64_u8(vld1q_u8(p)) };

        p = unsafe { p.add(16) };
        len -= 16;
        if len < 16 {
            break;
        }

        v0 = pmull64(fold_consts, v0) ^ pmull64_high(fold_consts, v0);
    }

    /* Multiply the 128-bit value by x^64 and reduce it back to 128 bits. */
    v0 = vextq_u64(v0, zero, 1) ^ pmull64_hi_lo(fold_consts, v0);

    /* Final Barrett reduction */
    let bconsts: uint64x2_t = unsafe { vld1q_u64(bconsts_val.as_ptr()) };
    let final_: uint64x2_t = pmull64(bconsts, v0);

    v0 ^= vextq_u64(zero, final_, 1) ^ pmull64_hi_lo(bconsts, final_);

    vgetq_lane_u64(v0, 1)
}

/* x^191 mod G, x^127 mod G */
static fold_consts_val: [u64; 2] = [
    0xeadc41fd2ba3d420,
    0x21e9761e252621ac,
];

/* floor(x^127 / G), (G - x^64) / x */
static bconsts_val: [u64; 2] = [
    0x27ecfa329aef9f77,
    0x34d926535897936a,
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
