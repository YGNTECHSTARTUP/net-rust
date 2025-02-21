use std::{iter::Sum, ops::Add};

pub fn borrowing() {
    let mut s = String::from("Hello Babe");
    let h = &mut s;
    h.push_str("Pika Pika");
    let y = &mut s;
    print!("{y}")
}
#[derive(Debug, Clone, Copy)]
struct Meters(f64);
#[derive(Debug, Clone, Copy)]
struct Centimeter(f64);
impl Add<Centimeter> for Meters {
    type Output = Meters;
    fn add(self, rhs: Centimeter) -> Self::Output {
        Meters(self.0 + (rhs.0 / 100.0))
    }
}

impl Add<Meters> for Centimeter {
    type Output = Centimeter;
    fn add(self, rhs: Meters) -> Self::Output {
        Centimeter(self.0 + (rhs.0 * 100.0))
    }
}

 struct Pikapi<T> {
    Pichu:T,
    Pikachu:T,
    Raichu:T
}

trait Max<T> {
    fn max(&self) -> T;
}

impl <T> Max<T> for Pikapi<T> where T:Copy+PartialOrd {
    fn max(&self) -> T {
        if self.Pichu >= self.Pikachu&&self.Pichu >= self.Raichu{
            self.Pichu
        }
        else if self.Pikachu >= self.Raichu {
            self.Pikachu
        }
        else {
            self.Raichu
        }
    }
}



pub fn generics() {
    let m1 = Meters(30.0);
    let c1 = Centimeter(39.9);
    println!("Sum in Meters {} m", (m1 + c1).0);
    println!("Sum in Centimeter {} cm", (c1 + m1).0);
    let PikaPoke = Pikapi{
        Pichu:10,
        Pikachu:20,
        Raichu:30
    };
    println!("Highes xp Pokemon is {}",PikaPoke.max());
}

fn long<T>(x: Vec<T>) -> T
where
    T: Add<Output = T> + Default,
{
    let mut sum = T::default();
    for i in x {
        sum = sum + i;
    }
    sum
}

// pub fn lifetime() {
//     let v1 = vec![1, 2, 3, 4, 5];
//     let v2 = vec![1, 2];
//     println!("{:?}", longer_vector(&v1, &v2));
// }

// fn longer_vector(x: &[i32], y: &[i32]) -> &[i32] {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// pub fn lifetime() {
//     let v1 = vec![1, 2, 3, 4, 5];
//     let v2 = vec![1, 2];
//     println!("{:?}", longer_vector(&v1, &v2));
// }

// fn longer_vector(x: &[i32], y: &[i32]) -> &[i32] {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// pub fn lifetime() {
//     let v1 = vec![1, 2, 3, 4, 5];
//     let v2 = vec![1, 2];
//     println!("{:?}", longer_vector(&v1, &v2));
// }

// fn longer_vector(x: &[i32], y: &[i32]) -> &[i32] {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// pub fn lifetime() {
//     let v1 = vec![1, 2, 3, 4, 5];
//     let v2 = vec![1, 2];
//     println!("{:?}", longer_vector(&v1, &v2));
// }

// fn longer_vector(x: &[i32], y: &[i32]) -> &[i32] {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// pub fn lifetime() {
//     let v1 = vec![1, 2, 3, 4, 5];
//     let v2 = vec![1, 2];
//     println!("{:?}", longer_vector(&v1, &v2));
// }

// fn longer_vector(x: &[i32], y: &[i32]) -> &[i32] {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// pub fn lifetime() {
//     let v1 = vec![1, 2, 3, 4, 5];
//     let v2 = vec![1, 2];
//     println!("{:?}", longer_vector(&v1, &v2));
// }

// fn longer_vector(x: &[i32], y: &[i32]) -> &[i32] {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// pub fn lifetime() {
//     let v1 = vec![1, 2, 3, 4, 5];
//     let v2 = vec![1, 2];
//     println!("{:?}", longer_vector(&v1, &v2));
// }

// fn longer_vector(x: &[i32], y: &[i32]) -> &[i32] {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
