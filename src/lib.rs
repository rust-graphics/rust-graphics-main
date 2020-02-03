#[cfg(target_os = "android")]
pub extern crate rust_graphics_android as android;

#[cfg(target_arch = "wasm32")]
pub extern crate wasm_bindgen;

#[cfg(target_os = "android")]
pub type Arg = &'static mut android::glue::AndroidApp;

#[cfg(any(target_os = "linux", target_os = "window", target_arch = "wasm32"))]
pub type Arg = ();

#[macro_export]
macro_rules! main {
    ($main_function:ident) => {
        #[cfg(any(target_os = "linux", target_os = "window"))]
        fn main() {
            $main_function(());
        }

        #[cfg(target_os = "android")]
        #[allow(dead_code)]
        #[no_mangle]
        extern "C" fn android_main(android_app: &'static mut $crate::android::glue::AndroidApp) {
            $main_function(android_app);
        }

        #[cfg(target_arch = "wasm32")]
        pub use $crate::{wasm_bindgen, wasm_bindgen::prelude::*};

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen(start)]
        pub fn rust_graphics_main_entry() -> Result<(), JsValue> {
            $main_function(());
            Ok(())
        }
    };
}
