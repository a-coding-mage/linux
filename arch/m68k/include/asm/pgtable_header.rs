/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __M68K_PGTABLE_H

// Dependency: <asm/page.h>

// In the C source, __uClinux__ selects <asm/pgtable_no.h>; otherwise it
// selects <asm/pgtable_mm.h>. The corresponding Rust dependencies are
// supplied externally.

// #ifndef __ASSEMBLER__
unsafe extern "C" {
    pub fn paging_init();
}
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
