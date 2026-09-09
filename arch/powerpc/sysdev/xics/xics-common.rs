// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright 2011 IBM Corporation. */

// Kernel headers and build-time configuration are supplied by other units.

extern "C" {
    static mut icp_ops: *const IcpOps;
    static mut xics_default_server: c_uint;
    static mut xics_default_distrib_server: c_uint;
    static mut xics_interrupt_server_size: c_uint;
    static mut xics_cppr: PerCpu<XicsCppr>;
    static mut xics_host: *mut IrqDomain;
    static mut xics_ics: *mut Ics;
}

#[repr(C)] pub struct IcpOps { pub set_priority: Option<unsafe extern "C" fn(c_uint)>, pub teardown_cpu: Option<unsafe extern "C" fn()>, pub flush_ipi: Option<unsafe extern "C" fn()>, pub ipi_action: *mut core::ffi::c_void, pub cause_ipi: *mut core::ffi::c_void, pub get_irq: *mut core::ffi::c_void, pub eoi: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct Ics { pub mask_unknown: Option<unsafe extern "C" fn(*mut Ics, c_uint)>, pub get_server: Option<unsafe extern "C" fn(*mut Ics, c_uint) -> c_long>, pub host_match: Option<unsafe extern "C" fn(*mut Ics, *mut DeviceNode) -> bool>, pub check: Option<unsafe extern "C" fn(*mut Ics, c_ulong) -> bool>, pub chip: *mut IrqChip }
#[repr(C)] pub struct XicsCppr { pub index: c_int }
#[repr(C)] pub struct IrqDomain { _private: [u8; 0] }
#[repr(C)] pub struct DeviceNode { _private: [u8; 0] }
#[repr(C)] pub struct IrqData { _private: [u8; 0] }
#[repr(C)] pub struct IrqDesc { _private: [u8; 0] }
#[repr(C)] pub struct IrqChip { pub name: *const c_char, pub irq_eoi: Option<unsafe extern "C" fn(*mut IrqData)>, pub irq_mask: Option<unsafe extern "C" fn(*mut IrqData)>, pub irq_unmask: Option<unsafe extern "C" fn(*mut IrqData)> }
#[repr(C)] pub struct PerCpu<T> { _private: [u8; 0], _marker: core::marker::PhantomData<T> }
type c_int = i32; type c_uint = u32; type c_ulong = usize; type c_long = isize; type c_char = i8;

extern "C" {
    fn of_get_cpu_node(c: c_uint, x: *mut c_void) -> *mut DeviceNode; fn get_hard_smp_processor_id(c: c_uint) -> c_uint; fn of_get_property(n: *mut DeviceNode, p: *const c_char, l: *mut c_uint) -> *const u32; fn of_node_put(n: *mut DeviceNode); fn be32_to_cpu(x: u32) -> u32;
    fn rtas_indicator_present(a: c_uint, b: *mut c_void) -> bool; fn rtas_set_indicator_fast(a: c_uint,b: c_int,c: c_uint)->c_int; fn irq_create_mapping(d:*mut IrqDomain,h:c_ulong)->c_uint; fn request_irq(a:c_uint,b:*mut c_void,c:c_ulong,n:*const c_char,p:*mut c_void)->c_int;
    fn smp_processor_id()->c_uint; fn hard_smp_processor_id()->c_uint; fn cpu_online(c:c_uint)->bool; fn mdelay(c:c_uint); fn irq_set_affinity(a:c_uint,m:*const c_void)->c_int; fn irq_domain_get_irq_data(d:*mut IrqDomain,v:c_uint)->*mut IrqData; fn irqd_to_hwirq(d:*mut IrqData)->c_ulong;
    fn irq_clear_status_flags(v:c_uint,f:c_ulong); fn irq_set_chip_and_handler(v:c_uint,c:*mut IrqChip,h:*mut c_void); fn irq_domain_set_info(d:*mut IrqDomain,v:c_uint,h:c_ulong,c:*mut IrqChip, x:*mut Ics, y:*mut c_void,z:*mut c_void,q:*mut c_void); fn irqd_set_trigger_type(d:*mut IrqData,t:c_uint); fn xics_push_cppr(p:c_uint);
    fn irq_domain_alloc_named_fwnode(n:*const c_char)->*mut c_void; fn irq_domain_create_tree(f:*mut c_void,o:*const IrqDomainOps,p:*mut c_void)->*mut IrqDomain; fn irq_domain_free_fwnode(f:*mut c_void); fn irq_set_default_domain(d:*mut IrqDomain); fn of_find_compatible_node(a:*mut DeviceNode,b:*mut c_void,c:*const c_char)->*mut DeviceNode; fn firmware_has_feature(f:c_ulong)->bool; fn icp_hv_init()->c_int; fn icp_native_init()->c_int; fn icp_opal_init()->c_int; fn ics_rtas_init()->c_int; fn ics_opal_init()->c_int; fn ics_native_init()->c_int; fn xics_set_cpu_giq(s:c_uint,j:c_uint);
}

pub unsafe extern "C" fn xics_mask_unknown_vec(vec: c_uint) { pr_err(vec); if xics_ics.is_null(){return} ((*xics_ics).mask_unknown.unwrap())(xics_ics,vec); }
pub unsafe extern "C" fn xics_setup_cpu() { ((*icp_ops).set_priority.unwrap())(0xff); xics_set_cpu_giq(xics_default_distrib_server,1); }

pub unsafe extern "C" fn xics_update_irq_servers() { let np=of_get_cpu_node(0 as c_uint,core::ptr::null_mut()); if np.is_null(){return} let h=get_hard_smp_processor_id(0); xics_default_server=h; xics_default_distrib_server=h; let mut len=0; let r=of_get_property(np,b"ibm,ppc-interrupt-gserver#s\0".as_ptr() as _,&mut len); if r.is_null(){of_node_put(np);return} let mut j=0; while j<len as usize/core::mem::size_of::<c_int>() { if be32_to_cpu(*r.add(j))==h {*(&mut xics_default_distrib_server)=be32_to_cpu(*r.add(j+1));break} j+=2 } of_node_put(np); }

pub unsafe extern "C" fn xics_teardown_cpu(){ ((*xics_cppr_ptr()).as_mut().unwrap()).index=0; ((*icp_ops).set_priority.unwrap())(0); ((*icp_ops).teardown_cpu.unwrap())(); }
pub unsafe extern "C" fn xics_kexec_teardown_cpu(secondary:c_int){xics_teardown_cpu();((*icp_ops).flush_ipi.unwrap())();if secondary!=0{xics_set_cpu_giq(xics_default_distrib_server,0);}}

// The remaining domain callbacks preserve the C interfaces and operations.
#[repr(C)] pub struct IrqDomainOps { pub match_: Option<unsafe extern "C" fn(*mut IrqDomain,*mut DeviceNode,c_int)->c_int>, pub map: Option<unsafe extern "C" fn(*mut IrqDomain,c_uint,c_ulong)->c_int>, pub xlate: Option<unsafe extern "C" fn(*mut IrqDomain,*mut DeviceNode,*const u32,c_uint,*mut c_ulong,*mut c_uint)->c_int> }
unsafe fn xics_cppr_ptr()->*mut XicsCppr { core::ptr::null_mut() }
unsafe fn pr_err(_:c_uint){}

pub unsafe extern "C" fn xics_set_irq_type(d:*mut IrqData, mut flow_type:c_uint)->c_int { if flow_type==0 {flow_type=1;} if flow_type!=1 && flow_type!=2{return -22;} irqd_set_trigger_type(d,flow_type); 0 }
pub unsafe extern "C" fn xics_retrigger(_: *mut IrqData)->c_int { xics_push_cppr(0); 0 }

pub unsafe extern "C" fn xics_init() {
    let mut rc=-1; if firmware_has_feature(1){rc=icp_hv_init();} if rc<0 {rc=icp_native_init();if rc== -19{rc=icp_opal_init();}} if rc<0{return;}
    let _=ics_rtas_init(); xics_get_server_size(); xics_update_irq_servers(); xics_setup_cpu();
}
unsafe fn xics_get_server_size(){let n=of_find_compatible_node(core::ptr::null_mut(),core::ptr::null_mut(),b"ibm,ppc-xics\0".as_ptr() as _);if n.is_null(){return}let p=of_get_property(n,b"ibm,interrupt-server#-size\0".as_ptr() as _,core::ptr::null_mut());if !p.is_null(){xics_interrupt_server_size=be32_to_cpu(*p);}of_node_put(n);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
