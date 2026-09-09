/*
 * Broadcom BCM470X / BCM5301X ARM platform code.
 *
 * Copyright 2013 Hauke Mehrtens <hauke@hauke-m.de>
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

// C dependencies supplied by the surrounding kernel translation.

const FSR_EXTERNAL: u32 = 1 << 12;
const FSR_READ: u32 = 0 << 10;
const FSR_IMPRECISE: u32 = 0x0406;

static BCM5301X_DT_COMPAT: [&'static core::ffi::c_char; 2] = [
    b"brcm,bcm4708\0".as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];

#[repr(C)]
pub struct PtRegs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn hook_fault_code(
        nr: i32,
        fnc: unsafe extern "C" fn(
            addr: libc::c_ulong,
            fsr: libc::c_uint,
            regs: *mut PtRegs,
        ) -> i32,
        sig: i32,
        code: i32,
        name: *const libc::c_char,
    );
}

unsafe extern "C" fn bcm5301x_abort_handler(
    _addr: libc::c_ulong,
    fsr: libc::c_uint,
    _regs: *mut PtRegs,
) -> i32 {
    /*
     * We want to ignore aborts forwarded from the PCIe bus that are
     * expected and shouldn't really be passed by the PCIe controller.
     * The biggest disadvantage is the same FSR code may be reported when
     * reading non-existing APB register and we shouldn't ignore that.
     */
    if fsr == (FSR_EXTERNAL | FSR_READ | FSR_IMPRECISE) {
        return 0;
    }

    1
}

unsafe extern "C" fn bcm5301x_init_early() {
    hook_fault_code(
        16 + 6,
        bcm5301x_abort_handler,
        SIGBUS,
        BUS_OBJERR,
        b"imprecise external abort\0".as_ptr() as *const libc::c_char,
    );
}

// DT_MACHINE_START(BCM5301X, "BCM5301X") / MACHINE_END.
// The surrounding ARM platform bindings provide the machine-descriptor type.
#[repr(C)]
pub struct Bcm5301xMachine {
    pub l2c_aux_val: libc::c_ulong,
    pub l2c_aux_mask: libc::c_ulong,
    pub dt_compat: *const *const libc::c_char,
    pub init_early: Option<unsafe extern "C" fn()>,
}

#[no_mangle]
pub static mut BCM5301X_MACHINE: Bcm5301xMachine = Bcm5301xMachine {
    l2c_aux_val: 0,
    l2c_aux_mask: !0,
    dt_compat: BCM5301X_DT_COMPAT.as_ptr(),
    init_early: Some(bcm5301x_init_early),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
