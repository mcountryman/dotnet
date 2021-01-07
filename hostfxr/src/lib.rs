use crate::error::HostFxrResult;
use crate::parameters::{HostFxrExtraParameters, HostFxrParameters};
use crate::wide::{IntoWideString, WideString};
use libloading::{Library, Symbol};

#[macro_use]
mod wide;
mod build;
pub mod error;
mod nethost;
mod parameters;

pub struct HostFxr<'a> {
  inner: hostfxr_sys::hostfxr_handle,
  library: HostFxrLibrary<'a>,
}

#[derive(Debug, Clone)]
pub struct HostFxrLibrary<'a> {
  set_error_writer: Symbol<'a, hostfxr_sys::hostfxr_set_error_writer_fn>,
  initialize_parameters: Symbol<'a, hostfxr_sys::hostfxr_initialize_parameters>,
  initialize_for_dotnet_command_line:
    Symbol<'a, hostfxr_sys::hostfxr_initialize_for_dotnet_command_line_fn>,
  initialize_for_runtime_config:
    Symbol<'a, hostfxr_sys::hostfxr_initialize_for_runtime_config_fn>,
  get_runtime_property_value:
    Symbol<'a, hostfxr_sys::hostfxr_get_runtime_property_value_fn>,
  set_runtime_property_value:
    Symbol<'a, hostfxr_sys::hostfxr_set_runtime_property_value_fn>,
  get_runtime_properties: Symbol<'a, hostfxr_sys::hostfxr_get_runtime_properties_fn>,
  run_app: Symbol<'a, hostfxr_sys::hostfxr_run_app_fn>,
  get_runtime_delegate: Symbol<'a, hostfxr_sys::hostfxr_get_runtime_delegate_fn>,
  close: Symbol<'a, hostfxr_sys::hostfxr_close_fn>,
}

impl<'a> HostFxrLibrary<'a> {
  pub fn from_library(library: &'a Library) -> HostFxrResult<Self> {
    Ok(unsafe {
      Self {
        set_error_writer: library.get(b"hostfxr_set_error_writer")?,

        initialize_parameters: library.get(b"hostfxr_initialize_parameters")?,
        initialize_for_dotnet_command_line: library
          .get(b"hostfxr_initialize_for_dotnet_command_line")?,
        initialize_for_runtime_config: library
          .get(b"hostfxr_initialize_for_runtime_config")?,

        get_runtime_property_value: library.get(b"hostfxr_get_runtime_property_value")?,
        set_runtime_property_value: library.get(b"hostfxr_set_runtime_property_value")?,
        get_runtime_properties: library.get(b"hostfxr_get_runtime_properties")?,
        run_app: library.get(b"hostfxr_run_app")?,
        get_runtime_delegate: library.get(b"hostfxr_get_runtime_delegate")?,
        close: library.get(b"hostfxr_close")?,
      }
    })
  }
}

// #[cfg(test)]
// mod tests {
//   use crate::fxr::HostFxrLibrary;
//   use crate::nethost::get_hostfxr;
//   use libloading::Library;
//
//   #[test]
//   fn test_invoke_main() {
//     let hostfxr = get_hostfxr().unwrap();
//     let hostfxr = Library::new(&hostfxr).unwrap();
//     let hostfxr = HostFxrLibrary::from_library(&hostfxr).unwrap();
//
//     assert_eq!(hostfxr.invoke_main(vec!["help"], "", "", "").unwrap(), 0);
//   }
// }
