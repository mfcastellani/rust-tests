use std::ops::Add;

#[derive(Debug)]
struct Creature
{
    name: String,
    life: u8,
}

impl Creature
{
    fn new<S>(name: S) -> Creature
        where S: Into<String>
    {
        Creature { name: name.into(), life: 32 }
    }
}

impl Drop for Creature
{
    fn drop(&mut self)
    {
        println!("{} is dead", self.name);
    }
}

pub(crate) fn into_drop_demo()
{
    let monster = Creature::new("Freddy");
    println!("{} has life {}", monster.name, monster.life);
    drop(monster);
}