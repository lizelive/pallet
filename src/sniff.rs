/// determine the type of file

use std::path::Path;

#[test]
fn test_sniff() {
    let filepath = Path::new("/workspaces/execs/echo");
    let kind = tree_magic::from_filepath(filepath);
    println!("{}", kind);
}
