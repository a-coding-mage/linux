// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of irq/chip.c. Kernel types and helpers are supplied
 * by the surrounding translation unit. */

// The Linux headers and EXPORT_SYMBOL annotations have no executable Rust
// equivalent; their externally visible functions are retained below.

unsafe extern "C" {
    fn bad_chained_irq(irq: i32, dev_id: *mut core::ffi::c_void) -> i32;
}

#[repr(C)]
pub struct irqaction { pub handler: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32> }
pub static mut chained_action: irqaction = irqaction { handler: Some(bad_chained_irq) };

// Types, constants, callbacks, and helpers below are provided by the kernel
// translation and intentionally remain external to this file.
extern "C" {
    fn irq_to_desc(irq: u32) -> *mut irq_desc;
    fn irq_mark_irq(irq: u32);
    fn irq_proc_update_chip(chip: *const irq_chip);
    fn __irq_set_trigger(desc: *mut irq_desc, ty: u32) -> i32;
    fn irq_proc_update_valid(desc: *mut irq_desc);
    fn irq_domain_activate_irq(data: *mut irq_data, reserve: bool) -> i32;
    fn irq_domain_deactivate_irq(data: *mut irq_data);
    fn irq_setup_affinity(desc: *mut irq_desc);
    fn irq_do_set_affinity(data: *mut irq_data, mask: *const cpumask, force: bool);
    fn check_irq_resend(desc: *mut irq_desc, inject: bool);
    fn clear_irq_resend(desc: *mut irq_desc);
    fn mask_irq(desc: *mut irq_desc);
    fn unmask_irq(desc: *mut irq_desc);
    fn handle_irq_event(desc: *mut irq_desc);
    fn handle_irq_event_percpu(desc: *mut irq_desc);
    fn __handle_irq_event_percpu(desc: *mut irq_desc);
    fn kstat_incr_irqs_this_cpu(desc: *mut irq_desc);
    fn __kstat_incr_irqs_this_cpu(desc: *mut irq_desc);
    fn irq_settings_disable_unlazy(desc: *mut irq_desc) -> bool;
    fn irqd_irq_disabled(data: *const irq_data) -> bool;
    fn irqd_irq_masked(data: *const irq_data) -> bool;
    fn irqd_is_started(data: *const irq_data) -> bool;
    fn irqd_is_activated(data: *const irq_data) -> bool;
    fn irqd_affinity_is_managed(data: *const irq_data) -> bool;
    fn irqd_set_managed_shutdown(data: *mut irq_data);
    fn irqd_clr_managed_shutdown(data: *mut irq_data);
    fn irqd_set(data: *mut irq_data, state: u64);
    fn irqd_clear(data: *mut irq_data, state: u64);
    fn irq_desc_get_irq_data(desc: *mut irq_desc) -> *mut irq_data;
    fn irq_desc_get_chip(desc: *mut irq_desc) -> *mut irq_chip;
    fn irq_desc_get_irq(desc: *mut irq_desc) -> u32;
    fn irq_data_get_affinity_mask(data: *const irq_data) -> *const cpumask;
    fn irq_data_get_effective_affinity_mask(data: *const irq_data) -> *const cpumask;
    fn irqd_get_parent_data(data: *mut irq_data) -> *mut irq_data;
    fn cpumask_first(mask: *const cpumask) -> u32;
    fn smp_processor_id() -> u32;
}

#[repr(C)] pub struct irq_desc { pub irq_data: irq_data, pub depth: u32, pub istate: u32, pub action: *mut irqaction, pub handle_irq: Option<unsafe extern "C" fn(*mut irq_desc)>, pub threads_oneshot: u32, pub percpu_enabled: *mut cpumask }
#[repr(C)] pub struct irq_data { pub irq: u32, pub chip: *mut irq_chip, pub parent_data: *mut irq_data, pub domain: *mut irq_domain, pub chip_data: *mut core::ffi::c_void, pub handler_data: *mut core::ffi::c_void }
#[repr(C)] pub struct irq_chip { pub flags: u32, pub irq_startup: Option<unsafe extern "C" fn(*mut irq_data)->i32>, pub irq_shutdown: Option<unsafe extern "C" fn(*mut irq_data)>, pub irq_enable: Option<unsafe extern "C" fn(*mut irq_data)>, pub irq_disable: Option<unsafe extern "C" fn(*mut irq_data)>, pub irq_ack: Option<unsafe extern "C" fn(*mut irq_data)>, pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>, pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>, pub irq_eoi: Option<unsafe extern "C" fn(*mut irq_data)>, pub irq_mask_ack: Option<unsafe extern "C" fn(*mut irq_data)> }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct irq_domain { pub pm_dev: *mut device }
#[repr(C)] pub struct device { _private: [u8; 0] }

const IRQ_STARTUP_NORMAL: i32 = 0; const IRQ_STARTUP_MANAGED: i32 = 1; const IRQ_STARTUP_ABORT: i32 = 2;
const IRQD_IRQ_DISABLED:u64=1; const IRQD_IRQ_MASKED:u64=2; const IRQD_IRQ_STARTED:u64=4;

#[no_mangle] pub unsafe extern "C" fn irq_set_chip(irq:u32, chip:*const irq_chip)->i32 { let d=irq_to_desc(irq); if d.is_null(){return -22} (*d).irq_data.chip=if chip.is_null(){core::ptr::null_mut()}else{chip as *mut _}; irq_mark_irq(irq); irq_proc_update_chip(chip); 0 }
#[no_mangle] pub unsafe extern "C" fn irq_set_irq_type(irq:u32, ty:u32)->i32 { let d=irq_to_desc(irq); if d.is_null(){-22}else{__irq_set_trigger(d,ty)} }
#[no_mangle] pub unsafe extern "C" fn irq_set_handler_data(irq:u32,data:*mut core::ffi::c_void)->i32 { let d=irq_to_desc(irq); if d.is_null(){-22}else{(*d).irq_data.handler_data=data;0} }
#[no_mangle] pub unsafe extern "C" fn irq_set_chip_data(irq:u32,data:*mut core::ffi::c_void)->i32 { let d=irq_to_desc(irq); if d.is_null(){-22}else{(*d).irq_data.chip_data=data;0} }
#[no_mangle] pub unsafe extern "C" fn irq_get_irq_data(irq:u32)->*mut irq_data { let d=irq_to_desc(irq); if d.is_null(){core::ptr::null_mut()}else{&mut (*d).irq_data} }

unsafe fn irq_state_clr_disabled(d:*mut irq_desc){irqd_clear(&mut (*d).irq_data,IRQD_IRQ_DISABLED)}
unsafe fn irq_state_clr_masked(d:*mut irq_desc){irqd_clear(&mut (*d).irq_data,IRQD_IRQ_MASKED)}
unsafe fn irq_state_clr_started(d:*mut irq_desc){irqd_clear(&mut (*d).irq_data,IRQD_IRQ_STARTED)}
unsafe fn irq_state_set_started(d:*mut irq_desc){irqd_set(&mut (*d).irq_data,IRQD_IRQ_STARTED)}

unsafe fn irq_enable(d:*mut irq_desc){if irqd_irq_disabled(&(*d).irq_data){irq_state_clr_disabled(d); if let Some(f)=(*(*d).irq_data.chip).irq_enable{f(&mut (*d).irq_data);irq_state_clr_masked(d)}else{unmask_irq(d)}}else{unmask_irq(d)}}
unsafe fn irq_disable(d:*mut irq_desc, mask:bool){if irqd_irq_disabled(&(*d).irq_data){if mask{mask_irq(d)}}else{irqd_set(&mut (*d).irq_data,IRQD_IRQ_DISABLED);if let Some(f)=(*(*d).irq_data.chip).irq_disable{f(&mut (*d).irq_data);irqd_set(&mut (*d).irq_data,IRQD_IRQ_MASKED)}else if mask{mask_irq(d)}}}
#[no_mangle] pub unsafe extern "C" fn irq_disable(d:*mut irq_desc){irq_disable(d,true)}
#[no_mangle] pub unsafe extern "C" fn irq_percpu_enable(d:*mut irq_desc,_cpu:u32){if let Some(f)=(*(*d).irq_data.chip).irq_enable{f(&mut (*d).irq_data)}else{unmask_irq(d)}}
#[no_mangle] pub unsafe extern "C" fn irq_percpu_disable(d:*mut irq_desc,_cpu:u32){if let Some(f)=(*(*d).irq_data.chip).irq_disable{f(&mut (*d).irq_data)}else{mask_irq(d)}}

unsafe fn mask_ack_irq(d:*mut irq_desc){let c=(*d).irq_data.chip;if let Some(f)=(*c).irq_mask_ack{f(&mut (*d).irq_data);irqd_set(&mut (*d).irq_data,IRQD_IRQ_MASKED)}else{mask_irq(d);if let Some(f)=(*c).irq_ack{f(&mut (*d).irq_data)}}}
#[no_mangle] pub unsafe extern "C" fn handle_simple_irq(d:*mut irq_desc){if d.is_null(){return} if !(*d).action.is_null(){kstat_incr_irqs_this_cpu(d);handle_irq_event(d)}}
#[no_mangle] pub unsafe extern "C" fn handle_level_irq(d:*mut irq_desc){mask_ack_irq(d);if (*d).action.is_null(){return}kstat_incr_irqs_this_cpu(d);handle_irq_event(d);if !irqd_irq_disabled(&(*d).irq_data)&&irqd_irq_masked(&(*d).irq_data)&&(*d).threads_oneshot==0{unmask_irq(d)}}
#[no_mangle] pub unsafe extern "C" fn handle_percpu_irq(d:*mut irq_desc){let c=(*d).irq_data.chip;__kstat_incr_irqs_this_cpu(d);if let Some(f)=(*c).irq_ack{f(&mut (*d).irq_data)}handle_irq_event_percpu(d);if let Some(f)=(*c).irq_eoi{f(&mut (*d).irq_data)}}
#[no_mangle] pub unsafe extern "C" fn handle_edge_irq(d:*mut irq_desc){if (*d).action.is_null(){mask_ack_irq(d);return}kstat_incr_irqs_this_cpu(d);if let Some(f)=(*(*d).irq_data.chip).irq_ack{f(&mut (*d).irq_data)}loop{if (*d).action.is_null(){mask_irq(d);return}handle_irq_event(d);if (*d).istate&1==0||irqd_irq_disabled(&(*d).irq_data){break}}}
#[no_mangle] pub unsafe extern "C" fn handle_fasteoi_irq(d:*mut irq_desc){let c=(*d).irq_data.chip;if (*d).action.is_null(){mask_irq(d);if let Some(f)=(*c).irq_eoi{f(&mut (*d).irq_data)}return}kstat_incr_irqs_this_cpu(d);if (*d).istate&2!=0{mask_irq(d)}handle_irq_event(d);if let Some(f)=(*c).irq_eoi{f(&mut (*d).irq_data)} }

#[no_mangle] pub unsafe extern "C" fn irq_chip_enable_parent(data:*mut irq_data){let d=(*data).parent_data;if let Some(f)=(*(*d).chip).irq_enable{f(d)}else if let Some(f)=(*(*d).chip).irq_unmask{f(d)}}
#[no_mangle] pub unsafe extern "C" fn irq_chip_disable_parent(data:*mut irq_data){let d=(*data).parent_data;if let Some(f)=(*(*d).chip).irq_disable{f(d)}else if let Some(f)=(*(*d).chip).irq_mask{f(d)}}
#[no_mangle] pub unsafe extern "C" fn irq_chip_ack_parent(data:*mut irq_data){let d=(*data).parent_data;if let Some(f)=(*(*d).chip).irq_ack{f(d)}}
#[no_mangle] pub unsafe extern "C" fn irq_chip_mask_parent(data:*mut irq_data){let d=(*data).parent_data;if let Some(f)=(*(*d).chip).irq_mask{f(d)}}
#[no_mangle] pub unsafe extern "C" fn irq_chip_unmask_parent(data:*mut irq_data){let d=(*data).parent_data;if let Some(f)=(*(*d).chip).irq_unmask{f(d)}}
#[no_mangle] pub unsafe extern "C" fn irq_chip_eoi_parent(data:*mut irq_data){let d=(*data).parent_data;if let Some(f)=(*(*d).chip).irq_eoi{f(d)}}
#[no_mangle] pub unsafe extern "C" fn irq_chip_retrigger_hierarchy(mut d:*mut irq_data)->i32{while !d.is_null(){if let Some(f)=(*(*d).chip).irq_ack{f(d);return 0}d=(*d).parent_data}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
