/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014 Felix Fietkau <nbd@nbd.name>
 * Copyright (C) 2004 - 2009 Ivo van Doorn <IvDoorn@gmail.com>
 */

/*
 * Original C dependencies:
 * #include <linux/build_bug.h>
 * #include <linux/kernel.h>
 * #include <asm/byteorder.h>
 */

/*
 * Bitfield access macros
 *
 * FIELD_{GET,PREP} macros take as first parameter shifted mask
 * from which they extract the base mask and shift amount.
 * Mask must be a compilation time constant.
 *
 * Example:
 *
 *  #define REG_FIELD_A  GENMASK(6, 0)
 *  #define REG_FIELD_B  BIT(7)
 *  #define REG_FIELD_C  GENMASK(15, 8)
 *  #define REG_FIELD_D  GENMASK(31, 16)
 *
 * Get:
 *  a = FIELD_GET(REG_FIELD_A, reg);
 *  b = FIELD_GET(REG_FIELD_B, reg);
 *
 * Set:
 *  reg = FIELD_PREP(REG_FIELD_A, 1) |
 *	  FIELD_PREP(REG_FIELD_B, 0) |
 *	  FIELD_PREP(REG_FIELD_C, c) |
 *	  FIELD_PREP(REG_FIELD_D, 0x40);
 *
 * Modify:
 *  reg &= ~REG_FIELD_C;
 *  reg |= FIELD_PREP(REG_FIELD_C, c);
 */

#[inline(always)]
pub const fn __bf_shf(x: u64) -> u32 {
    x.trailing_zeros()
}

/*
 * C macros __scalar_type_to_unsigned_cases, __unsigned_scalar_typeof,
 * __bf_cast_unsigned, and __BF_FIELD_CHECK rely on C typeof, _Generic,
 * statement expressions, and compile-time diagnostics from linux/build_bug.h.
 * The public field macros below preserve their value computations directly.
 */

/**
 * FIELD_MAX() - produce the maximum value representable by a field
 * @_mask: shifted mask defining the field's length and position
 *
 * FIELD_MAX() returns the maximum value that can be held in the field
 * specified by @_mask.
 */
#[macro_export]
macro_rules! FIELD_MAX {
    ($_mask:expr) => {{
        ($_mask) >> $crate::__bf_shf(($_mask) as u64)
    }};
}

/**
 * FIELD_FIT() - check if value fits in the field
 * @_mask: shifted mask defining the field's length and position
 * @_val:  value to test against the field
 *
 * Return: true if @_val can fit inside @_mask, false if @_val is too big.
 */
#[macro_export]
macro_rules! FIELD_FIT {
    ($_mask:expr, $_val:expr) => {{
        (((($_val) << $crate::__bf_shf(($_mask) as u64)) & !($_mask)) == 0)
    }};
}

/**
 * FIELD_PREP() - prepare a bitfield element
 * @_mask: shifted mask defining the field's length and position
 * @_val:  value to put in the field
 *
 * FIELD_PREP() masks and shifts up the value.  The result should
 * be combined with other fields of the bitfield using logical OR.
 */
#[macro_export]
macro_rules! FIELD_PREP {
    ($_mask:expr, $_val:expr) => {{
        (($_val) << $crate::__bf_shf(($_mask) as u64)) & ($_mask)
    }};
}

/**
 * FIELD_GET() - extract a bitfield element
 * @_mask: shifted mask defining the field's length and position
 * @_reg:  value of entire bitfield
 *
 * FIELD_GET() extracts the field specified by @_mask from the
 * bitfield passed in as @_reg by masking and shifting it down.
 */
#[macro_export]
macro_rules! FIELD_GET {
    ($_mask:expr, $_reg:expr) => {{
        (($_reg) & ($_mask)) >> $crate::__bf_shf(($_mask) as u64)
    }};
}

unsafe extern "C" {
    #[link_name = "__field_overflow"]
    pub fn __field_overflow();
    #[link_name = "__bad_mask"]
    pub fn __bad_mask();
}

#[inline(always)]
pub unsafe fn field_multiplier(field: u64) -> u64 {
    if (field | field.wrapping_sub(1)) & ((field | field.wrapping_sub(1)).wrapping_add(1)) != 0 {
        unsafe {
            __bad_mask();
        }
    }
    field & field.wrapping_neg()
}

#[inline(always)]
pub unsafe fn field_mask(field: u64) -> u64 {
    field / unsafe { field_multiplier(field) }
}

#[macro_export]
macro_rules! field_max {
    ($field:expr) => {{
        unsafe { $crate::field_mask(($field) as u64) as _ }
    }};
}

#[inline(always)]
pub unsafe fn u8_encode_bits(v: u8, field: u8) -> u8 {
    if (v & !(unsafe { field_mask(field as u64) } as u8)) != 0 {
        unsafe {
            __field_overflow();
        }
    }
    ((v & (unsafe { field_mask(field as u64) } as u8))
        .wrapping_mul(unsafe { field_multiplier(field as u64) } as u8)) as u8
}

#[inline(always)]
pub unsafe fn u8_replace_bits(old: u8, val: u8, field: u8) -> u8 {
    (old & !field) | unsafe { u8_encode_bits(val, field) }
}

#[inline(always)]
pub unsafe fn u8p_replace_bits(p: *mut u8, val: u8, field: u8) {
    unsafe {
        *p = (*p & !field) | u8_encode_bits(val, field);
    }
}

#[inline(always)]
pub unsafe fn u8_get_bits(v: u8, field: u8) -> u8 {
    ((v & field) as u64 / unsafe { field_multiplier(field as u64) }) as u8
}

#[inline(always)]
pub unsafe fn le16_encode_bits(v: u16, field: u16) -> __le16 {
    if (v & !(unsafe { field_mask(field as u64) } as u16)) != 0 {
        unsafe {
            __field_overflow();
        }
    }
    cpu_to_le16((v & (unsafe { field_mask(field as u64) } as u16)).wrapping_mul(unsafe {
        field_multiplier(field as u64)
    } as u16))
}

#[inline(always)]
pub unsafe fn le16_replace_bits(old: __le16, val: u16, field: u16) -> __le16 {
    (old & !cpu_to_le16(field)) | unsafe { le16_encode_bits(val, field) }
}

#[inline(always)]
pub unsafe fn le16p_replace_bits(p: *mut __le16, val: u16, field: u16) {
    unsafe {
        *p = (*p & !cpu_to_le16(field)) | le16_encode_bits(val, field);
    }
}

#[inline(always)]
pub unsafe fn le16_get_bits(v: __le16, field: u16) -> u16 {
    ((le16_to_cpu(v) & field) as u64 / unsafe { field_multiplier(field as u64) }) as u16
}

#[inline(always)]
pub unsafe fn be16_encode_bits(v: u16, field: u16) -> __be16 {
    if (v & !(unsafe { field_mask(field as u64) } as u16)) != 0 {
        unsafe {
            __field_overflow();
        }
    }
    cpu_to_be16((v & (unsafe { field_mask(field as u64) } as u16)).wrapping_mul(unsafe {
        field_multiplier(field as u64)
    } as u16))
}

#[inline(always)]
pub unsafe fn be16_replace_bits(old: __be16, val: u16, field: u16) -> __be16 {
    (old & !cpu_to_be16(field)) | unsafe { be16_encode_bits(val, field) }
}

#[inline(always)]
pub unsafe fn be16p_replace_bits(p: *mut __be16, val: u16, field: u16) {
    unsafe {
        *p = (*p & !cpu_to_be16(field)) | be16_encode_bits(val, field);
    }
}

#[inline(always)]
pub unsafe fn be16_get_bits(v: __be16, field: u16) -> u16 {
    ((be16_to_cpu(v) & field) as u64 / unsafe { field_multiplier(field as u64) }) as u16
}

#[inline(always)]
pub unsafe fn u16_encode_bits(v: u16, field: u16) -> u16 {
    if (v & !(unsafe { field_mask(field as u64) } as u16)) != 0 {
        unsafe {
            __field_overflow();
        }
    }
    (v & (unsafe { field_mask(field as u64) } as u16))
        .wrapping_mul(unsafe { field_multiplier(field as u64) } as u16)
}

#[inline(always)]
pub unsafe fn u16_replace_bits(old: u16, val: u16, field: u16) -> u16 {
    (old & !field) | unsafe { u16_encode_bits(val, field) }
}

#[inline(always)]
pub unsafe fn u16p_replace_bits(p: *mut u16, val: u16, field: u16) {
    unsafe {
        *p = (*p & !field) | u16_encode_bits(val, field);
    }
}

#[inline(always)]
pub unsafe fn u16_get_bits(v: u16, field: u16) -> u16 {
    ((v & field) as u64 / unsafe { field_multiplier(field as u64) }) as u16
}

#[inline(always)]
pub unsafe fn le32_encode_bits(v: u32, field: u32) -> __le32 {
    if (v & !(unsafe { field_mask(field as u64) } as u32)) != 0 {
        unsafe {
            __field_overflow();
        }
    }
    cpu_to_le32((v & (unsafe { field_mask(field as u64) } as u32)).wrapping_mul(unsafe {
        field_multiplier(field as u64)
    } as u32))
}

#[inline(always)]
pub unsafe fn le32_replace_bits(old: __le32, val: u32, field: u32) -> __le32 {
    (old & !cpu_to_le32(field)) | unsafe { le32_encode_bits(val, field) }
}

#[inline(always)]
pub unsafe fn le32p_replace_bits(p: *mut __le32, val: u32, field: u32) {
    unsafe {
        *p = (*p & !cpu_to_le32(field)) | le32_encode_bits(val, field);
    }
}

#[inline(always)]
pub unsafe fn le32_get_bits(v: __le32, field: u32) -> u32 {
    ((le32_to_cpu(v) & field) as u64 / unsafe { field_multiplier(field as u64) }) as u32
}

#[inline(always)]
pub unsafe fn be32_encode_bits(v: u32, field: u32) -> __be32 {
    if (v & !(unsafe { field_mask(field as u64) } as u32)) != 0 {
        unsafe {
            __field_overflow();
        }
    }
    cpu_to_be32((v & (unsafe { field_mask(field as u64) } as u32)).wrapping_mul(unsafe {
        field_multiplier(field as u64)
    } as u32))
}

#[inline(always)]
pub unsafe fn be32_replace_bits(old: __be32, val: u32, field: u32) -> __be32 {
    (old & !cpu_to_be32(field)) | unsafe { be32_encode_bits(val, field) }
}

#[inline(always)]
pub unsafe fn be32p_replace_bits(p: *mut __be32, val: u32, field: u32) {
    unsafe {
        *p = (*p & !cpu_to_be32(field)) | be32_encode_bits(val, field);
    }
}

#[inline(always)]
pub unsafe fn be32_get_bits(v: __be32, field: u32) -> u32 {
    ((be32_to_cpu(v) & field) as u64 / unsafe { field_multiplier(field as u64) }) as u32
}

#[inline(always)]
pub unsafe fn u32_encode_bits(v: u32, field: u32) -> u32 {
    if (v & !(unsafe { field_mask(field as u64) } as u32)) != 0 {
        unsafe {
            __field_overflow();
        }
    }
    (v & (unsafe { field_mask(field as u64) } as u32))
        .wrapping_mul(unsafe { field_multiplier(field as u64) } as u32)
}

#[inline(always)]
pub unsafe fn u32_replace_bits(old: u32, val: u32, field: u32) -> u32 {
    (old & !field) | unsafe { u32_encode_bits(val, field) }
}

#[inline(always)]
pub unsafe fn u32p_replace_bits(p: *mut u32, val: u32, field: u32) {
    unsafe {
        *p = (*p & !field) | u32_encode_bits(val, field);
    }
}

#[inline(always)]
pub unsafe fn u32_get_bits(v: u32, field: u32) -> u32 {
    ((v & field) as u64 / unsafe { field_multiplier(field as u64) }) as u32
}

#[inline(always)]
pub unsafe fn le64_encode_bits(v: u64, field: u64) -> __le64 {
    if (v & !unsafe { field_mask(field) }) != 0 {
        unsafe {
            __field_overflow();
        }
    }
    cpu_to_le64((v & unsafe { field_mask(field) }).wrapping_mul(unsafe { field_multiplier(field) }))
}

#[inline(always)]
pub unsafe fn le64_replace_bits(old: __le64, val: u64, field: u64) -> __le64 {
    (old & !cpu_to_le64(field)) | unsafe { le64_encode_bits(val, field) }
}

#[inline(always)]
pub unsafe fn le64p_replace_bits(p: *mut __le64, val: u64, field: u64) {
    unsafe {
        *p = (*p & !cpu_to_le64(field)) | le64_encode_bits(val, field);
    }
}

#[inline(always)]
pub unsafe fn le64_get_bits(v: __le64, field: u64) -> u64 {
    (le64_to_cpu(v) & field) / unsafe { field_multiplier(field) }
}

#[inline(always)]
pub unsafe fn be64_encode_bits(v: u64, field: u64) -> __be64 {
    if (v & !unsafe { field_mask(field) }) != 0 {
        unsafe {
            __field_overflow();
        }
    }
    cpu_to_be64((v & unsafe { field_mask(field) }).wrapping_mul(unsafe { field_multiplier(field) }))
}

#[inline(always)]
pub unsafe fn be64_replace_bits(old: __be64, val: u64, field: u64) -> __be64 {
    (old & !cpu_to_be64(field)) | unsafe { be64_encode_bits(val, field) }
}

#[inline(always)]
pub unsafe fn be64p_replace_bits(p: *mut __be64, val: u64, field: u64) {
    unsafe {
        *p = (*p & !cpu_to_be64(field)) | be64_encode_bits(val, field);
    }
}

#[inline(always)]
pub unsafe fn be64_get_bits(v: __be64, field: u64) -> u64 {
    (be64_to_cpu(v) & field) / unsafe { field_multiplier(field) }
}

#[inline(always)]
pub unsafe fn u64_encode_bits(v: u64, field: u64) -> u64 {
    if (v & !unsafe { field_mask(field) }) != 0 {
        unsafe {
            __field_overflow();
        }
    }
    (v & unsafe { field_mask(field) }).wrapping_mul(unsafe { field_multiplier(field) })
}

#[inline(always)]
pub unsafe fn u64_replace_bits(old: u64, val: u64, field: u64) -> u64 {
    (old & !field) | unsafe { u64_encode_bits(val, field) }
}

#[inline(always)]
pub unsafe fn u64p_replace_bits(p: *mut u64, val: u64, field: u64) {
    unsafe {
        *p = (*p & !field) | u64_encode_bits(val, field);
    }
}

#[inline(always)]
pub unsafe fn u64_get_bits(v: u64, field: u64) -> u64 {
    (v & field) / unsafe { field_multiplier(field) }
}
