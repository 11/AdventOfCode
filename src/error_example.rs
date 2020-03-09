// macro_rules! error {
//     ($($arg:tt)*) => {
//        $crate::Error::new(format!("{}", format_args!($($args)*)))
//     };
// }
//
// macro_rules! bail {
//     ($($arg:tt)*) => {
//         return Err($crate::Error::new(format!("{}", format_args!($($args)*))));
//     }
//
// }
//
// #[derive(Debug)]
// pub struct Error;
//
// impl MyError {
//     fn new<S>(s: S) -> Self where S: Into<String> {
//         Error
//     }
//
// impl From<io::Error> for MyError {
//     fn from(e: io::Error) -> Self {
//         Error
//     }
// }
//
// impl fmt::Display for MyError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         writeln!(f, "An error occurred")
//     }
// }
//
// impl std::error::Error for MyError {}
