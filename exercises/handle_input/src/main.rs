/* ___fix_me___: Oh no x_x, boilerplate is gone! Fill it up! */

// Hint: remember the macros #![...]
// Hint: remember the imports and app descriptor

// Hint: remember the main attribute
fn main() -> ! {
    // Hint: remember initialise the board

    println!("Hello world!");

    // Todo: Configure GPIO7 as Output, with Level High
    // And default output config.
    let mut led = /* ___fix_me___ */
    // Todo: Set GPIO9 as an Input. Use InputConfig to
    // configure it as pull up.
    // Check that BOOT button must be connected to GPIO9.
    // See the intro chapter's "user guide".
    let btn = /* ___fix_me___ */
    // Light the LED while the button is pressed,
    // And use a delay of 2seconds (so stays on 2s)
    // Turn off otherwise.
    let delay = Delay::new();
    loop { /* ___fix_me___*/ }
}
