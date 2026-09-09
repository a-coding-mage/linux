// SPDX-License-Identifier: GPL-2.0

/*
 * Low level utility routines for interacting with Hyper-V.
 *
 * Copyright (C) 2021, Microsoft, Inc.
 *
 * Author : Michael Kelley <mikelley@microsoft.com>
 */

// Dependencies are supplied by the surrounding kernel translation unit.

/*
 * hv_do_hypercall- Invoke the specified hypercall
 */
pub unsafe fn hv_do_hypercall(control: u64, input: *mut core::ffi::c_void,
                              output: *mut core::ffi::c_void) -> u64 {
    let mut res: arm_smccc_res = core::mem::zeroed();
    let input_address: u64 = if !input.is_null() {
        virt_to_phys(input)
    } else {
        0
    };
    let output_address: u64 = if !output.is_null() {
        virt_to_phys(output)
    } else {
        0
    };

    arm_smccc_1_1_hvc(HV_FUNC_ID, control, input_address, output_address, &mut res);
    res.a0
}

/*
 * hv_do_fast_hypercall8 -- Invoke the specified hypercall
 * with arguments in registers instead of physical memory.
 * Avoids the overhead of virt_to_phys for simple hypercalls.
 */
pub unsafe fn hv_do_fast_hypercall8(code: u16, input: u64) -> u64 {
    let mut res: arm_smccc_res = core::mem::zeroed();
    let control: u64 = (code as u64) | HV_HYPERCALL_FAST_BIT;

    arm_smccc_1_1_hvc(HV_FUNC_ID, control, input, &mut res);
    res.a0
}

/*
 * hv_do_fast_hypercall16 -- Invoke the specified hypercall
 * with arguments in registers instead of physical memory.
 * Avoids the overhead of virt_to_phys for simple hypercalls.
 */
pub unsafe fn hv_do_fast_hypercall16(code: u16, input1: u64, input2: u64) -> u64 {
    let mut res: arm_smccc_res = core::mem::zeroed();
    let control: u64 = (code as u64) | HV_HYPERCALL_FAST_BIT;

    arm_smccc_1_1_hvc(HV_FUNC_ID, control, input1, input2, &mut res);
    res.a0
}

/*
 * Set a single VP register to a 64-bit value.
 */
pub unsafe fn hv_set_vpreg(msr: u32, value: u64) {
    let mut res: arm_smccc_res = core::mem::zeroed();

    arm_smccc_1_1_hvc(
        HV_FUNC_ID,
        HVCALL_SET_VP_REGISTERS | HV_HYPERCALL_FAST_BIT | HV_HYPERCALL_REP_COMP_1,
        HV_PARTITION_ID_SELF,
        HV_VP_INDEX_SELF,
        msr,
        0,
        value,
        0,
        &mut res,
    );

    /*
     * Something is fundamentally broken in the hypervisor if
     * setting a VP register fails. There's really no way to
     * continue as a guest VM, so panic.
     */
    BUG_ON(!hv_result_success(res.a0));
}

/*
 * Get the value of a single VP register.  One version
 * returns just 64 bits and another returns the full 128 bits.
 * The two versions are separate to avoid complicating the
 * calling sequence for the more frequently used 64 bit version.
 */
pub unsafe fn hv_get_vpreg_128(msr: u32, result: *mut hv_get_vp_registers_output) {
    let mut args: arm_smccc_1_2_regs = core::mem::zeroed();
    let mut res: arm_smccc_1_2_regs = core::mem::zeroed();

    args.a0 = HV_FUNC_ID;
    args.a1 = HVCALL_GET_VP_REGISTERS | HV_HYPERCALL_FAST_BIT | HV_HYPERCALL_REP_COMP_1;
    args.a2 = HV_PARTITION_ID_SELF;
    args.a3 = HV_VP_INDEX_SELF;
    args.a4 = msr;

    /*
     * Use the SMCCC 1.2 interface because the results are in registers
     * beyond X0-X3.
     */
    arm_smccc_1_2_hvc(&args, &mut res);

    /*
     * Something is fundamentally broken in the hypervisor if
     * getting a VP register fails. There's really no way to
     * continue as a guest VM, so panic.
     */
    BUG_ON(!hv_result_success(res.a0));

    (*result).as64.low = res.a6;
    (*result).as64.high = res.a7;
}

pub unsafe fn hv_get_vpreg(msr: u32) -> u64 {
    let mut output: hv_get_vp_registers_output = core::mem::zeroed();

    hv_get_vpreg_128(msr, &mut output);

    output.as64.low
}

/*
 * hyperv_report_panic - report a panic to Hyper-V.  This function uses
 * the older version of the Hyper-V interface that admittedly doesn't
 * pass enough information to be useful beyond just recording the
 * occurrence of a panic. The parallel hv_kmsg_dump() uses the
 * new interface that allows reporting 4 Kbytes of data, which is much
 * more useful. Hyper-V on ARM64 always supports the newer interface, but
 * we retain support for the older version because the sysadmin is allowed
 * to disable the newer version via sysctl in case of information security
 * concerns about the more verbose version.
 */
pub unsafe fn hyperv_report_panic(regs: *mut pt_regs, err: isize, in_die: bool) {
    static mut PANIC_REPORTED: bool = false;
    let guest_id: u64;

    /* Don't report a panic to Hyper-V if we're not going to panic */
    if in_die && !panic_on_oops {
        return;
    }

    /*
     * We prefer to report panic on 'die' chain as we have proper
     * registers to report, but if we miss it (e.g. on BUG()) we need
     * to report it on 'panic'.
     *
     * Calling code in the 'die' and 'panic' paths ensures that only
     * one CPU is running this code, so no atomicity is needed.
     */
    if PANIC_REPORTED {
        return;
    }
    PANIC_REPORTED = true;

    guest_id = hv_get_vpreg(HV_REGISTER_GUEST_OS_ID);

    /*
     * Hyper-V provides the ability to store only 5 values.
     * Pick the passed in error value, the guest_id, the PC,
     * and the SP.
     */
    hv_set_vpreg(HV_REGISTER_GUEST_CRASH_P0, err as u64);
    hv_set_vpreg(HV_REGISTER_GUEST_CRASH_P1, guest_id);
    hv_set_vpreg(HV_REGISTER_GUEST_CRASH_P2, (*regs).pc);
    hv_set_vpreg(HV_REGISTER_GUEST_CRASH_P3, (*regs).sp);
    hv_set_vpreg(HV_REGISTER_GUEST_CRASH_P4, 0);

    /*
     * Let Hyper-V know there is crash data available
     */
    hv_set_vpreg(HV_REGISTER_GUEST_CRASH_CTL, HV_CRASH_CTL_CRASH_NOTIFY);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
