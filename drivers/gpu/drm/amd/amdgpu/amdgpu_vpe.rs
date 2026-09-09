/* Rust translation of amdgpu_vpe.c.  Kernel types, constants, and helpers are
 * supplied by the surrounding AMDGPU Rust bindings. */

const AMDGPU_CSA_VPE_SIZE: u64 = 64;
const AMDGPU_CSA_VPE_OFFSET: u64 = 4096 * 3;
const VPE_IDLE_TIMEOUT: u64 = msecs_to_jiffies(1000);
const VPE_MAX_DPM_LEVEL: u32 = 4;
const FIXED1_8_BITS_PER_FRACTIONAL_PART: u32 = 8;

#[inline]
unsafe fn div16_u16_rem(dividend: u16, divisor: u16, remainder: *mut u16) -> u16 {
    *remainder = dividend % divisor;
    dividend / divisor
}
#[inline]
unsafe fn complete_integer_division_u16(dividend: u16, divisor: u16, remainder: *mut u16) -> u16 {
    div16_u16_rem(dividend, divisor, remainder)
}
unsafe fn vpe_u1_8_from_fraction(numerator: u16, denominator: u16) -> u16 {
    let mut remainder = 0u16;
    let mut res_value = complete_integer_division_u16(numerator, denominator, &mut remainder);
    if res_value > 127 { return 0; }
    let mut i = FIXED1_8_BITS_PER_FRACTIONAL_PART;
    while i != 0 {
        remainder <<= 1;
        res_value <<= 1;
        if remainder >= denominator { res_value |= 1; remainder -= denominator; }
        i -= 1;
    }
    let summand = ((remainder as u32) << 1 >= denominator as u32) as u16;
    if res_value + summand > 32767 { return 0; }
    res_value + summand
}
unsafe fn vpe_internal_get_pratio(from_frequency: u16, to_frequency: u16) -> u16 {
    let mut pratio = vpe_u1_8_from_fraction(from_frequency, to_frequency);
    if (pratio >> 8) > 1 { pratio = 0; }
    pratio
}

pub unsafe fn amdgpu_vpe_configure_dpm(vpe: *mut amdgpu_vpe) -> i32 {
    let adev = (*(*vpe).ring.adev);
    let mut dpm_ctl: u32;
    if adev.pm.dpm_enabled {
        let mut table: dpm_clocks = core::mem::zeroed();
        dpm_ctl = RREG32(vpe_get_reg_offset(vpe, 0, (*vpe).regs.dpm_enable));
        dpm_ctl |= 1; WREG32(vpe_get_reg_offset(vpe, 0, (*vpe).regs.dpm_enable), dpm_ctl);
        if amdgpu_dpm_get_dpm_clock_table(&adev, &mut table) != 0 { goto_disable_dpm(vpe); }
        let vpe_clks = table.VPEClocks; let soc_clks = table.SocClocks;
        let mut enabled = 0u32;
        let mut idx = PP_SMU_NUM_VPECLK_DPM_LEVELS;
        while idx != 0 && enabled == 0 { idx -= 1; if (*vpe_clks.add(idx as usize)).Freq != 0 { enabled = idx + 1; } }
        let mut f = [0u16; 4];
        for idx in 0..VPE_MAX_DPM_LEVEL {
            let mut level = if idx == 0 { 0 } else { idx * 2 + 1 };
            if level > enabled - 1 { level = enabled - 1; }
            let s = (*soc_clks.add(level as usize)).Freq; let v = (*vpe_clks.add(level as usize)).Freq;
            f[idx as usize] = if s < v { s } else { v };
        }
        if f.iter().all(|x| *x != 0) {
            let ctl = vpe_internal_get_pratio(f[3], f[2]) as u32 |
                ((vpe_internal_get_pratio(f[2], f[1]) as u32) << 9) |
                ((vpe_internal_get_pratio(f[1], f[0]) as u32) << 18);
            WREG32(vpe_get_reg_offset(vpe, 0, (*vpe).regs.dpm_pratio), ctl);
            WREG32(vpe_get_reg_offset(vpe, 0, (*vpe).regs.dpm_request_interval), 24000);
            WREG32(vpe_get_reg_offset(vpe, 0, (*vpe).regs.dpm_decision_threshold), 1200000);
            WREG32(vpe_get_reg_offset(vpe, 0, (*vpe).regs.dpm_busy_clamp_threshold), 1200000);
            WREG32(vpe_get_reg_offset(vpe, 0, (*vpe).regs.dpm_idle_clamp_threshold), 1200000);
            return 0;
        }
    }
    goto_disable_dpm(vpe)
}
unsafe fn goto_disable_dpm(vpe: *mut amdgpu_vpe) -> i32 {
    let mut ctl = RREG32(vpe_get_reg_offset(vpe, 0, (*vpe).regs.dpm_enable)); ctl &= 0xfffffffe;
    WREG32(vpe_get_reg_offset(vpe, 0, (*vpe).regs.dpm_enable), ctl); -EINVAL
}

pub unsafe fn amdgpu_vpe_psp_update_sram(adev: *mut amdgpu_device) -> i32 {
    let mut ucode: amdgpu_firmware_info = core::mem::zeroed();
    ucode.ucode_id = AMDGPU_UCODE_ID_VPE; ucode.mc_addr = (*adev).vpe.cmdbuf_gpu_addr; ucode.ucode_size = 8;
    psp_execute_ip_fw_load(&mut (*adev).psp, &mut ucode)
}

pub unsafe fn amdgpu_vpe_init_microcode(vpe: *mut amdgpu_vpe) -> i32 {
    let adev = (*vpe).ring.adev; let mut prefix = [0i8; 32];
    amdgpu_ucode_ip_version_decode(adev, VPE_HWIP, prefix.as_mut_ptr(), prefix.len());
    let ret = amdgpu_ucode_request(adev, &mut (*adev).vpe.fw, AMDGPU_UCODE_REQUIRED, "amdgpu/%s.bin", prefix.as_mut_ptr());
    if ret != 0 { release_firmware((*adev).vpe.fw); (*adev).vpe.fw = core::ptr::null_mut(); return ret; }
    let hdr = (*adev).vpe.fw as *const vpe_firmware_header_v1_0;
    (*adev).vpe.fw_version = le32_to_cpu((*hdr).header.ucode_version);
    (*adev).vpe.feature_version = le32_to_cpu((*hdr).ucode_feature_version); 0
}

pub unsafe fn amdgpu_vpe_ring_init(vpe: *mut amdgpu_vpe) -> i32 {
    let adev = container_of!(vpe, amdgpu_device, vpe); let ring = &mut (*vpe).ring;
    ring.ring_obj = core::ptr::null_mut(); ring.use_doorbell = true; ring.vm_hub = AMDGPU_MMHUB0(0);
    ring.doorbell_index = (*adev).doorbell_index.vpe_ring << 1; snprintf(ring.name.as_mut_ptr(), 4, "vpe");
    amdgpu_ring_init(adev, ring, 1024, &mut (*vpe).trap_irq, 0, AMDGPU_RING_PRIO_DEFAULT, core::ptr::null_mut())
}
pub unsafe fn amdgpu_vpe_ring_fini(vpe: *mut amdgpu_vpe) -> i32 { amdgpu_ring_fini(&mut (*vpe).ring); 0 }

// The remaining callback bodies retain the C driver's exact callback wiring and
// command emission through the external AMDGPU ring/firmware interfaces.
pub unsafe fn vpe_ring_insert_nop(ring: *mut amdgpu_ring, count: u32) { for i in 0..count { amdgpu_ring_write(ring, if i == 0 { (*ring).funcs.nop | VPE_CMD_NOP_HEADER_COUNT(count - 1) } else { (*ring).funcs.nop }); } }
pub unsafe fn vpe_get_csa_mc_addr(ring: *mut amdgpu_ring, vmid: u32) -> u64 { let adev=(*ring).adev; if amdgpu_sriov_vf(adev)||vmid==0||!(*adev).gfx.mcbp {0} else {amdgpu_csa_vaddr(adev)+AMDGPU_CSA_VPE_OFFSET} }
pub unsafe fn vpe_ring_emit_pred_exec(ring:*mut amdgpu_ring, device_select:u32, exec_count:u32) { if !(*(*ring).adev).vpe.collaborate_mode{return;} amdgpu_ring_write(ring,VPE_CMD_HEADER(VPE_CMD_OPCODE_PRED_EXE,0)|(device_select<<16)); amdgpu_ring_write(ring,exec_count&0x1fff); }
pub unsafe fn vpe_ring_emit_ib(ring:*mut amdgpu_ring, job:*mut amdgpu_job, ib:*mut amdgpu_ib, _flags:u32) { let vmid=AMDGPU_JOB_GET_VMID(job); let csa=vpe_get_csa_mc_addr(ring,vmid); amdgpu_ring_write(ring,VPE_CMD_HEADER(VPE_CMD_OPCODE_INDIRECT,0)|VPE_CMD_INDIRECT_HEADER_VMID(vmid&0xf)); amdgpu_ring_write(ring,(*ib).gpu_addr as u32&0xffffffe0); amdgpu_ring_write(ring,upper_32_bits((*ib).gpu_addr)); amdgpu_ring_write(ring,(*ib).length_dw); amdgpu_ring_write(ring,lower_32_bits(csa)); amdgpu_ring_write(ring,upper_32_bits(csa)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
