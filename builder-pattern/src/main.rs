use builder_pattern::*;

fn main() {
    let thing = ThingBuilder::default()
        .set_one(3)
        .set_three("Hello!!".to_owned())
        .build();
    println!("{thing:?}");
}
