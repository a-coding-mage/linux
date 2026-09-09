// SPDX-License-Identifier: GPL-2.0
/*
 * unaligned.c: Unaligned load/store trap handling with special
 *              cases for the kernel to do them more quickly.
 *
 * Copyright (C) 1996 David S. Miller (davem@caip.rutgers.edu)
 * Copyright (C) 1996 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 */

// Kernel and architecture headers provide the referenced types, constants, and functions.

#[repr(C)]
enum direction {
    load,
    store,
    both,
    fpload,
    fpstore,
    invalid,
}

#[inline]
unsafe fn decode_direction(insn: u32) -> direction {
    let tmp = (insn >> 21) & 1;
    if tmp == 0 {
        direction::load
    } else if ((insn >> 19) & 0x3f) == 15 {
        direction::both
    } else {
        direction::store
    }
}

#[inline]
unsafe fn decode_access_size(mut insn: u32) -> i32 {
    insn = (insn >> 19) & 3;
    if insn == 0 {
        4
    } else if insn == 3 {
        8
    } else if insn == 2 {
        2
    } else {
        printk(b"Impossible unaligned trap. insn=%08x\n\0".as_ptr() as *const i8, insn);
        die_if_kernel(b"Byte sized unaligned access?!?!\0".as_ptr() as *const i8, core::ptr::null_mut());
        4
    }
}

#[inline]
unsafe fn decode_signedness(insn: u32) -> i32 {
    (insn & 0x400000) as i32
}

#[inline]
unsafe fn maybe_flush_windows(rs1: u32, rs2: u32, rd: u32) {
    if rs2 >= 16 || rs1 >= 16 || rd >= 16 {
        // Wheee...
        core::arch::asm!(
            "save %sp, -0x40, %sp", "save %sp, -0x40, %sp", "save %sp, -0x40, %sp",
            "save %sp, -0x40, %sp", "save %sp, -0x40, %sp", "save %sp, -0x40, %sp",
            "save %sp, -0x40, %sp", "restore; restore; restore; restore;",
            "restore; restore; restore;",
            options(nostack, preserves_flags)
        );
    }
}

#[inline]
unsafe fn sign_extend_imm13(imm: i32) -> i32 {
    (imm << 19) >> 19
}

#[inline]
unsafe fn fetch_reg(reg: u32, regs: *mut pt_regs) -> usize {
    if reg < 16 {
        return if reg == 0 { 0 } else { (*regs).u_regs[reg as usize] };
    }
    let win = (*regs).u_regs[UREG_FP] as *mut reg_window32;
    (*win).locals[(reg - 16) as usize]
}

#[inline]
unsafe fn safe_fetch_reg(reg: u32, regs: *mut pt_regs) -> usize {
    if reg < 16 {
        return if reg == 0 { 0 } else { (*regs).u_regs[reg as usize] };
    }
    let win = (*regs).u_regs[UREG_FP] as *mut reg_window32;
    if (win as usize) & 3 != 0 {
        return usize::MAX;
    }
    let mut ret = 0usize;
    if get_user(&mut ret, &(*win).locals[(reg - 16) as usize]) != 0 {
        return usize::MAX;
    }
    ret
}

#[inline]
unsafe fn fetch_reg_addr(reg: u32, regs: *mut pt_regs) -> *mut usize {
    if reg < 16 {
        return &mut (*regs).u_regs[reg as usize];
    }
    let win = (*regs).u_regs[UREG_FP] as *mut reg_window32;
    &mut (*win).locals[(reg - 16) as usize]
}

unsafe fn compute_effective_address(regs: *mut pt_regs, insn: u32) -> usize {
    let rs1 = (insn >> 14) & 0x1f;
    let rs2 = insn & 0x1f;
    let rd = (insn >> 25) & 0x1f;
    if insn & 0x2000 != 0 {
        maybe_flush_windows(rs1, 0, rd);
        fetch_reg(rs1, regs).wrapping_add(sign_extend_imm13(insn as i32) as usize)
    } else {
        maybe_flush_windows(rs1, rs2, rd);
        fetch_reg(rs1, regs).wrapping_add(fetch_reg(rs2, regs))
    }
}

pub unsafe fn safe_compute_effective_address(regs: *mut pt_regs, insn: u32) -> usize {
    let rs1 = (insn >> 14) & 0x1f;
    let rs2 = insn & 0x1f;
    let rd = (insn >> 25) & 0x1f;
    if insn & 0x2000 != 0 {
        maybe_flush_windows(rs1, 0, rd);
        safe_fetch_reg(rs1, regs).wrapping_add(sign_extend_imm13(insn as i32) as usize)
    } else {
        maybe_flush_windows(rs1, rs2, rd);
        safe_fetch_reg(rs1, regs).wrapping_add(safe_fetch_reg(rs2, regs))
    }
}

unsafe fn unaligned_panic(str_: *const i8) {
    panic(str_);
}

extern "C" {
    fn do_int_load(dest_reg: *mut usize, size: i32, saddr: *mut usize, is_signed: i32) -> i32;
    fn __do_int_store(dst_addr: *mut usize, size: i32, src_val: *mut usize) -> i32;
    fn smp_capture();
    fn smp_release();
}

unsafe fn do_int_store(reg_num: i32, size: i32, dst_addr: *mut usize, regs: *mut pt_regs) -> i32 {
    let mut zero = [0usize; 2];
    let src_val = if reg_num != 0 {
        fetch_reg_addr(reg_num as u32, regs)
    } else {
        if size == 8 { zero[1] = fetch_reg(1, regs); }
        zero.as_mut_ptr()
    };
    __do_int_store(dst_addr, size, src_val)
}

#[inline]
unsafe fn advance(regs: *mut pt_regs) {
    (*regs).pc = (*regs).npc;
    (*regs).npc = (*regs).npc.wrapping_add(4);
}

#[inline]
unsafe fn floating_point_load_or_store_p(insn: u32) -> i32 { ((insn >> 24) & 1) as i32 }
#[inline]
unsafe fn ok_for_kernel(insn: u32) -> bool { floating_point_load_or_store_p(insn) == 0 }

unsafe fn kernel_mna_trap_fault(regs: *mut pt_regs, insn: u32) {
    let entry = search_exception_tables((*regs).pc);
    if entry.is_null() {
        let address = compute_effective_address(regs, insn);
        if address < PAGE_SIZE { printk(b"Unable to handle kernel NULL pointer dereference in mna handler\0".as_ptr() as *const i8); }
        else { printk(b"Unable to handle kernel paging request in mna handler\0".as_ptr() as *const i8); }
        printk(b" at virtual address %08lx\n\0".as_ptr() as *const i8, address);
        die_if_kernel(b"Oops\0".as_ptr() as *const i8, regs);
    }
    (*regs).pc = (*entry).fixup;
    (*regs).npc = (*regs).pc.wrapping_add(4);
}

pub unsafe fn kernel_unaligned_trap(regs: *mut pt_regs, insn: u32) {
    let dir = decode_direction(insn);
    let size = decode_access_size(insn);
    if !ok_for_kernel(insn) || matches!(dir, direction::both) {
        printk(b"Unsupported unaligned load/store trap for kernel at <%08lx>.\n\0".as_ptr() as *const i8, (*regs).pc);
        unaligned_panic(b"Wheee. Kernel does fpu/atomic unaligned load/store.\0".as_ptr() as *const i8);
    } else {
        let addr = compute_effective_address(regs, insn);
        perf_sw_event(PERF_COUNT_SW_ALIGNMENT_FAULTS, 1, regs, addr);
        let err = match dir {
            direction::load => do_int_load(fetch_reg_addr((insn >> 25) & 0x1f, regs), size, addr as *mut usize, decode_signedness(insn)),
            direction::store => do_int_store(((insn >> 25) & 0x1f) as i32, size, addr as *mut usize, regs),
            _ => { panic(b"Impossible kernel unaligned trap.\0".as_ptr() as *const i8); 0 }
        };
        if err != 0 { kernel_mna_trap_fault(regs, insn); } else { advance(regs); }
    }
}

pub unsafe fn user_unaligned_trap(regs: *mut pt_regs, insn: u32) {
    send_sig_fault(SIGBUS, BUS_ADRALN, safe_compute_effective_address(regs, insn) as *mut core::ffi::c_void, current);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
