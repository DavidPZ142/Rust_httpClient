pub struct Student{
    pub name : String,
    pub age : u8,
}
impl Student {
    pub fn new (name: &str , age : u8) -> Student{
        Student {
            name : name.to_string(),
            age : age
        }
    }

    pub fn look_info(&self){
        println!("Name :{}", self.name)
    }
}