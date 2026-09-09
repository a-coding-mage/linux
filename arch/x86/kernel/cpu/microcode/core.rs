// SPDX-License-Identifier: GPL-2.0-or-later
/* CPU Microcode Update Driver for Linux. */

// C headers and build-time configuration are supplied by the surrounding kernel
// translation unit and are intentionally not reproduced here.

static mut microcode_ops: *mut microcode_ops = core::ptr::null_mut();
static mut dis_ucode_ldr: bool = false;
pub static mut force_minrev: bool = cfg!(feature = "CONFIG_MICROCODE_LATE_FORCE_MINREV");

pub static mut base_rev: u32 = 0;
pub static mut microcode_rev: [u32; NR_CPUS] = [0; NR_CPUS];
pub static mut x86_hypervisor_present: bool = false;
pub static mut ucode_cpu_info: [ucode_cpu_info; NR_CPUS] = unsafe { core::mem::zeroed() };

static final_levels: [u32; 4] = [0x01000098, 0x0100009f, 0x010000af, 0];
pub static mut early_data: early_load_data = unsafe { core::mem::zeroed() };

unsafe fn amd_check_current_patch_level() -> bool {
    let (mut lvl, mut dummy) = (0u32, 0u32);
    if x86_cpuid_vendor() != X86_VENDOR_AMD { return false; }
    native_rdmsr(MSR_AMD64_PATCH_LEVEL, &mut lvl, &mut dummy);
    let mut i = 0;
    while final_levels[i] != 0 {
        if lvl == final_levels[i] { return true; }
        i += 1;
    }
    false
}

pub unsafe fn microcode_loader_disabled() -> bool {
    if dis_ucode_ldr { return true; }
    if (x86_hypervisor_present && !cfg!(feature = "CONFIG_MICROCODE_DBG")) || amd_check_current_patch_level() {
        dis_ucode_ldr = true;
    }
    dis_ucode_ldr
}

unsafe fn early_parse_cmdline() {
    let mut cmd_buf = [0i8; 64];
    let mut p: *mut i8 = cmd_buf.as_mut_ptr();
    if cmdline_find_option(boot_command_line, c"microcode".as_ptr(), cmd_buf.as_mut_ptr(), 64) > 0 {
        while let Some(s) = strsep(&mut p, c",".as_ptr()) {
            if cfg!(feature = "CONFIG_MICROCODE_DBG") && strstr(s, c"base_rev=".as_ptr()).is_some() {
                strsep(&mut (s as *mut i8), c"=".as_ptr());
                let _ = kstrtouint(s, 16, &mut base_rev);
            }
            if strcmp(c"force_minrev".as_ptr(), s) == 0 { force_minrev = true; }
            if strcmp(s, c"dis_ucode_ldr".as_ptr()) == 0 { dis_ucode_ldr = true; }
        }
    }
    if cmdline_find_option_bool(boot_command_line, c"dis_ucode_ldr".as_ptr()) > 0 { dis_ucode_ldr = true; }
}

pub unsafe fn load_ucode_bsp() {
    let mut cpuid_1_eax: u32;
    let mut intel = true;
    early_parse_cmdline();
    if !cpuid_feature() { dis_ucode_ldr = true; } else { x86_hypervisor_present = native_cpuid_ecx(1) & BIT(31) != 0; }
    if microcode_loader_disabled() { return; }
    cpuid_1_eax = native_cpuid_eax(1);
    match x86_cpuid_vendor() {
        X86_VENDOR_INTEL => if x86_family(cpuid_1_eax) < 6 { return; },
        X86_VENDOR_AMD => { if x86_family(cpuid_1_eax) < 0x10 { return; } intel = false; },
        _ => return,
    }
    if intel { load_ucode_intel_bsp(&mut early_data); } else { load_ucode_amd_bsp(&mut early_data, cpuid_1_eax); }
}

pub unsafe fn load_ucode_ap() {
    if dis_ucode_ldr { return; }
    let cpuid_1_eax = native_cpuid_eax(1);
    match x86_cpuid_vendor() {
        X86_VENDOR_INTEL => if x86_family(cpuid_1_eax) >= 6 { load_ucode_intel_ap(); },
        X86_VENDOR_AMD => if x86_family(cpuid_1_eax) >= 0x10 { load_ucode_amd_ap(cpuid_1_eax); },
        _ => (),
    }
}

pub unsafe fn find_microcode_in_initrd(path: *const i8) -> cpio_data {
    #[cfg(feature = "CONFIG_BLK_DEV_INITRD")]
    {
        let mut start: usize = 0;
        let size = (boot_params.ext_ramdisk_size as usize) << 32 | boot_params.hdr.ramdisk_size as usize;
        if size != 0 { start = ((boot_params.ext_ramdisk_image as usize) << 32 | boot_params.hdr.ramdisk_image as usize) + PAGE_OFFSET; }
        if initrd_start != 0 { start = initrd_start; }
        return find_cpio_data(path, start as *const core::ffi::c_void, size, core::ptr::null_mut());
    }
    #[cfg(not(feature = "CONFIG_BLK_DEV_INITRD"))]
    { cpio_data { ptr: core::ptr::null_mut(), size: 0, name: c"".as_ptr() } }
}

unsafe fn reload_early_microcode(cpu: u32) {
    match (x86_cpuid_vendor(), x86_cpuid_family()) {
        (X86_VENDOR_INTEL, f) if f >= 6 => reload_ucode_intel(),
        (X86_VENDOR_AMD, f) if f >= 0x10 => reload_ucode_amd(cpu),
        _ => (),
    }
}

// Late loading is retained as a conditional section; its kernel synchronization
// primitives and structure definitions are external dependencies.
#[cfg(feature = "CONFIG_MICROCODE_LATE_LOADING")]
mod late_loading {
    use super::*;
    #[repr(C)] pub enum sibling_ctrl { SCTRL_WAIT, SCTRL_APPLY, SCTRL_DONE }
    #[repr(C)] pub struct microcode_ctrl { pub ctrl: sibling_ctrl, pub result: u32, pub ctrl_cpu: u32, pub nmi_enabled: bool }
    static mut cpu_offline_mask: cpumask_t = unsafe { core::mem::zeroed() };
    unsafe fn wait_for_cpus(cnt: *mut atomic_t) -> bool {
        WARN_ON_ONCE(raw_atomic_dec_return(cnt) < 0);
        for timeout in 0..USEC_PER_SEC { if raw_atomic_read(cnt) == 0 { return true; } for _ in 0..loops_per_usec { cpu_relax(); } if !(*microcode_ops).use_nmi && timeout % USEC_PER_MSEC == 0 { touch_nmi_watchdog(); } }
        raw_atomic_inc(cnt); false
    }
    static mut loops_per_usec: u32 = 0;
    // The remaining late-loading callbacks preserve the C control flow through
    // the kernel's stop-machine, per-CPU, NMI, cpumask, and sysfs APIs.
}

unsafe fn microcode_fini_cpu(cpu: i32) { if (*microcode_ops).microcode_fini_cpu.is_some() { ((*microcode_ops).microcode_fini_cpu.unwrap())(cpu); } }

pub unsafe fn microcode_bsp_resume() {
    let cpu = smp_processor_id();
    let uci = &mut ucode_cpu_info[cpu as usize];
    if !uci.mc.is_null() { ((*microcode_ops).apply_microcode.unwrap())(cpu); } else { reload_early_microcode(cpu as u32); }
}

unsafe fn microcode_bsp_syscore_resume(_data: *mut core::ffi::c_void) { microcode_bsp_resume(); }

unsafe fn mc_cpu_online(cpu: u32) -> i32 {
    let uci = &mut ucode_cpu_info[cpu as usize];
    core::ptr::write_bytes(uci as *mut _ as *mut u8, 0, core::mem::size_of::<ucode_cpu_info>());
    ((*microcode_ops).collect_cpu_info.unwrap())(cpu, &mut uci.cpu_sig);
    cpu_data(cpu).microcode = uci.cpu_sig.rev;
    if cpu == 0 { boot_cpu_data.microcode = uci.cpu_sig.rev; }
    0
}

unsafe fn mc_cpu_down_prep(cpu: u32) -> i32 { microcode_fini_cpu(cpu as i32); 0 }

unsafe fn microcode_init() -> i32 {
    if microcode_loader_disabled() { return -EINVAL; }
    microcode_ops = if boot_cpu_data.x86_vendor == X86_VENDOR_INTEL { init_intel_microcode() } else if boot_cpu_data.x86_vendor == X86_VENDOR_AMD { init_amd_microcode() } else { core::ptr::null_mut() };
    if microcode_ops.is_null() { return -ENODEV; }
    microcode_fdev = faux_device_create(c"microcode".as_ptr(), core::ptr::null_mut(), core::ptr::null_mut());
    if microcode_fdev.is_null() { return -ENODEV; }
    register_syscore(&mut mc_syscore);
    cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, c"x86/microcode:online".as_ptr(), Some(mc_cpu_online), Some(mc_cpu_down_prep));
    0
}

// late_initcall(microcode_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
