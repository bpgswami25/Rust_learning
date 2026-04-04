use procedural_macros::MyDebug;

#[derive(MyDebug)]
enum ServerError {
    #[debug("The connection timed out")]
    Timeout,
    #[debug("Permission was denied by the OS")]
    PermissionDenied,
    Unknown, // Will default to printing "DEBUG: Unknown"
}

//MyDebug macro expand the code like-
// impl std::fmt::Debug for ServerError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ServerError::Timeout => {
//                 f.write_fmt(format_args!("DEBUG: {0}", "The connection timed out"))
//             }
//             ServerError::PermissionDenied => {
//                 f.write_fmt(
//                     format_args!("DEBUG: {0}", "Permission was denied by the OS"),
//                 )
//             }
//             ServerError::Unknown => f.write_fmt(format_args!("DEBUG: {0}", "Unknown")),
//         }
//     }
// }

fn main() {
    let err = ServerError::Timeout;
    println!("{:?}", err); // Output: DEBUG: The connection timed out

    let perm_err = ServerError::PermissionDenied;
    println!("{:?}", perm_err); // Output: DEBUG: Permission was denied by the OS

    let unknown_err = ServerError::Unknown;
    println!("{:?}", unknown_err); // Output: DEBUG: Unknown
}
