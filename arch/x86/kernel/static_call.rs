// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;

/* Declared locally to avoid pulling asm/paravirt-spinlock.h header. */
#[cfg(feature = "CONFIG_PARAVIRT_SPINLOCKS")]
#[repr(C)]
pub struct qspinlock;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum insn_type {
    CALL = 0,
    NOP = 1,
    JMP = 2,
    RET = 3,
    JCC = 4,
}

/* ud1 %esp, %ecx - a 3 byte #UD unique to trampolines and a speculation stop. */
static tramp_ud: [u8; 3] = [0x0f, 0xb9, 0xcc];
/* cs cs cs xorl %eax, %eax - a single 5 byte instruction clearing %[er]ax. */
static xor5rax: [u8; 5] = [0x2e, 0x2e, 0x2e, 0x31, 0xc0];
static retinsn: [u8; 5] = [RET_INSN_OPCODE as u8, 0xcc, 0xcc, 0xcc, 0xcc];
/* ud1 (%edx),%rdi -- see __WARN_trap() / decode_bug(). */
static warninsn: [u8; 5] = [0x67, 0x48, 0x0f, 0xb9, 0x3a];

#[cfg(feature = "CONFIG_PARAVIRT_SPINLOCKS")]
#[cfg(feature = "CONFIG_64BIT")]
static unlockinsn: [u8; 5] = [0x3e, 0x3e, 0xc6, 0x07, 0x00];
#[cfg(feature = "CONFIG_PARAVIRT_SPINLOCKS")]
#[cfg(not(feature = "CONFIG_64BIT"))]
static unlockinsn: [u8; 5] = [0x3e, 0x3e, 0xc6, 0x00, 0x00];

unsafe fn __is_Jcc(insn: *mut u8) -> u8 {
    let mut ret = 0u8;
    if *insn == 0x0f {
        let tmp = *insn.add(1);
        if (tmp & 0xf0) == 0x80 { ret = tmp; }
    }
    ret
}

extern "C" {
    fn __static_call_return();
    fn __static_call_return0();
    fn __WARN_trap();
    #[cfg(feature = "CONFIG_PARAVIRT_SPINLOCKS")]
    fn __raw_callee_save___native_queued_spin_unlock(lock: *mut qspinlock);
}

unsafe fn __static_call_transform(insn: *mut c_void, mut typ: insn_type,
                                  mut func: *mut c_void, modinit: bool) {
    let mut emulate: *const c_void = core::ptr::null();
    let mut size: usize = CALL_INSN_SIZE as usize;
    let mut code: *const c_void;
    let mut op = 0u8;
    let mut buf = [0u8; 6];

    if (typ == insn_type::JMP || typ == insn_type::RET) && { op = __is_Jcc(insn as *mut u8); op != 0 } {
        typ = insn_type::JCC;
    }
    match typ {
        insn_type::CALL => {
            func = callthunks_translate_call_dest(func);
            code = text_gen_insn(CALL_INSN_OPCODE, insn, func);
            if func == &__static_call_return0 as *const _ as *mut c_void { emulate = code; code = xor5rax.as_ptr() as *const c_void; }
            if func == &__WARN_trap as *const _ as *mut c_void { emulate = code; code = warninsn.as_ptr() as *const c_void; }
            #[cfg(feature = "CONFIG_PARAVIRT_SPINLOCKS")]
            if func == __raw_callee_save___native_queued_spin_unlock as *const _ as *mut c_void { emulate = code; code = unlockinsn.as_ptr() as *const c_void; }
        }
        insn_type::NOP => { code = x86_nops[5].as_ptr() as *const c_void; }
        insn_type::JMP => { code = text_gen_insn(JMP32_INSN_OPCODE, insn, func); }
        insn_type::RET => { code = if cpu_wants_rethunk_at(insn) { text_gen_insn(JMP32_INSN_OPCODE, insn, x86_return_thunk) } else { retinsn.as_ptr() as *const c_void }; }
        insn_type::JCC => {
            if func.is_null() { func = __static_call_return as *const _ as *mut c_void; if cpu_wants_rethunk() { func = x86_return_thunk; } }
            buf[0] = 0x0f;
            __text_gen_insn(buf.as_mut_ptr().add(1), op, (insn as *mut u8).add(1), func, 5);
            code = buf.as_ptr() as *const c_void; size = 6;
        }
    }
    if libc_memcmp(insn, code, size) == 0 { return; }
    if system_state == SYSTEM_BOOTING || modinit { text_poke_early(insn, code, size); } else { smp_text_poke_single(insn, code, size, emulate); }
}

unsafe fn __static_call_validate(insn: *mut u8, tail: bool, tramp: bool) {
    let opcode = *insn;
    if tramp && libc_memcmp(insn.add(5), tramp_ud.as_ptr(), 3) != 0 { pr_err("trampoline signature fail"); BUG(); }
    if tail { if opcode == JMP32_INSN_OPCODE || opcode == RET_INSN_OPCODE || __is_Jcc(insn) != 0 { return; } }
    else if opcode == CALL_INSN_OPCODE || libc_memcmp(insn, x86_nops[5].as_ptr(), 5) == 0 || libc_memcmp(insn, xor5rax.as_ptr(), 5) == 0 || libc_memcmp(insn, warninsn.as_ptr(), 5) == 0 { return; }
    #[cfg(feature = "CONFIG_PARAVIRT_SPINLOCKS")]
    if libc_memcmp(insn, unlockinsn.as_ptr(), 5) == 0 { return; }
    pr_err("unexpected static_call insn opcode 0x%x at %pS\n", opcode, insn); BUG();
}

#[inline]
unsafe fn __sc_insn(null: bool, tail: bool) -> insn_type { core::mem::transmute((2 * tail as i32 + null as i32) as i32) }

pub unsafe fn arch_static_call_transform(site: *mut c_void, tramp: *mut c_void, func: *mut c_void, tail: bool) {
    mutex_lock(&text_mutex);
    if !tramp.is_null() && site.is_null() { __static_call_validate(tramp as *mut u8, true, true); __static_call_transform(tramp, __sc_insn(func.is_null(), true), func, false); }
    if IS_ENABLED(CONFIG_HAVE_STATIC_CALL_INLINE) && !site.is_null() { __static_call_validate(site as *mut u8, tail, false); __static_call_transform(site, __sc_insn(func.is_null(), tail), func, false); }
    mutex_unlock(&text_mutex);
}

pub unsafe fn __static_call_update_early(tramp: *mut c_void, func: *mut c_void) {
    BUG_ON(system_state != SYSTEM_BOOTING); BUG_ON(static_call_initialized);
    __text_gen_insn(tramp, JMP32_INSN_OPCODE, tramp, func, JMP32_INSN_SIZE); sync_core();
}

#[cfg(feature = "CONFIG_MITIGATION_RETHUNK")]
pub unsafe fn __static_call_fixup(tramp: *mut c_void, op: u8, dest: *mut c_void) -> bool {
    let addr = tramp as usize;
    if ((addr >> PAGE_SHIFT) != ((addr + 7) >> PAGE_SHIFT)) && !kernel_text_address((addr + 7) as *mut c_void) { return false; }
    if libc_memcmp((tramp as *mut u8).add(5), tramp_ud.as_ptr(), 3) != 0 { return false; }
    mutex_lock(&text_mutex);
    if op == RET_INSN_OPCODE || dest == __x86_return_thunk { __static_call_transform(tramp, insn_type::RET, core::ptr::null_mut(), true); }
    mutex_unlock(&text_mutex); true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
