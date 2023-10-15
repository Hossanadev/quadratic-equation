This is a math solution to the popular "Alimighty Formula"; a quadratic formula helps us solve any quadratic equation.
`ax^2 + bx + c = 0`

This solution is written in Rust programming by Hossanadev.

How it works:

The program makes use of command line arguments (args).

- It accepts 3 args, a, b, and c respectively.
- To run this program, install rust.
- Open this file, run this command:
  Example: assuming a is 1, b is 4 and c is 1;
  you run:
  `cargo run 1 4 1` then you'll see the result.

  Note: If the value of the discriminant is less than 0.0, you'll see `No real roots exist.` as a response.

  If the length of agrs passed is less than or greater than 3, example: `cargo run 1.0 3.8` or `cargo run 1.0 2.0 3.0 4.0` you'll get an error as response!
