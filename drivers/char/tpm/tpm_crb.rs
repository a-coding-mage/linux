// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of tpm_crb.c. Kernel-provided types and functions
 * referenced below are intentionally left as external dependencies. */

const ACPI_SIG_TPM2: &str = "TPM2";
const TPM_CRB_MAX_RESOURCES: usize = 3;

static CRB_ACPI_START_GUID: guid_t = GUID_INIT!(0x6BBF6CAB, 0x5463, 0x4714,
    0xB7, 0xCD, 0xF0, 0x20, 0x3C, 0x03, 0x68, 0xD4);

#[repr(C, packed)]
struct crb_regs_head { loc_state: u32, reserved1: u32, loc_ctrl: u32, loc_sts: u32,
    reserved2: [u8; 32], intf_id: u64, ctrl_ext: u64 }
#[repr(C, packed)]
struct crb_regs_tail { ctrl_req: u32, ctrl_sts: u32, ctrl_cancel: u32, ctrl_start: u32,
    ctrl_int_enable: u32, ctrl_int_sts: u32, ctrl_cmd_size: u32, ctrl_cmd_pa_low: u32,
    ctrl_cmd_pa_high: u32, ctrl_rsp_size: u32, ctrl_rsp_pa: u64 }

const CRB_ACPI_START_REVISION_ID: u32 = 1;
const CRB_ACPI_START_INDEX: u32 = 1;
const CRB_LOC_CTRL_REQUEST_ACCESS: u32 = 1 << 0;
const CRB_LOC_CTRL_RELINQUISH: u32 = 1 << 1;
const CRB_LOC_STATE_LOC_ASSIGNED: u32 = 1 << 1;
const CRB_LOC_STATE_TPM_REG_VALID_STS: u32 = 1 << 7;
const CRB_CTRL_REQ_CMD_READY: u32 = 1 << 0;
const CRB_CTRL_REQ_GO_IDLE: u32 = 1 << 1;
const CRB_CTRL_STS_ERROR: u32 = 1 << 0;
const CRB_START_INVOKE: u32 = 1;
const CRB_CANCEL_INVOKE: u32 = 1;
const CRB_DRV_STS_COMPLETE: u8 = 1;

#[repr(C)]
struct crb_priv {
    sm: u32, hid: *const c_char, regs_h: *mut crb_regs_head, regs_t: *mut crb_regs_tail,
    cmd: *mut u8, rsp: *mut u8, cmd_size: u32, smc_func_id: u32,
    pluton_start_addr: *mut u32, pluton_reply_addr: *mut u32, ffa_flags: u8, ffa_attributes: u8,
}
#[repr(C, packed)] struct tpm2_crb_smc { interrupt: u32, interrupt_flags: u8, op_flags: u8, reserved2: u16, smc_func_id: u32 }
#[repr(C, packed)] struct tpm2_crb_ffa { flags: u8, attributes: u8, partition_id: u16, reserved: [u8; 8] }
#[repr(C, packed)] struct tpm2_crb_pluton { start_addr: u64, reply_addr: u64 }

#[inline] unsafe fn tpm_crb_has_idle(start_method: u32) -> bool {
    start_method != ACPI_TPM2_START_METHOD && start_method != ACPI_TPM2_COMMAND_BUFFER_WITH_START_METHOD && start_method != ACPI_TPM2_COMMAND_BUFFER_WITH_ARM_SMC
}

unsafe fn crb_wait_for_reg_32(reg: *mut u32, mask: u32, value: u32, timeout: c_ulong) -> bool {
    let start = ktime_get(); let stop = ktime_add(start, ms_to_ktime(timeout));
    loop { if (ioread32(reg) & mask) == value { return true; } usleep_range(50, 100); if !ktime_before(ktime_get(), stop) { break; } }
    (ioread32(reg) & mask) == value
}

unsafe fn crb_try_pluton_doorbell(priv_: *mut crb_priv, wait: bool) -> c_int {
    if (*priv_).sm != ACPI_TPM2_COMMAND_BUFFER_WITH_PLUTON { return 0; }
    if !crb_wait_for_reg_32((*priv_).pluton_reply_addr, !0, 1, TPM2_TIMEOUT_C) { return -ETIME; }
    iowrite32(1, (*priv_).pluton_start_addr); if !wait { return 0; }
    if !crb_wait_for_reg_32((*priv_).pluton_start_addr, 0xffff_ffff, 0, 200) { return -ETIME; } 0
}

unsafe fn __crb_go_idle(dev: *mut device, p: *mut crb_priv, loc: c_int) -> c_int {
    if !tpm_crb_has_idle((*p).sm) { return 0; } iowrite32(CRB_CTRL_REQ_GO_IDLE, &mut (*(*p).regs_t).ctrl_req);
    if (*p).sm == ACPI_TPM2_CRB_WITH_ARM_FFA { let rc=tpm_crb_ffa_start(CRB_FFA_START_TYPE_COMMAND, loc); if rc != 0{return rc;} }
    let rc=crb_try_pluton_doorbell(p,true); if rc!=0{return rc;}
    if !crb_wait_for_reg_32(&mut (*(*p).regs_t).ctrl_req, CRB_CTRL_REQ_GO_IDLE, 0, TPM2_TIMEOUT_C) { dev_warn(dev, "goIdle timed out\n"); return -ETIME; } 0
}
unsafe fn __crb_cmd_ready(dev:*mut device,p:*mut crb_priv,loc:c_int)->c_int { if !tpm_crb_has_idle((*p).sm){return 0;} iowrite32(CRB_CTRL_REQ_CMD_READY,&mut (*(*p).regs_t).ctrl_req); if (*p).sm==ACPI_TPM2_CRB_WITH_ARM_FFA {let rc=tpm_crb_ffa_start(CRB_FFA_START_TYPE_COMMAND,loc);if rc!=0{return rc;}} let rc=crb_try_pluton_doorbell(p,true);if rc!=0{return rc;} if !crb_wait_for_reg_32(&mut (*(*p).regs_t).ctrl_req,CRB_CTRL_REQ_CMD_READY,0,TPM2_TIMEOUT_C){dev_warn(dev,"cmdReady timed out\n");return -ETIME;} 0 }
unsafe fn __crb_request_locality(dev:*mut device,p:*mut crb_priv,loc:c_int)->c_int { if p.is_null()||(*p).regs_h.is_null(){return 0;} iowrite32(CRB_LOC_CTRL_REQUEST_ACCESS,&mut (*(*p).regs_h).loc_ctrl); if (*p).sm==ACPI_TPM2_CRB_WITH_ARM_FFA {let rc=tpm_crb_ffa_start(CRB_FFA_START_TYPE_LOCALITY_REQUEST,loc);if rc!=0{return rc;}} let v=CRB_LOC_STATE_LOC_ASSIGNED|CRB_LOC_STATE_TPM_REG_VALID_STS;if !crb_wait_for_reg_32(&mut (*(*p).regs_h).loc_state,v,v,TPM2_TIMEOUT_C){dev_warn(dev,"TPM_LOC_STATE_x.requestAccess timed out\n");return -ETIME;} 0 }
unsafe fn __crb_relinquish_locality(dev:*mut device,p:*mut crb_priv,loc:c_int)->c_int { if (*p).regs_h.is_null(){return 0;} iowrite32(CRB_LOC_CTRL_RELINQUISH,&mut (*(*p).regs_h).loc_ctrl);if (*p).sm==ACPI_TPM2_CRB_WITH_ARM_FFA {let rc=tpm_crb_ffa_start(CRB_FFA_START_TYPE_LOCALITY_REQUEST,loc);if rc!=0{return rc;}} let m=CRB_LOC_STATE_LOC_ASSIGNED|CRB_LOC_STATE_TPM_REG_VALID_STS;if !crb_wait_for_reg_32(&mut (*(*p).regs_h).loc_state,m,CRB_LOC_STATE_TPM_REG_VALID_STS,TPM2_TIMEOUT_C){dev_warn(dev,"TPM_LOC_STATE_x.Relinquish timed out\n");return -ETIME;} 0 }

unsafe fn crb_status(chip:*mut tpm_chip)->u8 { let p=dev_get_drvdata((*chip).dev); if ioread32(&mut (*(*p).regs_t).ctrl_start)&CRB_START_INVOKE==0 {CRB_DRV_STS_COMPLETE}else{0} }
unsafe fn crb_recv(chip:*mut tpm_chip,buf:*mut u8,count:usize)->c_int { let p=dev_get_drvdata((*chip).dev); if count<TPM_HEADER_SIZE{return -EIO;} if ioread32(&mut (*(*p).regs_t).ctrl_sts)&CRB_CTRL_STS_ERROR!=0{return -EIO;} memcpy_fromio(buf,(*p).rsp,8); let expected=be32_to_cpup(buf.add(2) as *const be32); if expected>count||expected<TPM_HEADER_SIZE{return -EIO;} memcpy_fromio(buf.add(8),(*p).rsp.add(8),expected-8); expected as c_int }
unsafe fn crb_send(chip:*mut tpm_chip,buf:*mut u8,_bufsiz:usize,len:usize)->c_int { let p=dev_get_drvdata((*chip).dev); iowrite32(0,&mut (*(*p).regs_t).ctrl_cancel); if len>(*p).cmd_size{return -E2BIG;} if (*p).sm==ACPI_TPM2_COMMAND_BUFFER_WITH_PLUTON {__crb_cmd_ready((*chip).dev,p,(*chip).locality);} memcpy_toio((*p).cmd,buf,len); wmb(); if (*p).sm==ACPI_TPM2_COMMAND_BUFFER||(*p).sm==ACPI_TPM2_MEMORY_MAPPED||strcmp((*p).hid,b"MSFT0101\0".as_ptr() as *const c_char)==0{iowrite32(CRB_START_INVOKE,&mut (*(*p).regs_t).ctrl_start);} crb_try_pluton_doorbell(p,false) }
unsafe fn crb_req_canceled(chip:*mut tpm_chip,_status:u8)->bool { let p=dev_get_drvdata((*chip).dev); ioread32(&mut (*(*p).regs_t).ctrl_cancel)&CRB_CANCEL_INVOKE!=0 }

// C preprocessor conditionals, module metadata, and kernel registration are preserved as intent;
// the referenced kernel declarations are supplied by the surrounding translation unit.
#[allow(dead_code)] const _MODULE_DESCRIPTION: &str = "TPM2 Driver";

extern "C" {
    fn tpm_crb_ffa_start(kind: u32, loc: c_int) -> c_int;
    fn ktime_get() -> ktime_t; fn ktime_add(a: ktime_t,b: ktime_t)->ktime_t; fn ms_to_ktime(x:c_ulong)->ktime_t; fn ktime_before(a:ktime_t,b:ktime_t)->bool;
    fn ioread32(p:*mut u32)->u32; fn iowrite32(v:u32,p:*mut u32); fn usleep_range(a:u32,b:u32); fn dev_warn(d:*mut device,s:*const c_char,...);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
