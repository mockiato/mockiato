use mockiato::mockable;

#[mockable(static)]
trait Greeter: 'static {
    fn greet(&self, name: &str) -> String;
}

#[test]
fn test() {}
