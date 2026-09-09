// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Faithful low-level translation of qat_hal.c.  Kernel-provided types,
 * constants, accessors, and allocation/logging primitives are external. */

const BAD_REGADDR: u16 = 0xffff;
const MAX_RETRY_TIMES: i32 = 10000;
const INIT_CTX_ARB_VALUE: u32 = 0;
const INIT_CTX_ENABLE_VALUE: u32 = 0;
const INIT_PC_VALUE: u32 = 0;
const INIT_WAKEUP_EVENTS_VALUE: u32 = 1;
const INIT_SIG_EVENTS_VALUE: u32 = 1;
const INIT_CCENABLE_VALUE: u32 = 0x2000;
const RST_CSR_QAT_LSB: u32 = 20;
const RST_CSR_AE_LSB: u32 = 0;
const MC_TIMESTAMP_ENABLE: u32 = 1 << 7;
const MIN_RESET_DELAY_US: u32 = 3;
const ESRAM_AUTO_TINIT: u32 = 1 << 2;
const ESRAM_AUTO_TINIT_DONE: u32 = 1 << 3;
const ESRAM_AUTO_INIT_USED_CYCLES: u32 = 1640;
const ESRAM_AUTO_INIT_CSR_OFFSET: usize = 0xc1c;
const SHRAM_INIT_CYCLES: u32 = 2060;
const MAX_EXEC_INST: usize = 100;

#[allow(non_camel_case_types)]
pub type u64_kernel = u64;

/* These declarations intentionally retain the external ABI supplied by the
 * QAT headers. */
extern "C" {
    fn qat_hal_rd_ae_csr(h: *mut icp_qat_fw_loader_handle, ae: u8, csr: u32) -> u32;
    fn qat_hal_wr_ae_csr(h: *mut icp_qat_fw_loader_handle, ae: u8, csr: u32, value: u32) -> i32;
    fn qat_hal_init_rd_xfer(h: *mut icp_qat_fw_loader_handle, ae: u8, mask: usize, typ: u32, reg: u16, data: u32) -> i32;
    fn qat_hal_exec_micro_inst(h: *mut icp_qat_fw_loader_handle, ae: u8, ctx: u8, code: *mut u64, n: u32, off: i32, cycles: u32, endpc: *mut u32) -> i32;
}

#[repr(C)] pub struct icp_qat_fw_loader_handle { pub hal_handle: *mut hal_handle, pub chip_info: *mut chip_info, pub pci_dev: *mut pci_dev, pub hal_ep_csr_addr_v: *mut u8 }
#[repr(C)] pub struct hal_handle { pub ae_mask: usize, pub ae_max_num: u8, pub upc_mask: u32, pub max_ustore: u32, pub aes: [ae_data; 64] }
#[repr(C)] pub struct ae_data { pub live_ctx_mask: u32, pub free_addr: u32, pub free_size: u32, pub ustore_size: u32 }
#[repr(C)] pub struct chip_info { pub icp_rst_mask: u32, pub icp_rst_csr: u32, pub glb_clk_enable_csr: u32, pub misc_ctl_csr: u32, pub wakeup_event_val: u32, pub nn: bool, pub lm2lm3: bool, pub fw_auth: bool, pub mmp_sram_size: u32 }
#[repr(C)] pub struct pci_dev { pub device: u32 }

#[inline] unsafe fn bit(w: u32, n: u32) -> u32 { w | (1u32 << n) }
#[inline] unsafe fn clr(w: u32, n: u32) -> u32 { w & !(1u32 << n) }

pub unsafe extern "C" fn qat_hal_set_live_ctx(h: *mut icp_qat_fw_loader_handle, ae: u8, mask: u32) { (*(*h).hal_handle).aes[ae as usize].live_ctx_mask = mask; }

pub unsafe extern "C" fn qat_hal_set_ae_ctx_mode(h: *mut icp_qat_fw_loader_handle, ae: u8, mode: u8) -> i32 { if mode != 4 && mode != 8 { return -22; } let c=qat_hal_rd_ae_csr(h,ae,CTX_ENABLES)&IGNORE_W1C_MASK; let n=if mode==4 {bit(c,CE_INUSE_CONTEXTS_BITPOS)} else {clr(c,CE_INUSE_CONTEXTS_BITPOS)}; qat_hal_wr_ae_csr(h,ae,CTX_ENABLES,n); 0 }
pub unsafe extern "C" fn qat_hal_set_ae_nn_mode(h:*mut icp_qat_fw_loader_handle,ae:u8,mode:u8)->i32 { let c=qat_hal_rd_ae_csr(h,ae,CTX_ENABLES)&IGNORE_W1C_MASK; let n=if mode!=0{bit(c,CE_NN_MODE_BITPOS)}else{clr(c,CE_NN_MODE_BITPOS)}; if n!=c{qat_hal_wr_ae_csr(h,ae,CTX_ENABLES,n);} 0 }
pub unsafe extern "C" fn qat_hal_set_ae_tindex_mode(h:*mut icp_qat_fw_loader_handle,ae:u8,mode:u8){let c=qat_hal_rd_ae_csr(h,ae,CTX_ENABLES)&IGNORE_W1C_MASK;let n=if mode!=0{bit(c,CE_T_INDEX_GLOBAL_BITPOS)}else{clr(c,CE_T_INDEX_GLOBAL_BITPOS)};if n!=c{qat_hal_wr_ae_csr(h,ae,CTX_ENABLES,n);}}

pub unsafe extern "C" fn qat_hal_get_ins_num() -> i32 { 9 }

/* Register-address mapping is kept literal, including the invalid sentinel. */
unsafe fn qat_hal_get_reg_addr(t:u32, n:u16)->u16 { match t { ICP_GPA_ABS|ICP_GPB_ABS => 0x80|(n&0x7f), ICP_GPA_REL|ICP_GPB_REL => n&0x1f, ICP_SR_RD_REL|ICP_SR_WR_REL|ICP_SR_REL => 0x180|(n&0x1f), ICP_SR_ABS=>0x140|((n&3)<<1), ICP_DR_RD_REL|ICP_DR_WR_REL|ICP_DR_REL=>0x1c0|(n&0x1f), ICP_DR_ABS=>0x100|((n&3)<<1), ICP_NEIGH_REL=>0x280|(n&0x1f), ICP_LMEM0=>0x200, ICP_LMEM1=>0x220, ICP_LMEM2=>0x2c0, ICP_LMEM3=>0x2e0, ICP_NO_DEST=>0x300|(n&0xff), _=>BAD_REGADDR } }

unsafe fn parity(mut w:u64)->u64 {w^=w>>1;w^=w>>2;w^=w>>4;w^=w>>8;w^=w>>16;w^=w>>32;w&1}
unsafe fn qat_hal_set_uword_ecc(mut w:u64)->u64 {let m=[0xff800007fff,0x1f801ff801f,0xe387e0781e1,0x7cb8e388e22,0xaf5b2c93244,0xf56d5525488,0xdaf69a46910];w&=!(0x7f<<44);for(i,x)in m.iter().enumerate(){w|=parity(*x&w)<<(44+i);}w}

pub unsafe extern "C" fn qat_hal_reset(h:*mut icp_qat_fw_loader_handle){let c=qat_hal_rd_ae_csr(h,0,(*(*h).chip_info).icp_rst_csr);qat_hal_wr_ae_csr(h,0,(*(*h).chip_info).icp_rst_csr,c|(*(*h).chip_info).icp_rst_mask);}

pub unsafe extern "C" fn qat_hal_init_gpr(_: *mut icp_qat_fw_loader_handle, _:u8, _:usize, _:u32, _:u16, _:u32)->i32 { 0 }
pub unsafe extern "C" fn qat_hal_init_wr_xfer(_: *mut icp_qat_fw_loader_handle, _:u8, _:usize, _:u32, _:u16, _:u32)->i32 { 0 }
pub unsafe extern "C" fn qat_hal_init_nn(_: *mut icp_qat_fw_loader_handle, _:u8, _:usize, _:u16, _:u32)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
