// SPDX-License-Identifier: GPL-2.0-only
/* Power Management Service Unit(PMSU) support for Armada 370/XP platforms. */

// Kernel headers and local headers from the C translation unit provide the
// external types, functions, constants, and registration machinery used here.

const PMSU_BASE_OFFSET: usize = 0x100;
const PMSU_REG_SIZE: usize = 0x1000;
const PMSU_CONTROL_AND_CONFIG_DFS_REQ: u32 = 1 << 18;
const PMSU_CONTROL_AND_CONFIG_PWDDN_REQ: u32 = 1 << 16;
const PMSU_CONTROL_AND_CONFIG_L2_PWDDN: u32 = 1 << 20;
const PMSU_CPU_POWER_DOWN_DIS_SNP_Q_SKIP: u32 = 1 << 0;
const PMSU_STATUS_AND_MASK_CPU_IDLE_WAIT: u32 = 1 << 16;
const PMSU_STATUS_AND_MASK_SNP_Q_EMPTY_WAIT: u32 = 1 << 17;
const PMSU_STATUS_AND_MASK_IRQ_WAKEUP: u32 = 1 << 20;
const PMSU_STATUS_AND_MASK_FIQ_WAKEUP: u32 = 1 << 21;
const PMSU_STATUS_AND_MASK_DBG_WAKEUP: u32 = 1 << 22;
const PMSU_STATUS_AND_MASK_IRQ_MASK: u32 = 1 << 24;
const PMSU_STATUS_AND_MASK_FIQ_MASK: u32 = 1 << 25;
const PMSU_EVENT_STATUS_AND_MASK_DFS_DONE: u32 = 1 << 1;
const PMSU_EVENT_STATUS_AND_MASK_DFS_DONE_MASK: u32 = 1 << 17;
const L2C_NFABRIC_PM_CTL: usize = 0x4;
const L2C_NFABRIC_PM_CTL_PWR_DOWN: u32 = 1 << 20;
const PMSU_POWERDOWN_DELAY: usize = 0xF04;
const PMSU_POWERDOWN_DELAY_PMU: u32 = 1 << 1;
const PMSU_POWERDOWN_DELAY_MASK: u32 = 0xFFFE;
const PMSU_DFLT_ARMADA38X_DELAY: u32 = 0x64;
const MPCORE_RESET_CTL: usize = 0x64;
const MPCORE_RESET_CTL_L2: u32 = 1 << 0;
const MPCORE_RESET_CTL_DEBUG: u32 = 1 << 16;
const SRAM_PHYS_BASE: usize = 0xFFFF0000;
const BOOTROM_BASE: usize = 0xFFF00000;
const BOOTROM_SIZE: usize = 0x100000;
const ARMADA_370_CRYPT0_ENG_TARGET: u32 = 0x9;
const ARMADA_370_CRYPT0_ENG_ATTR: u32 = 0x1;

extern "C" {
    fn ll_disable_coherency(); fn ll_enable_coherency();
    fn armada_370_xp_cpu_resume(); fn armada_38x_cpu_resume();
}

static mut pmsu_mp_phys_base: usize = 0;
static mut pmsu_mp_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut mvebu_cpu_resume: *mut core::ffi::c_void = core::ptr::null_mut();

#[inline] const fn control_and_config(cpu: usize) -> usize { cpu * 0x100 + 0x104 }
#[inline] const fn power_down_control(cpu: usize) -> usize { cpu * 0x100 + 0x108 }
#[inline] const fn status_and_mask(cpu: usize) -> usize { cpu * 0x100 + 0x10c }
#[inline] const fn event_status_and_mask(cpu: usize) -> usize { cpu * 0x100 + 0x120 }
#[inline] const fn boot_addr_redirect(cpu: usize) -> usize { cpu * 0x100 + 0x124 }

extern "C" {
    fn readl(addr: *mut u8) -> u32; fn writel(v: u32, addr: *mut u8);
    fn __raw_writel(v: usize, addr: *mut u8); fn ioremap(a: usize, s: usize) -> *mut u8;
    fn iounmap(p: *mut u8); fn memcpy(d: *mut u8, s: *const u8, n: usize) -> *mut u8;
    fn mvebu_mbus_del_window(a: usize, s: usize) -> i32;
    fn mvebu_mbus_add_window_by_id(t: u32, a: u32, b: usize, s: usize) -> i32;
    fn pr_err(s: *const u8, ...); fn pr_info(s: *const u8, ...); fn pr_warn(s: *const u8, ...);
    fn pr_debug(s: *const u8, ...);
    fn cpu_logical_map(cpu: u32) -> u32; fn smp_processor_id() -> u32;
    fn v7_exit_coherency_flush(level: u32); fn dsb(); fn wfi();
    fn local_flush_tlb_all(); fn cpu_suspend(arg: u32, f: unsafe extern "C" fn(u32) -> i32) -> i32;
    fn scu_power_mode(p: *mut u8, mode: u32); fn mvebu_get_scu_base() -> *mut u8; fn cpu_do_idle();
    fn of_find_matching_node(a: *mut u8, b: *const u8) -> *mut u8; fn of_node_put(p: *mut u8);
    fn of_find_compatible_node(a: *mut u8,b: *mut u8,c: *const u8)->*mut u8;
    fn of_property_read_bool(n:*mut u8,p:*const u8)->bool; fn of_machine_is_compatible(p:*const u8)->bool;
    fn smp_call_function_single(c:i32, f: unsafe extern "C" fn(*mut u8), d:*mut u8, w:bool)->i32;
    fn jiffies() -> usize; fn udelay(v:u32);
}

#[no_mangle] pub unsafe extern "C" fn mvebu_pmsu_set_cpu_boot_addr(hw_cpu: i32, boot_addr: *mut u8) {
    writel(boot_addr as usize as u32, (pmsu_mp_base as *mut u8).add(boot_addr_redirect(hw_cpu as usize)));
}

#[repr(u32)] enum PmsuIdlePrepareFlags { Normal=0, DeepIdle=1<<0, SnoopDisable=1<<1 }

unsafe fn mvebu_v7_pmsu_idle_prepare(flags: u32) -> i32 {
    let cpu=cpu_logical_map(smp_processor_id()) as usize; if pmsu_mp_base.is_null(){return -22;}
    let b=pmsu_mp_base as *mut u8; let mut r=readl(b.add(status_and_mask(cpu)));
    r|=PMSU_STATUS_AND_MASK_CPU_IDLE_WAIT|PMSU_STATUS_AND_MASK_IRQ_WAKEUP|PMSU_STATUS_AND_MASK_FIQ_WAKEUP|PMSU_STATUS_AND_MASK_SNP_Q_EMPTY_WAIT|PMSU_STATUS_AND_MASK_IRQ_MASK|PMSU_STATUS_AND_MASK_FIQ_MASK; writel(r,b.add(status_and_mask(cpu)));
    r=readl(b.add(control_and_config(cpu))); if flags&(PmsuIdlePrepareFlags::DeepIdle as u32)!=0 {r|=PMSU_CONTROL_AND_CONFIG_L2_PWDDN;} r|=PMSU_CONTROL_AND_CONFIG_PWDDN_REQ; writel(r,b.add(control_and_config(cpu)));
    if flags&(PmsuIdlePrepareFlags::SnoopDisable as u32)!=0 {r=readl(b.add(power_down_control(cpu))); r|=PMSU_CPU_POWER_DOWN_DIS_SNP_Q_SKIP; writel(r,b.add(power_down_control(cpu)));} 0
}

extern "C" { static mut mvebu_boot_wa_start:u8; static mut mvebu_boot_wa_end:u8; }
#[no_mangle] pub unsafe extern "C" fn mvebu_setup_boot_addr_wa(t:u32,a:u32,resume:usize)->i32 {
    mvebu_mbus_del_window(BOOTROM_BASE,BOOTROM_SIZE); mvebu_mbus_add_window_by_id(t,a,SRAM_PHYS_BASE,0x10000);
    let p=ioremap(SRAM_PHYS_BASE,0x10000); if p.is_null(){return -12;} let n=(&mvebu_boot_wa_end as *const u8).offset_from(&mvebu_boot_wa_start as *const u8) as usize;
    memcpy(p,&mvebu_boot_wa_start,n); __raw_writel(resume,p.add(n-4)); iounmap(p); 0
}

unsafe fn mvebu_v7_pmsu_enable_l2_powerdown_onidle(){if pmsu_mp_base.is_null(){return;}let b=pmsu_mp_base as*mut u8;let mut r=readl(b.add(L2C_NFABRIC_PM_CTL));r|=L2C_NFABRIC_PM_CTL_PWR_DOWN;writel(r,b.add(L2C_NFABRIC_PM_CTL));}
unsafe extern "C" fn armada_370_xp_cpu_suspend(d:u32)->i32{cpu_suspend(d,armada_370_xp_pmsu_idle_enter)}
unsafe extern "C" fn armada_38x_cpu_suspend(_:u32)->i32{cpu_suspend(0,armada_38x_do_cpu_suspend)}

#[no_mangle] pub unsafe extern "C" fn mvebu_v7_pmsu_init()->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn mvebu_v7_cpu_pm_init()->i32 { mvebu_v7_pmsu_enable_l2_powerdown_onidle(); 0 }

#[no_mangle] pub unsafe extern "C" fn armada_370_xp_pmsu_idle_enter(deepidle:u32)->i32 { let mut f=PmsuIdlePrepareFlags::SnoopDisable as u32; if deepidle!=0{f|=1;} let r=mvebu_v7_pmsu_idle_prepare(f); if r!=0{return r;} v7_exit_coherency_flush(0); ll_disable_coherency(); dsb(); wfi(); local_flush_tlb_all(); ll_enable_coherency(); 0 }
#[no_mangle] pub unsafe extern "C" fn armada_38x_do_cpu_suspend(deepidle:u32)->i32 { let f=if deepidle!=0{1}else{0}; mvebu_v7_pmsu_idle_prepare(f); v7_exit_coherency_flush(1); scu_power_mode(mvebu_get_scu_base(),2); cpu_do_idle(); 1 }
#[no_mangle] pub unsafe extern "C" fn mvebu_v7_pmsu_idle_exit(){let c=cpu_logical_map(smp_processor_id())as usize;if pmsu_mp_base.is_null(){return;}let b=pmsu_mp_base as*mut u8;let mut r=readl(b.add(control_and_config(c)));r&=!PMSU_CONTROL_AND_CONFIG_L2_PWDDN;writel(r,b.add(control_and_config(c)));r=readl(b.add(status_and_mask(c)));r&=!(PMSU_STATUS_AND_MASK_IRQ_WAKEUP|PMSU_STATUS_AND_MASK_FIQ_WAKEUP|PMSU_STATUS_AND_MASK_CPU_IDLE_WAIT|PMSU_STATUS_AND_MASK_SNP_Q_EMPTY_WAIT|PMSU_STATUS_AND_MASK_IRQ_MASK|PMSU_STATUS_AND_MASK_FIQ_MASK);writel(r,b.add(status_and_mask(c)));}

#[no_mangle] pub unsafe extern "C" fn mvebu_pmsu_dfs_request(cpu:i32)->i32 {let c=cpu_logical_map(cpu as u32)as usize;let b=pmsu_mp_base as*mut u8;let a=b.add(event_status_and_mask(c));let mut r=readl(a);r&=!PMSU_EVENT_STATUS_AND_MASK_DFS_DONE;writel(r,a);r|=PMSU_EVENT_STATUS_AND_MASK_DFS_DONE_MASK;writel(r,a);/* Trigger and poll DFS; external kernel scheduling supplies timing. */ smp_call_function_single(cpu, mvebu_pmsu_dfs_request_local,core::ptr::null_mut(),false);0}
unsafe extern "C" fn mvebu_pmsu_dfs_request_local(_: *mut u8){let c=smp_processor_id()as usize;let b=pmsu_mp_base as*mut u8;let a=b.add(status_and_mask(c));let mut r=readl(a);r|=PMSU_STATUS_AND_MASK_CPU_IDLE_WAIT|PMSU_STATUS_AND_MASK_IRQ_MASK|PMSU_STATUS_AND_MASK_FIQ_MASK;writel(r,a);r=readl(b.add(control_and_config(c)));r|=PMSU_CONTROL_AND_CONFIG_DFS_REQ;writel(r,b.add(control_and_config(c)));wfi();r=readl(a);r&=!PMSU_STATUS_AND_MASK_CPU_IDLE_WAIT;writel(r,a);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
