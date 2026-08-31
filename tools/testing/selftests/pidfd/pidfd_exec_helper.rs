// C source defined _GNU_SOURCE and included stdio.h, stdlib.h, and unistd.h.

const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

unsafe extern "C" {
    fn pause() -> i32;
    fn _exit(status: i32) -> !;
}

fn main() {
    let argc: i32 = std::env::args().count() as i32;
    let argv: Vec<*mut i8> = Vec::new();
    let _ = (argc, argv);

    unsafe {
        if pause() != 0 {
            _exit(EXIT_FAILURE);
        }

        _exit(EXIT_SUCCESS);
    }
}
