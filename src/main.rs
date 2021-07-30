extern crate termion;

use std::io::{stdout, Stdout};
use std::sync::mpsc::Receiver;
use std::time::Duration;

use exitfailure::ExitFailure;
use structopt::StructOpt;
use termion::raw::{IntoRawMode, RawTerminal};
use std::thread::sleep;

mod key_handler;
mod notification;
mod sound;
mod view;

#[derive(StructOpt)]
struct Option {
    #[structopt(short = "w", long = "work", default_value = "25")]
    work_min: u16,
    #[structopt(short = "s", long = "short", default_value = "4")]
    short_break_min: u16,
    #[structopt(short = "l", long = "long", default_value = "8")]
    long_break_min: u16,
    #[structopt(short = "p", long = "per_long", default_value = "3")]
    per_long: u16,
    #[structopt(short = "n", long = "num", default_value = "8")]
    num_pomo: u16,
}

fn main() -> Result<(), ExitFailure> {
    // receive cli arguemnts
    let args = Option::from_args();

    // start key handler on another thread
    let receiver = key_handler::run();

    // start timer
    let mut stdout = stdout().into_raw_mode().unwrap();
    for round in 1..=args.num_pomo {
        // work timer
        if start_timer( // 时间结束就返回, 时间结束是返回false, 未结束按退出, 是返回true
            args.work_min*60,
            round, // 这里传的是当前round, 传这个参数的作用并不是控制功能, 而是影响显示
            args.num_pomo,
            &receiver,
            &mut stdout,
            view::flush_timer,
            true
        )? {
            return Ok(());
        }
        if round == args.num_pomo {
            notification::send("All done!! \u{1F389}")?;
            break;
        }
        notification::send("it's time to take a break \u{2615}")?;
        // sound::play(sound::SoundFile::BELL)?;

        // break interval
        // view::flush_break_interval(&mut stdout)?; // 函数名中含有flush的全都是显示, 这个显示不包含时间信息
        // if handle_input_on_interval(&mut stdout, &receiver)? { // 之前可以退出, 现在只有一个功能, 继续
        //     return Ok(());
        // }

        // break timer
        let break_sec = if round % args.per_long == 0 {
            args.long_break_min * 60
        } else {
            args.short_break_min * 60
        };
        if start_timer(
            break_sec,
            round,
            args.num_pomo,
            &receiver,
            &mut stdout,
            view::flush_timer,
            false
        )? {
            return Ok(());
        }

        notification::send("it's time to work again!! \u{1F4AA}")?;
        // sound::play(sound::SoundFile::BELL)?;

        // work interval
        // view::flush_work_interval(&mut stdout)?;
        // if handle_input_on_interval(&mut stdout, &receiver)? {
        //     return Ok(());
        // }
    }
    Ok(())
}

// 返回有2种情况, remaining_sec为0, 或者按了退出, 但是前者返回的是Ok(false), 后者返回Ok(true)
fn start_timer(
    remaining_sec: u16,
    current_round: u16,
    num_pomo: u16,
    receiver: &Receiver<key_handler::KeyAction>,
    stdout: &mut RawTerminal<Stdout>,
    flush_fn: fn(s: &mut RawTerminal<Stdout>, t: u16, c: u16, num_pomo: u16, is_work: bool) -> Result<(), failure::Error>,
    is_work: bool
) -> Result<bool, failure::Error> {
    let mut quited = false;
    let mut paused = false;
    let mut remaining_sec = remaining_sec;
    while remaining_sec != 0 {
        match handle_input_on_timer(receiver) {
            key_handler::KeyAction::Quit => {
                view::release_raw_mode(stdout)?;
                quited = true;
                break;
            }
            key_handler::KeyAction::Pause => paused = !paused,
            _ => (),
        }
        if !paused {
            flush_fn(stdout, remaining_sec, current_round, num_pomo, is_work)?;
            remaining_sec -= 10;
        }
        sleep(Duration::from_secs(10));
    }
    Ok(quited)
}

fn handle_input_on_timer(receiver: &Receiver<key_handler::KeyAction>) -> key_handler::KeyAction {
    match receiver.try_recv() {
        Ok(key_handler::KeyAction::Quit) => key_handler::KeyAction::Quit,
        Ok(key_handler::KeyAction::Pause) => key_handler::KeyAction::Pause,
        _ => key_handler::KeyAction::None,
    }
}

// 根据键盘返回, 搞不懂, 如果已经有了handle_input_on_timer, 还要这个干啥
fn handle_input_on_interval(
    stdout: &mut RawTerminal<Stdout>,
    receiver: &Receiver<key_handler::KeyAction>,
) -> Result<bool, failure::Error> {
    let mut quited = false;
    for received in receiver.iter() {
        match received {
            key_handler::KeyAction::Ok => break,
            // key_handler::KeyAction::Quit => {
            //     view::release_raw_mode(stdout)?;
            //     quited = true;
            //     break;
            // }
            _ => (),
        }
    }
    Ok(quited)
}
