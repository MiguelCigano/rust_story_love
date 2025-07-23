struct PlayEnemy<'a> {
    vec: &'a mut [i32] 
}

impl<'a> PlayEnemy<'a> {
    fn new(vec: &'a mut [i32]) -> Self {
        Self { vec }
    }

    fn show_play_1D(&mut self) {
        for &mut v in self.vec.iter_mut() {
            print! ("{} ", v);
        }
        println!();
    }

    fn change_vector(&mut self) {
        for value in self.vec.iter_mut() {
            *value = *value + 1;
        }
    }
}

fn main() {
    let mut vec = [1, 0, 1, 0, 1];
    let ln = vec.len();

    let mut enemy = PlayEnemy::new(&mut vec);
    println!("Length: {}", ln);
    enemy.show_play_1D();
    enemy.change_vector();
    enemy.show_play_1D();
}
