pub mod host;
pub use host::*;

pub mod object;
pub use object::*;

pub mod exception;
pub use exception::*;

pub struct Runtime {}

/*

dotnet::Runtime::core().init()?;

dotnet::invoke!(System.Console.WriteLine, "Hello dotnet!");

*/
