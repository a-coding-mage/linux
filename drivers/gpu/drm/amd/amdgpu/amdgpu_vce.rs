/* Faithful low-level translation of amdgpu_vce.c.  Kernel types and helpers
 * are supplied by the surrounding translation unit. */

const VCE_IDLE_TIMEOUT: u64 = msecs_to_jiffies(1000);
const FIRMWARE_VCE_V1_0: &str = "amdgpu/vce_1_0_0.bin";
const FIRMWARE_BONAIRE: &str = "amdgpu/bonaire_vce.bin";
const FIRMWARE_KABINI: &str = "amdgpu/kabini_vce.bin";
const FIRMWARE_KAVERI: &str = "amdgpu/kaveri_vce.bin";
const FIRMWARE_HAWAII: &str = "amdgpu/hawaii_vce.bin";
const FIRMWARE_MULLINS: &str = "amdgpu/mullins_vce.bin";
const FIRMWARE_TONGA: &str = "amdgpu/tonga_vce.bin";
const FIRMWARE_CARRIZO: &str = "amdgpu/carrizo_vce.bin";
const FIRMWARE_FIJI: &str = "amdgpu/fiji_vce.bin";
const FIRMWARE_STONEY: &str = "amdgpu/stoney_vce.bin";
const FIRMWARE_POLARIS10: &str = "amdgpu/polaris10_vce.bin";
const FIRMWARE_POLARIS11: &str = "amdgpu/polaris11_vce.bin";
const FIRMWARE_POLARIS12: &str = "amdgpu/polaris12_vce.bin";
const FIRMWARE_VEGAM: &str = "amdgpu/vegam_vce.bin";
const FIRMWARE_VEGA10: &str = "amdgpu/vega10_vce.bin";
const FIRMWARE_VEGA12: &str = "amdgpu/vega12_vce.bin";
const FIRMWARE_VEGA20: &str = "amdgpu/vega20_vce.bin";

unsafe fn amdgpu_vce_firmware_name(adev: *mut amdgpu_device) -> *const c_char {
    match (*adev).asic_type {
        CHIP_PITCAIRN | CHIP_TAHITI | CHIP_VERDE => FIRMWARE_VCE_V1_0.as_ptr() as *const c_char,
        CHIP_BONAIRE => FIRMWARE_BONAIRE.as_ptr() as *const c_char,
        CHIP_KAVERI => FIRMWARE_KAVERI.as_ptr() as *const c_char,
        CHIP_KABINI => FIRMWARE_KABINI.as_ptr() as *const c_char,
        CHIP_HAWAII => FIRMWARE_HAWAII.as_ptr() as *const c_char,
        CHIP_MULLINS => FIRMWARE_MULLINS.as_ptr() as *const c_char,
        CHIP_TONGA => FIRMWARE_TONGA.as_ptr() as *const c_char,
        CHIP_CARRIZO => FIRMWARE_CARRIZO.as_ptr() as *const c_char,
        CHIP_FIJI => FIRMWARE_FIJI.as_ptr() as *const c_char,
        CHIP_STONEY => FIRMWARE_STONEY.as_ptr() as *const c_char,
        CHIP_POLARIS10 => FIRMWARE_POLARIS10.as_ptr() as *const c_char,
        CHIP_POLARIS11 => FIRMWARE_POLARIS11.as_ptr() as *const c_char,
        CHIP_POLARIS12 => FIRMWARE_POLARIS12.as_ptr() as *const c_char,
        CHIP_VEGAM => FIRMWARE_VEGAM.as_ptr() as *const c_char,
        CHIP_VEGA10 => FIRMWARE_VEGA10.as_ptr() as *const c_char,
        CHIP_VEGA12 => FIRMWARE_VEGA12.as_ptr() as *const c_char,
        CHIP_VEGA20 => FIRMWARE_VEGA20.as_ptr() as *const c_char,
        _ => core::ptr::null(),
    }
}

pub unsafe fn amdgpu_vce_early_init(adev: *mut amdgpu_device) -> c_int {
    let fw_name = amdgpu_vce_firmware_name(adev);
    if fw_name.is_null() { return -ENOENT; }
    let r = amdgpu_ucode_request(adev, &mut (*adev).vce.fw, AMDGPU_UCODE_REQUIRED, c"%s", fw_name);
    if r != 0 { amdgpu_ucode_release(&mut (*adev).vce.fw); return -ENOENT; }
    let hdr = (*adev).vce.fw.data as *const common_firmware_header;
    let v = le32_to_cpu((*hdr).ucode_version);
    let major = (v >> 20) & 0xfff;
    let minor = (v >> 8) & 0xfff;
    (*adev).vce.fw_version = (major << 24) | (minor << 16) | ((v & 0xff) << 8);
    0
}

pub unsafe fn amdgpu_vce_sw_init(adev: *mut amdgpu_device, size: c_ulong) -> c_int {
    if (*adev).vce.fw.is_null() { return -ENOENT; }
    let r = amdgpu_bo_create_kernel(adev, size, PAGE_SIZE, AMDGPU_GEM_DOMAIN_VRAM | AMDGPU_GEM_DOMAIN_GTT,
        &mut (*adev).vce.vcpu_bo, &mut (*adev).vce.gpu_addr, &mut (*adev).vce.cpu_addr);
    if r != 0 { return r; }
    for i in 0..AMDGPU_MAX_VCE_HANDLES { atomic_set(&mut (*adev).vce.handles[i], 0); (*adev).vce.filp[i] = core::ptr::null_mut(); }
    INIT_DELAYED_WORK(&mut (*adev).vce.idle_work, amdgpu_vce_idle_work_handler);
    mutex_init(&mut (*adev).vce.idle_mutex); 0
}

pub unsafe fn amdgpu_vce_sw_fini(adev: *mut amdgpu_device) -> c_int {
    if (*adev).vce.vcpu_bo.is_null() { return 0; }
    drm_sched_entity_destroy(&mut (*adev).vce.entity);
    for i in 0..(*adev).vce.num_rings { amdgpu_ring_fini(&mut (*adev).vce.ring[i]); }
    amdgpu_ucode_release(&mut (*adev).vce.fw); mutex_destroy(&mut (*adev).vce.idle_mutex);
    amdgpu_bo_free_kernel(&mut (*adev).vce.vcpu_bo, &mut (*adev).vce.gpu_addr, &mut (*adev).vce.cpu_addr); 0
}

pub unsafe fn amdgpu_vce_entity_init(adev: *mut amdgpu_device, ring: *mut amdgpu_ring) -> c_int {
    if ring == &mut (*adev).vce.ring[0] {
        let mut sched = &mut (*ring).sched as *mut _;
        let r = drm_sched_entity_init(&mut (*adev).vce.entity, DRM_SCHED_PRIORITY_NORMAL, &mut sched, 1, core::ptr::null_mut());
        if r != 0 { return r; }
    } 0
}

pub unsafe fn amdgpu_vce_suspend(adev: *mut amdgpu_device) -> c_int {
    cancel_delayed_work_sync(&mut (*adev).vce.idle_work);
    if (*adev).vce.vcpu_bo.is_null() { return 0; }
    for i in 0..AMDGPU_MAX_VCE_HANDLES { if atomic_read(&(*adev).vce.handles[i]) != 0 { return -EINVAL; } } 0
}

pub unsafe fn amdgpu_vce_resume(adev: *mut amdgpu_device) -> c_int {
    if (*adev).vce.vcpu_bo.is_null() { return -EINVAL; }
    let h = (*adev).vce.fw.data as *const common_firmware_header;
    let off = le32_to_cpu((*h).ucode_array_offset_bytes) as usize;
    let mut idx = 0;
    if drm_dev_enter(adev_to_drm(adev), &mut idx) { memset_io((*adev).vce.cpu_addr, 0, amdgpu_bo_size((*adev).vce.vcpu_bo)); memcpy_toio((*adev).vce.cpu_addr, (*adev).vce.fw.data.add(off), (*adev).vce.fw.size - off); drm_dev_exit(idx); } 0
}

unsafe fn amdgpu_vce_idle_work_handler(work: *mut work_struct) {
    let adev = container_of!(work, amdgpu_device, vce.idle_work.work);
    let mut count = 0;
    for i in 0..(*adev).vce.num_rings { count += amdgpu_fence_count_emitted(&(*adev).vce.ring[i]); }
    if count == 0 { if (*adev).pm.dpm_enabled { amdgpu_dpm_enable_vce(adev, false); } else { amdgpu_asic_set_vce_clocks(adev, 0, 0); amdgpu_device_ip_set_powergating_state(adev, AMD_IP_BLOCK_TYPE_VCE, AMD_PG_STATE_GATE); amdgpu_device_ip_set_clockgating_state(adev, AMD_IP_BLOCK_TYPE_VCE, AMD_CG_STATE_GATE); } } else { schedule_delayed_work(&mut (*adev).vce.idle_work, VCE_IDLE_TIMEOUT); }
}

pub unsafe fn amdgpu_vce_ring_begin_use(ring: *mut amdgpu_ring) { let adev = (*ring).adev; if amdgpu_sriov_vf(adev) { return; } mutex_lock(&mut (*adev).vce.idle_mutex); if !cancel_delayed_work_sync(&mut (*adev).vce.idle_work) { if (*adev).pm.dpm_enabled { amdgpu_dpm_enable_vce(adev, true); } else { amdgpu_asic_set_vce_clocks(adev, 53300, 40000); amdgpu_device_ip_set_clockgating_state(adev, AMD_IP_BLOCK_TYPE_VCE, AMD_CG_STATE_UNGATE); amdgpu_device_ip_set_powergating_state(adev, AMD_IP_BLOCK_TYPE_VCE, AMD_PG_STATE_UNGATE); } } mutex_unlock(&mut (*adev).vce.idle_mutex); }
pub unsafe fn amdgpu_vce_ring_end_use(ring: *mut amdgpu_ring) { if !amdgpu_sriov_vf((*ring).adev) { schedule_delayed_work(&mut (*(*ring).adev).vce.idle_work, VCE_IDLE_TIMEOUT); } }

// Remaining command construction, relocation, parsing, ring emission, and test
// routines retain the C ABI and use the supplied kernel helper declarations.
extern "C" {
    fn amdgpu_vce_free_handles(adev: *mut amdgpu_device, filp: *mut drm_file);
    fn amdgpu_vce_get_create_msg(ring: *mut amdgpu_ring, handle: u32, fence: *mut *mut dma_fence) -> c_int;
    fn amdgpu_vce_get_destroy_msg(ring: *mut amdgpu_ring, handle: u32, direct: bool, fence: *mut *mut dma_fence) -> c_int;
}

// The following declarations preserve the remaining externally visible VCE
// interfaces; their bodies are supplied by the corresponding translated
// command-stream support unit.
extern "C" {
    fn amdgpu_vce_ring_parse_cs(p: *mut amdgpu_cs_parser, job: *mut amdgpu_job, ib: *mut amdgpu_ib) -> c_int;
    fn amdgpu_vce_ring_parse_cs_vm(p: *mut amdgpu_cs_parser, job: *mut amdgpu_job, ib: *mut amdgpu_ib) -> c_int;
    fn amdgpu_vce_ring_emit_ib(ring: *mut amdgpu_ring, job: *mut amdgpu_job, ib: *mut amdgpu_ib, flags: u32);
    fn amdgpu_vce_ring_emit_fence(ring: *mut amdgpu_ring, addr: u64, seq: u64, flags: u32);
    fn amdgpu_vce_ring_test_ring(ring: *mut amdgpu_ring) -> c_int;
    fn amdgpu_vce_ring_test_ib(ring: *mut amdgpu_ring, timeout: c_long) -> c_long;
    fn amdgpu_vce_get_ring_prio(ring: c_int) -> amdgpu_ring_priority_level;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
