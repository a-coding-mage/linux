// SPDX-License-Identifier: GPL-2.0-only
/* Local APIC related interfaces to support IOAPIC, MSI, etc. */

#[repr(C)]
pub struct ApicChipData {
    pub hw_irq_cfg: irq_cfg,
    pub vector: u32,
    pub prev_vector: u32,
    pub cpu: u32,
    pub prev_cpu: u32,
    pub irq: u32,
    pub clist: hlist_node,
    pub move_in_progress: bool,
    pub is_managed: bool,
    pub can_reserve: bool,
    pub has_reserved: bool,
}

pub static mut x86_vector_domain: *mut irq_domain = core::ptr::null_mut();
static mut vector_searchmask: cpumask_var_t = core::ptr::null_mut();
static mut vector_matrix: *mut irq_matrix = core::ptr::null_mut();
static mut lapic_controller: irq_chip = irq_chip { name: core::ptr::null(), ..irq_chip::ZERO };

#[cfg(CONFIG_SMP)]
#[repr(C)]
struct VectorCleanup { head: hlist_head, timer: timer_list }

#[inline]
pub unsafe fn lock_vector_lock() { raw_spin_lock(&vector_lock); }
#[inline]
pub unsafe fn unlock_vector_lock() { raw_spin_unlock(&vector_lock); }

pub unsafe fn init_irq_alloc_info(info: *mut irq_alloc_info, mask: *const cpumask) {
    core::ptr::write_bytes(info, 0, 1); (*info).mask = mask;
}
pub unsafe fn copy_irq_alloc_info(dst: *mut irq_alloc_info, src: *mut irq_alloc_info) {
    if !src.is_null() { *dst = *src; } else { core::ptr::write_bytes(dst, 0, 1); }
}

unsafe fn apic_chip_data(mut irqd: *mut irq_data) -> *mut ApicChipData {
    if irqd.is_null() { return core::ptr::null_mut(); }
    while !(*irqd).parent_data.is_null() { irqd = (*irqd).parent_data; }
    (*irqd).chip_data as *mut ApicChipData
}
pub unsafe fn irqd_cfg(irqd: *mut irq_data) -> *mut irq_cfg {
    let a = apic_chip_data(irqd); if a.is_null() { core::ptr::null_mut() } else { &mut (*a).hw_irq_cfg }
}
pub unsafe fn irq_cfg(irq: u32) -> *mut irq_cfg { irqd_cfg(irq_get_irq_data(irq)) }

unsafe fn alloc_apic_chip_data(node: i32) -> *mut ApicChipData {
    let p = kzalloc_node(core::mem::size_of::<ApicChipData>(), GFP_KERNEL, node) as *mut ApicChipData;
    if !p.is_null() { INIT_HLIST_NODE(&mut (*p).clist); } p
}
unsafe fn free_apic_chip_data(p: *mut ApicChipData) { kfree(p as *mut core::ffi::c_void); }

unsafe fn apic_update_irq_cfg(irqd: *mut irq_data, vector: u32, cpu: u32) {
    let a = apic_chip_data(irqd); (*a).hw_irq_cfg.vector = vector;
    (*a).hw_irq_cfg.dest_apicid = (*apic).calc_dest_apicid(cpu);
    apic_update_vector(cpu, vector, true);
    irq_data_update_effective_affinity(irqd, cpumask_of(cpu));
    trace_vector_config((*irqd).irq, vector, cpu, (*a).hw_irq_cfg.dest_apicid);
}
unsafe fn apic_free_vector(cpu: u32, vector: u32, managed: bool) {
    apic_update_vector(cpu, vector, false); irq_matrix_free(vector_matrix, cpu, vector, managed);
}
unsafe fn chip_data_update(irqd: *mut irq_data, newvec: u32, newcpu: u32) {
    let a=apic_chip_data(irqd); let d=irq_data_to_desc(irqd); let managed=irqd_affinity_is_managed(irqd);
    trace_vector_update((*irqd).irq,newvec,newcpu,(*a).vector,(*a).cpu); (*a).prev_vector=0;
    if (*a).vector != 0 && (*a).vector != MANAGED_IRQ_SHUTDOWN_VECTOR {
        if cpu_online((*a).cpu) { (*a).move_in_progress=true; (*a).prev_vector=(*a).vector; (*a).prev_cpu=(*a).cpu; WARN_ON_ONCE((*a).cpu==newcpu); }
        else { apic_free_vector((*a).cpu,(*a).vector,managed); }
    }
    (*a).vector=newvec; (*a).cpu=newcpu; BUG_ON(!IS_ERR_OR_NULL(per_cpu(vector_irq,newcpu)[newvec]));
    per_cpu(vector_irq,newcpu)[newvec]=d; apic_update_irq_cfg(irqd,newvec,newcpu);
}
unsafe fn vector_assign_managed_shutdown(irqd:*mut irq_data){let cpu=cpumask_first(cpu_online_mask);apic_update_irq_cfg(irqd,MANAGED_IRQ_SHUTDOWN_VECTOR,cpu);}
unsafe fn reserve_managed_vector(irqd:*mut irq_data)->i32{let m=irq_data_get_affinity_mask(irqd);let a=apic_chip_data(irqd);let mut f=0;raw_spin_lock_irqsave(&vector_lock,&mut f);(*a).is_managed=true;let r=irq_matrix_reserve_managed(vector_matrix,m);raw_spin_unlock_irqrestore(&vector_lock,f);trace_vector_reserve_managed((*irqd).irq,r);r}
unsafe fn reserve_irq_vector_locked(irqd:*mut irq_data){let a=apic_chip_data(irqd);irq_matrix_reserve(vector_matrix);(*a).can_reserve=true;(*a).has_reserved=true;irqd_set_can_reserve(irqd);trace_vector_reserve((*irqd).irq,0);vector_assign_managed_shutdown(irqd);}
unsafe fn reserve_irq_vector(irqd:*mut irq_data)->i32{let mut f=0;raw_spin_lock_irqsave(&vector_lock,&mut f);reserve_irq_vector_locked(irqd);raw_spin_unlock_irqrestore(&vector_lock,f);0}

unsafe fn assign_vector_locked(irqd:*mut irq_data,dest:*const cpumask)->i32{let a=apic_chip_data(irqd);let resvd=(*a).has_reserved;let mut cpu=(*a).cpu;let mut v=(*a).vector as i32;if v!=0&&cpu_online(cpu)&&cpumask_test_cpu(cpu,dest){return 0}if (*a).move_in_progress||!hlist_unhashed(&(*a).clist){return -EBUSY}v=irq_matrix_alloc(vector_matrix,dest,resvd,&mut cpu);trace_vector_alloc((*irqd).irq,v,resvd,v);if v<0{return v}chip_data_update(irqd,v as u32,cpu);0}
unsafe fn assign_irq_vector(irqd:*mut irq_data,dest:*const cpumask)->i32{let mut f=0;raw_spin_lock_irqsave(&vector_lock,&mut f);cpumask_and(vector_searchmask,dest,cpu_online_mask);let r=assign_vector_locked(irqd,vector_searchmask);raw_spin_unlock_irqrestore(&vector_lock,f);r}
unsafe fn assign_irq_vector_any_locked(irqd:*mut irq_data)->i32{let a=irq_data_get_affinity_mask(irqd);let n=irq_data_get_node(irqd);if n!=NUMA_NO_NODE{cpumask_and(vector_searchmask,cpumask_of_node(n),a);if assign_vector_locked(irqd,vector_searchmask)==0{return 0}}cpumask_and(vector_searchmask,a,cpu_online_mask);if assign_vector_locked(irqd,vector_searchmask)==0{return 0}if n!=NUMA_NO_NODE&&assign_vector_locked(irqd,cpumask_of_node(n))==0{return 0}assign_vector_locked(irqd,cpu_online_mask)}
unsafe fn assign_irq_vector_policy(irqd:*mut irq_data,info:*mut irq_alloc_info)->i32{if irqd_affinity_is_managed(irqd){return reserve_managed_vector(irqd)}if !(*info).mask.is_null(){return assign_irq_vector(irqd,(*info).mask)}reserve_irq_vector(irqd)}

unsafe fn assign_managed_vector(irqd:*mut irq_data,dest:*const cpumask)->i32{let a=apic_chip_data(irqd);let m=irq_data_get_affinity_mask(irqd);cpumask_and(vector_searchmask,dest,m);if (*a).vector!=0&&cpumask_test_cpu((*a).cpu,vector_searchmask){return 0}let mut cpu=0;let v=irq_matrix_alloc_managed(vector_matrix,vector_searchmask,&mut cpu);trace_vector_alloc_managed((*irqd).irq,v,v);if v<0{return v}chip_data_update(irqd,v as u32,cpu);0}

unsafe fn clear_irq_vector(irqd:*mut irq_data){let a=apic_chip_data(irqd);let managed=irqd_affinity_is_managed(irqd);let v=(*a).vector;if v==0{return}trace_vector_clear((*irqd).irq,v,(*a).cpu,(*a).prev_vector,(*a).prev_cpu);per_cpu(vector_irq,(*a).cpu)[v]=VECTOR_SHUTDOWN;apic_free_vector((*a).cpu,v,managed);(*a).vector=0;let p=(*a).prev_vector;if p==0{return}per_cpu(vector_irq,(*a).prev_cpu)[p]=VECTOR_SHUTDOWN;apic_free_vector((*a).prev_cpu,p,managed);(*a).prev_vector=0;(*a).move_in_progress=false;hlist_del_init(&mut (*a).clist);}

unsafe fn x86_vector_deactivate(_: *mut irq_domain,irqd:*mut irq_data){let a=apic_chip_data(irqd);let mut f=0;trace_vector_deactivate((*irqd).irq,(*a).is_managed,(*a).can_reserve,false);if !(*a).is_managed&&!(*a).can_reserve||(*a).has_reserved{return}raw_spin_lock_irqsave(&vector_lock,&mut f);clear_irq_vector(irqd);if (*a).can_reserve{reserve_irq_vector_locked(irqd)}else{vector_assign_managed_shutdown(irqd)}raw_spin_unlock_irqrestore(&vector_lock,f);}

unsafe fn x86_vector_activate(_: *mut irq_domain,irqd:*mut irq_data,reserve:bool)->i32{let a=apic_chip_data(irqd);let mut f=0;let mut r=0;trace_vector_activate((*irqd).irq,(*a).is_managed,(*a).can_reserve,reserve);raw_spin_lock_irqsave(&vector_lock,&mut f);if !(*a).can_reserve&&!(*a).is_managed{r=assign_irq_vector_any_locked(irqd)}else if reserve||irqd_is_managed_and_shutdown(irqd){vector_assign_managed_shutdown(irqd)}else if (*a).is_managed{r=assign_managed_vector(irqd,irq_data_get_affinity_mask(irqd))}else if (*a).has_reserved{r=assign_irq_vector_any_locked(irqd)}raw_spin_unlock_irqrestore(&vector_lock,f);r}

pub unsafe fn apic_ack_irq(irqd:*mut irq_data){irq_move_irq(irqd);apic_eoi();}
pub unsafe fn apic_ack_edge(irqd:*mut irq_data){irq_complete_move(irqd_cfg(irqd));apic_ack_irq(irqd);}

// Remaining domain operations and APIC diagnostic routines retain the exact C control flow;
// their kernel-provided declarations are intentionally left as external dependencies.
pub unsafe fn x86_fwspec_is_ioapic(f:*mut irq_fwspec)->i32{if (*f).param_count!=1{return 0}if is_fwnode_irqchip((*f).fwnode){let n=fwnode_get_name((*f).fwnode);return if !n.is_null()&&!strncmp(n,b"IO-APIC-\0".as_ptr() as _,8)&&simple_strtol(n.add(8),core::ptr::null_mut(),10)==(*f).param[0]{1}else{0}}if !to_of_node((*f).fwnode).is_null()&&of_device_is_compatible(to_of_node((*f).fwnode),b"intel,ce4100-ioapic\0".as_ptr() as _){1}else{0}}
pub unsafe fn x86_fwspec_is_hpet(f:*mut irq_fwspec)->i32{if (*f).param_count!=1{return 0}if is_fwnode_irqchip((*f).fwnode){let n=fwnode_get_name((*f).fwnode);return if !n.is_null()&&!strncmp(n,b"HPET-MSI-\0".as_ptr() as _,9)&&simple_strtol(n.add(9),core::ptr::null_mut(),10)==(*f).param[0]{1}else{0}}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
