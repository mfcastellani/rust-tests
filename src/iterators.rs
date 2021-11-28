pub(crate) fn iter_demo()
{
    let mut vec = vec![3, 2, 1];

    // ordinary iteration causes a move
    for x in &vec
    {
        println!("{}", *x);
    }

    // iter() = a bunch of immutable references

    //  ^^^
    for x in vec.iter()
    {
        println!("we got {}", x);
        // cannot modify things!
        // x += 1;
    }

    // iter adapter methods
    for x in vec.iter().rev()
    {
        println!("in reverse: {}", x);
    }

    // iter_mut() - mutable references, requires
    //              the vector to be declared mut
    for x in vec.iter_mut()
    {
        *x += 2;
    }
    println!("{:?}", vec);

    // into_iter() - move operation that transforms the collection into a by-value iterator
    //               not the same as ordinary iteration!
    //               useful when you need values but not the collection itself
    // extend() - automatically calls into_iter() to move elements from one collection to another
    let mut vec2 = vec![1, 2, 3];
    vec2.extend(vec);
    println!("{:?}", vec2);
}