use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
        pub(crate) use linux::Disk;
    } else if #[cfg(target_vendor = "apple")] {
        pub mod darwin;
        pub(crate) use darwin::Disk;
    } else {
        compile_error!("Unsupported OS");
    }

    // dragonflybsd: `getdisktabbyname`
    // freebsd: `getdiskbyname`
    // windows: either something in Virtual Disk Service or `DeviceIoControl` with
    // `IOCTL_DISK_GET_DRIVE_GEOMETRY` or Windows storage managment api
}
