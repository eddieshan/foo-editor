struct Piece {
    start: usize,
    size: usize
}

impl Default for Piece {
    fn default() -> Self { 
        Piece { start: 0, size: 0 }
    }
}

struct PieceCursor {
    pos: usize,
    offset: usize
}

pub struct PieceChain {
    buffer: Vec<u8>,
    pieces: Vec<Piece>,
    last_piece: usize
}

fn find_cursor(pos: usize, pieces: &Vec<Piece>) -> PieceCursor {
    let mut count = 0;

    for i in 0..pieces.len() {
        let piece = &pieces[i];
        let next_count = count + piece.size;
        if next_count == 0 {
            return PieceCursor { pos: 0, offset: 0 };
        } else if next_count < pos + 1 {
            count = next_count;
        } else {
            return PieceCursor { pos: i, offset: pos - count };
        }
    }

    PieceCursor { pos: pieces.len(), offset: 0 }
}

impl PieceChain {

    fn with_capacity(capacity: usize, n_pieces: usize) -> Self {
        let mut pieces = Vec::with_capacity(n_pieces);
        pieces.push(Default::default());
        PieceChain {
            buffer: Vec::with_capacity(capacity),
            pieces: pieces,
            last_piece: 0
        }
    }
    
    fn insert(&mut self, val: u8, pos: usize) -> usize {
        if pos == self.len() {
            if let Some(piece) = self.pieces.last_mut() {
                piece.size += 1;
            }
        } else {
            let cursor = find_cursor(pos, &self.pieces);
            let new_piece = Piece { start: self.len(), size: 1 };

            if cursor.offset == 0 {
                self.pieces.insert(cursor.pos, new_piece);
                self.last_piece = cursor.pos;
            } else {
                let mut piece = &mut self.pieces[cursor.pos];
                let index = piece.start + cursor.offset;
                let piece_right = Piece { start: index, size: piece.size - cursor.offset };

                piece.size = cursor.offset;

                let new_pos = cursor.pos + 1;

                self.pieces.insert(new_pos, piece_right);
                self.pieces.insert(new_pos, new_piece);

                self.last_piece = new_pos;
            }
        }

        self.buffer.push(val);

        pos + 1
    }

    fn append(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
        if let Some(piece) = self.pieces.last_mut() {
            piece.size += data.len();
        }
    }

    fn erase(&mut self, pos: usize) {
        let size = self.buffer.len();
        if size > 0 && pos < size {
            let cursor = find_cursor(pos, &self.pieces);
            let mut piece = &mut self.pieces[cursor.pos];
    
            if cursor.offset == 0 {
                piece.start += 1;
                piece.size -= 1;
            } else if cursor.offset == piece.size - 1 {
                piece.size -= 1;
            } else {
                let new_size = cursor.offset;
                let right_start = piece.start + new_size + 1;
                let piece_right = Piece {
                    start: right_start,
                    size: piece.size - cursor.offset - 1
                };
    
                piece.size = new_size;
    
                let new_pos = cursor.pos + 1;
                self.pieces.insert(new_pos, piece_right);    
                self.last_piece = new_pos;
            }
        }
    }

    fn clear(&mut self) {
        self.buffer.clear();
        self.pieces.clear();
        self.last_piece = 0;
        self.pieces.push(Default::default());
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }

    fn capacity(&self) -> usize {
        self.buffer.capacity()
    }
}