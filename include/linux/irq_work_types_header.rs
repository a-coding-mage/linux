/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <linux/smp_types.h> and <linux/types.h>:
// __call_single_node and rcuwait are supplied by those headers.

#[repr(C)]
pub struct irq_work {
    pub node: __call_single_node,
    pub func: Option<unsafe extern "C" fn(work: *mut irq_work)>,
    pub irqwait: rcuwait,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
