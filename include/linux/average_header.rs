/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation unit:
 * linux/bug.h, linux/compiler.h, and linux/log2.h.
 */

/*
 * Exponentially weighted moving average (EWMA)
 *
 * This implements a fixed-precision EWMA algorithm, with both the
 * precision and fall-off coefficient determined at compile-time
 * and built into the generated helper functions.
 */

/* Rust equivalent of DECLARE_EWMA.  The generated module is named after
 * `name`; its `Ewma` type and helpers correspond to ewma_##name and the
 * ewma_##name_{init,read,add} functions.
 */
macro_rules! declare_ewma {
    ($name:ident, $precision:expr, $weight_rcp:expr) => {
        pub mod $name {
            #[repr(C)]
            pub struct Ewma {
                pub internal: usize,
            }

            #[inline]
            pub unsafe fn init(e: *mut Ewma) {
                const _: () = assert!($precision <= 30);
                const _: () = assert!($weight_rcp != 0 && ($weight_rcp & ($weight_rcp - 1)) == 0);
                (*e).internal = 0;
            }

            #[inline]
            pub unsafe fn read(e: *mut Ewma) -> usize {
                const _: () = assert!($precision <= 30);
                const _: () = assert!($weight_rcp != 0 && ($weight_rcp & ($weight_rcp - 1)) == 0);
                core::ptr::read_volatile(&(*e).internal) >> $precision
            }

            #[inline]
            pub unsafe fn add(e: *mut Ewma, val: usize) {
                let internal = core::ptr::read_volatile(&(*e).internal);
                let weight_rcp = ($weight_rcp as usize).ilog2() as usize;
                let precision = $precision as usize;

                const _: () = assert!($precision <= 30);
                const _: () = assert!($weight_rcp != 0 && ($weight_rcp & ($weight_rcp - 1)) == 0);

                core::ptr::write_volatile(
                    &mut (*e).internal,
                    if internal != 0 {
                        (((internal << weight_rcp).wrapping_sub(internal))
                            .wrapping_add(val << precision))
                            >> weight_rcp
                    } else {
                        val << precision
                    },
                );
            }
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
