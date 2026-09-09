// SPDX-License-Identifier: GPL-2.0-or-later
/* Procedures for creating, accessing and interpreting the device tree. */

// C includes and build-time annotations are supplied by the surrounding kernel.

pub unsafe fn prom_early_alloc(size: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void {
    let ret = memblock_alloc_or_panic(size, SMP_CACHE_BYTES);
    prom_early_allocated = prom_early_allocated.wrapping_add(size);
    ret
}

unsafe fn sparc32_path_component(dp: *mut device_node, tmp_buf: *mut ::core::ffi::c_char) {
    let name = of_get_property(dp, b"name\0".as_ptr() as _, core::ptr::null_mut());
    let rprop = of_find_property(dp, b"reg\0".as_ptr() as _, core::ptr::null_mut());
    if rprop.is_null() { return; }
    let regs = (*rprop).value as *mut linux_prom_registers;
    sprintf(tmp_buf, b"%s@%x,%x\0".as_ptr() as _, name, (*regs).which_io, (*regs).phys_addr);
}

unsafe fn sbus_path_component(dp: *mut device_node, tmp_buf: *mut ::core::ffi::c_char) {
    let name = of_get_property(dp, b"name\0".as_ptr() as _, core::ptr::null_mut());
    let prop = of_find_property(dp, b"reg\0".as_ptr() as _, core::ptr::null_mut());
    if prop.is_null() { return; }
    let regs = (*prop).value as *mut linux_prom_registers;
    sprintf(tmp_buf, b"%s@%x,%x\0".as_ptr() as _, name, (*regs).which_io, (*regs).phys_addr);
}

unsafe fn pci_path_component(dp: *mut device_node, tmp_buf: *mut ::core::ffi::c_char) {
    let name = of_get_property(dp, b"name\0".as_ptr() as _, core::ptr::null_mut());
    let prop = of_find_property(dp, b"reg\0".as_ptr() as _, core::ptr::null_mut());
    if prop.is_null() { return; }
    let regs = (*prop).value as *mut linux_prom_pci_registers;
    let devfn = ((*regs).phys_hi >> 8) & 0xff;
    if devfn & 0x07 != 0 {
        sprintf(tmp_buf, b"%s@%x,%x\0".as_ptr() as _, name, devfn >> 3, devfn & 0x07);
    } else {
        sprintf(tmp_buf, b"%s@%x\0".as_ptr() as _, name, devfn >> 3);
    }
}

unsafe fn ebus_path_component(dp: *mut device_node, tmp_buf: *mut ::core::ffi::c_char) {
    let name = of_get_property(dp, b"name\0".as_ptr() as _, core::ptr::null_mut());
    let prop = of_find_property(dp, b"reg\0".as_ptr() as _, core::ptr::null_mut());
    if prop.is_null() { return; }
    let regs = (*prop).value as *mut linux_prom_registers;
    sprintf(tmp_buf, b"%s@%x,%x\0".as_ptr() as _, name, (*regs).which_io, (*regs).phys_addr);
}

unsafe fn ambapp_path_component(dp: *mut device_node, tmp_buf: *mut ::core::ffi::c_char) {
    let name = of_get_property(dp, b"name\0".as_ptr() as _, core::ptr::null_mut());
    let mut interrupt: ::core::ffi::c_int = 0;
    let prop = of_find_property(dp, b"reg\0".as_ptr() as _, core::ptr::null_mut());
    let reg0 = if prop.is_null() { (*dp).phandle as u32 } else { (*( (*prop).value as *mut amba_prom_registers)).phys_addr };
    let prop = of_find_property(dp, b"interrupts\0".as_ptr() as _, core::ptr::null_mut());
    let intr = if prop.is_null() { &mut interrupt as *mut _ as *mut u32 } else { (*prop).value as *mut u32 };
    sprintf(tmp_buf, b"%s@%x,%x\0".as_ptr() as _, name, *intr, reg0);
}

unsafe fn __build_path_component(dp: *mut device_node, tmp_buf: *mut ::core::ffi::c_char) {
    let parent = (*dp).parent;
    if !parent.is_null() {
        if of_node_is_type(parent, b"pci\0".as_ptr() as _) || of_node_is_type(parent, b"pciex\0".as_ptr() as _) { return pci_path_component(dp, tmp_buf); }
        if of_node_is_type(parent, b"sbus\0".as_ptr() as _) { return sbus_path_component(dp, tmp_buf); }
        if of_node_is_type(parent, b"ebus\0".as_ptr() as _) { return ebus_path_component(dp, tmp_buf); }
        if of_node_is_type(parent, b"ambapp\0".as_ptr() as _) { return ambapp_path_component(dp, tmp_buf); }
    }
    sparc32_path_component(dp, tmp_buf);
}

pub unsafe fn build_path_component(dp: *mut device_node) -> *mut ::core::ffi::c_char {
    let name = of_get_property(dp, b"name\0".as_ptr() as _, core::ptr::null_mut());
    let mut tmp_buf = [0 as ::core::ffi::c_char; 64];
    tmp_buf[0] = 0;
    __build_path_component(dp, tmp_buf.as_mut_ptr());
    if tmp_buf[0] == 0 { strscpy(tmp_buf.as_mut_ptr(), name, usize::MAX); }
    let n_sz = strlen(tmp_buf.as_ptr()) + 1;
    let n = prom_early_alloc(n_sz as _) as *mut ::core::ffi::c_char;
    strscpy(n, tmp_buf.as_ptr(), n_sz);
    n
}

unsafe extern "C" { fn restore_current(); }

pub unsafe fn of_console_init() {
    let msg = b"OF stdout device is: %s\n\0";
    let of_console_path_sz: usize = 256;
    of_console_path = prom_early_alloc(of_console_path_sz as _) as _;
    match prom_vers {
        PROM_V0 => {
            let mut skip = 0;
            let typ = match *(*romvec).pv_stdout {
                PROMDEV_SCREEN => b"display\0",
                PROMDEV_TTYB => { skip = 1; b"serial\0" },
                PROMDEV_TTYA => b"serial\0",
                _ => { prom_printf(b"Invalid PROM_V0 stdout value %u\n\0".as_ptr() as _, *(*romvec).pv_stdout); prom_halt(); return; }
            };
            let mut dp: *mut device_node = core::ptr::null_mut();
            let mut tmp = skip;
            while tmp >= 0 { dp = for_each_node_by_type(dp, typ.as_ptr() as _); if tmp == 0 { break; } tmp -= 1; }
            if dp.is_null() { prom_printf(b"Cannot find PROM_V0 console node.\n\0".as_ptr() as _); prom_halt(); }
            of_console_device = dp;
            sprintf(of_console_path, b"%pOF\0".as_ptr() as _, dp);
            if strcmp(typ.as_ptr() as _, b"serial\0".as_ptr() as _) == 0 { strcat(of_console_path, if skip != 0 { b":b\0".as_ptr() as _ } else { b":a\0".as_ptr() as _ }); }
        }
        _ => {
            let fd = (*(*romvec).pv_v2bootargs).fd_stdout;
            let node = (*(*romvec).pv_v2devops).v2_inst2pkg(fd);
            restore_current();
            if node == 0 { prom_printf(b"Cannot resolve stdout node from instance %08x.\n\0".as_ptr() as _, fd); prom_halt(); }
            let dp = of_find_node_by_phandle(node);
            if !of_node_is_type(dp, b"display\0".as_ptr() as _) && !of_node_is_type(dp, b"serial\0".as_ptr() as _) { prom_printf(b"Console device_type is neither display nor serial.\n\0".as_ptr() as _); prom_halt(); }
            of_console_device = dp;
            sprintf(of_console_path, b"%pOF\0".as_ptr() as _, dp);
        }
    }
    of_console_options = strrchr(of_console_path, b':' as _);
    if !of_console_options.is_null() { of_console_options = of_console_options.add(1); if *of_console_options == 0 { of_console_options = core::ptr::null_mut(); } }
    printk(msg.as_ptr() as _, of_console_path);
}

pub unsafe fn of_fill_in_cpu_data() {}
pub unsafe fn irq_trans_init(_dp: *mut device_node) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
