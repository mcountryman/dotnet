pub mod bridge;

cfg_if::cfg_if! {
  if #[cfg(feature = "rt_hostfxr")] {
    pub mod hostfxr;
    pub type Global = hostfxr::HostFxrRuntime;
  } else {
    compile_error!("No host selected");
  }
}
