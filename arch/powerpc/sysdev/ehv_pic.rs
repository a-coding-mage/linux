/*
 * Driver for ePAPR Embedded Hypervisor PIC
 *
 * Translated from ehv_pic.c. Kernel and architecture symbols are supplied by
 * external dependencies.
 */

use core::ffi::c_void;

extern "C" {
    static mut global_ehv_pic: *mut ehv_pic;
    static mut ehv_pic_lock: c_void;
    static mut hwirq_intspec: [u32; NR_EHV_PIC_INTS];
    static mut mpic_percpu_base_vaddr: *mut u32;

    fn virq_to_hw(irq: u32) -> u32;
    fn ev_int_set_mask(src: u32, mask: u32);
    fn ev_int_eoi(src: u32);
    fn ev_int_get_config(src: u32, config: *mut u32, prio: *mut u32, cpu_dest: *mut u32);
    fn ev_int_set_config(src: u32, config: u32, prio: u32, cpu_dest: i32);
    fn irq_choose_cpu(dest: *const cpumask) -> i32;
    fn irq_find_mapping(host: *mut irq_domain, irq: i32) -> u32;
    fn mfspr(spr: u32) -> i32;
    fn ev_int_iack(cpu: u32, irq: *mut i32);
    fn irq_domain_get_of_node(h: *mut irq_domain) -> *mut device_node;
    fn irq_set_chip_data(virq: u32, data: *mut irq_chip);
    fn irq_set_chip_and_handler(virq: u32, chip: *mut irq_chip, handler: unsafe extern "C" fn());
    fn irq_set_irq_type(virq: u32, flow_type: u32);
    fn of_find_compatible_node(from: *mut device_node, ty: *const u8, compatible: *const u8) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut c_void);
    fn of_fwnode_handle(node: *mut device_node) -> *mut c_void;
    fn irq_domain_create_linear(fwnode: *mut c_void, size: u32, ops: *const irq_domain_ops, data: *mut c_void) -> *mut irq_domain;
    fn of_iomap(node: *mut device_node, index: u32) -> *mut u32;
    fn of_property_read_bool(node: *mut device_node, propname: *const u8) -> bool;
    fn irq_set_default_domain(host: *mut irq_domain);
    fn pr_err(fmt: *const u8, ...);
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: usize);
}

const NR_EHV_PIC_INTS: usize = 256;
const IRQ_TYPE_MPIC_DIRECT: u32 = 4;
const MPIC_EOI: usize = 0x00b0;
const SPRN_EPR: u32 = 0x137;
const IRQ_SET_MASK_OK: i32 = 0;
const IRQ_SET_MASK_OK_NOCOPY: i32 = 0;
const IRQ_TYPE_NONE: u32 = 0;
const IRQ_TYPE_EDGE_FALLING: u32 = 1;
const IRQ_TYPE_EDGE_RISING: u32 = 2;
const IRQ_TYPE_LEVEL_LOW: u32 = 8;
const IRQ_TYPE_LEVEL_HIGH: u32 = 4;
const IRQ_TYPE_EDGE_BOTH: u32 = 3;
const IRQ_TYPE_SENSE_MASK: u32 = 0x0f;
const VECPRI_SENSE_EDGE: u32 = 0;
const VECPRI_SENSE_LEVEL: u32 = 1;
const VECPRI_POLARITY_POSITIVE: u32 = 0;
const VECPRI_POLARITY_NEGATIVE: u32 = 1;
const VECPRI_POLARITY_MASK: u32 = 1;
const VECPRI_SENSE_MASK: u32 = 1 << 1;

#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct irq_domain { _private: [u8; 0] }
pub type irq_hw_number_t = u64;
pub type irq_flow_handler_t = unsafe extern "C" fn();
#[repr(C)] pub struct irq_data { pub irq: u32 }
#[repr(C)] pub struct irq_chip {
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_eoi: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_set_type: Option<unsafe extern "C" fn(*mut irq_data, u32) -> i32>,
    pub irq_set_affinity: Option<unsafe extern "C" fn(*mut irq_data, *const cpumask, bool) -> i32>,
}
#[repr(C)] pub struct irq_domain_ops {
    pub match_: Option<unsafe extern "C" fn(*mut irq_domain, *mut device_node, u32) -> i32>,
    pub map: Option<unsafe extern "C" fn(*mut irq_domain, u32, irq_hw_number_t) -> i32>,
    pub xlate: Option<unsafe extern "C" fn(*mut irq_domain, *mut device_node, *const u32, u32, *mut irq_hw_number_t, *mut u32) -> i32>,
}
#[repr(C)] pub struct ehv_pic { pub irqhost: *mut irq_domain, pub hc_irq: irq_chip, pub coreint_flag: bool }

unsafe fn ehv_pic_unmask_irq(d: *mut irq_data) { ev_int_set_mask(virq_to_hw((*d).irq), 0); }
unsafe fn ehv_pic_mask_irq(d: *mut irq_data) { ev_int_set_mask(virq_to_hw((*d).irq), 1); }
unsafe fn ehv_pic_end_irq(d: *mut irq_data) { ev_int_eoi(virq_to_hw((*d).irq)); }
unsafe fn ehv_pic_direct_end_irq(_: *mut irq_data) { mpic_percpu_base_vaddr.add(MPIC_EOI / 4).write_volatile(0); }

unsafe fn ehv_pic_set_affinity(d: *mut irq_data, dest: *const cpumask, _: bool) -> i32 {
    let src = virq_to_hw((*d).irq); let mut config = 0; let mut prio = 0; let mut cpu_dest = 0; let flags: usize = 0;
    spin_lock_irqsave(&mut ehv_pic_lock, &mut (flags as usize)); ev_int_get_config(src, &mut config, &mut prio, &mut cpu_dest); ev_int_set_config(src, config, prio, irq_choose_cpu(dest)); spin_unlock_irqrestore(&mut ehv_pic_lock, flags); IRQ_SET_MASK_OK
}

unsafe fn ehv_pic_type_to_vecpri(ty: u32) -> u32 { match ty & IRQ_TYPE_SENSE_MASK { IRQ_TYPE_EDGE_RISING => VECPRI_SENSE_EDGE | VECPRI_POLARITY_POSITIVE, IRQ_TYPE_EDGE_FALLING | IRQ_TYPE_EDGE_BOTH => VECPRI_SENSE_EDGE | VECPRI_POLARITY_NEGATIVE, IRQ_TYPE_LEVEL_HIGH => VECPRI_SENSE_LEVEL | VECPRI_POLARITY_POSITIVE, _ => VECPRI_SENSE_LEVEL | VECPRI_POLARITY_NEGATIVE } }
unsafe fn ehv_pic_set_irq_type(d: *mut irq_data, mut flow_type: u32) -> i32 { if flow_type == IRQ_TYPE_NONE { flow_type = IRQ_TYPE_LEVEL_LOW; } let src = virq_to_hw((*d).irq); let mut vold=0; let mut prio=0; let mut cpu_dest=0; ev_int_get_config(src,&mut vold,&mut prio,&mut cpu_dest); let vecpri=ehv_pic_type_to_vecpri(flow_type); let _vnew = (vold & !(VECPRI_POLARITY_MASK | VECPRI_SENSE_MASK)) | vecpri; prio=8; ev_int_set_config(src,vecpri,prio,cpu_dest as i32); IRQ_SET_MASK_OK_NOCOPY }

static mut ehv_pic_irq_chip: irq_chip = irq_chip { irq_mask: Some(ehv_pic_mask_irq), irq_unmask: Some(ehv_pic_unmask_irq), irq_eoi: Some(ehv_pic_end_irq), irq_set_type: Some(ehv_pic_set_irq_type), irq_set_affinity: None };
static mut ehv_pic_direct_eoi_irq_chip: irq_chip = irq_chip { irq_mask: Some(ehv_pic_mask_irq), irq_unmask: Some(ehv_pic_unmask_irq), irq_eoi: Some(ehv_pic_direct_end_irq), irq_set_type: Some(ehv_pic_set_irq_type), irq_set_affinity: None };

unsafe fn ehv_pic_host_match(h: *mut irq_domain, node: *mut device_node, _: u32) -> i32 { let n=irq_domain_get_of_node(h); (n.is_null() || n==node) as i32 }
unsafe fn ehv_pic_host_map(h: *mut irq_domain, virq: u32, hw: irq_hw_number_t) -> i32 { let p=(*(h as *mut ehv_pic)); let mut chip=&mut ehv_pic_irq_chip as *mut irq_chip; if !mpic_percpu_base_vaddr.is_null() && hwirq_intspec[hw as usize] & IRQ_TYPE_MPIC_DIRECT != 0 { chip=&mut ehv_pic_direct_eoi_irq_chip; } irq_set_chip_data(virq,chip); irq_set_chip_and_handler(virq,chip,handle_fasteoi_irq); irq_set_irq_type(virq,IRQ_TYPE_NONE); 0 }
unsafe fn ehv_pic_host_xlate(_: *mut irq_domain, _: *mut device_node, intspec: *const u32, intsize: u32, out_hwirq: *mut irq_hw_number_t, out_flags: *mut u32) -> i32 { *out_hwirq=*intspec as irq_hw_number_t; if intsize>1 { hwirq_intspec[*intspec as usize]=*intspec.add(1); *out_flags=match *intspec.add(1)&!IRQ_TYPE_MPIC_DIRECT { 1=>IRQ_TYPE_EDGE_FALLING,2=>IRQ_TYPE_EDGE_RISING,8=>IRQ_TYPE_LEVEL_LOW,_=>IRQ_TYPE_LEVEL_HIGH }; } else {*out_flags=IRQ_TYPE_NONE;} 0 }
static ehv_pic_host_ops: irq_domain_ops = irq_domain_ops { match_: Some(ehv_pic_host_match), map: Some(ehv_pic_host_map), xlate: Some(ehv_pic_host_xlate) };
extern "C" { fn handle_fasteoi_irq(); }

#[no_mangle] pub unsafe extern "C" fn ehv_pic_get_irq() -> u32 { let mut irq=0; if (*global_ehv_pic).coreint_flag { irq=mfspr(SPRN_EPR); } else { ev_int_iack(0,&mut irq); } if irq == 0xffff { 0 } else { irq_find_mapping((*global_ehv_pic).irqhost,irq) } }

#[no_mangle] pub unsafe extern "C" fn ehv_pic_init() { let np=of_find_compatible_node(core::ptr::null_mut(),core::ptr::null(),b"epapr,hv-pic\0".as_ptr()); if np.is_null(){return;} let p=kzalloc_obj::<ehv_pic>(); if p.is_null(){of_node_put(np);return;} (*p).irqhost=irq_domain_create_linear(of_fwnode_handle(np),NR_EHV_PIC_INTS as u32,core::ptr::null(),p as *mut c_void); if (*p).irqhost.is_null(){of_node_put(np);kfree(p as *mut c_void);return;} (*p).coreint_flag=of_property_read_bool(np,b"has-external-proxy\0".as_ptr()); global_ehv_pic=p; irq_set_default_domain((*p).irqhost); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
