// SPDX-License-Identifier: GPL-2.0+

/*
 * Add an IPMI platform device.
 */

// C includes are supplied by the surrounding kernel translation unit.

extern "C" {
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn platform_device_alloc(name: *const i8, inst: u32) -> *mut platform_device;
    fn platform_device_add_resources(
        pdev: *mut platform_device,
        r: *mut resource,
        num: u32,
    ) -> i32;
    fn device_create_managed_software_node(
        dev: *mut device,
        properties: *mut property_entry,
        groups: *mut core::ffi::c_void,
    ) -> i32;
    fn platform_device_add(pdev: *mut platform_device) -> i32;
    fn platform_device_put(pdev: *mut platform_device);
    fn pr_err(fmt: *const i8, ...);
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn property_entry_u8(name: *const i8, value: u8) -> property_entry;
    fn property_entry_u16(name: *const i8, value: u16) -> property_entry;
}

#[repr(C)]
pub struct platform_device { pub dev: device }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct resource {
    pub start: u64,
    pub end: u64,
    pub name: *const i8,
    pub flags: u32,
}
#[repr(C)]
pub struct property_entry { _private: [u8; 0] }

#[repr(C)]
pub struct ipmi_plat_data {
    pub iftype: u32,
    pub type_: u8,
    pub regsize: u8,
    pub regspacing: u8,
    pub addr: u16,
    pub slave_addr: u8,
    pub addr_source: u8,
    pub regshift: u8,
    pub space: u32,
    pub irq: u64,
}

extern "C" {
    static IPMI_PLAT_IF_SI: u32;
    static IPMI_PLAT_IF_SSIF: u32;
    static SI_BT: u8;
    static SI_TYPE_INVALID: u8;
    static DEFAULT_REGSIZE: u8;
    static IPMI_IO_ADDR_SPACE: u32;
    static IORESOURCE_IO: u32;
    static IORESOURCE_MEM: u32;
    static IORESOURCE_IRQ: u32;
}

#[no_mangle]
pub unsafe extern "C" fn ipmi_platform_add(
    name: *const i8,
    inst: u32,
    p: *mut ipmi_plat_data,
) -> *mut platform_device {
    let mut pdev: *mut platform_device;
    let mut num_r: u32 = 1;
    let mut size: u32 = 0;
    let mut pidx: usize = 0;
    let mut r: [resource; 4] = core::mem::zeroed();
    let mut pr: [property_entry; 6] = core::mem::zeroed();
    let mut flags: u32;
    let mut rv: i32;

    memset(pr.as_mut_ptr() as *mut core::ffi::c_void, 0, core::mem::size_of_val(&pr));
    memset(r.as_mut_ptr() as *mut core::ffi::c_void, 0, core::mem::size_of_val(&r));

    if (*p).iftype == IPMI_PLAT_IF_SI {
        if (*p).type_ == SI_BT {
            size = 3;
        } else if (*p).type_ != SI_TYPE_INVALID {
            size = 2;
        }
        if (*p).regsize == 0 { (*p).regsize = DEFAULT_REGSIZE; }
        if (*p).regspacing == 0 { (*p).regspacing = (*p).regsize; }
        pr[pidx] = property_entry_u8(b"ipmi-type\0".as_ptr() as *const i8, (*p).type_); pidx += 1;
    } else if (*p).iftype == IPMI_PLAT_IF_SSIF {
        pr[pidx] = property_entry_u16(b"i2c-addr\0".as_ptr() as *const i8, (*p).addr); pidx += 1;
    }
    if (*p).slave_addr != 0 { pr[pidx] = property_entry_u8(b"slave-addr\0".as_ptr() as *const i8, (*p).slave_addr); pidx += 1; }
    pr[pidx] = property_entry_u8(b"addr-source\0".as_ptr() as *const i8, (*p).addr_source); pidx += 1;
    if (*p).regshift != 0 { pr[pidx] = property_entry_u8(b"reg-shift\0".as_ptr() as *const i8, (*p).regshift); pidx += 1; }
    pr[pidx] = property_entry_u8(b"reg-size\0".as_ptr() as *const i8, (*p).regsize);
    // Last entry must be left NULL to terminate it.

    pdev = platform_device_alloc(name, inst);
    if pdev.is_null() { pr_err(b"Error allocating IPMI platform device %s.%d\n\0".as_ptr() as *const i8, name, inst); return core::ptr::null_mut(); }
    if size == 0 { return goto_add_properties(pdev, pr.as_mut_ptr()); }
    flags = if (*p).space == IPMI_IO_ADDR_SPACE { IORESOURCE_IO } else { IORESOURCE_MEM };
    r[0] = resource { start: (*p).addr as u64, end: (*p).addr as u64 + (*p).regsize as u64 - 1, name: b"IPMI Address 1\0".as_ptr() as *const i8, flags };
    if size > 1 { r[1] = resource { start: r[0].start + (*p).regspacing as u64, end: r[0].start + (*p).regspacing as u64 + (*p).regsize as u64 - 1, name: b"IPMI Address 2\0".as_ptr() as *const i8, flags }; num_r += 1; }
    if size > 2 { r[2] = resource { start: r[1].start + (*p).regspacing as u64, end: r[1].start + (*p).regspacing as u64 + (*p).regsize as u64 - 1, name: b"IPMI Address 3\0".as_ptr() as *const i8, flags }; num_r += 1; }
    if (*p).irq != 0 { r[num_r as usize] = resource { start: (*p).irq, end: (*p).irq, name: b"IPMI IRQ\0".as_ptr() as *const i8, flags: IORESOURCE_IRQ }; num_r += 1; }
    rv = platform_device_add_resources(pdev, r.as_mut_ptr(), num_r);
    if rv != 0 { dev_err(&mut (*pdev).dev, b"Unable to add hard-code resources: %d\n\0".as_ptr() as *const i8, rv); platform_device_put(pdev); return core::ptr::null_mut(); }
    goto_add_properties(pdev, pr.as_mut_ptr())
}

unsafe fn goto_add_properties(pdev: *mut platform_device, pr: *mut property_entry) -> *mut platform_device {
    let mut rv = device_create_managed_software_node(&mut (*pdev).dev, pr, core::ptr::null_mut());
    if rv != 0 { dev_err(&mut (*pdev).dev, b"Unable to add hard-code properties: %d\n\0".as_ptr() as *const i8, rv); platform_device_put(pdev); return core::ptr::null_mut(); }
    rv = platform_device_add(pdev);
    if rv != 0 { dev_err(&mut (*pdev).dev, b"Unable to add hard-code device: %d\n\0".as_ptr() as *const i8, rv); platform_device_put(pdev); return core::ptr::null_mut(); }
    pdev
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
