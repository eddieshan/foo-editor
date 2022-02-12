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
    len: usize,
    pieces: Vec<Piece>
}

fn find_cursor(pos: usize, pieces: &Vec<Piece>) -> PieceCursor {
    let mut count = 0;
    let limit = pos + 1;

    for i in 0..pieces.len() {
        let piece = &pieces[i];
        let next_count = count + piece.size;
        if next_count == 0 {
            return PieceCursor { pos: 0, offset: 0 };
        } else if next_count < limit {
            count = next_count;
        } else {
            return PieceCursor { pos: i, offset: pos - count };
        }
    }

    PieceCursor { pos: pieces.len(), offset: 0 }
}

impl PieceChain {

    pub fn with_capacity(capacity: usize, n_pieces: usize) -> Self {
        let mut pieces = Vec::with_capacity(n_pieces);
        pieces.push(Default::default());
        PieceChain {
            buffer: Vec::with_capacity(capacity),
            len: 0,
            pieces: pieces
        }
    }
    
    pub fn insert(&mut self, val: u8, pos: usize) {
        if pos == self.len() {
            if let Some(piece) = self.pieces.last_mut() {
                piece.size += 1;
            }
        } else {
            let cursor = find_cursor(pos, &self.pieces);
            let new_piece = Piece { start: self.len(), size: 1 };

            if cursor.offset == 0 {
                self.pieces.insert(cursor.pos, new_piece);
            } else {
                let mut piece = &mut self.pieces[cursor.pos];
                let index = piece.start + cursor.offset;
                let piece_right = Piece { start: index, size: piece.size - cursor.offset };

                piece.size = cursor.offset;

                let new_pos = cursor.pos + 1;

                self.pieces.insert(new_pos, piece_right);
                self.pieces.insert(new_pos, new_piece);
            }
        }

        self.buffer.push(val);
        self.len += 1;
    }

    pub fn append(&mut self, data: &[u8]) {
        if let Some(piece) = self.pieces.last_mut() {
            let chunk_size = data.len();
            self.buffer.extend_from_slice(data);
            self.len += chunk_size;
            piece.size += chunk_size;
        }
    }

    pub fn erase(&mut self, pos: usize) {
        if pos < self.len {
            let cursor = find_cursor(pos, &self.pieces);
            let mut piece = &mut self.pieces[cursor.pos];
   
            if cursor.offset == 0 {
                piece.start += 1;
                piece.size -= 1;
            } else if cursor.offset == piece.size - 1 {
                piece.size -= 1;
            } else {
                let right_start = piece.start + cursor.offset + 1;
                let piece_right = Piece {
                    start: right_start,
                    size: piece.size - cursor.offset - 1
                };
    
                piece.size = cursor.offset;
    
                let new_pos = cursor.pos + 1;
                self.pieces.insert(new_pos, piece_right);    
            }

            self.len -= 1;
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.pieces.clear();
        self.pieces.push(Default::default());
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    pub fn copy_to(&self, buffer: &mut Vec<u8>) {
        buffer.clear();
        for piece in self.pieces.iter() {
            let end = piece.start + piece.size;
            buffer.extend_from_slice(&self.buffer[piece.start..end]);
        }
    }    
}