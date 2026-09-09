// SPDX-License-Identifier: GPL-2.0
/*
 * CRC32 and CRC32C using LoongArch crc* instructions
 *
 * Module based on mips/crypto/crc32-mips.c
 *
 * Copyright (C) 2014 Linaro Ltd <yazen.ghannam@linaro.org>
 * Copyright (C) 2018 MIPS Tech, LLC
 * Copyright (C) 2020-2023 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation:
// <asm/cpu-features.h>, <linux/unaligned.h>

macro_rules! _CRC32 {
    ($crc:expr, $value:expr, $size:ident, $kind:ident) => {{
        unsafe {
            core::arch::asm!(
                concat!(stringify!($kind), ".w.", stringify!($size), ".w %0, %1, %0"),
                inout(reg) $crc,
                in(reg) $value,
                options(nostack)
            );
        }
    }};
}

macro_rules! CRC32 {
    ($crc:expr, $value:expr, $size:ident) => {
        _CRC32!($crc, $value, $size, crc)
    };
}

macro_rules! CRC32C {
    ($crc:expr, $value:expr, $size:ident) => {
        _CRC32!($crc, $value, $size, crcc)
    };
}

// static __ro_after_init DEFINE_STATIC_KEY_FALSE(have_crc32);
static mut have_crc32: StaticKey = StaticKey::new_false();

#[inline]
unsafe fn crc32_le_arch(mut crc: u32, mut p: *const u8, mut len: usize) -> u32 {
    if !static_branch_likely(&have_crc32) {
        return crc32_le_base(crc, p, len);
    }

    while len >= core::mem::size_of::<u64>() {
        let value: u64 = get_unaligned_le64(p);

        CRC32!(crc, value, d);
        p = p.add(core::mem::size_of::<u64>());
        len -= core::mem::size_of::<u64>();
    }

    if len & core::mem::size_of::<u32>() != 0 {
        let value: u32 = get_unaligned_le32(p);

        CRC32!(crc, value, w);
        p = p.add(core::mem::size_of::<u32>());
    }

    if len & core::mem::size_of::<u16>() != 0 {
        let value: u16 = get_unaligned_le16(p);

        CRC32!(crc, value, h);
        p = p.add(core::mem::size_of::<u16>());
    }

    if len & core::mem::size_of::<u8>() != 0 {
        let value = *p;
        p = p.add(1);

        CRC32!(crc, value, b);
    }

    crc
}

#[inline]
unsafe fn crc32c_arch(mut crc: u32, mut p: *const u8, mut len: usize) -> u32 {
    if !static_branch_likely(&have_crc32) {
        return crc32c_base(crc, p, len);
    }

    while len >= core::mem::size_of::<u64>() {
        let value: u64 = get_unaligned_le64(p);

        CRC32C!(crc, value, d);
        p = p.add(core::mem::size_of::<u64>());
        len -= core::mem::size_of::<u64>();
    }

    if len & core::mem::size_of::<u32>() != 0 {
        let value: u32 = get_unaligned_le32(p);

        CRC32C!(crc, value, w);
        p = p.add(core::mem::size_of::<u32>());
    }

    if len & core::mem::size_of::<u16>() != 0 {
        let value: u16 = get_unaligned_le16(p);

        CRC32C!(crc, value, h);
        p = p.add(core::mem::size_of::<u16>());
    }

    if len & core::mem::size_of::<u8>() != 0 {
        let value = *p;
        p = p.add(1);

        CRC32C!(crc, value, b);
    }

    crc
}

// #define crc32_be_arch crc32_be_base /* not implemented on this arch */
// #define crc32_mod_init_arch crc32_mod_init_arch
unsafe fn crc32_mod_init_arch() {
    if cpu_has_crc32 {
        static_branch_enable(&mut have_crc32);
    }
}

#[inline]
fn crc32_optimizations_arch() -> u32 {
    if static_key_enabled(&have_crc32) {
        return CRC32_LE_OPTIMIZATION | CRC32C_OPTIMIZATION;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
