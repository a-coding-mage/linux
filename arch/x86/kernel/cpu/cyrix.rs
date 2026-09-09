// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies supplied by the surrounding translation unit.

unsafe fn __do_cyrix_devid(dir0: *mut u8, dir1: *mut u8) {
    let mut ccr2: u8;
    let mut ccr3: u8;

    ccr3 = getCx86(CX86_CCR3);
    setCx86(CX86_CCR3, ccr3 ^ 0x80);
    getCx86(0xc0);

    if getCx86(CX86_CCR3) == ccr3 {
        ccr2 = getCx86(CX86_CCR2);
        setCx86(CX86_CCR2, ccr2 ^ 0x04);
        getCx86(0xc0);
        if getCx86(CX86_CCR2) == ccr2 {
            *dir0 = 0xfd;
        } else {
            setCx86(CX86_CCR2, ccr2);
            *dir0 = 0xfe;
        }
    } else {
        setCx86(CX86_CCR3, ccr3);
        *dir0 = getCx86(CX86_DIR0);
        *dir1 = getCx86(CX86_DIR1);
    }
}

unsafe fn do_cyrix_devid(dir0: *mut u8, dir1: *mut u8) {
    let mut flags: c_ulong = 0;
    local_irq_save(&mut flags);
    __do_cyrix_devid(dir0, dir1);
    local_irq_restore(flags);
}

static mut Cx86_dir0_msb: u8 = 0;
static Cx86_model: [[u8; 9]; 8] = [*b"Cx486\0\0\0", *b"Cx486\0\0\0", *b"5x86 \0\0\0", *b"6x86\0\0\0\0", *b"MediaGX \0", *b"6x86MX \0", *b"M II \0\0\0\0", *b"Unknown\0"];
static Cx486_name: [[u8; 5]; 8] = [*b"SLC\0\0", *b"DLC\0\0", *b"SLC2\0", *b"DLC2\0", *b"SRx\0\0", *b"DRx\0\0", *b"SRx2\0", *b"DRx2\0"];
static Cx486S_name: [[u8; 4]; 4] = [*b"S\0\0\0", *b"S2\0\0", *b"Se\0\0", *b"S2e\0"];
static Cx486D_name: [[u8; 4]; 6] = [*b"DX\0\0", *b"DX2\0", *b"?\0\0\0", *b"?\0\0\0", *b"?\0\0\0", *b"DX4\0"];
static mut Cx86_cb: [u8; 23] = *b"?.5x Core/Bus Clock\0\0\0\0";
static cyrix_model_mult1: &[u8] = b"12??43";
static cyrix_model_mult2: &[u8] = b"12233445";

unsafe fn check_cx686_slop(c: *mut cpuinfo_x86) {
    let mut flags: c_ulong = 0;
    if Cx86_dir0_msb == 3 {
        local_irq_save(&mut flags);
        let ccr3 = getCx86(CX86_CCR3);
        setCx86(CX86_CCR3, (ccr3 & 0x0f) | 0x10);
        let ccr5 = getCx86(CX86_CCR5);
        if ccr5 & 2 != 0 { setCx86(CX86_CCR5, ccr5 & 0xfd); }
        setCx86(CX86_CCR3, ccr3);
        local_irq_restore(flags);
        if ccr5 & 2 != 0 {
            pr_info!("Recalibrating delay loop with SLOP bit reset\n");
            calibrate_delay();
            (*c).loops_per_jiffy = loops_per_jiffy;
        }
    }
}

unsafe fn set_cx86_reorder() {
    pr_info!("Enable Memory access reorder on Cyrix/NSC processor.\n");
    let mut ccr3 = getCx86(CX86_CCR3);
    setCx86(CX86_CCR3, (ccr3 & 0x0f) | 0x10);
    setCx86(CX86_PCR0, getCx86(CX86_PCR0) & !0x80);
    ccr3 |= 0xe0;
    setCx86(CX86_CCR3, ccr3);
}

unsafe fn set_cx86_memwb() {
    pr_info!("Enable Memory-Write-back mode on Cyrix/NSC processor.\n");
    setCx86(CX86_CCR2, getCx86(CX86_CCR2) & !0x04);
    write_cr0(read_cr0() | X86_CR0_NW);
    setCx86(CX86_CCR2, getCx86(CX86_CCR2) | 0x14);
}

unsafe fn geode_configure() {
    let mut flags: c_ulong = 0;
    local_irq_save(&mut flags);
    setCx86(CX86_CCR2, getCx86(CX86_CCR2) | 0x08);
    let ccr3 = getCx86(CX86_CCR3);
    setCx86(CX86_CCR3, (ccr3 & 0x0f) | 0x10);
    setCx86(CX86_CCR4, getCx86(CX86_CCR4) | 0x38);
    setCx86(CX86_CCR3, ccr3);
    set_cx86_memwb();
    set_cx86_reorder();
    local_irq_restore(flags);
}

unsafe fn early_init_cyrix(c: *mut cpuinfo_x86) {
    let mut dir0 = 0u8; let mut dir1 = 0u8;
    __do_cyrix_devid(&mut dir0, &mut dir1);
    match dir0 >> 4 { 3 | 5 => set_cpu_cap(c, X86_FEATURE_CYRIX_ARR), _ => {} }
}

unsafe fn init_cyrix(c: *mut cpuinfo_x86) {
    let mut dir0 = 0u8; let mut dir1 = 0u8;
    let mut p: *const u8 = core::ptr::null();
    if test_cpu_cap(c, 1 * 32 + 24) { clear_cpu_cap(c, 1 * 32 + 24); set_cpu_cap(c, X86_FEATURE_CXMMX); }
    do_cyrix_devid(&mut dir0, &mut dir1);
    check_cx686_slop(c);
    let mut dir0_msn = dir0 >> 4; let dir0_lsn = dir0 & 0xf;
    Cx86_dir0_msb = dir0_msn;
    (*c).x86_model = (dir1 >> 4) + 1; (*c).x86_stepping = dir1 & 0xf;
    match dir0_msn {
        0 => p = Cx486_name[(dir0_lsn & 7) as usize].as_ptr(),
        1 => p = if dir0_lsn & 8 != 0 { Cx486D_name[(dir0_lsn & 5) as usize].as_ptr() } else { Cx486S_name[(dir0_lsn & 3) as usize].as_ptr() },
        2 => { Cx86_cb[2] = cyrix_model_mult1[(dir0_lsn & 5) as usize]; p = Cx86_cb.as_ptr().add(2); }
        3 => { Cx86_cb[1] = b' '; Cx86_cb[2] = cyrix_model_mult1[(dir0_lsn & 5) as usize]; if dir1 > 0x21 { Cx86_cb[0] = b'L'; p = Cx86_cb.as_ptr(); (*c).x86_model += 1; } else { p = Cx86_cb.as_ptr().add(1); } set_cpu_cap(c, X86_FEATURE_CYRIX_ARR); set_cpu_bug(c, X86_BUG_COMA); }
        4 | 11 => { (*c).x86_cache_size = 16; if (*c).cpuid_level == 2 { setCx86(CX86_CCR7, getCx86(CX86_CCR7) | 1); if (0x30 <= dir1 && dir1 <= 0x6f) || (0x80 <= dir1 && dir1 <= 0x8f) { geode_configure(); return; } } else { Cx86_cb[2] = if dir0_lsn & 1 != 0 { b'3' } else { b'4' }; p = Cx86_cb.as_ptr().add(2); (*c).x86_model = if dir1 & 0x20 != 0 { 1 } else { 2 }; } }
        5 => { if dir1 > 7 { dir0_msn += 1; setCx86(CX86_CCR7, getCx86(CX86_CCR7) | 1); } else { set_cpu_bug(c, X86_BUG_COMA); } let tmp = if dir0_lsn & 7 == 0 || dir0_lsn & 1 != 0 { 2 } else { 0 }; Cx86_cb[tmp] = cyrix_model_mult2[(dir0_lsn & 7) as usize]; p = Cx86_cb.as_ptr().add(tmp); if (dir1 & 0x0f) > 4 || (dir1 & 0xf0) == 0x20 { (*c).x86_model += 1; } set_cpu_cap(c, X86_FEATURE_CYRIX_ARR); }
        0xf => match dir0_lsn { 0xd => { dir0_msn = 0; p = Cx486_name[boot_cpu_has(X86_FEATURE_FPU) as usize].as_ptr(); }, 0xe => { dir0_msn = 0; p = Cx486S_name[0].as_ptr(); }, _ => {} },
        _ => dir0_msn = 7,
    }
    strcpy((*c).x86_model_id.as_mut_ptr(), Cx86_model[(dir0_msn & 7) as usize].as_ptr());
    if !p.is_null() { strcat((*c).x86_model_id.as_mut_ptr(), p); }
}

unsafe fn init_nsc(c: *mut cpuinfo_x86) { if (*c).x86 == 5 && (*c).x86_model == 5 { cpu_detect_cache_sizes(c); } else { init_cyrix(c); } }

unsafe fn test_cyrix_52div() -> c_int {
    let mut test: u32;
    core::arch::asm!("sahf", "div {div:b}", "lahf", inout("eax") 5u32 => test, div = in(reg) 2u32, options(nostack));
    ((test >> 8) as u8 == 0x02) as c_int
}

unsafe fn cyrix_identify(c: *mut cpuinfo_x86) {
    if (*c).x86 == 4 && test_cyrix_52div() != 0 {
        strcpy((*c).x86_vendor_id.as_mut_ptr(), b"CyrixInstead\0".as_ptr()); (*c).x86_vendor = X86_VENDOR_CYRIX;
        let mut dir0 = 0u8; let mut dir1 = 0u8; do_cyrix_devid(&mut dir0, &mut dir1); dir0 >>= 4;
        if dir0 == 5 || dir0 == 3 { let mut flags: c_ulong = 0; pr_info!("Enabling CPUID on Cyrix processor.\n"); local_irq_save(&mut flags); let ccr3 = getCx86(CX86_CCR3); setCx86(CX86_CCR3, (ccr3 & 0x0f) | 0x10); setCx86(CX86_CCR4, getCx86(CX86_CCR4) | 0x80); setCx86(CX86_CCR3, ccr3); local_irq_restore(flags); }
    }
}

static cyrix_cpu_dev: cpu_dev = cpu_dev { c_vendor: b"Cyrix\0".as_ptr(), c_ident: [b"CyrixInstead\0".as_ptr()], c_early_init: Some(early_init_cyrix), c_init: Some(init_cyrix), c_identify: Some(cyrix_identify), c_x86_vendor: X86_VENDOR_CYRIX };
cpu_dev_register(cyrix_cpu_dev);
static nsc_cpu_dev: cpu_dev = cpu_dev { c_vendor: b"NSC\0".as_ptr(), c_ident: [b"Geode by NSC\0".as_ptr()], c_early_init: None, c_init: Some(init_nsc), c_identify: None, c_x86_vendor: X86_VENDOR_NSC };
cpu_dev_register(nsc_cpu_dev);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
