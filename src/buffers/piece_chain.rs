struct Piece {
    start: usize,
    size: usize
}

impl Default for Piece {
    fn default() -> Self { 
        Piece { start: 0, size: 0 }
    }
}

impl Piece {
    fn split_right(&self, offset: usize) -> Piece {
        Piece { 
            start: self.start + offset, 
            size: self.size - offset 
        }
    }

    fn shrink_left(&mut self) {
        self.size -= 1;
    }

    fn shrink_right(&mut self) {
        self.start += 1;
        self.size -= 1;
    }
    
    fn resize(&mut self, new_size: usize) {
        self.size = new_size;
    }    
}

struct PieceCursor {
    pos: usize,
    offset: usize
}

pub struct PieceChain {
    buffer: Vec<u8>,
    pieces: Vec<Piece>
}

// fn find_cursor(pos: usize, pieces: &Vec<Piece>) -> Option<PieceCursor> {
//     let mut count = 0;
//     let limit = pos + 1;

//     for i in 0..pieces.len() {
//         let next_count = count + pieces[i].size;
//         if next_count < limit {
//             count = next_count;
//         } else {
//             return Some(PieceCursor { pos: i, offset: pos - count });
//         }
//     }

//     None
// }

fn find_cursor(pos: usize, pieces: &Vec<Piece>) -> Option<PieceCursor> {
    let mut offset = pos;

    for i in 0..pieces.len() {
        if pieces[i].size <= offset {
            offset -= pieces[i].size;
        } else {
            return Some(PieceCursor { pos: i, offset: offset });
        }
    }

    None
}


impl PieceChain {

    pub fn with_capacity(capacity: usize, n_pieces: usize) -> Self {
        let mut pieces = Vec::with_capacity(n_pieces);
        pieces.push(Default::default());
        PieceChain {
            buffer: Vec::with_capacity(capacity),
            pieces: pieces
        }
    }

    pub fn insert(&mut self, val: u8, pos: usize) {
        let end_pos = self.buffer.len();

        if pos == end_pos {
            // The last piece can be unwrapped safely because the piece chain
            // must contain by definition at least one piece. 
            // If the piece chain has zero pieces, that's a an error that needs
            // to be detected so a panic from unwrap is convenient.
            let piece = self.pieces.last_mut().unwrap();
            piece.size += 1;
        } else if let Some(cursor) = find_cursor(pos, &self.pieces) {            
            let new_piece = Piece { start: end_pos, size: 1 };

            if cursor.offset == 0 {
                self.pieces.insert(cursor.pos, new_piece);
            } else {
                let piece = &mut self.pieces[cursor.pos];
                let piece_right = piece.split_right(cursor.offset);
                piece.resize(cursor.offset);

                let new_pos = cursor.pos + 1;
                self.pieces.insert(new_pos, piece_right);
                self.pieces.insert(new_pos, new_piece);
            }
        }

        self.buffer.push(val);
    }

    pub fn append(&mut self, data: &[u8]) {
        if let Some(piece) = self.pieces.last_mut() {
            let chunk_size = data.len();
            self.buffer.extend_from_slice(data);
            piece.size += chunk_size;
        }
    }

    pub fn erase(&mut self, pos: usize) {
        match find_cursor(pos, &self.pieces) {
            Some(cursor) if self.pieces[cursor.pos].size > 0 => {
                let piece = &mut self.pieces[cursor.pos];

                match cursor.offset {
                    0 => piece.shrink_right(),
                    offset if offset == piece.size - 1 => piece.shrink_left(),
                    _ => {
                        let piece_right = piece.split_right(cursor.offset + 1);
                        piece.resize(cursor.offset);

                        let new_pos = cursor.pos + 1;
                        self.pieces.insert(new_pos, piece_right); 
                    }
                };   
            },
            _ => { }
        }
    }    

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.pieces.clear();
        self.pieces.push(Default::default());
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