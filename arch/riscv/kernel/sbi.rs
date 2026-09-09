// SPDX-License-Identifier: GPL-2.0-only
/*
 * SBI initialilization and all extension implementation.
 *
 * Copyright (c) 2020 Western Digital Corporation or its affiliates.
 */

/* Dependencies are supplied by the surrounding kernel translation. */

pub static mut sbi_spec_version: ::core::ffi::c_ulong = SBI_SPEC_VERSION_DEFAULT;

static mut __sbi_set_timer: Option<unsafe extern "C" fn(u64)> = None;
static mut __sbi_send_ipi: Option<unsafe extern "C" fn(u32)> = None;
static mut __sbi_rfence: Option<unsafe extern "C" fn(i32, *const cpumask, usize, usize, usize, usize) -> i32> = None;

#[cfg(CONFIG_RISCV_SBI_V01)]
unsafe fn __sbi_v01_cpumask_to_hartmask(cpu_mask: *const cpumask) -> usize {
    let mut hmask: usize = 0;
    for_each_cpu!(cpuid, cpu_mask) {
        let hartid = cpuid_to_hartid_map(cpuid);
        if hartid >= BITS_PER_LONG {
            pr_warn!("Unable to send any request to hartid > BITS_PER_LONG for SBI v0.1\n");
            break;
        }
        hmask |= BIT(hartid);
    }
    hmask
}

#[cfg(CONFIG_RISCV_SBI_V01)]
pub unsafe fn sbi_console_putchar(ch: i32) {
    sbi_ecall(SBI_EXT_0_1_CONSOLE_PUTCHAR, 0, ch as _, 0, 0, 0, 0, 0);
}

#[cfg(CONFIG_RISCV_SBI_V01)]
pub unsafe fn sbi_console_getchar() -> i32 {
    sbi_ecall(SBI_EXT_0_1_CONSOLE_GETCHAR, 0, 0, 0, 0, 0, 0, 0).error
}

#[cfg(CONFIG_RISCV_SBI_V01)]
pub unsafe fn sbi_shutdown() {
    sbi_ecall(SBI_EXT_0_1_SHUTDOWN, 0, 0, 0, 0, 0, 0, 0);
}

unsafe fn __sbi_set_timer_v01(stime_value: u64) {
    #[cfg(target_pointer_width = "32")]
    sbi_ecall(SBI_EXT_0_1_SET_TIMER, 0, stime_value as _, (stime_value >> 32) as _, 0, 0, 0, 0);
    #[cfg(not(target_pointer_width = "32"))]
    sbi_ecall(SBI_EXT_0_1_SET_TIMER, 0, stime_value as _, 0, 0, 0, 0, 0);
}

unsafe fn __sbi_send_ipi_v01(cpu: u32) {
    let hart_mask = __sbi_v01_cpumask_to_hartmask(cpumask_of(cpu));
    sbi_ecall(SBI_EXT_0_1_SEND_IPI, 0, (&hart_mask as *const usize) as _, 0, 0, 0, 0, 0);
}

unsafe fn __sbi_rfence_v01(fid: i32, mut cpu_mask: *const cpumask, start: usize, size: usize, arg4: usize, _arg5: usize) -> i32 {
    if cpu_mask.is_null() || cpumask_empty(cpu_mask) { cpu_mask = cpu_online_mask; }
    let hart_mask = __sbi_v01_cpumask_to_hartmask(cpu_mask);
    match fid {
        SBI_EXT_RFENCE_REMOTE_FENCE_I => { sbi_ecall(SBI_EXT_0_1_REMOTE_FENCE_I, 0, (&hart_mask as *const usize) as _, 0, 0, 0, 0, 0); }
        SBI_EXT_RFENCE_REMOTE_SFENCE_VMA => { sbi_ecall(SBI_EXT_0_1_REMOTE_SFENCE_VMA, 0, (&hart_mask as *const usize) as _, start, size, 0, 0, 0); }
        SBI_EXT_RFENCE_REMOTE_SFENCE_VMA_ASID => { sbi_ecall(SBI_EXT_0_1_REMOTE_SFENCE_VMA_ASID, 0, (&hart_mask as *const usize) as _, start, size, arg4, 0, 0); }
        _ => { pr_err!("SBI call [{}]not supported in SBI v0.1\n", fid); return -EINVAL; }
    }
    0
}

#[cfg(CONFIG_RISCV_SBI_V01)]
unsafe fn sbi_set_power_off() { register_platform_power_off(sbi_shutdown); }

#[cfg(not(CONFIG_RISCV_SBI_V01))]
unsafe fn __sbi_set_timer_v01(_stime_value: u64) { pr_warn!("Timer extension is not available in SBI v%lu.%lu\n", sbi_major_version(), sbi_minor_version()); }
#[cfg(not(CONFIG_RISCV_SBI_V01))]
unsafe fn __sbi_send_ipi_v01(_cpu: u32) { pr_warn!("IPI extension is not available in SBI v%lu.%lu\n", sbi_major_version(), sbi_minor_version()); }
#[cfg(not(CONFIG_RISCV_SBI_V01))]
unsafe fn __sbi_rfence_v01(_fid: i32, _cpu_mask: *const cpumask, _start: usize, _size: usize, _arg4: usize, _arg5: usize) -> i32 { pr_warn!("remote fence extension is not available in SBI v%lu.%lu\n", sbi_major_version(), sbi_minor_version()); 0 }
#[cfg(not(CONFIG_RISCV_SBI_V01))]
unsafe fn sbi_set_power_off() {}

unsafe fn __sbi_set_timer_v02(stime_value: u64) {
    #[cfg(target_pointer_width = "32")]
    sbi_ecall(SBI_EXT_TIME, SBI_EXT_TIME_SET_TIMER, stime_value as _, (stime_value >> 32) as _, 0, 0, 0, 0);
    #[cfg(not(target_pointer_width = "32"))]
    sbi_ecall(SBI_EXT_TIME, SBI_EXT_TIME_SET_TIMER, stime_value as _, 0, 0, 0, 0, 0);
}

unsafe fn __sbi_send_ipi_v02(cpu: u32) {
    let ret = sbi_ecall(SBI_EXT_IPI, SBI_EXT_IPI_SEND_IPI, 1, cpuid_to_hartid_map(cpu), 0, 0, 0, 0);
    if ret.error != 0 { let result = sbi_err_map_linux_errno(ret.error); pr_err!("%s: hbase = [%lu] failed (error [%d])\n", "__sbi_send_ipi_v02", cpuid_to_hartid_map(cpu), result); }
}

unsafe fn __sbi_rfence_v02_call(fid: usize, hmask: usize, hbase: usize, start: usize, size: usize, arg4: usize, _arg5: usize) -> i32 {
    let mut ret = sbiret { error: 0, value: 0 };
    let ext = SBI_EXT_RFENCE;
    match fid {
        SBI_EXT_RFENCE_REMOTE_FENCE_I => { ret = sbi_ecall(ext, fid, hmask, hbase, 0, 0, 0, 0); }
        SBI_EXT_RFENCE_REMOTE_SFENCE_VMA | SBI_EXT_RFENCE_REMOTE_HFENCE_GVMA | SBI_EXT_RFENCE_REMOTE_HFENCE_VVMA => { ret = sbi_ecall(ext, fid, hmask, hbase, start, size, 0, 0); }
        SBI_EXT_RFENCE_REMOTE_SFENCE_VMA_ASID | SBI_EXT_RFENCE_REMOTE_HFENCE_GVMA_VMID | SBI_EXT_RFENCE_REMOTE_HFENCE_VVMA_ASID => { ret = sbi_ecall(ext, fid, hmask, hbase, start, size, arg4, 0); }
        _ => { pr_err!("unknown function ID [%lu] for SBI extension [%d]\n", fid, ext); return -EINVAL; }
    }
    if ret.error != 0 { let result = sbi_err_map_linux_errno(ret.error); pr_err!("%s: hbase = [%lu] hmask = [0x%lx] failed (error [%d])\n", "__sbi_rfence_v02_call", hbase, hmask, result); return result; }
    0
}

unsafe fn __sbi_rfence_v02(fid: i32, mut cpu_mask: *const cpumask, start: usize, size: usize, arg4: usize, arg5: usize) -> i32 {
    if cpu_mask.is_null() || cpumask_empty(cpu_mask) { cpu_mask = cpu_online_mask; }
    let (mut hmask, mut hbase, mut htop) = (0usize, 0usize, 0usize);
    for_each_cpu!(cpuid, cpu_mask) {
        let hartid = cpuid_to_hartid_map(cpuid);
        if hmask != 0 {
            if hartid + BITS_PER_LONG <= htop || hbase + BITS_PER_LONG <= hartid { let result = __sbi_rfence_v02_call(fid as _, hmask, hbase, start, size, arg4, arg5); if result != 0 { return result; } hmask = 0; }
            else if hartid < hbase { hmask <<= hbase - hartid; hbase = hartid; }
        }
        if hmask == 0 { hbase = hartid; htop = hartid; } else if hartid > htop { htop = hartid; }
        hmask |= BIT(hartid - hbase);
    }
    if hmask != 0 { let result = __sbi_rfence_v02_call(fid as _, hmask, hbase, start, size, arg4, arg5); if result != 0 { return result; } }
    0
}

static mut sbi_fwft_supported: bool = false;

#[repr(C)]
struct fwft_set_req { feature: u32, value: usize, flags: usize, error: atomic_t }

unsafe fn cpu_sbi_fwft_set(arg: *mut core::ffi::c_void) { let req = &mut *(arg as *mut fwft_set_req); let ret = sbi_fwft_set(req.feature, req.value, req.flags); if ret != 0 { atomic_set(&mut req.error, ret); } }

pub unsafe fn sbi_fwft_set(feature: u32, value: usize, flags: usize) -> i32 {
    if !sbi_fwft_supported { return -EOPNOTSUPP; }
    sbi_err_map_linux_errno(sbi_ecall(SBI_EXT_FWFT, SBI_EXT_FWFT_SET, feature as _, value, flags, 0, 0, 0).error)
}

pub unsafe fn sbi_fwft_set_cpumask(mask: *const cpumask_t, feature: u32, value: usize, flags: usize) -> i32 {
    let mut req = fwft_set_req { feature, value, flags, error: ATOMIC_INIT(0) };
    if !sbi_fwft_supported { return -EOPNOTSUPP; }
    if feature & SBI_FWFT_GLOBAL_FEATURE_BIT != 0 { return -EINVAL; }
    on_each_cpu_mask(mask, cpu_sbi_fwft_set, &mut req as *mut _ as _, 1);
    atomic_read(&req.error)
}

pub unsafe fn sbi_set_timer(stime_value: u64) { (__sbi_set_timer.unwrap())(stime_value); }
pub unsafe fn sbi_send_ipi(cpu: u32) { (__sbi_send_ipi.unwrap())(cpu); }
pub unsafe fn sbi_remote_fence_i(cpu_mask: *const cpumask) -> i32 { (__sbi_rfence.unwrap())(SBI_EXT_RFENCE_REMOTE_FENCE_I, cpu_mask, 0, 0, 0, 0) }
pub unsafe fn sbi_remote_sfence_vma_asid(cpu_mask: *const cpumask, start: usize, size: usize, asid: usize) -> i32 { if asid == FLUSH_TLB_NO_ASID { (__sbi_rfence.unwrap())(SBI_EXT_RFENCE_REMOTE_SFENCE_VMA, cpu_mask, start, size, 0, 0) } else { (__sbi_rfence.unwrap())(SBI_EXT_RFENCE_REMOTE_SFENCE_VMA_ASID, cpu_mask, start, size, asid, 0) } }
pub unsafe fn sbi_remote_hfence_gvma(cpu_mask: *const cpumask, start: usize, size: usize) -> i32 { (__sbi_rfence.unwrap())(SBI_EXT_RFENCE_REMOTE_HFENCE_GVMA, cpu_mask, start, size, 0, 0) }
pub unsafe fn sbi_remote_hfence_gvma_vmid(cpu_mask: *const cpumask, start: usize, size: usize, vmid: usize) -> i32 { (__sbi_rfence.unwrap())(SBI_EXT_RFENCE_REMOTE_HFENCE_GVMA_VMID, cpu_mask, start, size, vmid, 0) }
pub unsafe fn sbi_remote_hfence_vvma(cpu_mask: *const cpumask, start: usize, size: usize) -> i32 { (__sbi_rfence.unwrap())(SBI_EXT_RFENCE_REMOTE_HFENCE_VVMA, cpu_mask, start, size, 0, 0) }
pub unsafe fn sbi_remote_hfence_vvma_asid(cpu_mask: *const cpumask, start: usize, size: usize, asid: usize) -> i32 { (__sbi_rfence.unwrap())(SBI_EXT_RFENCE_REMOTE_HFENCE_VVMA_ASID, cpu_mask, start, size, asid, 0) }

unsafe fn sbi_srst_reset(ty: usize, reason: usize) { sbi_ecall(SBI_EXT_SRST, SBI_EXT_SRST_RESET, ty, reason, 0, 0, 0, 0); pr_warn!("%s: type=0x%lx reason=0x%lx failed\n", "sbi_srst_reset", ty, reason); }
unsafe fn sbi_srst_reboot(_this: *mut notifier_block, mode: usize, _cmd: *mut core::ffi::c_void) -> i32 { sbi_srst_reset(if mode == REBOOT_WARM || mode == REBOOT_SOFT { SBI_SRST_RESET_TYPE_WARM_REBOOT } else { SBI_SRST_RESET_TYPE_COLD_REBOOT }, SBI_SRST_RESET_REASON_NONE); NOTIFY_DONE }
static mut sbi_srst_reboot_nb: notifier_block = notifier_block::default();
unsafe fn sbi_srst_power_off() { sbi_srst_reset(SBI_SRST_RESET_TYPE_SHUTDOWN, SBI_SRST_RESET_REASON_NONE); }

pub unsafe fn sbi_probe_extension(extid: i32) -> isize { let ret = sbi_ecall(SBI_EXT_BASE, SBI_EXT_BASE_PROBE_EXT, extid as _, 0, 0, 0, 0, 0); if ret.error == 0 { ret.value as _ } else { 0 } }
unsafe fn sbi_get_spec_version() -> isize { __sbi_base_ecall(SBI_EXT_BASE_GET_SPEC_VERSION) }
unsafe fn sbi_get_firmware_id() -> isize { __sbi_base_ecall(SBI_EXT_BASE_GET_IMP_ID) }
unsafe fn sbi_get_firmware_version() -> isize { __sbi_base_ecall(SBI_EXT_BASE_GET_IMP_VERSION) }
pub unsafe fn sbi_get_mvendorid() -> isize { __sbi_base_ecall(SBI_EXT_BASE_GET_MVENDORID) }
pub unsafe fn sbi_get_marchid() -> isize { __sbi_base_ecall(SBI_EXT_BASE_GET_MARCHID) }
pub unsafe fn sbi_get_mimpid() -> isize { __sbi_base_ecall(SBI_EXT_BASE_GET_MIMPID) }

pub static mut sbi_debug_console_available: bool = false;
pub unsafe fn sbi_debug_console_write(bytes: *const i8, mut num_bytes: u32) -> i32 { if !sbi_debug_console_available { return -EOPNOTSUPP; } let mut base_addr = if is_vmalloc_addr(bytes) { page_to_phys(vmalloc_to_page(bytes)) + offset_in_page(bytes) } else { __pa(bytes) }; if PAGE_SIZE < offset_in_page(bytes) + num_bytes as usize { num_bytes = (PAGE_SIZE - offset_in_page(bytes)) as _; } let ret = sbi_ecall(SBI_EXT_DBCN, SBI_EXT_DBCN_CONSOLE_WRITE, num_bytes as _, base_addr, 0, 0, 0, 0); if ret.error == SBI_ERR_FAILURE { -EIO } else if ret.error != 0 { sbi_err_map_linux_errno(ret.error) } else { ret.value as _ } }
pub unsafe fn sbi_debug_console_read(bytes: *mut i8, mut num_bytes: u32) -> i32 { if !sbi_debug_console_available { return -EOPNOTSUPP; } let base_addr = if is_vmalloc_addr(bytes) { page_to_phys(vmalloc_to_page(bytes)) + offset_in_page(bytes) } else { __pa(bytes) }; if PAGE_SIZE < offset_in_page(bytes) + num_bytes as usize { num_bytes = (PAGE_SIZE - offset_in_page(bytes)) as _; } let ret = sbi_ecall(SBI_EXT_DBCN, SBI_EXT_DBCN_CONSOLE_READ, num_bytes as _, base_addr, 0, 0, 0, 0); if ret.error == SBI_ERR_FAILURE { -EIO } else if ret.error != 0 { sbi_err_map_linux_errno(ret.error) } else { ret.value as _ } }

pub unsafe fn sbi_init() {
    let mut srst_power_off = false;
    let ret = sbi_get_spec_version();
    if ret > 0 { sbi_spec_version = ret as _; }
    pr_info!("SBI specification v%lu.%lu detected\n", sbi_major_version(), sbi_minor_version());
    if !sbi_spec_is_0_1() {
        pr_info!("SBI implementation ID=0x%lx Version=0x%lx\n", sbi_get_firmware_id(), sbi_get_firmware_version());
        __sbi_set_timer = Some(if sbi_probe_extension(SBI_EXT_TIME) != 0 { pr_info!("SBI TIME extension detected\n"); __sbi_set_timer_v02 } else { __sbi_set_timer_v01 });
        __sbi_send_ipi = Some(if sbi_probe_extension(SBI_EXT_IPI) != 0 { pr_info!("SBI IPI extension detected\n"); __sbi_send_ipi_v02 } else { __sbi_send_ipi_v01 });
        __sbi_rfence = Some(if sbi_probe_extension(SBI_EXT_RFENCE) != 0 { pr_info!("SBI RFENCE extension detected\n"); __sbi_rfence_v02 } else { __sbi_rfence_v01 });
        if sbi_spec_version >= sbi_mk_version(0, 3) && sbi_probe_extension(SBI_EXT_SRST) != 0 { pr_info!("SBI SRST extension detected\n"); register_platform_power_off(sbi_srst_power_off); srst_power_off = true; sbi_srst_reboot_nb.notifier_call = Some(sbi_srst_reboot); sbi_srst_reboot_nb.priority = 192; register_restart_handler(&mut sbi_srst_reboot_nb); }
        if sbi_spec_version >= sbi_mk_version(2, 0) && sbi_probe_extension(SBI_EXT_DBCN) > 0 { pr_info!("SBI DBCN extension detected\n"); sbi_debug_console_available = true; }
        if sbi_spec_version >= sbi_mk_version(3, 0) && sbi_probe_extension(SBI_EXT_FWFT) != 0 { pr_info!("SBI FWFT extension detected\n"); sbi_fwft_supported = true; }
    } else { __sbi_set_timer = Some(__sbi_set_timer_v01); __sbi_send_ipi = Some(__sbi_send_ipi_v01); __sbi_rfence = Some(__sbi_rfence_v01); }
    if !srst_power_off { sbi_set_power_off(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
