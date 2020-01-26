#[cfg(any(target_os = "linux", target_os = "window"))]
#[macro_export]
macro_rules! main {
    ($main_block:block) => {
        fn main() {
            $main_block
        }
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! main {
    ($main_block:block) => {
        #[allow(dead_code)]
        #[no_mangle]
        extern "C" fn android_main(android_app: *mut std::os::raw::c_void) {
            $main_block
        }
    };
}
