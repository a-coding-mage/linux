/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding translation unit:
// amdgpu_ras.h, amdgpu_cs.h

pub const AMDGPU_MAX_JPEG_INSTANCES: usize = 4;
pub const AMDGPU_MAX_JPEG_RINGS: usize = 10;
pub const AMDGPU_MAX_JPEG_RINGS_4_0_3: usize = 8;
pub const JPEG_REG_RANGE_START: u32 = 0x4000;
pub const JPEG_REG_RANGE_END: u32 = 0x41c2;
pub const JPEG_ATOMIC_RANGE_START: u32 = 0x4120;
pub const JPEG_ATOMIC_RANGE_END: u32 = 0x412A;
pub const AMDGPU_JPEG_HARVEST_JPEG0: u32 = 1 << 0;
pub const AMDGPU_JPEG_HARVEST_JPEG1: u32 = 1 << 1;

// C preprocessor register-access macros, retained as Rust macros because they
// depend on symbols and object layout supplied by other headers.
#[macro_export]
macro_rules! WREG32_SOC15_JPEG_DPG_MODE {
    ($adev:expr, $inst_idx:expr, $offset:expr, $value:expr, $indirect:expr) => {{
        if !$indirect {
            WREG32_SOC15!(JPEG, GET_INST!(JPEG, $inst_idx), mmUVD_DPG_LMA_DATA, $value);
            WREG32_SOC15!(JPEG, GET_INST!(JPEG, $inst_idx), mmUVD_DPG_LMA_CTL,
                UVD_DPG_LMA_CTL__READ_WRITE_MASK |
                (($offset) << UVD_DPG_LMA_CTL__READ_WRITE_ADDR__SHIFT) |
                (($indirect) << UVD_DPG_LMA_CTL__SRAM_SEL__SHIFT));
        } else {
            unsafe {
                *(*$adev).jpeg.inst[$inst_idx].dpg_sram_curr_addr = $offset;
                (*$adev).jpeg.inst[$inst_idx].dpg_sram_curr_addr =
                    (*$adev).jpeg.inst[$inst_idx].dpg_sram_curr_addr.add(1);
                *(*$adev).jpeg.inst[$inst_idx].dpg_sram_curr_addr = $value;
                (*$adev).jpeg.inst[$inst_idx].dpg_sram_curr_addr =
                    (*$adev).jpeg.inst[$inst_idx].dpg_sram_curr_addr.add(1);
            }
        }
    }};
}

#[macro_export]
macro_rules! RREG32_SOC15_JPEG_DPG_MODE {
    ($inst_idx:expr, $offset:expr, $mask_en:expr) => {{
        WREG32_SOC15!(JPEG, $inst_idx, mmUVD_DPG_LMA_CTL,
            (0x0 << UVD_DPG_LMA_CTL__READ_WRITE__SHIFT) |
            (($mask_en) << UVD_DPG_LMA_CTL__MASK_EN__SHIFT) |
            (($offset) << UVD_DPG_LMA_CTL__READ_WRITE_ADDR__SHIFT));
        RREG32_SOC15!(JPEG, $inst_idx, mmUVD_DPG_LMA_DATA)
    }};
}

#[macro_export]
macro_rules! WREG32_SOC24_JPEG_DPG_MODE {
    ($inst_idx:expr, $offset:expr, $value:expr, $indirect:expr) => {{
        WREG32_SOC15!(JPEG, GET_INST!(JPEG, $inst_idx), regUVD_DPG_LMA_DATA, $value);
        WREG32_SOC15!(JPEG, GET_INST!(JPEG, $inst_idx), regUVD_DPG_LMA_MASK, 0xFFFFFFFF);
        WREG32_SOC15!(JPEG, GET_INST!(JPEG, $inst_idx), regUVD_DPG_LMA_CTL,
            UVD_DPG_LMA_CTL__READ_WRITE_MASK |
            (($offset) << UVD_DPG_LMA_CTL__READ_WRITE_ADDR__SHIFT) |
            (($indirect) << UVD_DPG_LMA_CTL__SRAM_SEL__SHIFT));
    }};
}

#[macro_export]
macro_rules! RREG32_SOC24_JPEG_DPG_MODE {
    ($inst_idx:expr, $offset:expr, $mask_en:expr) => {{
        WREG32_SOC15!(JPEG, GET_INST!(JPEG, $inst_idx), regUVD_DPG_LMA_MASK, 0xFFFFFFFF);
        WREG32_SOC15!(JPEG, GET_INST!(JPEG, $inst_idx), regUVD_DPG_LMA_CTL,
            UVD_DPG_LMA_CTL__MASK_EN_MASK |
            (($offset) << UVD_DPG_LMA_CTL__READ_WRITE_ADDR__SHIFT));
        RREG32_SOC15!(JPEG, $inst_idx, regUVD_DPG_LMA_DATA);
    }};
}

#[macro_export]
macro_rules! ADD_SOC24_JPEG_TO_DPG_SRAM {
    ($adev:expr, $inst_idx:expr, $offset:expr, $value:expr, $indirect:expr) => {{
        unsafe {
            *(*$adev).jpeg.inst[$inst_idx].dpg_sram_curr_addr = $offset;
            (*$adev).jpeg.inst[$inst_idx].dpg_sram_curr_addr =
                (*$adev).jpeg.inst[$inst_idx].dpg_sram_curr_addr.add(1);
            *(*$adev).jpeg.inst[$inst_idx].dpg_sram_curr_addr = $value;
            (*$adev).jpeg.inst[$inst_idx].dpg_sram_curr_addr =
                (*$adev).jpeg.inst[$inst_idx].dpg_sram_curr_addr.add(1);
        }
    }};
}

pub struct amdgpu_hwip_reg_entry;

#[repr(C)]
pub enum amdgpu_jpeg_caps { AMDGPU_JPEG_RRMT_ENABLED }

pub const fn AMDGPU_JPEG_CAPS(caps: u32) -> u32 { 1 << caps }

#[repr(C)]
pub struct amdgpu_jpeg_reg { pub jpeg_pitch: [u32; AMDGPU_MAX_JPEG_RINGS] }

#[repr(C)]
pub struct amdgpu_jpeg_inst {
    pub ring_dec: [amdgpu_ring; AMDGPU_MAX_JPEG_RINGS], pub irq: amdgpu_irq_src,
    pub ras_poison_irq: amdgpu_irq_src, pub external: amdgpu_jpeg_reg,
    pub dpg_sram_bo: *mut amdgpu_bo, pub pause_state: dpg_pause_state,
    pub dpg_sram_cpu_addr: *mut core::ffi::c_void, pub dpg_sram_gpu_addr: u64,
    pub dpg_sram_curr_addr: *mut u32, pub aid_id: u8,
}

#[repr(C)] pub struct amdgpu_jpeg_ras { pub ras_block: amdgpu_ras_block_object }

#[repr(C)]
pub struct amdgpu_jpeg {
    pub num_jpeg_inst: u8, pub inst: [amdgpu_jpeg_inst; AMDGPU_MAX_JPEG_INSTANCES],
    pub num_jpeg_rings: u32, pub internal: amdgpu_jpeg_reg, pub harvest_config: u32,
    pub idle_work: delayed_work, pub cur_state: amd_powergating_state,
    pub jpeg_pg_lock: mutex, pub total_submission_cnt: atomic_t,
    pub ras_if: *mut ras_common_if, pub ras: *mut amdgpu_jpeg_ras,
    pub inst_mask: u16, pub num_inst_per_aid: u8, pub indirect_sram: bool,
    pub supported_reset: u32, pub caps: u32, pub ip_dump: *mut u32, pub reg_count: u32,
    pub reg_list: *const amdgpu_hwip_reg_entry, pub disable_uq: bool, pub disable_kq: bool,
}

extern "C" {
    pub fn amdgpu_jpeg_sw_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_jpeg_sw_fini(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_jpeg_suspend(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_jpeg_resume(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_jpeg_ring_begin_use(ring: *mut amdgpu_ring);
    pub fn amdgpu_jpeg_ring_end_use(ring: *mut amdgpu_ring);
    pub fn amdgpu_jpeg_dec_ring_test_ring(ring: *mut amdgpu_ring) -> i32;
    pub fn amdgpu_jpeg_dec_ring_test_ib(ring: *mut amdgpu_ring, timeout: i64) -> i32;
    pub fn amdgpu_jpeg_process_poison_irq(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32;
    pub fn amdgpu_jpeg_ras_late_init(adev: *mut amdgpu_device, ras_block: *mut ras_common_if) -> i32;
    pub fn amdgpu_jpeg_ras_sw_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_jpeg_psp_update_sram(adev: *mut amdgpu_device, inst_idx: i32, ucode_id: AMDGPU_UCODE_ID) -> i32;
    pub fn amdgpu_debugfs_jpeg_sched_mask_init(adev: *mut amdgpu_device);
    pub fn amdgpu_jpeg_sysfs_reset_mask_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_jpeg_sysfs_reset_mask_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_jpeg_reg_dump_init(adev: *mut amdgpu_device, reg: *const amdgpu_hwip_reg_entry, count: u32) -> i32;
    pub fn amdgpu_jpeg_dump_ip_state(ip_block: *mut amdgpu_ip_block);
    pub fn amdgpu_jpeg_print_ip_state(ip_block: *mut amdgpu_ip_block, p: *mut drm_printer);
    pub fn amdgpu_jpeg_dec_parse_cs(parser: *mut amdgpu_cs_parser, job: *mut amdgpu_job, ib: *mut amdgpu_ib) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
