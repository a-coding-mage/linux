/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _PPC_KERNEL_CPM2_H

// Supplied by the corresponding dependency.
pub struct device_node;

extern "C" {
    pub fn cpm2_get_irq() -> u32;

    pub fn cpm2_pic_init(node: *mut device_node);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
