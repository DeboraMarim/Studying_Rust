# Studying_Rust


<img src="./img.png" style="max-height: 300px"/>

#### This repository brings together my studies in the Rust language. 
#### Study schedule in 90 days

#### start date: 7/24
#### end date: 10/24

## Each topic will be a branch

### Week 1 - Introduction to Rust

#### Days 1 to 3:
- [x] Understanding what Rust is.
- [x] Installing Rust.
- [x] "Hello, World!" in Rust.
- [x] Basic syntax and variables.

#### Days 4 to 7:

- [x] Data types in Rust.
- [x] Strings and String operations.
- [x] Control flow: if, else, loop, while, for.
- [x] First small project: build a program that performs basic mathematical operations.


### Week 2 - Data Structures in Rust


#### Days 8 to 10:

- [x] Tuples and Arrays.
- [x] Vectors.
- [x] Structures.


#### Days 11 to 14:

- [x] Enumerations and pattern matching.
- [x] Common collections.
- [x] Second small project: create a program that manipulates to-do lists.


### Week 3 - Functions and Testing in Rust

#### Days 15 to 17:

- [x] Functions.

#### Days 18 to 21:

- [x] Unit tests.
- [x] Integration tests.
- [x] Third small project: create a quiz application with questions and answers.


### Week 4 - Error Handling in Rust

#### Days 22 to 24:

- [ ] Introduction to error handling.
- [ ] The Result type and the ? operator.
- [ ] Panic!


#### Days 25 to 28:

- [ ] Creating custom error codes.
- [ ] Handling errors in programs.
- [ ] Fourth small project: create a log registration program with error handling.


### Week 5 - Advanced Concepts

#### Days 29 to 31:

- [ ] Deepening Traits.
- [ ] Lifetime and Borrowing.
- [ ] Rust's memory model.


#### Days 32 to 35:

- [ ] Optimization and performance.
- [ ] Macros in Rust.
- [ ] Fifth small project: create an inventory management application with complex functionalities.


### Week 6 to 9 - Practical Projects

#### Days 36 to 63:

- [ ] Creating a basic web server with Rust.
- [ ] Creating an open source code library in Rust and publishing it on Crates.io.
- [ ] Creating a simple language interpreter.
- [ ] Create a CLI (Command Line Interface) application.


### Week 10 to 13 - Advanced Topics and Final Project

#### Days 64 to 90:

- [ ] Concurrency and Parallelism.
- [ ] Networking with Rust.
- [ ] Asynchronous programming with Rust.
- [ ] Final Project: Create a complete REST API with authentication, logging, and data persistence.






```js 

************************************************************************************************************
************************************************************************************************************
        Information obtained from the documentation on 07/25/2023
        https://www.rust-lang.org/learn/get-started


Rust is a high-level systems programming language, secure in terms of concurrency, and with a performance similar to C and C++. Rust is a language that aims to prevent segmentation errors (segfaults) and ensure thread safety, all without the need for an automatic garbage collector. This makes Rust an efficient programming language in terms of resource use, which does not sacrifice ease of programming.

Here are some of the key features of Rust:

Safety without sacrificing performance: Rust has several features, such as compile-time memory management, which ensure the safety of the program without sacrificing performance.

Zero-cost abstractions: Rust allows creating abstractions without performance cost. This allows programmers to write high-level codes that are as efficient as low-level ones.

Efficient memory management: Rust manages memory through an ownership system with a set of rules that are checked at compile time, without the need for a garbage collector.

Concurrency without data races: Rust has strong concurrency control. The language provides an efficient and safe way to handle threads.

Interoperability with C: Rust offers the possibility of interoperating with C code. This is especially useful if you need to integrate your Rust code with an existing C codebase.

Rich tools: Rust comes with an integrated package manager called Cargo. It makes it very easy to download, compile, and manage dependencies for your Rust project.

The Rust language is becoming increasingly popular due to its emphasis on safety and performance, in addition to its active and rapidly growing developer community.

*******************************************************************************
*******************************************************************************
*******************************************************************************


Getting started

*******************************************************************************

You can try Rust online in the Rust Playground without installing anything on your computer.

( https://play.rust-lang.org/?version=stable&mode=debug&edition=2021 )

*******************************************************************************

Rustup: the Rust installer and version management tool
The primary way that folks install Rust is through a tool called Rustup, which is a Rust installer and version management tool.

It looks like you’re running macOS, Linux, or another Unix-like OS. To download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions.

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

*******************************************************************************

if you are on Windows
 See (https://forge.rust-lang.org/infra/other-installation-methods.html) 


*******************************************************************************


 Is Rust up to date?
Rust updates very frequently. If you have installed Rustup some time ago, chances are your Rust version is out of date. Get the latest version of Rust by running rustup update.

*******************************************************************************


Cargo: the Rust build tool and package manager
When you install Rustup you’ll also get the latest stable version of the Rust build tool and package manager, also known as Cargo. Cargo does lots of things:

* build your project with cargo build

* run your project with cargo run

* test your project with cargo test

* build documentation for your project with cargo doc

* publish a library to crates.io with cargo publish


To test that you have Rust and Cargo installed, you can run this in your terminal of choice:

                                        cargo --version

*******************************************************************************
*******************************************************************************
*******************************************************************************


Generating a new project

Let’s write a small application with our new Rust development environment. 
To start, we’ll use Cargo to make a new project for us. 
In your terminal of choice run:

                cargo new hello-rust

This will generate a new directory called hello-rust with the following files:


                                        hello-rust
                                        |- Cargo.toml
                                        |- src
                                            |- main.rs

* Cargo.toml is the manifest file for Rust. It’s where you keep metadata for your project, as well as dependencies.

* src/main.rs is where we’ll write our application code.

*******************************************************************************

* cargo new generates a "Hello, world!" project for us! We can run this program by moving into the new directory that we made and running this in our terminal:

                                        cargo run

You should see this in your terminal:



                                $ cargo run
                                Compiling hello-rust v0.1.0 (/Users/ag_dubs/rust/hello-rust)
                                    Finished dev [unoptimized + debuginfo] target(s) in 1.34s
                                    Running `target/debug/hello-rust`
                                Hello, world!



*******************************************************************************
*******************************************************************************
*******************************************************************************

Adding dependencies
Let’s add a dependency to our application. You can find all sorts of libraries on crates.io, the package registry for Rust. In Rust, we often refer to packages as “crates.”

In this project, we’ll use a crate called ferris-says.

In our Cargo.toml file we’ll add this information (that we got from the crate page):

[dependencies]
ferris-says = "0.2"
We can also do this by running cargo add ferris-says@0.2.

Now we can run:

cargo build

...and Cargo will install our dependency for us.

You’ll see that running this command created a new file for us, Cargo.lock. This file is a log of the exact versions of the dependencies we are using locally.

To use this dependency, we can open main.rs, remove everything that’s in there (it’s just another example), and add this line to it:

use ferris_says::say;
This line means that we can now use the say function that the ferris-says crate exports for us.


*******************************************************************************
*******************************************************************************
*******************************************************************************


A small Rust application
Now let’s write a small application with our new dependency. In our main.rs, add the following code:

use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
    
Once we save that, we can run our application by typing:

cargo run

Assuming everything went well, you should see your application print this to the screen:

----------------------------
< Hello fellow Rustaceans! >
----------------------------
              \
               \
                 _~^~^~_
             \) /  o o  \ (/
               '_   -   _'
               / '-----' \
    