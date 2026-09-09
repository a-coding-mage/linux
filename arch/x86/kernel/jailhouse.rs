// SPDX-License-Identifier: GPL-2.0
/*
 * Jailhouse paravirt_ops implementation
 *
 * Copyright (c) Siemens AG, 2015-2017
 *
 * Authors:
 *  Jan Kiszka <jan.kiszka@siemens.com>
 */

// Kernel and architecture dependencies supplied by the surrounding translation unit.

static mut setup_data: jailhouse_setup_data = jailhouse_setup_data::default();
const SETUP_DATA_V1_LEN: usize = core::mem::size_of::<jailhouse_setup_data_hdr>() +
    core::mem::size_of::<jailhouse_setup_data_v1>();
const SETUP_DATA_V2_LEN: usize = SETUP_DATA_V1_LEN +
    core::mem::size_of::<jailhouse_setup_data_v2>();

static mut precalibrated_tsc_khz: u32 = 0;

unsafe fn jailhouse_setup_irq(irq: u32) {
    let mut mp_irq = mpc_intsrc {
        type_: MP_INTSRC,
        irqtype: mp_INT,
        irqflag: MP_IRQPOL_ACTIVE_HIGH | MP_IRQTRIG_EDGE,
        srcbusirq: irq,
        dstirq: irq,
    };
    mp_save_irq(&mut mp_irq);
}

unsafe fn jailhouse_cpuid_base() -> u32 {
    if boot_cpu_data.cpuid_level < 0 || !boot_cpu_has(X86_FEATURE_HYPERVISOR) {
        return 0;
    }

    cpuid_base_hypervisor(*b"Jailhouse\0\0\0", 0)
}

unsafe fn jailhouse_detect() -> u32 {
    jailhouse_cpuid_base()
}

unsafe fn jailhouse_get_wallclock(now: *mut timespec64) {
    core::ptr::write_bytes(now, 0, 1);
}

unsafe fn jailhouse_timer_init() {
    lapic_timer_period = (*setup_data).v1.apic_khz * (1000 / HZ);
}

unsafe fn jailhouse_get_tsc() -> usize {
    precalibrated_tsc_khz as usize
}

unsafe fn jailhouse_x2apic_init() {
    // CONFIG_X86_X2APIC
    if !x2apic_enabled() {
        return;
    }
    /*
     * We do not have access to IR inside Jailhouse non-root cells.  So
     * we have to run in physical mode.
     */
    x2apic_phys = 1;
    /*
     * This will trigger the switch to apic_x2apic_phys.  Empty OEM IDs
     * ensure that only this APIC driver picks up the call.
     */
    default_acpi_madt_oem_check("", "");
}

unsafe fn jailhouse_parse_smp_config() {
    let ioapic_cfg = ioapic_domain_cfg {
        type_: IOAPIC_DOMAIN_STRICT,
        ops: &mp_ioapic_irqdomain_ops,
    };
    jailhouse_x2apic_init();

    register_lapic_address(0xfee00000);

    let mut cpu = 0;
    while cpu < (*setup_data).v1.num_cpus {
        topology_register_apic((*setup_data).v1.cpu_ids[cpu as usize], CPU_ACPIID_INVALID, true);
        cpu += 1;
    }

    smp_found_config = 1;

    if (*setup_data).v1.standard_ioapic {
        mp_register_ioapic(0, 0xfec00000, gsi_top, &ioapic_cfg);
        // IS_ENABLED(CONFIG_SERIAL_8250)
        if (*setup_data).hdr.version < 2 {
            /* Register 1:1 mapping for legacy UART IRQs 3 and 4 */
            jailhouse_setup_irq(3);
            jailhouse_setup_irq(4);
        }
    }
}

unsafe fn jailhouse_no_restart() {
    pr_notice!("Jailhouse: Restart not supported, halting\n");
    machine_halt();
}

unsafe fn jailhouse_pci_arch_init() -> i32 {
    pci_direct_init(1);

    /*
     * There are no bridges on the virtual PCI root bus under Jailhouse,
     * thus no other way to discover all devices than a full scan.
     * Respect any overrides via the command line, though.
     */
    if pcibios_last_bus < 0 {
        pcibios_last_bus = 0xff;
    }

    // CONFIG_PCI_MMCONFIG
    if (*setup_data).v1.pci_mmconfig_base != 0 {
        pci_mmconfig_add(0, 0, pcibios_last_bus, (*setup_data).v1.pci_mmconfig_base);
        pci_mmcfg_arch_init();
    }

    0
}

// CONFIG_SERIAL_8250
#[inline]
unsafe fn jailhouse_uart_enabled(uart_nr: u32) -> bool {
    ((*setup_data).v2.flags & (1 << uart_nr)) != 0
}

unsafe fn jailhouse_serial_fixup(_port: i32, up: *mut uart_port, _capabilities: *mut u32) {
    let pcuart_base: [u16; 4] = [0x3f8, 0x2f8, 0x3e8, 0x2e8];
    let mut n = 0;
    while n < pcuart_base.len() {
        if pcuart_base[n] as u32 != (*up).iobase {
            n += 1;
            continue;
        }

        if jailhouse_uart_enabled(n as u32) {
            pr_info!("Enabling UART{} (port 0x{:x})\n", n, (*up).iobase);
            jailhouse_setup_irq((*up).irq);
        } else {
            /* Deactivate UART if access isn't allowed */
            (*up).iobase = 0;
        }
        break;
    }
}

unsafe fn jailhouse_serial_workaround() {
    /*
     * There are flags inside setup_data that indicate availability of
     * platform UARTs since setup data version 2.
     *
     * In case of version 1, we don't know which UARTs belong Linux. In
     * this case, unconditionally register 1:1 mapping for legacy UART IRQs
     * 3 and 4.
     */
    if (*setup_data).hdr.version > 1 {
        serial8250_set_isa_configurator(Some(jailhouse_serial_fixup));
    }
}

unsafe fn jailhouse_init_platform() {
    let mut pa_data = boot_params.hdr.setup_data;
    let setup_data_len: usize;
    let mut header: setup_data;
    let mut mapping: *mut core::ffi::c_void;

    x86_init.irqs.pre_vector_init = x86_init_noop;
    x86_init.timers.timer_init = jailhouse_timer_init;
    x86_init.mpparse.find_mptable = x86_init_noop;
    x86_init.mpparse.early_parse_smp_cfg = x86_init_noop;
    x86_init.mpparse.parse_smp_cfg = jailhouse_parse_smp_config;
    x86_init.pci.arch_init = jailhouse_pci_arch_init;

    x86_platform.calibrate_cpu = jailhouse_get_tsc;
    x86_platform.calibrate_tsc = jailhouse_get_tsc;
    x86_platform.get_wallclock = jailhouse_get_wallclock;
    x86_platform.legacy.rtc = 0;
    x86_platform.legacy.warm_reset = 0;
    x86_platform.legacy.i8042 = X86_LEGACY_I8042_PLATFORM_ABSENT;

    legacy_pic = &null_legacy_pic;
    machine_ops.emergency_restart = jailhouse_no_restart;

    while pa_data != 0 {
        mapping = early_memremap(pa_data, core::mem::size_of::<setup_data>());
        core::ptr::copy_nonoverlapping(mapping as *const setup_data, &mut header, 1);
        early_memunmap(mapping, core::mem::size_of::<setup_data>());
        if header.type_ == SETUP_JAILHOUSE {
            break;
        }
        pa_data = header.next;
    }

    if pa_data == 0 {
        panic!("Jailhouse: No valid setup data found");
    }
    if header.len < core::mem::size_of::<jailhouse_setup_data_hdr>() {
        panic!("Jailhouse: Unsupported setup data structure");
    }

    pa_data += core::mem::offset_of!(setup_data, data) as u64;
    setup_data_len = core::cmp::min(core::mem::size_of_val(&setup_data), header.len as usize);
    mapping = early_memremap(pa_data, setup_data_len);
    core::ptr::copy_nonoverlapping(mapping as *const u8, &mut setup_data as *mut _ as *mut u8, setup_data_len);
    early_memunmap(mapping, setup_data_len);

    if setup_data.hdr.version == 0 ||
       setup_data.hdr.compatible_version != JAILHOUSE_SETUP_REQUIRED_VERSION ||
       (setup_data.hdr.version == 1 && header.len < SETUP_DATA_V1_LEN) ||
       (setup_data.hdr.version >= 2 && header.len < SETUP_DATA_V2_LEN) {
        panic!("Jailhouse: Unsupported setup data structure");
    }

    pmtmr_ioport = setup_data.v1.pm_timer_address;
    pr_debug!("Jailhouse: PM-Timer IO Port: %#x\n", pmtmr_ioport);
    precalibrated_tsc_khz = setup_data.v1.tsc_khz;
    setup_force_cpu_cap(X86_FEATURE_TSC_KNOWN_FREQ);
    pci_probe = 0;
    disable_acpi();
    jailhouse_serial_workaround();
}

pub unsafe fn jailhouse_paravirt() -> bool {
    jailhouse_cpuid_base() != 0
}

unsafe fn jailhouse_x2apic_available() -> bool {
    /*
     * The x2APIC is only available if the root cell enabled it. Jailhouse
     * does not support switching between xAPIC and x2APIC.
     */
    x2apic_enabled()
}

pub static x86_hyper_jailhouse: hypervisor_x86 = hypervisor_x86 {
    name: "Jailhouse",
    detect: jailhouse_detect,
    init: hypervisor_x86_init { init_platform: jailhouse_init_platform, x2apic_available: jailhouse_x2apic_available },
    ignore_nopv: true,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
