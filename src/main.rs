use sysfs_gpio::{Direction, Pin, Edge};
use tokio::join;
use std::thread::sleep;
use std::time::Duration;
use warp::Filter;

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

async fn blinker() -> Result<(), ()> {
    let my_led = Pin::new(23);
    let my_button = Pin::new(24);
    
    my_led.export();
    my_led.set_direction(Direction::Out);

    my_button.export();
    my_button.set_direction(Direction::In);

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
    Ok(())
}

async fn run_server() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    // let hello = warp::path!("door" / String)
    //     .map(|name| format!("Hello, {}!", name));

    let my_button = Pin::new(24);

    my_button.export();
    my_button.set_direction(Direction::In);

    my_button.set_edge(Edge::RisingEdge).unwrap();

    warp::serve(warp::any().map(move || {
        let door_state = match my_button.get_value() {
            Ok(val) => {
                if val > 0 {
                    "closed"
                } else {
                    "open"
                }
            },
            Err(_) => {
                "broken"
            }
        };
        format!("Door Checker.\n\nDoor is: {}", door_state)
    }))
        .run(([0, 0, 0, 0], 8080))
        .await;
}

#[tokio::main]
async fn main() {
    let blinker_future = tokio::spawn(async {
        // blinker().await;
    });

    let tide_future = tokio::spawn(
        async {
            run_server().await;
        }
    );
    
    join!(
        blinker_future,
        tide_future
    );
}