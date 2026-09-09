// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Support for 'mpc5200-simple-platform' compatible boards.
 *
 * Written by Marian Balakowicz <m8@semihalf.com>
 * Copyright (C) 2007 Semihalf
 *
 * Description:
 * This code implements support for a simple MPC52xx based boards which
 * do not need a custom platform specific setup. Such boards are
 * supported assuming the following:
 *
 * - GPIO pins are configured by the firmware,
 * - CDM configuration (clocking) is setup correctly by firmware,
 * - if the 'fsl,has-wdt' property is present in one of the
 *   gpt nodes, then it is safe to use such gpt to reset the board,
 * - PCI is supported if enabled in the kernel configuration
 *   and if there is a PCI bus node defined in the device tree.
 *
 * Boards that are compatible with this generic platform support
 * are listed in a 'board' table.
 */

// C header dependencies are supplied by other translation units.

#[repr(C)]
pub struct PpcMd {
    pub progress: Option<unsafe extern "C" fn(*const core::ffi::c_char, i32)>,
}

extern "C" {
    static mut ppc_md: PpcMd;
    fn mpc52xx_map_common_devices();
    fn mpc5200_setup_xlb_arbiter();
    fn mpc52xx_setup_pci();
    fn mpc52xx_declare_of_platform_devices();
    fn mpc52xx_init_irq();
    fn mpc52xx_get_irq() -> i32;
    fn mpc52xx_restart();
}

/*
 * Setup the architecture
 */
unsafe extern "C" fn mpc5200_simple_setup_arch() {
    if let Some(progress) = ppc_md.progress {
        progress(b"mpc5200_simple_setup_arch()\0".as_ptr() as *const core::ffi::c_char, 0);
    }

    /* Map important registers from the internal memory map */
    mpc52xx_map_common_devices();

    /* Some mpc5200 & mpc5200b related configuration */
    mpc5200_setup_xlb_arbiter();
}

/* list of the supported boards */
static BOARD: [Option<&'static core::ffi::c_char>; 13] = [
    Some(b"anonymous,a3m071\0".as_ptr() as *const core::ffi::c_char),
    Some(b"anonymous,a4m072\0".as_ptr() as *const core::ffi::c_char),
    Some(b"anon,charon\0".as_ptr() as *const core::ffi::c_char),
    Some(b"ifm,o2d\0".as_ptr() as *const core::ffi::c_char),
    Some(b"intercontrol,digsy-mtc\0".as_ptr() as *const core::ffi::c_char),
    Some(b"manroland,mucmc52\0".as_ptr() as *const core::ffi::c_char),
    Some(b"manroland,uc101\0".as_ptr() as *const core::ffi::c_char),
    Some(b"phytec,pcm030\0".as_ptr() as *const core::ffi::c_char),
    Some(b"phytec,pcm032\0".as_ptr() as *const core::ffi::c_char),
    Some(b"promess,motionpro\0".as_ptr() as *const core::ffi::c_char),
    Some(b"schindler,cm5200\0".as_ptr() as *const core::ffi::c_char),
    Some(b"tqc,tqm5200\0".as_ptr() as *const core::ffi::c_char),
    None,
];

#[repr(C)]
pub struct MachineDesc {
    pub name: *const core::ffi::c_char,
    pub compatibles: *const Option<&'static core::ffi::c_char>,
    pub setup_arch: Option<unsafe extern "C" fn()>,
    pub discover_phbs: Option<unsafe extern "C" fn()>,
    pub init: Option<unsafe extern "C" fn()>,
    pub init_irq: Option<unsafe extern "C" fn()>,
    pub get_irq: Option<unsafe extern "C" fn() -> i32>,
    pub restart: Option<unsafe extern "C" fn()>,
}

#[no_mangle]
pub static mpc5200_simple_platform: MachineDesc = MachineDesc {
    .name: b"mpc5200-simple-platform\0".as_ptr() as *const core::ffi::c_char,
    .compatibles: BOARD.as_ptr(),
    .setup_arch: Some(mpc5200_simple_setup_arch),
    .discover_phbs: Some(mpc52xx_setup_pci),
    .init: Some(mpc52xx_declare_of_platform_devices),
    .init_irq: Some(mpc52xx_init_irq),
    .get_irq: Some(mpc52xx_get_irq),
    .restart: Some(mpc52xx_restart),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
