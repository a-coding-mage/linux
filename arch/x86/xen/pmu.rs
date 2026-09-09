// SPDX-License-Identifier: GPL-2.0
// The included kernel/Xen declarations are supplied by the surrounding translation.

const XENPMU_IRQ_PROCESSING: u8 = 1;
#[repr(C)]
struct xenpmu { xenpmu_data: *mut xen_pmu_data, flags: u8 }
static mut xenpmu_shared: xenpmu = xenpmu { xenpmu_data: core::ptr::null_mut(), flags: 0 };

const F15H_NUM_COUNTERS: i32 = 6;
const F10H_NUM_COUNTERS: i32 = 4;
static mut amd_counters_base: u32 = 0;
static mut amd_ctrls_base: u32 = 0;
static mut amd_msr_step: i32 = 0;
static mut k7_counters_mirrored: i32 = 0;
static mut amd_num_counters: i32 = 0;
const MSR_TYPE_COUNTER: i32 = 0;
const MSR_TYPE_CTRL: i32 = 1;
const MSR_TYPE_GLOBAL: i32 = 2;
const MSR_TYPE_ARCH_COUNTER: i32 = 3;
const MSR_TYPE_ARCH_CTRL: i32 = 4;
const PMU_GENERAL_NR_SHIFT: u32 = 8;
const PMU_GENERAL_NR_BITS: u32 = 8;
const PMU_GENERAL_NR_MASK: u32 = (((1 << PMU_GENERAL_NR_BITS) - 1) << PMU_GENERAL_NR_SHIFT);
const PMU_FIXED_NR_SHIFT: u32 = 0;
const PMU_FIXED_NR_BITS: u32 = 5;
const PMU_FIXED_NR_MASK: u32 = (((1 << PMU_FIXED_NR_BITS) - 1) << PMU_FIXED_NR_SHIFT);
const INTEL_PMC_TYPE_SHIFT: u32 = 30;
static mut intel_num_arch_counters: i32 = 0;
static mut intel_num_fixed_counters: i32 = 0;

#[inline] unsafe fn get_xenpmu_data() -> *mut xen_pmu_data { xenpmu_shared.xenpmu_data }
#[inline] unsafe fn get_xenpmu_flags() -> u8 { xenpmu_shared.flags }

unsafe fn xen_pmu_arch_init() {
    if boot_cpu_data.x86_vendor == X86_VENDOR_AMD {
        match boot_cpu_data.x86 {
            0x15 => { amd_num_counters=F15H_NUM_COUNTERS; amd_counters_base=MSR_F15H_PERF_CTR; amd_ctrls_base=MSR_F15H_PERF_CTL; amd_msr_step=2; k7_counters_mirrored=1; }
            _ => { amd_num_counters=F10H_NUM_COUNTERS; amd_counters_base=MSR_K7_PERFCTR0; amd_ctrls_base=MSR_K7_EVNTSEL0; amd_msr_step=1; k7_counters_mirrored=0; }
        }
    } else if boot_cpu_data.x86_vendor == X86_VENDOR_HYGON {
        amd_num_counters=F10H_NUM_COUNTERS; amd_counters_base=MSR_K7_PERFCTR0; amd_ctrls_base=MSR_K7_EVNTSEL0; amd_msr_step=1; k7_counters_mirrored=0;
    } else {
        let (mut eax,mut ebx,mut ecx,mut edx)=(0,0,0,0); cpuid(0xa,&mut eax,&mut ebx,&mut ecx,&mut edx);
        intel_num_arch_counters=((eax&PMU_GENERAL_NR_MASK)>>PMU_GENERAL_NR_SHIFT) as i32;
        intel_num_fixed_counters=((edx&PMU_FIXED_NR_MASK)>>PMU_FIXED_NR_SHIFT) as i32;
    }
}

unsafe fn get_fam15h_addr(addr:u32)->u32 { match addr {
    MSR_K7_PERFCTR0..=MSR_K7_PERFCTR3 => MSR_F15H_PERF_CTR+(addr-MSR_K7_PERFCTR0),
    MSR_K7_EVNTSEL0..=MSR_K7_EVNTSEL3 => MSR_F15H_PERF_CTL+(addr-MSR_K7_EVNTSEL0), _=>addr } }
unsafe fn is_amd_pmu_msr(msr:u32)->bool {
    if boot_cpu_data.x86_vendor!=X86_VENDOR_AMD && boot_cpu_data.x86_vendor!=X86_VENDOR_HYGON{return false}
    (msr>=MSR_F15H_PERF_CTL&&msr<MSR_F15H_PERF_CTR+(amd_num_counters*2) as u32)||(msr>=MSR_K7_EVNTSEL0&&msr<MSR_K7_PERFCTR0+amd_num_counters as u32)
}
unsafe fn is_intel_pmu_msr(msr:u32,typ:*mut i32,index:*mut i32)->bool {
    if boot_cpu_data.x86_vendor!=X86_VENDOR_INTEL&&boot_cpu_data.x86_vendor!=X86_VENDOR_CENTAUR&&boot_cpu_data.x86_vendor!=X86_VENDOR_ZHAOXIN{return false}
    match msr {
        MSR_CORE_PERF_FIXED_CTR_CTRL|MSR_IA32_DS_AREA|MSR_IA32_PEBS_ENABLE=>{*typ=MSR_TYPE_CTRL;true}
        MSR_CORE_PERF_GLOBAL_CTRL|MSR_CORE_PERF_GLOBAL_STATUS|MSR_CORE_PERF_GLOBAL_OVF_CTRL=>{*typ=MSR_TYPE_GLOBAL;true}
        _=>{if msr>=MSR_CORE_PERF_FIXED_CTR0&&msr<MSR_CORE_PERF_FIXED_CTR0+intel_num_fixed_counters as u32{*index=(msr-MSR_CORE_PERF_FIXED_CTR0)as i32;*typ=MSR_TYPE_COUNTER;return true} if msr>=MSR_P6_EVNTSEL0&&msr<MSR_P6_EVNTSEL0+intel_num_arch_counters as u32{*index=(msr-MSR_P6_EVNTSEL0)as i32;*typ=MSR_TYPE_ARCH_CTRL;return true} let p=msr&MSR_PMC_ALIAS_MASK;if p>=MSR_IA32_PERFCTR0&&p<MSR_IA32_PERFCTR0+intel_num_arch_counters as u32{*typ=MSR_TYPE_ARCH_COUNTER;*index=(p-MSR_IA32_PERFCTR0)as i32;return true} false}
    }
}

unsafe fn xen_intel_pmu_emulate(msr:u32,val:*mut u64,typ:i32,index:i32,is_read:bool)->bool {
    let d=get_xenpmu_data();if d.is_null()||get_xenpmu_flags()&XENPMU_IRQ_PROCESSING==0{return false}
    let c=&mut(*d).pmu.c.intel;let r:*mut u64=match msr{MSR_CORE_PERF_GLOBAL_OVF_CTRL=>&mut c.global_ovf_ctrl,MSR_CORE_PERF_GLOBAL_STATUS=>&mut c.global_status,MSR_CORE_PERF_GLOBAL_CTRL=>&mut c.global_ctrl,MSR_CORE_PERF_FIXED_CTR_CTRL=>&mut c.fixed_ctrl,_=>match typ{MSR_TYPE_COUNTER=>c.fixed_counters.as_mut_ptr().add(index as usize),MSR_TYPE_ARCH_COUNTER=>&mut c.arch_counters[index as usize].counter,MSR_TYPE_ARCH_CTRL=>&mut c.arch_counters[index as usize].control,_=>return false}};
    if is_read{*val=*r}else{*r=*val;if msr==MSR_CORE_PERF_GLOBAL_OVF_CTRL{c.global_status&=!*val}}true
}
unsafe fn xen_amd_pmu_emulate(msr0:u32,val:*mut u64,is_read:bool)->bool {
    let d=get_xenpmu_data();if d.is_null()||get_xenpmu_flags()&XENPMU_IRQ_PROCESSING==0{return false}
    let mut msr=msr0;if k7_counters_mirrored!=0&&msr>=MSR_K7_EVNTSEL0&&msr<=MSR_K7_PERFCTR3{msr=get_fam15h_addr(msr)}
    let c=&mut(*d).pmu.c.amd;let mut off=0;for i in 0..amd_num_counters{if msr==amd_ctrls_base+off{let r=c.ctrls.as_mut_ptr().add(i as usize);if is_read{*val=*r}else{*r=*val};return true}if msr==amd_counters_base+off{let r=c.counters.as_mut_ptr().add(i as usize);if is_read{*val=*r}else{*r=*val};return true}off+=amd_msr_step}false
}
pub unsafe fn pmu_msr_chk_emulated(msr:u32,val:*mut u64,is_read:bool)->bool{let(mut typ,mut index)=(0,0);if is_amd_pmu_msr(msr){xen_amd_pmu_emulate(msr,val,is_read)}else if is_intel_pmu_msr(msr,&mut typ,&mut index){xen_intel_pmu_emulate(msr,val,typ,index,is_read)}else{false}}
unsafe fn xen_amd_read_pmc(counter:i32)->u64{let d=get_xenpmu_data();if d.is_null()||get_xenpmu_flags()&XENPMU_IRQ_PROCESSING==0{let mut v=0;native_read_msr_safe(amd_counters_base+(counter*amd_msr_step)as u32,&mut v);v}else{(*d).pmu.c.amd.counters[counter as usize]}}
unsafe fn xen_intel_read_pmc(counter:i32)->u64{let d=get_xenpmu_data();if d.is_null()||get_xenpmu_flags()&XENPMU_IRQ_PROCESSING==0{let mut v=0;let m=if counter&(1<<INTEL_PMC_TYPE_SHIFT)!=0{MSR_CORE_PERF_FIXED_CTR0+(counter as u32&0xffff)}else{MSR_IA32_PERFCTR0+counter as u32};native_read_msr_safe(m,&mut v);v}else if counter&(1<<INTEL_PMC_TYPE_SHIFT)!=0{(*d).pmu.c.intel.fixed_counters[(counter as u32&0xffff)as usize]}else{(*d).pmu.c.intel.arch_counters[counter as usize].counter}}
pub unsafe fn xen_read_pmc(counter:i32)->u64{if boot_cpu_data.x86_vendor!=X86_VENDOR_INTEL{xen_amd_read_pmc(counter)}else{xen_intel_read_pmc(counter)}}

// The following declarations preserve the remaining source interfaces; their kernel operations are external.
pub static mut is_xen_pmu: bool=false;

pub unsafe fn pmu_apic_update(val:u32)->i32 {
    let d=get_xenpmu_data(); if d.is_null(){pr_warn_once("%s: pmudata not initialized\n",__func__);return -EINVAL}
    (*d).pmu.l.lapic_lvtpc=val;
    if get_xenpmu_flags()&XENPMU_IRQ_PROCESSING!=0{return 0}
    HYPERVISOR_xenpmu_op(XENPMU_lvtpc_set,core::ptr::null_mut())
}

unsafe fn xen_guest_state()->u32 {
    let d=get_xenpmu_data();let mut state=0;if d.is_null(){pr_warn_once("%s: pmudata not initialized\n",__func__);return state}
    if !xen_initial_domain()||(*d).domain_id>=DOMID_SELF{return state}
    state|=PERF_GUEST_ACTIVE;
    if (*d).pmu.pmu_flags&PMU_SAMPLE_PV!=0{if (*d).pmu.pmu_flags&PMU_SAMPLE_USER!=0{state|=PERF_GUEST_USER}}else if (*d).pmu.r.regs.cpl&3!=0{state|=PERF_GUEST_USER} state
}
unsafe fn xen_get_guest_ip()->usize{let d=get_xenpmu_data();if d.is_null(){pr_warn_once("%s: pmudata not initialized\n",__func__);0}else{(*d).pmu.r.regs.ip}}

unsafe fn xen_convert_regs(x:*const xen_pmu_regs,r:*mut pt_regs,flags:u64){(*r).ip=(*x).ip;(*r).cs=(*x).cs;(*r).sp=(*x).sp;if flags&PMU_SAMPLE_PV!=0{if flags&PMU_SAMPLE_USER!=0{(*r).cs|=3}else{(*r).cs&=!3}}else if (*x).cpl!=0{(*r).cs|=3}else{(*r).cs&=!3}}

pub unsafe extern "C" fn xen_pmu_irq_handler(_irq:i32,_dev_id:*mut core::ffi::c_void)->irqreturn_t {
    let d=get_xenpmu_data();let old=get_xenpmu_flags();if d.is_null(){pr_warn_once("%s: pmudata not initialized\n",__func__);return IRQ_NONE}
    xenpmu_shared.flags=old|XENPMU_IRQ_PROCESSING;let mut regs=core::mem::zeroed::<pt_regs>();xen_convert_regs(&(*d).pmu.r.regs,&mut regs,(*d).pmu.pmu_flags);let ret=if x86_pmu.handle_irq(&mut regs){IRQ_HANDLED}else{IRQ_NONE};
    let err=HYPERVISOR_xenpmu_op(XENPMU_flush,core::ptr::null_mut());xenpmu_shared.flags=old;if err!=0{pr_warn_once("%s: failed hypercall, err: %d\n",__func__,err);IRQ_NONE}else{ret}
}

pub unsafe fn xen_pmu_init(cpu:i32){
    if xen_hvm_domain()||(cpu!=0&&!is_xen_pmu){return}
    let d=get_zeroed_page(GFP_KERNEL)as*mut xen_pmu_data;if d.is_null(){pr_err("VPMU init: No memory\n");return}
    let mut xp: xen_pmu_params=core::mem::zeroed();let pfn=virt_to_pfn(d);xp.val=pfn_to_mfn(pfn);xp.vcpu=cpu;xp.version.maj=XENPMU_VER_MAJ;xp.version.min=XENPMU_VER_MIN;
    let err=HYPERVISOR_xenpmu_op(XENPMU_init,&mut xp);if err!=0{if err==-EOPNOTSUPP||err==-ENOSYS{pr_info_once("VPMU disabled by hypervisor.\n")}else{pr_info_once("Could not initialize VPMU for cpu %d, error %d\n",cpu,err)}free_pages(d as usize,0);return}
    xenpmu_shared.xenpmu_data=d;xenpmu_shared.flags=0;if !is_xen_pmu{is_xen_pmu=true;perf_register_guest_info_callbacks(&xen_guest_cbs);xen_pmu_arch_init()}
}

pub unsafe fn xen_pmu_finish(cpu:i32){if xen_hvm_domain(){return}let mut xp:xen_pmu_params=core::mem::zeroed();xp.vcpu=cpu;xp.version.maj=XENPMU_VER_MAJ;xp.version.min=XENPMU_VER_MIN;let _=HYPERVISOR_xenpmu_op(XENPMU_finish,&mut xp);free_pages(xenpmu_shared.xenpmu_data as usize,0);xenpmu_shared.xenpmu_data=core::ptr::null_mut();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
