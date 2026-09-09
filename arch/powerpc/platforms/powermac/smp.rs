// SPDX-License-Identifier: GPL-2.0-or-later
/* SMP support for power macintosh. */

use core::ffi::c_void;

extern "C" {
    static mut ppc_md: PpcMd;
    static mut smp_ops: *mut SmpOps;
    static mut powersave_nap: i32;
    static mut tb_ticks_per_jiffy: u64;
    fn __secondary_start_pmac_0();
    fn smp_processor_id() -> i32;
    fn smp_ipi_demux();
    fn irq_set_chip_and_handler(virq: u32, chip: *const c_void, handler: *const c_void);
    fn irq_domain_create_nomap(a: *mut c_void, b: u32, ops: *const c_void, d: *mut c_void) -> *mut c_void;
    fn irq_create_direct_mapping(h: *mut c_void) -> u32;
    fn request_irq(irq: u32, f: unsafe extern "C" fn(i32, *mut c_void) -> i32, flags: u64, n: *const u8, d: *mut c_void) -> i32;
    fn irq_create_mapping(a: *mut c_void, h: u32) -> u32;
    fn set_cpu_present(cpu: i32, present: bool);
    fn set_cpu_possible(cpu: i32, possible: bool);
    fn ioremap(addr: usize, size: usize) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn in_8(p: *const u8) -> u8;
    fn out_8(p: *mut u8, v: u8);
    fn in_be32(p: *const u32) -> u32;
    fn out_be32(p: *mut u32, v: u32);
    fn mdelay(v: u32);
    fn get_tb() -> u64;
    fn set_tb(hi: u32, lo: u32);
    fn set_dec(v: u64);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn local_irq_disable();
    fn local_irq_enable();
    fn mb(); fn wmb(); fn barrier(); fn smp_wmb();
    fn of_find_node_by_name(n: *mut c_void, s: *const u8) -> *mut DeviceNode;
    fn of_find_node_by_path(s: *const u8) -> *mut DeviceNode;
    fn of_node_put(n: *mut DeviceNode);
    fn of_find_node_by_type(n: *mut DeviceNode, s: *const u8) -> *mut DeviceNode;
    fn of_get_parent(n: *mut DeviceNode) -> *mut DeviceNode;
    fn of_device_is_compatible(n: *mut DeviceNode, s: *const u8) -> bool;
    fn of_get_property(n: *mut DeviceNode, s: *const u8, len: *mut i32) -> *const u32;
    fn of_property_read_bool(n: *mut DeviceNode, s: *const u8) -> bool;
    fn of_machine_is_compatible(s: *const u8) -> bool;
    fn pmac_i2c_find_bus(n: *mut DeviceNode) -> *mut PmacI2cBus;
    fn pmac_i2c_setmode(b: *mut PmacI2cBus, mode: u32);
    fn pmac_i2c_xfer(b: *mut PmacI2cBus, addr: u8, len: u32, sub: u8, data: *mut u8, n: u32) -> i32;
    fn pmac_i2c_open(b: *mut PmacI2cBus, sync: i32) -> i32;
    fn pmac_i2c_close(b: *mut PmacI2cBus);
    fn pmac_pfunc_base_install(); fn pmac_i2c_init(); fn mpic_request_ipis(); fn mpic_setup_this_cpu();
    fn mpic_cpu_set_priority(v: u32); fn cleanup_cpu_mmu_context(); fn generic_cpu_disable() -> i32;
    fn generic_set_cpu_dead(cpu: i32); fn low_cpu_offline_self(); fn power4_idle(); fn generic_cpu_die(cpu: u32);
    fn pmac_call_feature(a: u32, b: *mut c_void, c: u32, d: u32);
    fn patch_branch(v: *mut u32, target: usize, flags: u32); fn patch_uint(v: *mut u32, x: u32);
    fn _get_L2CR() -> usize; fn _set_L2CR(v: usize); fn _get_L3CR() -> usize; fn _set_L3CR(v: usize);
    fn cpu_has_feature(v: u32) -> bool; fn g5_phy_disable_cpu1(); fn idle_task_exit();
    fn smp_generic_give_timebase(); fn smp_generic_take_timebase(); fn smp_mpic_message_pass();
}

#[repr(C)] pub struct DeviceNode { _private: [u8; 0] }
#[repr(C)] pub struct PmacI2cBus { _private: [u8; 0] }
#[repr(C)] pub struct PpcMd { pub progress: Option<unsafe extern "C" fn(*const u8, u32)> }
#[repr(C)] pub struct SmpOps { pub message_pass: Option<unsafe extern "C" fn()>, pub cause_ipi: Option<unsafe extern "C" fn(i32)>, pub cause_nmi_ipi: Option<unsafe extern "C" fn(i32)>, pub probe: Option<unsafe extern "C" fn()>, pub kick_cpu: Option<unsafe extern "C" fn(i32) -> i32>, pub setup_cpu: Option<unsafe extern "C" fn(i32)>, pub give_timebase: Option<unsafe extern "C" fn()>, pub take_timebase: Option<unsafe extern "C" fn()>, pub bringup_done: Option<unsafe extern "C" fn()>, pub cpu_disable: Option<unsafe extern "C" fn() -> i32>, pub cpu_die: Option<unsafe extern "C" fn(u32)>, pub cpu_offline_self: Option<unsafe extern "C" fn()> }

static mut pmac_tb_freeze: Option<unsafe extern "C" fn(i32)> = None;
static mut timebase: u64 = 0; static mut tb_req: i32 = 0;

const IRQ_HANDLED: i32 = 1; const IRQF_PERCPU: u64 = 0x10000; const IRQF_NO_THREAD: u64 = 0x10000000;
const ENOMEM: i32 = 12; const ENOENT: i32 = 2; const NR_CPUS: i32 = 32; const PAGE_OFFSET: usize = 0; const KERNELBASE: usize = 0; const BRANCH_SET_LINK: u32 = 1;
const HAMMERHEAD_BASE: usize=0xf8000000; const HHEAD_CONFIG: usize=0x90; const HHEAD_SEC_INTR: usize=0xc0; const PSURGE_PRI_INTR: usize=0xf3019000; const PSURGE_START: usize=0xf2800000; const PSURGE_QUAD_REG_ADDR: usize=0xf8800000;
const PSURGE_NONE:i32=-1; const PSURGE_DUAL:i32=0; const PSURGE_QUAD_OKEE:i32=1; const PSURGE_QUAD_COTTON:i32=2; const PSURGE_QUAD_ICEGRASS:i32=3;
const PSURGE_QUAD_IRQ_SET:usize=0; const PSURGE_QUAD_IRQ_CLR:usize=1; const PSURGE_QUAD_PRIMARY_ARB:usize=4; const PSURGE_QUAD_BOARD_ID:usize=6; const PSURGE_QUAD_WHICH_CPU:usize=7; const PSURGE_QUAD_CKSTOP_CTL:usize=3; const PSURGE_QUAD_RESET_CTL:usize=11;
static mut hhead_base:*mut u8=core::ptr::null_mut(); static mut quad_base:*mut u8=core::ptr::null_mut(); static mut psurge_pri_intr:*mut u32=core::ptr::null_mut(); static mut psurge_sec_intr:*mut u8=core::ptr::null_mut(); static mut psurge_start:*mut u32=core::ptr::null_mut(); static mut psurge_type:i32=PSURGE_NONE; static mut psurge_secondary_virq:u32=0;

unsafe fn qout(r:usize,v:u8){out_8(quad_base.add((r<<4)+4),v)} unsafe fn qin(r:usize)->u8{in_8(quad_base.add((r<<4)+4))&0xf} unsafe fn qbis(r:usize,v:u8){qout(r,qin(r)|v)} unsafe fn qbic(r:usize,v:u8){qout(r,qin(r)&!v)}
unsafe fn psurge_set_ipi(cpu:i32){if psurge_type==PSURGE_NONE{return} if cpu==0{in_be32(psurge_pri_intr);}else if psurge_type==PSURGE_DUAL{out_8(psurge_sec_intr,!0)}else{qout(0,1u8<<cpu)}}
unsafe fn psurge_clr_ipi(cpu:i32){if cpu>0{if psurge_type==PSURGE_DUAL{out_8(psurge_sec_intr,!0)}else if psurge_type!=PSURGE_NONE{qout(1,1u8<<cpu)}}}
unsafe extern "C" fn psurge_ipi_intr(_:i32,_:*mut c_void)->i32{psurge_clr_ipi(smp_processor_id());smp_ipi_demux();IRQ_HANDLED}
unsafe extern "C" fn smp_psurge_cause_ipi(cpu:i32){psurge_set_ipi(cpu)}

#[cfg(feature="CONFIG_PPC_PMAC32_PSURGE")]
unsafe fn psurge_quad_probe()->i32{let t=qin(PSURGE_QUAD_BOARD_ID) as i32;if t<PSURGE_QUAD_OKEE||t>PSURGE_QUAD_ICEGRASS||t!=(qin(PSURGE_QUAD_BOARD_ID)as i32){return PSURGE_DUAL} t}
#[cfg(feature="CONFIG_PPC_PMAC32_PSURGE")]
unsafe fn smp_psurge_probe(){let dn=of_find_node_by_name(core::ptr::null_mut(),b"hammerhead\0".as_ptr());if dn.is_null(){return}of_node_put(dn);hhead_base=ioremap(HAMMERHEAD_BASE,0x800);quad_base=ioremap(PSURGE_QUAD_REG_ADDR,1024);psurge_sec_intr=hhead_base.add(HHEAD_SEC_INTR);psurge_type=psurge_quad_probe();let n=if psurge_type!=PSURGE_DUAL{smp_ops.as_mut().unwrap().give_timebase=Some(smp_generic_give_timebase);smp_ops.as_mut().unwrap().take_timebase=Some(smp_generic_take_timebase);4}else{ iounmap(quad_base);if in_8(hhead_base.add(HHEAD_CONFIG))&2==0{iounmap(hhead_base);psurge_type=PSURGE_NONE;return}2};if n>NR_CPUS{return}for i in 1..n{set_cpu_present(i,true)}}
#[cfg(feature="CONFIG_PPC_PMAC32_PSURGE")]
#[no_mangle] pub static mut psurge_smp_ops:SmpOps=SmpOps{message_pass:None,cause_ipi:Some(smp_psurge_cause_ipi),cause_nmi_ipi:None,probe:Some(smp_psurge_probe),kick_cpu:None,setup_cpu:None,give_timebase:Some(smp_generic_give_timebase),take_timebase:Some(smp_generic_take_timebase),bringup_done:None,cpu_disable:None,cpu_die:None,cpu_offline_self:None};

unsafe extern "C" fn smp_core99_give_timebase(){let mut f=0;local_irq_save(&mut f);while tb_req==0{barrier()}tb_req=0;if let Some(x)=pmac_tb_freeze{x(1)};mb();timebase=get_tb();mb();while timebase!=0{barrier()}mb();if let Some(x)=pmac_tb_freeze{x(0)};mb();local_irq_restore(f)}
unsafe extern "C" fn smp_core99_take_timebase(){let mut f=0;local_irq_save(&mut f);tb_req=1;mb();while timebase==0{barrier()}mb();set_tb((timebase>>32)as u32,timebase as u32);timebase=0;mb();local_irq_restore(f)}
unsafe fn core99_init_caches(cpu:i32){if cpu==0{let _=_get_L2CR()}else{_set_L2CR(0)}}
unsafe extern "C" fn smp_core99_probe(){let mut n=0;let mut c=of_find_node_by_type(core::ptr::null_mut(),b"cpu\0".as_ptr());while !c.is_null(){n+=1;c=of_find_node_by_type(c,b"cpu\0".as_ptr())}if n<=1{return}pmac_pfunc_base_install();pmac_i2c_init();mpic_request_ipis();core99_init_caches(0)}
unsafe extern "C" fn smp_core99_kick_cpu(nr:i32)->i32{if nr<0||nr>3{return -ENOENT}let mut f=0;local_irq_save(&mut f);let v=(PAGE_OFFSET+0x100)as*mut u32;let save=*v;patch_branch(v,__secondary_start_pmac_0 as usize+nr as usize*8,BRANCH_SET_LINK);pmac_call_feature(0,core::ptr::null_mut(),nr as u32,0);mdelay(1);patch_uint(v,save);local_irq_restore(f);0}
unsafe extern "C" fn smp_core99_setup_cpu(cpu:i32){if cpu!=0{core99_init_caches(cpu)}mpic_setup_this_cpu()}
static mut core99_smp_ops:SmpOps=SmpOps{message_pass:Some(smp_mpic_message_pass),cause_ipi:None,cause_nmi_ipi:None,probe:Some(smp_core99_probe),kick_cpu:Some(smp_core99_kick_cpu),setup_cpu:Some(smp_core99_setup_cpu),give_timebase:Some(smp_core99_give_timebase),take_timebase:Some(smp_core99_take_timebase),bringup_done:None,cpu_disable:None,cpu_die:None,cpu_offline_self:None};

#[no_mangle] pub unsafe extern "C" fn pmac_setup_smp(){let mut n=of_find_node_by_name(core::ptr::null_mut(),b"uni-n\0".as_ptr());if n.is_null(){n=of_find_node_by_name(core::ptr::null_mut(),b"u3\0".as_ptr())}if n.is_null(){n=of_find_node_by_name(core::ptr::null_mut(),b"u4\0".as_ptr())}if !n.is_null(){of_node_put(n);smp_ops=&mut core99_smp_ops}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
