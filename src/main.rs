use std::{time::Duration, thread, io::{self, Write}};
use headless_chrome::{Browser, LaunchOptionsBuilder};
use sysinputs::keyboard::{Key, Physical};

const RESULTS: &str = "#views-placeholder > div:nth-child(3) > div.d-flex.h-100.flex-column.bg-white.border-top.border-primary.border-width-4 > div.zav-course-content > view4 > div.zav-fixed-height > lesson-panel > div > div > div.col-6.pr-5.d-flex.flex-wrap.align-content-start.h-100";

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

    println!("Opened, navigating to https://student.zav.cz/#!/login");
    
    tab.navigate_to("https://student.zav.cz/#!/login")?.wait_until_navigated()?;
    println!("Waiting for login.");

    loop {
        match tab.wait_until_navigated()?.get_url().as_str() {
            "https://student.zav.cz/#!/login" => (),
            "https://student.zav.cz/#!/courses" => {
                println!("Success, waiting for course selection.");
                let input = tab.wait_for_element("div > text-input > div#textInput")?;
                let results = tab.find_element(RESULTS)?;
                println!("Selected. Typing, don't move your mouse...");

                input.click()?;
                'outer: loop {
                    let mut i = 0;
                    let mut last = 0;
                    loop {
                        let elem = tab.find_element("div#scrollableText > div > p")?.get_description()?.children.unwrap();
                        let mut spaces = elem[0].node_value.match_indices(' ');
        
                        let text = if let Some((j, _)) = spaces.nth(i) {
                            let ret = &elem[0].node_value[last..j];
                            last = j;
                            ret
                        } else {
                            let text = &elem[0].node_value[last..elem[0].node_value.len()-2]; //'¶' has 2 bytes
                            println!("{}", text);
            
                            sysinputs::keyboard::send_str(text);
                            sysinputs::keyboard::send_key(Key::Physical(Physical::Return));
                            thread::sleep(Duration::from_millis(100));
                            break
                        };
        
                        print!("{}", text); io::stdout().flush()?;
        
                        sysinputs::keyboard::send_str(text);
                        i+=1;

                        if !results.get_description()?.children.unwrap().is_empty() {break 'outer}
                        //else {println!("{:#?}", tab.find_element(RESULTS)?.get_description()?.children.unwrap());}

                        thread::sleep(Duration::from_millis(50));
                    }
                }
                break;
            },
            _ => {
                println!("Please login.");
                tab.navigate_to("https://student.zav.cz/#!/login")?;
            }
        }
    }
    
    thread::park();

    Ok(())
}
