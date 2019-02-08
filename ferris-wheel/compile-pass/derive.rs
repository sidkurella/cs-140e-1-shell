// FIXME: Make me compile! Diff budget: 1 line.
// Was not originally Debug, Clone, Copy. Can't assign x to z.
// Debug needed for ? printf
#[derive(Debug, Clone, Copy)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

pub fn main() {
    println!("Duration: {:?}", Duration::MilliSeconds(1200));

    let x = Duration::Minutes(10);
    let y = x;
    let z = x;
}
