/* Rust translation of ppatomfwctrl.c. External kernel/ATOM definitions are supplied by dependent modules. */

unsafe fn pp_atomfwctrl_lookup_voltage_type_v4(info: *const atom_voltage_objects_info_v4_1, voltage_type: u8, voltage_mode: u8) -> *const atom_voltage_object_v4 {
    let size = le16_to_cpu((*info).table_header.structuresize) as usize;
    let mut offset = core::mem::offset_of!(atom_voltage_objects_info_v4_1, voltage_object);
    let start = info as usize;
    while offset + core::mem::size_of::<atom_voltage_object_header_v4>() <= size {
        let obj = (start + offset) as *const atom_voltage_object_v4;
        let obj_size = le16_to_cpu((*obj).gpio_voltage_obj.header.object_size) as usize;
        if obj_size < core::mem::size_of_val(&(*obj).gpio_voltage_obj.header) || offset + obj_size > size { break; }
        if voltage_type == (*obj).gpio_voltage_obj.header.voltage_type && voltage_mode == (*obj).gpio_voltage_obj.header.voltage_mode { return obj; }
        offset += obj_size;
    }
    core::ptr::null()
}

unsafe fn pp_atomfwctrl_get_voltage_info_table(hwmgr: *mut pp_hwmgr) -> *mut atom_voltage_objects_info_v4_1 {
    let idx = GetIndexIntoMasterDataTable(voltageobject_info);
    let addr = smu_atom_get_data_table((*hwmgr).adev, idx, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    PP_ASSERT_WITH_CODE(!addr.is_null(), "Error retrieving BIOS Table Address!", return core::ptr::null_mut());
    addr as *mut atom_voltage_objects_info_v4_1
}

pub unsafe fn pp_atomfwctrl_is_voltage_controlled_by_gpio_v4(hwmgr: *mut pp_hwmgr, voltage_type: u8, voltage_mode: u8) -> bool {
    let info = pp_atomfwctrl_get_voltage_info_table(hwmgr);
    PP_ASSERT_WITH_CODE(!info.is_null(), "Could not find Voltage Table in BIOS.", return false);
    !pp_atomfwctrl_lookup_voltage_type_v4(info, voltage_type, voltage_mode).is_null()
}

pub unsafe fn pp_atomfwctrl_get_voltage_table_v4(hwmgr: *mut pp_hwmgr, voltage_type: u8, voltage_mode: u8, table: *mut pp_atomfwctrl_voltage_table) -> i32 {
    let info = pp_atomfwctrl_get_voltage_info_table(hwmgr);
    PP_ASSERT_WITH_CODE(!info.is_null(), "Could not find Voltage Table in BIOS.", return -1);
    let obj = pp_atomfwctrl_lookup_voltage_type_v4(info, voltage_type, voltage_mode);
    if obj.is_null() { return -1; }
    (*table).count = 0;
    let mut result = 0;
    if voltage_mode == VOLTAGE_OBJ_GPIO_LUT {
        PP_ASSERT_WITH_CODE((*obj).gpio_voltage_obj.gpio_entry_num <= PP_ATOMFWCTRL_MAX_VOLTAGE_ENTRIES, "Too many voltage entries!", result = -1);
        if result == 0 {
            for i in 0..(*obj).gpio_voltage_obj.gpio_entry_num as usize {
                (*table).entries[i].value = le16_to_cpu((*obj).gpio_voltage_obj.voltage_gpio_lut[i].voltage_level_mv);
                (*table).entries[i].smio_low = le32_to_cpu((*obj).gpio_voltage_obj.voltage_gpio_lut[i].voltage_gpio_reg_val);
            }
            (*table).count = (*obj).gpio_voltage_obj.gpio_entry_num;
            (*table).mask_low = le32_to_cpu((*obj).gpio_voltage_obj.gpio_mask_val);
            (*table).phase_delay = (*obj).gpio_voltage_obj.phase_delay_us;
        }
    } else if voltage_mode == VOLTAGE_OBJ_SVID2 {
        (*table).psi1_enable = ((*obj).svid2_voltage_obj.loadline_psi1 & 0x20) >> 5;
        (*table).psi0_enable = (*obj).svid2_voltage_obj.psi0_enable & 1;
        (*table).max_vid_step = (*obj).svid2_voltage_obj.maxvstep;
        (*table).telemetry_offset = (*obj).svid2_voltage_obj.telemetry_offset;
        (*table).telemetry_slope = (*obj).svid2_voltage_obj.telemetry_gain;
    } else { PP_ASSERT_WITH_CODE(false, "Unsupported Voltage Object Mode!", result = -1); }
    result
}

pub unsafe fn pp_atomfwctrl_get_gpu_pll_dividers_vega10(hwmgr: *mut pp_hwmgr, clock_type: u32, clock_value: u32, dividers: *mut pp_atomfwctrl_clock_dividers_soc15) -> i32 {
    let adev = (*hwmgr).adev;
    let mut p: compute_gpu_clock_input_parameter_v1_8 = core::mem::zeroed();
    p.gpuclock_10khz = clock_value; p.gpu_clock_type = clock_type;
    let idx = GetIndexIntoMasterCmdTable(computegpuclockparam);
    if amdgpu_atom_execute_table((*adev).mode_info.atom_context, idx, &mut p as *mut _ as *mut u32, core::mem::size_of_val(&p)) != 0 { return -EINVAL; }
    let o = &*(&p as *const _ as *const compute_gpu_clock_output_parameter_v1_8);
    (*dividers).ulClock = le32_to_cpu(o.gpuclock_10khz); (*dividers).ulDid = le32_to_cpu(o.dfs_did);
    (*dividers).ulPll_fb_mult = le32_to_cpu(o.pll_fb_mult); (*dividers).ulPll_ss_fbsmult = le32_to_cpu(o.pll_ss_fbsmult);
    (*dividers).usPll_ss_slew_frac = le16_to_cpu(o.pll_ss_slew_frac); (*dividers).ucPll_ss_enable = o.pll_ss_enable; 0
}

unsafe fn copy_avfs_common(p: *mut pp_atomfwctrl_avfs_parameters, max: u32, min: u32, a0: u32, a1: u32, a2: u32, dc: u16, mean: u16, sigma: u16, info: &atom_asic_profiling_info_v4_1) {
    (*p).ulMaxVddc=max; (*p).ulMinVddc=min; (*p).ulMeanNsigmaAcontant0=a0; (*p).ulMeanNsigmaAcontant1=a1; (*p).ulMeanNsigmaAcontant2=a2;
    (*p).usMeanNsigmaDcTolSigma=dc; (*p).usMeanNsigmaPlatformMean=mean; (*p).usMeanNsigmaPlatformSigma=sigma;
    (*p).ulGbVdroopTableCksoffA0=le32_to_cpu(info.gb_vdroop_table_cksoff_a0); (*p).ulGbVdroopTableCksoffA1=le32_to_cpu(info.gb_vdroop_table_cksoff_a1); (*p).ulGbVdroopTableCksoffA2=le32_to_cpu(info.gb_vdroop_table_cksoff_a2);
    (*p).ulGbVdroopTableCksonA0=le32_to_cpu(info.gb_vdroop_table_ckson_a0); (*p).ulGbVdroopTableCksonA1=le32_to_cpu(info.gb_vdroop_table_ckson_a1); (*p).ulGbVdroopTableCksonA2=le32_to_cpu(info.gb_vdroop_table_ckson_a2);
}

pub unsafe fn pp_atomfwctrl_get_avfs_information(hwmgr: *mut pp_hwmgr, param: *mut pp_atomfwctrl_avfs_parameters) -> i32 {
    let idx=GetIndexIntoMasterDataTable(asic_profiling_info); let profile=smu_atom_get_data_table((*hwmgr).adev,idx,core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut()) as *mut atom_asic_profiling_info_v4_1;
    if profile.is_null(){return -1;} let hdr=profile as *const atom_common_table_header;
    if (*hdr).format_revision != 4 { pr_info!("Invalid VBIOS AVFS ProfilingInfo Revision!\n"); return -EINVAL; }
    if (*hdr).content_revision == 1 { copy_avfs_common(param,le32_to_cpu((*profile).maxvddc),le32_to_cpu((*profile).minvddc),le32_to_cpu((*profile).avfs_meannsigma_acontant0),le32_to_cpu((*profile).avfs_meannsigma_acontant1),le32_to_cpu((*profile).avfs_meannsigma_acontant2),le16_to_cpu((*profile).avfs_meannsigma_dc_tol_sigma),le16_to_cpu((*profile).avfs_meannsigma_platform_mean),le16_to_cpu((*profile).avfs_meannsigma_platform_sigma),&*profile); }
    else if (*hdr).content_revision == 2 { let p=&*(profile as *const atom_asic_profiling_info_v4_2); copy_avfs_common(param,le32_to_cpu(p.maxvddc),le32_to_cpu(p.minvddc),le32_to_cpu(p.avfs_meannsigma_acontant0),le32_to_cpu(p.avfs_meannsigma_acontant1),le32_to_cpu(p.avfs_meannsigma_acontant2),le16_to_cpu(p.avfs_meannsigma_dc_tol_sigma),le16_to_cpu(p.avfs_meannsigma_platform_mean),le16_to_cpu(p.avfs_meannsigma_platform_sigma),&*profile); }
    else { pr_info!("Invalid VBIOS AVFS ProfilingInfo Revision!\n"); return -EINVAL; } 0
}

pub unsafe fn pp_atomfwctrl_get_gpio_information(hwmgr:*mut pp_hwmgr,param:*mut pp_atomfwctrl_gpio_parameters)->i32 {
 let i=smu_atom_get_data_table((*hwmgr).adev,GetIndexIntoMasterDataTable(smu_info),core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut()) as *mut atom_smu_info_v3_1; if i.is_null(){pr_info!("Error retrieving BIOS smu_info Table Address!");return -1;}
 (*param).ucAcDcGpio=(*i).ac_dc_gpio_bit;(*param).ucAcDcPolarity=(*i).ac_dc_polarity;(*param).ucVR0HotGpio=(*i).vr0hot_gpio_bit;(*param).ucVR0HotPolarity=(*i).vr0hot_polarity;(*param).ucVR1HotGpio=(*i).vr1hot_gpio_bit;(*param).ucVR1HotPolarity=(*i).vr1hot_polarity;(*param).ucFwCtfGpio=(*i).fw_ctf_gpio_bit;(*param).ucFwCtfPolarity=(*i).fw_ctf_polarity;0
}
pub unsafe fn pp_atomfwctrl_get_clk_information_by_clkid(hwmgr:*mut pp_hwmgr,clk_id:u8,syspll_id:u8,frequency:*mut u32)->i32 { let mut p:atom_get_smu_clock_info_parameters_v3_1=core::mem::zeroed();p.clk_id=clk_id;p.syspll_id=syspll_id;p.command=GET_SMU_CLOCK_INFO_V3_1_GET_CLOCK_FREQ;p.dfsdid=0;let ix=GetIndexIntoMasterCmdTable(getsmuclockinfo);if amdgpu_atom_execute_table((*(*hwmgr).adev).mode_info.atom_context,ix,&mut p as *mut _ as *mut u32,core::mem::size_of_val(&p))!=0{return -EINVAL;}let o=&*(&p as *const _ as *const atom_get_smu_clock_info_output_parameters_v3_1);*frequency=le32_to_cpu(o.atom_smu_outputclkfreq.smu_clock_freq_hz)/10000;0 }
unsafe fn copy_boot(h:*mut pp_hwmgr,b:*mut pp_atomfwctrl_bios_boot_up_values,rev:u32,gfx:u32,u:u32,vd:u16,vdi:u16,m:u16,vg:u16,c:u8,ids:&[(u8,u8)]) {(*b).ulRevision=rev;(*b).ulGfxClk=gfx;(*b).ulUClk=u;(*b).usVddc=vd;(*b).usVddci=vdi;(*b).usMvddc=m;(*b).usVddGfx=vg;(*b).ucCoolingID=c;for &(id,pll) in ids {let mut f=0;if pp_atomfwctrl_get_clk_information_by_clkid(h,id,pll,&mut f)==0{match id{SMU11_SYSPLL0_SOCCLK_ID|SMU9_SYSPLL0_SOCCLK_ID=>(*b).ulSocClk=f,SMU11_SYSPLL0_DCEFCLK_ID|SMU9_SYSPLL0_DCEFCLK_ID=>(*b).ulDCEFClk=f,SMU11_SYSPLL0_ECLK_ID|SMU9_SYSPLL0_ECLK_ID=>(*b).ulEClk=f,SMU11_SYSPLL0_VCLK_ID|SMU9_SYSPLL0_VCLK_ID=>(*b).ulVClk=f,SMU11_SYSPLL0_DCLK_ID|SMU9_SYSPLL0_DCLK_ID=>(*b).ulDClk=f,_=>()} }}}
pub unsafe fn pp_atomfwctrl_get_vbios_bootup_values(h:*mut pp_hwmgr,b:*mut pp_atomfwctrl_bios_boot_up_values)->i32 {let i=smu_atom_get_data_table((*h).adev,GetIndexIntoMasterDataTable(firmwareinfo),core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut()) as *mut atom_common_table_header;if i.is_null(){pr_info!("Error retrieving BIOS firmwareinfo!");return -EINVAL;}if (*i).format_revision==3&&(*i).content_revision==2{let f=&*(i as *const atom_firmware_info_v3_2);copy_boot(h,b,f.firmware_revision,f.bootup_sclk_in10khz,f.bootup_mclk_in10khz,f.bootup_vddc_mv,f.bootup_vddci_mv,f.bootup_mvddc_mv,f.bootup_vddgfx_mv,f.coolingsolution_id,&[(SMU11_SYSPLL0_SOCCLK_ID,SMU11_SYSPLL0_ID),(SMU11_SYSPLL0_DCEFCLK_ID,SMU11_SYSPLL0_ID),(SMU11_SYSPLL0_ECLK_ID,SMU11_SYSPLL0_ID),(SMU11_SYSPLL0_VCLK_ID,SMU11_SYSPLL0_ID),(SMU11_SYSPLL0_DCLK_ID,SMU11_SYSPLL0_ID),(SMU11_SYSPLL1_0_FCLK_ID,SMU11_SYSPLL1_2_ID)]);}else if (*i).format_revision==3&&(*i).content_revision==1{let f=&*(i as *const atom_firmware_info_v3_1);copy_boot(h,b,f.firmware_revision,f.bootup_sclk_in10khz,f.bootup_mclk_in10khz,f.bootup_vddc_mv,f.bootup_vddci_mv,f.bootup_mvddc_mv,f.bootup_vddgfx_mv,f.coolingsolution_id,&[(SMU9_SYSPLL0_SOCCLK_ID,0),(SMU9_SYSPLL0_DCEFCLK_ID,0),(SMU9_SYSPLL0_ECLK_ID,0),(SMU9_SYSPLL0_VCLK_ID,0),(SMU9_SYSPLL0_DCLK_ID,0)]);}else{return -EINVAL;}0}
pub unsafe fn pp_atomfwctrl_get_smc_dpm_information(h:*mut pp_hwmgr,p:*mut pp_atomfwctrl_smc_dpm_parameters)->i32 {let i=smu_atom_get_data_table((*h).adev,GetIndexIntoMasterDataTable(smc_dpm_info),core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut()) as *mut atom_smc_dpm_info_v4_1;if i.is_null(){return -EINVAL;}core::ptr::copy_nonoverlapping(i as *const u8, p as *mut u8, core::mem::size_of::<pp_atomfwctrl_smc_dpm_parameters>());0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
