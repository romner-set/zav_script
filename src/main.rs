use std::{time::Duration, thread, io::{self, Write}};
use headless_chrome::{Browser, LaunchOptionsBuilder};
use sysinputs::keyboard::{Key, Physical};

const LOGIN: &str = "https://student.zav.cz/#!/login";
const NEXT: &str = "div.zav-fixed-height > lesson-panel > div > div > div.col-6.d-flex.justify-content-end.align-items-center.p-0 > button:nth-child(2)";

fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Opening chrome...");

    let browser = Browser::new(LaunchOptionsBuilder::default()
        .headless(false)
        .path(match std::env::args().nth(1) {
            Some(path) => Some(path.into()),
            None => None,
        }).build()?
    )?;
    let tab = browser.wait_for_initial_tab()?;
    tab.set_default_timeout(Duration::MAX);

    println!("Opened, navigating to {}", LOGIN);
    
    tab.navigate_to(LOGIN)?.wait_until_navigated()?;
    println!("Waiting for login.");

    loop {
        match tab.wait_until_navigated()?.get_url().as_str() {
            LOGIN => (),
            "https://student.zav.cz/#!/courses" => {
                println!("Success, waiting for course selection.");
                let input = tab.wait_for_element("div > text-input > div#textInput")?;
                println!("Selected. Typing, don't move your mouse...");
                //thread::park();

                input.click()?;
                
                let mut i = 0;
                let mut last = 0;
                loop {
                    let elem = tab.find_element("div#scrollableText > div > p")?.get_description()?.children.unwrap();
                    let mut spaces = elem[0].node_value.match_indices(' ');
    
                    if let Some((j, _)) = spaces.nth(i) {
                        let text = &elem[0].node_value[last..j];
                        last = j;
    
                        print!("{}", text); io::stdout().flush()?;
        
                        sysinputs::keyboard::send_str(text);
                        i+=1;
                    } else {
                        //println!("\nEND");

                        let text = &elem[0].node_value[last..];

                        for ch in text.chars() {
                            sysinputs::keyboard::send_key(if ch == '¶' {
                                Key::Physical(Physical::Return)
                            } else {
                                Key::Unicode(ch)
                            });
                        }

                        println!("{}", text);

                        thread::sleep(Duration::from_secs(1));
                        if tab.find_element("div#scrollableText > div > p")?.get_description()?.children.unwrap()[0].node_value.match_indices(' ')
                        .nth(i).is_none() {break}
                        //tab.find_element(NEXT)?.click()?;
                        //thread::park();
                        //break
                    };

                    //if let Ok(b) = tab.find_element(NEXT) {println!("Selecting next course."); b.click()?; break;}
                    //else {println!("{:#?}", tab.find_element(RESULTS)?.get_description()?.children.unwrap());}

                    //thread::sleep(Duration::from_millis(50));
                }
            },
            _ => {
                println!("Please login.");
                tab.navigate_to(LOGIN)?;
            }
        }
    }
}
