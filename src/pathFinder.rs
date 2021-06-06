


#[cfg(test)]
mod test {
    use crate::board::Board;
    #[test]
    fn starting_board() {
        let board = Board::create_default();
        let height = board.get_height();
        assert_eq!(height-1, shortest_distance((0,4), (7,4)));
        
    }
}