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

    let col: usize = std::env::args()
        .nth(1)
        .expect("no col specified")
        .parse()
        .expect("can not parse col number");

	if col == 0 {
		panic!("0 is an invalid col index");
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
				while buffer.peek()	== Some(&' ') {buffer.next();}
			}
			Some('\n') => {
				tmp_row.push(tmp_str.drain(..).collect());
				table.push(tmp_row.drain(..).collect());
			}
			Some(c) => {tmp_str.push(c);}
			None => break
		}
	}

	//leftover
	tmp_row.push(tmp_str.drain(..).collect());
	table.push(tmp_row.drain(..).collect());

	let mut out :String = table.iter().filter_map(|row|row.get(col - 1)).fold(String::new(), |mut acc, x|{
		acc.push_str(&x);
		acc.push('\n');
		acc
	});
	out.pop();
	let out = out;
	
    write!(io::stdout(), "{}", out)?;
    io::stdout().flush()?;
    Ok(())
}
