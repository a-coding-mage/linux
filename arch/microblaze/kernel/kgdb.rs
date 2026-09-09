/*
 * Microblaze KGDB support
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

const GDB_REG: usize = 0;
const GDB_PC: usize = 32;
const GDB_MSR: usize = 33;
const GDB_EAR: usize = 34;
const GDB_ESR: usize = 35;
const GDB_FSR: usize = 36;
const GDB_BTR: usize = 37;
const GDB_PVR: usize = 38;
const GDB_REDR: usize = 50;
const GDB_RPID: usize = 51;
const GDB_RZPR: usize = 52;
const GDB_RTLBX: usize = 53;
const GDB_RTLBSX: usize = 54; /* mfs can't read it */
const GDB_RTLBLO: usize = 55;
const GDB_RTLBHI: usize = 56;

/* keep pvr separately because it is unchangeable */
static mut pvr: pvr_s = pvr_s { pvr: [0; 11] };

pub unsafe fn pt_regs_to_gdb_regs(gdb_regs: *mut c_ulong, regs: *mut pt_regs) {
    let mut i: c_uint;
    let pt_regb = regs as *mut c_ulong;
    let mut temp: c_int;

    /* registers r0 - r31, pc, msr, ear, esr, fsr + do not save pt_mode */
    i = 0;
    while i < (core::mem::size_of::<pt_regs>() / 4 - 1) as c_uint {
        *gdb_regs.add(i as usize) = *pt_regb.add(i as usize);
        i += 1;
    }

    /* Branch target register can't be changed */
    core::arch::asm!("mfs {0}, rbtr", out(reg) temp);
    *gdb_regs.add(GDB_BTR) = temp as c_ulong;

    /* pvr part  - we have 11 pvr regs */
    i = 0;
    while i < (core::mem::size_of::<pvr_s>() / 4) as c_uint {
        *gdb_regs.add(GDB_PVR + i as usize) = (*core::ptr::addr_of!(pvr.pvr))[i as usize] as c_ulong;
        i += 1;
    }

    /* read special registers - can't be changed */
    core::arch::asm!("mfs {0}, redr", out(reg) temp);
    *gdb_regs.add(GDB_REDR) = temp as c_ulong;
    core::arch::asm!("mfs {0}, rpid", out(reg) temp);
    *gdb_regs.add(GDB_RPID) = temp as c_ulong;
    core::arch::asm!("mfs {0}, rzpr", out(reg) temp);
    *gdb_regs.add(GDB_RZPR) = temp as c_ulong;
    core::arch::asm!("mfs {0}, rtlbx", out(reg) temp);
    *gdb_regs.add(GDB_RTLBX) = temp as c_ulong;
    core::arch::asm!("mfs {0}, rtlblo", out(reg) temp);
    *gdb_regs.add(GDB_RTLBLO) = temp as c_ulong;
    core::arch::asm!("mfs {0}, rtlbhi", out(reg) temp);
    *gdb_regs.add(GDB_RTLBHI) = temp as c_ulong;
}

pub unsafe fn gdb_regs_to_pt_regs(gdb_regs: *mut c_ulong, regs: *mut pt_regs) {
    let mut i: c_uint;
    let pt_regb = regs as *mut c_ulong;

    /* pt_regs and gdb_regs have the same 37 values.
     * The rest of gdb_regs are unused and can't be changed.
     * r0 register value can't be changed too. */
    i = 1;
    while i < (core::mem::size_of::<pt_regs>() / 4 - 1) as c_uint {
        *pt_regb.add(i as usize) = *gdb_regs.add(i as usize);
        i += 1;
    }
}

pub unsafe extern "C" fn microblaze_kgdb_break(regs: *mut pt_regs) {
    if kgdb_handle_exception(1, SIGTRAP, 0, regs) != 0 {
        return;
    }

    /* Jump over the first arch_kgdb_breakpoint which is barrier to
     * get kgdb work. The same solution is used for powerpc */
    if core::ptr::read(regs).pc as *const u32 as *const u32 == core::ptr::read(core::ptr::addr_of!(arch_kgdb_ops.gdb_bpt_instr)) as *const _ as *const u32 {
        (*regs).pc += BREAK_INSTR_SIZE as c_ulong;
    }
}

/* untested */
pub unsafe fn sleeping_thread_to_gdb_regs(gdb_regs: *mut c_ulong, p: *mut task_struct) {
    let mut i: c_uint;
    let pt_regb = (*p).thread.regs as *mut c_ulong;

    /* registers r0 - r31, pc, msr, ear, esr, fsr + do not save pt_mode */
    i = 0;
    while i < (core::mem::size_of::<pt_regs>() / 4 - 1) as c_uint {
        *gdb_regs.add(i as usize) = *pt_regb.add(i as usize);
        i += 1;
    }

    /* pvr part  - we have 11 pvr regs */
    i = 0;
    while i < (core::mem::size_of::<pvr_s>() / 4) as c_uint {
        *gdb_regs.add(GDB_PVR + i as usize) = (*core::ptr::addr_of!(pvr.pvr))[i as usize] as c_ulong;
        i += 1;
    }
}

pub unsafe fn kgdb_arch_set_pc(regs: *mut pt_regs, ip: c_ulong) {
    (*regs).pc = ip;
}

pub unsafe fn kgdb_arch_handle_exception(
    _vector: c_int,
    _signo: c_int,
    _err_code: c_int,
    remcom_in_buffer: *mut c_char,
    _remcom_out_buffer: *mut c_char,
    regs: *mut pt_regs,
) -> c_int {
    let mut ptr: *mut c_char;
    let mut address: c_ulong = 0;

    match *remcom_in_buffer {
        b'c' as c_char => {
            /* handle the optional parameter */
            ptr = remcom_in_buffer.add(1);
            if kgdb_hex2long(&mut ptr, &mut address) {
                (*regs).pc = address;
            }

            return 0;
        }
        _ => {}
    }
    -1 /* this means that we do not want to exit from the handler */
}

pub unsafe fn kgdb_arch_init() -> c_int {
    get_pvr(&mut pvr); /* Fill PVR structure */
    0
}

pub unsafe fn kgdb_arch_exit() {
    /* Nothing to do */
}

/*
 * Global data
 */
#[cfg(target_endian = "little")]
pub static arch_kgdb_ops: kgdb_arch = kgdb_arch {
    gdb_bpt_instr: [0x18, 0x00, 0x0c, 0xba], /* brki r16, 0x18 */
};

#[cfg(not(target_endian = "little"))]
pub static arch_kgdb_ops: kgdb_arch = kgdb_arch {
    gdb_bpt_instr: [0xba, 0x0c, 0x00, 0x18], /* brki r16, 0x18 */
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
