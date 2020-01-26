#[cfg(any(target_os = "linux", target_os = "window"))]
pub type Arg = ();
#[cfg(target_os = "android")]
pub type Arg = &mut i32;

#[cfg(any(target_os = "linux", target_os = "window"))]
#[macro_export]
macro_rules! main {
    ($main_function:ident) => {
        fn main() {
            $main_function(());
        }
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! main {
    ($main_function:ident) => {
        #[allow(dead_code)]
        #[no_mangle]
        extern "C" fn android_main(android_app: *mut std::os::raw::c_void) {
            $main_function();
        }
    };
}
