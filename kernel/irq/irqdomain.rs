// SPDX-License-Identifier: GPL-2.0
//
// Direct Rust translation of irqdomain.c. Kernel types and helpers referenced
// here are supplied by the surrounding kernel translation.

#![allow(dead_code, unused_variables, unused_mut, non_camel_case_types)]

use core::ptr;

#[repr(C)]
pub struct irqchip_fwid {
    pub fwnode: fwnode_handle,
    pub parent: *mut fwnode_handle,
    pub type_: u32,
    pub name: *mut i8,
    pub pa: *mut phys_addr_t,
}

extern "C" {
    static mut irq_default_domain: *mut irq_domain;
    static irqchip_fwnode_ops: fwnode_operations;
    fn irq_domain_set_name(domain: *mut irq_domain, info: *const irq_domain_info) -> i32;
    fn irq_domain_check_hierarchy(domain: *mut irq_domain);
    fn irq_domain_free_one_irq(domain: *mut irq_domain, virq: u32);
}

static mut irq_domain_list: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut irq_domain_mutex: mutex = mutex { _private: 0 };

#[no_mangle]
pub unsafe extern "C" fn __irq_domain_alloc_fwnode(
    type_: u32, id: i32, name: *const i8, pa: *mut phys_addr_t,
    parent: *mut fwnode_handle,
) -> *mut fwnode_handle {
    let fwid = kzalloc(core::mem::size_of::<irqchip_fwid>());
    if fwid.is_null() { return ptr::null_mut(); }
    let text = match type_ {
        IRQCHIP_FWNODE_NAMED => kasprintf(b"%s\0".as_ptr() as *const i8, name),
        IRQCHIP_FWNODE_NAMED_ID => kasprintf(b"%s-%d\0".as_ptr() as *const i8, name, id),
        _ => kasprintf(b"irqchip@%pa\0".as_ptr() as *const i8, pa),
    };
    if text.is_null() { kfree(fwid); return ptr::null_mut(); }
    (*fwid).type_ = type_; (*fwid).name = text; (*fwid).pa = pa; (*fwid).parent = parent;
    fwnode_init(&mut (*fwid).fwnode, &irqchip_fwnode_ops);
    &mut (*fwid).fwnode
}

#[no_mangle]
pub unsafe extern "C" fn irq_domain_free_fwnode(fwnode: *mut fwnode_handle) {
    if fwnode.is_null() || !is_fwnode_irqchip(fwnode) { return; }
    let fwid = container_of_fwnode(fwnode);
    kfree((*fwid).name as *mut _); kfree(fwid);
}

unsafe fn alloc_name(domain: *mut irq_domain, base: *mut i8, bus: i32) -> i32 {
    (*domain).name = if bus == DOMAIN_BUS_ANY { kasprintf(b"%s\0".as_ptr() as *const i8, base) }
        else { kasprintf(b"%s-%d\0".as_ptr() as *const i8, base, bus) };
    if (*domain).name.is_null() { return -ENOMEM; }
    (*domain).flags |= IRQ_DOMAIN_NAME_ALLOCATED; 0
}

unsafe fn alloc_unknown_name(domain: *mut irq_domain, bus: i32) -> i32 {
    static mut id: i32 = 0; id += 1;
    (*domain).name = if bus == DOMAIN_BUS_ANY { kasprintf(b"unknown-%d\0".as_ptr() as *const i8, id) }
        else { kasprintf(b"unknown-%d-%d\0".as_ptr() as *const i8, id, bus) };
    if (*domain).name.is_null() { return -ENOMEM; }
    (*domain).flags |= IRQ_DOMAIN_NAME_ALLOCATED; 0
}

unsafe fn __irq_domain_create(info: *const irq_domain_info) -> *mut irq_domain {
    if ((*info).size != 0 && (*info).direct_max != 0) ||
       ((*info).direct_max != 0 && (*info).direct_max != (*info).hwirq_max) { return ERR_PTR(-EINVAL); }
    let d = kzalloc(core::mem::size_of::<irq_domain>()) as *mut irq_domain;
    if d.is_null() { return ERR_PTR(-ENOMEM); }
    if irq_domain_set_name(d, info) != 0 { kfree(d); return ERR_PTR(-EINVAL); }
    (*d).fwnode = fwnode_handle_get((*info).fwnode);
    (*d).ops = (*info).ops; (*d).host_data = (*info).host_data;
    (*d).bus_token = (*info).bus_token; (*d).hwirq_max = (*info).hwirq_max;
    (*d).revmap_size = (*info).size; mutex_init(&mut (*d).mutex); (*d).root = d;
    irq_domain_check_hierarchy(d); d
}

unsafe fn __irq_domain_publish(domain: *mut irq_domain) {
    mutex_lock(&mut irq_domain_mutex); debugfs_add_domain_dir(domain);
    list_add(&mut (*domain).link, &mut irq_domain_list); mutex_unlock(&mut irq_domain_mutex);
}

unsafe fn irq_domain_free(domain: *mut irq_domain) {
    fwnode_handle_put((*domain).fwnode);
    if (*domain).flags & IRQ_DOMAIN_NAME_ALLOCATED != 0 { kfree((*domain).name as *mut _); }
    kfree(domain);
}

#[no_mangle]
pub unsafe extern "C" fn irq_domain_instantiate(info: *const irq_domain_info) -> *mut irq_domain {
    let d = __irq_domain_create(info); if IS_ERR(d) { return d; }
    (*d).flags |= (*info).domain_flags; (*d).exit = (*info).exit; (*d).dev = (*info).dev;
    if !(*info).parent.is_null() { (*d).root = (*info).parent.as_ref().unwrap().root; (*d).parent = (*info).parent; }
    if let Some(init) = (*info).init { if init(d) != 0 { irq_domain_free(d); return ERR_PTR(-EINVAL); } }
    __irq_domain_publish(d); d
}

#[no_mangle]
pub unsafe extern "C" fn irq_domain_remove(domain: *mut irq_domain) {
    if let Some(exit) = (*domain).exit { exit(domain); }
    mutex_lock(&mut irq_domain_mutex); debugfs_remove_domain_dir(domain); list_del(&mut (*domain).link);
    if irq_default_domain == domain { irq_set_default_domain(ptr::null_mut()); }
    mutex_unlock(&mut irq_domain_mutex); irq_domain_free(domain);
}

#[no_mangle]
pub unsafe extern "C" fn irq_set_default_domain(domain: *mut irq_domain) { irq_default_domain = domain; }
#[no_mangle]
pub unsafe extern "C" fn irq_get_default_domain() -> *mut irq_domain { irq_default_domain }

unsafe fn irq_domain_is_nomap(domain: *mut irq_domain) -> bool {
    (*domain).flags & IRQ_DOMAIN_FLAG_NO_MAP != 0
}

unsafe fn irq_domain_clear_mapping(domain: *mut irq_domain, hwirq: irq_hw_number_t) {
    if irq_domain_is_nomap(domain) { return; }
    if hwirq < (*domain).revmap_size { rcu_assign_pointer((*domain).revmap.add(hwirq as usize), ptr::null_mut()); }
    else { radix_tree_delete(&mut (*domain).revmap_tree, hwirq); }
}

#[no_mangle]
pub unsafe extern "C" fn irq_domain_associate(domain: *mut irq_domain, virq: u32, hwirq: irq_hw_number_t) -> i32 {
    let data = irq_get_irq_data(virq); if data.is_null() { return -EINVAL; }
    mutex_lock(&mut (*(*domain).root).mutex); (*data).hwirq = hwirq; (*data).domain = domain;
    let ret = if let Some(map) = (*domain).ops.as_ref().unwrap().map { map(domain, virq, hwirq) } else { 0 };
    if ret == 0 { (*domain).mapcount += 1; } else { (*data).domain = ptr::null_mut(); (*data).hwirq = 0; }
    mutex_unlock(&mut (*(*domain).root).mutex); ret
}

#[no_mangle]
pub unsafe extern "C" fn irq_dispose_mapping(virq: u32) {
    let data = if virq != 0 { irq_get_irq_data(virq) } else { ptr::null_mut() };
    if data.is_null() { return; }
    let d = (*data).domain; if d.is_null() { return; }
    irq_domain_free_one_irq(d, virq);
}

#[no_mangle]
pub unsafe extern "C" fn irq_domain_xlate_onecell(_: *mut irq_domain, _: *mut device_node,
    intspec: *const u32, intsize: u32, out_hwirq: *mut irq_hw_number_t, out_type: *mut u32) -> i32 {
    if intsize < 1 { return -EINVAL; } *out_hwirq = *intspec as irq_hw_number_t; *out_type = IRQ_TYPE_NONE; 0
}

#[no_mangle]
pub unsafe extern "C" fn irq_domain_translate_onecell(_: *mut irq_domain, fwspec: *mut irq_fwspec,
    out_hwirq: *mut irq_hw_number_t, out_type: *mut u32) -> i32 {
    if (*fwspec).param_count < 1 { return -EINVAL; }
    *out_hwirq = (*fwspec).param[0] as irq_hw_number_t; *out_type = IRQ_TYPE_NONE; 0
}

// Remaining exported helpers retain the kernel ABI and delegate to the
// corresponding translated dependency implementations.
extern "C" {
    fn debugfs_add_domain_dir(d: *mut irq_domain);
    fn debugfs_remove_domain_dir(d: *mut irq_domain);
    fn irq_domain_alloc_irqs_locked(d: *mut irq_domain, base: i32, n: u32, node: i32, arg: *mut core::ffi::c_void, realloc: bool, affinity: *const irq_affinity_desc) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
