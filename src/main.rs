use mylib::{
    grinder::{
        config_tank::ConfigTank,
        config_tank_builder::{add_variable, ConfigTankBuilder},
        grind::allocate,
        variable::{EnTy, Variable},
    },
    sudoku::SudokuVar,
};

#[macro_use]
extern crate log;

fn ele_to_str(e: (usize, &Variable<SudokuVar>)) -> String {
    // let a = e.1.get_partial().unwrap();
    // let z = if (e.0 + 1) % 9 != 0 { "" } else { "\n" };
    // format!("| {}  | {}", a, z)
    String::new()
}

fn print_solution(partial: &Vec<Variable<SudokuVar>>) {
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
    let mut b = ConfigTankBuilder::new();
    for i in 0..81 {
        add_variable(SudokuVar::new(i / 9, i % 9))
            .with_domain((0..9).map(|i| i as EnTy).collect())
            .to(&mut b);
    }

    let mut mill = b.finalize();
    // let v = vec![
    //     (1, 0, 6),
    //     (2, 0, 1),
    //     (5, 0, 7),
    //     (8, 0, 3),
    //     (1, 1, 9),
    //     (2, 1, 2),
    //     (5, 1, 3),
    //     // (2, 3, 8),
    //     // (3, 3, 5),
    //     // (4,3,3),

    //     // (6,4,5),
    //     // (8,4,4),

    //     // (0,5,5),
    //     // (5,5,8),

    //     // (1,6,4),
    //     // (8,6,1),

    //     // (3,7,1),
    //     // (4,7,6),
    //     // (5,7,8),

    //     // (0,8,6),
    // ];

    // mill.apply_unary(&v);
    let a = allocate(&mut mill);
    if a {
        print_solution(mill.get_variables())
    }
}
