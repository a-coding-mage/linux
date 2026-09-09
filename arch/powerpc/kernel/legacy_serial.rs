// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies and build-time configuration are supplied by the surrounding tree.

const MAX_LEGACY_SERIAL_PORTS: usize = 8;

static mut LEGACY_SERIAL_PORTS: [plat_serial8250_port; MAX_LEGACY_SERIAL_PORTS + 1] =
    [plat_serial8250_port { ..unsafe { core::mem::zeroed() } }; MAX_LEGACY_SERIAL_PORTS + 1];
static mut LEGACY_SERIAL_INFOS: [legacy_serial_info; MAX_LEGACY_SERIAL_PORTS] =
    [legacy_serial_info { np: core::ptr::null_mut(), speed: 0, clock: 0,
        irq_check_parent: 0, taddr: 0, early_addr: core::ptr::null_mut() };
        MAX_LEGACY_SERIAL_PORTS];

#[repr(C)]
struct legacy_serial_info {
    np: *mut device_node,
    speed: u32,
    clock: u32,
    irq_check_parent: i32,
    taddr: phys_addr_t,
    early_addr: *mut core::ffi::c_void,
}

static LEGACY_SERIAL_PARENTS: [of_device_id; 7] = [
    of_device_id { type_: b"soc\0".as_ptr() as *const i8 },
    of_device_id { type_: b"tsi-bridge\0".as_ptr() as *const i8 },
    of_device_id { type_: b"opb\0".as_ptr() as *const i8 },
    of_device_id { compatible: b"ibm,opb\0".as_ptr() as *const i8 },
    of_device_id { compatible: b"simple-bus\0".as_ptr() as *const i8 },
    of_device_id { compatible: b"wrs,epld-localbus\0".as_ptr() as *const i8 },
    of_device_id { ..unsafe { core::mem::zeroed() } },
];
static mut LEGACY_SERIAL_COUNT: u32 = 0;
static mut LEGACY_SERIAL_CONSOLE: i32 = -1;
const LEGACY_PORT_FLAGS: upf_t = UPF_BOOT_AUTOCONF | UPF_SKIP_TEST | UPF_SHARE_IRQ | UPF_FIXED_PORT;

unsafe extern "C" fn tsi_serial_in(p: *mut uart_port, mut offset: u32) -> u32 {
    offset <<= (*p).regshift;
    if offset == UART_IIR {
        let tmp = readl((*p).membase.add((UART_IIR & !3) as usize));
        (tmp >> 16) & 0xff
    } else { readb((*p).membase.add(offset as usize)) as u32 }
}

unsafe extern "C" fn tsi_serial_out(p: *mut uart_port, mut offset: u32, value: u32) {
    offset <<= (*p).regshift;
    if !((offset == UART_IER) && (value & UART_IER_UUE) != 0) {
        writeb(value as u8, (*p).membase.add(offset as usize));
    }
}

unsafe fn add_legacy_port(np: *mut device_node, want_index: i32, iotype: i32,
    base: phys_addr_t, taddr: phys_addr_t, irq: c_ulong, flags: upf_t,
    irq_check_parent: i32) -> i32 {
    let mut clock: u32 = BASE_BAUD * 16;
    let mut shift: u32 = 0;
    let clk = of_get_property(np, c"clock-frequency".as_ptr(), core::ptr::null_mut());
    if !clk.is_null() && *clk != 0 { clock = be32_to_cpup(clk); }
    let spd = of_get_property(np, c"current-speed".as_ptr(), core::ptr::null_mut());
    let rs = of_get_property(np, c"reg-shift".as_ptr(), core::ptr::null_mut());
    if !rs.is_null() && *rs != 0 { shift = be32_to_cpup(rs); }
    let index = if want_index >= 0 && (want_index as usize) < MAX_LEGACY_SERIAL_PORTS {
        want_index as usize
    } else { LEGACY_SERIAL_COUNT as usize };
    if index >= MAX_LEGACY_SERIAL_PORTS { return -1; }
    if index as u32 >= LEGACY_SERIAL_COUNT { LEGACY_SERIAL_COUNT = index as u32 + 1; }
    let port = &mut LEGACY_SERIAL_PORTS[index];
    let info = &mut LEGACY_SERIAL_INFOS[index];
    if !info.np.is_null() {
        if (LEGACY_SERIAL_COUNT as usize) < MAX_LEGACY_SERIAL_PORTS {
            LEGACY_SERIAL_PORTS[LEGACY_SERIAL_COUNT as usize] = *port;
            LEGACY_SERIAL_INFOS[LEGACY_SERIAL_COUNT as usize] = core::ptr::read(info);
            LEGACY_SERIAL_COUNT += 1;
        }
    }
    core::ptr::write_bytes(port as *mut _, 0, 1);
    if iotype == UPIO_PORT { port.iobase = base; } else { port.mapbase = base; }
    port.iotype = iotype; port.uartclk = clock; port.irq = irq; port.flags = flags; port.regshift = shift;
    info.taddr = taddr; info.np = of_node_get(np); info.clock = clock;
    info.speed = if !spd.is_null() { be32_to_cpup(spd) } else { 0 };
    info.irq_check_parent = irq_check_parent;
    if iotype == UPIO_TSI { port.serial_in = Some(tsi_serial_in); port.serial_out = Some(tsi_serial_out); }
    index as i32
}

unsafe fn add_legacy_soc_port(np: *mut device_node, soc_dev: *mut device_node) -> i32 {
    if !of_property_present(np, c"clock-frequency".as_ptr()) || of_property_present(np, c"reg-offset".as_ptr()) || of_property_read_bool(np, c"used-by-rtas".as_ptr()) { return -1; }
    let addrp = of_get_address(soc_dev, 0, core::ptr::null_mut(), core::ptr::null_mut());
    if addrp.is_null() { return -1; }
    let addr = of_translate_address(soc_dev, addrp); if addr == OF_BAD_ADDR { return -1; }
    let tsi = of_get_parent(np);
    let typ = if of_node_is_type(tsi, c"tsi-bridge".as_ptr()) { UPIO_TSI } else { UPIO_MEM };
    add_legacy_port(np, -1, typ, addr, addr, 0, LEGACY_PORT_FLAGS, 0)
}

unsafe fn add_legacy_isa_port(np: *mut device_node, isa_brg: *mut device_node) -> i32 {
    let reg = of_get_property(np, c"reg".as_ptr(), core::ptr::null_mut()); if reg.is_null() { return -1; }
    if (be32_to_cpu(*reg) & 1) == 0 { return -1; }
    let typep = of_get_property(np, c"ibm,aix-loc".as_ptr(), core::ptr::null_mut());
    let mut index = -1; if !typep.is_null() && *typep == b'S' as i8 { index = simple_strtol(typep.add(1), core::ptr::null_mut(), 0) as i32 - 1; }
    let taddr = if !of_device_is_compatible(isa_brg, c"ibm,power8-lpc".as_ptr()) || of_property_present(isa_brg, c"ranges".as_ptr()) { let a = of_translate_address(np, reg); if a == OF_BAD_ADDR { 0 } else { a } } else { 0 };
    add_legacy_port(np, index, UPIO_PORT, be32_to_cpu(*reg.add(1)) as phys_addr_t, taddr, 0, LEGACY_PORT_FLAGS, 0)
}

unsafe fn setup_legacy_serial_console(console: i32) {
    let info = &mut LEGACY_SERIAL_INFOS[console as usize]; let port = &mut LEGACY_SERIAL_PORTS[console as usize];
    let stride = 1u32 << port.regshift;
    if info.taddr != 0 { info.early_addr = early_ioremap(info.taddr, 0x1000); if info.early_addr.is_null() { return; } udbg_uart_init_mmio(info.early_addr, stride); }
    else if port.iotype == UPIO_PORT && isa_io_special { udbg_uart_init_pio(port.iobase, stride); } else { return; }
    if info.speed == 0 { info.speed = udbg_probe_uart_speed(info.clock); } udbg_uart_setup(info.speed, info.clock);
}

// The remaining initcall bodies retain the source control flow and call external kernel APIs.
unsafe fn ioremap_legacy_serial_console() -> i32 {
    if LEGACY_SERIAL_CONSOLE < 0 { return 0; }
    let info = &mut LEGACY_SERIAL_INFOS[LEGACY_SERIAL_CONSOLE as usize]; let port = &mut LEGACY_SERIAL_PORTS[LEGACY_SERIAL_CONSOLE as usize];
    if info.early_addr.is_null() { return 0; }
    let vaddr = ioremap(info.taddr, 0x1000); if vaddr.is_null() { return -ENOMEM; }
    udbg_uart_init_mmio(vaddr, 1 << port.regshift); early_iounmap(info.early_addr, 0x1000); info.early_addr = core::ptr::null_mut(); 0
}

unsafe fn find_legacy_serial_ports() {
    let mut stdout = core::ptr::null_mut(); let mut path = of_get_property(of_chosen, c"linux,stdout-path".as_ptr(), core::ptr::null_mut());
    if path.is_null() { path = of_get_property(of_chosen, c"stdout-path".as_ptr(), core::ptr::null_mut()); }
    if !path.is_null() { stdout = of_find_node_by_path(path); }
    let mut np = core::ptr::null_mut();
    for_each_compatible_node!(np, "serial", "ns16550", { let parent = of_get_parent(np); if !parent.is_null() && !of_match_node(LEGACY_SERIAL_PARENTS.as_ptr(), parent).is_null() && of_device_is_available(np) { let index = add_legacy_soc_port(np, np); if index >= 0 && np == stdout { LEGACY_SERIAL_CONSOLE = index; } } of_node_put(parent); });
    for_each_node_by_type!(np, "serial", { let isa = of_get_parent(np); if of_node_name_eq(isa, c"isa".as_ptr()) || of_node_name_eq(isa, c"lpc".as_ptr()) { if of_device_is_available(np) { let index = add_legacy_isa_port(np, isa); if index >= 0 && np == stdout { LEGACY_SERIAL_CONSOLE = index; } } } of_node_put(isa); });
    of_node_put(stdout); if LEGACY_SERIAL_CONSOLE >= 0 { setup_legacy_serial_console(LEGACY_SERIAL_CONSOLE); }
}

unsafe fn fixup_port_irq(index: i32, mut np: *mut device_node, port: *mut plat_serial8250_port) {
    let mut virq = irq_of_parse_and_map(np, 0);
    if virq == 0 && LEGACY_SERIAL_INFOS[index as usize].irq_check_parent != 0 { np = of_get_parent(np); if np.is_null() { return; } virq = irq_of_parse_and_map(np, 0); of_node_put(np); }
    if virq != 0 { (*port).irq = virq; }
}
unsafe fn fixup_port_pio(index: i32, np: *mut device_node, port: *mut plat_serial8250_port) {
    #[cfg(CONFIG_PCI)] { let hose = pci_find_hose_for_OF_device(np); if !hose.is_null() { (*port).iobase += ((*hose).io_base_virt as usize as c_ulong) - isa_io_base; } }
}
unsafe fn fixup_port_mmio(_index: i32, _np: *mut device_node, port: *mut plat_serial8250_port) { (*port).membase = ioremap((*port).mapbase, 0x100); }
unsafe fn serial_dev_init() -> i32 {
    if LEGACY_SERIAL_COUNT == 0 { return -ENODEV; }
    for i in 0..LEGACY_SERIAL_COUNT as usize { let port = &mut LEGACY_SERIAL_PORTS[i]; let np = LEGACY_SERIAL_INFOS[i].np; if port.irq == 0 { fixup_port_irq(i as i32, np, port); } if port.iotype == UPIO_PORT { fixup_port_pio(i as i32, np, port); } if port.iotype == UPIO_MEM || port.iotype == UPIO_TSI { fixup_port_mmio(i as i32, np, port); } }
    platform_device_register(&mut serial_device)
}

#[cfg(CONFIG_SERIAL_8250_CONSOLE)]
unsafe fn check_legacy_serial_console() -> i32 {
    if !strstr(boot_command_line, c"console=".as_ptr()).is_null() { return -EBUSY; }
    if of_chosen.is_null() || LEGACY_SERIAL_CONSOLE < 0 { return -ENODEV; }
    let mut name = of_get_property(of_chosen, c"linux,stdout-path".as_ptr(), core::ptr::null_mut()); if name.is_null() { name = of_get_property(of_chosen, c"stdout-path".as_ptr(), core::ptr::null_mut()); } if name.is_null() { return -ENODEV; }
    let stdout = of_find_node_by_path(name); if stdout.is_null() { return -ENODEV; }
    name = of_get_property(stdout, c"name".as_ptr(), core::ptr::null_mut()); if name.is_null() { of_node_put(stdout); return -ENODEV; }
    if strcmp(name, c"serial".as_ptr()) != 0 { of_node_put(stdout); return -ENODEV; }
    let mut speed = 0; let mut offset = 0; let mut i = 0; while i < LEGACY_SERIAL_COUNT as usize { if stdout == LEGACY_SERIAL_INFOS[i].np { offset = i; speed = LEGACY_SERIAL_INFOS[i].speed; break; } i += 1; }
    if i >= LEGACY_SERIAL_COUNT as usize { of_node_put(stdout); return -ENODEV; } of_node_put(stdout);
    if speed != 0 { let mut opt = [0i8; 16]; sprintf(opt.as_mut_ptr(), c"%d".as_ptr(), speed); add_preferred_console(c"ttyS".as_ptr(), offset as i32, opt.as_ptr()) } else { add_preferred_console(c"ttyS".as_ptr(), offset as i32, core::ptr::null()) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
