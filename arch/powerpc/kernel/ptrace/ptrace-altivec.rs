// SPDX-License-Identifier: GPL-2.0-or-later

// <linux/regset.h>
// <linux/elf.h>
// <asm/switch_to.h>
// "ptrace-decl.h"

/*
 * Get/set all the altivec registers vr0..vr31, vscr, vrsave, in one go.
 * The transfer totals 34 quadword.  Quadwords 0-31 contain the
 * corresponding vector registers.  Quadword 32 contains the vscr as the
 * last word (offset 12) within that quadword.  Quadword 33 contains the
 * vrsave as the first word (offset 0) within the quadword.
 *
 * This definition of the VMX state is compatible with the current PPC32
 * ptrace interface.  This allows signal handling and ptrace to use the
 * same structures.  This also simplifies the implementation of a bi-arch
 * (combined (32- and 64-bit) gdb.
 */

pub unsafe fn vr_active(
    target: *mut task_struct,
    regset: *const user_regset,
) -> i32 {
    unsafe { flush_altivec_to_thread(target) };
    unsafe { if (*target).thread.used_vr { (*regset).n } else { 0 } }
}

/*
 * Regardless of transactions, 'vr_state' holds the current running
 * value of all the VMX registers and 'ckvr_state' holds the last
 * checkpointed value of all the VMX registers for the current
 * transaction to fall back on in case it aborts.
 *
 * Userspace interface buffer layout:
 *
 * struct data {
 *  vector128 vr[32];
 *  vector128 vscr;
 *  vector128 vrsave;
 * };
 */
pub unsafe fn vr_get(
    target: *mut task_struct,
    _regset: *const user_regset,
    mut to: membuf,
) -> i32 {
    let mut vrsave: elf_vrreg_t = unsafe { core::mem::zeroed() };

    unsafe { flush_altivec_to_thread(target) };

    // BUILD_BUG_ON(offsetof(struct thread_vr_state, vscr) !=
    //              offsetof(struct thread_vr_state, vr[32]));

    unsafe {
        membuf_write(
            &mut to,
            &(*target).thread.vr_state as *const _ as *const core::ffi::c_void,
            33 * core::mem::size_of::<vector128>(),
        );
        /*
         * Copy out only the low-order word of vrsave.
         */
        core::ptr::write_bytes(
            &mut vrsave as *mut _ as *mut u8,
            0,
            core::mem::size_of::<elf_vrreg_t>(),
        );
        vrsave.word = (*target).thread.vrsave;
        membuf_write(
            &mut to,
            &vrsave as *const _ as *const core::ffi::c_void,
            core::mem::size_of::<elf_vrreg_t>(),
        )
    }
}

/*
 * Regardless of transactions, 'vr_state' holds the current running
 * value of all the VMX registers and 'ckvr_state' holds the last
 * checkpointed value of all the VMX registers for the current
 * transaction to fall back on in case it aborts.
 *
 * Userspace interface buffer layout:
 *
 * struct data {
 *  vector128 vr[32];
 *  vector128 vscr;
 *  vector128 vrsave;
 * };
 */
pub unsafe fn vr_set(
    target: *mut task_struct,
    _regset: *const user_regset,
    mut pos: u32,
    mut count: u32,
    mut kbuf: *const core::ffi::c_void,
    mut ubuf: *const core::ffi::c_void,
) -> i32 {
    let mut ret: i32;

    unsafe { flush_altivec_to_thread(target) };

    // BUILD_BUG_ON(offsetof(struct thread_vr_state, vscr) !=
    //              offsetof(struct thread_vr_state, vr[32]));

    ret = unsafe {
        user_regset_copyin(
            &mut pos, &mut count, &mut kbuf, &mut ubuf,
            &mut (*target).thread.vr_state as *mut _ as *mut core::ffi::c_void,
            0, 33 * core::mem::size_of::<vector128>(),
        )
    };
    if ret == 0 && count > 0 {
        /*
         * We use only the first word of vrsave.
         */
        let start: u32 = (33 * core::mem::size_of::<vector128>()) as u32;
        let end: u32 = start + core::mem::size_of::<elf_vrreg_t>() as u32;
        let mut vrsave: elf_vrreg_t = unsafe { core::mem::zeroed() };

        vrsave.word = unsafe { (*target).thread.vrsave };
        ret = unsafe {
            user_regset_copyin(
                &mut pos, &mut count, &mut kbuf, &mut ubuf,
                &mut vrsave as *mut _ as *mut core::ffi::c_void,
                start, end,
            )
        };
        if ret == 0 {
            unsafe { (*target).thread.vrsave = vrsave.word };
        }
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
