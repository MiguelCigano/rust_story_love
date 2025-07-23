struct Person {
    name: String,
    email: String,
    age: i32,
    active: bool                           // avoid use ","
}

fn main() 
{
    // NOT mutable type Person
    let person_01 = Person {
        name: "Rusty".to_string(),        // static chain to string
        email: String::from("rs@gmail.com"), // other able way to string
        age: 20,
        active: true,                         
    };
    println!("Person: {}, with email {} is ACTIVE: {}.", person_01.name, person_01.email, person_01.active);
    
    // mutable type Person
    let mut person_02 = new_person_fn(String::from("Vanessa"), String::from("vanessa@gmail.com"));
    println!("Person: {}, email {} is ACTIVE: {}.", person_02.name, person_02.email, person_02.active);
    
    // passing mutable reference like argument
    change_status_person(&mut person_02);
    println!("Person: {}, email {} is ACTIVE: {}.", person_02.name, person_02.email, person_02.active);
}

fn new_person_fn(name: String, email: String) -> Person {
    return Person {
        name,           // recognizes implicitly the argument
        email,          // recognizes implicitly the argument
        age: 30,
        active: false,
    };
}

fn change_status_person(person_x: &mut Person) {
    return person_x.active = true;
}
