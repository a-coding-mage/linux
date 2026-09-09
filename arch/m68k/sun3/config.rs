/*
 *  linux/arch/m68k/sun3/config.c
 *
 *  Copyright (C) 1996,1997 Pekka Pietik{inen
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// C header dependencies are supplied by the surrounding kernel translation.

use core::ffi::c_void;

extern "C" {
    static mut m68k_machtype: i32;
    static mut m68k_cputype: i32;
    static mut m68k_fputype: i32;
    static mut m68k_mmutype: i32;
    static mut sun3_intreg: *mut u8;
    static mut max_pfn: usize;
    static mut high_memory: *mut c_void;
    static mut mach_sched_init: Option<unsafe extern "C" fn()>;
    static mut mach_init_IRQ: Option<unsafe extern "C" fn()>;
    static mut mach_reset: Option<unsafe extern "C" fn()>;
    static mut mach_get_model: Option<unsafe extern "C" fn()>;
    static mut mach_hwclk: Option<unsafe extern "C" fn()>;
    static mut mach_halt: Option<unsafe extern "C" fn()>;
    static mut mach_get_hardware_list: Option<unsafe extern "C" fn(*mut SeqFile)>;
    static mut m68k_num_memory: i32;
    static mut m68k_memory: [Memory; 1];
    static mut romvec: *mut Romvec;
    static mut idprom: *mut Idprom;
    static mut intersil_clock: *mut IntersilClock;

    fn seq_printf(m: *mut SeqFile, fmt: *const i8, ...);
    fn sun3_disable_interrupts();
    fn sun3_enable_interrupts();
    fn prom_init(p: *mut c_void);
    fn prom_reboot(s: *const i8) -> !;
    fn prom_halt() -> !;
    fn set_fc(fc: i32);
    fn idprom_init();
    fn m68k_setup_node(node: i32);
    fn intersil_clear();
    fn sun3_enable_irq(irq: i32);
    fn sun3_init_IRQ();
    fn sun3_get_model();
    fn sun3_hwclk();
    fn platform_device_register_simple(name: *const i8, id: i32, r: *const Resource, n: usize) -> *mut c_void;
}

#[repr(C)]
struct SeqFile;
#[repr(C)]
struct Romvec { pv_monid: *const i8, pv_sun3mem: *mut u32 }
#[repr(C)]
struct Idprom { id_machtype: u32 }
#[repr(C)]
struct IntersilClock { cmd_reg: u8, int_reg: u8 }
#[repr(C)]
struct Memory { size: u32 }
#[repr(C)]
struct Resource { flags: usize, start: usize, end: usize }

static mut sun3_reserved_pmeg: [i8; 256] = [0; 256];
static mut clock_va: *mut i8 = core::ptr::null_mut();
extern "C" { static mut availmem: usize; }
static mut num_pages: usize = 0;

unsafe extern "C" fn sun3_get_hardware_list(m: *mut SeqFile) {
    seq_printf(m, b"PROM Revision:\t%s\n\0".as_ptr() as *const i8, (*romvec).pv_monid);
}

unsafe extern "C" fn sun3_init() {
    let mut enable_register: u8;
    let mut i: i32;

    m68k_machtype = MACH_SUN3;
    m68k_cputype = CPU_68020;
    m68k_fputype = FPU_68881;
    m68k_mmutype = MMU_SUN3;
    clock_va = 0x0fe06000 as *mut i8;
    sun3_intreg = 0x0fe0a000 as *mut u8;
    sun3_disable_interrupts();
    prom_init(LINUX_OPPROM_BEGVM as *mut c_void);

    GET_CONTROL_BYTE(AC_SENABLE, enable_register);
    enable_register |= 0x50;
    SET_CONTROL_BYTE(AC_SENABLE, enable_register);
    GET_CONTROL_BYTE(AC_SENABLE, enable_register);

    /* This code looks suspicious, because it doesn't subtract
       memory belonging to the kernel from the available space */
    sun3_reserved_pmeg.fill(0);

    /* Reserve important PMEGS */
    /* FIXME: These should be probed instead of hardcoded */
    while i < 8 {
        sun3_reserved_pmeg[i as usize] = 1;
        i += 1;
    }
    sun3_reserved_pmeg[247] = 1;
    sun3_reserved_pmeg[248] = 1;
    sun3_reserved_pmeg[251] = 1;
    sun3_reserved_pmeg[254] = 1;
    sun3_reserved_pmeg[249] = 1;
    sun3_reserved_pmeg[252] = 1;
    sun3_reserved_pmeg[253] = 1;
    set_fc(USER_DATA);
}

unsafe extern "C" fn sun3_reboot() -> ! { prom_reboot(b"vmlinux\0".as_ptr() as *const i8) }
unsafe extern "C" fn sun3_halt() -> ! { prom_halt() }

unsafe extern "C" fn sun3_bootmem_alloc(mut memory_start: usize, mut memory_end: usize) {
    memory_start = (memory_start + (PAGE_SIZE - 1)) & PAGE_MASK;
    memory_end &= PAGE_MASK;
    max_pfn = __pa(memory_end) >> PAGE_SHIFT;
    num_pages = max_pfn;
    high_memory = memory_end as *mut c_void;
    availmem = memory_start;
    m68k_setup_node(0);
}

unsafe extern "C" fn config_sun3() {
    let memory_start: usize;
    let memory_end: usize;
    pr_info!("ARCH: SUN3\n");
    idprom_init();
    mach_sched_init = Some(sun3_sched_init);
    mach_init_IRQ = Some(sun3_init_IRQ);
    mach_reset = Some(sun3_reboot);
    mach_get_model = Some(sun3_get_model);
    mach_hwclk = Some(sun3_hwclk);
    mach_halt = Some(sun3_halt);
    mach_get_hardware_list = Some(sun3_get_hardware_list);
    memory_start = ((((&_end as *const _ as usize) + 0x2000) & !0x1fff));
    // PROM seems to want the last couple of physical pages. --m
    memory_end = (*(*romvec).pv_sun3mem as usize) + PAGE_OFFSET - 2 * PAGE_SIZE;
    m68k_num_memory = 1;
    m68k_memory[0].size = *(*romvec).pv_sun3mem;
    sun3_bootmem_alloc(memory_start, memory_end);
}

unsafe extern "C" fn sun3_sched_init() {
    sun3_disable_interrupts();
    (*intersil_clock).cmd_reg = INTERSIL_RUN | INTERSIL_INT_DISABLE | INTERSIL_24H_MODE;
    (*intersil_clock).int_reg = INTERSIL_HZ_100_MASK;
    intersil_clear();
    sun3_enable_irq(5);
    (*intersil_clock).cmd_reg = INTERSIL_RUN | INTERSIL_INT_ENABLE | INTERSIL_24H_MODE;
    sun3_enable_interrupts();
    intersil_clear();
}

// The following declarations preserve the CONFIG_SUN3_SCSI conditional code.
#[cfg(CONFIG_SUN3_SCSI)]
static sun3_scsi_vme_rsrc: [Resource; 4] = [
    Resource { flags: IORESOURCE_IRQ, start: SUN3_VEC_VMESCSI0, end: SUN3_VEC_VMESCSI0 },
    Resource { flags: IORESOURCE_MEM, start: 0xff200000, end: 0xff200021 },
    Resource { flags: IORESOURCE_IRQ, start: SUN3_VEC_VMESCSI1, end: SUN3_VEC_VMESCSI1 },
    Resource { flags: IORESOURCE_MEM, start: 0xff204000, end: 0xff204021 },
];

#[cfg(CONFIG_SUN3_SCSI)]
static sun3_scsi_rsrc: [Resource; 2] = [
    Resource { flags: IORESOURCE_IRQ, start: 2, end: 2 },
    Resource { flags: IORESOURCE_MEM, start: 0x00140000, end: 0x0014001f },
];

#[cfg(CONFIG_SUN3_SCSI)]
unsafe extern "C" fn sun3_platform_init() -> i32 {
    match (*idprom).id_machtype {
        x if x == (SM_SUN3 | SM_3_160) || x == (SM_SUN3 | SM_3_260) => {
            platform_device_register_simple(b"sun3_scsi_vme\0".as_ptr() as *const i8, -1, sun3_scsi_vme_rsrc.as_ptr(), 4);
        }
        x if x == (SM_SUN3 | SM_3_50) || x == (SM_SUN3 | SM_3_60) => {
            platform_device_register_simple(b"sun3_scsi\0".as_ptr() as *const i8, -1, sun3_scsi_rsrc.as_ptr(), 2);
        }
        _ => {}
    }
    0
}

// arch_initcall(sun3_platform_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
