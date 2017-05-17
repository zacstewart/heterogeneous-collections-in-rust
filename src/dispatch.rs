use std::fmt::Display;

pub trait Fancy: Display {
    fn fancy(&self) -> String;
}

impl Fancy for u8 {
    fn fancy(&self) -> String {
        format!("💙 {} 💙", self)
    }
}

impl Fancy for u16 {
    fn fancy(&self) -> String {
        format!("💚 {} 💚", self)
    }
}

pub fn static_print_fancy<T: Fancy>(thing: T) {
    println!("Static Print Fancy: {}", thing);
}

pub fn dynamic_print_fancy(thing: &Fancy) {
    println!("Dynamic Print Fancy: {}", thing);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_dispatch() {
        let number = 10u8;
        static_print_fancy(number);

        let number = 10u16;
        static_print_fancy(number);
    }

    #[test]
    fn dynamic_dispatch() {
        let trait_object = &10u8 as &Fancy;
        dynamic_print_fancy(trait_object);

        let number = 10u16;
        dynamic_print_fancy(&number);
    }

    #[test]
    fn trait_objects() {
        let boxed = Box::new(10u8) as Box<Fancy>;
        let shared_reference = &10u8 as &Fancy;
        let mut_reference = &mut 10u8 as &mut Fancy;
        let raw_pointer = &10u8 as *const Fancy;
        let mut_raw_pointer = &mut 10u8 as *mut Fancy;
    }

    #[test]
    fn object_safety() {
        let numbers = vec![1, 2, 3];
        let trait_object = &numbers as &Clone;
    }
}
