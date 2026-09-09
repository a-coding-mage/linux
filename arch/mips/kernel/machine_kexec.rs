// SPDX-License-Identifier: GPL-2.0-only
/*
 * machine_kexec.c for kexec
 * Created by <nschichan@corp.free.fr> on Thu Oct 12 15:15:06 2006
 */

use core::ffi::{c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct pt_regs { _private: [u8; 0] }
#[repr(C)]
pub struct fdt_header { _private: [u8; 0] }
#[repr(C)]
pub struct page { _private: [u8; 0] }
#[repr(C)]
pub struct kimage_segment {
    pub buf: *const c_void,
    pub bufsz: usize,
    pub mem: c_ulong,
    pub memsz: usize,
}
#[repr(C)]
pub struct kimage {
    pub type_: c_int,
    pub start: c_ulong,
    pub head: c_ulong,
    pub nr_segments: c_ulong,
    pub segment: *mut kimage_segment,
    pub control_code_page: *mut page,
}

extern "C" {
    pub static relocate_new_kernel: u8;
    pub static relocate_new_kernel_size: usize;
    pub static mut kexec_start_address: c_ulong;
    pub static mut kexec_indirection_page: c_ulong;
    pub static mut kexec_args: [c_ulong; 2];
    fn pr_debug(fmt: *const u8, ...);
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn fdt_check_header(fdt: *const fdt_header) -> c_int;
    fn phys_to_virt(addr: c_ulong) -> *mut c_void;
    fn page_address(page: *mut page) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn kexec_nonboot_cpu_func() -> bool;
    fn smp_processor_id() -> c_uint;
    fn cpu_online(cpu: c_uint) -> bool;
    fn set_cpu_online(cpu: c_uint, online: bool);
    fn local_irq_disable();
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn cpu_relax();
    fn kexec_reboot() -> !;
    fn smp_call_function(func: unsafe extern "C" fn(*mut c_void), info: *mut c_void, wait: c_int);
    fn num_online_cpus() -> c_uint;
    fn mdelay(ms: c_uint);
    fn default_machine_crash_shutdown(regs: *mut pt_regs);
    fn local_flush_icache_range(start: c_ulong, end: c_ulong);
    fn smp_mb__after_atomic();
    fn kexec_nonboot_cpu();
    fn __flush_cache_all();
    fn smp_wmb();
    fn atomic_set(v: *mut atomic_t, value: c_int);
    fn printk(fmt: *const u8, ...);
}

#[repr(C)]
pub struct atomic_t { pub counter: c_int }

static mut reboot_code_buffer: c_ulong = 0;

#[cfg(feature = "CONFIG_SMP")]
static mut relocated_kexec_smp_wait: Option<unsafe extern "C" fn(*mut c_void)> = None;
#[cfg(feature = "CONFIG_SMP")]
#[no_mangle]
pub static mut kexec_ready_to_reboot: atomic_t = atomic_t { counter: 0 };
#[cfg(feature = "CONFIG_SMP")]
#[no_mangle]
pub static mut _crash_smp_send_stop: Option<unsafe extern "C" fn()> = None;

#[no_mangle]
pub static mut _machine_kexec_shutdown: Option<unsafe extern "C" fn()> = None;
#[no_mangle]
pub static mut _machine_crash_shutdown: Option<unsafe extern "C" fn(*mut pt_regs)> = None;

unsafe fn kexec_image_info(kimage: *const kimage) {
    let mut i: c_ulong = 0;
    pr_debug(b"kexec kimage info:\n\0".as_ptr());
    pr_debug(b"  type:        %d\n\0".as_ptr(), (*kimage).type_);
    pr_debug(b"  start:       %lx\n\0".as_ptr(), (*kimage).start);
    pr_debug(b"  head:        %lx\n\0".as_ptr(), (*kimage).head);
    pr_debug(b"  nr_segments: %lu\n\0".as_ptr(), (*kimage).nr_segments);
    while i < (*kimage).nr_segments {
        let segment = &*(*kimage).segment.add(i as usize);
        pr_debug(b"    segment[%lu]: %016lx - %016lx, 0x%lx bytes, %lu pages\n\0".as_ptr(), i, segment.mem, segment.mem.wrapping_add(segment.memsz as c_ulong), segment.memsz as c_ulong, (segment.memsz as c_ulong) / PAGE_SIZE);
        i += 1;
    }
}

#[cfg(feature = "CONFIG_UHI_BOOT")]
unsafe extern "C" fn uhi_machine_kexec_prepare(kimage: *mut kimage) -> c_int {
    let mut i = 0;
    while i < (*kimage).nr_segments as usize {
        let fdt = core::mem::MaybeUninit::<fdt_header>::uninit();
        let segment = &*(*kimage).segment.add(i);
        if segment.memsz <= core::mem::size_of::<fdt_header>() { i += 1; continue; }
        if copy_from_user(fdt.as_ptr() as *mut c_void, segment.buf, core::mem::size_of::<fdt_header>()) != 0 { i += 1; continue; }
        if fdt_check_header(fdt.as_ptr()) != 0 { i += 1; continue; }
        kexec_args[0] = (-2i64) as c_ulong;
        kexec_args[1] = phys_to_virt(segment.mem) as c_ulong;
        break;
    }
    0
}

#[cfg(feature = "CONFIG_UHI_BOOT")]
#[no_mangle]
pub static mut _machine_kexec_prepare: Option<unsafe extern "C" fn(*mut kimage) -> c_int> = Some(uhi_machine_kexec_prepare);
#[cfg(not(feature = "CONFIG_UHI_BOOT"))]
#[no_mangle]
pub static mut _machine_kexec_prepare: Option<unsafe extern "C" fn(*mut kimage) -> c_int> = None;

const PAGE_SIZE: c_ulong = 4096;
const PAGE_MASK: c_ulong = !(PAGE_SIZE - 1);
const KEXEC_TYPE_DEFAULT: c_int = 0;
const IND_DONE: c_ulong = 1;
const IND_INDIRECTION: c_ulong = 2;
const IND_SOURCE: c_ulong = 4;
const IND_DESTINATION: c_ulong = 8;

#[no_mangle]
pub unsafe extern "C" fn machine_kexec_prepare(kimage: *mut kimage) -> c_int {
    #[cfg(feature = "CONFIG_SMP")]
    if !kexec_nonboot_cpu_func() { return -22; }
    kexec_image_info(kimage);
    if let Some(f) = _machine_kexec_prepare { return f(kimage); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn machine_kexec_cleanup(_kimage: *mut kimage) {}

#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" fn kexec_shutdown_secondary(_param: *mut c_void) {
    let cpu = smp_processor_id();
    if !cpu_online(cpu) { return; }
    set_cpu_online(cpu, false);
    local_irq_disable();
    while atomic_read(&kexec_ready_to_reboot) == 0 { cpu_relax(); }
    kexec_reboot();
}

#[no_mangle]
pub unsafe extern "C" fn machine_shutdown() {
    if let Some(f) = _machine_kexec_shutdown { f(); }
    #[cfg(feature = "CONFIG_SMP")]
    {
        smp_call_function(kexec_shutdown_secondary, core::ptr::null_mut(), 0);
        while num_online_cpus() > 1 { cpu_relax(); mdelay(1); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn machine_crash_shutdown(regs: *mut pt_regs) {
    if let Some(f) = _machine_crash_shutdown { f(regs); } else { default_machine_crash_shutdown(regs); }
}

#[cfg(feature = "CONFIG_SMP")]
#[no_mangle]
pub unsafe extern "C" fn kexec_nonboot_cpu_jump() {
    local_flush_icache_range(relocated_kexec_smp_wait as c_ulong, reboot_code_buffer.wrapping_add(relocate_new_kernel_size));
    if let Some(f) = relocated_kexec_smp_wait { f(core::ptr::null_mut()); }
}

#[no_mangle]
pub unsafe extern "C" fn kexec_reboot() -> ! {
    set_cpu_online(smp_processor_id(), true);
    smp_mb__after_atomic();
    #[cfg(feature = "CONFIG_SMP")]
    if smp_processor_id() > 0 { kexec_nonboot_cpu(); }
    local_flush_icache_range(reboot_code_buffer, reboot_code_buffer.wrapping_add(relocate_new_kernel_size));
    let do_kexec: unsafe extern "C" fn() -> ! = core::mem::transmute(reboot_code_buffer as *const c_void);
    do_kexec();
}

#[no_mangle]
pub unsafe extern "C" fn machine_kexec(image: *mut kimage) {
    reboot_code_buffer = page_address((*image).control_code_page) as c_ulong;
    kexec_start_address = phys_to_virt((*image).start) as c_ulong;
    if (*image).type_ == KEXEC_TYPE_DEFAULT { kexec_indirection_page = phys_to_virt((*image).head & PAGE_MASK) as c_ulong; } else { kexec_indirection_page = &mut (*image).head as *mut c_ulong as c_ulong; }
    memcpy(reboot_code_buffer as *mut c_void, &relocate_new_kernel as *const u8 as *const c_void, relocate_new_kernel_size);
    let mut ptr = &mut (*image).head as *mut c_ulong;
    loop {
        let entry = *ptr;
        if entry == 0 || entry & IND_DONE != 0 { break; }
        if entry & IND_SOURCE != 0 || entry & IND_INDIRECTION != 0 || entry & IND_DESTINATION != 0 { *ptr = phys_to_virt(entry) as c_ulong; }
        ptr = if entry & IND_INDIRECTION != 0 { phys_to_virt(entry & PAGE_MASK) as *mut c_ulong } else { ptr.add(1) };
    }
    set_cpu_online(smp_processor_id(), false);
    local_irq_disable();
    printk(b"Will call new kernel at %08lx\n\0".as_ptr(), (*image).start);
    printk(b"Bye ...\n\0".as_ptr());
    __flush_cache_all();
    #[cfg(feature = "CONFIG_SMP")]
    { relocated_kexec_smp_wait = Some(core::mem::transmute(reboot_code_buffer.wrapping_add((kexec_smp_wait as c_ulong).wrapping_sub(&relocate_new_kernel as *const u8 as c_ulong)))); smp_wmb(); atomic_set(&mut kexec_ready_to_reboot, 1); }
    kexec_reboot();
}

extern "C" { static kexec_smp_wait: u8; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
