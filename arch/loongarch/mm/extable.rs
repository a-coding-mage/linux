// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the kernel headers:
// linux/bitfield.h, linux/extable.h, linux/uaccess.h,
// asm/asm-extable.h, and asm/branch.h.

#[inline]
unsafe fn get_ex_fixup(ex: *const exception_table_entry) -> kernel::ffi::c_ulong {
    ((core::ptr::addr_of!((*ex).fixup) as kernel::ffi::c_ulong).wrapping_add((*ex).fixup as kernel::ffi::c_ulong))
}

#[inline]
unsafe fn regs_set_gpr(regs: *mut pt_regs, offset: kernel::ffi::c_uint, val: kernel::ffi::c_ulong) {
    if offset != 0 && offset <= MAX_REG_OFFSET {
        *((regs as kernel::ffi::c_ulong).wrapping_add(offset as kernel::ffi::c_ulong) as *mut kernel::ffi::c_ulong) = val;
    }
}

unsafe fn ex_handler_fixup(
    ex: *const exception_table_entry,
    regs: *mut pt_regs,
) -> bool {
    (*regs).csr_era = get_ex_fixup(ex);

    true
}

unsafe fn ex_handler_uaccess_err_zero(
    ex: *const exception_table_entry,
    regs: *mut pt_regs,
) -> bool {
    let reg_err = FIELD_GET(EX_DATA_REG_ERR, (*ex).data);
    let reg_zero = FIELD_GET(EX_DATA_REG_ZERO, (*ex).data);

    regs_set_gpr(regs, reg_err * core::mem::size_of::<kernel::ffi::c_ulong>() as kernel::ffi::c_uint, (-EFAULT) as kernel::ffi::c_ulong);
    regs_set_gpr(regs, reg_zero * core::mem::size_of::<kernel::ffi::c_ulong>() as kernel::ffi::c_uint, 0);
    (*regs).csr_era = get_ex_fixup(ex);

    true
}

pub unsafe fn fixup_exception(regs: *mut pt_regs) -> bool {
    let ex: *const exception_table_entry;

    ex = search_exception_tables(exception_era(regs));
    if ex.is_null() {
        return false;
    }

    match (*ex).type_ {
        EX_TYPE_FIXUP => ex_handler_fixup(ex, regs),
        EX_TYPE_UACCESS_ERR_ZERO => ex_handler_uaccess_err_zero(ex, regs),
        EX_TYPE_BPF => ex_handler_bpf(ex, regs),
        _ => BUG(),
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
