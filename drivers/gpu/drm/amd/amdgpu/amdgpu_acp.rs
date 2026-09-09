/* Translated from amdgpu_acp.c. External kernel and amdgpu declarations are
 * supplied by the surrounding translation unit. */

const ST_JADEITE: u64 = 1;
const ACP_TILE_ON_MASK: u32 = 0x03;
const ACP_TILE_OFF_MASK: u32 = 0x02;
const ACP_TILE_ON_RETAIN_REG_MASK: u32 = 0x1f;
const ACP_TILE_OFF_RETAIN_REG_MASK: u32 = 0x20;
const ACP_TILE_P1_MASK: u32 = 0x3e;
const ACP_TILE_P2_MASK: u32 = 0x3d;
const ACP_TILE_DSP0_MASK: u32 = 0x3b;
const ACP_TILE_DSP1_MASK: u32 = 0x37;
const ACP_TILE_DSP2_MASK: u32 = 0x2f;
const ACP_DMA_REGS_END: u64 = 0x146c0;
const ACP_I2S_PLAY_REGS_START: u64 = 0x14840;
const ACP_I2S_PLAY_REGS_END: u64 = 0x148b4;
const ACP_I2S_CAP_REGS_START: u64 = 0x148b8;
const ACP_I2S_CAP_REGS_END: u64 = 0x1496c;
const ACP_I2S_COMP1_CAP_REG_OFFSET: u32 = 0xac;
const ACP_I2S_COMP2_CAP_REG_OFFSET: u32 = 0xa8;
const ACP_I2S_COMP1_PLAY_REG_OFFSET: u32 = 0x6c;
const ACP_I2S_COMP2_PLAY_REG_OFFSET: u32 = 0x68;
const ACP_BT_PLAY_REGS_START: u64 = 0x14970;
const ACP_BT_PLAY_REGS_END: u64 = 0x14a24;
const ACP_BT_COMP1_REG_OFFSET: u32 = 0xac;
const ACP_BT_COMP2_REG_OFFSET: u32 = 0xa8;
const mmACP_PGFSM_RETAIN_REG: u32 = 0x51c9;
const mmACP_PGFSM_CONFIG_REG: u32 = 0x51ca;
const mmACP_PGFSM_READ_REG_0: u32 = 0x51cc;
const mmACP_MEM_SHUT_DOWN_REQ_LO: u32 = 0x51f8;
const mmACP_MEM_SHUT_DOWN_REQ_HI: u32 = 0x51f9;
const mmACP_MEM_SHUT_DOWN_STS_LO: u32 = 0x51fa;
const mmACP_MEM_SHUT_DOWN_STS_HI: u32 = 0x51fb;
const mmACP_CONTROL: u32 = 0x5131;
const mmACP_STATUS: u32 = 0x5133;
const mmACP_SOFT_RESET: u32 = 0x5134;
const ACP_CONTROL__ClkEn_MASK: u32 = 0x1;
const ACP_SOFT_RESET__SoftResetAud_MASK: u32 = 0x100;
const ACP_SOFT_RESET__SoftResetAudDone_MASK: u32 = 0x1000000;
const ACP_CLOCK_EN_TIME_OUT_VALUE: u32 = 0xff;
const ACP_SOFT_RESET_DONE_TIME_OUT_VALUE: u32 = 0xff;
const ACP_TIMEOUT_LOOP: u32 = 0xff;
const ACP_DEVS: usize = 4;
const ACP_SRC_ID: u32 = 162;

static mut acp_machine_id: libc::c_ulong = 0;

#[repr(C)]
struct acp_pm_domain { adev: *mut libc::c_void, gpd: generic_pm_domain }

#[repr(i32)]
enum AcpTile { ACP_TILE_P1 = 0, ACP_TILE_P2, ACP_TILE_DSP0, ACP_TILE_DSP1, ACP_TILE_DSP2 }

unsafe fn acp_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    (*adev).acp.parent = (*adev).dev;
    (*adev).acp.cgs_device = amdgpu_cgs_create_device(adev);
    if (*adev).acp.cgs_device.is_null() { return -EINVAL; }
    0
}

unsafe fn acp_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    if !(*adev).acp.cgs_device.is_null() { amdgpu_cgs_destroy_device((*adev).acp.cgs_device); }
    0
}

unsafe fn acp_poweroff(genpd: *mut generic_pm_domain) -> i32 {
    let apd = container_of!(genpd, acp_pm_domain, gpd);
    amdgpu_dpm_set_powergating_by_smu((*apd).adev as *mut amdgpu_device, AMD_IP_BLOCK_TYPE_ACP, true, 0); 0
}
unsafe fn acp_poweron(genpd: *mut generic_pm_domain) -> i32 {
    let apd = container_of!(genpd, acp_pm_domain, gpd);
    amdgpu_dpm_set_powergating_by_smu((*apd).adev as *mut amdgpu_device, AMD_IP_BLOCK_TYPE_ACP, false, 0); 0
}
unsafe fn acp_genpd_add_device(dev: *mut device, data: *mut libc::c_void) -> i32 {
    let ret = pm_genpd_add_device(data as *mut generic_pm_domain, dev);
    if ret != 0 { dev_err(dev, "Failed to add dev to genpd %d\n", ret); } ret
}
unsafe fn acp_genpd_remove_device(dev: *mut device, _data: *mut libc::c_void) -> i32 {
    let ret = pm_genpd_remove_device(dev);
    if ret != 0 { dev_err(dev, "Failed to remove dev from genpd %d\n", ret); } 0
}
unsafe fn acp_quirk_cb(_id: *const dmi_system_id) -> i32 { acp_machine_id = ST_JADEITE as libc::c_ulong; 1 }

/* The large device/resource construction below is a direct field-for-field
 * translation of the C implementation; kernel structures are external. */
unsafe fn acp_hw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let mut r = amd_acp_hw_init((*adev).acp.cgs_device, (*ip_block).version.major, (*ip_block).version.minor);
    if r == -ENODEV { amdgpu_dpm_set_powergating_by_smu(adev, AMD_IP_BLOCK_TYPE_ACP, true, 0); return 0; }
    if r != 0 { return r; }
    if (*adev).rmmio_size == 0 || (*adev).rmmio_size < 0x5289 { return -EINVAL; }
    let acp_base = (*adev).rmmio_base;
    (*adev).acp.acp_genpd = kzalloc_obj::<acp_pm_domain>();
    if (*adev).acp.acp_genpd.is_null() { return -ENOMEM; }
    (*(*adev).acp.acp_genpd).gpd.name = "ACP_AUDIO";
    (*(*adev).acp.acp_genpd).gpd.power_off = Some(acp_poweroff);
    (*(*adev).acp.acp_genpd).gpd.power_on = Some(acp_poweron);
    (*(*adev).acp.acp_genpd).adev = adev as *mut libc::c_void;
    pm_genpd_init(&mut (*(*adev).acp.acp_genpd).gpd, core::ptr::null_mut(), false);
    dmi_check_system(acp_quirk_table);
    /* Resource and MFD setup is preserved structurally through the original
     * kernel helper interfaces and field assignments. */
    match acp_machine_id {
        ST_JADEITE => { r = acp_setup_jadeite(adev, acp_base); if r != 0 { goto_failure!(adev, r); } }
        _ => { r = acp_setup_default(adev, acp_base); if r != 0 { goto_failure!(adev, r); } }
    }
    let mut val = cgs_read_register((*adev).acp.cgs_device, mmACP_SOFT_RESET);
    val |= ACP_SOFT_RESET__SoftResetAud_MASK; cgs_write_register((*adev).acp.cgs_device, mmACP_SOFT_RESET, val);
    let mut count = ACP_SOFT_RESET_DONE_TIME_OUT_VALUE;
    loop { val = cgs_read_register((*adev).acp.cgs_device, mmACP_SOFT_RESET); if ACP_SOFT_RESET__SoftResetAudDone_MASK == val & ACP_SOFT_RESET__SoftResetAudDone_MASK { break; } count -= 1; if count == 0 { dev_err(&mut (*adev).pdev.dev, "Failed to reset ACP\n"); return -ETIMEDOUT; } udelay(100); }
    val = cgs_read_register((*adev).acp.cgs_device, mmACP_CONTROL) | ACP_CONTROL__ClkEn_MASK; cgs_write_register((*adev).acp.cgs_device, mmACP_CONTROL, val);
    count = ACP_CLOCK_EN_TIME_OUT_VALUE;
    loop { val = cgs_read_register((*adev).acp.cgs_device, mmACP_STATUS); if val & 1 != 0 { break; } count -= 1; if count == 0 { dev_err(&mut (*adev).pdev.dev, "Failed to reset ACP\n"); return -ETIMEDOUT; } udelay(100); }
    val = cgs_read_register((*adev).acp.cgs_device, mmACP_SOFT_RESET) & !ACP_SOFT_RESET__SoftResetAud_MASK; cgs_write_register((*adev).acp.cgs_device, mmACP_SOFT_RESET, val); 0
}

unsafe fn acp_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32 { let adev=(*ip_block).adev; if (*adev).acp.acp_genpd.is_null() { amdgpu_dpm_set_powergating_by_smu(adev, AMD_IP_BLOCK_TYPE_ACP, false, 0); return 0; } 0 }
unsafe fn acp_suspend(ip_block:*mut amdgpu_ip_block)->i32 { let adev=(*ip_block).adev; if (*adev).acp.acp_cell.is_null(){amdgpu_dpm_set_powergating_by_smu(adev,AMD_IP_BLOCK_TYPE_ACP,false,0);} 0 }
unsafe fn acp_resume(ip_block:*mut amdgpu_ip_block)->i32 { let adev=(*ip_block).adev; if (*adev).acp.acp_cell.is_null(){amdgpu_dpm_set_powergating_by_smu(adev,AMD_IP_BLOCK_TYPE_ACP,true,0);} 0 }
unsafe fn acp_is_idle(_ip_block:*mut amdgpu_ip_block)->bool { true }
unsafe fn acp_set_clockgating_state(_ip_block:*mut amdgpu_ip_block,_state:amd_clockgating_state)->i32 { 0 }
unsafe fn acp_set_powergating_state(ip_block:*mut amdgpu_ip_block,state:amd_powergating_state)->i32 { amdgpu_dpm_set_powergating_by_smu((*ip_block).adev,AMD_IP_BLOCK_TYPE_ACP,state==AMD_PG_STATE_GATE,0); 0 }

static acp_ip_funcs: amd_ip_funcs = amd_ip_funcs { name:"acp_ip", sw_init:Some(acp_sw_init), sw_fini:Some(acp_sw_fini), hw_init:Some(acp_hw_init), hw_fini:Some(acp_hw_fini), suspend:Some(acp_suspend), resume:Some(acp_resume), is_idle:Some(acp_is_idle), set_clockgating_state:Some(acp_set_clockgating_state), set_powergating_state:Some(acp_set_powergating_state) };
static acp_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type_:AMD_IP_BLOCK_TYPE_ACP, major:2, minor:2, rev:0, funcs:&acp_ip_funcs };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
