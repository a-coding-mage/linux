// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2009 Sunplus Core Technology Co., Ltd.
 *  Lennox Wu <lennox.wu@sunplusct.com>
 *  Chen Liqin <liqin.chen@sunplusct.com>
 * Copyright (C) 2013 Regents of the University of California
 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// left as external names.

#[inline]
unsafe fn get_ex_fixup(ex: *const exception_table_entry) -> usize {
    (core::ptr::addr_of!((*ex).fixup) as usize).wrapping_add((*ex).fixup as usize)
}

unsafe fn ex_handler_fixup(ex: *const exception_table_entry, regs: *mut pt_regs) -> bool {
    (*regs).epc = get_ex_fixup(ex);
    true
}

#[inline]
unsafe fn regs_get_gpr(regs: *mut pt_regs, offset: u32) -> usize {
    if offset == 0 || offset > MAX_REG_OFFSET {
        return 0;
    }

    *((regs as usize).wrapping_add(offset as usize) as *const usize)
}

#[inline]
unsafe fn regs_set_gpr(regs: *mut pt_regs, offset: u32, val: usize) {
    if offset > MAX_REG_OFFSET {
        return;
    }

    if offset != 0 {
        *((regs as usize).wrapping_add(offset as usize) as *mut usize) = val;
    }
}

unsafe fn ex_handler_uaccess_err_zero(
    ex: *const exception_table_entry,
    regs: *mut pt_regs,
) -> bool {
    let reg_err = FIELD_GET!(EX_DATA_REG_ERR, (*ex).data) as u32;
    let reg_zero = FIELD_GET!(EX_DATA_REG_ZERO, (*ex).data) as u32;

    regs_set_gpr(regs, reg_err.wrapping_mul(core::mem::size_of::<usize>() as u32), (-EFAULT) as usize);
    regs_set_gpr(regs, reg_zero.wrapping_mul(core::mem::size_of::<usize>() as u32), 0);

    (*regs).epc = get_ex_fixup(ex);
    true
}

unsafe fn ex_handler_load_unaligned_zeropad(
    ex: *const exception_table_entry,
    regs: *mut pt_regs,
) -> bool {
    let reg_data = FIELD_GET!(EX_DATA_REG_DATA, (*ex).data) as u32;
    let reg_addr = FIELD_GET!(EX_DATA_REG_ADDR, (*ex).data) as u32;
    let addr = regs_get_gpr(regs, reg_addr.wrapping_mul(core::mem::size_of::<usize>() as u32));
    let offset = addr & 0x7usize;
    let aligned_addr = addr & !0x7usize;

    let data = *((aligned_addr) as *const usize) >> (offset * 8);

    regs_set_gpr(regs, reg_data.wrapping_mul(core::mem::size_of::<usize>() as u32), data);

    (*regs).epc = get_ex_fixup(ex);
    true
}

pub unsafe fn fixup_exception(regs: *mut pt_regs) -> bool {
    let ex: *const exception_table_entry = search_exception_tables((*regs).epc);
    if ex.is_null() {
        return false;
    }

    match (*ex).type_ {
        EX_TYPE_FIXUP => ex_handler_fixup(ex, regs),
        EX_TYPE_BPF => ex_handler_bpf(ex, regs),
        EX_TYPE_UACCESS_ERR_ZERO => ex_handler_uaccess_err_zero(ex, regs),
        EX_TYPE_LOAD_UNALIGNED_ZEROPAD => ex_handler_load_unaligned_zeropad(ex, regs),
        _ => BUG!(),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
