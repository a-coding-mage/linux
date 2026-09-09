// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel and architecture support.

extern "C" {
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
    static mut reboot_emergency: i32;
    static mut port_cf9_safe: bool;
}

static mut REBOOT_EMERGENCY: i32 = 0;
#[no_mangle]
pub static mut port_cf9_safe_local: bool = false;

unsafe extern "C" fn set_acpi_reboot(d: *const dmi_system_id) -> i32 {
    if reboot_type != BOOT_ACPI { reboot_type = BOOT_ACPI; pr_info("%s series board detected. Selecting %s-method for reboots.\n", (*d).ident, "ACPI"); }
    0
}
unsafe extern "C" fn set_bios_reboot(d: *const dmi_system_id) -> i32 {
    if reboot_type != BOOT_BIOS { reboot_type = BOOT_BIOS; pr_info("%s series board detected. Selecting %s-method for reboots.\n", (*d).ident, "BIOS"); }
    0
}
unsafe extern "C" fn set_efi_reboot(d: *const dmi_system_id) -> i32 {
    if reboot_type != BOOT_EFI && !efi_runtime_disabled() { reboot_type = BOOT_EFI; pr_info("%s series board detected. Selecting EFI-method for reboot.\n", (*d).ident); }
    0
}

pub unsafe extern "C" fn machine_real_restart(ty: u32) -> ! {
    local_irq_disable();
    spin_lock(&mut rtc_lock);
    CMOS_WRITE(0x00, 0x8f);
    spin_unlock(&mut rtc_lock);
    load_trampoline_pgtable();
    // CONFIG_X86_32 uses jmpl and the 64-bit build uses ljmpl.
    #[cfg(target_pointer_width = "32")]
    core::arch::asm!("jmpl *{0}", in(reg) (*real_mode_header).machine_real_restart_asm, in("eax") ty);
    #[cfg(not(target_pointer_width = "32"))]
    core::arch::asm!("ljmpl *[{0}]") ;
    core::hint::unreachable_unchecked()
}

unsafe extern "C" fn set_pci_reboot(d: *const dmi_system_id) -> i32 {
    if reboot_type != BOOT_CF9_FORCE { reboot_type = BOOT_CF9_FORCE; pr_info("%s series board detected. Selecting %s-method for reboots.\n", (*d).ident, "PCI"); }
    0
}
unsafe extern "C" fn set_kbd_reboot(d: *const dmi_system_id) -> i32 {
    if reboot_type != BOOT_KBD { reboot_type = BOOT_KBD; pr_info("%s series board detected. Selecting %s-method for reboot.\n", (*d).ident, "KBD"); }
    0
}

// This is a single DMI table handling all reboot quirks.  DMI_MATCH entries
// and the terminating empty entry are supplied by the kernel DMI definitions.
static reboot_dmi_table: [dmi_system_id; 1] = [dmi_system_id { callback: None, ident: "", matches: [] }];

unsafe extern "C" fn reboot_init() -> i32 {
    if !reboot_default { return 0; }
    let rv = dmi_check_system(reboot_dmi_table.as_ptr());
    if rv == 0 && efi_reboot_required() && !efi_runtime_disabled() { reboot_type = BOOT_EFI; }
    0
}

unsafe fn kb_wait() {
    for _i in 0..0x10000 {
        if (inb(0x64) & 0x02) == 0 { break; }
        udelay(2);
    }
}

#[cfg(feature = "CONFIG_KVM_X86")]
unsafe fn emergency_reboot_disable_virtualization() {
    local_irq_disable();
    if !x86_virt_emergency_disable_virtualization_cpu() { nmi_shootdown_cpus_on_restart(); }
}
#[cfg(not(feature = "CONFIG_KVM_X86"))]
unsafe fn emergency_reboot_disable_virtualization() {}

#[no_mangle]
pub unsafe extern "C" fn mach_reboot_fixups() {}

unsafe fn native_machine_emergency_restart() -> ! {
    let mut attempt = 0;
    let orig_reboot_type = reboot_type;
    if REBOOT_EMERGENCY != 0 { emergency_reboot_disable_virtualization(); }
    tboot_shutdown(TB_SHUTDOWN_REBOOT);
    let mode: u16 = if reboot_mode == REBOOT_WARM { 0x1234 } else { 0 };
    *(__va(0x472) as *mut u16) = mode;
    if efi_capsule_pending(core::ptr::null_mut()) { pr_info("EFI capsule is pending, forcing EFI reboot.\n"); reboot_type = BOOT_EFI; }
    loop {
        match reboot_type {
            BOOT_ACPI => { acpi_reboot(); reboot_type = BOOT_KBD; }
            BOOT_KBD => {
                mach_reboot_fixups();
                for _ in 0..10 { kb_wait(); udelay(50); outb(0xfe, 0x64); udelay(50); }
                if attempt == 0 && orig_reboot_type == BOOT_ACPI { attempt = 1; reboot_type = BOOT_ACPI; } else { reboot_type = BOOT_EFI; }
            }
            BOOT_EFI => { efi_reboot(reboot_mode, core::ptr::null_mut()); reboot_type = BOOT_BIOS; }
            BOOT_BIOS => { machine_real_restart(MRR_BIOS); reboot_type = BOOT_CF9_SAFE; }
            BOOT_CF9_FORCE => { port_cf9_safe_local = true; reboot_type = BOOT_CF9_SAFE; }
            BOOT_CF9_SAFE => {
                if port_cf9_safe_local { let code: u8 = if reboot_mode == REBOOT_WARM { 0x06 } else { 0x0e }; let cf9 = inb(0xcf9) & !code; outb(cf9 | 2, 0xcf9); udelay(50); outb(cf9 | code, 0xcf9); udelay(50); }
                reboot_type = BOOT_TRIPLE;
            }
            BOOT_TRIPLE => { idt_invalidate(); core::arch::asm!("int3"); reboot_type = BOOT_KBD; }
            _ => reboot_type = BOOT_KBD,
        }
    }
}

pub unsafe extern "C" fn native_machine_shutdown() {
    if kexec_in_progress { x86_platform.guest.enc_kexec_begin(); }
    clear_IO_APIC();
    local_irq_disable();
    stop_other_cpus();
    lapic_shutdown();
    restore_boot_irq_mode();
    hpet_disable();
    x86_platform.iommu_shutdown();
    if kexec_in_progress { x86_platform.guest.enc_kexec_finish(); }
}

unsafe fn __machine_emergency_restart(emergency: i32) { REBOOT_EMERGENCY = emergency; machine_ops.emergency_restart(); }
unsafe extern "C" fn native_machine_restart(command: *mut i8) { pr_notice("machine restart\n"); if !reboot_force { machine_shutdown(); } do_kernel_restart(command); __machine_emergency_restart(0); }
unsafe extern "C" fn native_machine_halt() { machine_shutdown(); tboot_shutdown(TB_SHUTDOWN_HALT); stop_this_cpu(core::ptr::null_mut()); }
unsafe extern "C" fn native_machine_power_off() { if kernel_can_power_off() { if !reboot_force { machine_shutdown(); } do_kernel_power_off(); } tboot_shutdown(TB_SHUTDOWN_HALT); }

#[no_mangle] pub unsafe extern "C" fn machine_power_off() { machine_ops.power_off(); }
#[no_mangle] pub unsafe extern "C" fn machine_shutdown() { machine_ops.shutdown(); }
#[no_mangle] pub unsafe extern "C" fn machine_emergency_restart() { __machine_emergency_restart(1); }
#[no_mangle] pub unsafe extern "C" fn machine_restart(cmd: *mut i8) { machine_ops.restart(cmd); }
#[no_mangle] pub unsafe extern "C" fn machine_halt() { machine_ops.halt(); }

#[no_mangle] pub static mut crashing_cpu: i32 = -1;

#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" fn crash_nmi_callback(val: u32, regs: *mut pt_regs) -> i32 {
    let cpu = raw_smp_processor_id();
    if cpu == crashing_cpu { return NMI_HANDLED; }
    local_irq_disable();
    if let Some(cb) = shootdown_callback { cb(cpu, regs); }
    x86_virt_emergency_disable_virtualization_cpu();
    atomic_dec(&mut waiting_for_crash_ipi);
    if let Some(stop) = smp_ops.stop_this_cpu { stop(); BUG(); }
    halt();
    loop { cpu_relax(); }
}

#[cfg(feature = "CONFIG_SMP")]
pub unsafe extern "C" fn nmi_shootdown_cpus(callback: nmi_shootdown_cb) {
    local_irq_disable();
    if WARN_ON_ONCE(crash_ipi_issued) { return; }
    crashing_cpu = smp_processor_id(); shootdown_callback = callback;
    atomic_set(&mut waiting_for_crash_ipi, num_online_cpus() - 1);
    set_emergency_nmi_handler(NMI_LOCAL, crash_nmi_callback);
    apic_send_IPI_allbutself(NMI_VECTOR); WRITE_ONCE(&mut crash_ipi_issued, 1);
    let mut msecs = 1000;
    while atomic_read(&waiting_for_crash_ipi) > 0 && msecs != 0 { mdelay(1); msecs -= 1; }
}
#[cfg(feature = "CONFIG_SMP")]
unsafe fn nmi_shootdown_cpus_on_restart() { if crash_ipi_issued == 0 { nmi_shootdown_cpus(None); } }
#[cfg(not(feature = "CONFIG_SMP"))]
pub unsafe extern "C" fn nmi_shootdown_cpus(_callback: nmi_shootdown_cb) {}
#[cfg(not(feature = "CONFIG_SMP"))]
unsafe fn nmi_shootdown_cpus_on_restart() {}

pub unsafe extern "C" fn run_crash_ipi_callback(regs: *mut pt_regs) { if crash_ipi_issued != 0 { crash_nmi_callback(0, regs); } }
pub unsafe extern "C" fn nmi_panic_self_stop(regs: *mut pt_regs) -> ! { loop { run_crash_ipi_callback(regs); cpu_relax(); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
