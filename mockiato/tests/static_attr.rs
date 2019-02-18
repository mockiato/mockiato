use mockiato::mockable;

#[mockable(static_references)]
trait Greeter: 'static {
    fn greet(&self, name: &str) -> String;
}

#[test]
fn test() {}
