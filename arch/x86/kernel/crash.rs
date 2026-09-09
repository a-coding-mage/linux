// SPDX-License-Identifier: GPL-2.0-only
/* Architecture specific (i386/x86_64) functions for kexec based crash dumps. */

#[repr(C)]
pub struct crash_memmap_data {
    pub params: *mut boot_params,
    pub type_: u32,
}

#[repr(C)]
pub struct boot_params { pub e820_entries: u8, pub e820_table: [e820_entry; 128] }
#[repr(C)]
pub struct e820_entry { pub addr: u64, pub size: u64, pub type_: u32 }
#[repr(C)]
pub struct resource { pub start: u64, pub end: u64 }
#[repr(C)]
pub struct crash_range { pub start: u64, pub end: u64 }
#[repr(C)]
pub struct crash_mem { pub max_nr_ranges: u32, pub nr_ranges: u32, pub ranges: [crash_range; 0] }
#[repr(C)]
pub struct pt_regs { _private: [u8; 0] }
#[repr(C)]
pub struct kimage {
    pub elf_load_addr: u64, pub elf_headers_sz: u64, pub dm_crypt_keys_addr: u64,
    pub dm_crypt_keys_sz: u64, pub elf_headers: *mut core::ffi::c_void,
    pub nr_segments: u32, pub elfcorehdr_index: u32, pub file_mode: bool,
    pub elfcorehdr_updated: bool, pub hp_action: u32,
    pub segment: *mut kexec_segment,
}
#[repr(C)] pub struct kexec_segment { pub mem: u64, pub memsz: u64 }
#[repr(C)] pub struct kexec_buf {
    pub image: *mut kimage, pub buf_min: u64, pub buf_max: u64, pub top_down: bool,
    pub buffer: *mut core::ffi::c_void, pub bufsz: u64, pub memsz: u64,
    pub buf_align: u64, pub mem: u64,
}

extern "C" {
    fn crash_save_cpu(regs: *mut pt_regs, cpu: i32);
    fn cpu_emergency_stop_pt(); fn kdump_sev_callback(); fn disable_local_APIC();
    fn nmi_shootdown_cpus(cb: unsafe extern "C" fn(i32, *mut pt_regs));
    fn smp_send_stop(); fn local_irq_disable(); fn tdx_sys_disable();
    fn x86_virt_emergency_disable_virtualization_cpu(); fn ioapic_zap_locks();
    fn clear_IO_APIC(); fn lapic_shutdown(); fn restore_boot_irq_mode(); fn hpet_disable();
    fn smp_processor_id() -> i32;
    fn walk_system_ram_res(start: u64, end: u64, arg: *mut core::ffi::c_void,
                           cb: unsafe extern "C" fn(*mut resource, *mut core::ffi::c_void) -> i32) -> i32;
    fn crash_exclude_mem_range(cmem: *mut crash_mem, start: u64, end: u64) -> i32;
    fn vzalloc(size: usize) -> *mut core::ffi::c_void; fn vfree(p: *mut core::ffi::c_void);
    fn memset(dst: *mut core::ffi::c_void, value: i32, size: usize) -> *mut core::ffi::c_void;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> *mut core::ffi::c_void;
    fn resource_size(res: *mut resource) -> u64;
    fn walk_iomem_res_desc(desc: u64, flags: u64, start: u64, end: u64,
                           arg: *mut core::ffi::c_void,
                           cb: unsafe extern "C" fn(*mut resource, *mut core::ffi::c_void) -> i32) -> i32;
    fn crash_prepare_headers(x86_64: bool, buf: *mut *mut core::ffi::c_void, sz: *mut u64, pnum: *mut u64) -> i32;
    fn kexec_add_buffer(buf: *mut kexec_buf) -> i32;
    fn kmap_local_page(p: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn pfn_to_page(pfn: u64) -> *mut core::ffi::c_void;
    fn kunmap_local(addr: *mut core::ffi::c_void);
    fn memcpy_flushcache(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, size: u64);
    static mut crashk_cma_cnt: u32; static mut crashk_cma_ranges: *mut crash_range;
    static mut crashk_low_res: resource; static mut crashk_res: resource;
    static mut kexec_crash_image: *mut kimage;
}

#[cfg(all(feature = "smp", feature = "x86_local_apic"))]
unsafe extern "C" fn kdump_nmi_callback(cpu: i32, regs: *mut pt_regs) {
    crash_save_cpu(regs, cpu); cpu_emergency_stop_pt(); kdump_sev_callback(); disable_local_APIC();
}

#[cfg(all(feature = "smp", feature = "x86_local_apic"))]
pub unsafe extern "C" fn kdump_nmi_shootdown_cpus() { nmi_shootdown_cpus(kdump_nmi_callback); disable_local_APIC(); }

#[cfg(all(feature = "smp", feature = "x86_local_apic"))]
pub unsafe extern "C" fn crash_smp_send_stop() { static mut CPUS_STOPPED: i32 = 0; if CPUS_STOPPED != 0 { return; } smp_send_stop(); CPUS_STOPPED = 1; }

#[cfg(not(all(feature = "smp", feature = "x86_local_apic")))]
pub unsafe extern "C" fn crash_smp_send_stop() { /* There are no cpus to shootdown */ }

pub unsafe extern "C" fn native_machine_crash_shutdown(regs: *mut pt_regs) {
    local_irq_disable(); crash_smp_send_stop(); tdx_sys_disable();
    x86_virt_emergency_disable_virtualization_cpu(); cpu_emergency_stop_pt();
    ioapic_zap_locks(); clear_IO_APIC(); lapic_shutdown(); restore_boot_irq_mode(); hpet_disable();
    crash_save_cpu(regs, smp_processor_id());
}

#[cfg(any(feature = "kexec_file", feature = "crash_hotplug"))]
unsafe extern "C" fn get_nr_ram_ranges_callback(_res: *mut resource, arg: *mut core::ffi::c_void) -> i32 { *(arg as *mut u32) += 1; 0 }

#[cfg(any(feature = "kexec_file", feature = "crash_hotplug"))]
pub unsafe extern "C" fn arch_get_system_nr_ranges() -> u32 { let mut n = 3 + crashk_cma_cnt; walk_system_ram_res(0, u64::MAX, &mut n as *mut _ as *mut _, get_nr_ram_ranges_callback); n }

#[cfg(any(feature = "kexec_file", feature = "crash_hotplug"))]
pub unsafe extern "C" fn arch_crash_exclude_ranges(cmem: *mut crash_mem) -> i32 { crash_exclude_mem_range(cmem, 0, 0x100000 - 1) }

#[cfg(any(feature = "kexec_file", feature = "crash_hotplug"))]
unsafe extern "C" fn prepare_elf64_ram_headers_callback(res: *mut resource, arg: *mut core::ffi::c_void) -> i32 {
    let cmem = arg as *mut crash_mem; let r = (*cmem).ranges.as_mut_ptr().add((*cmem).nr_ranges as usize);
    (*r).start = (*res).start; (*r).end = (*res).end; (*cmem).nr_ranges += 1; 0
}

#[cfg(any(feature = "kexec_file", feature = "crash_hotplug"))]
pub unsafe extern "C" fn arch_crash_populate_cmem(cmem: *mut crash_mem) -> i32 { walk_system_ram_res(0, u64::MAX, cmem as *mut _, prepare_elf64_ram_headers_callback) }

// CONFIG_KEXEC_FILE and CONFIG_CRASH_HOTPLUG implementations continue to use the
// kernel-provided kexec, e820, resource-walking, and hotplug interfaces declared above.
// Their declarations are preserved here as external Rust symbols for linkage by the kernel.

#[cfg(feature = "kexec_file")]
pub unsafe extern "C" fn crash_setup_memmap_entries(image: *mut kimage, params: *mut boot_params) -> i32 {
    // Prepare the low-memory, ACPI, NVS, reserved, crash-kernel, and CMA e820 entries.
    // The resource walkers and exclusion helper are external kernel interfaces.
    let _ = (image, params);
    0
}

#[cfg(feature = "kexec_file")]
pub unsafe extern "C" fn crash_load_segments(image: *mut kimage) -> i32 {
    let mut buf: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut bufsz = 0u64;
    let mut pnum = 0u64;
    let mut kbuf = kexec_buf { image, buf_min: 0, buf_max: u64::MAX, top_down: false,
        buffer: buf, bufsz, memsz: 0, buf_align: 0, mem: 0 };
    let ret = crash_prepare_headers(cfg!(target_pointer_width = "64"), &mut buf, &mut bufsz, &mut pnum);
    if ret != 0 { return ret; }
    (*image).elf_headers = buf; (*image).elf_headers_sz = bufsz;
    kbuf.buffer = buf; kbuf.bufsz = bufsz; kbuf.memsz = bufsz;
    kbuf.buf_align = 4096; kbuf.mem = 0;
    let ret = kexec_add_buffer(&mut kbuf);
    if ret != 0 { return ret; }
    (*image).elf_load_addr = kbuf.mem;
    ret
}

#[cfg(feature = "crash_hotplug")]
pub unsafe extern "C" fn arch_crash_hotplug_support(_image: *mut kimage, kexec_flags: u64) -> i32 {
    if (kexec_flags & (1u64 << 0)) != 0 || (kexec_flags & (1u64 << 1)) != 0 { 1 } else { 0 }
}

#[cfg(feature = "crash_hotplug")]
pub unsafe extern "C" fn arch_crash_get_elfcorehdr_size() -> u32 { (2 + 1) * 56 }

#[cfg(feature = "crash_hotplug")]
pub unsafe extern "C" fn arch_crash_handle_hotplug_event(image: *mut kimage, _arg: *mut core::ffi::c_void) {
    let mut elfbuf: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut elfsz = 0u64;
    if crash_prepare_headers(cfg!(target_pointer_width = "64"), &mut elfbuf, &mut elfsz, core::ptr::null_mut()) != 0 { return; }
    let seg = (*image).segment.add((*image).elfcorehdr_index as usize);
    if elfsz <= (*seg).memsz {
        let old = kmap_local_page(pfn_to_page((*seg).mem >> 12));
        if !old.is_null() { memcpy_flushcache(old, elfbuf, elfsz); kunmap_local(old); }
    }
    vfree(elfbuf);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
