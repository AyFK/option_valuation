# Some of my C++ code translated into Rust
## Why?
I wanted to learn Rust, this private C++ project let me try out most of Rusts
features.

## How it works
Not all stochastic volatility models have closed for solutions, this is the
general simulation environment i use to stress test different hedging
strategies for different stochastics pricing processes. To test your own
strategies in your own pricing processes you need to modify the 'mechanics'
and 'dynamics' files respectivly.


## /dynamics
Do you want to test your own stochastic differential equations?
* inside the 'enum_impl.rs' file add a new enum variant with
drift and diffusion paramters for your model along with matching
arms for the enum that calls your custom 'inference' and 'dy'
functions.
* 'mkdir /you_strategy', 'touch dy.rs' and 'touch inference.rs',
dont forget to add a 'mod.rs' in the directory.
* inside 'dy.rs' create a function that returns the solution to
your log-return stochastic differential equation.
* inside 'inference.rs'

## /mechanics
* ...
