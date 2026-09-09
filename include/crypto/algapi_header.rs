/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Cryptographic API for algorithms (low-level API). */

/* Dependencies supplied by the surrounding kernel translation. */

pub const MAX_ALGAPI_BLOCKSIZE: usize = 160;
pub const MAX_ALGAPI_ALIGNMASK: usize = 127;
pub const MAX_CIPHER_BLOCKSIZE: usize = 16;
pub const MAX_CIPHER_ALIGNMASK: usize = 15;

/* ARCH_DMA_MINALIGN selects CRYPTO_DMA_ALIGN when supplied by the build. */
pub const CRYPTO_DMA_ALIGN: usize = CRYPTO_MINALIGN;
pub const CRYPTO_DMA_PADDING: usize =
    (CRYPTO_DMA_ALIGN - 1) & !(CRYPTO_MINALIGN - 1);

pub enum crypto_aead {}
pub enum crypto_instance_forward {}
pub enum module {}
pub enum notifier_block {}
pub enum rtattr {}
pub enum scatterlist {}
pub enum seq_file {}
pub enum sk_buff {}
pub enum crypto_no_such_thing {}

#[repr(C)]
pub union crypto_instance_link {
    pub list: hlist_node,
    pub spawns: *mut crypto_spawn,
}

#[repr(C)]
pub struct crypto_instance {
    pub alg: crypto_alg,
    pub tmpl: *mut crypto_template,
    pub link: crypto_instance_link,
    pub __ctx: [u8; 0],
}

#[repr(C)]
pub struct crypto_template {
    pub list: list_head,
    pub instances: hlist_head,
    pub dead: hlist_head,
    pub module: *mut module,
    pub free_work: work_struct,
    pub create: Option<unsafe extern "C" fn(*mut crypto_template, *mut *mut rtattr) -> i32>,
    pub name: [c_char; CRYPTO_MAX_ALG_NAME],
}

#[repr(C)]
pub union crypto_spawn_link {
    pub inst: *mut crypto_instance,
    pub next: *mut crypto_spawn,
}

#[repr(C)]
pub struct crypto_spawn {
    pub list: list_head,
    pub alg: *mut crypto_alg,
    pub link: crypto_spawn_link,
    pub frontend: *const crypto_type,
    pub mask: u32,
    pub dead: bool,
    pub registered: bool,
}

#[repr(C)]
pub struct crypto_queue {
    pub list: list_head,
    pub backlog: *mut list_head,
    pub qlen: c_uint,
    pub max_qlen: c_uint,
}

#[repr(C)]
pub union scatter_walk_addr {
    pub addr: *mut core::ffi::c_void,
    pub __addr: *mut crypto_no_such_thing,
}

#[repr(C)]
pub struct scatter_walk {
    pub addr: scatter_walk_addr,
    pub sg: *mut scatterlist,
    pub offset: c_uint,
}

#[repr(C)]
pub struct crypto_attr_alg {
    pub name: [c_char; CRYPTO_MAX_ALG_NAME],
}

#[repr(C)]
pub struct crypto_attr_type {
    pub type_: u32,
    pub mask: u32,
}

extern "C" {
    pub fn crypto_register_alg(alg: *mut crypto_alg) -> i32;
    pub fn crypto_unregister_alg(alg: *mut crypto_alg);
    pub fn crypto_register_algs(algs: *mut crypto_alg, count: i32) -> i32;
    pub fn crypto_unregister_algs(algs: *mut crypto_alg, count: i32);
    pub fn crypto_mod_put(alg: *mut crypto_alg);
    pub fn crypto_register_template(tmpl: *mut crypto_template) -> i32;
    pub fn crypto_register_templates(tmpls: *mut crypto_template, count: i32) -> i32;
    pub fn crypto_unregister_template(tmpl: *mut crypto_template);
    pub fn crypto_unregister_templates(tmpls: *mut crypto_template, count: i32);
    pub fn crypto_lookup_template(name: *const c_char) -> *mut crypto_template;
    pub fn crypto_register_instance(tmpl: *mut crypto_template, inst: *mut crypto_instance) -> i32;
    pub fn crypto_unregister_instance(inst: *mut crypto_instance);
    pub fn crypto_grab_spawn(spawn: *mut crypto_spawn, inst: *mut crypto_instance, name: *const c_char, type_: u32, mask: u32) -> i32;
    pub fn crypto_drop_spawn(spawn: *mut crypto_spawn);
    pub fn crypto_spawn_tfm(spawn: *mut crypto_spawn, type_: u32, mask: u32) -> *mut crypto_tfm;
    pub fn crypto_spawn_tfm2(spawn: *mut crypto_spawn) -> *mut core::ffi::c_void;
    pub fn crypto_get_attr_type(tb: *mut *mut rtattr) -> *mut crypto_attr_type;
    pub fn crypto_check_attr_type(tb: *mut *mut rtattr, type_: u32, mask_ret: *mut u32) -> i32;
    pub fn crypto_attr_alg_name(rta: *mut rtattr) -> *const c_char;
    pub fn __crypto_inst_setname(inst: *mut crypto_instance, name: *const c_char, driver: *const c_char, alg: *mut crypto_alg) -> i32;
    pub fn crypto_init_queue(queue: *mut crypto_queue, max_qlen: c_uint);
    pub fn crypto_enqueue_request(queue: *mut crypto_queue, request: *mut crypto_async_request) -> i32;
    pub fn crypto_enqueue_request_head(queue: *mut crypto_queue, request: *mut crypto_async_request);
    pub fn crypto_dequeue_request(queue: *mut crypto_queue) -> *mut crypto_async_request;
    pub fn crypto_inc(a: *mut u8, size: c_uint);
    pub fn crypto_register_notifier(nb: *mut notifier_block) -> i32;
    pub fn crypto_unregister_notifier(nb: *mut notifier_block) -> i32;
}

pub const CRYPTO_MSG_ALG_REQUEST: u32 = 0;
pub const CRYPTO_MSG_ALG_REGISTER: u32 = 1;
pub const CRYPTO_MSG_ALG_LOADED: u32 = 2;
pub const CRYPTO_ALG_INHERITED_FLAGS: u32 =
    CRYPTO_ALG_ASYNC | CRYPTO_ALG_NEED_FALLBACK | CRYPTO_ALG_ALLOCATES_MEMORY;

#[inline]
pub unsafe fn crypto_queue_len(queue: *mut crypto_queue) -> c_uint { (*queue).qlen }

#[inline]
pub unsafe fn crypto_tfm_ctx(tfm: *mut crypto_tfm) -> *mut core::ffi::c_void { (*tfm).__crt_ctx }

#[inline]
pub unsafe fn crypto_tfm_ctx_align(tfm: *mut crypto_tfm, mut align: c_uint) -> *mut core::ffi::c_void {
    if align <= crypto_tfm_ctx_alignment() { align = 1; }
    PTR_ALIGN(crypto_tfm_ctx(tfm), align)
}

#[inline]
pub unsafe fn crypto_dma_align() -> c_uint { CRYPTO_DMA_ALIGN as c_uint }

#[inline]
pub unsafe fn crypto_dma_padding() -> c_uint {
    (crypto_dma_align() - 1) & !(crypto_tfm_ctx_alignment() - 1)
}

#[inline]
pub unsafe fn crypto_tfm_ctx_dma(tfm: *mut crypto_tfm) -> *mut core::ffi::c_void {
    crypto_tfm_ctx_align(tfm, crypto_dma_align())
}

#[inline]
pub unsafe fn crypto_instance_ctx(inst: *mut crypto_instance) -> *mut u8 { (*inst).__ctx.as_mut_ptr() }

#[inline]
pub unsafe fn crypto_requires_off(algt: *mut crypto_attr_type, off: u32) -> u32 {
    ((*algt).type_ ^ off) & (*algt).mask & off
}

#[inline]
pub unsafe fn crypto_algt_inherited_mask(algt: *mut crypto_attr_type) -> u32 {
    crypto_requires_off(algt, CRYPTO_ALG_INHERITED_FLAGS)
}

#[inline]
pub unsafe fn crypto_request_complete(req: *mut crypto_async_request, err: i32) {
    ((*req).complete)(*req, err);
}

#[inline]
pub unsafe fn crypto_tfm_alg_type(tfm: *mut crypto_tfm) -> u32 {
    (*(*tfm).__crt_alg).cra_flags & CRYPTO_ALG_TYPE_MASK
}

#[inline]
pub unsafe fn crypto_tfm_req_virt(tfm: *mut crypto_tfm) -> bool {
    ((*(*tfm).__crt_alg).cra_flags & CRYPTO_ALG_REQ_VIRT) != 0
}

#[inline]
pub unsafe fn crypto_request_flags(req: *mut crypto_async_request) -> u32 {
    (*req).flags & !CRYPTO_TFM_REQ_ON_STACK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
