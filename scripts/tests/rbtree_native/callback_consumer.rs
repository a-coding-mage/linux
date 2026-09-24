//! Independent actual-bindgen versus corrected nullable rbtree consumer.
use kernel::bindings::{rb_node, rb_root, RbAugmentRotate};
type RawRotate = Option<unsafe extern "C" fn(*mut rb_node, *mut rb_node)>;
// bindgen declarations are intentionally private inputs to the negative control.
#[allow(dead_code, unused_imports, unreachable_pub)]
mod raw {
    use kernel::{bindings::{rb_node, rb_root}, ffi};
    include!(env!("RBTREE_RAW_BINDINGS"));
}
#[allow(dead_code, unused_imports, unreachable_pub)]
mod raw_original {
    use kernel::{bindings::{rb_node, rb_root}, ffi};
    include!(env!("RBTREE_RAW_ORIGINAL_BINDINGS"));
}
unsafe extern "C" {
    fn callback_first(callback: RawRotate);
    #[link_name="oracle___rb_insert_augmented"]
    fn original_insert(node:*mut rb_node,root:*mut rb_root,callback:RbAugmentRotate);
    #[link_name="oracle___rb_erase_color"]
    fn original_erase(node:*mut rb_node,root:*mut rb_root,callback:RbAugmentRotate);
}
/// Actual public binding caller with a genuinely nullable callback.
///
/// # Safety
/// Fixture supplies live original-layout nodes and exact operation domain.
#[no_mangle]
pub unsafe extern "C" fn rust_call(original: bool, erase: bool, wrong: bool,
    node:*mut rb_node,root:*mut rb_root,callback:RawRotate) {
    if wrong {
        let f=match (original,erase) {
            (true,false)=>raw_original::__rb_insert_augmented,
            (true,true)=>raw_original::__rb_erase_color,
            (false,false)=>raw::__rb_insert_augmented,
            (false,true)=>raw::__rb_erase_color,
        };
        // SAFETY: Ordinary ABI only; deliberately wrong nominal outer KCFI.
        let f=unsafe { core::ptr::read_volatile(&f) };
        unsafe { f(node,root,callback) };
    } else {
        let f=match (original,erase) {
            (true,false)=>original_insert,
            (true,true)=>original_erase,
            (false,false)=>kernel::bindings::__rb_insert_augmented,
            (false,true)=>kernel::bindings::__rb_erase_color,
        };
        // SAFETY: Exact native public declaration; volatile prevents devirtualization.
        let f=unsafe { core::ptr::read_volatile(&f) };
        unsafe { f(node,root,RbAugmentRotate::from_option(callback)) };
    }
}
/// Deliberate wrong enclosing-signature control; ordinary ABI remains compatible.
///
/// # Safety
/// Protected execution must trap; unprotected callback_first accepts NULL.
#[no_mangle]
pub unsafe extern "C" fn rust_wrong_context() {
    let f:unsafe extern "C" fn(RawRotate)=callback_first;
    // SAFETY: Deliberate nominal CFI mismatch with identical machine ABI.
    let f:unsafe extern "C" fn(RbAugmentRotate)=unsafe { core::mem::transmute(f) };
    let f=unsafe { core::ptr::read_volatile(&f) };
    unsafe { f(RbAugmentRotate::from_option(None)) };
}
/// Exact layout comparison to original C typedef and node/root structures.
#[no_mangle]
pub static rust_layout:[usize;6]=[
    core::mem::size_of::<RbAugmentRotate>(),core::mem::align_of::<RbAugmentRotate>(),
    core::mem::size_of::<rb_node>(),core::mem::align_of::<rb_node>(),
    core::mem::size_of::<rb_root>(),core::mem::align_of::<rb_root>(),
];
