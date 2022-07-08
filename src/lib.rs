pub mod coords;
pub mod direction;
pub mod floor;
pub mod room;
pub mod dungeon;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
