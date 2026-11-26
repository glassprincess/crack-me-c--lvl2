use std::hint::black_box;
#[cfg(windows)]
use std::io::IsTerminal;
use std::io::{self, BufRead, Read, Write};

fn main() -> io::Result<()> {
    let wait = run(&mut io::stdin().lock(), &mut io::stdout().lock())?;
    if wait {
        wait_for_key()?;
    }
    Ok(())
}

fn run(input: &mut impl BufRead, output: &mut impl Write) -> io::Result<bool> {
    // Keep the challenge's arithmetic and decoys present in optimized builds.
    let (a1, a2, a3) = black_box((943_i32, 163_i32, 34_i32));
    let (b1, b2, b3) = black_box(("Yts", "rEw", "uJkl"));
    let rtt = format!("{}{}{}{}{}", (a1 ^ a2) / a3, b1, a3 / a2, b2, b3);
    let password = black_box("improreverser000");
    let _fake_password = black_box("Flag{ImPr0ReverseP}");
    let mut user_iq = black_box(120_i32);

    writeln!(output, "Crackme Password LVL - 2")?;
    write!(output, "Enter password : ")?;
    output.flush()?;
    let mut line = String::new();
    input.read_line(&mut line)?;
    let user_pass = line.strip_suffix('\n').unwrap_or(&line);
    let user_pass = user_pass.strip_suffix('\r').unwrap_or(user_pass);

    if user_pass == b1 {
        user_iq -= 20;
    }
    if user_pass == b2 {
        user_iq -= 15;
    }
    if user_pass == b3 {
        user_iq -= 35;
    }

    if user_iq.wrapping_mul(user_iq) < 0 {
        writeln!(output, "Correct Password! You win !")?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        writeln!(output, "bro i just scammed u lol")?;
        writeln!(output, "WRONGGGGGG!!!!!!!!!")?;
        print_flag(output)?;
        return Ok(false);
    }

    if user_pass == rtt {
        writeln!(output, " ")?;
        writeln!(output, "Correct Password! You win !")?;
        print_flag(output)?;
        return Ok(false);
    } else if user_pass != rtt {
        writeln!(output, " ")?;
        writeln!(output, "!!! WRONG !!!")?;
    } else if user_pass == password {
        writeln!(output, " ")?;
        writeln!(output, "u r scammed bro, try again lol")?;
    }

    writeln!(output, " ")?;
    write!(output, "Press any key to leave : ")?;
    output.flush()?;
    Ok(true)
}

fn print_flag(output: &mut impl Write) -> io::Result<()> {
    let encoded = [
        61_u8, 23, 26, 28, 0, 90, 90, 90, 14, 41, 62, 23, 23, 2, 44, 74, 21, 57, 41, 75, 56, 75,
        53, 60, 41, 58, 47, 40, 90, 90, 90, 6,
    ];
    let key = black_box(0x7b);
    let decoded: Vec<u8> = encoded.iter().map(|byte| byte ^ key).collect();
    writeln!(
        output,
        "Your Flag is - {}",
        String::from_utf8_lossy(&decoded)
    )
}

fn wait_for_key() -> io::Result<()> {
    #[cfg(windows)]
    if io::stdin().is_terminal() {
        #[link(name = "msvcrt")]
        extern "C" {
            fn _getwche() -> u16;
        }
        unsafe { _getwche() };
        return Ok(());
    }
    let mut key = [0u8; 1];
    let _ = io::stdin().read(&mut key)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn play(input: &str) -> (bool, String) {
        let mut output = Vec::new();
        let wait = run(&mut Cursor::new(input.as_bytes()), &mut output).unwrap();
        (wait, String::from_utf8(output).unwrap())
    }

    #[test]
    fn correct_password_reveals_the_original_flag_without_waiting() {
        let (wait, output) = play("22Yts0rEwuJkl\n");
        assert!(!wait);
        assert!(output.contains("Correct Password! You win !\n"));
        assert!(output.contains("Your Flag is - Flag{!!!uREllyW1nBR0C0NGRATS!!!}\n"));
        assert!(!output.contains("Press any key"));
    }

    #[test]
    fn decoys_and_wrong_inputs_keep_the_original_wrong_password_branch() {
        for input in [
            "wrong\n",
            "Yts\n",
            "rEw\n",
            "uJkl\n",
            "improreverser000\n",
            "Flag{ImPr0ReverseP}\n",
            "\n",
            "",
        ] {
            let (wait, output) = play(input);
            assert!(wait, "input {input:?}");
            assert_eq!(output, "Crackme Password LVL - 2\nEnter password :  \n!!! WRONG !!!\n \nPress any key to leave : ");
        }
    }

    #[test]
    fn windows_newline_is_removed_but_password_spaces_are_preserved() {
        assert!(!play("22Yts0rEwuJkl\r\n").0);
        assert!(!play("22Yts0rEwuJkl").0);
        assert!(play(" 22Yts0rEwuJkl\n").0);
        assert!(play("22Yts0rEwuJkl \n").0);
    }
}
