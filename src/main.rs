use std::{fs, ops::Sub, time::Duration};

use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptionsBuilder, Tab};

fn screenshot_all_tabs(browser: &Browser) -> Result<(), failure::Error> {
    let all_tabs = browser.get_tabs().lock().unwrap();

    for (pos, t) in all_tabs.iter().enumerate() {
        let png_data = t
            .wait_for_element("body")?
            .capture_screenshot(ScreenshotFormat::PNG)?;
        fs::write(format!("screenshot-{}.png", pos), &png_data)?;
    }

    Ok(())
}

fn check_ikea(url: &str) -> Result<bool, failure::Error> {
    let mut opts = LaunchOptionsBuilder::default();
    opts.headless(false);
    let browser = Browser::new(opts.build().expect("error building launch options"))?;

    let tab = browser.wait_for_initial_tab()?;

    println!("Navigating to URL...");
    tab.navigate_to(url)?;

    println!("Visiting Checkout Page...");
    tab.wait_for_element("div[data-testid=\"finalize-btn\"]")?
        .click()?;

    println!("Adding to shopping cart button...");
    let mut add_to_bag_center = tab.wait_for_element("kompis-add-to-bag")?.get_midpoint()?;
    add_to_bag_center.y = add_to_bag_center.y.sub(f64::from(20));
    tab.click_point(add_to_bag_center)?;

    println!("Waiting for success element");
    tab.wait_for_element("kompis-toaster")?;

    println!("Visiting shopping page...");
    tab.wait_for_element("button[aria-label=\"shopping-bag\"]")?
        .click()?;

    let all_tabs = browser.get_tabs().lock().unwrap();
    let last_tab = all_tabs.last().expect("this is the very last tab");

    last_tab.wait_until_navigated()?;
    println!("Waiting for cookie button...");
    last_tab.wait_for_element("body")?;
    println!("Clicking cookies button...");
    let bounds = last_tab.get_bounds()?;
    let x = bounds.width - 80;
    let y = bounds.height - 80;
    let point = headless_chrome::browser::tab::point::Point { x: x, y: y };

    last_tab.wait_for_element_with_custom_timeout(
        ".asdassdasnonexistant",
        Duration::from_secs(60000000),
    )?;

    // println!("Taking screenshots...");
    // screenshot_all_tabs(&browser)?;

    Ok(true)
}

fn main() {
    println!("Checking ikea, give me a minute...");
    let result = check_ikea(
        "https://www.ikea.com/addon-app/storageone/pax/web/latest/de/de/?vpcSource=email#/u/CF8BWP",
    )
    .expect("Error getting result");
    if result {
        println!("Everything is there, go and buy")
    } else {
        println!("Nope, did not work")
    }
}
