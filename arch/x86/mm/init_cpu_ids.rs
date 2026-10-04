// SPDX-License-Identifier: GPL-2.0-only
// X86_MATCH_VFM initializer policy over the configured native struct layout.
const fn invlpg_id(
    vendor: u16,
    family: u16,
    model: u16,
    minimum_microcode: kernel_ulong_t,
) -> x86_cpu_id {
    x86_cpu_id {
        vendor,
        family,
        model,
        steppings: b::RUST_MM_X86_STEPPING_ANY as u16,
        feature: b::RUST_MM_X86_FEATURE_ANY as u16,
        flags: b::RUST_MM_X86_CPU_ID_FLAG_ENTRY_VALID as u16,
        platform_mask: 0,
        type_: b::RUST_MM_X86_CPU_TYPE_ANY as u8,
        driver_data: minimum_microcode,
    }
}
static invlpg_miss_ids: [x86_cpu_id; 7] = [
    invlpg_id(
        b::RUST_MM_ALDERLAKE_VENDOR as u16,
        b::RUST_MM_ALDERLAKE_FAMILY as u16,
        b::RUST_MM_ALDERLAKE_MODEL as u16,
        0x2e,
    ),
    invlpg_id(
        b::RUST_MM_ALDERLAKE_L_VENDOR as u16,
        b::RUST_MM_ALDERLAKE_L_FAMILY as u16,
        b::RUST_MM_ALDERLAKE_L_MODEL as u16,
        0x42c,
    ),
    invlpg_id(
        b::RUST_MM_ATOM_GRACEMONT_VENDOR as u16,
        b::RUST_MM_ATOM_GRACEMONT_FAMILY as u16,
        b::RUST_MM_ATOM_GRACEMONT_MODEL as u16,
        0x11,
    ),
    invlpg_id(
        b::RUST_MM_RAPTORLAKE_VENDOR as u16,
        b::RUST_MM_RAPTORLAKE_FAMILY as u16,
        b::RUST_MM_RAPTORLAKE_MODEL as u16,
        0x118,
    ),
    invlpg_id(
        b::RUST_MM_RAPTORLAKE_P_VENDOR as u16,
        b::RUST_MM_RAPTORLAKE_P_FAMILY as u16,
        b::RUST_MM_RAPTORLAKE_P_MODEL as u16,
        0x4117,
    ),
    invlpg_id(
        b::RUST_MM_RAPTORLAKE_S_VENDOR as u16,
        b::RUST_MM_RAPTORLAKE_S_FAMILY as u16,
        b::RUST_MM_RAPTORLAKE_S_MODEL as u16,
        0x2e,
    ),
    unsafe { MaybeUninit::zeroed().assume_init() },
];
