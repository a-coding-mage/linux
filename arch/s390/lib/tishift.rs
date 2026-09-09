// SPDX-License-Identifier: GPL-2.0

// C dependencies: <linux/export.h>, <linux/types.h>, and "tishift.h".

#[repr(C)]
#[derive(Copy, Clone)]
struct TiParts {
    high: u64,
    low: u64,
}

#[repr(C)]
union Ti {
    val: i128,
    parts: TiParts,
}

// noinstr
pub unsafe fn __ashlti3(a: i128, shift: i32) -> i128 {
    let mut ti = Ti { val: a };

    if shift == 0 {
        return ti.val;
    }
    if shift < 64 {
        ti.parts = TiParts {
            high: (ti.parts.high << shift) | (ti.parts.low >> (64 - shift)),
            low: ti.parts.low << shift,
        };
    } else {
        ti.parts = TiParts {
            high: ti.parts.low << (shift - 64),
            low: 0,
        };
    }
    ti.val
}

// EXPORT_SYMBOL(__ashlti3);

// noinstr
pub unsafe fn __ashrti3(a: i128, shift: i32) -> i128 {
    let mut ti = Ti { val: a };

    if shift == 0 {
        return ti.val;
    }
    if shift < 64 {
        ti.parts = TiParts {
            low: (ti.parts.low >> shift) | (ti.parts.high << (64 - shift)),
            high: (ti.parts.high as i64 >> shift) as u64,
        };
    } else {
        ti.parts = TiParts {
            low: (ti.parts.high as i64 >> (shift - 64)) as u64,
            high: (ti.parts.high as i64 >> 63) as u64,
        };
    }
    ti.val
}

// EXPORT_SYMBOL(__ashrti3);

// noinstr
pub unsafe fn __lshrti3(a: i128, shift: i32) -> i128 {
    let mut ti = Ti { val: a };

    if shift == 0 {
        return ti.val;
    }
    if shift < 64 {
        ti.parts = TiParts {
            low: (ti.parts.low >> shift) | (ti.parts.high << (64 - shift)),
            high: ti.parts.high >> shift,
        };
    } else {
        ti.parts = TiParts {
            low: ti.parts.high >> (shift - 64),
            high: 0,
        };
    }
    ti.val
}

// EXPORT_SYMBOL(__lshrti3);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
