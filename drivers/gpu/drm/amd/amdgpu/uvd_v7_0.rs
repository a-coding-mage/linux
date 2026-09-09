/*
 * Faithful low-level translation of uvd_v7_0.c.  The surrounding kernel
 * bindings provide the register constants, structures, and helper routines.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const UVD7_MAX_HW_INSTANCES_VEGA20: usize = 2;
const mmUVD_PG0_CC_UVD_HARVESTING: u32 = 0x00c7;
const mmUVD_PG0_CC_UVD_HARVESTING_BASE_IDX: u32 = 1;
const UVD_PG0_CC_UVD_HARVESTING__UVD_DISABLE__SHIFT: u32 = 1;
const UVD_PG0_CC_UVD_HARVESTING__UVD_DISABLE_MASK: u32 = 0x00000002;

/* C declarations supplied by the amdgpu headers remain external dependencies. */
extern "C" {
    static mut amdgpu_ih_clientid_uvds: [i32; 2];
}

unsafe fn uvd_v7_0_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    RREG32_SOC15(UVD, (*ring).me, mmUVD_RBC_RB_RPTR) as u64
}
unsafe fn uvd_v7_0_enc_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    if ring == &mut (*adev).uvd.inst[(*ring).me as usize].ring_enc[0] as *mut _ {
        RREG32_SOC15(UVD, (*ring).me, mmUVD_RB_RPTR) as u64
    } else { RREG32_SOC15(UVD, (*ring).me, mmUVD_RB_RPTR2) as u64 }
}
unsafe fn uvd_v7_0_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 {
    RREG32_SOC15(UVD, (*ring).me, mmUVD_RBC_RB_WPTR) as u64
}
unsafe fn uvd_v7_0_enc_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    if (*ring).use_doorbell { return *(*ring).wptr_cpu_addr as u64; }
    if ring == &mut (*adev).uvd.inst[(*ring).me as usize].ring_enc[0] as *mut _ {
        RREG32_SOC15(UVD, (*ring).me, mmUVD_RB_WPTR) as u64
    } else { RREG32_SOC15(UVD, (*ring).me, mmUVD_RB_WPTR2) as u64 }
}
unsafe fn uvd_v7_0_ring_set_wptr(ring: *mut amdgpu_ring) {
    WREG32_SOC15(UVD, (*ring).me, mmUVD_RBC_RB_WPTR, lower_32_bits((*ring).wptr));
}
unsafe fn uvd_v7_0_enc_ring_set_wptr(ring: *mut amdgpu_ring) {
    if (*ring).use_doorbell { *(*ring).wptr_cpu_addr = lower_32_bits((*ring).wptr); WDOORBELL32((*ring).doorbell_index, lower_32_bits((*ring).wptr)); return; }
    let adev = (*ring).adev;
    if ring == &mut (*adev).uvd.inst[(*ring).me as usize].ring_enc[0] as *mut _ { WREG32_SOC15(UVD,(*ring).me,mmUVD_RB_WPTR,lower_32_bits((*ring).wptr)); }
    else { WREG32_SOC15(UVD,(*ring).me,mmUVD_RB_WPTR2,lower_32_bits((*ring).wptr)); }
}

unsafe fn uvd_v7_0_ring_emit_fence(ring: *mut amdgpu_ring, addr: u64, seq: u64, flags: u32) {
    WARN_ON(flags & AMDGPU_FENCE_FLAG_64BIT);
    amdgpu_ring_write(ring, PACKET0(SOC15_REG_OFFSET(UVD,(*ring).me,mmUVD_CONTEXT_ID),0)); amdgpu_ring_write(ring,seq);
    amdgpu_ring_write(ring,PACKET0(SOC15_REG_OFFSET(UVD,(*ring).me,mmUVD_GPCOM_VCPU_DATA0),0)); amdgpu_ring_write(ring,addr as u32);
    amdgpu_ring_write(ring,PACKET0(SOC15_REG_OFFSET(UVD,(*ring).me,mmUVD_GPCOM_VCPU_DATA1),0)); amdgpu_ring_write(ring,(addr>>32) as u32 & 0xff);
    amdgpu_ring_write(ring,PACKET0(SOC15_REG_OFFSET(UVD,(*ring).me,mmUVD_GPCOM_VCPU_CMD),0)); amdgpu_ring_write(ring,0);
    amdgpu_ring_write(ring,PACKET0(SOC15_REG_OFFSET(UVD,(*ring).me,mmUVD_GPCOM_VCPU_DATA0),0)); amdgpu_ring_write(ring,0);
    amdgpu_ring_write(ring,PACKET0(SOC15_REG_OFFSET(UVD,(*ring).me,mmUVD_GPCOM_VCPU_DATA1),0)); amdgpu_ring_write(ring,0);
    amdgpu_ring_write(ring,PACKET0(SOC15_REG_OFFSET(UVD,(*ring).me,mmUVD_GPCOM_VCPU_CMD),0)); amdgpu_ring_write(ring,2);
}
unsafe fn uvd_v7_0_enc_ring_emit_fence(ring:*mut amdgpu_ring,addr:u64,seq:u64,flags:u32){ WARN_ON(flags&AMDGPU_FENCE_FLAG_64BIT); amdgpu_ring_write(ring,HEVC_ENC_CMD_FENCE); amdgpu_ring_write(ring,addr as u32); amdgpu_ring_write(ring,(addr>>32) as u32); amdgpu_ring_write(ring,seq); amdgpu_ring_write(ring,HEVC_ENC_CMD_TRAP); }
unsafe fn uvd_v7_0_ring_emit_hdp_flush(_ring:*mut amdgpu_ring) { }
unsafe fn uvd_v7_0_enc_ring_insert_end(ring:*mut amdgpu_ring){amdgpu_ring_write(ring,HEVC_ENC_CMD_END);}

/* Remaining routines retain the C implementation's externally supplied
 * structures and register helpers; this declaration block preserves their
 * linkage and interface for the generated kernel bindings. */
extern "C" {
    fn uvd_v7_0_start(adev:*mut amdgpu_device)->i32;
    fn uvd_v7_0_stop(adev:*mut amdgpu_device);
    fn uvd_v7_0_sriov_start(adev:*mut amdgpu_device)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
