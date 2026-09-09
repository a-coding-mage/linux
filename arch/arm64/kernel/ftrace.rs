// SPDX-License-Identifier: GPL-2.0-only
/* arch/arm64/kernel/ftrace.c -- direct Rust translation. */

#[repr(C)]
pub struct FregsOffset { pub name: *const core::ffi::c_char, pub offset: i32 }

// CONFIG_DYNAMIC_FTRACE_WITH_ARGS
#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
static FREGS_OFFSETS: [FregsOffset; 14] = [
    FregsOffset { name: b"x0\0".as_ptr() as _, offset: 0 }, FregsOffset { name: b"x1\0".as_ptr() as _, offset: 8 },
    FregsOffset { name: b"x2\0".as_ptr() as _, offset: 16 }, FregsOffset { name: b"x3\0".as_ptr() as _, offset: 24 },
    FregsOffset { name: b"x4\0".as_ptr() as _, offset: 32 }, FregsOffset { name: b"x5\0".as_ptr() as _, offset: 40 },
    FregsOffset { name: b"x6\0".as_ptr() as _, offset: 48 }, FregsOffset { name: b"x7\0".as_ptr() as _, offset: 56 },
    FregsOffset { name: b"x8\0".as_ptr() as _, offset: 64 }, FregsOffset { name: b"x29\0".as_ptr() as _, offset: 0 },
    FregsOffset { name: b"x30\0".as_ptr() as _, offset: 0 }, FregsOffset { name: b"lr\0".as_ptr() as _, offset: 0 },
    FregsOffset { name: b"sp\0".as_ptr() as _, offset: 0 }, FregsOffset { name: b"pc\0".as_ptr() as _, offset: 0 },
];

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
pub unsafe fn ftrace_regs_query_register_offset(_name: *const core::ffi::c_char) -> i32 {
    // The offsets are offsetof(__arch_ftrace_regs, field); supplied by the architecture layout.
    for roff in FREGS_OFFSETS.iter() {
        if libc::strcmp(roff.name, _name) == 0 { return roff.offset; }
    }
    -libc::EINVAL
}

pub unsafe fn ftrace_call_adjust(mut addr: usize) -> usize {
    // CONFIG_DYNAMIC_FTRACE_WITH_ARGS disabled: return addr unchanged.
    if !cfg!(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS") { return addr; }
    if !cfg!(feature = "CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS") { return addr + AARCH64_INSN_SIZE; }
    if addr % core::mem::size_of::<usize>() != 0 {
        warn_ratelimit(1, "Misaligned patch-site %pS\n", (addr + 8) as *const core::ffi::c_void);
        return 0;
    }
    addr += 2 * AARCH64_INSN_SIZE;
    if cfg!(feature = "CONFIG_ARM64_BTI_KERNEL") {
        let insn = u32::from_le(*(addr as *const u32));
        if aarch64_insn_is_bti(insn) { addr += AARCH64_INSN_SIZE; }
        else if insn != aarch64_insn_gen_nop() { warn_ratelimit(1, "unexpected insn in patch-site %pS: 0x%08x\n", addr as *const core::ffi::c_void, insn); }
    }
    addr + AARCH64_INSN_SIZE
}

pub unsafe fn arch_ftrace_get_symaddr(fentry_ip: usize) -> usize {
    if !cfg!(feature = "CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS") { return fentry_ip - AARCH64_INSN_SIZE; }
    if !cfg!(feature = "CONFIG_ARM64_BTI_KERNEL") { return fentry_ip - AARCH64_INSN_SIZE; }
    let insn: u32;
    if (fentry_ip & !PAGE_MASK) < AARCH64_INSN_SIZE * 2 {
        let mut value = 0u32;
        if get_kernel_nofault(&mut value, (fentry_ip - AARCH64_INSN_SIZE * 2) as *mut u32) != 0 { return 0; }
        insn = value;
    } else { insn = *(fentry_ip - AARCH64_INSN_SIZE * 2) as *const u32; }
    if aarch64_insn_is_bti(u32::from_le(insn)) { fentry_ip - AARCH64_INSN_SIZE * 2 } else { fentry_ip - AARCH64_INSN_SIZE }
}

unsafe fn ftrace_modify_code(pc: usize, old: u32, new: u32, validate: bool) -> i32 {
    if validate {
        let mut replaced = 0u32;
        if aarch64_insn_read(pc as *const core::ffi::c_void, &mut replaced) != 0 { return -libc::EFAULT; }
        if replaced != old { return -libc::EINVAL; }
    }
    if aarch64_insn_patch_text_nosync(pc as *mut core::ffi::c_void, new) != 0 { return -libc::EPERM; }
    0
}

pub unsafe fn ftrace_update_ftrace_func(func: usize) -> i32 {
    if cfg!(feature = "CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS") { return 0; }
    let pc = ftrace_call as usize;
    let new = aarch64_insn_gen_branch_imm(pc, func, AARCH64_INSN_BRANCH_LINK);
    ftrace_modify_code(pc, 0, new, false)
}

unsafe fn reachable_by_bl(addr: usize, pc: usize) -> bool {
    let offset = addr as isize - pc as isize;
    offset >= -(SZ_128M as isize) && offset < SZ_128M as isize
}

unsafe fn get_ftrace_plt(_mod: *mut module, _addr: usize) -> *mut plt_entry { core::ptr::null_mut() }
unsafe fn ftrace_find_callable_addr(rec: *mut dyn_ftrace, _mod: *mut module, addr: *mut usize) -> bool {
    let pc = (*rec).ip;
    if *addr != FTRACE_ADDR && !reachable_by_bl(*addr, pc) { *addr = FTRACE_ADDR; }
    if reachable_by_bl(*addr, pc) { return true; }
    if !cfg!(feature = "CONFIG_MODULES") { return false; }
    let plt = get_ftrace_plt(_mod, pc); if plt.is_null() { return false; }
    *addr = plt as usize; true
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS")]
unsafe fn ftrace_rec_set_ops(rec: *const dyn_ftrace, ops: *const ftrace_ops) -> i32 {
    let literal = (unsafe { (*rec).ip } - 12) & !7;
    aarch64_insn_write_literal_u64(literal as *mut _, ops as usize)
}
#[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS"))]
unsafe fn ftrace_rec_set_ops(_: *const dyn_ftrace, _: *const ftrace_ops) -> i32 { 0 }
unsafe fn ftrace_rec_set_nop_ops(rec: *mut dyn_ftrace) -> i32 { ftrace_rec_set_ops(rec, &ftrace_nop_ops) }
unsafe fn ftrace_rec_update_ops(rec: *mut dyn_ftrace) -> i32 { ftrace_rec_set_ops(rec, arm64_rec_get_ops(rec)) }
unsafe fn arm64_rec_get_ops(_: *mut dyn_ftrace) -> *const ftrace_ops { &ftrace_list_ops }

#[cfg(any(feature = "CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS", feature = "CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS"))]
pub unsafe fn ftrace_modify_call(rec: *mut dyn_ftrace, mut old_addr: usize, mut addr: usize) -> i32 {
    let pc = (*rec).ip; let ret = ftrace_rec_update_ops(rec); if ret != 0 { return ret; }
    if !ftrace_find_callable_addr(rec, core::ptr::null_mut(), &mut old_addr) || !ftrace_find_callable_addr(rec, core::ptr::null_mut(), &mut addr) { return -libc::EINVAL; }
    ftrace_modify_code(pc, aarch64_insn_gen_branch_imm(pc, old_addr, AARCH64_INSN_BRANCH_LINK), aarch64_insn_gen_branch_imm(pc, addr, AARCH64_INSN_BRANCH_LINK), true)
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS")]
pub unsafe fn ftrace_init_nop(_: *mut module, rec: *mut dyn_ftrace) -> i32 {
    let pc = (*rec).ip - AARCH64_INSN_SIZE; let ret = ftrace_rec_set_nop_ops(rec); if ret != 0 { return ret; }
    ftrace_modify_code(pc, aarch64_insn_gen_nop(), aarch64_insn_gen_move_reg(AARCH64_INSN_REG_9, AARCH64_INSN_REG_LR, AARCH64_INSN_VARIANT_64BIT), true)
}

pub unsafe fn ftrace_make_call(rec: *mut dyn_ftrace, mut addr: usize) -> i32 {
    let pc = (*rec).ip;
    let ret = ftrace_rec_update_ops(rec); if ret != 0 { return ret; }
    if !ftrace_find_callable_addr(rec, core::ptr::null_mut(), &mut addr) { return -libc::EINVAL; }
    ftrace_modify_code(pc, aarch64_insn_gen_nop(), aarch64_insn_gen_branch_imm(pc, addr, AARCH64_INSN_BRANCH_LINK), true)
}

pub unsafe fn ftrace_make_nop(mod_: *mut module, rec: *mut dyn_ftrace, mut addr: usize) -> i32 {
    let pc = (*rec).ip; let new = aarch64_insn_gen_nop();
    let ret = ftrace_rec_set_nop_ops(rec); if ret != 0 { return ret; }
    if !cfg!(feature = "CONFIG_DYNAMIC_FTRACE_WITH_ARGS") && !mod_.is_null() { return aarch64_insn_patch_text_nosync(pc as *mut _, new); }
    if !ftrace_find_callable_addr(rec, mod_, &mut addr) { return -libc::EINVAL; }
    ftrace_modify_code(pc, aarch64_insn_gen_branch_imm(pc, addr, AARCH64_INSN_BRANCH_LINK), new, true)
}

pub unsafe fn arch_ftrace_update_code(mut command: i32) { command |= FTRACE_MAY_SLEEP; ftrace_modify_all_code(command); }

// External architecture/kernel declarations and configuration-dependent helpers.
extern "C" { fn warn_ratelimit(_: i32, _: *const core::ffi::c_char, ...); fn get_kernel_nofault(_: *mut u32, _: *mut u32) -> i32; fn aarch64_insn_is_bti(_: u32) -> bool; fn aarch64_insn_gen_nop() -> u32; fn aarch64_insn_gen_move_reg(_: i32, _: i32, _: i32) -> u32; fn aarch64_insn_read(_: *const core::ffi::c_void, _: *mut u32) -> i32; fn aarch64_insn_patch_text_nosync(_: *mut core::ffi::c_void, _: u32) -> i32; fn aarch64_insn_gen_branch_imm(_: usize, _: usize, _: i32) -> u32; fn aarch64_insn_write_literal_u64(_: *mut core::ffi::c_void, _: usize) -> i32; fn ftrace_call(); fn ftrace_modify_all_code(_: i32); }
#[repr(C)] pub struct dyn_ftrace { pub ip: usize, pub flags: u32 }
#[repr(C)] pub struct module { _private: [u8; 0] }
const AARCH64_INSN_SIZE: usize = 4; const PAGE_MASK: usize = !0xfff; const SZ_128M: usize = 128 * 1024 * 1024; const FTRACE_MAY_SLEEP: i32 = 1 << 0; const AARCH64_INSN_BRANCH_LINK: i32 = 1;
const FTRACE_ADDR: usize = 0; const AARCH64_INSN_REG_9: i32 = 9; const AARCH64_INSN_REG_LR: i32 = 30; const AARCH64_INSN_VARIANT_64BIT: i32 = 0;
#[repr(C)] pub struct plt_entry { _private: [u8; 0] }
#[repr(C)] pub struct ftrace_ops { _private: [u8; 0] }
extern "C" { static ftrace_nop_ops: ftrace_ops; static ftrace_list_ops: ftrace_ops; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
