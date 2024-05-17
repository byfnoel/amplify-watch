extern crate amplify_watch;

fn main() {
    let connection = amplify_watch::main();
    println!("{:#?}", connection);
}
