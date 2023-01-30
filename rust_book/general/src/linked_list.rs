
pub struct Node {
    pub data: u32,
    pub next: Option<Box<Node>>
}

impl Node {
    pub fn append(&mut self, val: u32){
        match &mut self.next{
            Some(next) => (*next).append(val),
            None => self.next = Some(Box::new(Node {data: val, next: None}))
        }
    }
    pub fn count(&self) -> u32{
        match &self.next{
            Some(next) => 1 + (*next).count(),
            None => 1,
        }
    }
    pub fn print_values(&self){
        match &self.next{
            Some(next) => {
                println!("{}", self.data);
                (*next).print_values();
            }
            None => println!("{}", self.data)
        }
    }
}