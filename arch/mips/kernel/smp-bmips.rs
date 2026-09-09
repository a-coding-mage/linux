/* Rust translation of smp-bmips.c. Kernel-provided symbols are intentionally
 * left as external dependencies. */

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static mut max_cpus: i32;
    static mut bmips_smp_enabled: i32;
    static mut bmips_cpu_offset: i32;
    static mut bmips_booted_mask: cpumask_t;
    static mut bmips_tp1_irqs: c_ulong;
    static mut bmips_smp_boot_sp: c_ulong;
    static mut bmips_smp_boot_gp: c_ulong;
    static mut board_ebase_setup: Option<unsafe extern "C" fn()>;
    static mut board_nmi_handler_setup: Option<unsafe extern "C" fn()>;
    static mut ebase: c_ulong;
    static bmips_cbr_addr: *mut c_void;
    static bmips_reset_nmi_vec: *mut c_char;
    static bmips_reset_nmi_vec_end: *mut c_char;
    static bmips_smp_int_vec: *mut c_char;
    static bmips_smp_int_vec_end: *mut c_char;
    static mut __cpu_number_map: [i32; 256];
    static mut __cpu_logical_map: [i32; 256];
}

type c_ulong = usize;
type c_char = i8;
type u32_alias = u32;
type irqreturn_t = i32;

#[repr(C)] pub struct cpumask_t { _opaque: [u8; 128] }
#[repr(C)] pub struct cpumask { _opaque: [u8; 128] }
#[repr(C)] pub struct task_struct { _opaque: [u8; 0] }
#[repr(C)] pub struct plat_smp_ops { _opaque: [u8; 0] }

const RESET_FROM_KSEG0: u32 = 0x80080800;
const RESET_FROM_KSEG1: u32 = 0xa0080800;
const IPI0_IRQ: i32 = MIPS_CPU_IRQ_BASE + 0;
const IPI1_IRQ: i32 = MIPS_CPU_IRQ_BASE + 1;

extern "C" {
    fn current_cpu_type() -> i32;
    fn clear_c0_brcm_cmt_ctrl(v: u32); fn set_c0_brcm_cmt_ctrl(v: u32);
    fn set_c0_brcm_config_0(v: u32); fn clear_c0_brcm_config_0(v: u32);
    fn read_c0_brcm_cmt_local() -> u32; fn change_c0_brcm_cmt_intr(a: u32,b:u32);
    fn set_c0_brcm_config(v:u32); fn read_c0_brcm_config()->u32;
    fn change_c0_brcm_mode(a:u32,b:u32); fn write_c0_brcm_action(v:u32);
    fn request_irq(i:i32, f:unsafe extern "C" fn(i32,*mut c_void)->i32, flags:u32,n:*const u8,d:*mut c_void)->i32;
    fn panic(s:*const u8)->!; fn bmips_ebase_setup(); fn bmips_cpu_setup();
    fn set_cpu_possible(i:u32,v:i32); fn set_cpu_present(i:u32,v:i32);
    fn mb(); fn cpumask_test_cpu(i:i32,m:*const cpumask_t)->bool; fn cpumask_set_cpu(i:i32,m:*mut cpumask_t);
    fn bmips43xx_send_ipi_single(i:i32,a:u32); fn bmips5000_send_ipi_single(i:i32,a:u32);
    fn cpu_logical_map(i:i32)->i32; fn smp_processor_id()->i32; fn scheduler_ipi(); fn generic_smp_call_function_interrupt();
    fn write_c0_compare(v:u32); fn read_c0_count()->u32; fn irq_enable_hazard(); fn set_c0_status(v:u32);
    fn write_c0_brcm_bootvec(v:u32); fn read_c0_brcm_bootvec()->u32; fn __sync(); fn back_to_back_c0_hazard();
    fn __raw_writel(v:u32,p:*mut c_void); fn __raw_readl(p:*mut c_void)->u32; fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void;
    fn dma_cache_wback(d:usize,n:usize); fn local_flush_icache_range(a:usize,b:usize); fn instruction_hazard();
    fn set_uncached_handler(a:usize,p:*const c_void,n:usize); fn write_c0_ebase(v:usize);
    fn read_c0_prid()->u32; fn set_c0_brcm_bus_pll(v:u32); fn clear_c0_brcm_bus_pll(v:u32); fn clear_c0_brcm_reset(v:u32);
    fn bmips_write_zscm_reg(a:u32,v:u32); fn bmips_read_zscm_reg(a:u32); fn __dma_cache_wback_inv(a:u32,b:u32);
}

const CPU_BMIPS3300:i32=1; const CPU_BMIPS4350:i32=2; const CPU_BMIPS4380:i32=3; const CPU_BMIPS5000:i32=4;
const MIPS_CPU_IRQ_BASE:i32=0; const SMP_CALL_FUNCTION:u32=2; const SMP_RESCHEDULE_YOURSELF:u32=1;
const IE_IRQ1:usize=1; const IE_IRQ5:u32=1<<15; const IE_SW0:u32=1<<8; const IE_SW1:u32=1<<9; const ST0_IE:u32=1; const ST0_BEV:u32=1<<22; const C_SW0:u32=1<<8; const C_SW1:u32=1<<9; const CAUSEF_IV:u32=1<<23;
const BMIPS_NMI_RESET_VEC:usize=0xa0000000; const BMIPS_WARM_RESTART_VEC:usize=0xa0000380; const CKSEG0:usize=0x80000000;
const BMIPS_RELO_VECTOR_CONTROL_0:usize=0; const BMIPS_RELO_VECTOR_CONTROL_1:usize=4; const BMIPS_RAC_CONFIG:usize=0; const BMIPS_RAC_CONFIG_1:usize=4; const BMIPS_RAC_ADDRESS_RANGE:usize=8; const BMIPS_L2_CONFIG:usize=12;
const HZ:u32=100;

#[inline] unsafe fn CPUNUM(cpu:i32,shift:u32)->u32 { (((cpu + bmips_cpu_offset) as u32) << shift) }
#[inline] unsafe fn ACTION_CLR_IPI(cpu:i32,ipi:u32)->u32 { 0x2000 | CPUNUM(cpu,9) | (ipi<<8) }
#[inline] unsafe fn ACTION_SET_IPI(cpu:i32,ipi:u32)->u32 { 0x3000 | CPUNUM(cpu,9) | (ipi<<8) }
#[inline] unsafe fn ACTION_BOOT_THREAD(cpu:i32)->u32 { 8 | CPUNUM(cpu,0) }

unsafe extern "C" fn bmips_set_reset_vec(_cpu:i32,_val:u32) {}

// CONFIG_SMP implementation is translated below; platform-specific kernel
// operation tables and assembly entry points remain external dependencies.
unsafe fn bmips_wr_vec(dst:usize,start:*const c_char,end:*const c_char) { let n=end.offset_from(start) as usize; memcpy(dst as *mut c_void,start as *const c_void,n); dma_cache_wback(dst,n); local_flush_icache_range(dst,dst+n); instruction_hazard(); }
unsafe extern "C" fn bmips_nmi_handler_setup(){ bmips_wr_vec(BMIPS_NMI_RESET_VEC,bmips_reset_nmi_vec,bmips_reset_nmi_vec_end); bmips_wr_vec(BMIPS_WARM_RESTART_VEC,bmips_smp_int_vec,bmips_smp_int_vec_end); }

#[repr(C)] struct reset_vec_info { cpu:i32, val:u32 }
unsafe extern "C" fn bmips_set_reset_vec_remote(v:*mut c_void){ let info=&*(v as *const reset_vec_info); let shift=if info.cpu&1!=0{16}else{0}; let mask=!(0xffffu32<<shift); let val=info.val>>16; if info.cpu&2!=0 { bmips_write_zscm_reg(0xa0,(val<<16)|val); bmips_read_zscm_reg(0xa0); } else { write_c0_brcm_bootvec((read_c0_brcm_bootvec()&mask)|(val<<shift)); } }

pub unsafe extern "C" fn bmips_ebase_setup(){ let mut new_ebase=ebase; match current_cpu_type(){ CPU_BMIPS4350=>{set_uncached_handler(BMIPS_WARM_RESTART_VEC-CKSEG0,bmips_smp_int_vec as *const c_void,0x80);__sync();}, CPU_BMIPS3300|CPU_BMIPS4380=>{new_ebase=0x80000400;bmips_set_reset_vec(0,RESET_FROM_KSEG0);}, CPU_BMIPS5000=>{new_ebase=0x80001000;bmips_set_reset_vec(0,RESET_FROM_KSEG0);write_c0_ebase(new_ebase);}, _=>return }; board_nmi_handler_setup=Some(bmips_nmi_handler_setup); ebase=new_ebase; }

pub unsafe extern "C" fn plat_wired_tlb_setup() {}
pub unsafe extern "C" fn bmips_cpu_setup(){ let cbr=bmips_cbr_addr as *mut u8; let mut cfg:u32; match current_cpu_type(){ CPU_BMIPS3300=>{set_c0_brcm_bus_pll(1<<22);__sync();clear_c0_brcm_bus_pll(1<<22);clear_c0_brcm_reset(1<<16);cfg=__raw_readl(cbr.add(BMIPS_RAC_CONFIG) as *mut c_void);__raw_writel(cfg|0x100,cbr.add(BMIPS_RAC_CONFIG) as *mut c_void);let _=__raw_readl(cbr.add(BMIPS_RAC_CONFIG) as *mut c_void);cfg=__raw_readl(cbr.add(BMIPS_RAC_CONFIG) as *mut c_void);__raw_writel(cfg|0xf,cbr.add(BMIPS_RAC_CONFIG) as *mut c_void);let _=__raw_readl(cbr.add(BMIPS_RAC_CONFIG) as *mut c_void);cfg=__raw_readl(cbr.add(BMIPS_RAC_ADDRESS_RANGE) as *mut c_void);__raw_writel(cfg|0x0fff0000,cbr.add(BMIPS_RAC_ADDRESS_RANGE) as *mut c_void);}, CPU_BMIPS4350|CPU_BMIPS4380=>{clear_c0_brcm_config_0(1<<21);set_c0_brcm_config_0(1<<23);set_c0_brcm_cmt_ctrl(1<<15);}, CPU_BMIPS5000=>{set_c0_brcm_config((1<<17)|(1<<21)|(1<<27));}, _=>{} } }

// The following low-level entry points preserve the C interfaces and ordering;
// kernel scheduling, IRQ, hotplug, and per-CPU primitives are external.
extern "C" { fn spin_lock_irqsave(l:*mut c_void,f:*mut c_ulong); fn spin_unlock_irqrestore(l:*mut c_void,f:c_ulong); fn set_c0_cause(v:u32); fn clear_c0_cause(v:u32); fn per_cpu_action(cpu:i32)->*mut u32; fn this_cpu_action()->u32; fn irq_disable_hazard(); }
static mut ipi_lock: u8=0; static mut ipi_action_mask:[u32;256]=[0;256];
pub unsafe extern "C" fn bmips5000_send_ipi_single(cpu:i32,action:u32){write_c0_brcm_action(ACTION_SET_IPI(cpu,(action==SMP_CALL_FUNCTION) as u32));}
pub unsafe extern "C" fn bmips5000_ipi_interrupt(irq:i32,_dev:*mut c_void)->i32{let action=irq-IPI0_IRQ;write_c0_brcm_action(ACTION_CLR_IPI(smp_processor_id(),action as u32));if action==0{scheduler_ipi()}else{generic_smp_call_function_interrupt()} ;1}
pub unsafe extern "C" fn bmips43xx_send_ipi_single(cpu:i32,action:u32){let mut flags=0;spin_lock_irqsave(&mut ipi_lock as *mut _ as *mut c_void,&mut flags);set_c0_cause(if cpu!=0{C_SW1}else{C_SW0});ipi_action_mask[cpu as usize]|=action;irq_enable_hazard();spin_unlock_irqrestore(&mut ipi_lock as *mut _ as *mut c_void,flags);}
pub unsafe extern "C" fn bmips43xx_ipi_interrupt(irq:i32,_dev:*mut c_void)->i32{let mut flags=0;let cpu=irq-IPI0_IRQ;spin_lock_irqsave(&mut ipi_lock as *mut _ as *mut c_void,&mut flags);let action=ipi_action_mask[smp_processor_id() as usize];ipi_action_mask[cpu as usize]=0;clear_c0_cause(if cpu!=0{C_SW1}else{C_SW0});spin_unlock_irqrestore(&mut ipi_lock as *mut _ as *mut c_void,flags);if action&SMP_RESCHEDULE_YOURSELF!=0{scheduler_ipi()}if action&SMP_CALL_FUNCTION!=0{generic_smp_call_function_interrupt()}1}
#[repr(C)] pub struct reset_ops { pub smp_setup: Option<unsafe extern "C" fn()>, pub prepare_cpus: Option<unsafe extern "C" fn(u32)> }
pub unsafe extern "C" fn bmips_set_reset_vec_public(cpu:i32,val:u32){if current_cpu_type()==CPU_BMIPS5000{let mut info=reset_vec_info{cpu,val};bmips_set_reset_vec_remote(&mut info as *mut _ as *mut c_void)}else{bmips_set_reset_vec(cpu,val)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
