cfg_if::cfg_if! {
  if #[cfg(feature = "host_core")] {
    pub mod core;
    pub type DefaultHost = core::CoreHost;
  } else {
    compile_error!("No host selected");
  }
}
