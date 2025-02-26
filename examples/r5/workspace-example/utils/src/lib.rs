pub mod bar;
pub use bar::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = bar::add(2, 2);
        assert_eq!(result.output, 4);
    }
}
