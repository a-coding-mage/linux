/*
 * Setup pointers to hardware-dependent routines.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996, 97, 98, 2000, 03, 04, 06 Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 2006,2007 Thomas Bogendoerfer (tsbogend@alpha.franken.de)
 */

// C headers and configuration-dependent declarations are supplied by other files.

pub static mut sni_brd_type: core::ffi::c_uint = 0;

extern "C" {
    fn sni_machine_restart(command: *mut core::ffi::c_char);
    fn sni_machine_power_off();
    fn set_io_port_base(base: usize);
    fn read_c0_prid() -> core::ffi::c_uint;
    fn sni_a20r_init();
    fn sni_pcit_init();
    fn sni_rm200_init();
    fn sni_pcimt_init();
    fn vgacon_register_screen(si: *mut screen_info);
    fn prom_getenv(name: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    fn strncmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char, n: usize) -> i32;
    fn strscpy(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char);
    fn add_preferred_console(name: *const core::ffi::c_char, index: i32, options: *const core::ffi::c_char);
    fn pr_debug(fmt: *const core::ffi::c_char, ...);
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn pci_read_config_word(dev: *mut pci_dev, where_: u32, val: *mut u16);
    fn vga_wseq(chip: *mut core::ffi::c_void, index: u8, data: u8);
}

#[repr(C)]
struct screen_info {
    orig_x: u16, orig_y: u16, orig_video_cols: u16, orig_video_lines: u16,
    orig_video_isVGA: u16, orig_video_points: u16,
}

#[repr(C)] struct pci_dev { _private: [u8; 0] }

unsafe fn sni_display_setup() {
    // Compiled only when CONFIG_VGA_CONSOLE and CONFIG_FW_ARC are enabled.
    #[cfg(all(feature = "CONFIG_VGA_CONSOLE", feature = "CONFIG_FW_ARC"))]
    {
        let mut si = core::mem::zeroed::<screen_info>();
        let di = ArcGetDisplayStatus(1);
        if !di.is_null() {
            si.orig_x = (*di).CursorXPosition;
            si.orig_y = (*di).CursorYPosition;
            si.orig_video_cols = (*di).CursorMaxXPosition;
            si.orig_video_lines = (*di).CursorMaxYPosition;
            si.orig_video_isVGA = VIDEO_TYPE_VGAC;
            si.orig_video_points = 16;
            vgacon_register_screen(&mut si);
        }
    }
}

unsafe fn sni_console_setup() {
    // Compiled only when CONFIG_FW_ARC is not enabled.
    #[cfg(not(feature = "CONFIG_FW_ARC"))]
    {
        static mut OPTIONS: [core::ffi::c_char; 8] = [0; 8];
        let cdev = prom_getenv(b"console_dev\0".as_ptr() as *const _);
        if strncmp(cdev, b"tty\0".as_ptr() as *const _, 3) == 0 {
            let ctype = prom_getenv(b"console\0".as_ptr() as *const _);
            let (port, baud) = match *ctype {
                b'r' as i8 => (1, prom_getenv(b"rbaud\0".as_ptr() as *const _)),
                _ => (0, prom_getenv(b"lbaud\0".as_ptr() as *const _)),
            };
            if !baud.is_null() { strscpy(OPTIONS.as_mut_ptr(), baud); }
            let name = if strncmp(cdev, b"tty552\0".as_ptr() as *const _, 6) == 0 {
                b"ttyS\0"
            } else { b"ttySC\0" };
            add_preferred_console(name.as_ptr() as *const _, port,
                if !baud.is_null() { OPTIONS.as_ptr() } else { core::ptr::null() });
        }
    }
}

#[cfg(feature = "DEBUG")]
unsafe fn sni_idprom_dump() {
    pr_debug(b"SNI IDProm dump:\n\0".as_ptr() as *const _);
    for i in 0..256 {
        if i % 16 == 0 { pr_debug(b"%04x \0".as_ptr() as *const _, i); }
        printk(b"%02x \0".as_ptr() as *const _, *((SNI_IDPROM_BASE + i) as *const u8));
        if i % 16 == 15 { printk(b"\n\0".as_ptr() as *const _); }
    }
}

pub unsafe fn plat_mem_setup() {
    set_io_port_base(SNI_PORT_BASE);
    #[cfg(feature = "CONFIG_EISA")]
    { EISA_bus = 1; }

    sni_brd_type = *(SNI_IDPROM_BRDTYPE as *const u8) as core::ffi::c_uint;
    let cputype = *(SNI_IDPROM_CPUTYPE as *const u8) as i32;
    system_type = match sni_brd_type {
        SNI_BRD_TOWER_OASIC => match cputype { SNI_CPU_M8030 => b"RM400-330\0", SNI_CPU_M8031 => b"RM400-430\0", SNI_CPU_M8037 => b"RM400-530\0", SNI_CPU_M8034 => b"RM400-730\0", _ => b"RM400-xxx\0" },
        SNI_BRD_MINITOWER => match cputype { SNI_CPU_M8021 | SNI_CPU_M8043 => b"RM400-120\0", SNI_CPU_M8040 => b"RM400-220\0", SNI_CPU_M8053 => b"RM400-225\0", SNI_CPU_M8050 => b"RM400-420\0", _ => b"RM400-xxx\0" },
        SNI_BRD_PCI_TOWER => b"RM400-Cxx\0", SNI_BRD_RM200 => b"RM200-xxx\0", SNI_BRD_PCI_MTOWER => b"RM300-Cxx\0",
        SNI_BRD_PCI_DESKTOP => match read_c0_prid() & PRID_IMP_MASK { PRID_IMP_R4600 | PRID_IMP_R4700 => b"RM200-C20\0", PRID_IMP_R5000 => b"RM200-C40\0", _ => b"RM200-Cxx\0" },
        SNI_BRD_PCI_TOWER_CPLUS => b"RM400-Exx\0", SNI_BRD_PCI_MTOWER_CPLUS => b"RM300-Exx\0", _ => b"\0",
    }.as_ptr() as *const _;
    pr_debug(b"Found SNI brdtype %02x name %s\n\0".as_ptr() as *const _, sni_brd_type, system_type);
    #[cfg(feature = "DEBUG")] sni_idprom_dump();
    match sni_brd_type {
        SNI_BRD_10 | SNI_BRD_10NEW | SNI_BRD_TOWER_OASIC | SNI_BRD_MINITOWER => sni_a20r_init(),
        SNI_BRD_PCI_TOWER | SNI_BRD_PCI_TOWER_CPLUS => sni_pcit_init(),
        SNI_BRD_RM200 => sni_rm200_init(),
        SNI_BRD_PCI_MTOWER | SNI_BRD_PCI_DESKTOP | SNI_BRD_PCI_MTOWER_CPLUS => sni_pcimt_init(), _ => {}
    }
    _machine_restart = Some(sni_machine_restart);
    pm_power_off = Some(sni_machine_power_off);
    sni_display_setup();
    sni_console_setup();
}

#[cfg(feature = "CONFIG_PCI")]
unsafe fn quirk_cirrus_ram_size(dev: *mut pci_dev) {
    let mut cmd = 0u16;
    pci_read_config_word(dev, PCI_COMMAND, &mut cmd);
    if cmd & (PCI_COMMAND_IO | PCI_COMMAND_MEMORY) == (PCI_COMMAND_IO | PCI_COMMAND_MEMORY) {
        vga_wseq(core::ptr::null_mut(), CL_SEQR6, 0x12);
        vga_wseq(core::ptr::null_mut(), CL_SEQRF, 0x18);
    }
}

// DECLARE_PCI_FIXUP_FINAL registrations retained as external integration requirements.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
