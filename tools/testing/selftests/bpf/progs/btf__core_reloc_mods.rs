// Depends on definitions from "core_reloc_types.h".

#[no_mangle]
pub extern "C" fn f(x: core_reloc_mods) {
    let _ = x;
}
