use std::io::{Result, Error, ErrorKind};

pub fn to_slice_3(val: usize, target: &mut [u8]) -> Result<()> {
    if val < 1000 && target.len() > 2 {
        let d0 = val/100;
        let d0_rest = val%100;
        let d1 = d0_rest/10;
        let d2 = d0_rest%10;

        // It is safe to unwrap.
        // val is guaranteed to be in the range 0..999, so 
        // d0, d1 and d2 will always be in the range 0..9.
        if d0 != 0 {
            target[0] = u8::try_from(d0 + 48).unwrap();
        }
    
        if d1 != 0 || d0 != 0 {
            target[1] = u8::try_from(d1 + 48).unwrap();
        } 
    
        if d2 != 0 || d1 != 0 {            
            target[2] = u8::try_from(d2 + 48).unwrap();
        }

        Ok(())
    } else {
        Err(Error::from(ErrorKind::Other))
    }    
}