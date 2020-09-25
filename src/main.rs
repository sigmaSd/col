use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let buffer = std::env::args().nth(2);
    let buffer = if let Some(buffer) = buffer {
        buffer
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    let col: isize = std::env::args()
        .nth(1)
        .expect("no col specified")
        .parse()
        .expect("can not parse col number");

    if col == 0 {
        write!(io::stdout(), "{}", buffer)?;
        io::stdout().flush()?;
        return Ok(());
    }

    let mut table: Vec<Vec<String>> = Default::default();
    let mut tmp_row: Vec<String> = Default::default();
    let mut tmp_str: String = Default::default();

    let mut buffer = buffer.chars().peekable();
    loop {
        let c = buffer.next();
        match c {
            Some(' ') => {
                tmp_row.push(tmp_str.drain(..).collect());
                while buffer.peek() == Some(&' ') {
                    buffer.next();
                }
            }
            Some('\n') => {
                tmp_row.push(tmp_str.drain(..).collect());
                table.push(tmp_row.drain(..).collect());
            }
            Some(c) => {
                tmp_str.push(c);
            }
            None => break,
        }
    }

    //leftover
    if !tmp_str.is_empty() {
        tmp_row.push(tmp_str.drain(..).collect());
    }
    if !tmp_row.is_empty() {
        table.push(tmp_row.drain(..).collect());
    }

    let out: String = table
        .iter()
        .filter_map(|row| {
            let idx = if col > 0 {
                col - 1
            } else {
                let idx = col + row.len() as isize - 1;
                if idx < 0 {
                    return None;
                } else {
                    idx
                }
            };
            row.get(idx as usize)
        })
        .fold(String::new(), |mut acc, x| {
            acc.push_str(&x);
            acc.push('\n');
            acc
        });

    write!(io::stdout(), "{}", out)?;
    io::stdout().flush()?;
    Ok(())
}
