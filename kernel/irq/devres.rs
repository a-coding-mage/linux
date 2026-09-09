// SPDX-License-Identifier: GPL-2.0
// C dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
struct irq_devres {
    irq: ::core::ffi::c_uint,
    dev_id: *mut ::core::ffi::c_void,
}

unsafe extern "C" {
    fn free_irq(irq: ::core::ffi::c_uint, dev_id: *mut ::core::ffi::c_void);
    fn dev_err_probe(dev: *mut device, rc: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn devres_alloc(release: unsafe extern "C" fn(*mut device, *mut ::core::ffi::c_void), size: usize, gfp: ::core::ffi::c_uint) -> *mut ::core::ffi::c_void;
    fn devres_free(res: *mut ::core::ffi::c_void);
    fn dev_name(dev: *mut device) -> *const ::core::ffi::c_char;
    fn request_threaded_irq(irq: ::core::ffi::c_uint, handler: irq_handler_t, thread_fn: irq_handler_t, irqflags: ::core::ffi::c_ulong, devname: *const ::core::ffi::c_char, dev_id: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn devres_add(dev: *mut device, res: *mut ::core::ffi::c_void);
    fn request_any_context_irq(irq: ::core::ffi::c_uint, handler: irq_handler_t, irqflags: ::core::ffi::c_ulong, devname: *const ::core::ffi::c_char, dev_id: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn devres_release(dev: *mut device, release: unsafe extern "C" fn(*mut device, *mut ::core::ffi::c_void), match_fn: unsafe extern "C" fn(*mut device, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::core::ffi::c_int, data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn warn_on(condition: bool);
    fn irq_free_descs(from: ::core::ffi::c_uint, cnt: ::core::ffi::c_uint);
    fn __irq_alloc_descs(irq: ::core::ffi::c_int, from: ::core::ffi::c_uint, cnt: ::core::ffi::c_uint, node: ::core::ffi::c_int, owner: *mut module, affinity: *const irq_affinity_desc) -> ::core::ffi::c_int;
}

#[repr(C)]
struct device;
#[repr(C)]
struct module;
#[repr(C)]
struct irq_affinity_desc;
type irq_handler_t = Option<unsafe extern "C" fn() -> ::core::ffi::c_int>;

unsafe extern "C" fn devm_irq_release(_dev: *mut device, res: *mut ::core::ffi::c_void) {
    let this = res as *mut irq_devres;
    free_irq((*this).irq, (*this).dev_id);
}

unsafe extern "C" fn devm_irq_match(_dev: *mut device, res: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let this = res as *mut irq_devres;
    let matched = data as *mut irq_devres;
    ((*this).irq == (*matched).irq && (*this).dev_id == (*matched).dev_id) as ::core::ffi::c_int
}

unsafe fn devm_request_result(dev: *mut device, rc: ::core::ffi::c_int, irq: ::core::ffi::c_uint, handler: irq_handler_t, thread_fn: irq_handler_t, devname: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if rc >= 0 { return rc; }
    let empty = b"\0".as_ptr() as *const ::core::ffi::c_char;
    dev_err_probe(dev, rc, b"request_irq(%u) %ps %ps %s\n\0".as_ptr() as *const _, irq, handler, thread_fn, if devname.is_null() { empty } else { devname })
}

unsafe fn __devm_request_threaded_irq(dev: *mut device, irq: ::core::ffi::c_uint, handler: irq_handler_t, thread_fn: irq_handler_t, irqflags: ::core::ffi::c_ulong, mut devname: *const ::core::ffi::c_char, dev_id: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let dr = devres_alloc(devm_irq_release, core::mem::size_of::<irq_devres>(), 0);
    if dr.is_null() { return -12; }
    if devname.is_null() { devname = dev_name(dev); }
    let rc = request_threaded_irq(irq, handler, thread_fn, irqflags, devname, dev_id);
    if rc != 0 { devres_free(dr); return rc; }
    let dr = dr as *mut irq_devres;
    (*dr).irq = irq; (*dr).dev_id = dev_id;
    devres_add(dev, dr as *mut _);
    0
}

pub unsafe fn devm_request_threaded_irq(dev: *mut device, irq: ::core::ffi::c_uint, handler: irq_handler_t, thread_fn: irq_handler_t, irqflags: ::core::ffi::c_ulong, devname: *const ::core::ffi::c_char, dev_id: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let rc = __devm_request_threaded_irq(dev, irq, handler, thread_fn, irqflags, devname, dev_id);
    devm_request_result(dev, rc, irq, handler, thread_fn, devname)
}

unsafe fn __devm_request_any_context_irq(dev: *mut device, irq: ::core::ffi::c_uint, handler: irq_handler_t, irqflags: ::core::ffi::c_ulong, mut devname: *const ::core::ffi::c_char, dev_id: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let dr = devres_alloc(devm_irq_release, core::mem::size_of::<irq_devres>(), 0);
    if dr.is_null() { return -12; }
    if devname.is_null() { devname = dev_name(dev); }
    let rc = request_any_context_irq(irq, handler, irqflags, devname, dev_id);
    if rc < 0 { devres_free(dr); return rc; }
    let dr = dr as *mut irq_devres;
    (*dr).irq = irq; (*dr).dev_id = dev_id;
    devres_add(dev, dr as *mut _);
    rc
}

pub unsafe fn devm_request_any_context_irq(dev: *mut device, irq: ::core::ffi::c_uint, handler: irq_handler_t, irqflags: ::core::ffi::c_ulong, devname: *const ::core::ffi::c_char, dev_id: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let rc = __devm_request_any_context_irq(dev, irq, handler, irqflags, devname, dev_id);
    devm_request_result(dev, rc, irq, handler, None, devname)
}

pub unsafe fn devm_free_irq(dev: *mut device, irq: ::core::ffi::c_uint, dev_id: *mut ::core::ffi::c_void) {
    let mut match_data = irq_devres { irq, dev_id };
    warn_on(devres_release(dev, devm_irq_release, devm_irq_match, &mut match_data as *mut _ as *mut _ ) != 0);
}

#[repr(C)]
struct irq_desc_devres { from: ::core::ffi::c_uint, cnt: ::core::ffi::c_uint }

unsafe extern "C" fn devm_irq_desc_release(_dev: *mut device, res: *mut ::core::ffi::c_void) {
    let this = res as *mut irq_desc_devres;
    irq_free_descs((*this).from, (*this).cnt);
}

pub unsafe fn __devm_irq_alloc_descs(dev: *mut device, irq: ::core::ffi::c_int, from: ::core::ffi::c_uint, cnt: ::core::ffi::c_uint, node: ::core::ffi::c_int, owner: *mut module, affinity: *const irq_affinity_desc) -> ::core::ffi::c_int {
    let dr = devres_alloc(devm_irq_desc_release, core::mem::size_of::<irq_desc_devres>(), 0);
    if dr.is_null() { return -12; }
    let base = __irq_alloc_descs(irq, from, cnt, node, owner, affinity);
    if base < 0 { devres_free(dr); return base; }
    let dr = dr as *mut irq_desc_devres;
    (*dr).from = base as _; (*dr).cnt = cnt;
    devres_add(dev, dr as *mut _);
    base
}

// CONFIG_GENERIC_IRQ_CHIP and CONFIG_IRQ_DOMAIN sections require their kernel-provided types and symbols.

#[cfg(CONFIG_GENERIC_IRQ_CHIP)]
#[repr(C)]
pub struct irq_chip_generic;
#[cfg(CONFIG_GENERIC_IRQ_CHIP)]
#[repr(C)]
pub struct irq_chip_type;
#[cfg(CONFIG_GENERIC_IRQ_CHIP)]
pub type irq_flow_handler_t = Option<unsafe extern "C" fn() -> ()>;
#[cfg(CONFIG_GENERIC_IRQ_CHIP)]
pub type irq_gc_flags = ::core::ffi::c_uint;

#[cfg(CONFIG_GENERIC_IRQ_CHIP)]
unsafe extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: ::core::ffi::c_uint) -> *mut ::core::ffi::c_void;
    fn irq_init_generic_chip(gc: *mut irq_chip_generic, name: *const ::core::ffi::c_char, num_ct: ::core::ffi::c_int, irq_base: ::core::ffi::c_uint, reg_base: *mut ::core::ffi::c_void, handler: irq_flow_handler_t);
    fn irq_remove_generic_chip(gc: *mut irq_chip_generic, msk: u32, clr: ::core::ffi::c_uint, set: ::core::ffi::c_uint);
    fn irq_setup_generic_chip(gc: *mut irq_chip_generic, msk: u32, flags: irq_gc_flags, clr: ::core::ffi::c_uint, set: ::core::ffi::c_uint);
}

#[cfg(CONFIG_GENERIC_IRQ_CHIP)]
pub unsafe fn devm_irq_alloc_generic_chip(dev: *mut device, name: *const ::core::ffi::c_char, num_ct: ::core::ffi::c_int, irq_base: ::core::ffi::c_uint, reg_base: *mut ::core::ffi::c_void, handler: irq_flow_handler_t) -> *mut irq_chip_generic {
    let gc = devm_kzalloc(dev, core::mem::size_of::<irq_chip_generic>() + core::mem::size_of::<irq_chip_type>() * num_ct as usize, 0) as *mut irq_chip_generic;
    if !gc.is_null() { irq_init_generic_chip(gc, name, num_ct, irq_base, reg_base, handler); }
    gc
}

#[cfg(CONFIG_GENERIC_IRQ_CHIP)]
#[repr(C)]
struct irq_generic_chip_devres { gc: *mut irq_chip_generic, msk: u32, clr: ::core::ffi::c_uint, set: ::core::ffi::c_uint }

#[cfg(CONFIG_GENERIC_IRQ_CHIP)]
unsafe extern "C" fn devm_irq_remove_generic_chip(_dev: *mut device, res: *mut ::core::ffi::c_void) {
    let this = res as *mut irq_generic_chip_devres;
    irq_remove_generic_chip((*this).gc, (*this).msk, (*this).clr, (*this).set);
}

#[cfg(CONFIG_GENERIC_IRQ_CHIP)]
pub unsafe fn devm_irq_setup_generic_chip(dev: *mut device, gc: *mut irq_chip_generic, msk: u32, flags: irq_gc_flags, clr: ::core::ffi::c_uint, set: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let dr = devres_alloc(devm_irq_remove_generic_chip, core::mem::size_of::<irq_generic_chip_devres>(), 0);
    if dr.is_null() { return -12; }
    irq_setup_generic_chip(gc, msk, flags, clr, set);
    let dr = dr as *mut irq_generic_chip_devres;
    (*dr).gc = gc; (*dr).msk = msk; (*dr).clr = clr; (*dr).set = set;
    devres_add(dev, dr as *mut _);
    0
}

#[cfg(CONFIG_IRQ_DOMAIN)]
#[repr(C)]
pub struct irq_domain;
#[cfg(CONFIG_IRQ_DOMAIN)]
#[repr(C)]
pub struct irq_domain_info;
#[cfg(CONFIG_IRQ_DOMAIN)]
unsafe extern "C" {
    fn irq_domain_remove(domain: *mut irq_domain);
    fn irq_domain_instantiate(info: *const irq_domain_info) -> *mut irq_domain;
}
#[cfg(CONFIG_IRQ_DOMAIN)]
unsafe extern "C" fn devm_irq_domain_remove(_dev: *mut device, res: *mut ::core::ffi::c_void) {
    irq_domain_remove(*(res as *mut *mut irq_domain));
}
#[cfg(CONFIG_IRQ_DOMAIN)]
pub unsafe fn devm_irq_domain_instantiate(dev: *mut device, info: *const irq_domain_info) -> *mut irq_domain {
    let dr = devres_alloc(devm_irq_domain_remove, core::mem::size_of::<*mut irq_domain>(), 0);
    if dr.is_null() { return (-12isize) as *mut irq_domain; }
    let domain = irq_domain_instantiate(info);
    if !domain.is_null() {
        *(dr as *mut *mut irq_domain) = domain;
        devres_add(dev, dr);
    } else { devres_free(dr); }
    domain
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
