// SPDX-License-Identifier: GPL-2.0
/* Translated from leon_kernel.c. Linux and architecture dependencies are external. */

use core::ffi::c_void;

extern "C" {
    static mut leon3_irqctrl_regs: *mut leon3_irqctrl_regs_map;
    static mut leon3_gptimer_regs: *mut leon3_gptimer_regs_map;
    static mut leondebug_irq_disable: i32;
    static mut leon_debug_irqout: i32;
    static mut amba_system_id: usize;
    static mut leon3_gptimer_irq: usize;
    static mut sparc_leon_eirq: u32;
}

#[repr(C)] pub struct leon3_irqctrl_regs_map { pub mask: [u32; 32], pub iclear: u32, pub intid: [u32; 32], pub icsel: [u32; 32], pub mpstatus: u32 }
#[repr(C)] pub struct leon3_gptimer_regs_map { pub e: [leon3_gptimer_entry; 8], pub config: u32 }
#[repr(C)] pub struct leon3_gptimer_entry { pub rld: u32, pub val: u32, pub ctrl: u32 }
#[repr(C)] pub struct irq_desc { pub handle_irq: Option<unsafe extern "C" fn(*mut irq_desc)> }
#[repr(C)] pub struct irq_bucket { pub irq: u32 }
#[repr(C)] pub struct irq_data { pub chip_data: *mut c_void, pub irq: u32 }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct property { pub value: *mut c_void }
#[repr(C)] pub struct clock_event_device { pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)> }
#[repr(C)] pub struct irq_chip { pub name: *const u8 }
pub type irq_flow_handler_t = Option<unsafe extern "C" fn(*mut irq_desc)>;
pub type irqreturn_t = i32;

extern "C" {
    static mut irq_map: *mut *mut irq_bucket;
    static mut boot_cpu_id: i32;
    static mut nr_cpu_ids: u32;
    static mut sparc_config: sparc_config_t;
    static mut dummy_master_l10_counter: u32;
    static mut master_l10_counter: *mut u32;
    static mut patchme_maybe_smp_msg: [u32; 1];
    static mut local_ops: *mut local_ops_t;
    static mut cpu_online_mask: *const cpumask;
}
#[repr(C)] pub struct sparc_config_t { pub get_cycles_offset: Option<unsafe extern "C" fn() -> u32>, pub cs_period: u32, pub features: u32, pub init_timers: Option<unsafe extern "C" fn()>, pub build_device_irq: Option<unsafe extern "C" fn(u32) -> u32>, pub clock_rate: u32, pub clear_clock_irq: Option<unsafe extern "C" fn()>, pub load_profile_irq: Option<unsafe extern "C" fn(i32, u32)> }
#[repr(C)] pub struct local_ops_t { pub cache_all: Option<unsafe extern "C" fn()> }

const LEON_DO_ACK_HW: usize = 1;
const IRQ_SET_MASK_OK: i32 = 0;
const IRQ_HANDLED: i32 = 1;
const FEAT_L10_CLOCKSOURCE: u32 = 1;
const FEAT_L10_CLOCKEVENT: u32 = 2;
const HZ: u32 = 100;
const LEON3_GPTIMER_CTRL_PENDING: u32 = 1;
const LEON3_GPTIMER_SEPIRQ: u32 = 0;
const LEON3_GPTIMER_TIMERS: u32 = 8;
const LEON3_GPTIMER_EN: u32 = 1;
const LEON3_GPTIMER_RL: u32 = 2;
const LEON3_GPTIMER_LD: u32 = 4;
const LEON3_GPTIMER_IRQEN: u32 = 8;

static mut leon_irq_lock: usize = 0;
static mut leon3_gptimer_idx: usize = 0;
static mut leon3_gptimer_ackmask: u32 = 0;

extern "C" {
    fn sparc_leon3_cpuid() -> i32; fn generic_handle_irq(u32); fn leon_build_device_irq(u32, irq_flow_handler_t, *const u8, i32) -> u32;
    fn irq_link(u32); fn irq_unlink(u32); fn irq_alloc(u32,u32)->u32; fn irq_to_desc(u32)->*mut irq_desc;
    fn handle_bad_irq(*mut irq_desc); fn handle_simple_irq(*mut irq_desc); fn handle_percpu_irq(*mut irq_desc);
    fn irq_set_chip_and_handler_name(u32,*mut irq_chip,irq_flow_handler_t,*const u8); fn irq_set_chip_data(u32,*mut c_void); fn irq_get_chip_data(u32)->*mut c_void;
    fn irq_data_get_affinity_mask(*mut irq_data)->*const cpumask; fn spin_lock_irqsave(*mut usize,*mut usize); fn spin_unlock_irqrestore(*mut usize,usize);
    fn cpumask_first_and(*const cpumask,*const cpumask)->u32; fn cpumask_subset(*const cpumask,*const cpumask)->bool;
    fn of_find_node_by_path(*const u8)->*mut device_node; fn of_find_node_by_name(*mut device_node,*const u8)->*mut device_node; fn of_find_property(*mut device_node,*const u8,*mut i32)->*mut property;
    fn request_irq(u32,*const c_void,u32,*const u8,*mut c_void)->i32; fn timer_interrupt(i32,*mut c_void); fn irq_enter(); fn irq_exit(); fn smp_processor_id()->i32;
    fn local_irq_save(*mut usize); fn local_irq_restore(usize); fn prom_halt(); fn printk(*const u8,...); fn pr_warn(*const u8,...); fn pr_err(*const u8,...); fn BUG()->!;
}

#[inline] unsafe fn leon_eirq_get(cpu: i32) -> u32 { ((*leon3_irqctrl_regs).intid[cpu as usize]) & 0x1f }
unsafe fn leon_handle_ext_irq(_desc: *mut irq_desc) { let eirq=leon_eirq_get(sparc_leon3_cpuid()); let p=*irq_map.add(eirq as usize); if (eirq&0x10)!=0 && !p.is_null() && (*p).irq!=0 { generic_handle_irq((*p).irq); } }
unsafe fn leon_eirq_setup(eirq:u32) { if eirq<1 || eirq>0xf { return; } let veirq=leon_build_device_irq(eirq,Some(leon_handle_ext_irq),b"extirq\0".as_ptr(),0); irq_link(veirq); let old=(*leon3_irqctrl_regs).mask[boot_cpu_id as usize]; (*leon3_irqctrl_regs).mask[boot_cpu_id as usize]=old|(1<<eirq); sparc_leon_eirq=eirq; }
#[no_mangle] pub unsafe extern "C" fn leon_get_irqmask(irq:u32)->usize { if irq==0 || ((irq>0xf)&&sparc_leon_eirq==0) || ((irq>0x1f)&&sparc_leon_eirq!=0) { 0 } else { 1usize << irq } }
unsafe fn leon_unmask_irq(data:*mut irq_data) { let mask=(*data).chip_data as usize; let cpu=boot_cpu_id; (*leon3_irqctrl_regs).mask[cpu as usize]|=mask as u32; }
unsafe fn leon_mask_irq(data:*mut irq_data) { let mask=(*data).chip_data as usize; let cpu=boot_cpu_id; (*leon3_irqctrl_regs).mask[cpu as usize]&=!(mask as u32); }
unsafe fn leon_set_affinity(data:*mut irq_data,_dest:*const cpumask,_force:bool)->i32 { let _=data; IRQ_SET_MASK_OK }
unsafe fn leon_startup_irq(data:*mut irq_data)->u32 { irq_link((*data).irq); leon_unmask_irq(data); 0 }
unsafe fn leon_shutdown_irq(data:*mut irq_data) { leon_mask_irq(data); irq_unlink((*data).irq); }
unsafe fn leon_eoi_irq(data:*mut irq_data) { let mask=(*data).chip_data as usize; if mask&LEON_DO_ACK_HW!=0 { (*leon3_irqctrl_regs).iclear=(mask&!LEON_DO_ACK_HW) as u32; } }
static mut leon_irq: irq_chip = irq_chip { name:b"leon\0".as_ptr() };
#[no_mangle] pub unsafe extern "C" fn leon_build_device_irq(real_irq:u32,flow_handler:irq_flow_handler_t,name:*const u8,do_ack:i32)->u32 { let mut mask=leon_get_irqmask(real_irq); if mask==0{return 0} let irq=irq_alloc(real_irq,real_irq); if irq==0{return 0} if do_ack!=0 {mask|=LEON_DO_ACK_HW;} let desc=irq_to_desc(irq); if desc.is_null() || (*desc).handle_irq.is_none() { irq_set_chip_and_handler_name(irq,&mut leon_irq,flow_handler,name); irq_set_chip_data(irq,mask as *mut c_void); } irq }
unsafe fn _leon_build_device_irq(_op:*mut platform_device,real_irq:u32)->u32 { leon_build_device_irq(real_irq,Some(handle_simple_irq),b"edge\0".as_ptr(),0) }
#[no_mangle] pub unsafe extern "C" fn leon_update_virq_handling(virq:u32,flow_handler:irq_flow_handler_t,name:*const u8,do_ack:i32) { let mut mask=irq_get_chip_data(virq) as usize & !LEON_DO_ACK_HW; if do_ack!=0 {mask|=LEON_DO_ACK_HW;} irq_set_chip_and_handler_name(virq,&mut leon_irq,flow_handler,name); irq_set_chip_data(virq,mask as *mut c_void); }
unsafe fn leon_cycles_offset()->u32 { let e=&(*leon3_gptimer_regs).e[leon3_gptimer_idx]; let rld=e.rld; let val=e.val; let ctrl=e.ctrl; if ctrl&LEON3_GPTIMER_CTRL_PENDING!=0 {2*rld-val} else {rld-val} }
unsafe fn leon_clear_clock_irq() { let e=&mut (*leon3_gptimer_regs).e[leon3_gptimer_idx]; e.ctrl &= leon3_gptimer_ackmask; }
unsafe fn leon_load_profile_irq(_cpu:i32,_limit:u32) {}
#[no_mangle] pub unsafe extern "C" fn leon_init_timers() {
    sparc_config.get_cycles_offset=Some(leon_cycles_offset); sparc_config.cs_period=1_000_000/HZ; sparc_config.features|=FEAT_L10_CLOCKSOURCE;
    leondebug_irq_disable=0; leon_debug_irqout=0; master_l10_counter=&mut dummy_master_l10_counter; dummy_master_l10_counter=0;
    let root=of_find_node_by_path(b"/ambapp0\0".as_ptr()); if root.is_null(){BUG()}
    let mut len=0; let pp=of_find_property(root,b"systemid\0".as_ptr(),&mut len); if !pp.is_null(){amba_system_id=*( (*pp).value as *mut usize );}
    let mut np=of_find_node_by_name(root,b"GAISLER_IRQMP\0".as_ptr()); if np.is_null(){np=of_find_node_by_name(root,b"01_00d\0".as_ptr()); if np.is_null(){BUG()}}
    let pp=of_find_property(np,b"reg\0".as_ptr(),&mut len); if pp.is_null(){BUG()} leon3_irqctrl_regs=*( (*pp).value as *mut *mut leon3_irqctrl_regs_map );
    let mut nnp=root; loop { np=of_find_node_by_name(nnp,b"GAISLER_GPTIMER\0".as_ptr()); if np.is_null(){np=of_find_node_by_name(nnp,b"01_011\0".as_ptr()); if np.is_null(){BUG()}}
        let mut ampopts=0i32; let pp=of_find_property(np,b"ampopts\0".as_ptr(),&mut len); if !pp.is_null(){ampopts=*( (*pp).value as *mut i32 ); if ampopts==0 {nnp=np;continue;}}
        leon3_gptimer_idx=(ampopts as usize)&7; let pp=of_find_property(np,b"reg\0".as_ptr(),&mut len); if !pp.is_null(){leon3_gptimer_regs=*( (*pp).value as *mut *mut leon3_gptimer_regs_map );} let pp=of_find_property(np,b"interrupts\0".as_ptr(),&mut len); if !pp.is_null(){leon3_gptimer_irq=*( (*pp).value as *mut u32 ) as usize;} break;
    }
    if leon3_gptimer_regs.is_null()||leon3_irqctrl_regs.is_null()||leon3_gptimer_irq==0 {BUG()}
    let e=&mut (*leon3_gptimer_regs).e[leon3_gptimer_idx]; e.ctrl|=LEON3_GPTIMER_CTRL_PENDING; leon3_gptimer_ackmask=if e.ctrl&LEON3_GPTIMER_CTRL_PENDING!=0 {!LEON3_GPTIMER_CTRL_PENDING}else{!0}; e.val=0; e.rld=(1_000_000/HZ)-1; e.ctrl=0;
    let eirq=(((*leon3_irqctrl_regs).mpstatus>>16)&0xf) as u32; if eirq!=0 {leon_eirq_setup(eirq)}
    let config=(*leon3_gptimer_regs).config; if config&(1<<LEON3_GPTIMER_SEPIRQ)!=0 {leon3_gptimer_irq+=leon3_gptimer_idx;} else if config&LEON3_GPTIMER_TIMERS>1 {pr_warn(b"GPTIMER uses shared irqs, using other timers of the same core will fail.\n\0".as_ptr());}
    let irq=_leon_build_device_irq(core::ptr::null_mut(),leon3_gptimer_irq as u32); if request_irq(irq,timer_interrupt as *const c_void,0x10,b"timer\0".as_ptr(),core::ptr::null_mut())!=0 {pr_err(b"Unable to attach timer IRQ%d\n\0".as_ptr(),irq);prom_halt();} e.ctrl=LEON3_GPTIMER_EN|LEON3_GPTIMER_RL|LEON3_GPTIMER_LD|LEON3_GPTIMER_IRQEN;
}
#[no_mangle] pub unsafe extern "C" fn leon_init_IRQ() { sparc_config.init_timers=Some(leon_init_timers); sparc_config.build_device_irq=Some(_leon_build_device_irq); sparc_config.clock_rate=1_000_000; sparc_config.clear_clock_irq=Some(leon_clear_clock_irq); sparc_config.load_profile_irq=Some(leon_load_profile_irq); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
