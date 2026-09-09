/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by bcm63xx_cpu.h in the original C header.

#[inline]
pub unsafe fn is_bcm63xx_internal_registers(offset: phys_addr_t) -> i32 {
    match bcm63xx_get_cpu_id() {
        BCM3368_CPU_ID => {
            if offset >= 0xfff80000 {
                return 1;
            }
        }
        BCM6338_CPU_ID | BCM6345_CPU_ID | BCM6348_CPU_ID | BCM6358_CPU_ID => {
            if offset >= 0xfff00000 {
                return 1;
            }
        }
        BCM6328_CPU_ID | BCM6362_CPU_ID | BCM6368_CPU_ID => {
            if offset >= 0xb0000000 && offset < 0xb1000000 {
                return 1;
            }
        }
        _ => {}
    }
    0
}

#[inline]
pub unsafe fn plat_ioremap(
    offset: phys_addr_t,
    size: ::core::primitive::usize,
    flags: ::core::primitive::usize,
) -> *mut ::core::ffi::c_void {
    let _ = (size, flags);
    if is_bcm63xx_internal_registers(offset) != 0 {
        return offset as *mut ::core::ffi::c_void;
    }
    ::core::ptr::null_mut()
}

#[inline]
pub unsafe fn plat_iounmap(addr: *const ::core::ffi::c_void) -> i32 {
    is_bcm63xx_internal_registers(addr as ::core::primitive::usize as phys_addr_t)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
