struct Person<PetType, PetType2>
where 
    PetType: Animal + NotDangerous,
    PetType2: Animal + Dangerous
{
    first_name: String,
    pet: PetType,
    pet2: PetType2,
}

trait Animal {
    fn make_sound(&self) -> ();
}

trait Dangerous {}

trait NotDangerous {}

struct Dog{}
impl NotDangerous for Dog{}
impl Animal for Dog {
    fn make_sound(&self) {
        println!("Dog barks!");
    }
}

struct Cat {}
impl NotDangerous for Cat {}
impl Animal for Cat {
    fn make_sound(&self) {
        println!("Cat meows!");
    }
}

struct Bear {
}
impl Dangerous for Bear {}
impl Animal for Bear {
    fn make_sound(&self) {
        println!("Bear screams!");
    }
}

struct Tiger {}
impl Dangerous for Tiger {}
impl Animal for Tiger {
    fn make_sound(&self) {
        println!("Tiger roars!");
    }
}

impl Dog {
    fn bark(&self){
        println!("Bark!");
    }
}

pub fn create_person(){
    let pet1 = Dog{};
    let pet4 = Tiger{};
    pet1.make_sound();
    pet4.make_sound();
    let p1 = Person{first_name: "Rama".to_string(), pet: pet1, pet2: pet4};
    println!("{}", p1.first_name);
}