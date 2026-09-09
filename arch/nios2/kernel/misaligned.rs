/*
 *  linux/arch/nios2/kernel/misaligned.c
 *
 *  basic emulation for mis-aligned accesses on the NIOS II cpu
 *  modelled after the version for arm in arm/alignment.c
 *
 *  Brad Parker <brad@heeltoe.com>
 *  Copyright (C) 2010 Ambient Corporation
 *  Copyright (c) 2010 Altera Corporation, San Jose, California, USA.
 *  Copyright (c) 2010 Arrow Electronics, Inc.
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file COPYING in the main directory of
 * this archive for more details.
 */

// External kernel types and functions supplied by other files.
use core::ffi::c_int;

const INST_LDHU: u32 = 0x0b;
const INST_STH: u32 = 0x0d;
const INST_LDH: u32 = 0x0f;
const INST_STW: u32 = 0x15;
const INST_LDW: u32 = 0x17;

static mut MA_USERMODE: u32 = 0;
const UM_WARN: u32 = 0x01;
const UM_FIXUP: u32 = 0x02;
const UM_SIGNAL: u32 = 0x04;
const KM_WARN: u32 = 0x08;

#[repr(C)]
pub struct PtRegs {
    pub ea: usize,
    pub sp: usize,
    pub ra: usize,
}

#[repr(C)]
pub struct SwitchStack {
    _private: [u8; 0],
}

extern "C" {
    fn fixup_exception(fp: *mut PtRegs) -> bool;
    fn user_mode(fp: *mut PtRegs) -> bool;
    fn rdctl(control: c_int) -> u32;
    fn exception(signal: c_int, fp: *mut PtRegs, code: c_int, address: usize);
    fn pr_err(format: *const u8, ...);
    static mut current_pid: c_int;
}

const CTL_BADADDR: c_int = 0;
const SIGSEGV: c_int = 11;
const SEGV_MAPERR: c_int = 1;
const SIGBUS: c_int = 7;
const BUS_ADRALN: c_int = 1;

static SYS_STACK_FRAME_REG_OFFSET: [u8; 32] = [
    /* struct pt_regs */
    8, 9, 10, 11, 12, 13, 14, 15, 1, 2, 3, 4, 5, 6, 7, 0,
    /* struct switch_stack */
    16, 17, 18, 19, 20, 21, 22, 23, 0, 0, 0, 0, 0, 0, 0, 0,
];

static mut REG_OFFSETS: [isize; 32] = [0; 32];

#[inline]
unsafe fn get_reg_val(fp: *mut PtRegs, reg: usize) -> u32 {
    let p = (fp as *mut u8).offset(REG_OFFSETS[reg]) as *const u32;
    p.read()
}

#[inline]
unsafe fn put_reg_val(fp: *mut PtRegs, reg: usize, val: u32) {
    let p = (fp as *mut u8).offset(REG_OFFSETS[reg]) as *mut u32;
    p.write(val);
}

/*
 * (mis)alignment handler
 */
#[no_mangle]
pub unsafe extern "C" fn handle_unaligned_c(fp: *mut PtRegs, mut cause: c_int) {
    let mut isn: u32;
    let mut addr: u32;
    let mut val: u32;
    let in_kernel: bool;
    let mut a: u8;
    let mut b: u8;
    let mut d0: u8;
    let mut d1: u8;
    let mut d2: u8;
    let mut d3: u8;
    let mut imm16: i16;
    let mut fault: u32;

    (*fp).ea -= 4;

    if fixup_exception(fp) {
        return;
    }

    in_kernel = !user_mode(fp);
    isn = ((*fp).ea as *const u32).read();
    fault = 0;

    /* do fixup if in kernel or mode turned on */
    if in_kernel || (MA_USERMODE & UM_FIXUP) != 0 {
        /* decompose instruction */
        a = ((isn >> 27) & 0x1f) as u8;
        b = ((isn >> 22) & 0x1f) as u8;
        imm16 = ((isn >> 6) & 0xffff) as i16;
        addr = get_reg_val(fp, a as usize).wrapping_add(imm16 as u32);

        /* do fixup to saved registers */
        match isn & 0x3f {
            INST_LDHU => {
                d0 = (addr as *const u8).read();
                d1 = (addr.wrapping_add(1) as *const u8).read();
                val = ((d1 as u32) << 8) | d0 as u32;
                put_reg_val(fp, b as usize, val);
            }
            INST_STH => {
                val = get_reg_val(fp, b as usize);
                d1 = (val >> 8) as u8;
                d0 = val as u8;
                (addr as *mut u8).write(d0);
                (addr.wrapping_add(1) as *mut u8).write(d1);
            }
            INST_LDH => {
                d0 = (addr as *const u8).read();
                d1 = (addr.wrapping_add(1) as *const u8).read();
                val = (((((d1 as u16) << 8) | d0 as u16) as i16) as i32) as u32;
                put_reg_val(fp, b as usize, val);
            }
            INST_STW => {
                val = get_reg_val(fp, b as usize);
                d3 = (val >> 24) as u8;
                d2 = (val >> 16) as u8;
                d1 = (val >> 8) as u8;
                d0 = val as u8;
                (addr as *mut u8).write(d0);
                (addr.wrapping_add(1) as *mut u8).write(d1);
                (addr.wrapping_add(2) as *mut u8).write(d2);
                (addr.wrapping_add(3) as *mut u8).write(d3);
            }
            INST_LDW => {
                d0 = (addr as *const u8).read();
                d1 = (addr.wrapping_add(1) as *const u8).read();
                d2 = (addr.wrapping_add(2) as *const u8).read();
                d3 = (addr.wrapping_add(3) as *const u8).read();
                val = ((d3 as u32) << 24) | ((d2 as u32) << 16) | ((d1 as u32) << 8) | d0 as u32;
                put_reg_val(fp, b as usize, val);
            }
            _ => {}
        }
    }

    addr = rdctl(CTL_BADADDR);
    cause >>= 2;

    if fault != 0 {
        if in_kernel {
            pr_err(b"fault during kernel misaligned fixup @ %#lx; addr 0x%08x; isn=0x%08x\0".as_ptr(), (*fp).ea, addr, isn);
        } else {
            pr_err(b"fault during user misaligned fixup @ %#lx; isn=%08x addr=0x%08x sp=0x%08lx pid=%d\0".as_ptr(), (*fp).ea, isn, addr, (*fp).sp, current_pid);
            exception(SIGSEGV, fp, SEGV_MAPERR, (*fp).ea);
            return;
        }
    }

    if in_kernel {
        (*fp).ea += 4;
        if (MA_USERMODE & KM_WARN) != 0 {
            pr_err(b"kernel unaligned access @ %#lx; BADADDR 0x%08x; cause=%d, isn=0x%08x\0".as_ptr(), (*fp).ea, addr, cause, isn);
        }
        return;
    }

    if (MA_USERMODE & UM_WARN) != 0 {
        pr_err(b"user unaligned access @ %#lx; isn=0x%08lx ea=0x%08lx ra=0x%08lx sp=0x%08lx\0".as_ptr(), addr as usize, isn as usize, (*fp).ea, (*fp).ra, (*fp).sp);
    }

    if (MA_USERMODE & UM_SIGNAL) != 0 {
        exception(SIGBUS, fp, BUS_ADRALN, (*fp).ea);
    } else {
        (*fp).ea += 4;
    }
}

unsafe fn misaligned_calc_reg_offsets() {
    let mut offset: isize = 0;
    for i in 0..16 {
        let r = SYS_STACK_FRAME_REG_OFFSET[i] as usize;
        REG_OFFSETS[r] = offset;
        offset += 4;
    }

    offset = -(core::mem::size_of::<SwitchStack>() as isize);
    for i in 16..32 {
        let r = SYS_STACK_FRAME_REG_OFFSET[i] as usize;
        REG_OFFSETS[r] = offset;
        offset += 4;
    }
}

unsafe fn misaligned_init() -> c_int {
    /* default mode - silent fix */
    MA_USERMODE = UM_FIXUP | KM_WARN;
    misaligned_calc_reg_offsets();
    0
}

// Equivalent of fs_initcall(misaligned_init).
#[used]
static MISALIGNED_INIT: unsafe fn() -> c_int = misaligned_init;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
