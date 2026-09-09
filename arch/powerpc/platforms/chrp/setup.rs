// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 1995  Linus Torvalds
 *  Adapted from 'alpha' version by Gary Thomas
 *  Modified by Cort Dougan (cort@cs.nmt.edu)
 */

// bootup setup stuff..
// C headers and kernel-provided symbols are supplied by the surrounding build.

extern "C" {
    fn rtas_indicator_progress(s: *mut i8, v: u16);
}

static mut _chrp_type: i32 = 0;
static mut chrp_mpic: *mut mpic = core::ptr::null_mut();
static mut heartbeat_timer: timer_list = timer_list {};
static mut event_scan_interval: c_ulong = 0;
extern "C" { static mut loops_per_jiffy: c_ulong; }
static mut briq_SPOR: *mut u32 = core::ptr::null_mut();

static gg2_memtypes: [&[u8]; 4] = [b"FPM\0", b"SDRAM\0", b"EDO\0", b"BEDO\0"];
static gg2_cachesizes: [&[u8]; 4] = [b"256 KB\0", b"512 KB\0", b"1 MB\0", b"Reserved\0"];
static gg2_cachetypes: [&[u8]; 4] = [b"Asynchronous\0", b"Reserved\0", b"Flow-Through Synchronous\0", b"Pipelined Synchronous\0"];
static gg2_cachemodes: [&[u8]; 4] = [b"Disabled\0", b"Write-Through\0", b"Copy-Back\0", b"Transparent Mode\0"];
static chrp_names: [&[u8]; 8] = [b"Unknown\0", b"\0", b"\0", b"\0", b"Motorola\0", b"IBM or Longtrail\0", b"Genesi Pegasos\0", b"Total Impact Briq\0"];

unsafe fn chrp_show_cpuinfo(m: *mut seq_file) {
    let mut i: i32;
    let mut sdramen: i32;
    let mut t: u32;
    let mut root: *mut device_node;
    let mut model: *const i8 = b"\0".as_ptr() as *const i8;
    root = of_find_node_by_path(b"/\0".as_ptr() as *const i8);
    if !root.is_null() { model = of_get_property(root, b"model\0".as_ptr() as *const i8, core::ptr::null_mut()); }
    seq_printf(m, b"machine\t\t: CHRP %s\n\0".as_ptr() as *const i8, model);
    if !model.is_null() && strncmp(model, b"IBM,LongTrail\0".as_ptr() as *const i8, 13) == 0 {
        sdramen = ((in_le32(gg2_pci_config_base.add(GG2_PCI_DRAM_CTRL)) >> 31) & 1) as i32;
        i = 0;
        while i < if sdramen != 0 { 4 } else { 6 } {
            t = in_le32(gg2_pci_config_base.add(GG2_PCI_DRAM_BANK0 + (i as usize) * 4));
            if t & 1 != 0 {
                model = match (t >> 8) & 0x1f { 0x1f => b"4 MB\0", 0x1e => b"8 MB\0", 0x1c => b"16 MB\0", 0x18 => b"32 MB\0", 0x10 => b"64 MB\0", 0x00 => b"128 MB\0", _ => b"Reserved\0" }.as_ptr() as *const i8;
                seq_printf(m, b"memory bank %d\t: %s %s\n\0".as_ptr() as *const i8, i, model, gg2_memtypes[if sdramen != 0 { 1 } else { ((t >> 1) & 3) as usize }].as_ptr());
            }
            i += 1;
        }
        t = in_le32(gg2_pci_config_base.add(GG2_PCI_CC_CTRL));
        seq_printf(m, b"board l2\t: %s %s (%s)\n\0".as_ptr() as *const i8, gg2_cachesizes[((t >> 7) & 3) as usize].as_ptr(), gg2_cachetypes[((t >> 2) & 3) as usize].as_ptr(), gg2_cachemodes[(t & 3) as usize].as_ptr());
    }
    of_node_put(root);
}

unsafe fn sio_write(val: u8, index: u8) { outb(index, 0x15c); outb(val, 0x15d); }
unsafe fn sio_read(index: u8) -> u8 { outb(index, 0x15c); inb(0x15d) }
unsafe fn sio_fixup_irq(name: *const i8, device: u8, level: u8, typ: u8) {
    sio_write(device, 7); let active = sio_read(0x30); let level0 = sio_read(0x70); let type0 = sio_read(0x71);
    if level0 != level || type0 != typ || active == 0 { printk(KERN_WARNING, b"sio: irq remapping\0".as_ptr() as *const i8, name); sio_write(1, 0x30); sio_write(level, 0x70); sio_write(typ, 0x71); }
}
unsafe fn sio_init() { let root = of_find_node_by_path(b"/\0".as_ptr() as *const i8); if root.is_null() { return; } let model = of_get_property(root, b"model\0".as_ptr() as *const i8, core::ptr::null_mut()); if !model.is_null() && strncmp(model, b"IBM,LongTrail\0".as_ptr() as *const i8, 13) == 0 { sio_fixup_irq(b"keyboard\0".as_ptr() as *const i8, 0, 1, 2); sio_fixup_irq(b"mouse\0".as_ptr() as *const i8, 1, 12, 2); } of_node_put(root); }

unsafe fn pegasos_set_l2cr() {
    if _chrp_type != _CHRP_Pegasos { return; }
    let np = of_find_node_by_type(core::ptr::null_mut(), b"cpu\0".as_ptr() as *const i8);
    if !np.is_null() { let l2cr = of_get_property(np, b"l2cr\0".as_ptr() as *const i8, core::ptr::null_mut()) as *const u32; if l2cr.is_null() { printk(b"\0".as_ptr() as *const i8); } else if *l2cr & 0x80000000 == 0 { _set_L2CR(0); _set_L2CR(*l2cr | 0x80000000); } }
    of_node_put(np);
}

unsafe extern "C" fn briq_restart(_cmd: *mut i8) -> ! { local_irq_disable(); if !briq_SPOR.is_null() { out_be32(briq_SPOR, 0); } loop {} }

unsafe extern "C" fn chrp_init() {
    if strstr(boot_command_line, b"console=\0".as_ptr() as *const i8) != core::ptr::null_mut() || of_chosen.is_null() { return; }
    let node = of_find_node_by_path(b"/\0".as_ptr() as *const i8); if node.is_null() { return; }
    let property = of_get_property(node, b"model\0".as_ptr() as *const i8, core::ptr::null_mut()); if property.is_null() || strcmp(property, b"Pegasos2\0".as_ptr() as *const i8) != 0 { of_node_put(node); return; }
    let property = of_get_property(of_chosen, b"linux,stdout-path\0".as_ptr() as *const i8, core::ptr::null_mut()); if property.is_null() { of_node_put(node); return; }
    of_node_put(node); let node = of_find_node_by_path(property); if node.is_null() { return; }
    if of_node_is_type(node, b"serial\0".as_ptr() as *const i8) && (of_node_name_eq(node, b"failsafe\0".as_ptr() as *const i8) || of_node_name_eq(node, b"serial\0".as_ptr() as *const i8)) { add_preferred_console(b"ttyS\0".as_ptr() as *const i8, 0, core::ptr::null()); }
    of_node_put(node);
}

unsafe extern "C" fn chrp_setup_arch() {
    let root = of_find_node_by_path(b"/\0".as_ptr() as *const i8); let machine = if !root.is_null() { of_get_property(root, b"model\0".as_ptr() as *const i8, core::ptr::null_mut()) } else { core::ptr::null() };
    loops_per_jiffy = 50000000 / HZ;
    if !machine.is_null() && strncmp(machine, b"Pegasos\0".as_ptr() as *const i8, 7) == 0 { _chrp_type = _CHRP_Pegasos; } else if !machine.is_null() && strncmp(machine, b"IBM\0".as_ptr() as *const i8, 3) == 0 { _chrp_type = _CHRP_IBM; } else if !machine.is_null() && strncmp(machine, b"MOT\0".as_ptr() as *const i8, 3) == 0 { _chrp_type = _CHRP_Motorola; } else if !machine.is_null() && strncmp(machine, b"TotalImpact,BRIQ-1\0".as_ptr() as *const i8, 18) == 0 { _chrp_type = _CHRP_briq; briq_SPOR = ioremap(0xff0000e8, 4); ppc_md.restart = Some(briq_restart); } else { _chrp_type = _CHRP_IBM; }
    of_node_put(root); printk(b"chrp type = %x [%s]\n\0".as_ptr() as *const i8, _chrp_type, chrp_names[_chrp_type as usize].as_ptr()); rtas_initialize(); pegasos_set_l2cr(); sio_init();
}

// Remaining machine-vector and interrupt setup declarations retain the source interfaces.
unsafe extern "C" fn chrp_init2() { request_region(0x20,0x20,b"pic1\0".as_ptr() as *const i8); request_region(0xa0,0x20,b"pic2\0".as_ptr() as *const i8); request_region(0,0x20,b"dma1\0".as_ptr() as *const i8); request_region(0x40,0x20,b"timer\0".as_ptr() as *const i8); request_region(0x80,0x10,b"dma page reg\0".as_ptr() as *const i8); request_region(0xc0,0x20,b"dma2\0".as_ptr() as *const i8); }
unsafe extern "C" fn chrp_probe() -> i32 { let dtype = of_get_flat_dt_prop(of_get_flat_dt_root(), b"device_type\0".as_ptr() as *const i8, core::ptr::null_mut()); if dtype.is_null() || strcmp(dtype,b"chrp\0".as_ptr() as *const i8) != 0 { return 0; } DMA_MODE_READ=0x44; DMA_MODE_WRITE=0x48; pm_power_off=Some(rtas_power_off); chrp_init(); 1 }

unsafe extern "C" fn chrp_8259_cascade(desc: *mut irq_desc) { let chip = irq_desc_get_chip(desc); let cascade_irq = i8259_irq(); if cascade_irq != 0 { generic_handle_irq(cascade_irq); } ((*chip).irq_eoi)(core::ptr::addr_of_mut!((*desc).irq_data)); }

unsafe extern "C" fn chrp_find_openpic() {
    let np = of_find_node_by_type(core::ptr::null_mut(), b"open-pic\0".as_ptr() as *const i8); if np.is_null() { return; }
    let root = of_find_node_by_path(b"/\0".as_ptr() as *const i8); let mut na = 1; let mut opaddr: c_ulong = 0;
    if !root.is_null() { na = of_n_addr_cells(root); }
    let mut r = resource { start: 0 }; if of_address_to_resource(np, 0, &mut r) == 0 { opaddr = r.start; }
    let mut len = 0; let iranges = of_get_property(np, b"interrupt-ranges\0".as_ptr() as *const i8, &mut len);
    if !iranges.is_null() { len /= 2 * core::mem::size_of::<u32>() as i32; }
    let isu_size = if len > 1 { *iranges.add(3) } else { 0 };
    chrp_mpic = mpic_alloc(np, opaddr, MPIC_NO_RESET, isu_size, 0, b" MPIC    \0".as_ptr() as *const i8);
    if !chrp_mpic.is_null() { mpic_init(chrp_mpic); ppc_md.get_irq = Some(mpic_get_irq); }
    of_node_put(root); of_node_put(np); let _ = na;
}

unsafe extern "C" fn chrp_find_8259() {
    let mut np: *mut device_node = core::ptr::null_mut(); let mut pic: *mut device_node = core::ptr::null_mut(); let mut ack: c_ulong = 0;
    // for_each_node_by_type(np, "interrupt-controller") and compatibility probing.
    while !(np = of_find_compatible_node(np, b"interrupt-controller\0".as_ptr() as *const i8, b"chrp,iic\0".as_ptr() as *const i8)).is_null() { pic = np; break; }
    if pic.is_null() && !chrp_mpic.is_null() { return; }
    np = core::ptr::null_mut(); while !(np = of_find_node_by_name(np, b"pci\0".as_ptr() as *const i8)).is_null() { let p = of_get_property(np,b"8259-interrupt-acknowledge\0".as_ptr() as *const i8,core::ptr::null_mut()); if !p.is_null() { ack=*p.add(of_n_addr_cells(np)-1); break; } }
    of_node_put(np); i8259_init(pic, ack); if ppc_md.get_irq.is_none() { ppc_md.get_irq=Some(i8259_irq); irq_set_default_domain(i8259_get_host()); }
    if !chrp_mpic.is_null() { let cascade_irq=irq_of_parse_and_map(pic,0); if cascade_irq != 0 { irq_set_chained_handler(cascade_irq,Some(chrp_8259_cascade)); } }
}

unsafe extern "C" fn chrp_init_IRQ() { chrp_find_openpic(); chrp_find_8259(); if _chrp_type == _CHRP_Pegasos { ppc_md.get_irq=Some(i8259_irq); } }

// define_machine(chrp) { .name = "CHRP", .probe = chrp_probe, .setup_arch = chrp_setup_arch,
// .discover_phbs = chrp_find_bridges, .init = chrp_init2, .show_cpuinfo = chrp_show_cpuinfo,
// .init_IRQ = chrp_init_IRQ, .restart = rtas_restart, .halt = rtas_halt,
// .time_init = chrp_time_init, .set_rtc_time = chrp_set_rtc_time,
// .get_rtc_time = chrp_get_rtc_time, .phys_mem_access_prot = pci_phys_mem_access_prot }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
