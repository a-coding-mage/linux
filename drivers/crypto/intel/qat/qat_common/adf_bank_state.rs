// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2025 Intel Corporation */

// C dependencies supplied by the surrounding kernel translation.

const ADF_RP_INT_SRC_SEL_F_RISE_MASK: u32 = (1u32 << 2) - 1;
const ADF_RP_INT_SRC_SEL_F_FALL_MASK: u32 = (1u32 << 3) - 1;
const ADF_RP_INT_SRC_SEL_RANGE_WIDTH: u32 = 4;

unsafe fn check_stat(
    op: unsafe extern "C" fn(*mut core::ffi::c_void, u32) -> u32,
    expect_val: u32,
    name: *const core::ffi::c_char,
    base: *mut core::ffi::c_void,
    bank: u32,
) -> i32 {
    let actual_val = op(base, bank);

    if expect_val == actual_val {
        return 0;
    }

    pr_err!("Fail to restore %s register. Expected %#x, actual %#x\n", name, expect_val, actual_val);
    -22
}

unsafe fn bank_state_save(
    ops: *mut adf_hw_csr_ops,
    base: *mut core::ffi::c_void,
    bank: u32,
    state: *mut adf_bank_state,
    num_rings: u32,
) {
    (*state).ringstat0 = ((*ops).read_csr_stat)(base, bank);
    (*state).ringuostat = ((*ops).read_csr_uo_stat)(base, bank);
    (*state).ringestat = ((*ops).read_csr_e_stat)(base, bank);
    (*state).ringnestat = ((*ops).read_csr_ne_stat)(base, bank);
    (*state).ringnfstat = ((*ops).read_csr_nf_stat)(base, bank);
    (*state).ringfstat = ((*ops).read_csr_f_stat)(base, bank);
    (*state).ringcstat0 = ((*ops).read_csr_c_stat)(base, bank);
    (*state).iaintflagen = ((*ops).read_csr_int_en)(base, bank);
    (*state).iaintflagreg = ((*ops).read_csr_int_flag)(base, bank);
    (*state).iaintflagsrcsel0 = ((*ops).read_csr_int_srcsel)(base, bank);
    (*state).iaintcolen = ((*ops).read_csr_int_col_en)(base, bank);
    (*state).iaintcolctl = ((*ops).read_csr_int_col_ctl)(base, bank);
    (*state).iaintflagandcolen = ((*ops).read_csr_int_flag_and_col)(base, bank);
    (*state).ringexpstat = ((*ops).read_csr_exp_stat)(base, bank);
    (*state).ringexpintenable = ((*ops).read_csr_exp_int_en)(base, bank);
    (*state).ringsrvarben = ((*ops).read_csr_ring_srv_arb_en)(base, bank);

    for i in 0..num_rings {
        (*state).rings[i as usize].head = ((*ops).read_csr_ring_head)(base, bank, i);
        (*state).rings[i as usize].tail = ((*ops).read_csr_ring_tail)(base, bank, i);
        (*state).rings[i as usize].config = ((*ops).read_csr_ring_config)(base, bank, i);
        (*state).rings[i as usize].base = ((*ops).read_csr_ring_base)(base, bank, i);
    }
}

unsafe fn bank_state_restore(
    ops: *mut adf_hw_csr_ops, base: *mut core::ffi::c_void, bank: u32,
    state: *mut adf_bank_state, num_rings: u32, tx_rx_gap: i32,
) -> i32 {
    for i in 0..num_rings { ((*ops).write_csr_ring_base)(base, bank, i, (*state).rings[i as usize].base); }
    for i in 0..num_rings { ((*ops).write_csr_ring_config)(base, bank, i, (*state).rings[i as usize].config); }

    for i in 0..num_rings / 2 {
        let tx = (i as i32 * (tx_rx_gap + 1)) as u32;
        let rx = (tx as i32 + tx_rx_gap) as u32;
        ((*ops).write_csr_ring_head)(base, bank, tx, (*state).rings[tx as usize].head);
        ((*ops).write_csr_ring_tail)(base, bank, tx, (*state).rings[tx as usize].tail);
        if (*state).ringestat & (1u32 << tx) != 0 {
            let mut val = ((*ops).read_csr_int_srcsel)(base, bank);
            val |= ADF_RP_INT_SRC_SEL_F_RISE_MASK;
            ((*ops).write_csr_int_srcsel_w_val)(base, bank, val);
            ((*ops).write_csr_ring_head)(base, bank, tx, (*state).rings[tx as usize].head);
        }
        ((*ops).write_csr_ring_tail)(base, bank, rx, (*state).rings[rx as usize].tail);
        let mut val = ((*ops).read_csr_int_srcsel)(base, bank);
        val |= ADF_RP_INT_SRC_SEL_F_RISE_MASK << ADF_RP_INT_SRC_SEL_RANGE_WIDTH;
        ((*ops).write_csr_int_srcsel_w_val)(base, bank, val);
        ((*ops).write_csr_ring_head)(base, bank, rx, (*state).rings[rx as usize].head);
        val = ((*ops).read_csr_int_srcsel)(base, bank);
        val |= ADF_RP_INT_SRC_SEL_F_FALL_MASK << ADF_RP_INT_SRC_SEL_RANGE_WIDTH;
        ((*ops).write_csr_int_srcsel_w_val)(base, bank, val);
        if (*state).ringfstat & (1u32 << rx) != 0 { ((*ops).write_csr_ring_tail)(base, bank, rx, (*state).rings[rx as usize].tail); }
    }

    ((*ops).write_csr_int_flag_and_col)(base, bank, (*state).iaintflagandcolen);
    ((*ops).write_csr_int_en)(base, bank, (*state).iaintflagen);
    ((*ops).write_csr_int_col_en)(base, bank, (*state).iaintcolen);
    ((*ops).write_csr_int_srcsel_w_val)(base, bank, (*state).iaintflagsrcsel0);
    ((*ops).write_csr_exp_int_en)(base, bank, (*state).ringexpintenable);
    ((*ops).write_csr_int_col_ctl)(base, bank, (*state).iaintcolctl);
    if (*state).ringexpstat != 0 { pr_info!("Bank %u state not fully restored due to exception in saved state (%#x)\n", bank, (*state).ringexpstat); return 0; }
    let tmp_val = ((*ops).read_csr_exp_stat)(base, bank);
    if tmp_val != 0 { pr_err!("Bank %u restored with exception: %#x\n", bank, tmp_val); return -14; }
    ((*ops).write_csr_ring_srv_arb_en)(base, bank, (*state).ringsrvarben);
    let checks = [((*ops).read_csr_stat, (*state).ringstat0, c"ringstat"), ((*ops).read_csr_e_stat, (*state).ringestat, c"ringestat"), ((*ops).read_csr_ne_stat, (*state).ringnestat, c"ringnestat"), ((*ops).read_csr_nf_stat, (*state).ringnfstat, c"ringnfstat"), ((*ops).read_csr_f_stat, (*state).ringfstat, c"ringfstat"), ((*ops).read_csr_c_stat, (*state).ringcstat0, c"ringcstat")];
    for (op, expected, name) in checks { let ret = check_stat(op, expected, name.as_ptr(), base, bank); if ret != 0 { return ret; } }
    0
}

// Public entry points.  GET_HW_DATA, GET_CSR_OPS, adf_get_etr_base, and
// device logging are supplied by the surrounding kernel translation.
pub unsafe fn adf_bank_state_save(accel_dev: *mut adf_accel_dev, bank_number: u32, state: *mut adf_bank_state) -> i32 {
    let hw_data = GET_HW_DATA(accel_dev);
    let csr_ops = GET_CSR_OPS(accel_dev);
    let csr_base = adf_get_etr_base(accel_dev);
    if bank_number >= (*hw_data).num_banks || state.is_null() { return -22; }
    dev_dbg!(GET_DEV(accel_dev), "Saving state of bank %d\n", bank_number);
    bank_state_save(csr_ops, csr_base, bank_number, state, (*hw_data).num_rings_per_bank);
    0
}

pub unsafe fn adf_bank_state_restore(accel_dev: *mut adf_accel_dev, bank_number: u32, state: *mut adf_bank_state) -> i32 {
    let hw_data = GET_HW_DATA(accel_dev);
    let csr_ops = GET_CSR_OPS(accel_dev);
    let csr_base = adf_get_etr_base(accel_dev);
    if bank_number >= (*hw_data).num_banks || state.is_null() { return -22; }
    dev_dbg!(GET_DEV(accel_dev), "Restoring state of bank %d\n", bank_number);
    let ret = bank_state_restore(csr_ops, csr_base, bank_number, state, (*hw_data).num_rings_per_bank, (*hw_data).tx_rx_gap);
    if ret != 0 { dev_err!(GET_DEV(accel_dev), "Unable to restore state of bank %d\n", bank_number); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
