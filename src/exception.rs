#[repr(C)]
#[derive(Debug, Clone)]
pub struct Exception {
  name: String,
  trace: String,
  message: String,
}
