// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * dwarf-regs.c : Mapping of DWARF debug register numbers into register names.
 * Extracted from probe-finder.c
 *
 * Written by Masami Hiramatsu <mhiramat@redhat.com>
 */

use core::ffi::{c_char, c_int, c_ulong};

const EINVAL: c_int = 22;
const ENOENT: c_int = 2;

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

#[repr(C)]
struct dwarf_regs_idx {
    name: *const c_char,
    dwarf_regnum: c_int,
}

unsafe impl Sync for dwarf_regs_idx {}

static i386_regidx_table: [dwarf_regs_idx; 60] = [
    dwarf_regs_idx { name: b"eax\0".as_ptr() as *const c_char, dwarf_regnum: 0 }, dwarf_regs_idx { name: b"ax\0".as_ptr() as *const c_char, dwarf_regnum: 0 }, dwarf_regs_idx { name: b"al\0".as_ptr() as *const c_char, dwarf_regnum: 0 },
    dwarf_regs_idx { name: b"ecx\0".as_ptr() as *const c_char, dwarf_regnum: 1 }, dwarf_regs_idx { name: b"cx\0".as_ptr() as *const c_char, dwarf_regnum: 1 }, dwarf_regs_idx { name: b"cl\0".as_ptr() as *const c_char, dwarf_regnum: 1 },
    dwarf_regs_idx { name: b"edx\0".as_ptr() as *const c_char, dwarf_regnum: 2 }, dwarf_regs_idx { name: b"dx\0".as_ptr() as *const c_char, dwarf_regnum: 2 }, dwarf_regs_idx { name: b"dl\0".as_ptr() as *const c_char, dwarf_regnum: 2 },
    dwarf_regs_idx { name: b"ebx\0".as_ptr() as *const c_char, dwarf_regnum: 3 }, dwarf_regs_idx { name: b"bx\0".as_ptr() as *const c_char, dwarf_regnum: 3 }, dwarf_regs_idx { name: b"bl\0".as_ptr() as *const c_char, dwarf_regnum: 3 },
    dwarf_regs_idx { name: b"esp\0".as_ptr() as *const c_char, dwarf_regnum: 4 }, dwarf_regs_idx { name: b"sp\0".as_ptr() as *const c_char, dwarf_regnum: 4 }, dwarf_regs_idx { name: b"$stack\0".as_ptr() as *const c_char, dwarf_regnum: 4 },
    dwarf_regs_idx { name: b"ebp\0".as_ptr() as *const c_char, dwarf_regnum: 5 }, dwarf_regs_idx { name: b"bp\0".as_ptr() as *const c_char, dwarf_regnum: 5 },
    dwarf_regs_idx { name: b"esi\0".as_ptr() as *const c_char, dwarf_regnum: 6 }, dwarf_regs_idx { name: b"si\0".as_ptr() as *const c_char, dwarf_regnum: 6 },
    dwarf_regs_idx { name: b"edi\0".as_ptr() as *const c_char, dwarf_regnum: 7 }, dwarf_regs_idx { name: b"di\0".as_ptr() as *const c_char, dwarf_regnum: 7 },
    // 8 - Return Address RA
    dwarf_regs_idx { name: b"eflags\0".as_ptr() as *const c_char, dwarf_regnum: 9 }, dwarf_regs_idx { name: b"flags\0".as_ptr() as *const c_char, dwarf_regnum: 9 },
    // 10 - reserved
    dwarf_regs_idx { name: b"st0\0".as_ptr() as *const c_char, dwarf_regnum: 11 },
    dwarf_regs_idx { name: b"st1\0".as_ptr() as *const c_char, dwarf_regnum: 12 },
    dwarf_regs_idx { name: b"st2\0".as_ptr() as *const c_char, dwarf_regnum: 13 },
    dwarf_regs_idx { name: b"st3\0".as_ptr() as *const c_char, dwarf_regnum: 14 },
    dwarf_regs_idx { name: b"st4\0".as_ptr() as *const c_char, dwarf_regnum: 15 },
    dwarf_regs_idx { name: b"st5\0".as_ptr() as *const c_char, dwarf_regnum: 16 },
    dwarf_regs_idx { name: b"st6\0".as_ptr() as *const c_char, dwarf_regnum: 17 },
    dwarf_regs_idx { name: b"st7\0".as_ptr() as *const c_char, dwarf_regnum: 18 },
    // 19-20 - reserved
    dwarf_regs_idx { name: b"xmm0\0".as_ptr() as *const c_char, dwarf_regnum: 21 },
    dwarf_regs_idx { name: b"xmm1\0".as_ptr() as *const c_char, dwarf_regnum: 22 },
    dwarf_regs_idx { name: b"xmm2\0".as_ptr() as *const c_char, dwarf_regnum: 23 },
    dwarf_regs_idx { name: b"xmm3\0".as_ptr() as *const c_char, dwarf_regnum: 24 },
    dwarf_regs_idx { name: b"xmm4\0".as_ptr() as *const c_char, dwarf_regnum: 25 },
    dwarf_regs_idx { name: b"xmm5\0".as_ptr() as *const c_char, dwarf_regnum: 26 },
    dwarf_regs_idx { name: b"xmm6\0".as_ptr() as *const c_char, dwarf_regnum: 27 },
    dwarf_regs_idx { name: b"xmm7\0".as_ptr() as *const c_char, dwarf_regnum: 28 },
    dwarf_regs_idx { name: b"mm0\0".as_ptr() as *const c_char, dwarf_regnum: 29 },
    dwarf_regs_idx { name: b"mm1\0".as_ptr() as *const c_char, dwarf_regnum: 30 },
    dwarf_regs_idx { name: b"mm2\0".as_ptr() as *const c_char, dwarf_regnum: 31 },
    dwarf_regs_idx { name: b"mm3\0".as_ptr() as *const c_char, dwarf_regnum: 32 },
    dwarf_regs_idx { name: b"mm4\0".as_ptr() as *const c_char, dwarf_regnum: 33 },
    dwarf_regs_idx { name: b"mm5\0".as_ptr() as *const c_char, dwarf_regnum: 34 },
    dwarf_regs_idx { name: b"mm6\0".as_ptr() as *const c_char, dwarf_regnum: 35 },
    dwarf_regs_idx { name: b"mm7\0".as_ptr() as *const c_char, dwarf_regnum: 36 },
    // 37-38 - unknown
    dwarf_regs_idx { name: b"mxcsr\0".as_ptr() as *const c_char, dwarf_regnum: 39 }, // 128-bit Media Control and Status
    dwarf_regs_idx { name: b"es\0".as_ptr() as *const c_char, dwarf_regnum: 40 },
    dwarf_regs_idx { name: b"cs\0".as_ptr() as *const c_char, dwarf_regnum: 41 },
    dwarf_regs_idx { name: b"ss\0".as_ptr() as *const c_char, dwarf_regnum: 42 },
    dwarf_regs_idx { name: b"ds\0".as_ptr() as *const c_char, dwarf_regnum: 43 },
    dwarf_regs_idx { name: b"fs\0".as_ptr() as *const c_char, dwarf_regnum: 44 },
    dwarf_regs_idx { name: b"gs\0".as_ptr() as *const c_char, dwarf_regnum: 45 },
    // 46-47 - reserved
    dwarf_regs_idx { name: b"tr\0".as_ptr() as *const c_char, dwarf_regnum: 48 }, // Task Register
    dwarf_regs_idx { name: b"ldtr\0".as_ptr() as *const c_char, dwarf_regnum: 49 }, // LDT Register
    // 50-92 - reserved
    dwarf_regs_idx { name: b"fs.base\0".as_ptr() as *const c_char, dwarf_regnum: 92 },
    dwarf_regs_idx { name: b"gs.base\0".as_ptr() as *const c_char, dwarf_regnum: 93 },
    // End of regular dwarf registers.
    dwarf_regs_idx { name: b"eip\0".as_ptr() as *const c_char, dwarf_regnum: DWARF_REG_PC },
    dwarf_regs_idx { name: b"ip\0".as_ptr() as *const c_char, dwarf_regnum: DWARF_REG_PC },
];

static x86_64_regidx_table: [dwarf_regs_idx; 85] = [
    dwarf_regs_idx { name: b"rax\0".as_ptr() as *const c_char, dwarf_regnum: 0 }, dwarf_regs_idx { name: b"eax\0".as_ptr() as *const c_char, dwarf_regnum: 0 }, dwarf_regs_idx { name: b"ax\0".as_ptr() as *const c_char, dwarf_regnum: 0 }, dwarf_regs_idx { name: b"al\0".as_ptr() as *const c_char, dwarf_regnum: 0 },
    dwarf_regs_idx { name: b"rdx\0".as_ptr() as *const c_char, dwarf_regnum: 1 }, dwarf_regs_idx { name: b"edx\0".as_ptr() as *const c_char, dwarf_regnum: 1 }, dwarf_regs_idx { name: b"dx\0".as_ptr() as *const c_char, dwarf_regnum: 1 }, dwarf_regs_idx { name: b"dl\0".as_ptr() as *const c_char, dwarf_regnum: 1 },
    dwarf_regs_idx { name: b"rcx\0".as_ptr() as *const c_char, dwarf_regnum: 2 }, dwarf_regs_idx { name: b"ecx\0".as_ptr() as *const c_char, dwarf_regnum: 2 }, dwarf_regs_idx { name: b"cx\0".as_ptr() as *const c_char, dwarf_regnum: 2 }, dwarf_regs_idx { name: b"cl\0".as_ptr() as *const c_char, dwarf_regnum: 2 },
    dwarf_regs_idx { name: b"rbx\0".as_ptr() as *const c_char, dwarf_regnum: 3 }, dwarf_regs_idx { name: b"edx\0".as_ptr() as *const c_char, dwarf_regnum: 3 }, dwarf_regs_idx { name: b"bx\0".as_ptr() as *const c_char, dwarf_regnum: 3 }, dwarf_regs_idx { name: b"bl\0".as_ptr() as *const c_char, dwarf_regnum: 3 },
    dwarf_regs_idx { name: b"rsi\0".as_ptr() as *const c_char, dwarf_regnum: 4 }, dwarf_regs_idx { name: b"esi\0".as_ptr() as *const c_char, dwarf_regnum: 4 }, dwarf_regs_idx { name: b"si\0".as_ptr() as *const c_char, dwarf_regnum: 4 }, dwarf_regs_idx { name: b"sil\0".as_ptr() as *const c_char, dwarf_regnum: 4 },
    dwarf_regs_idx { name: b"rdi\0".as_ptr() as *const c_char, dwarf_regnum: 5 }, dwarf_regs_idx { name: b"edi\0".as_ptr() as *const c_char, dwarf_regnum: 5 }, dwarf_regs_idx { name: b"di\0".as_ptr() as *const c_char, dwarf_regnum: 5 }, dwarf_regs_idx { name: b"dil\0".as_ptr() as *const c_char, dwarf_regnum: 5 },
    dwarf_regs_idx { name: b"rbp\0".as_ptr() as *const c_char, dwarf_regnum: 6 }, dwarf_regs_idx { name: b"ebp\0".as_ptr() as *const c_char, dwarf_regnum: 6 }, dwarf_regs_idx { name: b"bp\0".as_ptr() as *const c_char, dwarf_regnum: 6 }, dwarf_regs_idx { name: b"bpl\0".as_ptr() as *const c_char, dwarf_regnum: 6 },
    dwarf_regs_idx { name: b"rsp\0".as_ptr() as *const c_char, dwarf_regnum: 7 }, dwarf_regs_idx { name: b"esp\0".as_ptr() as *const c_char, dwarf_regnum: 7 }, dwarf_regs_idx { name: b"sp\0".as_ptr() as *const c_char, dwarf_regnum: 7 }, dwarf_regs_idx { name: b"spl\0".as_ptr() as *const c_char, dwarf_regnum: 7 },
    dwarf_regs_idx { name: b"r8\0".as_ptr() as *const c_char, dwarf_regnum: 8 }, dwarf_regs_idx { name: b"r8d\0".as_ptr() as *const c_char, dwarf_regnum: 8 }, dwarf_regs_idx { name: b"r8w\0".as_ptr() as *const c_char, dwarf_regnum: 8 }, dwarf_regs_idx { name: b"r8b\0".as_ptr() as *const c_char, dwarf_regnum: 8 },
    dwarf_regs_idx { name: b"r9\0".as_ptr() as *const c_char, dwarf_regnum: 9 }, dwarf_regs_idx { name: b"r9d\0".as_ptr() as *const c_char, dwarf_regnum: 9 }, dwarf_regs_idx { name: b"r9w\0".as_ptr() as *const c_char, dwarf_regnum: 9 }, dwarf_regs_idx { name: b"r9b\0".as_ptr() as *const c_char, dwarf_regnum: 9 },
    dwarf_regs_idx { name: b"r10\0".as_ptr() as *const c_char, dwarf_regnum: 10 }, dwarf_regs_idx { name: b"r10d\0".as_ptr() as *const c_char, dwarf_regnum: 10 }, dwarf_regs_idx { name: b"r10w\0".as_ptr() as *const c_char, dwarf_regnum: 10 }, dwarf_regs_idx { name: b"r10b\0".as_ptr() as *const c_char, dwarf_regnum: 10 },
    dwarf_regs_idx { name: b"r11\0".as_ptr() as *const c_char, dwarf_regnum: 11 }, dwarf_regs_idx { name: b"r11d\0".as_ptr() as *const c_char, dwarf_regnum: 11 }, dwarf_regs_idx { name: b"r11w\0".as_ptr() as *const c_char, dwarf_regnum: 11 }, dwarf_regs_idx { name: b"r11b\0".as_ptr() as *const c_char, dwarf_regnum: 11 },
    dwarf_regs_idx { name: b"r12\0".as_ptr() as *const c_char, dwarf_regnum: 12 }, dwarf_regs_idx { name: b"r12d\0".as_ptr() as *const c_char, dwarf_regnum: 12 }, dwarf_regs_idx { name: b"r12w\0".as_ptr() as *const c_char, dwarf_regnum: 12 }, dwarf_regs_idx { name: b"r12b\0".as_ptr() as *const c_char, dwarf_regnum: 12 },
    dwarf_regs_idx { name: b"r13\0".as_ptr() as *const c_char, dwarf_regnum: 13 }, dwarf_regs_idx { name: b"r13d\0".as_ptr() as *const c_char, dwarf_regnum: 13 }, dwarf_regs_idx { name: b"r13w\0".as_ptr() as *const c_char, dwarf_regnum: 13 }, dwarf_regs_idx { name: b"r13b\0".as_ptr() as *const c_char, dwarf_regnum: 13 },
    dwarf_regs_idx { name: b"r14\0".as_ptr() as *const c_char, dwarf_regnum: 14 }, dwarf_regs_idx { name: b"r14d\0".as_ptr() as *const c_char, dwarf_regnum: 14 }, dwarf_regs_idx { name: b"r14w\0".as_ptr() as *const c_char, dwarf_regnum: 14 }, dwarf_regs_idx { name: b"r14b\0".as_ptr() as *const c_char, dwarf_regnum: 14 },
    dwarf_regs_idx { name: b"r15\0".as_ptr() as *const c_char, dwarf_regnum: 15 }, dwarf_regs_idx { name: b"r15d\0".as_ptr() as *const c_char, dwarf_regnum: 15 }, dwarf_regs_idx { name: b"r15w\0".as_ptr() as *const c_char, dwarf_regnum: 15 }, dwarf_regs_idx { name: b"r15b\0".as_ptr() as *const c_char, dwarf_regnum: 15 },
    // 16 - Return Address RA
    dwarf_regs_idx { name: b"xmm0\0".as_ptr() as *const c_char, dwarf_regnum: 17 },
    dwarf_regs_idx { name: b"xmm1\0".as_ptr() as *const c_char, dwarf_regnum: 18 },
    dwarf_regs_idx { name: b"xmm2\0".as_ptr() as *const c_char, dwarf_regnum: 19 },
    dwarf_regs_idx { name: b"xmm3\0".as_ptr() as *const c_char, dwarf_regnum: 20 },
    dwarf_regs_idx { name: b"xmm4\0".as_ptr() as *const c_char, dwarf_regnum: 21 },
    dwarf_regs_idx { name: b"xmm5\0".as_ptr() as *const c_char, dwarf_regnum: 22 },
    dwarf_regs_idx { name: b"xmm6\0".as_ptr() as *const c_char, dwarf_regnum: 23 },
    dwarf_regs_idx { name: b"xmm7\0".as_ptr() as *const c_char, dwarf_regnum: 24 },
    dwarf_regs_idx { name: b"xmm8\0".as_ptr() as *const c_char, dwarf_regnum: 25 },
    dwarf_regs_idx { name: b"xmm9\0".as_ptr() as *const c_char, dwarf_regnum: 26 },
    dwarf_regs_idx { name: b"xmm10\0".as_ptr() as *const c_char, dwarf_regnum: 27 },
    dwarf_regs_idx { name: b"xmm11\0".as_ptr() as *const c_char, dwarf_regnum: 28 },
    dwarf_regs_idx { name: b"xmm12\0".as_ptr() as *const c_char, dwarf_regnum: 29 },
    dwarf_regs_idx { name: b"xmm13\0".as_ptr() as *const c_char, dwarf_regnum: 30 },
    dwarf_regs_idx { name: b"xmm14\0".as_ptr() as *const c_char, dwarf_regnum: 31 },
    dwarf_regs_idx { name: b"xmm15\0".as_ptr() as *const c_char, dwarf_regnum: 32 },
    dwarf_regs_idx { name: b"st0\0".as_ptr() as *const c_char, dwarf_regnum: 33 },
    dwarf_regs_idx { name: b"st1\0".as_ptr() as *const c_char, dwarf_regnum: 34 },
    dwarf_regs_idx { name: b"st2\0".as_ptr() as *const c_char, dwarf_regnum: 35 },
    dwarf_regs_idx { name: b"st3\0".as_ptr() as *const c_char, dwarf_regnum: 36 },
    dwarf_regs_idx { name: b"st4\0".as_ptr() as *const c_char, dwarf_regnum: 37 },
    dwarf_regs_idx { name: b"st5\0".as_ptr() as *const c_char, dwarf_regnum: 38 },
    dwarf_regs_idx { name: b"st6\0".as_ptr() as *const c_char, dwarf_regnum: 39 },
    dwarf_regs_idx { name: b"st7\0".as_ptr() as *const c_char, dwarf_regnum: 40 },
    dwarf_regs_idx { name: b"mm0\0".as_ptr() as *const c_char, dwarf_regnum: 41 },
    dwarf_regs_idx { name: b"mm1\0".as_ptr() as *const c_char, dwarf_regnum: 42 },
    dwarf_regs_idx { name: b"mm2\0".as_ptr() as *const c_char, dwarf_regnum: 43 },
    dwarf_regs_idx { name: b"mm3\0".as_ptr() as *const c_char, dwarf_regnum: 44 },
    dwarf_regs_idx { name: b"mm4\0".as_ptr() as *const c_char, dwarf_regnum: 45 },
    dwarf_regs_idx { name: b"mm5\0".as_ptr() as *const c_char, dwarf_regnum: 46 },
    dwarf_regs_idx { name: b"mm6\0".as_ptr() as *const c_char, dwarf_regnum: 47 },
    dwarf_regs_idx { name: b"mm7\0".as_ptr() as *const c_char, dwarf_regnum: 48 },
    dwarf_regs_idx { name: b"rflags\0".as_ptr() as *const c_char, dwarf_regnum: 49 }, dwarf_regs_idx { name: b"eflags\0".as_ptr() as *const c_char, dwarf_regnum: 49 }, dwarf_regs_idx { name: b"flags\0".as_ptr() as *const c_char, dwarf_regnum: 49 },
    dwarf_regs_idx { name: b"es\0".as_ptr() as *const c_char, dwarf_regnum: 50 },
    dwarf_regs_idx { name: b"cs\0".as_ptr() as *const c_char, dwarf_regnum: 51 },
    dwarf_regs_idx { name: b"ss\0".as_ptr() as *const c_char, dwarf_regnum: 52 },
    dwarf_regs_idx { name: b"ds\0".as_ptr() as *const c_char, dwarf_regnum: 53 },
    dwarf_regs_idx { name: b"fs\0".as_ptr() as *const c_char, dwarf_regnum: 54 },
    dwarf_regs_idx { name: b"gs\0".as_ptr() as *const c_char, dwarf_regnum: 55 },
    // 56-47 - reserved
    dwarf_regs_idx { name: b"fs.base\0".as_ptr() as *const c_char, dwarf_regnum: 58 },
    dwarf_regs_idx { name: b"gs.base\0".as_ptr() as *const c_char, dwarf_regnum: 59 },
    // 60-61 - reserved
    dwarf_regs_idx { name: b"tr\0".as_ptr() as *const c_char, dwarf_regnum: 62 }, // Task Register
    dwarf_regs_idx { name: b"ldtr\0".as_ptr() as *const c_char, dwarf_regnum: 63 }, // LDT Register
    dwarf_regs_idx { name: b"mxcsr\0".as_ptr() as *const c_char, dwarf_regnum: 64 }, // 128-bit Media Control and Status
    dwarf_regs_idx { name: b"fcw\0".as_ptr() as *const c_char, dwarf_regnum: 65 }, // x87 Control Word
    dwarf_regs_idx { name: b"fsw\0".as_ptr() as *const c_char, dwarf_regnum: 66 }, // x87 Status Word
    // End of regular dwarf registers.
    dwarf_regs_idx { name: b"rip\0".as_ptr() as *const c_char, dwarf_regnum: DWARF_REG_PC }, dwarf_regs_idx { name: b"eip\0".as_ptr() as *const c_char, dwarf_regnum: DWARF_REG_PC }, dwarf_regs_idx { name: b"ip\0".as_ptr() as *const c_char, dwarf_regnum: DWARF_REG_PC },
];

unsafe fn get_regnum(entries: *const dwarf_regs_idx, num_entries: c_ulong, mut name: *const c_char) -> c_int {
    if *name != b'%' as c_char {
        return -EINVAL;
    }

    name = name.add(1);
    let mut i: c_ulong = 0;
    while i < num_entries {
        let entry = entries.add(i as usize);
        if strcmp((*entry).name, name) == 0 {
            return (*entry).dwarf_regnum;
        }
        i += 1;
    }
    -ENOENT
}

#[no_mangle]
pub unsafe extern "C" fn __get_dwarf_regnum_i386(name: *const c_char) -> c_int {
    get_regnum(i386_regidx_table.as_ptr(), i386_regidx_table.len() as c_ulong, name)
}

#[no_mangle]
pub unsafe extern "C" fn __get_dwarf_regnum_x86_64(name: *const c_char) -> c_int {
    get_regnum(x86_64_regidx_table.as_ptr(), x86_64_regidx_table.len() as c_ulong, name)
}

#[no_mangle]
pub extern "C" fn __get_dwarf_regnum_for_perf_regnum_i386(perf_regnum: c_int) -> c_int {
    if perf_regnum == 0 {
        return 0;
    }

    let dwarf_regnum = match perf_regnum {
        PERF_REG_X86_AX => 0,
        PERF_REG_X86_BX => 3,
        PERF_REG_X86_CX => 1,
        PERF_REG_X86_DX => 2,
        PERF_REG_X86_SI => 6,
        PERF_REG_X86_DI => 7,
        PERF_REG_X86_BP => 5,
        PERF_REG_X86_SP => 4,
        PERF_REG_X86_IP => 8,
        PERF_REG_X86_FLAGS => 9,
        PERF_REG_X86_CS => 41,
        PERF_REG_X86_SS => 42,
        PERF_REG_X86_DS => 43,
        PERF_REG_X86_ES => 40,
        PERF_REG_X86_FS => 44,
        PERF_REG_X86_GS => 45,
        PERF_REG_X86_XMM0 => 21,
        PERF_REG_X86_XMM1 => 22,
        PERF_REG_X86_XMM2 => 23,
        PERF_REG_X86_XMM3 => 24,
        PERF_REG_X86_XMM4 => 25,
        PERF_REG_X86_XMM5 => 26,
        PERF_REG_X86_XMM6 => 27,
        PERF_REG_X86_XMM7 => 28,
        _ => 0,
    };

    if perf_regnum < 0 || dwarf_regnum == 0 {
        return -ENOENT;
    }

    dwarf_regnum
}

#[no_mangle]
pub extern "C" fn __get_dwarf_regnum_for_perf_regnum_x86_64(perf_regnum: c_int) -> c_int {
    if perf_regnum == 0 {
        return 0;
    }

    let dwarf_regnum = match perf_regnum {
        PERF_REG_X86_AX => 0,
        PERF_REG_X86_BX => 3,
        PERF_REG_X86_CX => 2,
        PERF_REG_X86_DX => 1,
        PERF_REG_X86_SI => 4,
        PERF_REG_X86_DI => 5,
        PERF_REG_X86_BP => 6,
        PERF_REG_X86_SP => 7,
        PERF_REG_X86_IP => 16,
        PERF_REG_X86_FLAGS => 49,
        PERF_REG_X86_CS => 51,
        PERF_REG_X86_SS => 52,
        PERF_REG_X86_DS => 53,
        PERF_REG_X86_ES => 50,
        PERF_REG_X86_FS => 54,
        PERF_REG_X86_GS => 55,
        PERF_REG_X86_R8 => 8,
        PERF_REG_X86_R9 => 9,
        PERF_REG_X86_R10 => 10,
        PERF_REG_X86_R11 => 11,
        PERF_REG_X86_R12 => 12,
        PERF_REG_X86_R13 => 13,
        PERF_REG_X86_R14 => 14,
        PERF_REG_X86_R15 => 15,
        PERF_REG_X86_XMM0 => 17,
        PERF_REG_X86_XMM1 => 18,
        PERF_REG_X86_XMM2 => 19,
        PERF_REG_X86_XMM3 => 20,
        PERF_REG_X86_XMM4 => 21,
        PERF_REG_X86_XMM5 => 22,
        PERF_REG_X86_XMM6 => 23,
        PERF_REG_X86_XMM7 => 24,
        PERF_REG_X86_XMM8 => 25,
        PERF_REG_X86_XMM9 => 26,
        PERF_REG_X86_XMM10 => 27,
        PERF_REG_X86_XMM11 => 28,
        PERF_REG_X86_XMM12 => 29,
        PERF_REG_X86_XMM13 => 30,
        PERF_REG_X86_XMM14 => 31,
        PERF_REG_X86_XMM15 => 32,
        _ => 0,
    };

    if perf_regnum < 0 || dwarf_regnum == 0 {
        return -ENOENT;
    }

    dwarf_regnum
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
