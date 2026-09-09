// SPDX-License-Identifier: GPL-2.0
/*
 * Platform driver for the Synopsys DesignWare DMA Controller
 *
 * Copyright (C) 2007-2008 Atmel Corporation
 * Copyright (C) 2010-2011 ST Microelectronics
 * Copyright (C) 2013 Intel Corporation
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn dw_dma_of_xlate(
    dma_spec: *mut of_phandle_args,
    ofdma: *mut of_dma,
) -> *mut dma_chan {
    let dw = (*ofdma).of_dma_data as *mut dw_dma;
    let mut slave = dw_dma_slave {
        dma_dev: (*(*dw).dma.dev),
        ..core::mem::zeroed()
    };
    let mut cap: dma_cap_mask_t = core::mem::zeroed();

    if (*dma_spec).args_count < 3 || (*dma_spec).args_count > 4 {
        return core::ptr::null_mut();
    }

    slave.src_id = (*dma_spec).args[0];
    slave.dst_id = (*dma_spec).args[0];
    slave.m_master = (*dma_spec).args[1];
    slave.p_master = (*dma_spec).args[2];
    if (*dma_spec).args_count >= 4 {
        slave.channels = (*dma_spec).args[3];
    }

    if WARN_ON(
        slave.src_id >= DW_DMA_MAX_NR_REQUESTS
            || slave.dst_id >= DW_DMA_MAX_NR_REQUESTS
            || slave.m_master >= (*dw).pdata.nr_masters
            || slave.p_master >= (*dw).pdata.nr_masters
            || slave.channels >= BIT((*dw).pdata.nr_channels),
    ) {
        return core::ptr::null_mut();
    }

    dma_cap_zero(&mut cap);
    dma_cap_set(DMA_SLAVE, &mut cap);

    /* TODO: there should be a simpler way to do this */
    dma_request_channel(cap, dw_dma_filter, &mut slave)
}

unsafe fn dw_dma_parse_dt(
    pdev: *mut platform_device,
) -> *mut dw_dma_platform_data {
    let np = (*pdev).dev.of_node;
    let mut pdata: *mut dw_dma_platform_data;
    let mut tmp: u32;
    let mut arr: [u32; DW_DMA_MAX_NR_MASTERS as usize] = [0; DW_DMA_MAX_NR_MASTERS as usize];
    let mut nr_masters: u32 = 0;
    let mut nr_channels: u32 = 0;

    if of_property_read_u32(np, c"dma-masters", &mut nr_masters) != 0 {
        return core::ptr::null_mut();
    }
    if nr_masters < 1 || nr_masters > DW_DMA_MAX_NR_MASTERS {
        return core::ptr::null_mut();
    }

    if of_property_read_u32(np, c"dma-channels", &mut nr_channels) != 0 {
        return core::ptr::null_mut();
    }
    if nr_channels > DW_DMA_MAX_NR_CHANNELS {
        return core::ptr::null_mut();
    }

    pdata = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<dw_dma_platform_data>(), GFP_KERNEL)
        as *mut dw_dma_platform_data;
    if pdata.is_null() {
        return core::ptr::null_mut();
    }

    (*pdata).nr_masters = nr_masters;
    (*pdata).nr_channels = nr_channels;

    of_property_read_u32(np, c"chan_allocation_order", &mut (*pdata).chan_allocation_order);
    of_property_read_u32(np, c"chan_priority", &mut (*pdata).chan_priority);
    of_property_read_u32(np, c"block_size", &mut (*pdata).block_size);

    /* Try deprecated property first */
    if of_property_read_u32_array(np, c"data_width", arr.as_mut_ptr(), nr_masters) == 0 {
        tmp = 0;
        while tmp < nr_masters {
            (*pdata).data_width[tmp as usize] = BIT(arr[tmp as usize] & 0x07);
            tmp += 1;
        }
    }

    /* If "data_width" and "data-width" both provided use the latter one */
    of_property_read_u32_array(np, c"data-width", (*pdata).data_width.as_mut_ptr(), nr_masters);

    memset32((*pdata).multi_block.as_mut_ptr(), 1, nr_channels);
    of_property_read_u32_array(np, c"multi-block", (*pdata).multi_block.as_mut_ptr(), nr_channels);

    memset32((*pdata).max_burst.as_mut_ptr(), DW_DMA_MAX_BURST, nr_channels);
    of_property_read_u32_array(np, c"snps,max-burst-len", (*pdata).max_burst.as_mut_ptr(), nr_channels);

    of_property_read_u32(np, c"snps,dma-protection-control", &mut (*pdata).protctl);
    if (*pdata).protctl > CHAN_PROTCTL_MASK {
        return core::ptr::null_mut();
    }

    pdata
}

unsafe fn dw_dma_of_controller_register(dw: *mut dw_dma) {
    let dev = (*dw).dma.dev;
    let mut ret: i32;

    if (*dev).of_node.is_null() {
        return;
    }

    ret = of_dma_controller_register((*dev).of_node, dw_dma_of_xlate, dw as *mut core::ffi::c_void);
    if ret != 0 {
        dev_err(dev, c"could not register of_dma_controller\n");
    }
}

unsafe fn dw_dma_of_controller_free(dw: *mut dw_dma) {
    let dev = (*dw).dma.dev;

    if (*dev).of_node.is_null() {
        return;
    }

    of_dma_controller_free((*dev).of_node);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
