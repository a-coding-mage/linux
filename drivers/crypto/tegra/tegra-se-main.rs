// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2023 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
/* Crypto driver for NVIDIA Security Engine in Tegra Chips */

// Linux/kernel, crypto, host1x, and tegra-se declarations are supplied by
// external dependencies in the surrounding repository.

unsafe fn tegra_se_cmdbuf_get(host_bo: *mut host1x_bo) -> *mut host1x_bo {
    let cmdbuf = container_of!(host_bo, tegra_se_cmdbuf, bo);
    kref_get(&mut (*cmdbuf).ref_);
    host_bo
}

unsafe fn tegra_se_cmdbuf_release(ref_: *mut kref) {
    let cmdbuf = container_of!(ref_, tegra_se_cmdbuf, ref_);
    dma_free_attrs((*cmdbuf).dev, (*cmdbuf).size, (*cmdbuf).addr,
                   (*cmdbuf).iova, 0);
    kfree(cmdbuf);
}

unsafe fn tegra_se_cmdbuf_put(host_bo: *mut host1x_bo) {
    let cmdbuf = container_of!(host_bo, tegra_se_cmdbuf, bo);
    kref_put(&mut (*cmdbuf).ref_, tegra_se_cmdbuf_release);
}

unsafe fn tegra_se_cmdbuf_pin(dev: *mut device, bo: *mut host1x_bo,
                              direction: dma_data_direction) -> *mut host1x_bo_mapping {
    let cmdbuf = container_of!(bo, tegra_se_cmdbuf, bo);
    let map = kzalloc::<host1x_bo_mapping>();
    if map.is_null() { return ERR_PTR(-ENOMEM); }
    kref_init(&mut (*map).ref_);
    (*map).bo = bo;
    (*map).direction = direction;
    (*map).dev = dev;
    (*map).sgt = kzalloc::<sg_table>();
    if (*map).sgt.is_null() { kfree(map); return ERR_PTR(-ENOMEM); }
    let mut err = dma_get_sgtable(dev, (*map).sgt, (*cmdbuf).addr,
                                  (*cmdbuf).iova, (*cmdbuf).words * 4);
    if err != 0 { sg_free_table((*map).sgt); kfree((*map).sgt); kfree(map); return ERR_PTR(err); }
    err = dma_map_sgtable(dev, (*map).sgt, direction, 0);
    if err != 0 { sg_free_table((*map).sgt); kfree((*map).sgt); kfree(map); return ERR_PTR(err); }
    (*map).phys = sg_dma_address((*map).sgt).sgl;
    (*map).size = (*cmdbuf).words * 4;
    (*map).chunks = err;
    map
}

unsafe fn tegra_se_cmdbuf_unpin(map: *mut host1x_bo_mapping) {
    if map.is_null() { return; }
    dma_unmap_sgtable((*map).dev, (*map).sgt, (*map).direction, 0);
    sg_free_table((*map).sgt);
    kfree((*map).sgt);
    kfree(map);
}

unsafe fn tegra_se_cmdbuf_mmap(host_bo: *mut host1x_bo) -> *mut core::ffi::c_void {
    let cmdbuf = container_of!(host_bo, tegra_se_cmdbuf, bo);
    (*cmdbuf).addr
}

unsafe fn tegra_se_cmdbuf_munmap(_host_bo: *mut host1x_bo, _addr: *mut core::ffi::c_void) {}

static tegra_se_cmdbuf_ops: host1x_bo_ops = host1x_bo_ops {
    get: Some(tegra_se_cmdbuf_get), put: Some(tegra_se_cmdbuf_put),
    pin: Some(tegra_se_cmdbuf_pin), unpin: Some(tegra_se_cmdbuf_unpin),
    mmap: Some(tegra_se_cmdbuf_mmap), munmap: Some(tegra_se_cmdbuf_munmap),
};

unsafe fn tegra_se_host1x_bo_alloc(se: *mut tegra_se, size: ssize_t) -> *mut tegra_se_cmdbuf {
    let dev = (*(*se).dev).parent;
    let cmdbuf = kzalloc::<tegra_se_cmdbuf>();
    if cmdbuf.is_null() { return core::ptr::null_mut(); }
    (*cmdbuf).addr = dma_alloc_attrs(dev, size, &mut (*cmdbuf).iova, GFP_KERNEL, 0);
    if (*cmdbuf).addr.is_null() { return core::ptr::null_mut(); }
    (*cmdbuf).size = size; (*cmdbuf).dev = dev;
    host1x_bo_init(&mut (*cmdbuf).bo, &tegra_se_cmdbuf_ops);
    kref_init(&mut (*cmdbuf).ref_);
    cmdbuf
}

pub unsafe fn tegra_se_host1x_submit(se: *mut tegra_se, cmdbuf: *mut tegra_se_cmdbuf, size: u32) -> i32 {
    let job = host1x_job_alloc((*se).channel, 1, 0, true);
    if job.is_null() { dev_err((*se).dev, "failed to allocate host1x job\n"); return -ENOMEM; }
    (*job).syncpt = host1x_syncpt_get((*se).syncpt); (*job).syncpt_incrs = 1;
    (*job).client = &mut (*se).client; (*job).class = (*se).client.class;
    (*job).serialize = true; (*job).engine_fallback_streamid = (*se).stream_id;
    (*job).engine_streamid_offset = SE_STREAM_ID; (*cmdbuf).words = size;
    host1x_job_add_gather(job, &mut (*cmdbuf).bo, size, 0);
    let mut ret = host1x_job_pin(job, (*se).dev);
    if ret != 0 { dev_err((*se).dev, "failed to pin host1x job\n"); host1x_job_put(job); return ret; }
    ret = host1x_job_submit(job);
    if ret != 0 { dev_err((*se).dev, "failed to submit host1x job\n"); host1x_job_unpin(job); host1x_job_put(job); return ret; }
    ret = host1x_syncpt_wait((*job).syncpt, (*job).syncpt_end, MAX_SCHEDULE_TIMEOUT, core::ptr::null_mut());
    if ret != 0 { dev_err((*se).dev, "host1x job timed out\n"); host1x_job_put(job); return ret; }
    host1x_job_put(job); 0
}

unsafe fn tegra_se_client_init(client: *mut host1x_client) -> i32 {
    let se = container_of!(client, tegra_se, client); let mut ret;
    (*se).channel = host1x_channel_request(&mut (*se).client);
    if (*se).channel.is_null() { dev_err((*se).dev, "host1x channel map failed\n"); return -ENODEV; }
    (*se).syncpt = host1x_syncpt_request(&mut (*se).client, 0);
    if (*se).syncpt.is_null() { dev_err((*se).dev, "host1x syncpt allocation failed\n"); host1x_channel_put((*se).channel); return -EINVAL; }
    (*se).syncpt_id = host1x_syncpt_id((*se).syncpt);
    (*se).cmdbuf = tegra_se_host1x_bo_alloc(se, SZ_4K); if (*se).cmdbuf.is_null() { host1x_syncpt_put((*se).syncpt); host1x_channel_put((*se).channel); return -ENOMEM; }
    (*se).keybuf = tegra_se_host1x_bo_alloc(se, SZ_4K); if (*se).keybuf.is_null() { tegra_se_cmdbuf_put(&mut (*se).cmdbuf.bo); host1x_syncpt_put((*se).syncpt); host1x_channel_put((*se).channel); return -ENOMEM; }
    ret = ((*(*se).hw).init_alg)(se);
    if ret != 0 { dev_err((*se).dev, "failed to register algorithms\n"); tegra_se_cmdbuf_put(&mut (*se).keybuf.bo); tegra_se_cmdbuf_put(&mut (*se).cmdbuf.bo); host1x_syncpt_put((*se).syncpt); host1x_channel_put((*se).channel); return ret; }
    0
}

unsafe fn tegra_se_client_deinit(client: *mut host1x_client) -> i32 {
    let se = container_of!(client, tegra_se, client); ((*(*se).hw).deinit_alg)(se);
    tegra_se_cmdbuf_put(&mut (*se).cmdbuf.bo); host1x_syncpt_put((*se).syncpt); host1x_channel_put((*se).channel); 0
}

static tegra_se_client_ops: host1x_client_ops = host1x_client_ops { init: Some(tegra_se_client_init), exit: Some(tegra_se_client_deinit) };

unsafe fn tegra_se_host1x_register(se: *mut tegra_se) -> i32 {
    INIT_LIST_HEAD!(&mut (*se).client.list); (*se).client.dev = (*se).dev; (*se).client.ops = &tegra_se_client_ops; (*se).client.class = (*(*se).hw).host1x_class; (*se).client.num_syncpts = 1; host1x_client_register(&mut (*se).client); 0
}

unsafe fn tegra_se_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev; let se = devm_kzalloc::<tegra_se>(dev, GFP_KERNEL); if se.is_null() { return -ENOMEM; }
    (*se).dev = dev; (*se).owner = TEGRA_GPSE_ID; (*se).hw = device_get_match_data(dev); (*se).base = devm_platform_ioremap_resource(pdev, 0); if IS_ERR((*se).base) { return PTR_ERR((*se).base); }
    dma_set_mask_and_coherent(dev, DMA_BIT_MASK(39)); platform_set_drvdata(pdev, se); (*se).clk = devm_clk_get_enabled(dev, core::ptr::null()); if IS_ERR((*se).clk) { return dev_err_probe(dev, PTR_ERR((*se).clk), "failed to enable clocks\n"); }
    if tegra_dev_iommu_get_stream_id(dev, &mut (*se).stream_id) == 0 { return dev_err_probe(dev, -ENODEV, "failed to get IOMMU stream ID\n"); }
    writel((*se).stream_id, (*se).base.add(SE_STREAM_ID)); (*se).engine = crypto_engine_alloc_init(dev, 0); if (*se).engine.is_null() { return -ENOMEM; }
    let mut ret = crypto_engine_start((*se).engine); if ret != 0 { crypto_engine_exit((*se).engine); return dev_err_probe(dev, ret, "failed to start crypto engine\n"); }
    ret = tegra_se_host1x_register(se); if ret != 0 { crypto_engine_exit((*se).engine); return dev_err_probe(dev, ret, "failed to init host1x params\n"); } 0
}

unsafe fn tegra_se_remove(pdev: *mut platform_device) { let se = platform_get_drvdata(pdev); crypto_engine_exit((*se).engine); host1x_client_unregister(&mut (*se).client); }

static tegra234_aes1_regs: tegra_se_regs = tegra_se_regs { config: SE_AES1_CFG, op: SE_AES1_OPERATION, last_blk: SE_AES1_LAST_BLOCK, linear_ctr: SE_AES1_LINEAR_CTR, aad_len: SE_AES1_AAD_LEN, cryp_msg_len: SE_AES1_CRYPTO_MSG_LEN, manifest: SE_AES1_KEYMANIFEST, key_addr: SE_AES1_KEY_ADDR, key_data: SE_AES1_KEY_DATA, key_dst: SE_AES1_KEY_DST, result: SE_AES1_CMAC_RESULT };
static tegra234_hash_regs: tegra_se_regs = tegra_se_regs { config: SE_SHA_CFG, op: SE_SHA_OPERATION, manifest: SE_SHA_KEYMANIFEST, key_addr: SE_SHA_KEY_ADDR, key_data: SE_SHA_KEY_DATA, key_dst: SE_SHA_KEY_DST, result: SE_SHA_HASH_RESULT };
static tegra234_aes_hw: tegra_se_hw = tegra_se_hw { regs: &tegra234_aes1_regs, kac_ver: 1, host1x_class: 0x3b, init_alg: tegra_init_aes, deinit_alg: tegra_deinit_aes };
static tegra234_hash_hw: tegra_se_hw = tegra_se_hw { regs: &tegra234_hash_regs, kac_ver: 1, host1x_class: 0x3d, init_alg: tegra_init_hash, deinit_alg: tegra_deinit_hash };

static tegra_se_of_match: [of_device_id; 3] = [
    of_device_id { compatible: "nvidia,tegra234-se-aes", data: &tegra234_aes_hw },
    of_device_id { compatible: "nvidia,tegra234-se-hash", data: &tegra234_hash_hw }, of_device_id { }
];

static tegra_se_driver: platform_driver = platform_driver { driver: driver { name: "tegra-se", of_match_table: &tegra_se_of_match }, probe: Some(tegra_se_probe), remove: Some(tegra_se_remove) };
unsafe fn tegra_se_host1x_probe(dev: *mut host1x_device) -> i32 { host1x_device_init(dev) }
unsafe fn tegra_se_host1x_remove(dev: *mut host1x_device) { host1x_device_exit(dev); }
static tegra_se_host1x_driver: host1x_driver = host1x_driver { driver: driver { name: "tegra-se-host1x" }, probe: Some(tegra_se_host1x_probe), remove: Some(tegra_se_host1x_remove), subdevs: &tegra_se_of_match };

unsafe fn tegra_se_module_init() -> i32 { let ret = host1x_driver_register(&tegra_se_host1x_driver); if ret != 0 { return ret; } platform_driver_register(&tegra_se_driver) }
unsafe fn tegra_se_module_exit() { host1x_driver_unregister(&tegra_se_host1x_driver); platform_driver_unregister(&tegra_se_driver); }

module_init!(tegra_se_module_init);
module_exit!(tegra_se_module_exit);
// MODULE_DESCRIPTION("NVIDIA Tegra Security Engine Driver");
// MODULE_AUTHOR("Akhil R <akhilrajeev@nvidia.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
