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

    fn extend(&mut self, increment: usize) {
        self.size += increment;
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
    chain: Vec<Piece>
}

impl PieceChain {

    pub fn with_capacity(capacity: usize, n_pieces: usize) -> Self {
        let mut chain = Vec::with_capacity(n_pieces);
        chain.push(Default::default());
        PieceChain {
            buffer: Vec::with_capacity(capacity),
            chain: chain
        }
    }

    pub fn insert(&mut self, val: u8, pos: usize) {
        let last_pos = self.chain.last().map_or(0, |p| p.start + p.size);
        let size = self.buffer.len();
        match (self.find(pos), last_pos == size) {
            (None, true)  => self.chain.last_mut().unwrap().extend(1),
            (None, false) => self.chain.push(Piece { start: size, size: 1 }),
            (Some(cursor), _) => {
                let new_piece = Piece { start: size, size: 1 };

                if cursor.offset == 0 {
                    self.chain.insert(cursor.pos, new_piece);
                } else {
                    let piece = &mut self.chain[cursor.pos];
                    let piece_right = piece.split_right(cursor.offset);
                    piece.resize(cursor.offset);
    
                    let new_pos = cursor.pos + 1;
    
                    self.chain.insert(new_pos, piece_right);
                    self.chain.insert(new_pos, new_piece);
                }
            }
        }

        self.buffer.push(val);
    }

    pub fn append(&mut self, data: &[u8]) {
        if let Some(piece) = self.chain.last_mut() {
            piece.extend(data.len());
            self.buffer.extend_from_slice(data);
        }
    }

    pub fn erase(&mut self, pos: usize) {
        if let Some(cursor) = self.find(pos) {
            let n_pieces = self.chain.len();
            let piece = &mut self.chain[cursor.pos];

            if piece.size > 0 {
                match (piece.size, n_pieces, cursor.offset) {
                    (1, 1, _) => piece.shrink_left(),
                    (1, _, _) => { self.chain.remove(cursor.pos); },
                    (_, _, 0) => piece.shrink_right(),
                    (_, _, offset) if offset == piece.size - 1 => piece.shrink_left(),
                    (_, _, _) => {
                        let piece_right = piece.split_right(cursor.offset + 1);
                        piece.resize(cursor.offset);
            
                        let new_pos = cursor.pos + 1;
                        self.chain.insert(new_pos, piece_right);                    
                    }
                };
            }
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.chain.clear();
        self.chain.push(Default::default());
    }

    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    pub fn copy_to(&self, buffer: &mut Vec<u8>) {
        buffer.clear();
        for piece in self.chain.iter() {
            let end = piece.start + piece.size;
            buffer.extend_from_slice(&self.buffer[piece.start..end]);
        }
    }

    fn find(&self, pos: usize) -> Option<PieceCursor> {
        let mut offset = pos;
    
        for i in 0..self.chain.len() {
            match self.chain[i].size {
                size if offset >= size => offset -= size,
                size => return Some(PieceCursor { pos: i, offset: offset })
            }
        }
    
        None
    }    
}