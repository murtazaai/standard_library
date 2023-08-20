/// Declarative macros

/// 'create_function' creates a new function.
#[macro_export]
macro_rules! create_function {
    // This macros takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macros converts an `ident` into a string.
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}

/// 'print_result' prints the results.
#[macro_export]
macro_rules! print_result {
    // This macros takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression);
    };
}

/// `and_or!` will compare `$left` and `$right`
/// in different ways depending on how you invoke it:
#[macro_export]
macro_rules! and_or {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    };
    // ^ each arm must end with a semicolon.
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    };
}

/// `find_min!` will calculate the minimum of any number of arguments.
#[macro_export]
macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    )
}

/// 'to_bytes' converts '*str' into bytes.
#[macro_export]
macro_rules! to_bytes {
    ($a:expr) => {
        $a.as_bytes()
    }
}

