/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header guard: __ASM_BUG_H
 *
 * The CONFIG_BUG and _MIPS_ISA build-time conditions from the source are
 * preserved below as Rust configuration/intent comments where this isolated
 * header cannot determine their values.
 */

/* CONFIG_BUG */

/// Trigger the architecture-specific BUG trap and does not return.
#[inline(always)]
pub unsafe fn BUG() -> ! {
    core::arch::asm!("break {0}", const BRK_BUG);
    core::hint::unreachable_unchecked()
}

/// Architecture BUG support is available.
pub const HAVE_ARCH_BUG: bool = true;

/* _MIPS_ISA > _MIPS_ISA_MIPS1 */

/// Architecture-specific BUG_ON implementation.
#[inline(always)]
pub unsafe fn __BUG_ON(condition: u64) {
    // The C source uses __builtin_constant_p(condition) to fold constants.
    // Rust has no file-local equivalent for that compiler predicate; the
    // runtime trap operation is retained directly.
    core::arch::asm!(
        "tne $0, {condition}, {brk_bug}",
        condition = in(reg) condition,
        brk_bug = const BRK_BUG,
    );
}

/// Invoke __BUG_ON after converting the expression to unsigned long intent.
#[macro_export]
macro_rules! BUG_ON {
    ($condition:expr) => {{
        unsafe { $crate::__BUG_ON(($condition) as u64) }
    }};
}

/// Architecture-specific BUG_ON support is available.
pub const HAVE_ARCH_BUG_ON: bool = true;

/* asm-generic/bug.h declarations are supplied by the surrounding build. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
