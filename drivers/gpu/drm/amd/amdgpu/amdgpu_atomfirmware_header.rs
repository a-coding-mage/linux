/*
 * Copyright 2014 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C dependency: get_index_into_master_table(master_table, table_name)
// expands to offsetof(struct master_table, table_name) / sizeof(uint16_t).
#[macro_export]
macro_rules! get_index_into_master_table {
    ($master_table:ty, $table_name:ident) => {
        ::core::mem::offset_of!($master_table, $table_name) / ::core::mem::size_of::<u16>()
    };
}

// External declarations supplied by other translation units.
extern "C" {
    pub fn amdgpu_atomfirmware_query_firmware_capability(
        adev: *mut amdgpu_device,
    ) -> u32;
    pub fn amdgpu_atomfirmware_gpu_virtualization_supported(
        adev: *mut amdgpu_device,
    ) -> bool;
    pub fn amdgpu_atomfirmware_scratch_regs_init(adev: *mut amdgpu_device);
    pub fn amdgpu_atomfirmware_allocate_fb_scratch(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_atomfirmware_get_integrated_system_info(
        adev: *mut amdgpu_device,
        vram_width: *mut i32,
        vram_type: *mut i32,
        vram_vendor: *mut i32,
    ) -> i32;
    pub fn amdgpu_atomfirmware_get_umc_info(
        adev: *mut amdgpu_device,
        vram_width: *mut i32,
        vram_type: *mut i32,
        vram_vendor: *mut i32,
    ) -> i32;
    pub fn amdgpu_atomfirmware_get_vram_info(
        adev: *mut amdgpu_device,
        vram_width: *mut i32,
        vram_type: *mut i32,
        vram_vendor: *mut i32,
    ) -> i32;
    pub fn amdgpu_atomfirmware_get_uma_carveout_info(
        adev: *mut amdgpu_device,
        uma_info: *mut amdgpu_uma_carveout_info,
    ) -> i32;
    pub fn amdgpu_atomfirmware_get_clock_info(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_atomfirmware_get_gfx_info(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_atomfirmware_mem_ecc_supported(adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_atomfirmware_sram_ecc_supported(adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_atomfirmware_ras_rom_addr(
        adev: *mut amdgpu_device,
        i2c_address: *mut u8,
    ) -> bool;
    pub fn amdgpu_atomfirmware_mem_training_supported(adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_atomfirmware_dynamic_boot_config_supported(
        adev: *mut amdgpu_device,
    ) -> bool;
    pub fn amdgpu_atomfirmware_get_fw_reserved_fb_size(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_atomfirmware_asic_init(adev: *mut amdgpu_device, fb_reset: bool) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
