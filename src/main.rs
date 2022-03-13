use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn blink(length: u64, led: Pin) -> Result<(), sysfs_gpio::Error> {
    led.set_value(1).unwrap();
    sleep(Duration::from_millis(length));
    led.set_value(0).unwrap();
    sleep(Duration::from_millis(length));
    Ok(())
}

fn letter_to_morse(led: Pin, letter: char) -> Result<(), sysfs_gpio::Error> {
    match letter {
        's' => {
            let mut i = 0;
            loop {
                blink(200, led);
                i = i + 1;
                if i > 2 { break; }
            }
        }
        'o' => {
            let mut i = 0;
            loop {
                blink(600, led);
                i = i + 1;
                if i > 1 { break; }   
            }
        }
        _ => ()
    }
    Ok(())
}

fn main() {
    let my_led = Pin::new(23);
    my_led.with_exported(|| {
        my_led.set_direction(Direction::Out).unwrap();
        loop {
            letter_to_morse(my_led, 's');
            letter_to_morse(my_led, 'o');
            letter_to_morse(my_led, 's');
            sleep(Duration::from_millis(1000));
        }
        Ok(())
    }).unwrap();
}