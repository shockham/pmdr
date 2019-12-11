use std::env;
use std::iter;
use std::str::FromStr;
use std::thread::sleep;
use std::time::{Duration, Instant};

enum State {
    Pomo(u64),
    Rest,
}

fn main() -> Result<(), ()> {
    let pomo_length = env::args()
        .nth(1)
        .map_or(25, |n| u64::from_str(&n).unwrap_or(25));

    let rest_length = env::args()
        .nth(2)
        .map_or(5, |n| u64::from_str(&n).unwrap_or(5));

    let pomo_num = env::args()
        .nth(3)
        .map_or(4, |n| u64::from_str(&n).unwrap_or(4));

    let clear_space: String = iter::repeat(" ")
        .take(pomo_length.to_string().len())
        .flat_map(|s| s.chars())
        .collect();

    let incr_duration = Duration::new(60, 0);

    for pnum in 0..pomo_num {
        // Work time
        run_timer(pomo_length, &clear_space, incr_duration, State::Pomo(pnum))?;

        // Rest time
        run_timer(rest_length, &clear_space, incr_duration, State::Rest)?;
    }

    eprint!("\rDONE");
    Ok(())
}

fn run_timer(len: u64, clr: &String, dur: Duration, state: State) -> Result<(), ()> {
    let start = Instant::now();
    for _ in 0..len {
        let state_str: String = match state {
            State::Pomo(t) => t.to_string(),
            State::Rest => "r".into(),
        };

        eprint!(
            "\r{}:{}/{}{}",
            state_str,
            len - start.elapsed().as_secs(),
            len,
            clr
        );

        sleep(dur);
    }

    Ok(())
}

macro_rules! run_timer_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (len, pomo) = $value;
            let result_rest = run_timer(len, &"  ".into(), Duration::new(0, 1), State::Rest);
            assert!(result_rest.is_ok());

            let result_pomo = run_timer(len, &"  ".into(), Duration::new(0, 1), State::Pomo(pomo));
            assert!(result_pomo.is_ok());
        }
    )*
    }
}

run_timer_tests! {
    run_timer_test_0: (0, 0),
    run_timer_test_1: (1, 1),
    run_timer_test_2: (2, 2),
    run_timer_test_3: (3, 3),
    run_timer_test_4: (4, 4),
    run_timer_test_5: (5, 5),
    run_timer_test_10: (10, 10),
    run_timer_test_25: (25, 25),
    run_timer_test_50: (50, 50),
    run_timer_test_100: (100, 100),
}
