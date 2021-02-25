use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();

    print!("Modulo: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("error: unable to read user input");

    let n = input.trim().parse::<u16>().unwrap();


    println!("");
    let mut input = String::new();

    println!("(Addition)       (1)");
    println!("Multiplication   (2)");
    print!("Operation: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("error: unable to read user input");

    let operation: u16 = if !input.trim().is_empty() {
        if input.trim().parse::<u16>().unwrap() == 2 {
            2
        } else {
            1
        }
    } else {
        1
    };


    println!("{}", operation);
    let mut input = String::new();

    print!("Omissions (seperate by comma): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");

    let ommissions: Vec<u16> = input.trim()
                                    .split(",")
                                    .filter(|x| !x.is_empty())
                                    .map(|x| x.parse::<u16>().unwrap())
                                    .collect();

    /*
\begin{table}[]
\begin{tabular}{lllll}
&  &  &  &  \\
&  &  &  &  \\
&  &  &  &  \\
&  &  &  &
\end{tabular}
\end{table}
    */

    let alignment = "l".repeat(n as usize);

    print!("\n\\begin{{table}}[] \n\\begin{{tabular}}{{|l|{}|}} \n\\hline \n", alignment);

    let mut bar_numbers: Vec<u16> = (0..n).collect();

    bar_numbers.retain(|x| {
        !ommissions.iter().any(|y| x == y)
    });

    let mut bar: String = bar_numbers.iter().map(|x| {let mut x = x.to_string();x.push_str(" & ");x}).collect::<String>();
    bar.truncate(bar.len()-2);
    print!("  & {}\\\\\n\\hline \n", bar);
    // print!("{}\\\\\n", (0..n).map(|x| x.to_string()).collect::<Vec<_>>().join(" & "));

    for j in 0..n {

        if ommissions.iter().any(|x| j == *x) {
            continue;
        }

        print!("{}", j);

        for i in 0..n {
            if ommissions.iter().any(|x| i == *x) {
                continue;
            }

            if operation == 1 {
                print!(" & {}", i+j % n);
            } else {
                print!(" & {}", i*j % n);
            }
        }
        print!(" \\\\ \n");
    }
    print!("\\hline \n\\end{{tabular}} \n\\end{{table}} \n")
}
