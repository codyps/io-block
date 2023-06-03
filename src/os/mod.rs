use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
        pub use linux::*;
    } else if #[cfg(target_vendor = "apple")] {
        pub mod darwin;
        pub use darwin::*;
    } else {
        compile_error!("Unsupported OS");
    }
}
