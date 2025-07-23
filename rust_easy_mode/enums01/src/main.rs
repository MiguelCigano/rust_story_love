struct User {
    name: String,
    email: String,
    plan: UserStatus,
    access_id_01: UserAccessId,
    access_id_02: UserAccessId
}

enum UserStatus {
    FREE,
    PREMIUM,
}

enum UserAccessId {
    POOL(String),
    GYM(String),
}

fn check_access(plan: &UserStatus) -> bool {
    match plan {
        UserStatus::FREE => false,
        UserStatus::PREMIUM => true
    }
}

fn show_access(access_id: &UserAccessId) {
    match access_id {
        UserAccessId::POOL(id) => println!("User has access to POOL: {}", id),
        UserAccessId::GYM(id) => println!("User has access to GYM: {}", id),
    }
}

fn main() {

    let user01 = User {
        name: String::from("Jesus Miguel"),
        email: String::from("jm@rust.com"),
        plan: UserStatus::PREMIUM,
        access_id_01: UserAccessId::POOL(String::from("j.miguel")),
        access_id_02: UserAccessId::GYM(String::from("j.miguel")),
    };

    let access = check_access(&user01.plan);
    println!("User: {}, access: {}.", user01.name, access);

    if access {
        show_access(&user01.access_id_01)
    }
    else {
        println!("User does not have access to POOL!");
    }

    if access {
        show_access(&user01.access_id_02)
    }
    else {
        println!("User does not have access to GYM!");
    }
}
