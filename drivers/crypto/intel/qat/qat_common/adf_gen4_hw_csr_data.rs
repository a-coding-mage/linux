// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2024 Intel Corporation */

// Definitions and macros are supplied by adf_gen4_hw_csr_data.h and other
// dependencies in the translated repository.

unsafe fn build_csr_ring_base_addr(addr: dma_addr_t, size: u32) -> u64 {
    BUILD_RING_BASE_ADDR(addr, size)
}

unsafe fn read_csr_ring_head(csr_base_addr: *mut core::ffi::c_void, bank: u32, ring: u32) -> u32 {
    READ_CSR_RING_HEAD(csr_base_addr, bank, ring)
}

unsafe fn write_csr_ring_head(csr_base_addr: *mut core::ffi::c_void, bank: u32, ring: u32, value: u32) {
    WRITE_CSR_RING_HEAD(csr_base_addr, bank, ring, value);
}

unsafe fn read_csr_ring_tail(csr_base_addr: *mut core::ffi::c_void, bank: u32, ring: u32) -> u32 {
    READ_CSR_RING_TAIL(csr_base_addr, bank, ring)
}

unsafe fn write_csr_ring_tail(csr_base_addr: *mut core::ffi::c_void, bank: u32, ring: u32, value: u32) {
    WRITE_CSR_RING_TAIL(csr_base_addr, bank, ring, value);
}

unsafe fn read_csr_stat(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_STAT(csr_base_addr, bank) }
unsafe fn read_csr_uo_stat(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_UO_STAT(csr_base_addr, bank) }
unsafe fn read_csr_e_stat(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_E_STAT(csr_base_addr, bank) }
unsafe fn read_csr_ne_stat(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_NE_STAT(csr_base_addr, bank) }
unsafe fn read_csr_nf_stat(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_NF_STAT(csr_base_addr, bank) }
unsafe fn read_csr_f_stat(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_F_STAT(csr_base_addr, bank) }
unsafe fn read_csr_c_stat(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_C_STAT(csr_base_addr, bank) }
unsafe fn read_csr_exp_stat(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_EXP_STAT(csr_base_addr, bank) }
unsafe fn read_csr_exp_int_en(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_EXP_INT_EN(csr_base_addr, bank) }
unsafe fn write_csr_exp_int_en(csr_base_addr: *mut core::ffi::c_void, bank: u32, value: u32) { WRITE_CSR_EXP_INT_EN(csr_base_addr, bank, value); }
unsafe fn read_csr_ring_config(csr_base_addr: *mut core::ffi::c_void, bank: u32, ring: u32) -> u32 { READ_CSR_RING_CONFIG(csr_base_addr, bank, ring) }
unsafe fn write_csr_ring_config(csr_base_addr: *mut core::ffi::c_void, bank: u32, ring: u32, value: u32) { WRITE_CSR_RING_CONFIG(csr_base_addr, bank, ring, value); }
unsafe fn read_csr_ring_base(csr_base_addr: *mut core::ffi::c_void, bank: u32, ring: u32) -> dma_addr_t { READ_CSR_RING_BASE(csr_base_addr, bank, ring) }
unsafe fn write_csr_ring_base(csr_base_addr: *mut core::ffi::c_void, bank: u32, ring: u32, addr: dma_addr_t) { WRITE_CSR_RING_BASE(csr_base_addr, bank, ring, addr); }
unsafe fn read_csr_int_en(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_INT_EN(csr_base_addr, bank) }
unsafe fn write_csr_int_en(csr_base_addr: *mut core::ffi::c_void, bank: u32, value: u32) { WRITE_CSR_INT_EN(csr_base_addr, bank, value); }
unsafe fn read_csr_int_flag(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_INT_FLAG(csr_base_addr, bank) }
unsafe fn write_csr_int_flag(csr_base_addr: *mut core::ffi::c_void, bank: u32, value: u32) { WRITE_CSR_INT_FLAG(csr_base_addr, bank, value); }
unsafe fn read_csr_int_srcsel(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_INT_SRCSEL(csr_base_addr, bank) }
unsafe fn write_csr_int_srcsel(csr_base_addr: *mut core::ffi::c_void, bank: u32) { WRITE_CSR_INT_SRCSEL(csr_base_addr, bank); }
unsafe fn write_csr_int_srcsel_w_val(csr_base_addr: *mut core::ffi::c_void, bank: u32, value: u32) { WRITE_CSR_INT_SRCSEL_W_VAL(csr_base_addr, bank, value); }
unsafe fn read_csr_int_col_en(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_INT_COL_EN(csr_base_addr, bank) }
unsafe fn write_csr_int_col_en(csr_base_addr: *mut core::ffi::c_void, bank: u32, value: u32) { WRITE_CSR_INT_COL_EN(csr_base_addr, bank, value); }
unsafe fn read_csr_int_col_ctl(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_INT_COL_CTL(csr_base_addr, bank) }
unsafe fn write_csr_int_col_ctl(csr_base_addr: *mut core::ffi::c_void, bank: u32, value: u32) { WRITE_CSR_INT_COL_CTL(csr_base_addr, bank, value); }
unsafe fn read_csr_int_flag_and_col(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_INT_FLAG_AND_COL(csr_base_addr, bank) }
unsafe fn write_csr_int_flag_and_col(csr_base_addr: *mut core::ffi::c_void, bank: u32, value: u32) { WRITE_CSR_INT_FLAG_AND_COL(csr_base_addr, bank, value); }
unsafe fn read_csr_ring_srv_arb_en(csr_base_addr: *mut core::ffi::c_void, bank: u32) -> u32 { READ_CSR_RING_SRV_ARB_EN(csr_base_addr, bank) }
unsafe fn write_csr_ring_srv_arb_en(csr_base_addr: *mut core::ffi::c_void, bank: u32, value: u32) { WRITE_CSR_RING_SRV_ARB_EN(csr_base_addr, bank, value); }

unsafe fn get_int_col_ctl_enable_mask() -> u32 {
    ADF_RING_CSR_INT_COL_CTL_ENABLE
}

pub unsafe fn adf_gen4_init_hw_csr_ops(csr_ops: *mut adf_hw_csr_ops) {
    (*csr_ops).build_csr_ring_base_addr = Some(build_csr_ring_base_addr);
    (*csr_ops).read_csr_ring_head = Some(read_csr_ring_head);
    (*csr_ops).write_csr_ring_head = Some(write_csr_ring_head);
    (*csr_ops).read_csr_ring_tail = Some(read_csr_ring_tail);
    (*csr_ops).write_csr_ring_tail = Some(write_csr_ring_tail);
    (*csr_ops).read_csr_stat = Some(read_csr_stat);
    (*csr_ops).read_csr_uo_stat = Some(read_csr_uo_stat);
    (*csr_ops).read_csr_e_stat = Some(read_csr_e_stat);
    (*csr_ops).read_csr_ne_stat = Some(read_csr_ne_stat);
    (*csr_ops).read_csr_nf_stat = Some(read_csr_nf_stat);
    (*csr_ops).read_csr_f_stat = Some(read_csr_f_stat);
    (*csr_ops).read_csr_c_stat = Some(read_csr_c_stat);
    (*csr_ops).read_csr_exp_stat = Some(read_csr_exp_stat);
    (*csr_ops).read_csr_exp_int_en = Some(read_csr_exp_int_en);
    (*csr_ops).write_csr_exp_int_en = Some(write_csr_exp_int_en);
    (*csr_ops).read_csr_ring_config = Some(read_csr_ring_config);
    (*csr_ops).write_csr_ring_config = Some(write_csr_ring_config);
    (*csr_ops).read_csr_ring_base = Some(read_csr_ring_base);
    (*csr_ops).write_csr_ring_base = Some(write_csr_ring_base);
    (*csr_ops).read_csr_int_en = Some(read_csr_int_en);
    (*csr_ops).write_csr_int_en = Some(write_csr_int_en);
    (*csr_ops).read_csr_int_flag = Some(read_csr_int_flag);
    (*csr_ops).write_csr_int_flag = Some(write_csr_int_flag);
    (*csr_ops).read_csr_int_srcsel = Some(read_csr_int_srcsel);
    (*csr_ops).write_csr_int_srcsel = Some(write_csr_int_srcsel);
    (*csr_ops).write_csr_int_srcsel_w_val = Some(write_csr_int_srcsel_w_val);
    (*csr_ops).read_csr_int_col_en = Some(read_csr_int_col_en);
    (*csr_ops).write_csr_int_col_en = Some(write_csr_int_col_en);
    (*csr_ops).read_csr_int_col_ctl = Some(read_csr_int_col_ctl);
    (*csr_ops).write_csr_int_col_ctl = Some(write_csr_int_col_ctl);
    (*csr_ops).read_csr_int_flag_and_col = Some(read_csr_int_flag_and_col);
    (*csr_ops).write_csr_int_flag_and_col = Some(write_csr_int_flag_and_col);
    (*csr_ops).read_csr_ring_srv_arb_en = Some(read_csr_ring_srv_arb_en);
    (*csr_ops).write_csr_ring_srv_arb_en = Some(write_csr_ring_srv_arb_en);
    (*csr_ops).get_int_col_ctl_enable_mask = Some(get_int_col_ctl_enable_mask);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
