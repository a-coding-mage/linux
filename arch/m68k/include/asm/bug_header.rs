/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_MMU, CONFIG_BUG, CONFIG_DEBUG_BUGVERBOSE, and CONFIG_SUN3 are
// build-time conditions from the original C header.

#[cfg(all(CONFIG_MMU, CONFIG_BUG))]
#[macro_export]
macro_rules! BUG {
    () => {{
        #[cfg(CONFIG_DEBUG_BUGVERBOSE)]
        {
            #[cfg(not(CONFIG_SUN3))]
            {
                pr_crit!("kernel BUG at {}:{}!\n", file!(), line!());
                barrier_before_unreachable();
                unsafe { core::intrinsics::abort() }
            }
            #[cfg(CONFIG_SUN3)]
            {
                pr_crit!("kernel BUG at {}:{}!\n", file!(), line!());
                barrier_before_unreachable();
                panic!("BUG!");
            }
        }
        #[cfg(not(CONFIG_DEBUG_BUGVERBOSE))]
        {
            barrier_before_unreachable();
            unsafe { core::intrinsics::abort() }
        }
    }};
}

#[cfg(all(CONFIG_MMU, CONFIG_BUG))]
pub const HAVE_ARCH_BUG: bool = true;

// The original header includes <asm-generic/bug.h>; its declarations are
// supplied by the corresponding Rust dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
