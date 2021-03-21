use mylib::algo::allocate;
use mylib::algo::EnTy;
use mylib::algo::{Mill, Variable};

#[macro_use]
extern crate log;

fn ele_to_str(e: (usize, &Variable)) -> String {
    let a = e.1.get_partial().unwrap();
    let z = if (e.0 + 1) % 9 != 0 { "" } else { "\n" };
    format!("| {}  | {}", a, z)
}

fn print_solution(partial: &Vec<Variable>) {
    let mut s = String::new();
    partial
        .iter()
        .enumerate()
        .for_each(|i| s.push_str(&ele_to_str(i)));
    println!("{}", s);
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        // .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    let _ = setup_logger();
    let mut mill = Mill::new();
    let v = vec![
        (1, 0, 6),
        (2, 0, 1),
        (5, 0, 7),
        (8, 0, 3),
        (1, 1, 9),
        (2, 1, 2),
        (5, 1, 3),
        // (2, 3, 8),
        // (3, 3, 5),
        // (4,3,3),

        // (6,4,5),
        // (8,4,4),

        // (0,5,5),
        // (5,5,8),

        // (1,6,4),
        // (8,6,1),

        // (3,7,1),
        // (4,7,6),
        // (5,7,8),

        // (0,8,6),
    ];

    mill.apply_unary(&v);
    let a = allocate(&mut mill);
    if a {
        print_solution(mill.get_variables())
    }
}
