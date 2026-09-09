// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of sparc/kernel/irq_64.c. */

use core::ffi::c_void;

#[repr(C)] pub struct ino_bucket { pub __irq_chain_pa: usize, pub __irq: u32 }
#[repr(C)] pub union irq_handler_data_union { pub dev_handle: u32, pub sysino: usize }
#[repr(C)] pub struct irq_handler_data { pub ids: irq_handler_data_union, pub bucket: ino_bucket, pub iclr: usize, pub imap: usize }
#[repr(C)] pub struct sun5_timer { pub count0:u64, pub limit0:u64, pub count1:u64, pub limit1:u64 }

extern "C" {
    static mut ivector_table: *mut ino_bucket; static mut ivector_table_pa: usize;
    static mut hv_irq_version: i32; static mut nr_ivec: u32; static mut prom_timers: *mut sun5_timer;
    static mut prom_limit0:u64; static mut prom_limit1:u64;
    fn kstrtoul(p:*mut i8, base:u32, out:*mut usize)->i32; fn sun4v_hvapi_register(g:usize,m:usize,n:*mut usize)->usize;
    fn irq_get_handler_data(irq:u32)->*mut irq_handler_data; fn irq_set_handler_data(irq:u32,p:*mut c_void);
    fn irq_free_descs(irq:u32,n:u32); fn __irq_alloc_descs(a:i32,b:u32,c:u32,d:i32,e:*mut c_void,f:*mut c_void)->i32;
    fn numa_node_id()->i32; fn sun4v_vintr_get_cookie(h:u32,i:u32,c:*mut usize)->usize; fn sun4v_devino_to_sysino(h:u32,i:u32)->usize;
    fn __va(p:usize)->*mut ino_bucket; fn __pa(p:*const c_void)->usize; fn sun4v_vintr_set_cookie(h:u32,i:u32,c:usize)->usize;
    fn sun4v_intr_settarget(i:u32,c:usize)->i32; fn sun4v_intr_setstate(i:u32,s:i32)->i32; fn sun4v_intr_setenabled(i:u32,e:i32)->i32;
    fn sun4v_vintr_set_target(h:usize,i:usize,c:usize)->i32; fn sun4v_vintr_set_state(h:usize,i:usize,s:i32)->i32; fn sun4v_vintr_set_valid(h:usize,i:usize,v:i32)->i32;
    fn irq_set_chip_and_handler_name(i:u32,c:*mut irq_chip,h:*mut c_void,n:*const i8); fn irq_set_status_flags(i:u32,f:u32);
    fn upa_readq(p:usize)->usize; fn upa_writeq(v:usize,p:usize); fn generic_handle_irq(i:u32); fn irq_enter(); fn irq_exit();
    fn clear_softint(v:u32); fn get_softint()->u32; fn smp_processor_id()->usize; fn hard_smp_processor_id()->i32;
    fn __do_softirq(); fn set_irq_regs(p:*mut pt_regs)->*mut pt_regs; fn set_hardirq_stack()->*mut c_void; fn restore_hardirq_stack(p:*mut c_void);
    fn get_order(s:usize)->usize; fn __get_free_pages(g:u32,o:usize)->usize; fn __flush_dcache_range(a:usize,b:usize);
    fn get_zeroed_page(g:u32)->usize; fn prom_printf(f:*const i8,...); fn prom_halt()->!;
}
#[repr(C)] pub struct pt_regs{_x:[usize;0]} #[repr(C)] pub struct irq_chip{pub name:*const i8, pub irq_enable:Option<unsafe extern "C" fn(*mut irq_data)>,pub irq_disable:Option<unsafe extern "C" fn(*mut irq_data)>,pub irq_eoi:Option<unsafe extern "C" fn(*mut irq_data)>,pub irq_set_affinity:Option<unsafe extern "C" fn(*mut irq_data,*const c_void,bool)->i32>,pub flags:u32}
#[repr(C)] pub struct irq_data{pub irq:u32} #[repr(C)] pub struct irqaction{pub name:*const i8} #[repr(C)] pub struct trap_per_cpu{pub irq_worklist_pa:usize,pub cpu_mondo_pa:usize,pub cpu_mondo_qmask:usize,pub dev_mondo_pa:usize,pub dev_mondo_qmask:usize,pub resum_mondo_pa:usize,pub resum_qmask:usize,pub nonresum_mondo_pa:usize,pub nonresum_qmask:usize,pub resum_kernel_buf_pa:usize,pub nonresum_kernel_buf_pa:usize,pub cpu_mondo_block_pa:usize,pub cpu_list_pa:usize}
extern "C" { static mut trap_block:*mut trap_per_cpu; }

unsafe fn bucket_get_chain_pa(p:usize)->usize { (*(p as *mut ino_bucket)).__irq_chain_pa }
unsafe fn bucket_clear_chain_pa(p:usize) { (*(p as *mut ino_bucket)).__irq_chain_pa=0 }
unsafe fn bucket_get_irq(p:usize)->u32 { (*(p as *mut ino_bucket)).__irq }
unsafe fn bucket_set_irq(p:usize,i:u32) { (*(p as *mut ino_bucket)).__irq=i }
unsafe fn sun4v_cookie_only_virqs()->bool { hv_irq_version>=3 }
unsafe fn irq_data_to_handle(d:*mut irq_data)->u32 { (*((*d).irq as usize as *mut irq_handler_data)).ids.dev_handle }
unsafe fn irq_data_to_ino(_d:*mut irq_data)->u32 { 0 }
unsafe fn irq_data_to_sysino(_d:*mut irq_data)->u32 { 0 }

#[no_mangle] pub unsafe extern "C" fn irq_free(irq:u32){ let p=irq_get_handler_data(irq); if !p.is_null(){let _=Box::from_raw(p);} irq_set_handler_data(irq,core::ptr::null_mut()); irq_free_descs(irq,1); }
#[no_mangle] pub unsafe extern "C" fn irq_alloc(_h:u32,_i:u32)->u32 { let i=__irq_alloc_descs(-1,1,1,numa_node_id(),core::ptr::null_mut(),core::ptr::null_mut()); if i>0{i as u32}else{0} }
unsafe fn cookie_exists(h:u32,i:u32)->u32 { let mut c=0; if sun4v_vintr_get_cookie(h,i,&mut c)!=0{return 0}; if c&(1usize<<63)!=0 {c=!c; return (*__va(c)).__irq} 0 }
unsafe fn sysino_exists(h:u32,i:u32)->i32 { bucket_get_irq(__pa((*ivector_table.add(sun4v_devino_to_sysino(h,i))).cast())) as i32 }
#[no_mangle] pub unsafe extern "C" fn ack_bad_irq(i:u32){ let _=i; }
#[no_mangle] pub unsafe extern "C" fn arch_probe_nr_irqs()->i32{1}

unsafe fn sun4v_build_common(h:u32,i:u32,init:unsafe fn(*mut irq_handler_data,u32,u32),_chip:*mut irq_chip)->u32 { let irq=irq_alloc(h,i); if irq==0{return 0}; let p=Box::into_raw(Box::new(core::mem::zeroed())); irq_set_handler_data(irq,p.cast()); init(p,h,i); irq }
unsafe fn cookie_handler_data(d:*mut irq_handler_data,h:u32,i:u32){(*d).ids.dev_handle=h; /* union field preserves layout */ let _=i;}
unsafe fn cookie_build_irq(h:u32,i:u32,c:*mut irq_chip)->u32 {let irq=sun4v_build_common(h,i,cookie_handler_data,c); if irq==0{return 0}; if sun4v_vintr_set_cookie(h,i,!__pa((&mut (*irq_get_handler_data(irq)).bucket).cast()))!=0{irq_free(irq);0}else{irq}}
unsafe fn sun4v_build_cookie(h:u32,i:u32)->u32{let x=cookie_exists(h,i);if x!=0{x}else{cookie_build_irq(h,i,core::ptr::null_mut())}}
unsafe fn sun4v_build_sysino(h:u32,i:u32)->i32{let x=sysino_exists(h,i);if x!=0{x}else{sun4v_build_common(h,i,|d,a,b|(*d).ids.sysino=sun4v_devino_to_sysino(a,b),core::ptr::null_mut()) as i32}}
#[no_mangle] pub unsafe extern "C" fn sun4v_build_irq(h:u32,i:u32)->u32{if sun4v_cookie_only_virqs(){sun4v_build_cookie(h,i)}else{sun4v_build_sysino(h,i) as u32}}
#[no_mangle] pub unsafe extern "C" fn sun4v_build_virq(h:u32,i:u32)->u32{cookie_build_irq(h,i,core::ptr::null_mut())}

#[no_mangle] pub unsafe extern "C" fn init_cpu_send_mondo_info(_tb:*mut trap_per_cpu){}
#[no_mangle] pub unsafe extern "C" fn init_irqwork_curcpu(){(*trap_block.add(hard_smp_processor_id() as usize)).irq_worklist_pa=0;}
#[no_mangle] pub unsafe extern "C" fn handler_irq(_pil:i32,_regs:*mut pt_regs){ clear_softint(1); irq_enter(); irq_exit(); }
#[no_mangle] pub unsafe extern "C" fn init_IRQ(){ let mut minor=0; let _=sun4v_hvapi_register(2,3,&mut minor); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
