// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
/*
 * libfdt - Flat Device Tree manipulation
 * Copyright (C) 2014 David Gibson <david@gibson.dropbear.id.au>
 * Copyright (C) 2018 embedded brains GmbH
 */

// Dependencies supplied by the surrounding libfdt translation.

unsafe fn fdt_cells(fdt: *const core::ffi::c_void, nodeoffset: i32, name: *const core::ffi::c_char) -> i32 {
    let mut len: i32 = 0;
    let c: *const fdt32_t = fdt_getprop(fdt, nodeoffset, name, &mut len);
    if c.is_null() {
        return len;
    }

    if len != core::mem::size_of::<fdt32_t>() as i32 {
        return -FDT_ERR_BADNCELLS;
    }

    let val: u32 = fdt32_to_cpu(*c);
    if val > FDT_MAX_NCELLS {
        return -FDT_ERR_BADNCELLS;
    }

    val as i32
}

pub unsafe fn fdt_address_cells(fdt: *const core::ffi::c_void, nodeoffset: i32) -> i32 {
    let val: i32 = fdt_cells(fdt, nodeoffset, c"#address-cells".as_ptr());
    if val == 0 {
        return -FDT_ERR_BADNCELLS;
    }
    if val == -FDT_ERR_NOTFOUND {
        return 2;
    }
    val
}

pub unsafe fn fdt_size_cells(fdt: *const core::ffi::c_void, nodeoffset: i32) -> i32 {
    let val: i32 = fdt_cells(fdt, nodeoffset, c"#size-cells".as_ptr());
    if val == -FDT_ERR_NOTFOUND {
        return 1;
    }
    val
}

/* This function assumes that [address|size]_cells is 1 or 2 */
pub unsafe fn fdt_appendprop_addrrange(
    fdt: *mut core::ffi::c_void,
    parent: i32,
    nodeoffset: i32,
    name: *const core::ffi::c_char,
    addr: u64,
    size: u64,
) -> i32 {
    let ret: i32 = fdt_address_cells(fdt, parent);
    if ret < 0 {
        return ret;
    }
    let addr_cells: i32 = ret;

    let ret: i32 = fdt_size_cells(fdt, parent);
    if ret < 0 {
        return ret;
    }
    let size_cells: i32 = ret;

    /* check validity of address */
    let mut data: [u8; core::mem::size_of::<fdt64_t>() * 2] = [0; core::mem::size_of::<fdt64_t>() * 2];
    let mut prop: *mut u8 = data.as_mut_ptr();
    if addr_cells == 1 {
        if (addr > u32::MAX as u64) || ((u32::MAX as u64 + 1 - addr) < size) {
            return -FDT_ERR_BADVALUE;
        }

        fdt32_st(prop, addr as u32);
    } else if addr_cells == 2 {
        fdt64_st(prop, addr);
    } else {
        return -FDT_ERR_BADNCELLS;
    }

    /* check validity of size */
    prop = prop.add(addr_cells as usize * core::mem::size_of::<fdt32_t>());
    if size_cells == 1 {
        if size > u32::MAX as u64 {
            return -FDT_ERR_BADVALUE;
        }

        fdt32_st(prop, size as u32);
    } else if size_cells == 2 {
        fdt64_st(prop, size);
    } else {
        return -FDT_ERR_BADNCELLS;
    }

    fdt_appendprop(
        fdt,
        nodeoffset,
        name,
        data.as_ptr() as *const core::ffi::c_void,
        (addr_cells + size_cells) * core::mem::size_of::<fdt32_t>() as i32,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
