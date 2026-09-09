/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_ARM_MEMBLOCK_H

#[repr(C)]
pub struct machine_desc {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn arm_memblock_init(machine: *const machine_desc);
    pub fn arm_memblock_steal(size: phys_addr_t, align: phys_addr_t) -> phys_addr_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
