extern crate rust_gpiozero;
use rust_gpiozero::*;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

enum MorsePulse {
  Long,
  Short,
}

fn main() {
  let mut morse_code: HashMap<char, Vec<MorsePulse>> = HashMap::new();
  morse_code.insert('a', vec![MorsePulse::Long]);
  morse_code.insert(
    'b',
    vec![
      MorsePulse::Long,
      MorsePulse::Short,
      MorsePulse::Short,
      MorsePulse::Short,
    ],
  );
  morse_code.insert(
    'c',
    vec![
      MorsePulse::Long,
      MorsePulse::Short,
      MorsePulse::Long,
      MorsePulse::Short,
    ],
  );
  morse_code.insert(
    'd',
    vec![MorsePulse::Long, MorsePulse::Short, MorsePulse::Short],
  );
  morse_code.insert('e', vec![MorsePulse::Short]);
  morse_code.insert(
    'f',
    vec![
      MorsePulse::Short,
      MorsePulse::Short,
      MorsePulse::Long,
      MorsePulse::Short,
    ],
  );
  morse_code.insert(
    'g',
    vec![MorsePulse::Long, MorsePulse::Long, MorsePulse::Short],
  );
  morse_code.insert(
    'h',
    vec![
      MorsePulse::Short,
      MorsePulse::Short,
      MorsePulse::Short,
      MorsePulse::Short,
    ],
  );
  morse_code.insert('i', vec![MorsePulse::Short, MorsePulse::Short]);
  morse_code.insert(
    'j',
    vec![
      MorsePulse::Short,
      MorsePulse::Long,
      MorsePulse::Long,
      MorsePulse::Long,
    ],
  );
  morse_code.insert(
    'k',
    vec![MorsePulse::Long, MorsePulse::Short, MorsePulse::Long],
  );
  morse_code.insert(
    'l',
    vec![
      MorsePulse::Short,
      MorsePulse::Long,
      MorsePulse::Short,
      MorsePulse::Short,
    ],
  );
  morse_code.insert('m', vec![MorsePulse::Long, MorsePulse::Long]);
  morse_code.insert('n', vec![MorsePulse::Long, MorsePulse::Short]);
  morse_code.insert(
    'o',
    vec![MorsePulse::Long, MorsePulse::Long, MorsePulse::Long],
  );
  morse_code.insert(
    'p',
    vec![
      MorsePulse::Short,
      MorsePulse::Long,
      MorsePulse::Long,
      MorsePulse::Short,
    ],
  );
  morse_code.insert(
    'q',
    vec![
      MorsePulse::Long,
      MorsePulse::Long,
      MorsePulse::Short,
      MorsePulse::Long,
    ],
  );
  morse_code.insert(
    'r',
    vec![MorsePulse::Short, MorsePulse::Long, MorsePulse::Short],
  );
  morse_code.insert(
    's',
    vec![MorsePulse::Short, MorsePulse::Short, MorsePulse::Short],
  );
  morse_code.insert('t', vec![MorsePulse::Long]);
  morse_code.insert(
    'u',
    vec![MorsePulse::Short, MorsePulse::Short, MorsePulse::Long],
  );
  morse_code.insert(
    'v',
    vec![
      MorsePulse::Short,
      MorsePulse::Short,
      MorsePulse::Short,
      MorsePulse::Long,
    ],
  );
  morse_code.insert(
    'w',
    vec![MorsePulse::Short, MorsePulse::Long, MorsePulse::Long],
  );
  morse_code.insert(
    'x',
    vec![
      MorsePulse::Long,
      MorsePulse::Short,
      MorsePulse::Short,
      MorsePulse::Long,
    ],
  );
  morse_code.insert(
    'y',
    vec![
      MorsePulse::Long,
      MorsePulse::Short,
      MorsePulse::Long,
      MorsePulse::Long,
    ],
  );
  morse_code.insert(
    'z',
    vec![
      MorsePulse::Long,
      MorsePulse::Long,
      MorsePulse::Short,
      MorsePulse::Short,
    ],
  );
  let mut led = LED::new(4);

  let speed = 50;

  let sentence: &str = "hello";

  for c in sentence.chars() {
    let pulse_sequence = morse_code.get(&c).unwrap();

    for pulse in pulse_sequence {
      match pulse {
        MorsePulse::Long => {
          led.on();
          sleep(Duration::from_millis(speed * 3));
          led.off();
        }
        
        MorsePulse::Short => {
          led.on();
          sleep(Duration::from_millis(speed * 1));
          led.off();
        }
      }

      sleep(Duration::from_millis(speed * 3));
    }

    sleep(Duration::from_millis(speed * 7));
  }
}
