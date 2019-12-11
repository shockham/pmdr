use std::iter;
use std::thread::sleep;
use std::time::{Duration, Instant};

enum State {
    Pomo(u64),
    Rest,
}

fn main() -> Result<(), ()> {
    let pomo_length = 25;
    let rest_length = 5;
    let pomo_num = 4;

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
    for _ in 0..=len {
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

#[test]
fn test_run_timer_rest() {
    let result = run_timer(10u64, &"  ".into(), Duration::new(0, 1), State::Rest);
    assert!(result.is_ok()) ;
}

#[test]
fn test_run_timer_pomo() {
    let result = run_timer(10u64, &"  ".into(), Duration::new(0, 1), State::Pomo(0));
    assert!(result.is_ok()) ;
}
