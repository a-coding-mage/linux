/*
 * Freescale Memory Controller kernel module
 *
 * Author: York Sun <york.sun@nxp.com>
 *
 * Copyright 2016 NXP Semiconductor
 *
 * Derived from mpc85xx_edac.c
 * Author: Dave Jiang <djiang@mvista.com>
 *
 * 2006-2007 (c) MontaVista Software, Inc. This file is licensed under
 * the terms of the GNU General Public License version 2. This program
 * is licensed "as is" without any warranty of any kind, whether express
 * or implied.
 */

// pr_fmt(fmt) expands to KBUILD_MODNAME ": " fmt.
// Dependencies supplied by edac_module.h and fsl_ddr_edac.h are external.

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const ::core::ffi::c_char,
    pub data: *const ::core::ffi::c_void,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const ::core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub driver: platform_driver_driver,
}

extern "C" {
    static mut edac_op_state: ::core::ffi::c_int;
    fn ghes_get_devices() -> bool;
    fn fsl_mc_err_probe() -> ::core::ffi::c_int;
    fn fsl_mc_err_remove() -> ::core::ffi::c_int;
    fn platform_driver_register(driver: *mut platform_driver) -> ::core::ffi::c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn pr_err(fmt: *const ::core::ffi::c_char, ...);
}

// Values are supplied by the EDAC dependency headers.
const EBUSY: ::core::ffi::c_int = 16;
const EDAC_OPSTATE_POLL: ::core::ffi::c_int = 0;
const EDAC_OPSTATE_INT: ::core::ffi::c_int = 2;
extern "C" {
    static TYPE_IMX9: ::core::ffi::c_int;
}

static mut fsl_ddr_mc_err_of_match: [of_device_id; 3] = [
    of_device_id {
        compatible: b"fsl,qoriq-memory-controller\0".as_ptr() as *const _,
        data: ::core::ptr::null(),
    },
    of_device_id {
        compatible: b"nxp,imx9-memory-controller\0".as_ptr() as *const _,
        data: unsafe { &TYPE_IMX9 as *const _ as *const ::core::ffi::c_void },
    },
    of_device_id {
        compatible: ::core::ptr::null(),
        data: ::core::ptr::null(),
    },
];

static mut fsl_ddr_mc_err_driver: platform_driver = platform_driver {
    probe: Some(fsl_mc_err_probe),
    remove: Some(fsl_mc_err_remove),
    driver: platform_driver_driver {
        name: b"fsl_ddr_mc_err\0".as_ptr() as *const _,
        of_match_table: unsafe { fsl_ddr_mc_err_of_match.as_ptr() },
    },
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fsl_ddr_mc_init() -> ::core::ffi::c_int {
    let res: ::core::ffi::c_int;

    if ghes_get_devices() {
        return -EBUSY;
    }

    /* make sure error reporting method is sane */
    match edac_op_state {
        EDAC_OPSTATE_POLL | EDAC_OPSTATE_INT => {}
        _ => {
            edac_op_state = EDAC_OPSTATE_INT;
        }
    }

    res = platform_driver_register(&mut fsl_ddr_mc_err_driver);
    if res != 0 {
        pr_err(b"MC fails to register\n\0".as_ptr() as *const _);
        return res;
    }

    0
}

// module_init(fsl_ddr_mc_init)

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fsl_ddr_mc_exit() {
    platform_driver_unregister(&mut fsl_ddr_mc_err_driver);
}

// module_exit(fsl_ddr_mc_exit)
// MODULE_DEVICE_TABLE(of, fsl_ddr_mc_err_of_match)
// MODULE_DESCRIPTION("Freescale Layerscape EDAC driver")
// MODULE_LICENSE("GPL")
// MODULE_AUTHOR("NXP Semiconductor")
// module_param(edac_op_state, int, 0444)
// MODULE_PARM_DESC(edac_op_state, "EDAC Error Reporting state: 0=Poll, 2=Interrupt")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
