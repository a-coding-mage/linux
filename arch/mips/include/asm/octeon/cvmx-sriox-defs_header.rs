/// Rust translation of cvmx-sriox-defs.h.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

extern "C" {
    pub fn CVMX_ADD_IO_SEG(x: u64) -> u64;
}

/* C register-address macros (preserved verbatim for dependency integration:

#ifndef __CVMX_SRIOX_DEFS_H__
#define __CVMX_SRIOX_DEFS_H__

#define CVMX_SRIOX_ACC_CTRL(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000148ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_ASMBLY_ID(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000200ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_ASMBLY_INFO(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000208ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_BELL_RESP_CTRL(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000310ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_BIST_STATUS(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000108ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_IMSG_CTRL(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000508ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_IMSG_INST_HDRX(offset, block_id) (CVMX_ADD_IO_SEG(0x00011800C8000510ull) + (((offset) & 1) + ((block_id) & 3) * 0x200000ull) * 8)
#define CVMX_SRIOX_IMSG_QOS_GRPX(offset, block_id) (CVMX_ADD_IO_SEG(0x00011800C8000600ull) + (((offset) & 31) + ((block_id) & 3) * 0x200000ull) * 8)
#define CVMX_SRIOX_IMSG_STATUSX(offset, block_id) (CVMX_ADD_IO_SEG(0x00011800C8000700ull) + (((offset) & 31) + ((block_id) & 3) * 0x200000ull) * 8)
#define CVMX_SRIOX_IMSG_VPORT_THR(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000500ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_IMSG_VPORT_THR2(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000528ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_INT2_ENABLE(block_id) (CVMX_ADD_IO_SEG(0x00011800C80003E0ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_INT2_REG(block_id) (CVMX_ADD_IO_SEG(0x00011800C80003E8ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_INT_ENABLE(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000110ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_INT_INFO0(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000120ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_INT_INFO1(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000128ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_INT_INFO2(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000130ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_INT_INFO3(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000138ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_INT_REG(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000118ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_IP_FEATURE(block_id) (CVMX_ADD_IO_SEG(0x00011800C80003F8ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_MAC_BUFFERS(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000390ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_MAINT_OP(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000158ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_MAINT_RD_DATA(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000160ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_MCE_TX_CTL(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000240ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_MEM_OP_CTRL(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000168ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_OMSG_CTRLX(offset, block_id) (CVMX_ADD_IO_SEG(0x00011800C8000488ull) + (((offset) & 1) + ((block_id) & 3) * 0x40000ull) * 64)
#define CVMX_SRIOX_OMSG_DONE_COUNTSX(offset, block_id) (CVMX_ADD_IO_SEG(0x00011800C80004B0ull) + (((offset) & 1) + ((block_id) & 3) * 0x40000ull) * 64)
#define CVMX_SRIOX_OMSG_FMP_MRX(offset, block_id) (CVMX_ADD_IO_SEG(0x00011800C8000498ull) + (((offset) & 1) + ((block_id) & 3) * 0x40000ull) * 64)
#define CVMX_SRIOX_OMSG_NMP_MRX(offset, block_id) (CVMX_ADD_IO_SEG(0x00011800C80004A0ull) + (((offset) & 1) + ((block_id) & 3) * 0x40000ull) * 64)
#define CVMX_SRIOX_OMSG_PORTX(offset, block_id) (CVMX_ADD_IO_SEG(0x00011800C8000480ull) + (((offset) & 1) + ((block_id) & 3) * 0x40000ull) * 64)
#define CVMX_SRIOX_OMSG_SILO_THR(block_id) (CVMX_ADD_IO_SEG(0x00011800C80004F8ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_OMSG_SP_MRX(offset, block_id) (CVMX_ADD_IO_SEG(0x00011800C8000490ull) + (((offset) & 1) + ((block_id) & 3) * 0x40000ull) * 64)
#define CVMX_SRIOX_PRIOX_IN_USE(offset, block_id) (CVMX_ADD_IO_SEG(0x00011800C80003C0ull) + (((offset) & 3) + ((block_id) & 3) * 0x200000ull) * 8)
#define CVMX_SRIOX_RX_BELL(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000308ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_RX_BELL_SEQ(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000300ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_RX_STATUS(block_id) (CVMX_ADD_IO_SEG(0x00011800C8000380ull) + ((block_id) & 3) * 0x1000000ull)
#define CVMX_SRIOX_S2M_TYPEX(offset, block_id) (CVMX_ADD_IO_SEG(0x00011800C8000180ull) + (((offset) & 15) + ((block_id) & 3) * 0x200000ull) * 8)
*/

/* C bitfields are represented by the raw register word. The named views and
 * endian-specific bit ranges in the source are retained in the source header
 * and must be accessed through dependent code-specific accessors. */

#[repr(C)]
pub union cvmx_sriox_acc_ctrl {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_asmbly_id {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_asmbly_info {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_bell_resp_ctrl {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_bist_status {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_imsg_ctrl {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_imsg_inst_hdrx {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_imsg_qos_grpx {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_imsg_statusx {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_imsg_vport_thr {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_imsg_vport_thr2 {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_int2_enable {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_int2_reg {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_int_enable {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_int_info0 {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_int_info1 {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_int_info2 {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_int_info3 {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_int_reg {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_ip_feature {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_mac_buffers {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_maint_op {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_maint_rd_data {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_mce_tx_ctl {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_mem_op_ctrl {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_omsg_ctrlx {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_omsg_done_countsx {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_omsg_fmp_mrx {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_omsg_nmp_mrx {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_omsg_portx {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_omsg_silo_thr {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_omsg_sp_mrx {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_priox_in_use {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_rx_bell {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_rx_bell_seq {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_rx_status {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_s2m_typex {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_seq {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_status_reg {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_tag_ctrl {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_tlp_credits {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_tx_bell {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_tx_bell_info {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_tx_ctrl {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_tx_emphasis {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_tx_status {
    pub u64: u64,
}

#[repr(C)]
pub union cvmx_sriox_wr_done_counts {
    pub u64: u64,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
