// SPDX-License-Identifier: GPL-2.0
// External kernel and instruction-decoder dependencies are supplied elsewhere.

unsafe fn decode_branch_type(insn: *mut insn) -> i32 {
    let mut ext: i32;

    if insn_get_opcode(insn) != 0 {
        return X86_BR_ABORT;
    }

    match (*insn).opcode.bytes[0] {
        0xf => match (*insn).opcode.bytes[1] {
            0x05 | 0x34 => X86_BR_SYSCALL,
            0x07 | 0x35 => X86_BR_SYSRET,
            0x80..=0x8f => X86_BR_JCC,
            _ => X86_BR_NONE,
        },
        0x70..=0x7f => X86_BR_JCC,
        0xc2 | 0xc3 | 0xca | 0xcb => X86_BR_RET,
        0xcf => X86_BR_IRET,
        0xcc..=0xce => X86_BR_INT,
        0xe8 => {
            if insn_get_immediate(insn) != 0 || (*insn).immediate1.value == 0 {
                return X86_BR_ZERO_CALL;
            }
            X86_BR_CALL
        }
        0x9a => X86_BR_CALL,
        0xe0..=0xe3 => X86_BR_JCC,
        0xe9..=0xeb => X86_BR_JMP,
        0xff => {
            if insn_get_modrm(insn) != 0 {
                return X86_BR_ABORT;
            }
            ext = (((*insn).modrm.bytes[0] >> 3) & 0x7) as i32;
            match ext {
                2 | 3 => X86_BR_IND_CALL,
                4 | 5 => X86_BR_IND_JMP,
                _ => X86_BR_NONE,
            }
        }
        _ => X86_BR_NONE,
    }
}

/*
 * return the type of control flow change at address "from"
 * instruction is not necessarily a branch (in case of interrupt).
 *
 * The branch type returned also includes the priv level of the
 * target of the control flow change (X86_BR_USER, X86_BR_KERNEL).
 *
 * If a branch type is unknown OR the instruction cannot be
 * decoded (e.g., text page not present), then X86_BR_NONE is
 * returned.
 *
 * While recording branches, some processors can report the "from"
 * address to be that of an instruction preceding the actual branch
 * when instruction fusion occurs. If fusion is expected, attempt to
 * find the type of the first branch instruction within the next
 * MAX_INSN_SIZE bytes and if found, provide the offset between the
 * reported "from" address and the actual branch instruction address.
 */
unsafe fn get_branch_type(from: c_ulong, to: c_ulong, abort: i32, fused: bool, offset: *mut i32) -> i32 {
    let mut insn: insn;
    let mut addr: *mut c_void;
    let mut bytes_read: i32;
    let mut bytes_left: i32;
    let mut insn_offset: i32;
    let mut ret: i32 = X86_BR_NONE;
    let to_plm: i32;
    let from_plm: i32;
    let mut buf: [u8; MAX_INSN_SIZE as usize];
    let mut is64: i32 = 0;

    if !offset.is_null() { *offset = 0; }
    to_plm = if kernel_ip(to) { X86_BR_KERNEL } else { X86_BR_USER };
    from_plm = if kernel_ip(from) { X86_BR_KERNEL } else { X86_BR_USER };
    if from == 0 || to == 0 { return X86_BR_NONE; }
    if abort != 0 { return X86_BR_ABORT | to_plm; }

    if from_plm == X86_BR_USER {
        if (*current).mm.is_null() { return X86_BR_NONE; }
        bytes_left = copy_from_user_nmi(buf.as_mut_ptr(), from as *const c_void, MAX_INSN_SIZE);
        bytes_read = MAX_INSN_SIZE - bytes_left;
        if bytes_read == 0 { return X86_BR_NONE; }
        addr = buf.as_mut_ptr() as *mut c_void;
    } else {
        if kernel_text_address(from) && !in_gate_area_no_mm(from) {
            addr = from as *mut c_void;
            bytes_read = MAX_INSN_SIZE;
        } else { return X86_BR_NONE; }
    }

    // CONFIG_X86_64: retain the 64-bit ABI detection conditional.
    #[cfg(target_arch = "x86_64")]
    { is64 = if kernel_ip(addr as c_ulong) || any_64bit_mode(current_pt_regs()) { 1 } else { 0 }; }
    insn_init(&mut insn, addr, bytes_read, is64 != 0);
    ret = decode_branch_type(&mut insn);
    insn_offset = 0;

    while fused && ret == X86_BR_NONE {
        if insn_get_length(&mut insn) != 0 || insn.length == 0 { break; }
        insn_offset += insn.length as i32;
        bytes_read -= insn.length as i32;
        if bytes_read < 0 { break; }
        insn_init(&mut insn, addr.add(insn_offset as usize), bytes_read, is64 != 0);
        ret = decode_branch_type(&mut insn);
    }
    if !offset.is_null() { *offset = insn_offset; }
    if from_plm == X86_BR_USER && to_plm == X86_BR_KERNEL
        && ret != X86_BR_SYSCALL && ret != X86_BR_INT { ret = X86_BR_IRQ; }
    if ret != X86_BR_NONE { ret |= to_plm; }
    ret
}

pub unsafe fn branch_type(from: c_ulong, to: c_ulong, abort: i32) -> i32 {
    get_branch_type(from, to, abort, false, core::ptr::null_mut())
}

pub unsafe fn branch_type_fused(from: c_ulong, to: c_ulong, abort: i32, offset: *mut i32) -> i32 {
    get_branch_type(from, to, abort, true, offset)
}

const X86_BR_TYPE_MAP_MAX: usize = 16;
static BRANCH_MAP: [i32; X86_BR_TYPE_MAP_MAX] = [
    PERF_BR_CALL, PERF_BR_RET, PERF_BR_SYSCALL, PERF_BR_SYSRET,
    PERF_BR_UNKNOWN, PERF_BR_ERET, PERF_BR_COND, PERF_BR_UNCOND,
    PERF_BR_IRQ, PERF_BR_IND_CALL, PERF_BR_UNKNOWN, PERF_BR_UNKNOWN,
    PERF_BR_NO_TX, PERF_BR_CALL, PERF_BR_UNKNOWN, PERF_BR_IND,
];

pub fn common_branch_type(mut type_: i32) -> i32 {
    type_ >>= 2;
    if type_ != 0 {
        let i = type_.trailing_zeros() as usize;
        if i < X86_BR_TYPE_MAP_MAX { return BRANCH_MAP[i]; }
    }
    PERF_BR_UNKNOWN
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
