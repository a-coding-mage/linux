// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012-2015 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

/*
 * Non-VHE: Both host and guest must save everything.
 */

pub unsafe fn __sysreg_save_state_nvhe(ctxt: *mut kvm_cpu_context) {
    __sysreg_save_el1_state(ctxt);
    __sysreg_save_common_state(ctxt);
    __sysreg_save_user_state(ctxt);
    __sysreg_save_el2_return_state(ctxt);
}

pub unsafe fn __sysreg_restore_state_nvhe(ctxt: *mut kvm_cpu_context) {
    let midr: u64 = ctxt_midr_el1(ctxt);

    __sysreg_restore_el1_state(ctxt, midr, ctxt_sys_reg(ctxt, MPIDR_EL1));
    __sysreg_restore_common_state(ctxt);
    __sysreg_restore_user_state(ctxt);
    __sysreg_restore_el2_return_state(ctxt);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
