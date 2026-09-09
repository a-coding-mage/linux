/*
 * Copyright (C) 2013 Altera Corporation
 * Copyright (C) 2011 Thomas Chou
 * Copyright (C) 2011 Walter Goossens
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License. See the file COPYING in the main directory of this
 * archive for more details.
 */

// C dependencies supplied by the surrounding kernel environment:
// linux/init.h, linux/of_address.h, linux/of_fdt.h, linux/err.h,
// linux/slab.h, linux/sys_soc.h, linux/io.h, linux/clk-provider.h

#[repr(C)]
struct OfDeviceId {
    compatible: *const core::ffi::c_char,
    data: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
struct SocDeviceAttribute {
    machine: *mut core::ffi::c_char,
    family: *const core::ffi::c_char,
}

#[repr(C)]
struct SocDevice {
    _private: [u8; 0],
}

unsafe extern "C" {
    static of_fixed_clk_setup: unsafe extern "C" fn();

    fn kzalloc_obj<T>() -> *mut T;
    fn of_flat_dt_get_machine_name() -> *const core::ffi::c_char;
    fn kasprintf(gfp: usize, format: *const core::ffi::c_char, ...) -> *mut core::ffi::c_char;
    fn soc_device_register(attr: *mut SocDeviceAttribute) -> *mut SocDevice;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn of_clk_init(match_table: *const OfDeviceId);
}

const GFP_KERNEL: usize = 0;

static CLK_MATCH: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"fixed-clock\0".as_ptr() as *const core::ffi::c_char,
        data: Some(of_fixed_clk_setup),
    },
    OfDeviceId {
        compatible: core::ptr::null(),
        data: None,
    },
];

unsafe extern "C" fn nios2_soc_device_init() -> i32 {
    let mut soc_dev: *mut SocDevice;
    let soc_dev_attr: *mut SocDeviceAttribute;
    let machine: *const core::ffi::c_char;

    soc_dev_attr = kzalloc_obj::<SocDeviceAttribute>();
    if !soc_dev_attr.is_null() {
        machine = of_flat_dt_get_machine_name();
        if !machine.is_null() {
            (*soc_dev_attr).machine = kasprintf(GFP_KERNEL, b"%s\0".as_ptr() as *const core::ffi::c_char, machine);
        }

        (*soc_dev_attr).family = b"Nios II\0".as_ptr() as *const core::ffi::c_char;

        soc_dev = soc_device_register(soc_dev_attr);
        if (soc_dev as isize) < 0 && (soc_dev as isize) >= -4095 {
            kfree((*soc_dev_attr).machine as *mut core::ffi::c_void);
            kfree(soc_dev_attr as *mut core::ffi::c_void);
        }
    }

    of_clk_init(CLK_MATCH.as_ptr());

    0
}

// device_initcall(nios2_soc_device_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
