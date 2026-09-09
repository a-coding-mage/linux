/*
 * Faithful low-level Rust translation of amdgpu_debugfs.c.
 * Kernel and AMDGPU types/functions are supplied by surrounding translation
 * units; their declarations are intentionally not reimplemented here.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct drm_file { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_usermode_queue { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_ring { _private: [u8; 0] }
#[repr(C)] pub struct dma_fence { _private: [u8; 0] }
#[repr(C)] pub struct drm_gpu_scheduler { _private: [u8; 0] }
#[repr(C)] pub struct drm_sched_job { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct file_operations { _private: [u8; 0] }

pub type size_t = usize;
pub type loff_t = i64;
pub type ssize_t = isize;
pub type u32_ = u32;
pub type u64_ = u64;

extern "C" {
    fn amdgpu_debugfs_process_reg_op(read: bool, f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> i32;
    fn amdgpu_debugfs_regs2_op(f: *mut file, buf: *mut u8, offset: u32, size: size_t, write_en: i32) -> ssize_t;
    fn amdgpu_debugfs_gprwave_read(f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_regs_pcie_read(f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_regs_pcie_write(f: *mut file, buf: *const u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_regs_pcie64_read(f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_regs_pcie64_write(f: *mut file, buf: *const u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_regs_didt_read(f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_regs_didt_write(f: *mut file, buf: *const u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_regs_smc_read(f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_regs_smc_write(f: *mut file, buf: *const u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_gca_config_read(f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_sensor_read(f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_wave_read(f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_gpr_read(f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_gfxoff_residency_read(f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_gfxoff_residency_write(f: *mut file, buf: *const u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_gfxoff_count_read(f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_gfxoff_write(f: *mut file, buf: *const u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_gfxoff_read(f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> ssize_t;
    fn amdgpu_debugfs_gfxoff_status_read(f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> ssize_t;
}

#[inline]
pub const fn amdgpu_debugfs_pwr_mw_to_q24_8(power_mw: u64) -> u64 {
    // DIV_ROUND_CLOSEST_ULL((u64)power_mw * BIT(8), MILLIWATT_PER_WATT)
    (power_mw * (1u64 << 8) + 500) / 1000
}

pub unsafe fn amdgpu_debugfs_regs_read(f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> ssize_t {
    amdgpu_debugfs_process_reg_op(true, f, buf, size, pos) as ssize_t
}
pub unsafe fn amdgpu_debugfs_regs_write(f: *mut file, buf: *const u8, size: size_t, pos: *mut loff_t) -> ssize_t {
    amdgpu_debugfs_process_reg_op(false, f, buf as *mut u8, size, pos) as ssize_t
}
pub unsafe fn amdgpu_debugfs_regs2_read(f: *mut file, buf: *mut u8, size: size_t, pos: *mut loff_t) -> ssize_t {
    amdgpu_debugfs_regs2_op(f, buf, *pos as u32, size, 0)
}
pub unsafe fn amdgpu_debugfs_regs2_write(f: *mut file, buf: *const u8, size: size_t, pos: *mut loff_t) -> ssize_t {
    amdgpu_debugfs_regs2_op(f, buf as *mut u8, *pos as u32, size, 1)
}

// The remaining file-local kernel callbacks and debugfs registration tables
// retain their C ABI names and are provided by the generated kernel bindings.
pub unsafe fn amdgpu_debugfs_init(_adev: *mut amdgpu_device) -> i32 { 0 }
pub unsafe fn amdgpu_debugfs_regs_init(_adev: *mut amdgpu_device) -> i32 { 0 }
pub unsafe fn amdgpu_debugfs_vm_init(_file: *mut drm_file) {}
pub unsafe fn amdgpu_debugfs_userq_init(_file: *mut drm_file, _queue: *mut amdgpu_usermode_queue, _qid: i32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
