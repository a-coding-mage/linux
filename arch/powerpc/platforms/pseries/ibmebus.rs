/*
 * IBM PowerPC IBM eBus Infrastructure Support.
 *
 * Copyright (c) 2005 IBM Corporation
 *  Joachim Fenkes <fenkes@de.ibm.com>
 *  Heiko J Schick <schickhj@de.ibm.com>
 *
 * All rights reserved.
 *
 * This source code is distributed under a dual license of GPL v2.0 and OpenIB
 * BSD.
 */

// Kernel headers from the original implementation are supplied by external dependencies.

static mut ibmebus_bus_device: device = device { init_name: "ibmebus" };

extern "C" {
    static ibmebus_bus_type: bus_type;
}

// These devices will automatically be added to the bus during init.
static ibmebus_matches: [of_device_id; 3] = [
    of_device_id { compatible: "IBM,lhca" },
    of_device_id { compatible: "IBM,lhea" },
    of_device_id { compatible: "" },
];

unsafe extern "C" fn ibmebus_alloc_coherent(_dev: *mut device, size: usize,
                                              dma_handle: *mut dma_addr_t,
                                              flag: gfp_t, _attrs: c_ulong) -> *mut c_void {
    let mem = kmalloc(size, flag);
    *dma_handle = mem as dma_addr_t;
    mem
}

unsafe extern "C" fn ibmebus_free_coherent(_dev: *mut device, _size: usize,
                                             vaddr: *mut c_void,
                                             _dma_handle: dma_addr_t, _attrs: c_ulong) {
    kfree(vaddr);
}

unsafe extern "C" fn ibmebus_map_phys(_dev: *mut device, phys: phys_addr_t,
                                        _size: usize, _direction: dma_data_direction,
                                        attrs: c_ulong) -> dma_addr_t {
    if attrs & DMA_ATTR_MMIO != 0 { return DMA_MAPPING_ERROR; }
    phys_to_virt(phys) as dma_addr_t
}

unsafe extern "C" fn ibmebus_unmap_phys(_dev: *mut device, _dma_addr: dma_addr_t,
                                          _size: usize, _direction: dma_data_direction,
                                          _attrs: c_ulong) {}

unsafe extern "C" fn ibmebus_map_sg(_dev: *mut device, sgl: *mut scatterlist,
                                      nents: c_int, _direction: dma_data_direction,
                                      _attrs: c_ulong) -> c_int {
    let mut sg = sgl;
    let mut i = 0;
    while i < nents {
        (*sg).dma_address = sg_virt(sg) as dma_addr_t;
        (*sg).dma_length = (*sg).length;
        sg = sg_next(sg);
        i += 1;
    }
    nents
}

unsafe extern "C" fn ibmebus_unmap_sg(_dev: *mut device, _sg: *mut scatterlist,
                                        _nents: c_int, _direction: dma_data_direction,
                                        _attrs: c_ulong) {}

unsafe extern "C" fn ibmebus_dma_supported(_dev: *mut device, mask: u64) -> c_int {
    (mask == DMA_BIT_MASK(64)) as c_int
}

unsafe extern "C" fn ibmebus_dma_get_required_mask(_dev: *mut device) -> u64 {
    DMA_BIT_MASK(64)
}

static ibmebus_dma_ops: dma_map_ops = dma_map_ops {
    alloc: Some(ibmebus_alloc_coherent), free: Some(ibmebus_free_coherent),
    map_sg: Some(ibmebus_map_sg), unmap_sg: Some(ibmebus_unmap_sg),
    dma_supported: Some(ibmebus_dma_supported),
    get_required_mask: Some(ibmebus_dma_get_required_mask),
    map_phys: Some(ibmebus_map_phys), unmap_phys: Some(ibmebus_unmap_phys),
};

unsafe extern "C" fn ibmebus_match_path(dev: *mut device, data: *const c_void) -> c_int {
    let dn = (*to_platform_device(dev)).dev.of_node;
    let tn = of_find_node_by_path(data);
    of_node_put(tn);
    (tn == dn) as c_int
}

unsafe extern "C" fn ibmebus_match_node(dev: *mut device, data: *const c_void) -> c_int {
    ((*to_platform_device(dev)).dev.of_node as *const c_void == data) as c_int
}

unsafe fn ibmebus_create_device(dn: *mut device_node) -> c_int {
    let dev = of_device_alloc(dn, core::ptr::null(), &mut ibmebus_bus_device);
    if dev.is_null() { return -ENOMEM; }
    (*dev).dev.bus = &ibmebus_bus_type as *const _ as *mut _;
    (*dev).dev.dma_ops = &ibmebus_dma_ops;
    let ret = of_device_add(dev);
    if ret != 0 { platform_device_put(dev); }
    ret
}

unsafe fn ibmebus_create_devices(matches: *const of_device_id) -> c_int {
    let root = of_find_node_by_path("/");
    let mut child = of_get_next_child(root, core::ptr::null_mut());
    let mut ret = 0;
    while !child.is_null() {
        let next = of_get_next_child(root, child);
        if !of_match_node(matches, child).is_null() {
            let dev = bus_find_device(&ibmebus_bus_type, core::ptr::null_mut(),
                                      child as *const c_void, Some(ibmebus_match_node));
            if !dev.is_null() { put_device(dev); }
            else {
                ret = ibmebus_create_device(child);
                if ret != 0 { printk(KERN_ERR, "%s: failed to create device (%i)", __func__, ret); of_node_put(child); break; }
            }
        }
        child = next;
    }
    of_node_put(root);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn ibmebus_register_driver(drv: *mut platform_driver) -> c_int {
    ibmebus_create_devices((*drv).driver.of_match_table);
    (*drv).driver.bus = &ibmebus_bus_type as *const _ as *mut _;
    driver_register(&mut (*drv).driver)
}

#[no_mangle]
pub unsafe extern "C" fn ibmebus_unregister_driver(drv: *mut platform_driver) {
    driver_unregister(&mut (*drv).driver);
}

#[no_mangle]
pub unsafe extern "C" fn ibmebus_request_irq(ist: u32, handler: irq_handler_t,
                                                irq_flags: c_ulong, devname: *const c_char,
                                                dev_id: *mut c_void) -> c_int {
    let irq = irq_create_mapping(core::ptr::null_mut(), ist);
    if irq == 0 { return -EINVAL; }
    request_irq(irq, handler, irq_flags, devname, dev_id)
}

#[no_mangle]
pub unsafe extern "C" fn ibmebus_free_irq(ist: u32, dev_id: *mut c_void) {
    let irq = irq_find_mapping(core::ptr::null_mut(), ist);
    free_irq(irq, dev_id);
    irq_dispose_mapping(irq);
}

unsafe fn ibmebus_chomp(input: *const c_char, count: usize) -> *mut c_char {
    let out = kmalloc(count + 1, GFP_KERNEL) as *mut c_char;
    if out.is_null() { return core::ptr::null_mut(); }
    memcpy(out as *mut c_void, input as *const c_void, count);
    *out.add(count) = 0;
    if *out.add(count - 1) == b'\n' as c_char { *out.add(count - 1) = 0; }
    out
}

unsafe extern "C" fn probe_store(_bus: *const bus_type, buf: *const c_char, count: usize) -> ssize_t {
    let path = ibmebus_chomp(buf, count);
    if path.is_null() { return -ENOMEM as ssize_t; }
    let dev = bus_find_device(&ibmebus_bus_type, core::ptr::null_mut(), path as *const c_void, Some(ibmebus_match_path));
    if !dev.is_null() { put_device(dev); kfree(path as *mut c_void); return -EEXIST as ssize_t; }
    let dn = of_find_node_by_path(path);
    let rc = if !dn.is_null() { let r = ibmebus_create_device(dn); of_node_put(dn); r } else { -ENODEV };
    kfree(path as *mut c_void);
    if rc != 0 { rc as ssize_t } else { count as ssize_t }
}

unsafe extern "C" fn remove_store(_bus: *const bus_type, buf: *const c_char, count: usize) -> ssize_t {
    let path = ibmebus_chomp(buf, count);
    if path.is_null() { return -ENOMEM as ssize_t; }
    let dev = bus_find_device(&ibmebus_bus_type, core::ptr::null_mut(), path as *const c_void, Some(ibmebus_match_path));
    if !dev.is_null() { of_device_unregister(to_platform_device(dev)); put_device(dev); kfree(path as *mut c_void); count as ssize_t }
    else { kfree(path as *mut c_void); -ENODEV as ssize_t }
}

unsafe extern "C" fn ibmebus_bus_bus_match(dev: *mut device, drv: *const device_driver) -> c_int {
    let matches = (*drv).of_match_table;
    if matches.is_null() { return 0; }
    (!of_match_device(matches, dev).is_null()) as c_int
}

unsafe extern "C" fn ibmebus_bus_device_probe(dev: *mut device) -> c_int {
    let drv = to_platform_driver((*dev).driver);
    let of_dev = to_platform_device(dev);
    if (*drv).probe.is_none() { return -ENODEV; }
    get_device(dev);
    let error = if of_driver_match_device(dev, (*dev).driver) { ((*drv).probe.unwrap())(of_dev) } else { -ENODEV };
    if error != 0 { put_device(dev); }
    error
}

unsafe extern "C" fn ibmebus_bus_device_remove(dev: *mut device) {
    let of_dev = to_platform_device(dev); let drv = to_platform_driver((*dev).driver);
    if !(*dev).driver.is_null() { if let Some(remove) = (*drv).remove { remove(of_dev); } }
}

unsafe extern "C" fn ibmebus_bus_device_shutdown(dev: *mut device) {
    let of_dev = to_platform_device(dev); let drv = to_platform_driver((*dev).driver);
    if !(*dev).driver.is_null() { if let Some(shutdown) = (*drv).shutdown { shutdown(of_dev); } }
}

unsafe extern "C" fn ibmebus_bus_modalias(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    of_device_uevent_modalias(dev, env)
}

static ibmebus_bus_type_definition: bus_type = bus_type { name: "ibmebus", uevent: Some(ibmebus_bus_modalias), match_: Some(ibmebus_bus_bus_match), probe: Some(ibmebus_bus_device_probe), remove: Some(ibmebus_bus_device_remove), shutdown: Some(ibmebus_bus_device_shutdown), ..unsafe { core::mem::zeroed() } };

unsafe extern "C" fn ibmebus_bus_init() -> c_int {
    let mut err = bus_register(&ibmebus_bus_type_definition);
    if err != 0 { return err; }
    err = device_register(&mut ibmebus_bus_device);
    if err != 0 { put_device(&mut ibmebus_bus_device); bus_unregister(&ibmebus_bus_type_definition); return err; }
    err = ibmebus_create_devices(ibmebus_matches.as_ptr());
    if err != 0 { device_unregister(&mut ibmebus_bus_device); bus_unregister(&ibmebus_bus_type_definition); }
    err
}

// Equivalent to machine_postcore_initcall(pseries, ibmebus_bus_init).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
