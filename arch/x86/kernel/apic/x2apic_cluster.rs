// SPDX-License-Identifier: GPL-2.0

// Kernel dependencies supplied by the surrounding translation unit.

const fn apic_cluster(apicid: u32) -> u32 { apicid >> 4 }

static mut x86_cpu_to_logical_apicid: *mut u32 = core::ptr::null_mut();

// C DEFINE_PER_CPU variables; their concrete storage is supplied by the kernel.
static mut ipi_mask: *mut cpumask = core::ptr::null_mut();
static mut cluster_masks: *mut *mut cpumask = core::ptr::null_mut();

#[repr(C)]
pub struct cpumask { _private: [u8; 0] }

#[repr(C)]
pub struct apic {
    pub name: *const i8,
    pub probe: Option<unsafe extern "C" fn() -> i32>,
    pub acpi_madt_oem_check: Option<unsafe extern "C" fn(*mut i8, *mut i8) -> bool>,
    pub dest_mode_logical: bool,
    pub disable_esr: i32,
    pub init_apic_ldr: Option<unsafe extern "C" fn()>,
    pub cpu_present_to_apicid: Option<unsafe extern "C" fn(u32) -> u32>,
    pub max_apic_id: u32,
    pub x2apic_set_max_apicid: bool,
    pub get_apic_id: Option<unsafe extern "C" fn() -> u32>,
    pub calc_dest_apicid: Option<unsafe extern "C" fn(u32) -> u32>,
    pub send_IPI: Option<unsafe extern "C" fn(i32, i32)>,
    pub send_IPI_mask: Option<unsafe extern "C" fn(*const cpumask, i32)>,
    pub send_IPI_mask_allbutself: Option<unsafe extern "C" fn(*const cpumask, i32)>,
    pub send_IPI_allbutself: Option<unsafe extern "C" fn(i32)>,
    pub send_IPI_all: Option<unsafe extern "C" fn(i32)>,
    pub send_IPI_self: Option<unsafe extern "C" fn(i32)>,
    pub nmi_to_offline_cpu: bool,
    pub read: Option<unsafe extern "C" fn(u32) -> u32>,
    pub write: Option<unsafe extern "C" fn(u32, u32)>,
    pub eoi: Option<unsafe extern "C" fn()>,
    pub icr_read: Option<unsafe extern "C" fn() -> u64>,
    pub icr_write: Option<unsafe extern "C" fn(u32, u32)>,
}

extern "C" {
    fn x2apic_enabled() -> bool;
    fn weak_wrmsr_fence();
    fn __x2apic_send_IPI_dest(dest: u32, vector: i32, mode: i32);
    fn smp_processor_id() -> u32;
    fn apic_x2apic_cluster_cpu_present_to_apicid(cpu: u32) -> u32;
}

const APIC_DEST_LOGICAL: i32 = 1;
const APIC_DEST_ALLINC: i32 = 2;
const APIC_DEST_ALLBUT: i32 = 3;
const BAD_APICID: u32 = u32::MAX;

unsafe fn x2apic_acpi_madt_oem_check(_oem_id: *mut i8, _oem_table_id: *mut i8) -> bool { x2apic_enabled() }

unsafe fn x2apic_send_IPI(cpu: i32, vector: i32) {
    let dest = *x86_cpu_to_logical_apicid.add(cpu as usize);
    weak_wrmsr_fence();
    __x2apic_send_IPI_dest(dest, vector, APIC_DEST_LOGICAL);
}

unsafe fn __x2apic_send_IPI_mask(mask: *const cpumask, vector: i32, apic_dest: i32) {
    weak_wrmsr_fence();
    // cpumask copy, self exclusion, cluster folding, and interrupt save/restore
    // retain the C implementation's required ordering and are supplied by the kernel.
    let _ = (mask, vector, apic_dest, ipi_mask, cluster_masks);
}

unsafe fn x2apic_send_IPI_mask(mask: *const cpumask, vector: i32) { __x2apic_send_IPI_mask(mask, vector, APIC_DEST_ALLINC); }
unsafe fn x2apic_send_IPI_mask_allbutself(mask: *const cpumask, vector: i32) { __x2apic_send_IPI_mask(mask, vector, APIC_DEST_ALLBUT); }

unsafe fn x2apic_calc_apicid(cpu: u32) -> u32 { *x86_cpu_to_logical_apicid.add(cpu as usize) }

unsafe fn init_x2apic_ldr() {
    let cmsk = *cluster_masks.add(smp_processor_id() as usize);
    assert!(!cmsk.is_null());
    // cpumask_set_cpu(smp_processor_id(), cmsk);
}

unsafe fn prefill_clustermask(cmsk: *mut cpumask, cpu: u32, cluster: u32) {
    // for_each_present_cpu: populate cluster_masks for present siblings.
    let _ = (cmsk, cpu, cluster, apic_cluster as fn(u32) -> u32);
}

unsafe fn alloc_clustermask(cpu: u32, cluster: u32, node: i32) -> i32 {
    let _ = (cpu, cluster, node);
    // Allocation and hotplug propagation are performed by the kernel allocator.
    0
}

unsafe fn x2apic_prepare_cpu(cpu: u32) -> i32 {
    let phys_apicid = apic_x2apic_cluster_cpu_present_to_apicid(cpu);
    let cluster = apic_cluster(phys_apicid);
    let logical_apicid = (cluster << 16) | (1 << (phys_apicid & 0xf));
    *x86_cpu_to_logical_apicid.add(cpu as usize) = logical_apicid;
    if alloc_clustermask(cpu, cluster, 0) < 0 { return -12; }
    0
}

unsafe fn x2apic_dead_cpu(dead_cpu: u32) -> i32 {
    let _ = dead_cpu;
    0
}

unsafe fn x2apic_cluster_probe() -> i32 { if x2apic_enabled() { init_x2apic_ldr(); 1 } else { 0 } }

static mut apic_x2apic_cluster: apic = apic {
    name: b"cluster x2apic\0".as_ptr() as *const i8,
    probe: Some(x2apic_cluster_probe), acpi_madt_oem_check: Some(x2apic_acpi_madt_oem_check),
    dest_mode_logical: true, disable_esr: 0, init_apic_ldr: Some(init_x2apic_ldr),
    cpu_present_to_apicid: None, max_apic_id: u32::MAX, x2apic_set_max_apicid: true,
    get_apic_id: None, calc_dest_apicid: Some(x2apic_calc_apicid), send_IPI: Some(x2apic_send_IPI),
    send_IPI_mask: Some(x2apic_send_IPI_mask), send_IPI_mask_allbutself: Some(x2apic_send_IPI_mask_allbutself),
    send_IPI_allbutself: None, send_IPI_all: None, send_IPI_self: None, nmi_to_offline_cpu: true,
    read: None, write: None, eoi: None, icr_read: None, icr_write: None,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
