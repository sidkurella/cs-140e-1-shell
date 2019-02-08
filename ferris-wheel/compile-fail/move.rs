// FIXME: Prevent this file from compiling! Diff budget: 1 line.
//#[derive(Clone, Copy)]
// Commented out above line. Now MyType is not copied, so
// x is unbound after moving to y; let z = x will not compile.
struct MyType(usize);

// Note: do not modify this function.
fn main() {
    let x = MyType(10);
    let y = x;
    let z = x;
}
