// FIXME: Make me pass! Diff budget: 25 lines.
use std::cmp;

#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

impl Duration {
    fn to_millis(&self) -> u64 {
        match self {
            Duration::MilliSeconds(x) => *x as u64,
            Duration::Seconds(x) => (*x as u64) * 1000,
            Duration::Minutes(x) => (*x as u64) * 1000 * 60
        }
    }
}

impl cmp::PartialEq for Duration {
    fn eq(&self, other: &Duration) -> bool {
        self.to_millis() == other.to_millis()
    }
}

fn main() {
    use Duration::Seconds;
    use Duration::Minutes;
    use Duration::MilliSeconds;
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
