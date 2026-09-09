/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PixelFormatBitfield {
    pub offset: u8,
    pub length: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PixelFormatColorFields {
    pub alpha: PixelFormatBitfield,
    pub red: PixelFormatBitfield,
    pub green: PixelFormatBitfield,
    pub blue: PixelFormatBitfield,
}

#[repr(C)]
pub union PixelFormatFields {
    pub color: PixelFormatColorFields,
    pub index: PixelFormatBitfield,
}

#[repr(C)]
pub struct PixelFormat {
    pub bits_per_pixel: u8,
    pub indexed: bool,
    pub fields: PixelFormatFields,
}

pub const PIXEL_FORMAT_C8: PixelFormat = PixelFormat {
    bits_per_pixel: 8,
    indexed: true,
    fields: PixelFormatFields {
        index: PixelFormatBitfield { offset: 0, length: 8 },
    },
};

pub const PIXEL_FORMAT_XRGB1555: PixelFormat = PixelFormat {
    bits_per_pixel: 16,
    indexed: false,
    fields: PixelFormatFields {
        color: PixelFormatColorFields {
            alpha: PixelFormatBitfield { offset: 0, length: 0 },
            red: PixelFormatBitfield { offset: 10, length: 5 },
            green: PixelFormatBitfield { offset: 5, length: 5 },
            blue: PixelFormatBitfield { offset: 0, length: 5 },
        },
    },
};

pub const PIXEL_FORMAT_RGB565: PixelFormat = PixelFormat {
    bits_per_pixel: 16,
    indexed: false,
    fields: PixelFormatFields {
        color: PixelFormatColorFields {
            alpha: PixelFormatBitfield { offset: 0, length: 0 },
            red: PixelFormatBitfield { offset: 11, length: 5 },
            green: PixelFormatBitfield { offset: 5, length: 6 },
            blue: PixelFormatBitfield { offset: 0, length: 5 },
        },
    },
};

pub const PIXEL_FORMAT_RGB888: PixelFormat = PixelFormat {
    bits_per_pixel: 24,
    indexed: false,
    fields: PixelFormatFields {
        color: PixelFormatColorFields {
            alpha: PixelFormatBitfield { offset: 0, length: 0 },
            red: PixelFormatBitfield { offset: 16, length: 8 },
            green: PixelFormatBitfield { offset: 8, length: 8 },
            blue: PixelFormatBitfield { offset: 0, length: 8 },
        },
    },
};

pub const PIXEL_FORMAT_XRGB8888: PixelFormat = PixelFormat {
    bits_per_pixel: 32,
    indexed: false,
    fields: PixelFormatFields {
        color: PixelFormatColorFields {
            alpha: PixelFormatBitfield { offset: 0, length: 0 },
            red: PixelFormatBitfield { offset: 16, length: 8 },
            green: PixelFormatBitfield { offset: 8, length: 8 },
            blue: PixelFormatBitfield { offset: 0, length: 8 },
        },
    },
};

pub const PIXEL_FORMAT_XBGR8888: PixelFormat = PixelFormat {
    bits_per_pixel: 32,
    indexed: false,
    fields: PixelFormatFields {
        color: PixelFormatColorFields {
            alpha: PixelFormatBitfield { offset: 0, length: 0 },
            red: PixelFormatBitfield { offset: 0, length: 8 },
            green: PixelFormatBitfield { offset: 8, length: 8 },
            blue: PixelFormatBitfield { offset: 16, length: 8 },
        },
    },
};

pub const PIXEL_FORMAT_XRGB2101010: PixelFormat = PixelFormat {
    bits_per_pixel: 32,
    indexed: false,
    fields: PixelFormatFields {
        color: PixelFormatColorFields {
            alpha: PixelFormatBitfield { offset: 0, length: 0 },
            red: PixelFormatBitfield { offset: 20, length: 10 },
            green: PixelFormatBitfield { offset: 10, length: 10 },
            blue: PixelFormatBitfield { offset: 0, length: 10 },
        },
    },
};

/**
 * pixel_format_cmp - Compares two pixel-format descriptions
 *
 * @lhs: a pixel-format description
 * @rhs: a pixel-format description
 *
 * Compares two pixel-format descriptions for their order. The semantics
 * are equivalent to memcmp().
 *
 * Returns:
 * 0 if both arguments describe the same pixel format, less-than-zero if lhs < rhs,
 * or greater-than-zero if lhs > rhs.
 */
pub unsafe fn pixel_format_cmp(lhs: *const PixelFormat, rhs: *const PixelFormat) -> i32 {
    macro_rules! cmp_field {
        ($left:expr, $right:expr) => {{
            let ret = ($left as i32) - ($right as i32);
            if ret != 0 {
                return ret;
            }
        }};
    }
    macro_rules! cmp_bitfield {
        ($left:expr, $right:expr) => {{
            cmp_field!($left.offset, $right.offset);
            cmp_field!($left.length, $right.length);
        }};
    }

    let lhs_ref = &*lhs;
    let rhs_ref = &*rhs;
    cmp_field!(lhs_ref.bits_per_pixel, rhs_ref.bits_per_pixel);
    cmp_field!(lhs_ref.indexed, rhs_ref.indexed);

    if lhs_ref.indexed {
        cmp_bitfield!(lhs_ref.fields.index, rhs_ref.fields.index);
    } else {
        let lhs_color = lhs_ref.fields.color;
        let rhs_color = rhs_ref.fields.color;
        cmp_bitfield!(lhs_color.alpha, rhs_color.alpha);
        cmp_bitfield!(lhs_color.red, rhs_color.red);
        cmp_bitfield!(lhs_color.green, rhs_color.green);
        cmp_bitfield!(lhs_color.blue, rhs_color.blue);
    }

    0
}

/**
 * pixel_format_equal - Compares two pixel-format descriptions for equality
 *
 * @lhs: a pixel-format description
 * @rhs: a pixel-format description
 *
 * Returns:
 * True if both arguments describe the same pixel format, or false otherwise.
 */
pub unsafe fn pixel_format_equal(lhs: *const PixelFormat, rhs: *const PixelFormat) -> bool {
    pixel_format_cmp(lhs, rhs) == 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
