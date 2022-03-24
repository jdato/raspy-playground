use sysfs_gpio::{Direction, Pin, Edge};
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
    let my_button = Pin::new(24);
    
    my_led.export();
    my_led.set_direction(Direction::Out);

    my_button.export();
    my_button.set_direction(Direction::In);

    // loop {
    //     match my_button.get_value() {
    //         Ok(val) => {
    //             println!("{:?}", val);
                
    //             if val == 1 {
    //                 blink(100, my_led);
    //                 blink(100, my_led);
    //                 blink(100, my_led);
    //                 blink(100, my_led);
    //                 blink(100, my_led);
    //             }
    //         }
    //         Err(err) => println!("{:?}", err)
    //     }

    //     sleep(Duration::from_millis(100));
    // }

    my_button.set_edge(Edge::RisingEdge).unwrap();

    loop {
        match my_button.get_poller().unwrap().poll(60_000) {
            Ok(val) => match val {
                Some(1) => {
                    println!("{:?}", val);
                    blink(100, my_led);
                },
                _ => ()
            },
            Err(_) => ()
        }
    }

    my_led.unexport();
    my_button.unexport();
}