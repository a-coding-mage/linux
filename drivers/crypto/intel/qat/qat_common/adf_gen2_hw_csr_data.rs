// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2024 Intel Corporation */

// The following CSR operations are supplied by the corresponding hardware
// header/dependency.  They are kept as calls here to preserve the C macros'
// source-level behavior.

type DmaAddrT = u64;
type IomemPtr = *mut core::ffi::c_void;

#[repr(C)]
pub struct adf_hw_csr_ops {
    pub build_csr_ring_base_addr: Option<unsafe fn(DmaAddrT, u32) -> u64>,
    pub read_csr_ring_head: Option<unsafe fn(IomemPtr, u32, u32) -> u32>,
    pub write_csr_ring_head: Option<unsafe fn(IomemPtr, u32, u32, u32)>,
    pub read_csr_ring_tail: Option<unsafe fn(IomemPtr, u32, u32) -> u32>,
    pub write_csr_ring_tail: Option<unsafe fn(IomemPtr, u32, u32, u32)>,
    pub read_csr_e_stat: Option<unsafe fn(IomemPtr, u32) -> u32>,
    pub write_csr_ring_config: Option<unsafe fn(IomemPtr, u32, u32, u32)>,
    pub write_csr_ring_base: Option<unsafe fn(IomemPtr, u32, u32, DmaAddrT)>,
    pub write_csr_int_flag: Option<unsafe fn(IomemPtr, u32, u32)>,
    pub write_csr_int_srcsel: Option<unsafe fn(IomemPtr, u32)>,
    pub write_csr_int_col_en: Option<unsafe fn(IomemPtr, u32, u32)>,
    pub write_csr_int_col_ctl: Option<unsafe fn(IomemPtr, u32, u32)>,
    pub write_csr_int_flag_and_col: Option<unsafe fn(IomemPtr, u32, u32)>,
    pub write_csr_ring_srv_arb_en: Option<unsafe fn(IomemPtr, u32, u32)>,
}

unsafe fn build_csr_ring_base_addr(addr: DmaAddrT, size: u32) -> u64 {
    BUILD_RING_BASE_ADDR(addr, size)
}

unsafe fn read_csr_ring_head(csr_base_addr: IomemPtr, bank: u32, ring: u32) -> u32 {
    READ_CSR_RING_HEAD(csr_base_addr, bank, ring)
}

unsafe fn write_csr_ring_head(csr_base_addr: IomemPtr, bank: u32, ring: u32, value: u32) {
    WRITE_CSR_RING_HEAD(csr_base_addr, bank, ring, value);
}

unsafe fn read_csr_ring_tail(csr_base_addr: IomemPtr, bank: u32, ring: u32) -> u32 {
    READ_CSR_RING_TAIL(csr_base_addr, bank, ring)
}

unsafe fn write_csr_ring_tail(csr_base_addr: IomemPtr, bank: u32, ring: u32, value: u32) {
    WRITE_CSR_RING_TAIL(csr_base_addr, bank, ring, value);
}

unsafe fn read_csr_e_stat(csr_base_addr: IomemPtr, bank: u32) -> u32 {
    READ_CSR_E_STAT(csr_base_addr, bank)
}

unsafe fn write_csr_ring_config(csr_base_addr: IomemPtr, bank: u32, ring: u32, value: u32) {
    WRITE_CSR_RING_CONFIG(csr_base_addr, bank, ring, value);
}

unsafe fn write_csr_ring_base(csr_base_addr: IomemPtr, bank: u32, ring: u32, addr: DmaAddrT) {
    WRITE_CSR_RING_BASE(csr_base_addr, bank, ring, addr);
}

unsafe fn write_csr_int_flag(csr_base_addr: IomemPtr, bank: u32, value: u32) {
    WRITE_CSR_INT_FLAG(csr_base_addr, bank, value);
}

unsafe fn write_csr_int_srcsel(csr_base_addr: IomemPtr, bank: u32) {
    WRITE_CSR_INT_SRCSEL(csr_base_addr, bank);
}

unsafe fn write_csr_int_col_en(csr_base_addr: IomemPtr, bank: u32, value: u32) {
    WRITE_CSR_INT_COL_EN(csr_base_addr, bank, value);
}

unsafe fn write_csr_int_col_ctl(csr_base_addr: IomemPtr, bank: u32, value: u32) {
    WRITE_CSR_INT_COL_CTL(csr_base_addr, bank, value);
}

unsafe fn write_csr_int_flag_and_col(csr_base_addr: IomemPtr, bank: u32, value: u32) {
    WRITE_CSR_INT_FLAG_AND_COL(csr_base_addr, bank, value);
}

unsafe fn write_csr_ring_srv_arb_en(csr_base_addr: IomemPtr, bank: u32, value: u32) {
    WRITE_CSR_RING_SRV_ARB_EN(csr_base_addr, bank, value);
}

pub unsafe fn adf_gen2_init_hw_csr_ops(csr_ops: *mut adf_hw_csr_ops) {
    (*csr_ops).build_csr_ring_base_addr = Some(build_csr_ring_base_addr);
    (*csr_ops).read_csr_ring_head = Some(read_csr_ring_head);
    (*csr_ops).write_csr_ring_head = Some(write_csr_ring_head);
    (*csr_ops).read_csr_ring_tail = Some(read_csr_ring_tail);
    (*csr_ops).write_csr_ring_tail = Some(write_csr_ring_tail);
    (*csr_ops).read_csr_e_stat = Some(read_csr_e_stat);
    (*csr_ops).write_csr_ring_config = Some(write_csr_ring_config);
    (*csr_ops).write_csr_ring_base = Some(write_csr_ring_base);
    (*csr_ops).write_csr_int_flag = Some(write_csr_int_flag);
    (*csr_ops).write_csr_int_srcsel = Some(write_csr_int_srcsel);
    (*csr_ops).write_csr_int_col_en = Some(write_csr_int_col_en);
    (*csr_ops).write_csr_int_col_ctl = Some(write_csr_int_col_ctl);
    (*csr_ops).write_csr_int_flag_and_col = Some(write_csr_int_flag_and_col);
    (*csr_ops).write_csr_ring_srv_arb_en = Some(write_csr_ring_srv_arb_en);
}

// EXPORT_SYMBOL_GPL(adf_gen2_init_hw_csr_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
