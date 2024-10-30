trait  animal {
    fn speak(&self);
}

struct dog{
    name: String,
}

impl dog {
    fn new(name: &str) -> dog {
        dog {
            name: name.to_string(),
        }
    }
}

impl animal for dog {

    
    fn speak(&self) {
        print!("Wow")
    }
}
fn main() {
   let x= dog::new("bobby");
    x.speak();
  
}

