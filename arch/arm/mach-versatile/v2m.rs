// SPDX-License-Identifier: GPL-2.0
// Translated from v2m.c. Kernel headers and symbols are supplied by other files.

use core::ffi::c_void;

const SYS_FLAGSSET: usize = 0x030;
const SYS_FLAGSCLR: usize = 0x034;

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

extern "C" {
    fn of_find_compatible_node(
        from: *mut DeviceNode,
        ty: *const c_void,
        compatible: *const u8,
    ) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: i32) -> *mut c_void;
    fn writel(value: u32, address: *mut c_void);
    fn warn_on(condition: bool) -> bool;
}

pub unsafe fn vexpress_flags_set(data: u32) {
    static mut BASE: *mut c_void = core::ptr::null_mut();

    if BASE.is_null() {
        let node = of_find_compatible_node(
            core::ptr::null_mut(),
            core::ptr::null(),
            b"arm,vexpress-sysreg\0".as_ptr(),
        );

        BASE = of_iomap(node, 0);
    }

    if warn_on(BASE.is_null()) {
        return;
    }

    writel(u32::MAX, BASE.add(SYS_FLAGSCLR));
    writel(data, BASE.add(SYS_FLAGSSET));
}

pub static V2M_DT_MATCH: [*const u8; 2] = [b"arm,vexpress\0".as_ptr(), core::ptr::null()];

// DT_MACHINE_START(VEXPRESS_DT, "ARM-Versatile Express")
//     .dt_compat = v2m_dt_match,
//     .l2c_aux_val = 0x00400000,
//     .l2c_aux_mask = 0xfe0fffff,
//     .smp = smp_ops(vexpress_smp_dt_ops),
//     .smp_init = smp_init_ops(vexpress_smp_init_ops),
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
