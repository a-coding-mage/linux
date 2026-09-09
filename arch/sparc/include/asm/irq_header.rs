/* SPDX-License-Identifier: GPL-2.0 */

// C source selected <asm/irq_64.h> when __sparc__ and __arch64__ were
// defined; otherwise it selected <asm/irq_32.h>. The selected declarations
// are supplied by the corresponding Rust dependency/module.
#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
// Source dependency: asm/irq_64.h

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
// Source dependency: asm/irq_32.h

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
