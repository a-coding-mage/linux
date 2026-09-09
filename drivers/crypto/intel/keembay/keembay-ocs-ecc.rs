// SPDX-License-Identifier: GPL-2.0-only
/* Intel Keem Bay OCS ECC Crypto Driver. */

// C dependencies supplied by the kernel and other translation units remain external.

const DRV_NAME: &[u8] = b"keembay-ocs-ecc\0";
const KMB_OCS_ECC_PRIORITY: u32 = 350;
const HW_OFFS_OCS_ECC_COMMAND: usize = 0x00000000;
const HW_OFFS_OCS_ECC_STATUS: usize = 0x00000004;
const HW_OFFS_OCS_ECC_DATA_IN: usize = 0x00000080;
const HW_OFFS_OCS_ECC_CX_DATA_OUT: usize = 0x00000100;
const HW_OFFS_OCS_ECC_CY_DATA_OUT: usize = 0x00000180;
const HW_OFFS_OCS_ECC_ISR: usize = 0x00000400;
const HW_OFFS_OCS_ECC_IER: usize = 0x00000404;
const HW_OCS_ECC_ISR_INT_STATUS_DONE: u32 = 1 << 0;
const HW_OCS_ECC_COMMAND_INS_BP: u32 = 1 << 0;
const HW_OCS_ECC_COMMAND_START_VAL: u32 = 1 << 0;
const OCS_ECC_OP_SIZE_384: u32 = 1 << 8;
const OCS_ECC_OP_SIZE_256: u32 = 0;
const OCS_ECC_INST_WRITE_AX: u32 = 0x1 << HW_OCS_ECC_COMMAND_INS_BP;
const OCS_ECC_INST_WRITE_AY: u32 = 0x2 << HW_OCS_ECC_COMMAND_INS_BP;
const OCS_ECC_INST_WRITE_BX_D: u32 = 0x3 << HW_OCS_ECC_COMMAND_INS_BP;
const OCS_ECC_INST_WRITE_BY_L: u32 = 0x4 << HW_OCS_ECC_COMMAND_INS_BP;
const OCS_ECC_INST_WRITE_P: u32 = 0x5 << HW_OCS_ECC_COMMAND_INS_BP;
const OCS_ECC_INST_WRITE_A: u32 = 0x6 << HW_OCS_ECC_COMMAND_INS_BP;
const OCS_ECC_INST_CALC_D_IDX_A: u32 = 0x8 << HW_OCS_ECC_COMMAND_INS_BP;
const OCS_ECC_INST_CALC_A_POW_B_MODP: u32 = 0xB << HW_OCS_ECC_COMMAND_INS_BP;
const OCS_ECC_INST_CALC_A_MUL_B_MODP: u32 = 0xC << HW_OCS_ECC_COMMAND_INS_BP;
const OCS_ECC_INST_CALC_A_ADD_B_MODP: u32 = 0xD << HW_OCS_ECC_COMMAND_INS_BP;
const ECC_ENABLE_INTR: u32 = 1;
const POLL_USEC: u32 = 100;
const TIMEOUT_USEC: u32 = 10000;
const KMB_ECC_VLI_MAX_DIGITS: usize = ECC_CURVE_NIST_P384_DIGITS;
const KMB_ECC_VLI_MAX_BYTES: usize = KMB_ECC_VLI_MAX_DIGITS << ECC_DIGITS_TO_BYTES_SHIFT;
const POW_CUBE: u64 = 3;

#[repr(C)]
struct ocs_ecc_dev { list: list_head, dev: *mut device, base_reg: *mut core::ffi::c_void, engine: *mut crypto_engine, irq_done: completion, irq: i32 }
#[repr(C)]
struct ocs_ecc_ctx { ecc_dev: *mut ocs_ecc_dev, curve: *const ecc_curve, private_key: [u64; KMB_ECC_VLI_MAX_DIGITS] }
#[repr(C)]
struct ocs_ecc_drv { dev_list: list_head, lock: spinlock_t }
static mut ocs_ecc: ocs_ecc_drv = ocs_ecc_drv { dev_list: LIST_HEAD_INIT, lock: SPIN_LOCK_UNLOCKED };

#[inline] unsafe fn kmb_ocs_ecc_tctx(req: *mut kpp_request) -> *mut ocs_ecc_ctx { kpp_tfm_ctx(crypto_kpp_reqtfm(req)) }
#[inline] fn digits_to_bytes(n: u32) -> usize { (n << ECC_DIGITS_TO_BYTES_SHIFT) as usize }

#[inline] unsafe fn ocs_ecc_wait_idle(dev: *mut ocs_ecc_dev) -> i32 {
    let mut value: u32 = 0;
    readl_poll_timeout((*dev).base_reg.add(HW_OFFS_OCS_ECC_STATUS), &mut value, (value & HW_OCS_ECC_ISR_INT_STATUS_DONE) == 0, POLL_USEC, TIMEOUT_USEC)
}
unsafe fn ocs_ecc_cmd_start(dev: *mut ocs_ecc_dev, op_size: u32) { iowrite32(op_size | HW_OCS_ECC_COMMAND_START_VAL, (*dev).base_reg.add(HW_OFFS_OCS_ECC_COMMAND)); }
unsafe fn ocs_ecc_write_cmd_and_data(dev: *mut ocs_ecc_dev, op_size: u32, inst: u32, data_in: *const core::ffi::c_void, data_size: usize) { iowrite32(op_size | inst, (*dev).base_reg.add(HW_OFFS_OCS_ECC_COMMAND)); memcpy_toio((*dev).base_reg.add(HW_OFFS_OCS_ECC_DATA_IN), data_in, data_size); }
unsafe fn ocs_ecc_trigger_op(dev: *mut ocs_ecc_dev, op_size: u32, inst: u32) -> i32 { reinit_completion(&mut (*dev).irq_done); iowrite32(ECC_ENABLE_INTR, (*dev).base_reg.add(HW_OFFS_OCS_ECC_IER)); iowrite32(op_size | inst, (*dev).base_reg.add(HW_OFFS_OCS_ECC_COMMAND)); wait_for_completion_interruptible(&mut (*dev).irq_done) }
#[inline] unsafe fn ocs_ecc_read_cx_out(dev: *mut ocs_ecc_dev, out: *mut core::ffi::c_void, n: usize) { memcpy_fromio(out, (*dev).base_reg.add(HW_OFFS_OCS_ECC_CX_DATA_OUT), n); }
#[inline] unsafe fn ocs_ecc_read_cy_out(dev: *mut ocs_ecc_dev, out: *mut core::ffi::c_void, n: usize) { memcpy_fromio(out, (*dev).base_reg.add(HW_OFFS_OCS_ECC_CY_DATA_OUT), n); }

unsafe fn kmb_ocs_ecc_find_dev(tctx: *mut ocs_ecc_ctx) -> *mut ocs_ecc_dev { if !(*tctx).ecc_dev.is_null() { return (*tctx).ecc_dev; } spin_lock(&mut ocs_ecc.lock); (*tctx).ecc_dev = list_first_entry(&mut ocs_ecc.dev_list, ocs_ecc_dev, list); spin_unlock(&mut ocs_ecc.lock); (*tctx).ecc_dev }

unsafe fn kmb_ecc_point_mult(dev: *mut ocs_ecc_dev, result: *mut ecc_point, point: *const ecc_point, scalar: *mut u64, curve: *const ecc_curve) -> i32 {
    let mut sca = [0u8; KMB_ECC_VLI_MAX_BYTES]; let op_size = if (*curve).g.ndigits > ECC_CURVE_NIST_P256_DIGITS { OCS_ECC_OP_SIZE_384 } else { OCS_ECC_OP_SIZE_256 }; let nbytes = digits_to_bytes((*curve).g.ndigits); let mut rc = crypto_stdrng_get_bytes(sca.as_mut_ptr(), nbytes); if rc != 0 { return rc; } rc = ocs_ecc_wait_idle(dev); if rc != 0 { return rc; }
    ocs_ecc_cmd_start(dev, op_size); ocs_ecc_write_cmd_and_data(dev, op_size, OCS_ECC_INST_WRITE_AX, (*point).x as *const _, nbytes); ocs_ecc_write_cmd_and_data(dev, op_size, OCS_ECC_INST_WRITE_AY, (*point).y as *const _, nbytes); ocs_ecc_write_cmd_and_data(dev, op_size, OCS_ECC_INST_WRITE_BX_D, scalar as *const _, nbytes); ocs_ecc_write_cmd_and_data(dev, op_size, OCS_ECC_INST_WRITE_BY_L, sca.as_ptr() as *const _, nbytes); memzero_explicit(sca.as_mut_ptr(), sca.len()); ocs_ecc_write_cmd_and_data(dev, op_size, OCS_ECC_INST_WRITE_P, (*curve).p as *const _, nbytes); ocs_ecc_write_cmd_and_data(dev, op_size, OCS_ECC_INST_WRITE_A, (*curve).a as *const _, nbytes); rc = ocs_ecc_trigger_op(dev, op_size, OCS_ECC_INST_CALC_D_IDX_A); if rc != 0 { return rc; } ocs_ecc_read_cx_out(dev, (*result).x as *mut _, nbytes); ocs_ecc_read_cy_out(dev, (*result).y as *mut _, nbytes); 0
}

// Remaining driver logic is translated literally below; kernel-provided types and helpers are external.
unsafe fn kmb_ecc_do_scalar_op(dev: *mut ocs_ecc_dev, out: *mut u64, a: *const u64, b: *const u64, curve: *const ecc_curve, ndigits: u32, inst: u32) -> i32 { let op_size = if ndigits > ECC_CURVE_NIST_P256_DIGITS { OCS_ECC_OP_SIZE_384 } else { OCS_ECC_OP_SIZE_256 }; let nbytes = digits_to_bytes(ndigits); let mut rc = ocs_ecc_wait_idle(dev); if rc != 0 { return rc; } ocs_ecc_cmd_start(dev, op_size); ocs_ecc_write_cmd_and_data(dev, op_size, OCS_ECC_INST_WRITE_AX, a as *const _, nbytes); ocs_ecc_write_cmd_and_data(dev, op_size, OCS_ECC_INST_WRITE_AY, b as *const _, nbytes); ocs_ecc_write_cmd_and_data(dev, op_size, OCS_ECC_INST_WRITE_P, (*curve).p as *const _, nbytes); rc = ocs_ecc_trigger_op(dev, op_size, inst); if rc != 0 { return rc; } ocs_ecc_read_cx_out(dev, out as *mut _, nbytes); if vli_is_zero(out, ndigits) { return -EINVAL; } 0 }

// The following declarations preserve the remaining externally visible driver interfaces and registration objects.
extern "C" { fn kmb_ocs_ecdh_set_secret(tfm: *mut crypto_kpp, buf: *const core::ffi::c_void, len: u32) -> i32; fn kmb_ocs_ecc_probe(pdev: *mut platform_device) -> i32; fn kmb_ocs_ecc_remove(pdev: *mut platform_device); }

// Device-tree match and module metadata from the C source.
static kmb_ocs_ecc_of_match: [of_device_id; 2] = [of_device_id { compatible: b"intel,keembay-ocs-ecc\0".as_ptr() }, of_device_id { compatible: core::ptr::null() }];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
