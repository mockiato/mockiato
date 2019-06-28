use mockiato::mockable;

#[mockable(remote = 1)]
trait NotAStringLiteral {}

#[mockable(remote = "foo", remote = "bar")]
trait SpecifiedMoreThanOnce {}

#[mockable(remote = "foo?")]
trait InvalidPath {}

fn main() {}
