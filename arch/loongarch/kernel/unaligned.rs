// SPDX-License-Identifier: GPL-2.0
/*
 * Handle unaligned accesses by emulation.
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 *
 * Derived from MIPS:
 * Copyright (C) 1996, 1998, 1999, 2002 by Ralf Baechle
 * Copyright (C) 1999 Silicon Graphics, Inc.
 * Copyright (C) 2014 Imagination Technologies Ltd.
 */

// Kernel headers and architecture dependencies are supplied by other Rust units.

#[cfg(CONFIG_DEBUG_FS)]
static mut UNALIGNED_INSTRUCTIONS_USER: u32 = 0;
#[cfg(CONFIG_DEBUG_FS)]
static mut UNALIGNED_INSTRUCTIONS_KERNEL: u32 = 0;

#[inline]
unsafe fn read_fpr(idx: u32) -> u64 {
    let mut value: u64 = 0;
    match idx {
        0..=31 => {
            // The original uses architecture-specific inline assembly for each FPR.
            // Keep the register selection explicit through the external helper ABI.
            value = __read_fpr(idx);
        }
        _ => panic!("unexpected idx '{}'", idx),
    }
    value
}

#[inline]
unsafe fn write_fpr(idx: u32, value: u64) {
    match idx {
        0..=31 => __write_fpr(idx, value),
        _ => panic!("unexpected idx '{}'", idx),
    }
}

extern "C" {
    fn __read_fpr(idx: u32) -> u64;
    fn __write_fpr(idx: u32, value: u64);
}

pub unsafe fn emulate_load_store_insn(
    regs: *mut pt_regs,
    addr: *mut core::ffi::c_void,
    pc: *mut u32,
) {
    let mut fp = false;
    let mut sign: bool;
    let mut write: bool;
    let user = user_mode(regs);
    let mut res: u32;
    let mut size: u32 = 0;
    let mut value: u64 = 0;
    let mut insn: loongarch_instruction = core::mem::zeroed();

    perf_sw_event(PERF_COUNT_SW_EMULATION_FAULTS, 1, regs, 0);
    __get_inst(&mut insn.word, pc, user);

    sign = false;
    write = false;
    match insn.reg2i12_format.opcode {
        ldh_op => { size = 2; sign = true; }
        ldhu_op => { size = 2; }
        sth_op => { size = 2; sign = true; write = true; }
        ldw_op => { size = 4; sign = true; }
        ldwu_op => { size = 4; }
        stw_op => { size = 4; sign = true; write = true; }
        ldd_op => { size = 8; sign = true; }
        std_op => { size = 8; sign = true; write = true; }
        flds_op => { size = 4; fp = true; sign = true; }
        fsts_op => { size = 4; fp = true; sign = true; write = true; }
        fldd_op => { size = 8; fp = true; sign = true; }
        fstd_op => { size = 8; fp = true; sign = true; write = true; }
        _ => {}
    }
    match insn.reg2i14_format.opcode {
        ldptrw_op => { size = 4; sign = true; write = false; }
        stptrw_op => { size = 4; sign = true; write = true; }
        ldptrd_op => { size = 8; sign = true; write = false; }
        stptrd_op => { size = 8; sign = true; write = true; }
        _ => {}
    }
    match insn.reg3_format.opcode {
        ldxh_op => { size = 2; sign = true; write = false; }
        ldxhu_op => { size = 2; sign = false; write = false; }
        stxh_op => { size = 2; sign = true; write = true; }
        ldxw_op => { size = 4; sign = true; write = false; }
        ldxwu_op => { size = 4; sign = false; write = false; }
        stxw_op => { size = 4; sign = true; write = true; }
        ldxd_op => { size = 8; sign = true; write = false; }
        stxd_op => { size = 8; sign = true; write = true; }
        fldxs_op => { size = 4; fp = true; sign = true; write = false; }
        fstxs_op => { size = 4; fp = true; sign = true; write = true; }
        fldxd_op => { size = 8; fp = true; sign = true; write = false; }
        fstxd_op => { size = 8; fp = true; sign = true; write = true; }
        _ => {}
    }

    if size == 0 { goto_sigbus(regs); return; }
    if user && !access_ok(addr, size) { goto_sigbus(regs); return; }

    if !write {
        res = unaligned_read(addr, &mut value, size, sign);
        if res != 0 { goto_fault(regs); return; }
        if !fp {
            (*regs).regs[insn.reg3_format.rd as usize] = value;
        } else if is_fpu_owner() {
            write_fpr(insn.reg3_format.rd, value);
        } else {
            set_fpr64(&mut current().thread.fpu.fpr[insn.reg3_format.rd as usize], 0, value);
        }
    } else {
        if !fp {
            value = (*regs).regs[insn.reg3_format.rd as usize];
        } else if is_fpu_owner() {
            value = read_fpr(insn.reg3_format.rd);
        } else {
            value = get_fpr64(&current().thread.fpu.fpr[insn.reg3_format.rd as usize], 0);
        }
        res = unaligned_write(addr, value, size);
        if res != 0 { goto_fault(regs); return; }
    }

    #[cfg(CONFIG_DEBUG_FS)]
    if user { UNALIGNED_INSTRUCTIONS_USER += 1; } else { UNALIGNED_INSTRUCTIONS_KERNEL += 1; }
    compute_return_era(regs);
    return;
}

unsafe fn goto_fault(regs: *mut pt_regs) {
    if fixup_exception(regs) != 0 { return; }
    die_if_kernel("Unhandled kernel unaligned access", regs);
    force_sig(SIGSEGV);
}

unsafe fn goto_sigbus(regs: *mut pt_regs) {
    die_if_kernel("Unhandled kernel unaligned access", regs);
    force_sig(SIGBUS);
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn debugfs_unaligned() -> i32 {
    debugfs_create_u32("unaligned_instructions_user", S_IRUGO, arch_debugfs_dir, &mut UNALIGNED_INSTRUCTIONS_USER);
    debugfs_create_u32("unaligned_instructions_kernel", S_IRUGO, arch_debugfs_dir, &mut UNALIGNED_INSTRUCTIONS_KERNEL);
    0
}

// arch_initcall(debugfs_unaligned);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
