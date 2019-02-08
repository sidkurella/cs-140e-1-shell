// FIXME: Prevent this file from compiling! Diff budget: 1 line.

mod a {
    // Function was originally public; deleted pub.
    // Function is private so cannot call a::f() outside of the module.
    fn f() { }
}

// Do not modify this function.
fn main() {
    a::f();
}
