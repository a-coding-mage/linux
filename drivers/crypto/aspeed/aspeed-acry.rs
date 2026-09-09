// SPDX-License-Identifier: GPL-2.0+
/* Copyright 2021 Aspeed Technology Inc. */

// C dependencies supplied by the surrounding kernel translation.

const ASPEED_ACRY_TRIGGER: usize = 0x000;
const ASPEED_ACRY_DMA_CMD: usize = 0x048;
const ASPEED_ACRY_DMA_SRC_BASE: usize = 0x04c;
const ASPEED_ACRY_DMA_LEN: usize = 0x050;
const ASPEED_ACRY_RSA_KEY_LEN: usize = 0x058;
const ASPEED_ACRY_INT_MASK: usize = 0x3f8;
const ASPEED_ACRY_STATUS: usize = 0x3fc;
const ACRY_CMD_RSA_TRIGGER: u32 = 1 << 0;
const ACRY_CMD_DMA_RSA_TRIGGER: u32 = 1 << 1;
const ACRY_CMD_DMA_SRAM_MODE_RSA: u32 = 0x3 << 4;
const ACRY_CMD_DMEM_AHB: u32 = 1 << 8;
const ACRY_CMD_DMA_SRAM_AHB_ENGINE: u32 = 0;
const ACRY_RSA_ISR: u32 = 1 << 1;
const ASPEED_ACRY_BUFF_SIZE: usize = 0x1800;
const ASPEED_ACRY_SRAM_MAX_LEN: usize = 2048;
const ASPEED_ACRY_RSA_MAX_KEY_LEN: usize = 512;
const CRYPTO_FLAGS_BUSY: c_ulong = 1 << 1;
const BYTES_PER_DWORD: usize = 4;
const AHBC_REGION_PROT: usize = 0x240;
const REGION_ACRYM: u32 = 1 << 23;

#[repr(C)]
pub struct aspeed_acry_dev {
    pub regs: *mut core::ffi::c_void,
    pub dev: *mut device,
    pub irq: c_int,
    pub clk: *mut clk,
    pub ahbc: *mut regmap,
    pub req: *mut akcipher_request,
    pub done_task: tasklet_struct,
    pub resume: Option<unsafe extern "C" fn(*mut aspeed_acry_dev) -> c_int>,
    pub flags: c_ulong,
    pub acry_sram: *mut core::ffi::c_void,
    pub buf_addr: *mut core::ffi::c_void,
    pub buf_dma_addr: dma_addr_t,
    pub crypt_engine_rsa: *mut crypto_engine,
    pub exp_dw_mapping: [c_int; ASPEED_ACRY_RSA_MAX_KEY_LEN],
    pub mod_dw_mapping: [c_int; ASPEED_ACRY_RSA_MAX_KEY_LEN],
    pub data_byte_mapping: [c_int; ASPEED_ACRY_SRAM_MAX_LEN],
}

#[repr(C)]
pub struct aspeed_acry_ctx {
    pub acry_dev: *mut aspeed_acry_dev,
    pub key: rsa_key,
    pub enc: c_int,
    pub n: *mut u8,
    pub e: *mut u8,
    pub d: *mut u8,
    pub n_sz: usize,
    pub e_sz: usize,
    pub d_sz: usize,
    pub trigger: Option<unsafe extern "C" fn(*mut aspeed_acry_dev) -> c_int>,
    pub fallback_tfm: *mut crypto_akcipher,
}

#[repr(C)]
pub struct aspeed_acry_alg {
    pub acry_dev: *mut aspeed_acry_dev,
    pub akcipher: akcipher_engine_alg,
}

#[repr(C)]
pub enum aspeed_rsa_key_mode { ASPEED_RSA_EXP_MODE = 0, ASPEED_RSA_MOD_MODE, ASPEED_RSA_DATA_MODE }

unsafe fn ast_acry_write(a: *mut aspeed_acry_dev, val: u32, offset: usize) {
    core::ptr::write_volatile((*a).regs.cast::<u8>().add(offset).cast::<u32>(), val);
}
unsafe fn ast_acry_read(a: *mut aspeed_acry_dev, offset: usize) -> u32 {
    core::ptr::read_volatile((*a).regs.cast::<u8>().add(offset).cast::<u32>())
}

unsafe fn akcipher_request_cast(req: *mut crypto_async_request) -> *mut akcipher_request {
    (req.cast::<u8>().sub(core::mem::offset_of!(akcipher_request, base)))
        .cast::<akcipher_request>()
}

unsafe extern "C" fn aspeed_acry_do_fallback(req: *mut akcipher_request) -> c_int {
    let cipher = crypto_akcipher_reqtfm(req);
    let ctx = akcipher_tfm_ctx(cipher);
    akcipher_request_set_tfm(req, (*ctx).fallback_tfm);
    let err = if (*ctx).enc != 0 { crypto_akcipher_encrypt(req) } else { crypto_akcipher_decrypt(req) };
    akcipher_request_set_tfm(req, cipher);
    err
}

unsafe fn aspeed_acry_need_fallback(req: *mut akcipher_request) -> bool {
    let ctx = akcipher_tfm_ctx(crypto_akcipher_reqtfm(req));
    (*ctx).key.n_sz > ASPEED_ACRY_RSA_MAX_KEY_LEN
}

unsafe extern "C" fn aspeed_acry_handle_queue(a: *mut aspeed_acry_dev, req: *mut akcipher_request) -> c_int {
    if aspeed_acry_need_fallback(req) { return aspeed_acry_do_fallback(req); }
    crypto_transfer_akcipher_request_to_engine((*a).crypt_engine_rsa, req)
}

unsafe extern "C" fn aspeed_acry_do_request(engine: *mut crypto_engine, areq: *mut core::ffi::c_void) -> c_int {
    let req = akcipher_request_cast(areq.cast());
    let ctx = akcipher_tfm_ctx(crypto_akcipher_reqtfm(req));
    let a = (*ctx).acry_dev;
    (*a).req = req;
    (*a).flags |= CRYPTO_FLAGS_BUSY;
    ((*ctx).trigger.unwrap())(a)
}

unsafe extern "C" fn aspeed_acry_complete(a: *mut aspeed_acry_dev, err: c_int) -> c_int {
    (*a).flags &= !CRYPTO_FLAGS_BUSY;
    crypto_finalize_akcipher_request((*a).crypt_engine_rsa, (*a).req, err);
    err
}

unsafe fn aspeed_acry_rsa_sg_copy_to_buffer(a: *mut aspeed_acry_dev, buf: *mut u8, src: *mut scatterlist, nbytes: usize) {
    static mut DRAM_BUFFER: [u8; ASPEED_ACRY_SRAM_MAX_LEN] = [0; ASPEED_ACRY_SRAM_MAX_LEN];
    scatterwalk_map_and_copy(DRAM_BUFFER.as_mut_ptr().cast(), src, 0, nbytes, 0);
    let mut i = 0usize;
    for j in (0..nbytes).rev() { *buf.add((*a).data_byte_mapping[i] as usize) = DRAM_BUFFER[j]; i += 1; }
    while i < ASPEED_ACRY_SRAM_MAX_LEN { *buf.add((*a).data_byte_mapping[i] as usize) = 0; i += 1; }
}

unsafe extern "C" fn aspeed_acry_rsa_ctx_copy(a: *mut aspeed_acry_dev, buf: *mut core::ffi::c_void, xbuf: *const u8, mut nbytes: usize, mode: aspeed_rsa_key_mode) -> c_int {
    if nbytes > ASPEED_ACRY_RSA_MAX_KEY_LEN { return -ENOMEM; }
    let mut src = xbuf;
    while nbytes > 0 && *src == 0 { src = src.add(1); nbytes -= 1; }
    let mut nbits = (nbytes * 8) as c_int;
    if nbytes > 0 { nbits -= ((*src).leading_zeros() as c_int) - ((BITS_PER_LONG - 8) as c_int); }
    let ndw = (nbytes + 3) / 4;
    if nbytes > 0 {
        let mut i = (4 - nbytes % 4) % 4;
        let mut data: u32 = 0;
        for j in (1..=ndw).rev() {
            while i < 4 { data = (data << 8) | (*src as u32); src = src.add(1); i += 1; }
            i = 0;
            let idx = if mode == aspeed_rsa_key_mode::ASPEED_RSA_EXP_MODE { (*a).exp_dw_mapping[j - 1] } else { (*a).mod_dw_mapping[j - 1] };
            (buf.cast::<u32>().add(idx as usize)).write(data.to_le());
        }
    }
    nbits
}

unsafe extern "C" fn aspeed_acry_rsa_transfer(a: *mut aspeed_acry_dev) -> c_int {
    ast_acry_write(a, ACRY_CMD_DMEM_AHB, ASPEED_ACRY_DMA_CMD);
    regmap_update_bits((*a).ahbc, AHBC_REGION_PROT, REGION_ACRYM, 0);
    static mut DRAM_BUFFER: [u8; ASPEED_ACRY_SRAM_MAX_LEN] = [0; ASPEED_ACRY_SRAM_MAX_LEN];
    let mut result = ASPEED_ACRY_SRAM_MAX_LEN as c_int; let mut i = 0; let mut leading = true;
    for j in (0..ASPEED_ACRY_SRAM_MAX_LEN).rev() { let idx = (*a).data_byte_mapping[j] as usize; let v = core::ptr::read_volatile((*a).acry_sram.cast::<u8>().add(idx)); if v == 0 && leading { result -= 1; } else { leading = false; DRAM_BUFFER[i] = v; i += 1; } }
    let req = (*a).req;
    if result <= (*req).dst_len as c_int { scatterwalk_map_and_copy(DRAM_BUFFER.as_mut_ptr().cast(), (*req).dst, 0, result as usize, 1); (*req).dst_len = result as usize; }
    memzero_explicit((*a).buf_addr, ASPEED_ACRY_BUFF_SIZE);
    aspeed_acry_complete(a, 0)
}

unsafe extern "C" fn aspeed_acry_rsa_trigger(a: *mut aspeed_acry_dev) -> c_int {
    let req = (*a).req; let ctx = akcipher_tfm_ctx(crypto_akcipher_reqtfm(req));
    if (*ctx).n.is_null() || (*ctx).n_sz == 0 { return -EINVAL; }
    memzero_explicit((*a).buf_addr, ASPEED_ACRY_BUFF_SIZE);
    aspeed_acry_rsa_sg_copy_to_buffer(a, (*a).buf_addr.cast(), (*req).src, (*req).src_len);
    let nm = aspeed_acry_rsa_ctx_copy(a, (*a).buf_addr, (*ctx).n, (*ctx).n_sz, aspeed_rsa_key_mode::ASPEED_RSA_MOD_MODE);
    let (exp, exp_sz) = if (*ctx).enc != 0 { ((*ctx).e, (*ctx).e_sz) } else { ((*ctx).key.d.cast(), (*ctx).key.d_sz) };
    if exp.is_null() || exp_sz == 0 { return -EINVAL; }
    let ne = aspeed_acry_rsa_ctx_copy(a, (*a).buf_addr, exp, exp_sz, aspeed_rsa_key_mode::ASPEED_RSA_EXP_MODE);
    ast_acry_write(a, (*a).buf_dma_addr as u32, ASPEED_ACRY_DMA_SRC_BASE); ast_acry_write(a, ((ne << 16) + nm) as u32, ASPEED_ACRY_RSA_KEY_LEN); ast_acry_write(a, ASPEED_ACRY_BUFF_SIZE as u32, ASPEED_ACRY_DMA_LEN);
    (*a).resume = Some(aspeed_acry_rsa_transfer); regmap_update_bits((*a).ahbc, AHBC_REGION_PROT, REGION_ACRYM, REGION_ACRYM); ast_acry_write(a, ACRY_RSA_ISR, ASPEED_ACRY_INT_MASK); ast_acry_write(a, ACRY_CMD_DMA_SRAM_MODE_RSA, ASPEED_ACRY_DMA_CMD); ast_acry_write(a, ACRY_CMD_RSA_TRIGGER | ACRY_CMD_DMA_RSA_TRIGGER, ASPEED_ACRY_TRIGGER); 0
}

unsafe extern "C" fn aspeed_acry_rsa_enc(req: *mut akcipher_request) -> c_int { let ctx = akcipher_tfm_ctx(crypto_akcipher_reqtfm(req)); (*ctx).trigger = Some(aspeed_acry_rsa_trigger); (*ctx).enc = 1; aspeed_acry_handle_queue((*ctx).acry_dev, req) }
unsafe extern "C" fn aspeed_acry_rsa_dec(req: *mut akcipher_request) -> c_int { let ctx = akcipher_tfm_ctx(crypto_akcipher_reqtfm(req)); (*ctx).trigger = Some(aspeed_acry_rsa_trigger); (*ctx).enc = 0; aspeed_acry_handle_queue((*ctx).acry_dev, req) }

unsafe fn aspeed_rsa_key_copy(src: *mut u8, len: usize) -> *mut u8 { kmemdup(src.cast(), len, GFP_KERNEL).cast() }
unsafe fn aspeed_rsa_set_n(c: *mut aspeed_acry_ctx, v: *mut u8, l: usize) -> c_int { (*c).n_sz=l; (*c).n=aspeed_rsa_key_copy(v,l); if (*c).n.is_null(){-ENOMEM}else{0} }
unsafe fn aspeed_rsa_set_e(c: *mut aspeed_acry_ctx, v: *mut u8, l: usize) -> c_int { (*c).e_sz=l; (*c).e=aspeed_rsa_key_copy(v,l); if (*c).e.is_null(){-ENOMEM}else{0} }
unsafe fn aspeed_rsa_set_d(c: *mut aspeed_acry_ctx, v: *mut u8, l: usize) -> c_int { (*c).d_sz=l; (*c).d=aspeed_rsa_key_copy(v,l); if (*c).d.is_null(){-ENOMEM}else{0} }
unsafe fn aspeed_rsa_key_free(c: *mut aspeed_acry_ctx) { kfree_sensitive((*c).n.cast()); kfree_sensitive((*c).e.cast()); kfree_sensitive((*c).d.cast()); (*c).n_sz=0; (*c).e_sz=0; (*c).d_sz=0; }

unsafe extern "C" fn aspeed_acry_rsa_setkey(tfm: *mut crypto_akcipher, key: *const core::ffi::c_void, keylen: c_uint, privkey: c_int) -> c_int {
    let c=akcipher_tfm_ctx(tfm); let ret=if privkey!=0{rsa_parse_priv_key(&mut (*c).key,key,keylen)}else{rsa_parse_pub_key(&mut (*c).key,key,keylen)}; if ret!=0{return ret}; if (*c).key.n_sz>ASPEED_ACRY_RSA_MAX_KEY_LEN{return 0};
    let mut r=aspeed_rsa_set_n(c,(*c).key.n.cast(),(*c).key.n_sz); if r!=0{return r}; r=aspeed_rsa_set_e(c,(*c).key.e.cast(),(*c).key.e_sz); if r!=0{aspeed_rsa_key_free(c);return r}; if privkey!=0{r=aspeed_rsa_set_d(c,(*c).key.d.cast(),(*c).key.d_sz);if r!=0{aspeed_rsa_key_free(c);return r;}} 0
}
unsafe extern "C" fn aspeed_acry_rsa_set_pub_key(t:*mut crypto_akcipher,k:*const core::ffi::c_void,l:c_uint)->c_int{let c=akcipher_tfm_ctx(t);let r=crypto_akcipher_set_pub_key((*c).fallback_tfm,k,l);if r!=0{r}else{aspeed_acry_rsa_setkey(t,k,l,0)}}
unsafe extern "C" fn aspeed_acry_rsa_set_priv_key(t:*mut crypto_akcipher,k:*const core::ffi::c_void,l:c_uint)->c_int{let c=akcipher_tfm_ctx(t);let r=crypto_akcipher_set_priv_key((*c).fallback_tfm,k,l);if r!=0{r}else{aspeed_acry_rsa_setkey(t,k,l,1)}}
unsafe extern "C" fn aspeed_acry_rsa_max_size(t:*mut crypto_akcipher)->c_uint{let c=akcipher_tfm_ctx(t);if (*c).key.n_sz>ASPEED_ACRY_RSA_MAX_KEY_LEN{crypto_akcipher_maxsize((*c).fallback_tfm)}else{(*c).n_sz as c_uint}}
unsafe extern "C" fn aspeed_acry_rsa_init_tfm(t:*mut crypto_akcipher)->c_int{let c=akcipher_tfm_ctx(t);(*c).fallback_tfm=crypto_alloc_akcipher(crypto_tfm_alg_name(&mut (*t).base),0,CRYPTO_ALG_ASYNC|CRYPTO_ALG_NEED_FALLBACK);if (*c).fallback_tfm.is_null(){-ENOMEM}else{0}}
unsafe extern "C" fn aspeed_acry_rsa_exit_tfm(t:*mut crypto_akcipher){crypto_free_akcipher((*akcipher_tfm_ctx(t)).fallback_tfm)}

static mut ASPEED_ACRY_AKCIPHER_ALGS: [aspeed_acry_alg; 1] = [unsafe { core::mem::zeroed() }];
unsafe fn aspeed_acry_register(a:*mut aspeed_acry_dev){for alg in ASPEED_ACRY_AKCIPHER_ALGS.iter_mut(){alg.acry_dev=a;crypto_engine_register_akcipher(&mut alg.akcipher);}}
unsafe fn aspeed_acry_unregister(_: *mut aspeed_acry_dev){for alg in ASPEED_ACRY_AKCIPHER_ALGS.iter_mut(){crypto_engine_unregister_akcipher(&mut alg.akcipher);}}
unsafe extern "C" fn aspeed_acry_irq(_:c_int,dev:*mut core::ffi::c_void)->irqreturn_t{let a=dev.cast::<aspeed_acry_dev>();let sts=ast_acry_read(a,ASPEED_ACRY_STATUS);ast_acry_write(a,sts,ASPEED_ACRY_STATUS);if sts&ACRY_RSA_ISR!=0{ast_acry_write(a,0,ASPEED_ACRY_TRIGGER);if (*a).flags&CRYPTO_FLAGS_BUSY!=0{tasklet_schedule(&mut (*a).done_task);}} IRQ_HANDLED}
unsafe fn aspeed_acry_sram_mapping(a:*mut aspeed_acry_dev){let mut j=0;for i in 0..ASPEED_ACRY_SRAM_MAX_LEN/4{(*a).exp_dw_mapping[i]=j;(*a).mod_dw_mapping[i]=j+4;for k in 0..4{(*a).data_byte_mapping[i*4+k]=(j+8)*4+k as c_int;}j+=1;if j%4==0{j+=8;}}}
unsafe extern "C" fn aspeed_acry_done_task(data: c_ulong){let a=data as *mut aspeed_acry_dev;((*a).resume.unwrap())(a);}

// Device-tree match table, platform probe/remove, platform-driver registration, and module metadata.
#[no_mangle] pub static mut aspeed_acry_of_matches: [of_device_id; 2] = [of_device_id { compatible: b"aspeed,ast2600-acry\0".as_ptr().cast() }, of_device_id { compatible: core::ptr::null() }];
#[no_mangle] pub static mut aspeed_acry_driver: platform_driver = unsafe { core::mem::zeroed() };

unsafe extern "C" fn aspeed_acry_probe(pdev:*mut platform_device)->c_int{
    let dev=&mut (*pdev).dev; let a=devm_kzalloc(dev,core::mem::size_of::<aspeed_acry_dev>(),GFP_KERNEL).cast::<aspeed_acry_dev>(); if a.is_null(){return -ENOMEM}; (*a).dev=dev; platform_set_drvdata(pdev,a.cast());
    (*a).regs=devm_platform_ioremap_resource(pdev,0); if (*a).regs.is_null(){return -ENODEV}; (*a).acry_sram=devm_platform_ioremap_resource(pdev,1); if (*a).acry_sram.is_null(){return -ENODEV}; (*a).irq=platform_get_irq(pdev,0); if (*a).irq<0{return (*a).irq};
    let r=devm_request_irq(dev,(*a).irq,Some(aspeed_acry_irq),0,dev_name(dev),a.cast());if r!=0{return r};(*a).ahbc=syscon_regmap_lookup_by_phandle((*dev).of_node,b"aspeed,ahbc\0".as_ptr().cast());if (*a).ahbc.is_null(){return -ENODEV};(*a).crypt_engine_rsa=crypto_engine_alloc_init(dev,true);if (*a).crypt_engine_rsa.is_null(){return -ENOMEM};let r=crypto_engine_start((*a).crypt_engine_rsa);if r!=0{return r};tasklet_init(&mut (*a).done_task,Some(aspeed_acry_done_task),a as c_ulong);ast_acry_write(a,ACRY_CMD_DMEM_AHB,ASPEED_ACRY_DMA_CMD);aspeed_acry_sram_mapping(a);(*a).buf_addr=dmam_alloc_coherent(dev,ASPEED_ACRY_BUFF_SIZE,&mut (*a).buf_dma_addr,GFP_KERNEL);if (*a).buf_addr.is_null(){return -ENOMEM};aspeed_acry_register(a);0
}
unsafe extern "C" fn aspeed_acry_remove(pdev:*mut platform_device){let a=platform_get_drvdata(pdev).cast::<aspeed_acry_dev>();aspeed_acry_unregister(a);crypto_engine_exit((*a).crypt_engine_rsa);tasklet_kill(&mut (*a).done_task);}

// MODULE_DEVICE_TABLE(of, aspeed_acry_of_matches); module_platform_driver(aspeed_acry_driver);
// MODULE_AUTHOR("Neal Liu <neal_liu@aspeedtech.com>");
// MODULE_DESCRIPTION("ASPEED ACRY driver for hardware RSA Engine");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
