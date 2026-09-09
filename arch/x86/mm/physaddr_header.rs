/* SPDX-License-Identifier: GPL-2.0 */
// Dependency intent: <asm/processor.h>

#[cfg(CONFIG_PHYS_ADDR_T_64BIT)]
#[inline]
pub unsafe fn phys_addr_valid(addr: resource_size_t) -> i32 {
    if (addr >> boot_cpu_data.x86_phys_bits) == 0 {
        1
    } else {
        0
    }
}

#[cfg(not(CONFIG_PHYS_ADDR_T_64BIT))]
#[inline]
pub unsafe fn phys_addr_valid(_addr: resource_size_t) -> i32 {
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
