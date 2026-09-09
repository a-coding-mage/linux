// SPDX-License-Identifier: GPL-2.0
/* Tests for presence or absence of hardware registers.
 * This code was originally in atari/config.c, but I noticed
 * that it was also in drivers/nubus/nubus.c and I wanted to
 * use it in hp300/config.c, so it seemed sensible to pull it
 * out into its own file.
 *
 * The test is for use when trying to read a hardware register
 * that isn't present would cause a bus error. We set up a
 * temporary handler so that this doesn't kill the kernel.
 *
 * There is a test-by-reading and a test-by-writing; I present
 * them here complete with the comments from the original atari
 * config.c...
 *                -- PMM <pmaydell@chiark.greenend.org.uk>, 05/1998
 */

/* This function tests for the presence of an address, specially a
 * hardware register address. It is called very early in the kernel
 * initialization process, when the VBR register isn't set up yet. On
 * an Atari, it still points to address 0, which is unmapped. So a bus
 * error would cause another bus error while fetching the exception
 * vector, and the CPU would do nothing at all. So we needed to set up
 * a temporary VBR and a vector table for the duration of the test.
 */

unsafe extern "C" {
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
}

#[no_mangle]
pub unsafe extern "C" fn hwreg_present(regp: *mut core::ffi::c_void) -> i32 {
    let mut ret: i32 = 0;
    let mut flags: usize = 0;
    let mut save_sp: isize;
    let mut save_vbr: isize;
    let mut tmp_vectors: [isize; 3] = [0; 3];

    local_irq_save(&mut flags as *mut usize);
    core::arch::asm!(
        "movec %/vbr,{save_vbr}",
        "movel #Lberr1,{tmp_vectors}@(8)",
        "movec {tmp_vectors},%/vbr",
        "movel %/sp,{save_sp}",
        "moveq #0,{ret}",
        "tstb {regp}@",
        "nop",
        "moveq #1,{ret}",
        "Lberr1:",
        "movel {save_sp},%/sp",
        "movec {save_vbr},%/vbr",
        save_sp = lateout(reg) save_sp,
        save_vbr = lateout(reg) save_vbr,
        ret = lateout(reg) ret,
        regp = in(reg) regp,
        tmp_vectors = in(reg) tmp_vectors.as_mut_ptr(),
        options(nostack)
    );
    local_irq_restore(flags);

    ret
}

/* Basically the same, but writes a value into a word register, protected
 * by a bus error handler. Returns 1 if successful, 0 otherwise.
 */

#[no_mangle]
pub unsafe extern "C" fn hwreg_write(
    regp: *mut core::ffi::c_void,
    val: u16,
) -> i32 {
    let mut ret: i32 = 0;
    let mut flags: usize = 0;
    let mut save_sp: isize;
    let mut save_vbr: isize;
    let mut tmp_vectors: [isize; 3] = [0; 3];

    local_irq_save(&mut flags as *mut usize);
    core::arch::asm!(
        "movec %/vbr,{save_vbr}",
        "movel #Lberr2,{tmp_vectors}@(8)",
        "movec {tmp_vectors},%/vbr",
        "movel %/sp,{save_sp}",
        "moveq #0,{ret}",
        "movew {val},{regp}@",
        "nop",
        /*
         * If this nop isn't present, 'ret' may already be loaded
         * with 1 at the time the bus error happens!
         */
        "moveq #1,{ret}",
        "Lberr2:",
        "movel {save_sp},%/sp",
        "movec {save_vbr},%/vbr",
        save_sp = lateout(reg) save_sp,
        save_vbr = lateout(reg) save_vbr,
        ret = lateout(reg) ret,
        regp = in(reg) regp,
        tmp_vectors = in(reg) tmp_vectors.as_mut_ptr(),
        val = in(reg) val,
        options(nostack)
    );
    local_irq_restore(flags);

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
