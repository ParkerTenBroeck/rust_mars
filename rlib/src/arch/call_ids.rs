/// $a0 is integer to print
pub const PRINT_INTEGER: u32 = 1;
/// $f12 is float to print
pub const PRINT_FLOAT: u32 = 2;
/// $f12 is double to print
pub const PRINT_DOUBLE: u32 = 3;

/// $a0 is address of string to print 
/// null terminated
pub const PRINT_STRING: u32 = 4;

/// $v0 contains integer read
pub const READ_INTEGER: u32 = 5;
/// $f0 contains float read
pub const READ_FLOAT: u32 = 5;
/// $f0 contains double read
pub const READ_DOUBLE: u32 = 5;
/// $a0 = address of input buffer
/// $a1 = max characters to be read
pub const READ_STRING: u32 = 5;

/// $a0 is number of bytes to be allocated
/// $v0 contains the address of allocated memory
pub const ALLOCATE_HEAP: u32 = 9;

/// Terminate execution
pub const TERMINATE_EXEC: u32 = 10;

/// $a0 = character to print
pub const PRINT_CHARACTER: u32 = 11;


/// $v0 = character to print
pub const READ_CHARACTER: u32 = 12;