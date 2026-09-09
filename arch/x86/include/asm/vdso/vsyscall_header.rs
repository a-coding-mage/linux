/* SPDX-License-Identifier: GPL-2.0 */

pub const __VDSO_PAGES: usize = 6;

pub const VDSO_NR_VCLOCK_PAGES: usize = 2;

// Direct translation of VDSO_VCLOCK_PAGES_START(_b).
#[macro_export]
macro_rules! VDSO_VCLOCK_PAGES_START {
    ($b:expr) => {
        ($b) + (__VDSO_PAGES - VDSO_NR_VCLOCK_PAGES) * PAGE_SIZE
    };
}

pub const VDSO_PAGE_PVCLOCK_OFFSET: usize = 0;
pub const VDSO_PAGE_HVCLOCK_OFFSET: usize = 1;

// C dependencies supplied by the surrounding build:
// <vdso/datapage.h>
// <asm/vgtod.h>
// The asm-generic header is included after the definitions above:
// <asm-generic/vdso/vsyscall.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
