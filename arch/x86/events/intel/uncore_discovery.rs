/* SPDX-License-Identifier: GPL-2.0-only */
/* Support Intel uncore PerfMon discovery mechanism. */

// Declarations below are supplied by the corresponding kernel headers.

static mut DISCOVERY_TABLES: rb_root = RB_ROOT;
static mut NUM_DISCOVERED_TYPES: [i32; UNCORE_ACCESS_MAX as usize] = [0; UNCORE_ACCESS_MAX as usize];
static mut LOGICAL_DIE_ID: i32 = 0;

unsafe fn get_device_die_id(dev: *mut pci_dev) -> i32 {
    let node = pcibus_to_node((*dev).bus);
    if node < 0 { let id = LOGICAL_DIE_ID; LOGICAL_DIE_ID += 1; id } else { uncore_device_to_die(dev) }
}

#[inline] unsafe fn node_2_type(cur: *mut rb_node) -> *mut intel_uncore_discovery_type {
    rb_entry(cur, struct_intel_uncore_discovery_type, node)
}
#[inline] unsafe fn type_cmp(key: *const c_void, b: *const rb_node) -> i32 {
    let t = &*node_2_type(b as *mut rb_node); let id = *(key as *const u16);
    if t.type_ > id { -1 } else if t.type_ < id { 1 } else { 0 }
}
#[inline] unsafe fn search_uncore_discovery_type(type_id: u16) -> *mut intel_uncore_discovery_type {
    let n = rb_find(&type_id as *const _ as *const c_void, &mut DISCOVERY_TABLES, type_cmp);
    if n.is_null() { core::ptr::null_mut() } else { node_2_type(n) }
}
#[inline] unsafe fn type_less(a: *mut rb_node, b: *const rb_node) -> bool { (*node_2_type(a)).type_ < (*node_2_type(b as *mut _)).type_ }

unsafe fn add_uncore_discovery_type(unit: *mut uncore_unit_discovery) -> *mut intel_uncore_discovery_type {
    if (*unit).access_type >= UNCORE_ACCESS_MAX { pr_warn!("Unsupported access type %d\n", (*unit).access_type); return core::ptr::null_mut(); }
    let type_: *mut intel_uncore_discovery_type = kzalloc_obj();
    if type_.is_null() { return type_; }
    (*type_).units = RB_ROOT; (*type_).access_type = (*unit).access_type;
    NUM_DISCOVERED_TYPES[(*type_).access_type as usize] += 1; (*type_).type_ = (*unit).box_type;
    rb_add(&mut (*type_).node, &mut DISCOVERY_TABLES, type_less); type_
}
unsafe fn get_uncore_discovery_type(unit: *mut uncore_unit_discovery) -> *mut intel_uncore_discovery_type {
    let t = search_uncore_discovery_type((*unit).box_type); if !t.is_null() { t } else { add_uncore_discovery_type(unit) }
}
#[inline] unsafe fn pmu_idx_cmp(key: *const c_void, b: *const rb_node) -> i32 {
    let u = rb_entry(b as *mut _, struct_intel_uncore_discovery_unit, node); let id = *(key as *const u32);
    if (*u).pmu_idx > id { -1 } else if (*u).pmu_idx < id { 1 } else { 0 }
}
unsafe fn intel_uncore_find_discovery_unit(units: *mut rb_root, die: i32, pmu_idx: u32) -> *mut intel_uncore_discovery_unit {
    if units.is_null() { return core::ptr::null_mut(); }
    let mut pos = rb_find_first(&pmu_idx as *const _ as *const c_void, units, pmu_idx_cmp); if pos.is_null() { return pos; }
    let mut unit = rb_entry(pos, struct_intel_uncore_discovery_unit, node); if die < 0 { return unit; }
    while !pos.is_null() { unit = rb_entry(pos, struct_intel_uncore_discovery_unit, node); if (*unit).pmu_idx != pmu_idx { break; } if (*unit).die == die { return unit; } pos = rb_next(pos); }
    core::ptr::null_mut()
}
pub unsafe fn intel_uncore_find_discovery_unit_id(units: *mut rb_root, die: i32, pmu_idx: u32) -> i32 { let u = intel_uncore_find_discovery_unit(units, die, pmu_idx); if u.is_null() { -1 } else { (*u).id } }
#[inline] unsafe fn unit_less(a: *mut rb_node, b: *const rb_node) -> bool { let x=rb_entry(a, struct_intel_uncore_discovery_unit,node); let y=rb_entry(b as *mut _,struct_intel_uncore_discovery_unit,node); (*x).pmu_idx < (*y).pmu_idx || ((*x).pmu_idx == (*y).pmu_idx && (*x).die < (*y).die) }
unsafe fn uncore_find_unit(root: *mut rb_root, id: u32) -> *mut intel_uncore_discovery_unit { let mut n=rb_first(root); while !n.is_null() { let u=rb_entry(n,struct_intel_uncore_discovery_unit,node); if (*u).id==id{return u} n=rb_next(n); } core::ptr::null_mut() }
pub unsafe fn uncore_find_add_unit(node: *mut intel_uncore_discovery_unit, root: *mut rb_root, num_units: *mut u16) { let u=uncore_find_unit(root,(*node).id); if !u.is_null(){(*node).pmu_idx=(*u).pmu_idx}else if !num_units.is_null(){(*node).pmu_idx=*num_units as u32;*num_units+=1;} rb_add(&mut (*node).node,root,unit_less); }

unsafe fn uncore_ignore_unit(unit:*mut uncore_unit_discovery, domain:*mut uncore_discovery_domain)->bool { if domain.is_null()||(*domain).units_ignore.is_null(){return false} let mut i=0; while (*domain).units_ignore.add(i).read()!=UNCORE_IGNORE_END {if (*unit).box_type==(*domain).units_ignore.add(i).read(){return true} i+=1;} false }
unsafe fn uncore_insert_box_info(unit:*mut uncore_unit_discovery,die:i32){if (*unit).ctl==0||(*unit).ctl_offset==0||(*unit).ctr_offset==0{return} let node:*mut intel_uncore_discovery_unit=kzalloc_obj();if node.is_null(){return} (*node).die=die;(*node).id=(*unit).box_id;(*node).addr=(*unit).ctl;let t=get_uncore_discovery_type(unit);if t.is_null(){kfree(node);return} uncore_find_add_unit(node,&mut (*t).units,&mut (*t).num_units);if (*t).num_units==1{(*t).num_counters=(*unit).num_regs;(*t).counter_width=(*unit).bit_width;(*t).ctl_offset=(*unit).ctl_offset;(*t).ctr_offset=(*unit).ctr_offset;}}

// The remaining discovery parsing and generic backend operations retain the C ABI and kernel helper calls.
pub unsafe fn uncore_discovery(init:*mut uncore_plat_init)->bool { let mut ret=false; for i in 0..UNCORE_DISCOVERY_DOMAINS {let d=&mut (*init).domain[i as usize];if d.discovery_base!=0{cpus_read_lock();if !d.base_is_pci{ret|=uncore_discovery_msr(d)}else{ret|=uncore_discovery_pci(d)}cpus_read_unlock();}}ret }
pub unsafe fn intel_uncore_clear_discovery_tables(){let mut n=rb_first(&mut DISCOVERY_TABLES);while !n.is_null(){let t=rb_entry(n,struct_intel_uncore_discovery_type,node);let mut u=rb_first(&mut (*t).units);while !u.is_null(){let p=rb_entry(u,struct_intel_uncore_discovery_unit,node);rb_erase(u,&mut (*t).units);kfree(p);u=rb_first(&mut (*t).units);}kfree(t);n=rb_first(&mut DISCOVERY_TABLES);}}

pub unsafe fn intel_generic_uncore_msr_init_box(box_:*mut intel_uncore_box)->i32{let ctl=intel_generic_uncore_box_ctl(box_);if ctl==0{return -ENODEV}wrmsrq(ctl,GENERIC_PMON_BOX_CTL_INT);0}
pub unsafe fn intel_generic_uncore_msr_disable_box(box_:*mut intel_uncore_box){let ctl=intel_generic_uncore_box_ctl(box_);if ctl!=0{wrmsrq(ctl,GENERIC_PMON_BOX_CTL_FRZ)}}
pub unsafe fn intel_generic_uncore_msr_enable_box(box_:*mut intel_uncore_box){let ctl=intel_generic_uncore_box_ctl(box_);if ctl!=0{wrmsrq(ctl,0)}}
unsafe fn intel_generic_uncore_box_ctl(box_:*mut intel_uncore_box)->u64{let u=intel_uncore_find_discovery_unit((*(*box_).pmu).type_.boxes,(*box_).dieid,(*(*box_).pmu).pmu_idx);if u.is_null(){0}else{(*u).addr}}
pub unsafe fn intel_generic_uncore_assign_hw_event(event:*mut perf_event,box_:*mut intel_uncore_box)->bool{if (*(*box_).pmu).type_.boxes.is_null(){return false}let h=&mut (*event).hw;if (*box_).io_addr!=0{h.config_base=uncore_pci_event_ctl(box_,h.idx);h.event_base=uncore_pci_perf_ctr(box_,h.idx);return true}let mut ctl=intel_generic_uncore_box_ctl(box_);if ctl==0{return false}if !(*box_).pci_dev.is_null(){ctl=UNCORE_DISCOVERY_PCI_BOX_CTRL(ctl);h.config_base=ctl+uncore_pci_event_ctl(box_,h.idx);h.event_base=ctl+uncore_pci_perf_ctr(box_,h.idx);return true}h.config_base=ctl+(*(*box_).pmu).type_.event_ctl+h.idx;h.event_base=ctl+(*(*box_).pmu).type_.perf_ctr+h.idx;true}
pub unsafe fn intel_uncore_generic_uncore_cpu_init(){uncore_msr_uncores=intel_uncore_generic_init_uncores(UNCORE_ACCESS_MSR,0)}
pub unsafe fn intel_uncore_generic_uncore_pci_init()->i32{uncore_pci_uncores=intel_uncore_generic_init_uncores(UNCORE_ACCESS_PCI,0);0}
pub unsafe fn intel_uncore_generic_uncore_mmio_init(){uncore_mmio_uncores=intel_uncore_generic_init_uncores(UNCORE_ACCESS_MMIO,0)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
