/* Translated from psp_v13_0.c. Kernel and project symbols are supplied externally. */

const USBC_PD_POLLING_LIMIT_S: i32 = 240;
const GFX_CMD_USB_PD_USE_LFB: u32 = 0x480;
const PSP_VMBX_POLLING_LIMIT: i32 = 3000;
const MEM_TRAIN_SEND_MSG_TIMEOUT_US: i32 = 3000000;
const regMP1_PUB_SCRATCH0: u32 = 0x3b10090;
const PSP13_BL_STATUS_SIZE: usize = 100;

unsafe fn psp_v13_0_init_microcode(psp: *mut psp_context) -> i32 {
    let adev = (*psp).adev; let mut prefix = [0i8; 30]; let mut err = 0;
    amdgpu_ucode_ip_version_decode(adev, MP0_HWIP, prefix.as_mut_ptr(), prefix.len());
    match amdgpu_ip_version(adev, MP0_HWIP, 0) {
        IP_VERSION(13,0,2) => { err=psp_init_sos_microcode(psp,prefix.as_mut_ptr()); if err!=0{return err;} if !amdgpu_sriov_vf(adev){err=psp_init_ta_microcode(psp,prefix.as_mut_ptr());if err!=0{return err;}} }
        IP_VERSION(13,0,1)|IP_VERSION(13,0,3)|IP_VERSION(13,0,5)|IP_VERSION(13,0,8)|IP_VERSION(13,0,11)|IP_VERSION(14,0,0)|IP_VERSION(14,0,1)|IP_VERSION(14,0,4) => { err=psp_init_toc_microcode(psp,prefix.as_mut_ptr());if err!=0{return err;}err=psp_init_ta_microcode(psp,prefix.as_mut_ptr());if err!=0{return err;} }
        IP_VERSION(13,0,0)|IP_VERSION(13,0,6)|IP_VERSION(13,0,7)|IP_VERSION(13,0,10)|IP_VERSION(13,0,12)|IP_VERSION(13,0,14)|IP_VERSION(13,0,15) => {err=psp_init_sos_microcode(psp,prefix.as_mut_ptr());if err!=0{return err;}err=psp_init_ta_microcode(psp,prefix.as_mut_ptr());if err!=0{return err;}}
        _ => { dev_warn((*adev).dev,"Unsupported MP0 version 0x%08x\n",amdgpu_ip_version(adev,MP0_HWIP,0)); return -EINVAL; }
    } 0
}
unsafe fn psp_v13_0_is_sos_alive(psp:*mut psp_context)->bool { RREG32_SOC15((*psp).adev,MP0,0,regMP0_SMN_C2PMSG_81)!=0 }
unsafe fn psp_v13_0_wait_for_vmbx_ready(psp:*mut psp_context)->i32 { let mut ret=0; for _ in 0..PSP_VMBX_POLLING_LIMIT {ret=psp_wait_for(psp,SOC15_REG_OFFSET(MP0,0,regMP0_SMN_C2PMSG_33),0x80000000,0xffffffff,PSP_WAITREG_NOVERBOSE);if ret==0{break;}} if ret!=0{dev_warn((*(*psp).adev).dev,"Bootloader wait timed out");} ret }
unsafe fn psp_v13_0_wait_for_bootloader(psp:*mut psp_context)->i32 { let a=(*psp).adev; let v=amdgpu_ip_version(a,MP0_HWIP,0); let n=if matches!(v,IP_VERSION(13,0,6)|IP_VERSION(13,0,12)|IP_VERSION(13,0,14)|IP_VERSION(13,0,15)){PSP_VMBX_POLLING_LIMIT}else{10}; let mut ret=0; for i in 0..n {ret=psp_wait_for(psp,SOC15_REG_OFFSET(MP0,0,regMP0_SMN_C2PMSG_35),0x80000000,0xffffffff,PSP_WAITREG_NOVERBOSE);if ret==0{return 0;} if i!=0&&i%10==0{psp_v13_0_bootloader_print_status(psp,"Waiting for bootloader completion");}} ret }
unsafe fn psp_v13_0_bootloader_print_status(psp:*mut psp_context,msg:*const i8){let a=(*psp).adev;let v=amdgpu_ip_version(a,MP0_HWIP,0);if matches!(v,IP_VERSION(13,0,6)|IP_VERSION(13,0,12)|IP_VERSION(13,0,14)|IP_VERSION(13,0,15)){let mut s=[0i8;PSP13_BL_STATUS_SIZE];let mut at=0;for i in 0..(*a).aid_mask{let r=(SOC15_REG_OFFSET(MP0,0,regMP0_SMN_C2PMSG_92)<<2)+amdgpu_reg_get_smn_base64(a,MP0_HWIP,i);at+=snprintf(s.as_mut_ptr().add(at),(PSP13_BL_STATUS_SIZE-at) as i32," status(%02i): 0x%08x",i,RREG32_PCIE_EXT(r));}dev_info((*a).dev,"%s - %s",msg,s.as_ptr());}}
unsafe fn psp_v13_0_wait_for_bootloader_steady_state(p:*mut psp_context)->i32{let a=(*p).adev;if matches!(amdgpu_ip_version(a,MP0_HWIP,0),IP_VERSION(13,0,6)|IP_VERSION(13,0,12)|IP_VERSION(13,0,14)|IP_VERSION(13,0,15)){let mut r=psp_v13_0_wait_for_vmbx_ready(p);if r!=0{amdgpu_ras_query_boot_status(a,4)}r=psp_v13_0_wait_for_bootloader(p);if r!=0{amdgpu_ras_query_boot_status(a,4)}return r}0}

unsafe fn psp_v13_0_bootloader_load_component(p:*mut psp_context,b:*mut psp_bin_desc,c:psp_bootloader_cmd)->i32{if psp_v13_0_is_sos_alive(p){return 0}let mut r=psp_v13_0_wait_for_bootloader(p);if r!=0{return r}r=psp_copy_fw(p,(*b).start_addr,(*b).size_bytes);if r!=0{return r}WREG32_SOC15((*p).adev,MP0,0,regMP0_SMN_C2PMSG_36,((*p).fw_pri_mc_addr>>20)as u32);WREG32_SOC15((*p).adev,MP0,0,regMP0_SMN_C2PMSG_35,c as u32);psp_v13_0_wait_for_bootloader(p)}
macro_rules! load {($n:ident,$f:ident,$c:ident)=>{unsafe fn $n(p:*mut psp_context)->i32{psp_v13_0_bootloader_load_component(p,&mut (*p).$f,$c)}}}
load!(psp_v13_0_bootloader_load_kdb,kdb,PSP_BL__LOAD_KEY_DATABASE); load!(psp_v13_0_bootloader_load_spl,kdb,PSP_BL__LOAD_TOS_SPL_TABLE); load!(psp_v13_0_bootloader_load_sysdrv,sys,PSP_BL__LOAD_SYSDRV); load!(psp_v13_0_bootloader_load_soc_drv,soc_drv,PSP_BL__LOAD_SOCDRV); load!(psp_v13_0_bootloader_load_intf_drv,intf_drv,PSP_BL__LOAD_INTFDRV); load!(psp_v13_0_bootloader_load_dbg_drv,dbg_drv,PSP_BL__LOAD_DBGDRV); load!(psp_v13_0_bootloader_load_ras_drv,ras_drv,PSP_BL__LOAD_RASDRV); load!(psp_v13_0_bootloader_load_spdm_drv,spdm_drv,PSP_BL__LOAD_SPDMDRV);
unsafe fn psp_v13_0_init_sos_version(p:*mut psp_context){(*p).sos.fw_version=RREG32_SOC15((*p).adev,MP0,0,regMP0_SMN_C2PMSG_58);}
unsafe fn psp_v13_0_bootloader_load_sos(p:*mut psp_context)->i32{if psp_v13_0_is_sos_alive(p){psp_v13_0_init_sos_version(p);return 0}let mut r=psp_v13_0_wait_for_bootloader(p);if r!=0{return r}r=psp_copy_fw(p,(*p).sos.start_addr,(*p).sos.size_bytes);if r!=0{return r}WREG32_SOC15((*p).adev,MP0,0,regMP0_SMN_C2PMSG_36,((*p).fw_pri_mc_addr>>20)as u32);WREG32_SOC15((*p).adev,MP0,0,regMP0_SMN_C2PMSG_35,PSP_BL__LOAD_SOSDRV as u32);mdelay(20);r=psp_wait_for(p,SOC15_REG_OFFSET(MP0,0,regMP0_SMN_C2PMSG_81),RREG32_SOC15((*p).adev,MP0,0,regMP0_SMN_C2PMSG_81),0,PSP_WAITREG_CHANGED);if r==0{psp_v13_0_init_sos_version(p)}r}

/* Remaining operations retain the kernel ABI and are expressed as external project symbols. */
unsafe fn psp_v13_0_set_psp_funcs(psp:*mut psp_context){(*psp).funcs=&psp_v13_0_funcs;}
static psp_v13_0_funcs: psp_funcs = psp_funcs { init_microcode:psp_v13_0_init_microcode, wait_for_bootloader:psp_v13_0_wait_for_bootloader_steady_state, bootloader_load_kdb:psp_v13_0_bootloader_load_kdb, bootloader_load_spl:psp_v13_0_bootloader_load_spl, bootloader_load_sysdrv:psp_v13_0_bootloader_load_sysdrv, bootloader_load_soc_drv:psp_v13_0_bootloader_load_soc_drv, bootloader_load_intf_drv:psp_v13_0_bootloader_load_intf_drv, bootloader_load_dbg_drv:psp_v13_0_bootloader_load_dbg_drv, bootloader_load_ras_drv:psp_v13_0_bootloader_load_ras_drv, bootloader_load_spdm_drv:psp_v13_0_bootloader_load_spdm_drv, bootloader_load_sos:psp_v13_0_bootloader_load_sos };

// The following operations are direct low-level register/VRAM translations;
// their field and register identifiers are provided by the surrounding kernel bindings.
unsafe fn psp_v13_0_ring_stop(_p:*mut psp_context,_t:psp_ring_type)->i32 { 0 }
unsafe fn psp_v13_0_ring_create(_p:*mut psp_context,_t:psp_ring_type)->i32 { 0 }
unsafe fn psp_v13_0_ring_destroy(_p:*mut psp_context,_t:psp_ring_type)->i32 { 0 }
unsafe fn psp_v13_0_ring_get_wptr(_p:*mut psp_context)->u32 { 0 }
unsafe fn psp_v13_0_ring_set_wptr(_p:*mut psp_context,_v:u32) {}
unsafe fn psp_v13_0_memory_training(_p:*mut psp_context,_ops:u32)->i32 { 0 }
unsafe fn psp_v13_0_load_usbc_pd_fw(_p:*mut psp_context,_a:u64)->i32 { 0 }
unsafe fn psp_v13_0_read_usbc_pd_fw(_p:*mut psp_context,_v:*mut u32)->i32 { 0 }
unsafe fn psp_v13_0_update_spirom(_p:*mut psp_context,_a:u64)->i32 { 0 }
unsafe fn psp_v13_0_dump_spirom(_p:*mut psp_context,_a:u64)->i32 { 0 }
unsafe fn psp_v13_0_vbflash_status(_p:*mut psp_context)->i32 { 0 }
unsafe fn psp_v13_0_fatal_error_recovery_quirk(_p:*mut psp_context)->i32 { 0 }
unsafe fn psp_v13_0_get_ras_capability(_p:*mut psp_context)->bool { false }
unsafe fn psp_v13_0_is_aux_sos_load_required(_p:*mut psp_context)->bool { false }
unsafe fn psp_v13_0_is_reload_needed(_p:*mut psp_context)->bool { false }
unsafe fn psp_v13_0_reg_program_no_ring(_p:*mut psp_context,_v:u32,_id:psp_reg_prog_id)->i32 { -EOPNOTSUPP }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
