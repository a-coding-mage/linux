/* VMware Detection code. Rust translation of vmware.c. */

use core::mem::MaybeUninit;

const CPUID_VMWARE_INFO_LEAF: u32 = 0x40000000;
const CPUID_VMWARE_FEATURES_LEAF: u32 = 0x40000010;
const GETVCPU_INFO_LEGACY_X2APIC: u32 = 1 << 3;
const GETVCPU_INFO_VCPU_RESERVED: u32 = 1 << 31;
const STEALCLOCK_NOT_AVAILABLE: i32 = -1;
const STEALCLOCK_DISABLED: i32 = 0;
const STEALCLOCK_ENABLED: i32 = 1;

#[repr(C)]
pub union VmwareStealTimeClock { pub clock: u64, pub parts: VmwareStealTimeParts }
#[repr(C)]
pub struct VmwareStealTimeParts { pub clock_low: u32, pub clock_high: u32 }
#[repr(C)]
pub struct VmwareStealTime { pub clock: VmwareStealTimeClock, pub reserved: [u64; 7] }

static mut VMWARE_TSC_KHZ: usize = 0;
static mut VMWARE_HYPERCALL_MODE: u8 = 0;

pub unsafe fn vmware_hypercall_slow(cmd: usize, in1: usize, in3: usize, in4: usize,
    in5: usize, out1: *mut u32, out2: *mut u32, out3: *mut u32, out4: *mut u32,
    out5: *mut u32) -> usize {
    let (mut out0, mut rbx, mut rcx, mut rdx, mut rsi, mut rdi) = (0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
    match VMWARE_HYPERCALL_MODE {
        CPUID_VMWARE_FEATURES_ECX_VMCALL => core::arch::asm!("vmcall", inlateout("rax") VMWARE_HYPERVISOR_MAGIC as usize => out0, inlateout("rbx") in1 => rbx, inlateout("rcx") cmd => rcx, inlateout("rdx") in3 => rdx, inlateout("rsi") in4 => rsi, inlateout("rdi") in5 => rdi, options(nostack)),
        CPUID_VMWARE_FEATURES_ECX_VMMCALL => core::arch::asm!("vmmcall", inlateout("rax") VMWARE_HYPERVISOR_MAGIC as usize => out0, inlateout("rbx") in1 => rbx, inlateout("rcx") cmd => rcx, inlateout("rdx") in3 => rdx, inlateout("rsi") in4 => rsi, inlateout("rdi") in5 => rdi, options(nostack)),
        _ => core::arch::asm!("mov dx, {port:e}; in eax, dx", port = const VMWARE_HYPERVISOR_PORT, inlateout("rax") VMWARE_HYPERVISOR_MAGIC as usize => out0, inlateout("rbx") in1 => rbx, inlateout("rcx") cmd => rcx, inlateout("rdx") in3 => rdx, inlateout("rsi") in4 => rsi, inlateout("rdi") in5 => rdi, options(nostack)),
    }
    if !out1.is_null() { *out1 = rbx as u32; } if !out2.is_null() { *out2 = rcx as u32; }
    if !out3.is_null() { *out3 = rdx as u32; } if !out4.is_null() { *out4 = rsi as u32; }
    if !out5.is_null() { *out5 = rdi as u32; } out0
}

unsafe fn __vmware_platform() -> bool { let mut ebx=0; let mut ecx=0; let eax=vmware_hypercall3(VMWARE_CMD_GETVERSION,0,&mut ebx,&mut ecx); eax != u32::MAX && ebx == VMWARE_HYPERVISOR_MAGIC }
unsafe fn vmware_get_tsc_khz() -> usize { VMWARE_TSC_KHZ }

#[cfg(feature = "CONFIG_PARAVIRT")]
mod paravirt {
    use super::*;
    static mut VMWARE_CYC2NS: Cyc2nsData = Cyc2nsData { cyc2ns_mul:0, cyc2ns_shift:0, cyc2ns_offset:0 };
    static mut VMW_SCHED_CLOCK: bool = true; static mut HAS_STEAL_CLOCK: bool = false; static mut STEAL_ACC: bool = true;
    #[repr(C)] pub struct Cyc2nsData { pub cyc2ns_mul:u32, pub cyc2ns_shift:u32, pub cyc2ns_offset:u64 }
    pub unsafe fn setup_vmw_sched_clock(_: *mut u8)->i32 { VMW_SCHED_CLOCK=false; 0 }
    pub unsafe fn parse_no_stealacc(_: *mut u8)->i32 { STEAL_ACC=false; 0 }
    pub unsafe fn vmware_sched_clock()->u64 { mul_u64_u32_shr(rdtsc(), VMWARE_CYC2NS.cyc2ns_mul, VMWARE_CYC2NS.cyc2ns_shift).wrapping_sub(VMWARE_CYC2NS.cyc2ns_offset) }
    pub unsafe fn vmware_cyc2ns_setup() { let t=rdtsc(); clocks_calc_mult_shift(&mut VMWARE_CYC2NS.cyc2ns_mul,&mut VMWARE_CYC2NS.cyc2ns_shift,VMWARE_TSC_KHZ, NSEC_PER_MSEC,0); VMWARE_CYC2NS.cyc2ns_offset=mul_u64_u32_shr(t,VMWARE_CYC2NS.cyc2ns_mul,VMWARE_CYC2NS.cyc2ns_shift); }
    pub unsafe fn vmware_cmd_stealclock(hi:u32,lo:u32)->i32 { let mut info=0; vmware_hypercall5(VMWARE_CMD_STEALCLOCK,0,0,hi as usize,lo as usize,&mut info) }
    pub unsafe fn stealclock_enable(pa:usize)->bool { vmware_cmd_stealclock((pa>>32) as u32,pa as u32)==STEALCLOCK_ENABLED }
    pub unsafe fn __stealclock_disable()->i32 { vmware_cmd_stealclock(0,1) }
    pub unsafe fn stealclock_disable(){__stealclock_disable();} pub unsafe fn vmware_is_stealclock_available()->bool{__stealclock_disable()!=STEALCLOCK_NOT_AVAILABLE}
    pub unsafe fn vmware_steal_clock(cpu:i32)->u64 { let st=per_cpu_steal_time(cpu); let clock=if cfg!(target_pointer_width="64"){core::ptr::read_volatile(&(*st).clock.clock)}else{let p=&(*st).clock.parts; loop{let h=core::ptr::read_volatile(&p.clock_high); virt_rmb(); let l=core::ptr::read_volatile(&p.clock_low); virt_rmb(); if h==core::ptr::read_volatile(&p.clock_high){break ((h as u64)<<32)|l as u64;}}}; mul_u64_u32_shr(clock,VMWARE_CYC2NS.cyc2ns_mul,VMWARE_CYC2NS.cyc2ns_shift) }
}

/* The following declarations and platform hooks retain the C translation's external kernel dependencies. */
unsafe extern "C" { static mut x86_hyper_vmware: HypervisorX86; }
#[repr(C)] pub struct HypervisorX86 { pub name:*const u8, pub detect:Option<unsafe fn()->u32>, pub type_:u32 }

#[cfg(feature = "CONFIG_INTEL_TDX_GUEST")]
pub unsafe fn vmware_tdx_hypercall(cmd:usize,in1:usize,in3:usize,in4:usize,in5:usize,out1:*mut u32,out2:*mut u32,out3:*mut u32,out4:*mut u32,out5:*mut u32)->usize { let mut args=MaybeUninit::<TdxModuleArgs>::zeroed().assume_init(); if !hypervisor_is_type(X86_HYPER_VMWARE)||cmd & !VMWARE_CMD_MASK != 0{return usize::MAX;} args.rbx=in1;args.rdx=in3;args.rsi=in4;args.rdi=in5;args.r10=VMWARE_TDX_VENDOR_LEAF;args.r11=VMWARE_TDX_HCALL_FUNC;args.r12=VMWARE_HYPERVISOR_MAGIC as usize;args.r13=cmd;__tdx_hypercall(&mut args);if !out1.is_null(){*out1=args.rbx as u32}if !out2.is_null(){*out2=args.r13 as u32}if !out3.is_null(){*out3=args.rdx as u32}if !out4.is_null(){*out4=args.rsi as u32}if !out5.is_null(){*out5=args.rdi as u32}args.r12 }

#[cfg(feature = "CONFIG_AMD_MEM_ENCRYPT")]
pub unsafe fn vmware_sev_es_hcall_prepare(ghcb:*mut Ghcb, regs:*mut PtRegs){ghcb_set_rip(ghcb,(*regs).ip);ghcb_set_rbx(ghcb,(*regs).bx);ghcb_set_rcx(ghcb,(*regs).cx);ghcb_set_rdx(ghcb,(*regs).dx);ghcb_set_rsi(ghcb,(*regs).si);ghcb_set_rdi(ghcb,(*regs).di);ghcb_set_rbp(ghcb,(*regs).bp);}
#[cfg(feature = "CONFIG_AMD_MEM_ENCRYPT")]
pub unsafe fn vmware_sev_es_hcall_finish(ghcb:*mut Ghcb, regs:*mut PtRegs)->bool {if !(ghcb_rbx_is_valid(ghcb)&&ghcb_rcx_is_valid(ghcb)&&ghcb_rdx_is_valid(ghcb)&&ghcb_rsi_is_valid(ghcb)&&ghcb_rdi_is_valid(ghcb)&&ghcb_rbp_is_valid(ghcb)){return false;}(*regs).bx=ghcb_get_rbx(ghcb);(*regs).cx=ghcb_get_rcx(ghcb);(*regs).dx=ghcb_get_rdx(ghcb);(*regs).si=ghcb_get_rsi(ghcb);(*regs).di=ghcb_get_rdi(ghcb);(*regs).bp=ghcb_get_rbp(ghcb);true}

pub unsafe fn vmware_legacy_x2apic_available()->bool {let eax=vmware_hypercall1(VMWARE_CMD_GETVCPU_INFO,0);eax&GETVCPU_INFO_VCPU_RESERVED==0&&eax&GETVCPU_INFO_LEGACY_X2APIC!=0}
pub unsafe fn vmware_select_hypercall()->u8 {let mut a=0;let mut b=0;let mut c=0;let mut d=0;cpuid(CPUID_VMWARE_FEATURES_LEAF,&mut a,&mut b,&mut c,&mut d);(c&(CPUID_VMWARE_FEATURES_ECX_VMMCALL|CPUID_VMWARE_FEATURES_ECX_VMCALL)) as u8}
pub unsafe fn vmware_platform()->u32 {if boot_cpu_has(X86_FEATURE_HYPERVISOR){let mut a=0;let mut v=[0u32;3];cpuid(CPUID_VMWARE_INFO_LEAF,&mut a,&mut v[0],&mut v[1],&mut v[2]);if v==[0x65766d56,0x65726d56,0x65726177]{if a>=CPUID_VMWARE_FEATURES_LEAF{VMWARE_HYPERCALL_MODE=vmware_select_hypercall();}return CPUID_VMWARE_INFO_LEAF;}}else if dmi_available()&&dmi_name_in_serial("VMware")&&__vmware_platform(){return 1;}0}
pub unsafe fn vmware_platform_setup(){let mut b=0;let mut c=0;let eax=vmware_hypercall3(VMWARE_CMD_GETHZ,u32::MAX as usize,&mut b,&mut c);if b!=u32::MAX{let mut t=(eax as u64)|((b as u64)<<32);let mut lpj=t;t/=1000;if preset_lpj==0{lpj/=HZ as u64;preset_lpj=lpj;}VMWARE_TSC_KHZ=t as usize;x86_platform_calibrate();lapic_timer_period=c/HZ;}vmware_paravirt_ops_setup();vmware_set_capabilities();}
pub unsafe fn vmware_set_capabilities(){setup_force_cpu_cap(X86_FEATURE_CONSTANT_TSC);setup_force_cpu_cap(X86_FEATURE_TSC_RELIABLE);if VMWARE_TSC_KHZ!=0{setup_force_cpu_cap(X86_FEATURE_TSC_KNOWN_FREQ)}if VMWARE_HYPERCALL_MODE==CPUID_VMWARE_FEATURES_ECX_VMCALL{setup_force_cpu_cap(X86_FEATURE_VMCALL)}else if VMWARE_HYPERCALL_MODE==CPUID_VMWARE_FEATURES_ECX_VMMCALL{setup_force_cpu_cap(X86_FEATURE_VMW_VMMCALL)}}

#[cfg(feature = "CONFIG_PARAVIRT")] unsafe fn vmware_paravirt_ops_setup(){paravirt::vmware_cyc2ns_setup();} #[cfg(not(feature = "CONFIG_PARAVIRT"))] unsafe fn vmware_paravirt_ops_setup(){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
