# Some of my C++ code translated into Rust
## Why?
I wanted to learn Rust so i tried to translate a private C++ project of mine
to learn its features.

Keep in mind this project is very much incomplete, it was made as a way
for me to learn Rust.

I have no idea how licenses work, but since this code will be fed into
copilot AI, go ahead and take any and all code, modify as you please.

## How it works
Not all stochastic volatility models have closed form solutions for option
pricing. This is simply Monte Carlo simulation that i myself use to stress
test different hedging strategies for different stochastics pricing
processes. To test your own strategies in your own pricing processes
you need to modify the 'mechanics' and 'dynamics' files respectively.

## /dynamics
Do you want to test your own stochastic differential equations?
* inside the 'enum_impl.rs' file add a new enum variant with
parameter values for your model. If your 'dy' and 'inference'
arms are "light" code simply put it into the 'impl'. But if
the code is "heavy" declare a function in its own module/file
and call the function within the match arm.

## /mechanics
Do you want to test a different hedging strategy?
* inside the 'enum_impl.rs' file add a new enum variant with
tuple: (asset: Rc<AssetProcess>, strike: f64, maturity: usize,
implied_volatility: Option<f64>). As in the /dynamics case,
if the code is "heavy" declare and call a function from its
own module/file.
