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
pub const READ_FLOAT: u32 = 6;
/// $f0 contains double read
pub const READ_DOUBLE: u32 = 7;
/// $a0 address of input buffer
/// $a1 buf len
pub const READ_STRING: u32 = 8;

/// $a0 is number of bytes to be allocated
/// $v0 contains the address of allocated memory
pub const ALLOCATE_HEAP: u32 = 9;


/// Terminate execution
pub const TERMINATE_EXEC: u32 = 10;

/// Terminate execution with value
/// $a0 termination result
pub const TERMINATE_EXEC_VAL: u32 = 17;

/// $a0 = character to print
pub const PRINT_CHARACTER: u32 = 11;


/// $v0 = character to print
pub const READ_CHARACTER: u32 = 12;


/// $a0 lower 32 bits of time
/// $a1 = higher 32 bits of time
pub const SYSTEM_TIME_MS: u32 = 30;

/// $a0 is integer to print
pub const PRINT_INTEGER_HEX: u32 = 34;
/// $a0 is integer to print
pub const PRINT_INTEGER_BIN: u32 = 35;
/// $a0 is integer to print
pub const PRINT_INTEGER_UNSIGNED: u32 = 36;

/// $a0 is length of time to sleep in ms
pub const SLEEP_MS: u32 = 32;
