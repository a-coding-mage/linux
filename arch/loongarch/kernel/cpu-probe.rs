// SPDX-License-Identifier: GPL-2.0
/* Processor capabilities determination functions. */

// Kernel and architecture dependencies are supplied by other translation units.

pub static mut elf_hwcap: u32 = 0;

unsafe fn cpu_set_fpu_fcsr_mask(c: *mut cpuinfo_loongarch) {
    let fcsr: u64 = (*c).fpu_csr0;
    let mask: u64 = FPU_CSR_ALL_X | FPU_CSR_ALL_E | FPU_CSR_ALL_S | FPU_CSR_RM;
    let sr = read_csr_euen();
    enable_fpu();
    let fcsr0 = fcsr & mask;
    write_fcsr(LOONGARCH_FCSR0, fcsr0);
    let fcsr0 = read_fcsr(LOONGARCH_FCSR0);
    let fcsr1 = fcsr | !mask;
    write_fcsr(LOONGARCH_FCSR0, fcsr1);
    let fcsr1 = read_fcsr(LOONGARCH_FCSR0);
    write_fcsr(LOONGARCH_FCSR0, fcsr);
    write_csr_euen(sr);
    (*c).fpu_mask = !(fcsr0 ^ fcsr1) & !mask;
}

/* simd = -1/0/128/256 */
static mut simd: u32 = u32::MAX;

unsafe fn cpu_setup_simd(mut str_: *mut i8) -> i32 {
    get_option(&mut str_, &mut simd);
    pr_info!("Set SIMD width = {}\n", simd);
    0
}

unsafe fn cpu_final_simd() -> i32 {
    let c = &mut cpu_data[0];
    if simd < 128 { c.options &= !LOONGARCH_CPU_LSX; elf_hwcap &= !HWCAP_LOONGARCH_LSX; }
    if simd < 256 { c.options &= !LOONGARCH_CPU_LASX; elf_hwcap &= !HWCAP_LOONGARCH_LASX; }
    simd = 0;
    if c.options & LOONGARCH_CPU_LSX != 0 { simd = 128; }
    if c.options & LOONGARCH_CPU_LASX != 0 { simd = 256; }
    pr_info!("Final SIMD width = {}\n", simd);
    0
}

unsafe fn set_elf_platform(cpu: i32, plat: *const i8) { if cpu == 0 { __elf_platform = plat; } }

pub static mut vm_map_base: usize = 0;

unsafe fn cpu_probe_addrbits(c: *mut cpuinfo_loongarch) {
    // CONFIG_32BIT selects the 32-bit definitions; otherwise use CPU configuration registers.
    #[cfg(CONFIG_32BIT)] {
        (*c).pabits = cpu_pabits; (*c).vabits = cpu_vabits; vm_map_base = KVRANGE;
    }
    #[cfg(not(CONFIG_32BIT))] {
        (*c).pabits = (read_cpucfg(LOONGARCH_CPUCFG1) & CPUCFG1_PABITS) >> 4;
        (*c).vabits = (read_cpucfg(LOONGARCH_CPUCFG1) & CPUCFG1_VABITS) >> 12;
        vm_map_base = 0usize.wrapping_sub(1usize << (*c).vabits);
    }
}

unsafe fn set_isa(c: *mut cpuinfo_loongarch, isa: u32) {
    match isa {
        LOONGARCH_CPU_ISA_LA64 => { (*c).isa_level |= LOONGARCH_CPU_ISA_LA64; (*c).isa_level |= LOONGARCH_CPU_ISA_LA32S; (*c).isa_level |= LOONGARCH_CPU_ISA_LA32R; }
        LOONGARCH_CPU_ISA_LA32S => { (*c).isa_level |= LOONGARCH_CPU_ISA_LA32S; (*c).isa_level |= LOONGARCH_CPU_ISA_LA32R; }
        LOONGARCH_CPU_ISA_LA32R => { (*c).isa_level |= LOONGARCH_CPU_ISA_LA32R; }
        _ => {}
    }
}

unsafe fn cpu_probe_common(c: *mut cpuinfo_loongarch) {
    let mut config: u32;
    (*c).options = LOONGARCH_CPU_CPUCFG | LOONGARCH_CPU_CSR | LOONGARCH_CPU_VINT;
    elf_hwcap = HWCAP_LOONGARCH_CPUCFG;
    config = read_cpucfg(LOONGARCH_CPUCFG1);
    match config & CPUCFG1_ISA { 0 => set_isa(c, LOONGARCH_CPU_ISA_LA32R), 1 => set_isa(c, LOONGARCH_CPU_ISA_LA32S), 2 => set_isa(c, LOONGARCH_CPU_ISA_LA64), _ => pr_warn!("Warning: unknown ISA level\n") }
    if config & CPUCFG1_PAGING != 0 { (*c).options |= LOONGARCH_CPU_TLB; }
    if config & CPUCFG1_IOCSR != 0 { (*c).options |= LOONGARCH_CPU_IOCSR; }
    if config & CPUCFG1_MSGINT != 0 { (*c).options |= LOONGARCH_CPU_MSGINT; }
    if config & CPUCFG1_UAL != 0 { (*c).options |= LOONGARCH_CPU_UAL; elf_hwcap |= HWCAP_LOONGARCH_UAL; }
    if config & CPUCFG1_CRC32 != 0 { (*c).options |= LOONGARCH_CPU_CRC32; elf_hwcap |= HWCAP_LOONGARCH_CRC32; }
    config = read_cpucfg(LOONGARCH_CPUCFG2);
    if config & CPUCFG2_LAM != 0 { (*c).options |= LOONGARCH_CPU_LAM; elf_hwcap |= HWCAP_LOONGARCH_LAM; }
    if config & CPUCFG2_LAM_BH != 0 { (*c).options |= LOONGARCH_CPU_LAM_BH; elf_hwcap |= HWCAP_LOONGARCH_LAM_BH; }
    if config & CPUCFG2_SCQ != 0 { (*c).options |= LOONGARCH_CPU_SCQ; elf_hwcap |= HWCAP_LOONGARCH_SCQ; }
    if config & CPUCFG2_FP != 0 { (*c).options |= LOONGARCH_CPU_FPU; elf_hwcap |= HWCAP_LOONGARCH_FPU; }
    // CONFIG_CPU_HAS_LSX/LASX/LBT conditionally include their corresponding feature probes.
    if config & CPUCFG2_LSX != 0 && simd >= 128 { (*c).options |= LOONGARCH_CPU_LSX; elf_hwcap |= HWCAP_LOONGARCH_LSX; }
    if config & CPUCFG2_LASX != 0 && simd >= 256 { (*c).options |= LOONGARCH_CPU_LASX; elf_hwcap |= HWCAP_LOONGARCH_LASX; }
    if config & CPUCFG2_COMPLEX != 0 { (*c).options |= LOONGARCH_CPU_COMPLEX; elf_hwcap |= HWCAP_LOONGARCH_COMPLEX; }
    if config & CPUCFG2_CRYPTO != 0 { (*c).options |= LOONGARCH_CPU_CRYPTO; elf_hwcap |= HWCAP_LOONGARCH_CRYPTO; }
    if config & CPUCFG2_PTW != 0 { (*c).options |= LOONGARCH_CPU_PTW; elf_hwcap |= HWCAP_LOONGARCH_PTW; }
    if config & CPUCFG2_LSPW != 0 { (*c).options |= LOONGARCH_CPU_LSPW; elf_hwcap |= HWCAP_LOONGARCH_LSPW; }
    if config & CPUCFG2_LVZP != 0 { (*c).options |= LOONGARCH_CPU_LVZ; elf_hwcap |= HWCAP_LOONGARCH_LVZ; }
    if config & CPUCFG2_X86BT != 0 { (*c).options |= LOONGARCH_CPU_LBT_X86; elf_hwcap |= HWCAP_LOONGARCH_LBT_X86; }
    if config & CPUCFG2_ARMBT != 0 { (*c).options |= LOONGARCH_CPU_LBT_ARM; elf_hwcap |= HWCAP_LOONGARCH_LBT_ARM; }
    if config & CPUCFG2_MIPSBT != 0 { (*c).options |= LOONGARCH_CPU_LBT_MIPS; elf_hwcap |= HWCAP_LOONGARCH_LBT_MIPS; }
    config = read_cpucfg(LOONGARCH_CPUCFG6);
    if config & CPUCFG6_PMP != 0 { (*c).options |= LOONGARCH_CPU_PMP; }
    config = csr_read32(LOONGARCH_CSR_ASID); config = (config & CSR_ASID_BIT) >> CSR_ASID_BIT_SHIFT;
    set_cpu_asid_mask(c, GENMASK(config - 1, 0));
    config = read_csr_prcfg1(); (*c).timerbits = (config & CSR_CONF1_TMRBITS) >> CSR_CONF1_TMRBITS_SHIFT;
    (*c).ksave_mask = GENMASK((config & CSR_CONF1_KSNUM) - 1, 0) & !(EXC_KSAVE_MASK | PERCPU_KSAVE_MASK | KVM_KSAVE_MASK);
    config = read_csr_prcfg3();
    match config & CSR_CONF3_TLBTYPE {
        0 => { (*c).tlbsizemtlb=0; (*c).tlbsizestlbsets=0; (*c).tlbsizestlbways=0; (*c).tlbsize=0; }
        1 => { (*c).tlbsizemtlb=((config&CSR_CONF3_MTLBSIZE)>>CSR_CONF3_MTLBSIZE_SHIFT)+1; (*c).tlbsizestlbsets=0; (*c).tlbsizestlbways=0; (*c).tlbsize=(*c).tlbsizemtlb; }
        2 => { (*c).tlbsizemtlb=((config&CSR_CONF3_MTLBSIZE)>>CSR_CONF3_MTLBSIZE_SHIFT)+1; (*c).tlbsizestlbsets=1<<((config&CSR_CONF3_STLBIDX)>>CSR_CONF3_STLBIDX_SHIFT); (*c).tlbsizestlbways=((config&CSR_CONF3_STLBWAYS)>>CSR_CONF3_STLBWAYS_SHIFT)+1; (*c).tlbsize=(*c).tlbsizemtlb+(*c).tlbsizestlbsets*(*c).tlbsizestlbways; }
        _ => pr_warn!("Warning: unknown TLB type\n")
    }
    if get_num_brps() + get_num_wrps() != 0 { (*c).options |= LOONGARCH_CPU_WATCH; }
}

pub const MAX_NAME_LEN: usize = 32;
pub const VENDOR_OFFSET: usize = 0;
pub const CPUNAME_OFFSET: usize = 9;
static mut cpu_full_name: [i8; MAX_NAME_LEN] = *b"        -        \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

unsafe fn cpu_probe_loongson(c: *mut cpuinfo_loongarch, cpu: u32) {
    let core_name = id_to_core_name((*c).processor_id);
    match BIT(fls((*c).isa_level)-1) { LOONGARCH_CPU_ISA_LA32R|LOONGARCH_CPU_ISA_LA32S => { (*c).cputype=CPU_LOONGSON32; __cpu_family[cpu as usize]="Loongson-32bit"; }, LOONGARCH_CPU_ISA_LA64 => { (*c).cputype=CPU_LOONGSON64; __cpu_family[cpu as usize]="Loongson-64bit"; }, _ => {} }
    pr_info!("{} Processor probed ({} Core)\n", __cpu_family[cpu as usize], core_name);
    if !cpu_has_iocsr { __cpu_full_name[cpu as usize]="Unknown"; return; }
    *(cpu_full_name.as_mut_ptr().add(VENDOR_OFFSET) as *mut u64) = iocsr_read64(LOONGARCH_IOCSR_VENDOR);
    *(cpu_full_name.as_mut_ptr().add(CPUNAME_OFFSET) as *mut u64) = iocsr_read64(LOONGARCH_IOCSR_CPUNAME);
    if __cpu_full_name[cpu as usize].is_null() { __cpu_full_name[cpu as usize] = if cpu_full_name[0] == 0 { "Unknown" } else { cpu_full_name.as_ptr() as *const i8 }; }
    let config=iocsr_read32(LOONGARCH_IOCSR_FEATURES);
    if config&IOCSRF_CSRIPI!=0 { (*c).options|=LOONGARCH_CPU_CSRIPI; } if config&IOCSRF_EXTIOI!=0 { (*c).options|=LOONGARCH_CPU_EXTIOI; } if config&IOCSRF_FREQSCALE!=0 { (*c).options|=LOONGARCH_CPU_SCALEFREQ; } if config&IOCSRF_FLATMODE!=0 { (*c).options|=LOONGARCH_CPU_FLATMODE; } if config&IOCSRF_EIODECODE!=0 { (*c).options|=LOONGARCH_CPU_EIODECODE; } if config&IOCSRF_AVEC!=0 { (*c).options|=LOONGARCH_CPU_AVECINT; } if config&IOCSRF_REDIRECT!=0 { (*c).options|=LOONGARCH_CPU_REDIRECTINT; } if config&IOCSRF_VM!=0 { (*c).options|=LOONGARCH_CPU_HYPERVISOR; }
}

#[cfg(CONFIG_64BIT)] pub static mut __ua_limit: u64 = 0;
pub static mut __cpu_family: [*const i8; NR_CPUS] = [core::ptr::null(); NR_CPUS];
pub static mut __cpu_full_name: [*const i8; NR_CPUS] = [core::ptr::null(); NR_CPUS];
pub static mut __elf_platform: *const i8 = core::ptr::null();

unsafe fn cpu_report() { pr_info!("CPU{} revision is: {:08x} ({})\n", smp_processor_id(), current_cpu_data.processor_id, cpu_family_string()); if current_cpu_data.options&LOONGARCH_CPU_FPU!=0 { pr_info!("FPU{} revision is: {:08x}\n", smp_processor_id(), current_cpu_data.fpu_vers); } }

pub unsafe fn cpu_probe() {
    let cpu=smp_processor_id(); let c=&mut current_cpu_data;
    set_elf_platform(cpu, "loongarch"); c.cputype=CPU_UNKNOWN; c.processor_id=read_cpucfg(LOONGARCH_CPUCFG0); c.fpu_vers=(read_cpucfg(LOONGARCH_CPUCFG2)&CPUCFG2_FPVERS)>>3; c.fpu_csr0=FPU_CSR_RN; c.fpu_mask=FPU_CSR_RSVD;
    cpu_probe_common(c); per_cpu_trap_init(cpu); if c.processor_id&PRID_COMP_MASK==PRID_COMP_LOONGSON { cpu_probe_loongson(c,cpu); }
    BUG_ON(__cpu_family[cpu as usize].is_null()); BUG_ON(c.cputype==CPU_UNKNOWN); cpu_probe_addrbits(c);
    #[cfg(CONFIG_64BIT)] if cpu==0 { __ua_limit = !((1u64<<cpu_vabits)-1); }
    cpu_report();
}

pub unsafe fn cpu_show_spectre_v1(_dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize { sysfs_emit(buf, "Mitigation: __user pointer sanitization\n") }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
