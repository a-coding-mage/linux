// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of alpha/kernel/smp.c. */

// C header dependencies are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct CpuDataAlpha {
    pub loops_per_jiffy: c_ulong,
    pub last_asn: c_ulong,
    pub need_new_asn: c_int,
    pub asn_lock: c_int,
    pub prof_counter: c_ulong,
    pub prof_multiplier: c_ulong,
    pub ipi_count: c_ulong,
}

#[repr(C)]
struct IpiData { bits: c_ulong }

#[repr(C)]
pub struct FlushTlbPageStruct {
    pub vma: *mut VmAreaStruct,
    pub mm: *mut MmStruct,
    pub addr: c_ulong,
}

pub const DEBUG_SMP: c_int = 0;
pub static mut cpu_data: [CpuDataAlpha; NR_CPUS as usize] = [CpuDataAlpha { loops_per_jiffy: 0, last_asn: 0, need_new_asn: 0, asn_lock: 0, prof_counter: 0, prof_multiplier: 0, ipi_count: 0 }; NR_CPUS as usize];
static mut ipi_data: [IpiData; NR_CPUS as usize] = [IpiData { bits: 0 }; NR_CPUS as usize];

#[repr(C)]
#[derive(Copy, Clone)]
pub enum IpiMessageType { IPI_RESCHEDULE, IPI_CALL_FUNC, IPI_CPU_STOP }

static mut smp_secondary_alive: c_int = 0;
pub static mut smp_num_probed: c_int = 0;
pub static mut smp_num_cpus: c_int = 1;

#[inline]
unsafe fn smp_store_cpu_info(cpuid: c_int) {
    cpu_data[cpuid as usize].loops_per_jiffy = loops_per_jiffy;
    cpu_data[cpuid as usize].last_asn = ASN_FIRST_VERSION;
    cpu_data[cpuid as usize].need_new_asn = 0;
    cpu_data[cpuid as usize].asn_lock = 0;
}

#[inline]
unsafe fn smp_setup_percpu_timer(cpuid: c_int) {
    cpu_data[cpuid as usize].prof_counter = 1;
    cpu_data[cpuid as usize].prof_multiplier = 1;
}

unsafe fn wait_boot_cpu_to_stop(cpuid: c_int) {
    let stop = jiffies + 10 * HZ;
    while time_before(jiffies, stop) {
        if smp_secondary_alive == 0 { return; }
        barrier();
    }
    printk(b"wait_boot_cpu_to_stop: FAILED on CPU %d, hanging now\n\0".as_ptr(), cpuid);
    loop { barrier(); }
}

pub unsafe fn smp_callin() {
    let cpuid = hard_smp_processor_id();
    if cpu_online(cpuid) { printk(b"??, cpu 0x%x already present??\n\0".as_ptr(), cpuid); BUG(); }
    set_cpu_online(cpuid, true);
    wrmces(7); trap_init(); wrent(entInt, 0);
    smp_setup_percpu_timer(cpuid); init_clockevent();
    if !alpha_mv.smp_callin.is_null() { ((*alpha_mv.smp_callin))( ); }
    mmgrab(&mut init_mm); (*current).active_mm = &mut init_mm;
    notify_cpu_starting(cpuid); local_irq_enable();
    wait_boot_cpu_to_stop(cpuid); mb(); calibrate_delay();
    smp_store_cpu_info(cpuid); wmb(); smp_secondary_alive = 1;
    cpu_startup_entry(CPUHP_AP_ONLINE_IDLE);
}

unsafe fn wait_for_txrdy(cpumask: c_ulong) -> c_int {
    if hwrpb.txrdy & cpumask == 0 { return 0; }
    let timeout = jiffies + 10 * HZ;
    while time_before(jiffies, timeout) {
        if hwrpb.txrdy & cpumask == 0 { return 0; }
        udelay(10); barrier();
    }
    -1
}

unsafe fn send_secondary_console_msg(str_: *mut c_char, cpuid: c_int) {
    let cpu = (hwrpb as *mut u8).add(hwrpb.processor_offset as usize + cpuid as usize * hwrpb.processor_size as usize) as *mut PercpuStruct;
    let cpumask = 1usize.wrapping_shl(cpuid as u32) as c_ulong;
    if wait_for_txrdy(cpumask) != 0 { printk(b"Processor %x not ready\n\0".as_ptr(), cpuid); return; }
    let len = strlen(str_);
    (*cpu).ipc_buffer[0] = len as u64;
    memcpy((*cpu).ipc_buffer.as_mut_ptr().add(1) as *mut c_void, str_ as *const c_void, len);
    wmb(); set_bit(cpuid as c_ulong, &mut hwrpb.rxrdy);
    if wait_for_txrdy(cpumask) != 0 { printk(b"Processor %x not ready\n\0".as_ptr(), cpuid); }
}

unsafe fn recv_secondary_console_msg() {
    let txrdy = hwrpb.txrdy; let mycpu = hard_smp_processor_id();
    for i in 0..NR_CPUS {
        if txrdy & (1usize.wrapping_shl(i as u32) as c_ulong) == 0 { continue; }
        let cpu = (hwrpb as *mut u8).add(hwrpb.processor_offset as usize + i as usize * hwrpb.processor_size as usize) as *mut PercpuStruct;
        let cnt = ((*cpu).ipc_buffer[0] >> 32) as usize;
        let mut buf = [0i8; 80];
        if cnt == 0 || cnt >= 80 { strcpy(buf.as_mut_ptr(), b"<<< BOGUS MSG >>>\0".as_ptr() as *const c_char); }
        else { memcpy(buf.as_mut_ptr() as *mut c_void, (*cpu).ipc_buffer.as_ptr().add(1) as *const c_void, cnt); buf[cnt] = 0; }
        printk(b"recv_secondary_console_msg: on %d message is '%s'\n\0".as_ptr(), mycpu, buf.as_ptr());
    }
    hwrpb.txrdy = 0;
}

unsafe fn secondary_cpu_start(cpuid: c_int, idle: *mut TaskStruct) -> c_int {
    let cpu = (hwrpb as *mut u8).add(hwrpb.processor_offset as usize + cpuid as usize * hwrpb.processor_size as usize) as *mut PercpuStruct;
    let hwpcb = (*cpu).hwpcb as *mut PcbStruct; let ipcb = &mut (*task_thread_info(idle)).pcb;
    (*hwpcb).ksp = ipcb as *mut _ as c_ulong + core::mem::size_of::<ThreadUnion>() as c_ulong - 16;
    (*hwpcb).usp = 0; (*hwpcb).ptbr = ipcb.ptbr; (*hwpcb).pcc = 0; (*hwpcb).asn = 0;
    (*hwpcb).unique = virt_to_phys(ipcb); (*hwpcb).flags = ipcb.flags; (*hwpcb).res1 = 0; (*hwpcb).res2 = 0;
    hwrpb.CPU_restart = __smp_callin; hwrpb.CPU_restart_data = __smp_callin as c_ulong; hwrpb_update_checksum(&mut hwrpb);
    (*cpu).flags |= 0x22; (*cpu).flags &= !1; wmb(); send_secondary_console_msg(b"START\r\n\0".as_ptr() as *mut c_char, cpuid);
    let timeout = jiffies + 10 * HZ;
    while time_before(jiffies, timeout) { if (*cpu).flags & 1 != 0 { return 0; } udelay(10); barrier(); }
    printk(b"SMP: Processor %d failed to start.\n\0".as_ptr(), cpuid); -1
}

unsafe fn smp_boot_one_cpu(cpuid: c_int, idle: *mut TaskStruct) -> c_int {
    smp_secondary_alive = -1; if secondary_cpu_start(cpuid, idle) != 0 { return -1; }
    mb(); smp_secondary_alive = 0; let timeout = jiffies + HZ;
    while time_before(jiffies, timeout) { if smp_secondary_alive == 1 { return 0; } udelay(10); barrier(); }
    printk(b"SMP: Processor %d is stuck.\n\0".as_ptr(), cpuid); -1
}

pub unsafe fn setup_smp() {
    if boot_cpuid != 0 { printk(b"SMP: Booting off cpu %d instead of 0?\n\0".as_ptr(), boot_cpuid); }
    if hwrpb.nr_processors > 1 { let base = (hwrpb as *mut u8).add(hwrpb.processor_offset as usize); let pal = (*(base as *mut PercpuStruct)).pal_revision;
        for i in 0..hwrpb.nr_processors { let cpu = base.add(i as usize * hwrpb.processor_size as usize) as *mut PercpuStruct; if (*cpu).flags & 0x1cc == 0x1cc { smp_num_probed += 1; set_cpu_possible(i, true); set_cpu_present(i, true); (*cpu).pal_revision = pal; } }
    } else { smp_num_probed = 1; }
}

pub unsafe fn smp_prepare_cpus(max_cpus: c_uint) { memset(ipi_data.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&ipi_data)); (*current_thread_info()).cpu = boot_cpuid; smp_store_cpu_info(boot_cpuid); smp_setup_percpu_timer(boot_cpuid); if smp_num_probed == 1 || max_cpus == 0 { init_cpu_possible(cpumask_of(boot_cpuid)); init_cpu_present(cpumask_of(boot_cpuid)); return; } smp_num_cpus = smp_num_probed; }
pub unsafe fn __cpu_up(cpu: c_uint, tidle: *mut TaskStruct) -> c_int { smp_boot_one_cpu(cpu as c_int, tidle); if cpu_online(cpu as c_int) { 0 } else { -ENOSYS } }

pub unsafe fn smp_cpus_done(_max_cpus: c_uint) { let mut bogosum = 0; for cpu in 0..NR_CPUS { if cpu_online(cpu) { bogosum += cpu_data[cpu as usize].loops_per_jiffy; } } printk(b"SMP: Total of %d processors activated (%lu.%02lu BogoMIPS).\n\0".as_ptr(), num_online_cpus(), (bogosum + 2500)/(500000/HZ), ((bogosum+2500)/(5000/HZ))%100); }

unsafe fn send_ipi_message(to_whom: *const CpuMask, operation: IpiMessageType) { mb(); for_each_cpu(|i| { set_bit(operation as c_ulong, &mut ipi_data[i as usize].bits); }, to_whom); mb(); for_each_cpu(|i| wripir(i), to_whom); }
pub unsafe fn handle_ipi(_regs: *mut PtRegs) { let this_cpu=smp_processor_id(); let pending=&mut ipi_data[this_cpu as usize].bits; mb(); let mut ops=xchg(pending,0); while ops!=0 { mb(); while ops!=0 { let which=__ffs(ops & ops.wrapping_neg()); ops &= !(1<<which); match which { 0=>scheduler_ipi(), 1=>generic_smp_call_function_interrupt(), 2=>halt(), _=>{} } } mb(); } cpu_data[this_cpu as usize].ipi_count+=1; if hwrpb.txrdy!=0 { recv_secondary_console_msg(); } }
pub unsafe fn arch_smp_send_reschedule(cpu:c_int) { send_ipi_message(cpumask_of(cpu), IpiMessageType::IPI_RESCHEDULE); }
pub unsafe fn smp_send_stop() { let mut to=CpuMask::default(); cpumask_copy(&mut to,cpu_online_mask); cpumask_clear_cpu(smp_processor_id(),&mut to); send_ipi_message(&to,IpiMessageType::IPI_CPU_STOP); }
pub unsafe fn arch_send_call_function_ipi_mask(mask:*const CpuMask) { send_ipi_message(mask,IpiMessageType::IPI_CALL_FUNC); }
pub unsafe fn arch_send_call_function_single_ipi(cpu:c_int) { send_ipi_message(cpumask_of(cpu),IpiMessageType::IPI_CALL_FUNC); }
unsafe fn ipi_imb(_: *mut c_void) { imb(); }
pub unsafe fn smp_imb() { on_each_cpu(ipi_imb,core::ptr::null_mut(),1); }
unsafe fn ipi_flush_tlb_all(_: *mut c_void) { tbia(); }
pub unsafe fn flush_tlb_all() { on_each_cpu(ipi_flush_tlb_all,core::ptr::null_mut(),1); }
unsafe fn asn_locked() -> bool { cpu_data[smp_processor_id() as usize].asn_lock != 0 }
unsafe fn ipi_flush_tlb_mm(x:*mut c_void) { let mm=x as *mut MmStruct; if mm==(*current).active_mm && !asn_locked(){flush_tlb_current(mm)}else{flush_tlb_other(mm)} }
pub unsafe fn flush_tlb_mm(mm:*mut MmStruct) { preempt_disable(); if mm==(*current).active_mm { flush_tlb_current(mm); if atomic_read(&mut (*mm).mm_users)<=1 { for cpu in 0..NR_CPUS { if cpu_online(cpu)&&cpu!=smp_processor_id()&&(*mm).context[cpu as usize]!=0 {(*mm).context[cpu as usize]=0;} } preempt_enable(); return; } } smp_call_function(ipi_flush_tlb_mm,mm,1); preempt_enable(); }
unsafe fn ipi_flush_tlb_page(x:*mut c_void) { let d=&mut *(x as *mut FlushTlbPageStruct); if d.mm==(*current).active_mm&&!asn_locked(){flush_tlb_current_page(d.mm,d.vma,d.addr)}else{flush_tlb_other(d.mm)} }
pub unsafe fn flush_tlb_page(vma:*mut VmAreaStruct,addr:c_ulong) { let mut d=FlushTlbPageStruct{vma,mm:(*vma).vm_mm,addr}; flush_tlb_mm(d.mm); let _=d; }
pub unsafe fn flush_tlb_range(vma:*mut VmAreaStruct,_start:c_ulong,_end:c_ulong){flush_tlb_mm((*vma).vm_mm)}
unsafe fn ipi_flush_icache_page(x:*mut c_void){let mm=x as *mut MmStruct;if mm==(*current).active_mm&&!asn_locked(){__load_new_mm_context(mm)}else{flush_tlb_other(mm)}}
pub unsafe fn flush_icache_user_page(vma:*mut VmAreaStruct,_page:*mut Page,_addr:c_ulong,_len:c_int){let mm=(*vma).vm_mm;if (*vma).vm_flags&VM_EXEC==0{return;}preempt_disable();if mm==(*current).active_mm{__load_new_mm_context(mm);if atomic_read(&mut (*mm).mm_users)<=1{for cpu in 0..NR_CPUS{if cpu_online(cpu)&&cpu!=smp_processor_id()&&(*mm).context[cpu as usize]!=0{(*mm).context[cpu as usize]=0;}}preempt_enable();return;}}smp_call_function(ipi_flush_icache_page,mm,1);preempt_enable();}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
