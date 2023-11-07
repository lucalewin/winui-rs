mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    #![link(name = "Microsoft.WindowsAppRuntime.Bootstrap")]

    include!(concat!(env!("OUT_DIR"), "/bootstrap.rs"));
}

const RELEASE_MAJOR_MINOR: u32 = 0x00010004;
const VERSION_TAG: ffi::PCWSTR = "".as_ptr() as *const u16;
const RUNTIME_VERSION: u64 = 0x0FA003DA02630000;
const PACKAGE_VERSION: ffi::PACKAGE_VERSION = ffi::PACKAGE_VERSION {
    __bindgen_anon_1: ffi::PACKAGE_VERSION__bindgen_ty_1 {
        Version: RUNTIME_VERSION,
    },
};
const BOOTSTRAP_OPTIONS: ffi::MddBootstrapInitializeOptions =
    ffi::MddBootstrapInitializeOptions_MddBootstrapInitializeOptions_OnNoMatch_ShowUI;

pub fn initialize() -> std::io::Result<()> {
    unsafe {
        ffi::MddBootstrapInitialize2(
            RELEASE_MAJOR_MINOR,
            VERSION_TAG,
            PACKAGE_VERSION,
            BOOTSTRAP_OPTIONS,
        )
    };

    Ok(())
}

pub fn shutdown() -> std::io::Result<()> {
    unsafe { ffi::MddBootstrapShutdown() };

    Ok(())
}
