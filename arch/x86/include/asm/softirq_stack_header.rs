/* SPDX-License-Identifier: GPL-2.0 */

// x86_64 builds depend on the declarations from <asm/irq_stack.h>.
// Non-x86_64 builds depend on the declarations from
// <asm-generic/softirq_stack.h>.
#[cfg(target_arch = "x86_64")]
use crate::asm::irq_stack::*;

#[cfg(not(target_arch = "x86_64"))]
use crate::asm_generic::softirq_stack::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
