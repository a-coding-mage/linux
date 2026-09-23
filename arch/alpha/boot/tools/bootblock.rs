// SPDX-License-Identifier: GPL-2.0
//! Alpha SRM boot-block encoding, independent of the build host's layout.

pub(crate) const BLOCK_SIZE: usize = 512;

pub(crate) fn checksum(block: &mut [u8; BLOCK_SIZE]) {
    let sum = block[..504].chunks_exact(8).fold(0u64, |sum, word| {
        sum.wrapping_add(u64::from_le_bytes(word.try_into().unwrap()))
    });
    block[504..].copy_from_slice(&sum.to_le_bytes());
}

pub(crate) fn perror(output: &mut Vec<u8>, name: &[u8], error: Option<&std::io::Error>) {
    output.extend_from_slice(name);
    output.extend_from_slice(b": ");
    if let Some(error) = error {
        let message = error.to_string();
        output.extend_from_slice(
            message
                .split(" (os error ")
                .next()
                .unwrap_or(&message)
                .as_bytes(),
        );
    } else {
        // The C tools call perror after a short successful read, with errno=0.
        output.extend_from_slice(b"Success");
    }
    output.push(b'\n');
}
