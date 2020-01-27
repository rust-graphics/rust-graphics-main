#[cfg(any(target_os = "linux", target_os = "window"))]
pub type Arg = ();

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
pub extern crate rust_graphics_android as android;

#[cfg(target_os = "android")]
pub type Arg = &'static mut android::glue::AndroidApp;

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! main {
    ($main_function:ident) => {
        #[allow(dead_code)]
        #[no_mangle]
        extern "C" fn android_main(android_app: &'static mut $crate::android::glue::AndroidApp) {
            $main_function(android_app);
        }
    };
}
