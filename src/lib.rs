mod styles;

pub fn stuff() {
    println!("dsadsadsa");
}

pub mod my_mod {
    pub fn stuff2() {
        println!("dsadsa");
    }
}

pub mod my_mod2 {
    pub fn stuff3() {
        println!("dsadsadsadsadsadsad");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
