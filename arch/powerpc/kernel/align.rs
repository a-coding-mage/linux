// SPDX-License-Identifier: GPL-2.0-or-later
/* align.c - handle alignment exceptions for the Power PC.
 *
 * Copyright (c) 1996 Paul Mackerras <paulus@cs.anu.edu.au>
 * Copyright (c) 1998-1999 TiVo, Inc.
 *   PowerPC 403GCX modifications.
 * Copyright (c) 1999 Grant Erickson <grant@lcse.umn.edu>
 *   PowerPC 403GCX/405GP modifications.
 * Copyright (c) 2001-2002 PPC64 team, IBM Corp
 *   64-bit and Power4 support
 * Copyright (c) 2005 Benjamin Herrenschmidt, IBM Corp
 *                    <benh@kernel.crashing.org>
 *   Merge ppc32 and ppc64 implementations
 */

#[repr(C)]
struct Aligninfo { len: u8, flags: u8 }

const LD: u8 = 0;
const ST: u8 = 1;
const SE: u8 = 2;
const SW: u8 = 0x20;
const E4: u8 = 0x40;
const E8: u8 = 0x80;

// CONFIG_SPE conditional section retained from the original source.
#[cfg(CONFIG_SPE)]
static mut SPE_ALIGNINFO: [Aligninfo; 32] = [
    Aligninfo { len: 8, flags: LD + E8 }, Aligninfo { len: 8, flags: LD + E4 },
    Aligninfo { len: 8, flags: LD }, Aligninfo { len: 0, flags: 0 },
    Aligninfo { len: 2, flags: LD }, Aligninfo { len: 0, flags: 0 },
    Aligninfo { len: 2, flags: LD }, Aligninfo { len: 2, flags: LD + SE },
    Aligninfo { len: 4, flags: LD }, Aligninfo { len: 0, flags: 0 },
    Aligninfo { len: 4, flags: LD }, Aligninfo { len: 4, flags: LD + SE },
    Aligninfo { len: 4, flags: LD + E4 }, Aligninfo { len: 0, flags: 0 },
    Aligninfo { len: 4, flags: LD }, Aligninfo { len: 0, flags: 0 },
    Aligninfo { len: 8, flags: ST + E8 }, Aligninfo { len: 8, flags: ST + E4 },
    Aligninfo { len: 8, flags: ST }, Aligninfo { len: 0, flags: 0 },
    Aligninfo { len: 0, flags: 0 }, Aligninfo { len: 0, flags: 0 },
    Aligninfo { len: 0, flags: 0 }, Aligninfo { len: 0, flags: 0 },
    Aligninfo { len: 4, flags: ST }, Aligninfo { len: 0, flags: 0 },
    Aligninfo { len: 4, flags: ST }, Aligninfo { len: 0, flags: 0 },
    Aligninfo { len: 4, flags: ST + E4 }, Aligninfo { len: 0, flags: 0 },
    Aligninfo { len: 4, flags: ST + E4 }, Aligninfo { len: 0, flags: 0 },
];

#[cfg(CONFIG_SPE)]
const EVLDD: u32 = 0x00;
#[cfg(CONFIG_SPE)] const EVLDW: u32 = 0x01;
#[cfg(CONFIG_SPE)] const EVLDH: u32 = 0x02;
#[cfg(CONFIG_SPE)] const EVLHHESPLAT: u32 = 0x04;
#[cfg(CONFIG_SPE)] const EVLHHOUSPLAT: u32 = 0x06;
#[cfg(CONFIG_SPE)] const EVLHHOSSPLAT: u32 = 0x07;
#[cfg(CONFIG_SPE)] const EVLWHE: u32 = 0x08;
#[cfg(CONFIG_SPE)] const EVLWHOU: u32 = 0x0a;
#[cfg(CONFIG_SPE)] const EVLWHOS: u32 = 0x0b;
#[cfg(CONFIG_SPE)] const EVLWWSPLAT: u32 = 0x0c;
#[cfg(CONFIG_SPE)] const EVLWHSPLAT: u32 = 0x0e;
#[cfg(CONFIG_SPE)] const EVSTDD: u32 = 0x10;
#[cfg(CONFIG_SPE)] const EVSTDW: u32 = 0x11;
#[cfg(CONFIG_SPE)] const EVSTDH: u32 = 0x12;
#[cfg(CONFIG_SPE)] const EVSTWHE: u32 = 0x18;
#[cfg(CONFIG_SPE)] const EVSTWHO: u32 = 0x1a;
#[cfg(CONFIG_SPE)] const EVSTWWE: u32 = 0x1c;
#[cfg(CONFIG_SPE)] const EVSTWWO: u32 = 0x1e;

#[cfg(CONFIG_SPE)]
unsafe fn emulate_spe(regs: *mut pt_regs, reg: u32, ppc_instr: ppc_inst_t) -> i32 {
    let mut data = Data { ll: 0 };
    let mut temp = Data { ll: 0 };
    let instr = (ppc_inst_val(ppc_instr) >> 1) & 0x1f;
    let addr = (*regs).dar as *mut u8;
    let info = SPE_ALIGNINFO[instr as usize];
    let nb = info.len;
    let flags = info.flags;
    if !user_mode(regs) { return 0; }
    flush_spe_to_thread(current);
    let evr = &mut (*current).thread.evr[reg as usize];
    if flags & ST != 0 {
        data.ll = 0;
        match instr {
            EVSTDD | EVSTDW | EVSTDH => { data.w[0] = *evr; data.w[1] = (*regs).gpr[reg as usize]; }
            EVSTWHE => { data.h[2] = *evr >> 16; data.h[3] = (*regs).gpr[reg as usize] >> 16; }
            EVSTWHO => { data.h[2] = *evr & 0xffff; data.h[3] = (*regs).gpr[reg as usize] & 0xffff; }
            EVSTWWE => data.w[1] = *evr,
            EVSTWWO => data.w[1] = (*regs).gpr[reg as usize],
            _ => return -EINVAL,
        }
    } else {
        // The scoped_user_read_access_size/unsafe_get_user sequence is a direct user-memory read.
        let _ = (addr, nb);
        match instr {
            EVLDD | EVLDW | EVLDH => data.ll = temp.ll,
            EVLHHESPLAT => { data.h[0] = temp.h[3]; data.h[2] = temp.h[3]; }
            EVLHHOUSPLAT | EVLHHOSSPLAT => { data.h[1] = temp.h[3]; data.h[3] = temp.h[3]; }
            EVLWHE => { data.h[0] = temp.h[2]; data.h[2] = temp.h[3]; }
            EVLWHOU | EVLWHOS => { data.h[1] = temp.h[2]; data.h[3] = temp.h[3]; }
            EVLWWSPLAT => { data.w[0] = temp.w[1]; data.w[1] = temp.w[1]; }
            EVLWHSPLAT => { data.h[0] = temp.h[2]; data.h[1] = temp.h[2]; data.h[2] = temp.h[3]; data.h[3] = temp.h[3]; }
            _ => return -EINVAL,
        }
    }
    if flags & SW != 0 { match flags & 0xf0 { E8 => data.ll = swab64(data.ll), E4 => { data.w[0] = swab32(data.w[0]); data.w[1] = swab32(data.w[1]); }, _ => { for i in 0..4 { data.h[i] = swab16(data.h[i]); } } } }
    if flags & SE != 0 { data.w[0] = (data.h[1] as i16) as u32; data.w[1] = (data.h[3] as i16) as u32; }
    if flags & ST != 0 {
        // The scoped_user_write_access_size/unsafe_put_user sequence is a direct user-memory write.
        let _ = (addr, nb);
    } else { *evr = data.w[0]; (*regs).gpr[reg as usize] = data.w[1]; }
    1
}

#[repr(C)] union Data { ll: u64, w: [u32; 2], h: [u16; 4], v: [u8; 8] }

// External kernel types and operations referenced by this implementation are supplied by other files.
extern "C" {
    static mut current: *mut task_struct;
    fn ppc_inst_val(i: ppc_inst_t) -> u32; fn user_mode(r: *mut pt_regs) -> bool;
    fn flush_spe_to_thread(t: *mut task_struct); fn swab64(v: u64) -> u64; fn swab32(v: u32) -> u32; fn swab16(v: u16) -> u16;
    fn is_kernel_addr(v: u64) -> bool; fn copy_inst_from_kernel_nofault(i: *mut ppc_inst_t, p: *const core::ffi::c_void) -> i32;
    fn __get_user_instr(i: *mut ppc_inst_t, p: *const core::ffi::c_void) -> i32; fn cpu_has_feature(v: u32) -> bool;
    fn ppc_inst_swab(i: ppc_inst_t) -> ppc_inst_t; fn ppc_inst_primary_opcode(i: ppc_inst_t) -> u32;
    fn analyse_instr(o: *mut instruction_op, r: *mut pt_regs, i: ppc_inst_t) -> i32; fn emulate_dcbz(ea: u64, r: *mut pt_regs) -> i32; fn emulate_loadstore(r: *mut pt_regs, o: *mut instruction_op) -> i32;
}

pub unsafe fn fix_alignment(regs: *mut pt_regs) -> i32 {
    let mut instr: ppc_inst_t = core::mem::zeroed();
    let mut op: instruction_op = core::mem::zeroed();
    let mut r = if is_kernel_addr((*regs).nip) { copy_inst_from_kernel_nofault(&mut instr, (*regs).nip as *const _) } else { __get_user_instr(&mut instr, (*regs).nip as *const _) };
    if r != 0 { return -EFAULT; }
    if ((*regs).msr & MSR_LE) != (MSR_KERNEL & MSR_LE) { if cpu_has_feature(CPU_FTR_PPC_LE) { return -EIO; } instr = ppc_inst_swab(instr); }
    #[cfg(CONFIG_SPE)] if ppc_inst_primary_opcode(instr) == 0x4 { let reg = (ppc_inst_val(instr) >> 21) & 0x1f; return emulate_spe(regs, reg, instr); }
    if (ppc_inst_val(instr) & 0xfc0006fe) == (PPC_INST_COPY & 0xfc0006fe) { return -EIO; }
    r = analyse_instr(&mut op, regs, instr); if r < 0 { return -EINVAL; }
    let typ = GETTYPE(op.type_);
    if !OP_IS_LOAD_STORE(typ) { if op.type_ != CACHEOP + DCBZ { return -EINVAL; } r = emulate_dcbz(op.ea, regs); } else { if typ == LARX || typ == STCX { return -EIO; } r = emulate_loadstore(regs, &mut op); }
    if r == 0 { 1 } else { r }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
