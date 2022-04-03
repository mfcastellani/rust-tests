#[derive(Debug)]
struct Student
{
    name: String,
}

impl Student
{
    fn new(name: &str) -> Student
    {
        Student {
            name: name.into(),
        }
    }
}

impl Drop for Student
{
    fn drop(&mut self)
    {
        println!("{} is dead", self.name);
    }
}

#[derive(Debug)]
struct Course
{
    name: String,
}

impl Course
{
    fn new(name: &str) -> Course
    {
        Course {
            name: name.into(),
        }
    }
}

impl Drop for Course
{
    fn drop(&mut self)
    {
        println!("{} is dead", self.name);
    }
}

#[derive(Debug)]
struct Enrollment<'a>
{
    student: &'a Student,
    course: &'a Course,
}

impl<'a> Enrollment<'a>
{
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a>
    {
        Enrollment {student, course}
    }
}

impl<'a> Drop for Enrollment<'a>
{
    fn drop(&mut self)
    {
        println!("Enrollment is dead");
    }
}

pub(crate) fn lifetime_demo()
{
    let joseph = Student::new("Joseph");
    let course = Course::new("Rust Course");
    let enroll = Enrollment::new(&joseph, &course);

    println!("{:?}", enroll);

    // drop(joseph); cannot move out of `joseph` because it is borrowed
    drop(enroll);
}

