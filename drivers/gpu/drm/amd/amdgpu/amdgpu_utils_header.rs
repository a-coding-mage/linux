/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2025 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

/* Generic 2-bit capability attribute encoding:
 * 00 INVALID, 01 RO, 10 WO, 11 RW
 */
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_cap_attr {
    AMDGPU_CAP_ATTR_INVALID = 0,
    AMDGPU_CAP_ATTR_RO = 1 << 0,
    AMDGPU_CAP_ATTR_WO = 1 << 1,
    AMDGPU_CAP_ATTR_RW = (1 << 0) | (1 << 1),
}

pub const AMDGPU_CAP_ATTR_BITS: usize = 2;
pub const AMDGPU_CAP_ATTR_MAX: u32 = (1u32 << AMDGPU_CAP_ATTR_BITS) - 1;

/* Internal helper to build helpers for a given enum NAME. */
#[macro_export]
macro_rules! DECLARE_ATTR_CAP_CLASS_HELPERS {
    ($name:ident) => {
        pub const $name##_BITMAP_BITS: usize = $name##_COUNT * AMDGPU_CAP_ATTR_BITS;
        #[repr(C)]
        pub struct $name##_caps {
            pub bmap: [usize; (($name##_BITMAP_BITS + usize::BITS as usize - 1)
                               / usize::BITS as usize)],
        }

        #[inline]
        pub const fn $name##_ATTR_START(cap: $name##_cap_id) -> usize {
            cap as usize * AMDGPU_CAP_ATTR_BITS
        }

        #[inline]
        pub unsafe fn $name##_attr_init(c: *mut $name##_caps) {
            if !c.is_null() {
                (*c).bmap.fill(0);
            }
        }

        #[inline]
        pub unsafe fn $name##_attr_set(
            c: *mut $name##_caps,
            cap: $name##_cap_id,
            attr: amdgpu_cap_attr,
        ) -> i32 {
            if c.is_null() || cap as usize >= $name##_COUNT
                || attr as u32 > AMDGPU_CAP_ATTR_MAX
            {
                return -22; /* -EINVAL */
            }
            let bit = $name##_ATTR_START(cap);
            let word = bit / usize::BITS as usize;
            let shift = bit % usize::BITS as usize;
            let mask = AMDGPU_CAP_ATTR_MAX as usize << shift;
            (*c).bmap[word] = ((*c).bmap[word] & !mask) | ((attr as usize) << shift);
            0
        }

        #[inline]
        pub unsafe fn $name##_attr_get(
            c: *const $name##_caps,
            cap: $name##_cap_id,
            out: *mut amdgpu_cap_attr,
        ) -> i32 {
            if c.is_null() || out.is_null() || cap as usize >= $name##_COUNT {
                return -22; /* -EINVAL */
            }
            let bit = $name##_ATTR_START(cap);
            let word = bit / usize::BITS as usize;
            let shift = bit % usize::BITS as usize;
            *out = core::mem::transmute::<u32, amdgpu_cap_attr>
                ((((*c).bmap[word] >> shift) & AMDGPU_CAP_ATTR_MAX as usize) as u32);
            0
        }

        #[inline]
        pub unsafe fn $name##_cap_is_ro(c: *const $name##_caps, id: $name##_cap_id) -> bool {
            let mut a = amdgpu_cap_attr::AMDGPU_CAP_ATTR_INVALID;
            $name##_attr_get(c, id, &mut a) == 0 && a == amdgpu_cap_attr::AMDGPU_CAP_ATTR_RO
        }
        #[inline]
        pub unsafe fn $name##_cap_is_wo(c: *const $name##_caps, id: $name##_cap_id) -> bool {
            let mut a = amdgpu_cap_attr::AMDGPU_CAP_ATTR_INVALID;
            $name##_attr_get(c, id, &mut a) == 0 && a == amdgpu_cap_attr::AMDGPU_CAP_ATTR_WO
        }
        #[inline]
        pub unsafe fn $name##_cap_is_rw(c: *const $name##_caps, id: $name##_cap_id) -> bool {
            let mut a = amdgpu_cap_attr::AMDGPU_CAP_ATTR_INVALID;
            $name##_attr_get(c, id, &mut a) == 0 && a == amdgpu_cap_attr::AMDGPU_CAP_ATTR_RW
        }
    };
}

/* Element expander for enum creation. */
#[macro_export]
macro_rules! _CAP_ENUM_ELEM { ($x:ident) => { $x, }; }

/* Public macro: declare enum + helpers from an X-macro list. */
#[macro_export]
macro_rules! DECLARE_ATTR_CAP_CLASS {
    ($name:ident, $list:ident) => {
        #[repr(usize)]
        #[derive(Copy, Clone, PartialEq, Eq)]
        pub enum $name##_cap_id { $list!(_CAP_ENUM_ELEM) $name##_COUNT }
        DECLARE_ATTR_CAP_CLASS_HELPERS!($name);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
