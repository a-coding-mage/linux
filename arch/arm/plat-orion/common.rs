/* Rust translation of arch/arm/plat-orion/common.c. */

/* C headers and build-time kernel definitions are supplied by other files. */

pub unsafe fn orion_clkdev_add(con_id: *const i8, dev_id: *const i8, clk: *mut clk) {
    clkdev_create(clk, con_id, b"%s\0".as_ptr() as *const i8, dev_id);
}

pub unsafe fn orion_clkdev_init(tclk: *mut clk) {
    orion_clkdev_add(core::ptr::null(), b"orion_spi.0\0".as_ptr() as *const i8, tclk);
    orion_clkdev_add(core::ptr::null(), b"orion_spi.1\0".as_ptr() as *const i8, tclk);
    orion_clkdev_add(core::ptr::null(), concat!(MV643XX_ETH_NAME, ".0\0").as_ptr() as *const i8, tclk);
    orion_clkdev_add(core::ptr::null(), concat!(MV643XX_ETH_NAME, ".1\0").as_ptr() as *const i8, tclk);
    orion_clkdev_add(core::ptr::null(), concat!(MV643XX_ETH_NAME, ".2\0").as_ptr() as *const i8, tclk);
    orion_clkdev_add(core::ptr::null(), concat!(MV643XX_ETH_NAME, ".3\0").as_ptr() as *const i8, tclk);
    orion_clkdev_add(core::ptr::null(), b"orion_wdt\0".as_ptr() as *const i8, tclk);
    orion_clkdev_add(core::ptr::null(), concat!(MV64XXX_I2C_CTLR_NAME, ".0\0").as_ptr() as *const i8, tclk);
}

unsafe fn fill_resources(device: *mut platform_device, resources: *mut resource, mapbase: resource_size_t, size: resource_size_t) {
    (*device).resource = resources;
    (*device).num_resources = 1;
    (*resources).flags = IORESOURCE_MEM;
    (*resources).start = mapbase;
    (*resources).end = mapbase.wrapping_add(size);
}

unsafe fn fill_resources_irq(device: *mut platform_device, resources: *mut resource, mapbase: resource_size_t, size: resource_size_t, irq: u32) {
    fill_resources(device, resources, mapbase, size);
    (*device).num_resources += 1;
    (*resources.add(1)).flags = IORESOURCE_IRQ;
    (*resources.add(1)).start = irq as _;
    (*resources.add(1)).end = irq as _;
}

unsafe fn uart_get_clk_rate(c: *mut clk) -> c_ulong { clk_prepare_enable(c); clk_get_rate(c) }

unsafe fn uart_complete(orion_uart: *mut platform_device, data: *mut plat_serial8250_port, resources: *mut resource, membase: *mut core::ffi::c_void, mapbase: resource_size_t, irq: u32, c: *mut clk) {
    (*data).mapbase = mapbase; (*data).membase = membase; (*data).irq = irq;
    (*data).uartclk = uart_get_clk_rate(c); (*orion_uart).dev.platform_data = data as *mut _;
    fill_resources_irq(orion_uart, resources, mapbase, 0xff, irq); platform_device_register(orion_uart);
}

static mut orion_uart0_data: [plat_serial8250_port; 2] = unsafe { core::mem::zeroed() };
static mut orion_uart1_data: [plat_serial8250_port; 2] = unsafe { core::mem::zeroed() };
static mut orion_uart2_data: [plat_serial8250_port; 2] = unsafe { core::mem::zeroed() };
static mut orion_uart3_data: [plat_serial8250_port; 2] = unsafe { core::mem::zeroed() };
static mut orion_uart0_resources: [resource; 2] = unsafe { core::mem::zeroed() };
static mut orion_uart1_resources: [resource; 2] = unsafe { core::mem::zeroed() };
static mut orion_uart2_resources: [resource; 2] = unsafe { core::mem::zeroed() };
static mut orion_uart3_resources: [resource; 2] = unsafe { core::mem::zeroed() };
static mut orion_uart0: platform_device = unsafe { core::mem::zeroed() };
static mut orion_uart1: platform_device = unsafe { core::mem::zeroed() };
static mut orion_uart2: platform_device = unsafe { core::mem::zeroed() };
static mut orion_uart3: platform_device = unsafe { core::mem::zeroed() };
static mut orion_rtc_resource: [resource; 2] = unsafe { core::mem::zeroed() };

/* The four UART instances retain their C static storage and platform IDs. */
pub unsafe fn orion_uart0_init(m: *mut core::ffi::c_void, b: resource_size_t, i: u32, c: *mut clk) { uart_complete(&mut orion_uart0, orion_uart0_data.as_mut_ptr(), orion_uart0_resources.as_mut_ptr(), m,b,i,c); }
pub unsafe fn orion_uart1_init(m: *mut core::ffi::c_void, b: resource_size_t, i: u32, c: *mut clk) { uart_complete(&mut orion_uart1, orion_uart1_data.as_mut_ptr(), orion_uart1_resources.as_mut_ptr(), m,b,i,c); }
pub unsafe fn orion_uart2_init(m: *mut core::ffi::c_void, b: resource_size_t, i: u32, c: *mut clk) { uart_complete(&mut orion_uart2, orion_uart2_data.as_mut_ptr(), orion_uart2_resources.as_mut_ptr(), m,b,i,c); }
pub unsafe fn orion_uart3_init(m: *mut core::ffi::c_void, b: resource_size_t, i: u32, c: *mut clk) { uart_complete(&mut orion_uart3, orion_uart3_data.as_mut_ptr(), orion_uart3_resources.as_mut_ptr(), m,b,i,c); }

pub unsafe fn orion_rtc_init(mapbase: c_ulong, irq: c_ulong) { orion_rtc_resource[0].start=mapbase; orion_rtc_resource[0].end=mapbase+SZ_32-1; orion_rtc_resource[0].flags=IORESOURCE_MEM; orion_rtc_resource[1].start=irq; orion_rtc_resource[1].end=irq; orion_rtc_resource[1].flags=IORESOURCE_IRQ; platform_device_register_simple(b"rtc-mv\0".as_ptr() as *const i8,-1,orion_rtc_resource.as_mut_ptr(),2); }

unsafe fn ge_complete(shared: *mut mv643xx_eth_shared_platform_data, res: *mut resource, irq: c_ulong, shared_dev: *mut platform_device, mdio: *mut platform_device, eth: *mut mv643xx_eth_platform_data, ge: *mut platform_device) { (*res).start=irq; (*res).end=irq; (*eth).shared=shared_dev; (*ge).dev.platform_data=eth as *mut _; platform_device_register(shared_dev); if !mdio.is_null(){platform_device_register(mdio);} platform_device_register(ge); }

/* Platform objects and their initialization routines, preserving the C layout. */
pub unsafe fn orion_i2c_init(b:c_ulong,i:c_ulong,f:c_ulong){orion_i2c_pdata.freq_m=f;fill_resources_irq(&mut orion_i2c,orion_i2c_resources.as_mut_ptr(),b,SZ_32-1,i as u32);platform_device_register(&mut orion_i2c);}
pub unsafe fn orion_i2c_1_init(b:c_ulong,i:c_ulong,f:c_ulong){orion_i2c_1_pdata.freq_m=f;fill_resources_irq(&mut orion_i2c_1,orion_i2c_1_resources.as_mut_ptr(),b,SZ_32-1,i as u32);platform_device_register(&mut orion_i2c_1);}
pub unsafe fn orion_spi_init(b:c_ulong){fill_resources(&mut orion_spi,&mut orion_spi_resources,b,SZ_512-1);platform_device_register(&mut orion_spi);}
pub unsafe fn orion_spi_1_init(b:c_ulong){fill_resources(&mut orion_spi_1,&mut orion_spi_1_resources,b,SZ_512-1);platform_device_register(&mut orion_spi_1);}

pub unsafe fn orion_ehci_init(b:c_ulong,i:c_ulong,p:orion_ehci_phy_ver){orion_ehci_data.phy_version=p;fill_resources_irq(&mut orion_ehci,orion_ehci_resources.as_mut_ptr(),b,SZ_4K-1,i as u32);platform_device_register(&mut orion_ehci);}
pub unsafe fn orion_ehci_1_init(b:c_ulong,i:c_ulong){fill_resources_irq(&mut orion_ehci_1,orion_ehci_1_resources.as_mut_ptr(),b,SZ_4K-1,i as u32);platform_device_register(&mut orion_ehci_1);}
pub unsafe fn orion_ehci_2_init(b:c_ulong,i:c_ulong){fill_resources_irq(&mut orion_ehci_2,orion_ehci_2_resources.as_mut_ptr(),b,SZ_4K-1,i as u32);platform_device_register(&mut orion_ehci_2);}
pub unsafe fn orion_sata_init(d:*mut mv_sata_platform_data,b:c_ulong,i:c_ulong){orion_sata.dev.platform_data=d as *mut _;fill_resources_irq(&mut orion_sata,orion_sata_resources.as_mut_ptr(),b,0x5000-1,i as u32);platform_device_register(&mut orion_sata);}
pub unsafe fn orion_crypto_init(b:c_ulong,s:c_ulong,n:c_ulong,i:c_ulong){fill_resources_irq(&mut orion_crypto,orion_crypto_resources.as_mut_ptr(),b,0xffff,i as u32);orion_crypto.num_resources=3;orion_crypto_resources[2].start=s;orion_crypto_resources[2].end=s+n-1;platform_device_register(&mut orion_crypto);}

pub unsafe fn orion_ge00_init(d:*mut mv643xx_eth_platform_data,b:c_ulong,i:c_ulong,e:c_ulong,t:u32){fill_resources(&mut orion_ge00_shared,orion_ge00_shared_resources.as_mut_ptr(),b+0x2000,SZ_16K-1);fill_resources_irq(&mut orion_ge_mvmdio,orion_ge_mvmdio_resources.as_mut_ptr(),b+0x2004,0x84-1,e as u32);orion_ge00_shared_data.tx_csum_limit=t;ge_complete(&mut orion_ge00_shared_data,orion_ge00_resources.as_mut_ptr(),i,&mut orion_ge00_shared,&mut orion_ge_mvmdio,d,&mut orion_ge00);}
pub unsafe fn orion_ge01_init(d:*mut mv643xx_eth_platform_data,b:c_ulong,i:c_ulong,t:u32){fill_resources(&mut orion_ge01_shared,orion_ge01_shared_resources.as_mut_ptr(),b+0x2000,SZ_16K-1);orion_ge01_shared_data.tx_csum_limit=t;ge_complete(&mut orion_ge01_shared_data,orion_ge01_resources.as_mut_ptr(),i,&mut orion_ge01_shared,core::ptr::null_mut(),d,&mut orion_ge01);}
pub unsafe fn orion_ge10_init(d:*mut mv643xx_eth_platform_data,b:c_ulong,i:c_ulong){fill_resources(&mut orion_ge10_shared,orion_ge10_shared_resources.as_mut_ptr(),b+0x2000,SZ_16K-1);ge_complete(&mut orion_ge10_shared_data,orion_ge10_resources.as_mut_ptr(),i,&mut orion_ge10_shared,core::ptr::null_mut(),d,&mut orion_ge10);}
pub unsafe fn orion_ge11_init(d:*mut mv643xx_eth_platform_data,b:c_ulong,i:c_ulong){fill_resources(&mut orion_ge11_shared,orion_ge11_shared_resources.as_mut_ptr(),b+0x2000,SZ_16K-1);ge_complete(&mut orion_ge11_shared_data,orion_ge11_resources.as_mut_ptr(),i,&mut orion_ge11_shared,core::ptr::null_mut(),d,&mut orion_ge11);}

pub unsafe fn orion_xor0_init(l:c_ulong,h:c_ulong,a:c_ulong,b:c_ulong){orion_xor0_shared_resources[0].start=l;orion_xor0_shared_resources[0].end=l+0xff;orion_xor0_shared_resources[1].start=h;orion_xor0_shared_resources[1].end=h+0xff;orion_xor0_shared_resources[2].start=a;orion_xor0_shared_resources[2].end=a;orion_xor0_shared_resources[3].start=b;orion_xor0_shared_resources[3].end=b;dma_cap_set(DMA_MEMCPY,orion_xor0_channels_data[0].cap_mask);dma_cap_set(DMA_XOR,orion_xor0_channels_data[0].cap_mask);dma_cap_set(DMA_MEMCPY,orion_xor0_channels_data[1].cap_mask);dma_cap_set(DMA_XOR,orion_xor0_channels_data[1].cap_mask);platform_device_register(&mut orion_xor0_shared);}
pub unsafe fn orion_xor1_init(l:c_ulong,h:c_ulong,a:c_ulong,b:c_ulong){orion_xor1_shared_resources[0].start=l;orion_xor1_shared_resources[0].end=l+0xff;orion_xor1_shared_resources[1].start=h;orion_xor1_shared_resources[1].end=h+0xff;orion_xor1_shared_resources[2].start=a;orion_xor1_shared_resources[2].end=a;orion_xor1_shared_resources[3].start=b;orion_xor1_shared_resources[3].end=b;dma_cap_set(DMA_MEMCPY,orion_xor1_channels_data[0].cap_mask);dma_cap_set(DMA_XOR,orion_xor1_channels_data[0].cap_mask);dma_cap_set(DMA_MEMCPY,orion_xor1_channels_data[1].cap_mask);dma_cap_set(DMA_XOR,orion_xor1_channels_data[1].cap_mask);platform_device_register(&mut orion_xor1_shared);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
