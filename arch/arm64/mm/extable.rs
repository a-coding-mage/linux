// SPDX-License-Identifier: GPL-2.0
/*
 * Based on arch/arm/mm/extable.c
 */

// The declarations below are supplied by the corresponding kernel headers and
// other translation units.

#[repr(C)]
pub struct exception_table_entry {
    pub insn: i32,
    pub fixup: i32,
    pub r#type: u32,
    pub data: u32,
}

#[repr(C)]
pub struct pt_regs {
    pub pc: u64,
}

extern "C" {
    fn search_exception_tables(addr: c_ulong) -> *const exception_table_entry;
    fn instruction_pointer(regs: *const pt_regs) -> c_ulong;
    fn pt_regs_write_reg(regs: *mut pt_regs, reg: i32, val: c_ulong);
    fn pt_regs_read_reg(regs: *const pt_regs, reg: i32) -> c_ulong;
    fn ex_handler_bpf(ex: *const exception_table_entry, regs: *mut pt_regs) -> bool;
    fn BUG() -> !;
}

type c_ulong = usize;

// Header-provided constants: EX_DATA_UACCESS_WRITE, EX_DATA_REG_ERR,
// EX_DATA_REG_ZERO, EX_DATA_REG_DATA, EX_DATA_REG_ADDR, EX_TYPE_BPF,
// EX_TYPE_UACCESS_ERR_ZERO, EX_TYPE_KACCESS_ERR_ZERO,
// EX_TYPE_UACCESS_CPY, EX_TYPE_LOAD_UNALIGNED_ZEROPAD, ESR_ELx_WNR, and EFAULT.

#[inline]
unsafe fn field_get(mask: u32, value: u32) -> u32 {
    (value & mask) >> mask.trailing_zeros()
}

unsafe fn cpy_faulted_on_uaccess(
    ex: *const exception_table_entry,
    esr: c_ulong,
) -> bool {
    let uaccess_is_write = field_get(EX_DATA_UACCESS_WRITE, (*ex).data);
    let fault_on_write = esr & ESR_ELx_WNR as c_ulong;

    uaccess_is_write as c_ulong == fault_on_write
}

pub unsafe fn insn_may_access_user(addr: c_ulong, esr: c_ulong) -> bool {
    let ex = search_exception_tables(addr);

    if ex.is_null() {
        return false;
    }

    match (*ex).r#type {
        EX_TYPE_UACCESS_CPY => cpy_faulted_on_uaccess(ex, esr),
        _ => true,
    }
}

#[inline]
unsafe fn get_ex_fixup(ex: *const exception_table_entry) -> c_ulong {
    (&(*ex).fixup as *const i32 as c_ulong).wrapping_add((*ex).fixup as c_ulong)
}

unsafe fn ex_handler_uaccess_err_zero(
    ex: *const exception_table_entry,
    regs: *mut pt_regs,
) -> bool {
    let reg_err = field_get(EX_DATA_REG_ERR, (*ex).data) as i32;
    let reg_zero = field_get(EX_DATA_REG_ZERO, (*ex).data) as i32;

    pt_regs_write_reg(regs, reg_err, (-EFAULT) as c_ulong);
    pt_regs_write_reg(regs, reg_zero, 0);

    (*regs).pc = get_ex_fixup(ex) as u64;
    true
}

unsafe fn ex_handler_uaccess_cpy(
    ex: *const exception_table_entry,
    regs: *mut pt_regs,
    esr: c_ulong,
) -> bool {
    /* Do not fix up faults on kernel memory accesses */
    if !cpy_faulted_on_uaccess(ex, esr) {
        return false;
    }

    (*regs).pc = get_ex_fixup(ex) as u64;
    true
}

unsafe fn ex_handler_load_unaligned_zeropad(
    ex: *const exception_table_entry,
    regs: *mut pt_regs,
) -> bool {
    let reg_data = field_get(EX_DATA_REG_DATA, (*ex).data) as i32;
    let reg_addr = field_get(EX_DATA_REG_ADDR, (*ex).data) as i32;
    let mut data: c_ulong;
    let mut addr: c_ulong;
    let offset: c_ulong;

    addr = pt_regs_read_reg(regs, reg_addr);

    offset = addr & 0x7;
    addr &= !0x7;

    data = *(addr as *const c_ulong);

    #[cfg(not(target_endian = "big"))]
    {
        data >>= 8 * offset;
    }
    #[cfg(target_endian = "big")]
    {
        data <<= 8 * offset;
    }

    pt_regs_write_reg(regs, reg_data, data);

    (*regs).pc = get_ex_fixup(ex) as u64;
    true
}

pub unsafe fn fixup_exception(regs: *mut pt_regs, esr: c_ulong) -> bool {
    let ex = search_exception_tables(instruction_pointer(regs));
    if ex.is_null() {
        return false;
    }

    match (*ex).r#type {
        EX_TYPE_BPF => ex_handler_bpf(ex, regs),
        EX_TYPE_UACCESS_ERR_ZERO | EX_TYPE_KACCESS_ERR_ZERO => {
            ex_handler_uaccess_err_zero(ex, regs)
        }
        EX_TYPE_UACCESS_CPY => ex_handler_uaccess_cpy(ex, regs, esr),
        EX_TYPE_LOAD_UNALIGNED_ZEROPAD => ex_handler_load_unaligned_zeropad(ex, regs),
        _ => BUG(),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
