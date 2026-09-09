// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of the Loongson64 SMP implementation. */

// Kernel headers and symbols are supplied by the surrounding kernel translation.

type U32 = u32;
type U64 = u64;
type CInt = i32;
type Iomem = *mut u8;

#[repr(C)] pub struct TaskStruct { _private: [u8; 0] }
#[repr(C)] pub struct CpuMask { _private: [u8; 0] }
#[repr(C)] pub struct CpuData { pub package: CInt }
#[allow(non_camel_case_types)] type irqreturn_t = CInt;

extern "C" {
    static mut cpu_state: CInt;
    static mut ipi_set0_regs: [Iomem; 16];
    static mut ipi_clear0_regs: [Iomem; 16];
    static mut ipi_status0_regs: [Iomem; 16];
    static mut ipi_en0_regs: [Iomem; 16];
    static mut ipi_mailbox_buf: [Iomem; 16];
    static mut loongson_sysconf: LoongsonSysconf;
    static mut cpu_data: [CpuData; 256];
    static mut __cpu_number_map: [CInt; 256];
    static mut __cpu_logical_map: [CInt; 256];
    static mut smp_group: [*mut u8; 16];
    fn smp_bootstrap();
    fn cpu_has_csr() -> CInt;
    fn csr_readl(reg: U64) -> U32;
    fn csr_writel(v: U32, reg: U64);
    fn csr_writeq(v: U64, reg: U64);
    fn readl_relaxed(p: Iomem) -> U32;
    fn writel_relaxed(v: U32, p: Iomem);
    fn writeq_relaxed(v: U64, p: Iomem);
    fn nudge_writes();
    fn cpu_logical_map(cpu: CInt) -> CInt;
    fn smp_processor_id() -> CInt;
    fn task_thread_info(t: *mut TaskStruct) -> *mut u8;
    fn __KSTK_TOS(t: *mut TaskStruct) -> *mut u8;
    fn pr_debug(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn ffs(v: U32) -> CInt;
    fn scheduler_ipi();
    fn irq_enter(); fn irq_exit(); fn generic_smp_call_function_interrupt(); fn irq_work_run();
    fn irq_local_enable();
    fn change_c0_status(mask: U32, value: U32);
    fn write_c0_compare(v: U32); fn read_c0_count() -> U32; fn read_c0_status() -> U32;
    fn request_irq(irq: CInt, handler: unsafe extern "C" fn(CInt, *mut u8) -> irqreturn_t, flags: U32, name: *const u8, dev: *mut u8) -> CInt;
    fn init_cpu_possible(mask: *const u8); fn init_cpu_present(mask: *const u8);
    fn set_cpu_possible(cpu: CInt, possible: bool);
    fn cpu_set_core(data: *mut CpuData, core: CInt); fn cpu_set_cluster(data: *mut CpuData, cluster: CInt);
}

#[repr(C)] pub struct LoongsonSysconf { pub cores_per_node: CInt, pub nr_cpus: CInt, pub reserved_cpus_mask: U32, pub cores_per_package: CInt }

const LS_IPI_IRQ: CInt = MIPS_CPU_IRQ_BASE + 6;
const MIPS_CPU_IRQ_BASE: CInt = 0; // supplied by the target platform
const IRQ_HANDLED: irqreturn_t = 1;
const ST0_IM: U32 = 0; const STATUSF_IP7: U32 = 0; const STATUSF_IP6: U32 = 0; const STATUSF_IP3: U32 = 0; const STATUSF_IP2: U32 = 0;
const CPU_ONLINE: CInt = 1; const SMP_RESCHEDULE_YOURSELF: U32 = 1; const SMP_CALL_FUNCTION: U32 = 2; const SMP_IRQ_WORK: U32 = 4;
const IRQF_PERCPU: U32 = 0; const IRQF_NO_SUSPEND: U32 = 0;
const HZ: U32 = 100;
extern "C" { static mips_hpt_frequency: U32; }

static mut ipi_read_clear: Option<unsafe extern "C" fn(CInt) -> U32> = None;
static mut ipi_write_action: Option<unsafe extern "C" fn(CInt, U32)> = None;
static mut ipi_write_enable: Option<unsafe extern "C" fn(CInt)> = None;
static mut ipi_clear_buf: Option<unsafe extern "C" fn(CInt)> = None;
static mut ipi_write_buf: Option<unsafe extern "C" fn(CInt, *mut TaskStruct)> = None;

unsafe fn csr_mail_send(data: U64, cpu: CInt, mailbox: CInt) {
    let mut val = CSR_MAIL_SEND_BLOCK | (CSR_MAIL_SEND_BOX_HIGH(mailbox) << CSR_MAIL_SEND_BOX_SHIFT) | ((cpu as U64) << CSR_MAIL_SEND_CPU_SHIFT) | (data & CSR_MAIL_SEND_H32_MASK);
    csr_writeq(val, LOONGSON_CSR_MAIL_SEND);
    val = CSR_MAIL_SEND_BLOCK | (CSR_MAIL_SEND_BOX_LOW(mailbox) << CSR_MAIL_SEND_BOX_SHIFT) | ((cpu as U64) << CSR_MAIL_SEND_CPU_SHIFT) | (data << CSR_MAIL_SEND_BUF_SHIFT);
    csr_writeq(val, LOONGSON_CSR_MAIL_SEND);
}
unsafe extern "C" fn csr_ipi_read_clear(_: CInt) -> U32 { let action = csr_readl(LOONGSON_CSR_IPI_STATUS); csr_writel(action, LOONGSON_CSR_IPI_CLEAR); action }
unsafe extern "C" fn csr_ipi_write_action(cpu: CInt, mut action: U32) { while action != 0 { let irq = ffs(action) as U32; let val = CSR_IPI_SEND_BLOCK | (irq - 1) | ((cpu as U32) << CSR_IPI_SEND_CPU_SHIFT); csr_writel(val, LOONGSON_CSR_IPI_SEND); action &= !(1u32 << (irq - 1)); } }
unsafe extern "C" fn csr_ipi_write_enable(_: CInt) { csr_writel(0xffffffff, LOONGSON_CSR_IPI_EN); }
unsafe extern "C" fn csr_ipi_clear_buf(_: CInt) { csr_writeq(0, LOONGSON_CSR_MAIL_BUF0); }
unsafe extern "C" fn csr_ipi_write_buf(cpu: CInt, idle: *mut TaskStruct) { let a = [smp_bootstrap as usize as U64, __KSTK_TOS(idle) as usize as U64, task_thread_info(idle) as usize as U64, 0]; csr_mail_send(a[3], cpu_logical_map(cpu), 3); csr_mail_send(a[2], cpu_logical_map(cpu), 2); csr_mail_send(a[1], cpu_logical_map(cpu), 1); csr_mail_send(a[0], cpu_logical_map(cpu), 0); }
unsafe extern "C" fn legacy_ipi_read_clear(cpu: CInt) -> U32 { let a = readl_relaxed(ipi_status0_regs[cpu_logical_map(cpu) as usize]); writel_relaxed(a, ipi_clear0_regs[cpu_logical_map(cpu) as usize]); nudge_writes(); a }
unsafe extern "C" fn legacy_ipi_write_action(cpu: CInt, action: U32) { writel_relaxed(action, ipi_set0_regs[cpu as usize]); nudge_writes(); }
unsafe extern "C" fn legacy_ipi_write_enable(cpu: CInt) { writel_relaxed(0xffffffff, ipi_en0_regs[cpu_logical_map(cpu) as usize]); }
unsafe extern "C" fn legacy_ipi_clear_buf(cpu: CInt) { writeq_relaxed(0, ipi_mailbox_buf[cpu_logical_map(cpu) as usize].add(0)); }
unsafe extern "C" fn legacy_ipi_write_buf(cpu: CInt, idle: *mut TaskStruct) { let a = [smp_bootstrap as usize as U64, __KSTK_TOS(idle) as usize as U64, task_thread_info(idle) as usize as U64, 0]; let p = ipi_mailbox_buf[cpu_logical_map(cpu) as usize]; writeq_relaxed(a[3],p.add(0x18)); writeq_relaxed(a[2],p.add(0x10)); writeq_relaxed(a[1],p.add(8)); writeq_relaxed(a[0],p); nudge_writes(); }

unsafe fn csr_ipi_probe() { if cpu_has_csr() != 0 && csr_readl(LOONGSON_CSR_FEATURES) & LOONGSON_CSRF_IPI != 0 { ipi_read_clear=Some(csr_ipi_read_clear); ipi_write_action=Some(csr_ipi_write_action); ipi_write_enable=Some(csr_ipi_write_enable); ipi_clear_buf=Some(csr_ipi_clear_buf); ipi_write_buf=Some(csr_ipi_write_buf); } else { ipi_read_clear=Some(legacy_ipi_read_clear); ipi_write_action=Some(legacy_ipi_write_action); ipi_write_enable=Some(legacy_ipi_write_enable); ipi_clear_buf=Some(legacy_ipi_clear_buf); ipi_write_buf=Some(legacy_ipi_write_buf); } }

unsafe fn init_regs() { let groups = [SMP_CORE_GROUP0_BASE,SMP_CORE_GROUP1_BASE,SMP_CORE_GROUP2_BASE,SMP_CORE_GROUP3_BASE]; let offs=[SMP_CORE0_OFFSET,SMP_CORE1_OFFSET,SMP_CORE2_OFFSET,SMP_CORE3_OFFSET]; for g in 0..4 { for c in 0..4 { let n=g*4+c; ipi_set0_regs[n]=(groups[g]+offs[c]+SET0) as Iomem; ipi_clear0_regs[n]=(groups[g]+offs[c]+CLEAR0) as Iomem; ipi_status0_regs[n]=(groups[g]+offs[c]+STATUS0) as Iomem; ipi_en0_regs[n]=(groups[g]+offs[c]+EN0) as Iomem; ipi_mailbox_buf[n]=(groups[g]+offs[c]+BUF) as Iomem; } } }

unsafe fn loongson3_send_ipi_single(cpu: CInt, action: U32) { (ipi_write_action.unwrap())(cpu_logical_map(cpu), action); }
unsafe fn loongson3_send_ipi_mask(mask: *const CpuMask, action: U32) { for_each_cpu(|i| (ipi_write_action.unwrap())(cpu_logical_map(i),action), mask); }
unsafe extern "C" fn loongson3_ipi_interrupt(_: CInt, _: *mut u8) -> irqreturn_t { let cpu=smp_processor_id(); let action=(ipi_read_clear.unwrap())(cpu); if action&SMP_RESCHEDULE_YOURSELF!=0 { scheduler_ipi(); } if action&SMP_CALL_FUNCTION!=0 { irq_enter(); generic_smp_call_function_interrupt(); irq_exit(); } if action&SMP_IRQ_WORK!=0 { irq_work_run(); } IRQ_HANDLED }

unsafe fn loongson3_init_secondary() { let cpu=smp_processor_id(); change_c0_status(ST0_IM,STATUSF_IP7|STATUSF_IP6|STATUSF_IP3|STATUSF_IP2); (ipi_write_enable.unwrap())(cpu); cpu_state=CPU_ONLINE; cpu_set_core(&mut cpu_data[cpu as usize],cpu_logical_map(cpu)%loongson_sysconf.cores_per_package); cpu_data[cpu as usize].package=cpu_logical_map(cpu)/loongson_sysconf.cores_per_package; }
unsafe fn loongson3_smp_finish() { let cpu=smp_processor_id(); write_c0_compare(read_c0_count()+mips_hpt_frequency/HZ); irq_local_enable(); (ipi_clear_buf.unwrap())(cpu); }
unsafe fn loongson3_smp_setup() { init_cpu_possible(core::ptr::null()); let mut max=0; for i in 0..16 { if smp_group[i].is_null(){break} max+=loongson_sysconf.cores_per_node; } if max<loongson_sysconf.nr_cpus { loongson_sysconf.nr_cpus=if max!=0{max}else{1}; } let mut num=0; for i in 0..loongson_sysconf.nr_cpus { if loongson_sysconf.reserved_cpus_mask&(1u32<<i)!=0 { __cpu_number_map[i as usize]=-1; } else { __cpu_number_map[i as usize]=num; __cpu_logical_map[num as usize]=i; set_cpu_possible(num,true); cpu_set_cluster(&mut cpu_data[num as usize],i/4); num+=1; } } while num<loongson_sysconf.nr_cpus { __cpu_logical_map[num as usize]=-1; num+=1; } csr_ipi_probe(); init_regs(); if !smp_group[0].is_null(){(ipi_write_enable.unwrap())(0);} cpu_set_core(&mut cpu_data[0],cpu_logical_map(0)%loongson_sysconf.cores_per_package); cpu_data[0].package=cpu_logical_map(0)/loongson_sysconf.cores_per_package; }
unsafe fn loongson3_prepare_cpus(_: U32) { if request_irq(LS_IPI_IRQ,loongson3_ipi_interrupt,IRQF_PERCPU|IRQF_NO_SUSPEND,b"SMP_IPI\0".as_ptr() as *const u8,core::ptr::null_mut())!=0 { pr_err(b"Failed to request IPI IRQ\n\0".as_ptr()); } init_cpu_present(core::ptr::null()); cpu_state=CPU_ONLINE; }
unsafe fn loongson3_boot_secondary(cpu: CInt, idle: *mut TaskStruct) -> CInt { (ipi_write_buf.unwrap())(cpu,idle); 0 }

// External constants/macros referenced above are provided by the target kernel translation.
extern "C" { static CSR_MAIL_SEND_BLOCK: U64; static CSR_MAIL_SEND_BOX_SHIFT: U32; static CSR_MAIL_SEND_CPU_SHIFT: U32; static CSR_MAIL_SEND_H32_MASK: U64; static CSR_MAIL_SEND_BUF_SHIFT: U32; static LOONGSON_CSR_MAIL_SEND: U64; static LOONGSON_CSR_IPI_STATUS: U64; static LOONGSON_CSR_IPI_CLEAR: U64; static LOONGSON_CSR_IPI_SEND_BLOCK: U32; static LOONGSON_CSR_IPI_SEND_CPU_SHIFT: U32; static LOONGSON_CSR_IPI_SEND: U64; static LOONGSON_CSR_IPI_EN: U64; static LOONGSON_CSR_MAIL_BUF0: U64; static LOONGSON_CSR_FEATURES: U64; static LOONGSON_CSRF_IPI: U32; static SMP_CORE_GROUP0_BASE: usize; static SMP_CORE_GROUP1_BASE: usize; static SMP_CORE_GROUP2_BASE: usize; static SMP_CORE_GROUP3_BASE: usize; static SMP_CORE0_OFFSET: usize; static SMP_CORE1_OFFSET: usize; static SMP_CORE2_OFFSET: usize; static SMP_CORE3_OFFSET: usize; static SET0: usize; static CLEAR0: usize; static STATUS0: usize; static EN0: usize; static BUF: usize; }
extern "C" { fn CSR_MAIL_SEND_BOX_HIGH(x:CInt)->U64; fn CSR_MAIL_SEND_BOX_LOW(x:CInt)->U64; fn for_each_cpu(f: unsafe extern "C" fn(CInt), mask:*const CpuMask); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
