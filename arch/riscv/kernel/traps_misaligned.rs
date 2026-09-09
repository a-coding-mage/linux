// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 Western Digital Corporation or its affiliates.
 */

// Kernel headers and symbols referenced by this translation are supplied by
// the surrounding kernel bindings.

#[cfg(feature = "CONFIG_FPU")]
extern "C" {
    fn put_f32_reg(fp_reg: libc::c_ulong, value: libc::c_ulong);
    fn put_f64_reg(fp_reg: libc::c_ulong, value: libc::c_ulong);
    fn get_f32_reg(fp_reg: libc::c_ulong) -> libc::c_ulong;
    #[cfg(target_pointer_width = "64")]
    fn get_f64_reg(fp_reg: libc::c_ulong) -> libc::c_ulong;
    #[cfg(target_pointer_width = "32")]
    fn get_f64_reg(fp_reg: libc::c_ulong, value: *mut u64);
}

#[cfg(feature = "CONFIG_FPU")]
unsafe fn set_f32_rd(insn: libc::c_ulong, regs: *mut pt_regs, val: libc::c_ulong) -> i32 {
    put_f32_reg((insn >> 7) & 0x1f, val);
    (*regs).status |= SR_FS_DIRTY;
    0
}

#[cfg(feature = "CONFIG_FPU")]
unsafe fn set_f64_rd(insn: libc::c_ulong, regs: *mut pt_regs, val: u64) -> i32 {
    put_f64_reg((insn >> 7) & 0x1f, val as libc::c_ulong);
    (*regs).status |= SR_FS_DIRTY;
    0
}

#[cfg(feature = "CONFIG_FPU")]
unsafe fn get_f64_rs(insn: libc::c_ulong, off: u8, regs: *mut pt_regs) -> libc::c_ulong {
    let fp_reg = (insn >> off) & 0x1f;
    #[cfg(target_pointer_width = "64")]
    let val = get_f64_reg(fp_reg);
    #[cfg(target_pointer_width = "32")]
    let val = { let mut v = 0u64; get_f64_reg(fp_reg, &mut v); v as libc::c_ulong };
    (*regs).status |= SR_FS_DIRTY;
    val
}

#[cfg(feature = "CONFIG_FPU")]
unsafe fn get_f32_rs(insn: libc::c_ulong, off: u8, regs: *mut pt_regs) -> libc::c_ulong {
    let val = get_f32_reg((insn >> off) & 0x1f);
    (*regs).status |= SR_FS_DIRTY;
    val
}

#[cfg(not(feature = "CONFIG_FPU"))]
unsafe fn set_f32_rd(_: libc::c_ulong, _: *mut pt_regs, _: libc::c_ulong) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_FPU"))]
unsafe fn set_f64_rd(_: libc::c_ulong, _: *mut pt_regs, _: u64) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_FPU"))]
unsafe fn get_f64_rs(_: libc::c_ulong, _: u8, _: *mut pt_regs) -> libc::c_ulong { 0 }
#[cfg(not(feature = "CONFIG_FPU"))]
unsafe fn get_f32_rs(_: libc::c_ulong, _: u8, _: *mut pt_regs) -> libc::c_ulong { 0 }

#[repr(C)]
union reg_data { data_bytes: [u8; 8], data_ulong: libc::c_ulong, data_u64: u64 }

// sysctl hooks
#[no_mangle]
pub static mut unaligned_enabled: i32 = 1; /* Enabled by default */

unsafe fn get_insn(regs: *mut pt_regs, mut epc: libc::c_ulong, r_insn: *mut libc::c_ulong) -> i32 {
    let mut insn = 0;
    if epc & 2 != 0 {
        let mut tmp = 0u16;
        if __read_insn(regs, &mut tmp, epc as *const u8) != 0 { return -EFAULT; }
        insn = (tmp as libc::c_ulong) & GENMASK(15, 0);
        if (insn & __INSN_LENGTH_MASK) != __INSN_LENGTH_32 { *r_insn = insn; return 0; }
        epc += 2;
        if __read_insn(regs, &mut tmp, epc as *const u8) != 0 { return -EFAULT; }
        *r_insn = ((tmp as libc::c_ulong) << 16) | insn;
    } else {
        if __read_insn(regs, &mut insn, epc as *const u8) != 0 { return -EFAULT; }
        if (insn & __INSN_LENGTH_MASK) != __INSN_LENGTH_32 { insn &= GENMASK(15, 0); }
        *r_insn = insn;
    }
    0
}

// This is the direct Rust expression of the C __read_insn statement macro.
unsafe fn __read_insn<T>(regs: *mut pt_regs, out: *mut T, addr: *const u8) -> i32 {
    if user_mode(regs) { get_user(out, addr) } else { core::ptr::write_unaligned(out, core::ptr::read_unaligned(addr as *const T)); 0 }
}

unsafe fn handle_vector_misaligned_load(_: *mut pt_regs) -> i32 { -1 }
unsafe fn handle_scalar_misaligned_load(_: *mut pt_regs) -> i32 { -1 }
unsafe fn handle_scalar_misaligned_store(_: *mut pt_regs) -> i32 { -1 }

#[no_mangle]
pub unsafe extern "C" fn handle_misaligned_load(regs: *mut pt_regs) -> i32 {
    let epc = (*regs).epc;
    let mut insn = 0;
    if IS_ENABLED(CONFIG_RISCV_VECTOR_MISALIGNED) {
        if get_insn(regs, epc, &mut insn) != 0 { return -1; }
        if insn_is_vector(insn) { return handle_vector_misaligned_load(regs); }
    }
    if IS_ENABLED(CONFIG_RISCV_SCALAR_MISALIGNED) { return handle_scalar_misaligned_load(regs); }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn handle_misaligned_store(regs: *mut pt_regs) -> i32 {
    if IS_ENABLED(CONFIG_RISCV_SCALAR_MISALIGNED) { return handle_scalar_misaligned_store(regs); }
    -1
}

unsafe fn all_cpus_unaligned_scalar_access_emulated() -> bool {
    for_each_online_cpu!(cpu, { if per_cpu!(misaligned_access_speed, cpu) != RISCV_HWPROBE_MISALIGNED_SCALAR_EMULATED { return false; } });
    true
}

static mut misaligned_traps_delegated: bool = false;

#[no_mangle]
pub unsafe extern "C" fn check_unaligned_access_emulated_all_cpus() -> bool { all_cpus_unaligned_scalar_access_emulated() }

#[no_mangle]
pub unsafe extern "C" fn unaligned_access_init() {}

unsafe fn cpu_online_sbi_unaligned_setup(_: u32) -> i32 { 0 }
unsafe fn cpu_online_check_unaligned_access_emulated(_: u32) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn cpu_online_unaligned_access_init(cpu: u32) -> i32 {
    let ret = cpu_online_sbi_unaligned_setup(cpu); if ret != 0 { return ret; }
    cpu_online_check_unaligned_access_emulated(cpu)
}

#[no_mangle]
pub unsafe extern "C" fn misaligned_traps_can_delegate() -> bool {
    misaligned_traps_delegated || all_cpus_unaligned_scalar_access_emulated()
}

// External kernel types, constants, macros, and helpers intentionally remain
// unresolved here and are provided by the containing kernel translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
