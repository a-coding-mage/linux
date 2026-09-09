// SPDX-License-Identifier: GPL-2.0-only
/*
 * ARTPEC-6 device support.
 */

// External kernel dependencies supplied by other translation units.
use core::ffi::c_char;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct arm_smccc_res {
    pub a0: u64,
    pub a1: u64,
    pub a2: u64,
    pub a3: u64,
}

unsafe extern "C" {
    fn syscon_regmap_lookup_by_compatible(compatible: *const c_char) -> *mut regmap;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn arm_smccc_smc(
        a0: u64,
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        res: *mut arm_smccc_res,
    );
    fn warn_on(condition: bool) -> bool;
}

pub const ARTPEC6_DMACFG_REGNUM: u32 = 0x10;
pub const ARTPEC6_DMACFG_UARTS_BURST: u32 = 0xff;

pub const SECURE_OP_L2C_WRITEREG: u64 = 0xb4000001;

unsafe fn artpec6_init_machine() {
    let regmap = syscon_regmap_lookup_by_compatible(b"axis,artpec6-syscon\0".as_ptr() as *const c_char);

    // Equivalent to !IS_ERR(regmap); the kernel error-pointer convention is
    // supplied by the surrounding kernel translation.
    if !regmap.is_null() {
        /* Use PL011 DMA Burst Request signal instead of DMA
         * Single Request
         */
        regmap_write(
            regmap,
            ARTPEC6_DMACFG_REGNUM,
            ARTPEC6_DMACFG_UARTS_BURST,
        );
    }
}

unsafe fn artpec6_l2c310_write_sec(val: usize, reg: u32) {
    let mut res = arm_smccc_res {
        a0: 0,
        a1: 0,
        a2: 0,
        a3: 0,
    };

    arm_smccc_smc(
        SECURE_OP_L2C_WRITEREG,
        reg as u64,
        val as u64,
        0,
        0,
        0,
        0,
        0,
        &mut res,
    );

    warn_on(res.a0 != 0);
}

pub static ARTPEC6_DT_MATCH: [*const c_char; 2] = [
    b"axis,artpec6\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(ARTPEC6, "Axis ARTPEC-6 Platform")
// MACHINE_END
// The architecture-specific machine descriptor is represented by the
// following source-level configuration values and callbacks.
#[repr(C)]
pub struct Artpec6MachineDesc {
    pub l2c_aux_val: u32,
    pub l2c_aux_mask: u32,
    pub l2c_write_sec: unsafe fn(usize, u32),
    pub init_machine: unsafe fn(),
    pub dt_compat: *const *const c_char,
}

pub static mut ARTPEC6_MACHINE: Artpec6MachineDesc = Artpec6MachineDesc {
    l2c_aux_val: 0x0c000000,
    l2c_aux_mask: 0xf3ffffff,
    l2c_write_sec: artpec6_l2c310_write_sec,
    init_machine: artpec6_init_machine,
    dt_compat: ARTPEC6_DT_MATCH.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
