use std::io::{Stdout, Result, Write};

use crate::{ansi, theme};
use crate::core::*;

pub fn render(stdout: &mut Stdout, ln: usize, total_ln: usize, info: &TermInfo) -> Result<()> {
    stdout.write(ansi::HOME)?;
    stdout.write(theme::GUTTER_DEFAULT)?;

    for i in 1..=total_ln {
        if i == ln {
            stdout.write(theme::GUTTER_HIGHLIGHT)?;
            print!("{:>3} ", i);
            stdout.write(theme::GUTTER_DEFAULT)?;
        } else {
            print!("{:>3} ", i);
        }            
        
        stdout.write(ansi::NEXT_LINE)?;
    }

    stdout.write(ansi::RESET)?;

    Ok(())
}
