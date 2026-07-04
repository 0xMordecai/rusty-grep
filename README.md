# rusty-grep
this project will needed background knowledge to understand a real-world project such as `ripgrep`.

Along the way:
  - we’ll show how to make our command line tool use the terminal features that many other command line tools use.
  - We’ll read the value of an environment variable to allow the user to configure the behavior of our tool. 
  - We’ll also print error messages to the standard error console stream (`stderr`) instead of standard output (`stdout`) so that, for example, the user can redirect successful output to a file while still seeing error messages onscreen.


this project will combine a number of concepts :
  - Organizing code
  - Using vectors and strings
  - Handling errors
  - Using traits and lifetimes where appropriate
  - Writing tests

## Separating Concerns in Binary Projects
The organizational problem of allocating responsibility for multiple tasks to the `main` function is common to many binary projects. As a result, many Rust programmers find it useful to split up the separate concerns of a binary program when the `main` function starts getting large. This process has the following steps:
  
  - Split your program into a main.rs file and a lib.rs file and move your program’s logic to lib.rs.
  - As long as your command line parsing logic is small, it can remain in the `main` function.
  - When the command line parsing logic starts getting complicated, extract it from the `main` function into other functions or types.

The responsibilities that remain in the main function after this process should be limited to the following:
  
  - Calling the command line parsing logic with the argument values
  - Setting up any other configuration
  - Calling a `run` function in lib.rs
  - Handling the error if `run` returns an error

This pattern is about separating concerns: **main.rs** handles running the program and **lib.rs** handles all the logic of the task at hand. Because you can’t test the `main` function directly, this structure lets you test all of your program’s logic by moving it out of the `main` function. The code that remains in the `main` function will be small enough to verify its correctness by reading it. Let’s rework our program by following this process.
