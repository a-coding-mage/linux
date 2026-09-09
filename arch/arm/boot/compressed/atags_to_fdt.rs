// SPDX-License-Identifier: GPL-2.0
// External dependencies correspond to linux/libfdt_env.h, asm/setup.h,
// libfdt.h, and misc.h from the original implementation.

#[cfg(CONFIG_ARM_ATAG_DTB_COMPAT_CMDLINE_EXTEND)]
const DO_EXTEND_CMDLINE: bool = true;
#[cfg(not(CONFIG_ARM_ATAG_DTB_COMPAT_CMDLINE_EXTEND))]
const DO_EXTEND_CMDLINE: bool = false;

const NR_BANKS: usize = 16;

unsafe fn node_offset(fdt: *mut core::ffi::c_void, node_path: *const i8) -> i32 {
    let mut offset = fdt_path_offset(fdt as *const core::ffi::c_void, node_path);
    if offset == -FDT_ERR_NOTFOUND {
        // Add the node to root if not found, dropping the leading '/'.
        offset = fdt_add_subnode(fdt, 0, node_path.add(1));
    }
    offset
}

unsafe fn setprop(
    fdt: *mut core::ffi::c_void,
    node_path: *const i8,
    property: *const i8,
    val_array: *const core::ffi::c_void,
    size: i32,
) -> i32 {
    let offset = node_offset(fdt, node_path);
    if offset < 0 {
        return offset;
    }
    fdt_setprop(fdt, offset, property, val_array, size)
}

unsafe fn setprop_string(
    fdt: *mut core::ffi::c_void,
    node_path: *const i8,
    property: *const i8,
    string: *const i8,
) -> i32 {
    let offset = node_offset(fdt, node_path);
    if offset < 0 {
        return offset;
    }
    fdt_setprop_string(fdt, offset, property, string)
}

unsafe fn setprop_cell(
    fdt: *mut core::ffi::c_void,
    node_path: *const i8,
    property: *const i8,
    val: u32,
) -> i32 {
    let offset = node_offset(fdt, node_path);
    if offset < 0 {
        return offset;
    }
    fdt_setprop_cell(fdt, offset, property, val)
}

unsafe fn getprop(
    fdt: *const core::ffi::c_void,
    node_path: *const i8,
    property: *const i8,
    len: *mut i32,
) -> *const core::ffi::c_void {
    let offset = fdt_path_offset(fdt, node_path);
    if offset == -FDT_ERR_NOTFOUND {
        return core::ptr::null();
    }
    fdt_getprop(fdt, offset, property, len)
}

unsafe fn get_cell_size(fdt: *const core::ffi::c_void) -> u32 {
    let mut len = 0;
    let mut cell_size = 1;
    let size_len = getprop(fdt, b"/\0".as_ptr() as *const i8, b"#size-cells\0".as_ptr() as *const i8, &mut len);
    if !size_len.is_null() {
        cell_size = fdt32_to_cpu(*(size_len as *const u32));
    }
    cell_size
}

unsafe fn merge_fdt_bootargs(fdt: *mut core::ffi::c_void, fdt_cmdline: *const i8) {
    let mut cmdline = [0i8; COMMAND_LINE_SIZE];
    let mut len = 0;
    let mut ptr = cmdline.as_mut_ptr();
    let fdt_bootargs = getprop(fdt as *const _, b"/chosen\0".as_ptr() as *const i8, b"bootargs\0".as_ptr() as *const i8, &mut len);
    if !fdt_bootargs.is_null() && len < COMMAND_LINE_SIZE as i32 {
        core::ptr::copy_nonoverlapping(fdt_bootargs as *const i8, ptr, len as usize);
        ptr = ptr.add((len - 1) as usize);
    }
    if !fdt_cmdline.is_null() {
        len = strlen(fdt_cmdline) as i32;
        if ptr.offset_from(cmdline.as_ptr()) + len as isize + 2 < COMMAND_LINE_SIZE as isize {
            *ptr = b' ' as i8;
            ptr = ptr.add(1);
            core::ptr::copy_nonoverlapping(fdt_cmdline, ptr, len as usize);
            ptr = ptr.add(len as usize);
        }
    }
    *ptr = 0;
    setprop_string(fdt, b"/chosen\0".as_ptr() as *const i8, b"bootargs\0".as_ptr() as *const i8, cmdline.as_ptr());
}

unsafe fn hex_str(mut out: *mut i8, mut value: u32) {
    for _ in 0..8 {
        let mut digit = value >> 28;
        value <<= 4;
        digit &= 0xf;
        *out = if digit < 10 { (digit + b'0' as u32) as i8 } else { (digit + b'A' as u32 - 10) as i8 };
        out = out.add(1);
    }
    *out = 0;
}

/* Convert and fold provided ATAGs into the provided FDT. */
pub unsafe fn atags_to_fdt(atag_list: *mut core::ffi::c_void, fdt: *mut core::ffi::c_void, total_space: i32) -> i32 {
    let mut atag = atag_list as *mut tag;
    let mut mem_reg_property = [0u32; 2 * 2 * NR_BANKS];
    let mut memcount = 0i32;
    let mut memsize = 0u32;

    if (atag_list as usize & 0x3) != 0 { return 1; }
    if *(atag_list as *const u32) == cpu_to_fdt32(FDT_MAGIC) { return 0; }
    if (*atag).hdr.tag != ATAG_CORE || ((*atag).hdr.size != tag_size(tag_core) && (*atag).hdr.size != 2) { return 1; }
    let mut ret = fdt_open_into(fdt, fdt, total_space);
    if ret < 0 { return ret; }

    while (*atag).hdr.tag != ATAG_NONE {
        if (*atag).hdr.tag == ATAG_CMDLINE {
            if DO_EXTEND_CMDLINE { merge_fdt_bootargs(fdt, (*atag).u.cmdline.cmdline.as_ptr()); }
            else { setprop_string(fdt, b"/chosen\0".as_ptr() as *const i8, b"bootargs\0".as_ptr() as *const i8, (*atag).u.cmdline.cmdline.as_ptr()); }
        } else if (*atag).hdr.tag == ATAG_MEM {
            if memcount < (mem_reg_property.len() as i32) && (*atag).u.mem.size != 0 {
                memsize = get_cell_size(fdt);
                if memsize == 2 {
                    let p = mem_reg_property.as_mut_ptr() as *mut u64;
                    *p.add(memcount as usize) = cpu_to_fdt64((*atag).u.mem.start as u64);
                    memcount += 1;
                    *p.add(memcount as usize) = cpu_to_fdt64((*atag).u.mem.size as u64);
                    memcount += 1;
                } else {
                    mem_reg_property[memcount as usize] = cpu_to_fdt32((*atag).u.mem.start);
                    memcount += 1;
                    mem_reg_property[memcount as usize] = cpu_to_fdt32((*atag).u.mem.size);
                    memcount += 1;
                }
            }
        } else if (*atag).hdr.tag == ATAG_INITRD2 {
            let start = (*atag).u.initrd.start;
            setprop_cell(fdt, b"/chosen\0".as_ptr() as *const i8, b"linux,initrd-start\0".as_ptr() as *const i8, start);
            setprop_cell(fdt, b"/chosen\0".as_ptr() as *const i8, b"linux,initrd-end\0".as_ptr() as *const i8, start + (*atag).u.initrd.size);
        } else if (*atag).hdr.tag == ATAG_SERIAL {
            let mut serno = [0i8; 18];
            hex_str(serno.as_mut_ptr(), (*atag).u.serialnr.high);
            hex_str(serno.as_mut_ptr().add(8), (*atag).u.serialnr.low);
            setprop_string(fdt, b"/\0".as_ptr() as *const i8, b"serial-number\0".as_ptr() as *const i8, serno.as_ptr());
        }
        atag = (atag as *mut u8).add((*atag).hdr.size as usize * 4) as *mut tag;
    }
    if memcount != 0 { setprop(fdt, b"/memory\0".as_ptr() as *const i8, b"reg\0".as_ptr() as *const i8, mem_reg_property.as_ptr() as *const _, 4 * memcount * memsize as i32); }
    ret = fdt_pack(fdt);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
