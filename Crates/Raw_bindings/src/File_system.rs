#[Binding_tool::Bind_function_WASM]
pub fn Open_file(Path: &str, Mode: u32, File_identifier: &mut u16) -> u32 {}

#[Binding_tool::Bind_function_WASM]
pub fn Close_file(File_identifier: u16) -> u32 {}

#[Binding_tool::Bind_function_WASM]
pub fn Read_file(File_identifier: u16, Buffer: &mut [u8], Read_size: &mut u32) -> u32 {}

#[Binding_tool::Bind_function_WASM]
pub fn Write_file(File_identifier: u16, Buffer: &[u8], Write_size: &mut u32) -> u32 {}

#[Binding_tool::Bind_function_WASM]
pub fn Flush_file(File_identifier: u16) -> u32 {}

#[Binding_tool::Bind_function_WASM]
pub fn Get_file_type(File_identifier: u16, Type_reference: &mut u32) -> u32 {}

#[Binding_tool::Bind_function_WASM]
pub fn Get_file_size(File_identifier: u16, Size_reference: &mut u64) -> u32 {}

#[Binding_tool::Bind_function_WASM]
pub fn Get_file_position(File_identifier: u16, Position_reference: &mut u64) -> u32 {}

#[Binding_tool::Bind_function_WASM]
pub fn Set_file_position(File_identifier: u16, Mode: u32, Value: u64) -> u32 {}

#[Binding_tool::Bind_function_WASM]
pub fn Delete_file(Path: &str) -> u32 {}

#[Binding_tool::Bind_function_WASM]
pub fn Create_directory(Path: &str) -> u32 {}

#[Binding_tool::Bind_function_WASM]
pub fn Create_directory_recursive(Path: &str) -> u32 {}

#[Binding_tool::Bind_function_WASM]
pub fn Delete_directory(Path: &str) -> u32 {}

#[Binding_tool::Bind_function_WASM]
pub fn Delete_directory_recursive(Path: &str) -> u32 {}

#[Binding_tool::Bind_function_WASM]
pub fn Move(Path: &str, Destination: &str) -> u32 {}
