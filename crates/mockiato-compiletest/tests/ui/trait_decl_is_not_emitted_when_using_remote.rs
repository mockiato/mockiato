use mockiato::mockable;
use std::io;

#[mockable(remote = "io::Write")]
trait Write {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;

    fn flush(&mut self) -> io::Result<()>;
}

type _AssertTraitDeclIsNotEmitted = dyn Write;

fn main() {
}
