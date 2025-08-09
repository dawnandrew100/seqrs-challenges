# seq.rs-challenges

Welcome to **seq.rs-challenges**!
This repo is a collection of monthly coding challenges
focused on **bioinformatics in Rust** 🦀.

This repository serves as the home for:

- **Monthly bioinformatics challenges** posted on [seq.rs](https://dawnandrew100.github.io/seq.rs/)
- Community Rust solutions
- Discussions around performance, algorithms, and problem-solving in Rust

Whether you're a beginner to Rust or an experienced bioinformatician,
these challenges are a great way to sharpen your skills!

## How It Works

1. **New Challenge Every Month**
   A new bioinformatics coding challenge is published at the start of
   each month.

2. **Solve in Rust**
   Use idiomatic, performant, and readable Rust to tackle the problem.

3. **Submit Your Solution**
   Fork this repo, create a folder under the current month's directory using
   your GitHub username, add your solution, and open a pull request.
   See full [submission instructions](#submission-instructions) below.

4. **Join the Discussion**
   Share your approach, and ask questions in the
   [GitHub Discussions](https://github.com/dawnandrew100/seqrs-challenges/discussions).

## Submission Instructions

1. Fork this repository
2. Navigate to the appropriate challenge folder `year/month/`
3. Create a new folder in `submissions/`

    ```bash
    cargo new <your_github_username>
    ```

4. Inside your newly created project folder, update your `Cargo.toml` to include
the shared tests crate as a dev-dependency.

    ```bash
    [dev-dependencies]
    tests = { path = "../../../tests" }
    ```

    *(Adjust the relative path as needed)*

5. Add your solution source files inside the `src/` folder (you can add extra binaries
under `src/bin/` if needed).

    ```bash
    src/2025/07july/submissions/alice/
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs
    │   ├── main.rs
    │   └── bin/
    │       └── part1.rs

    ```

6. Write your tests using the shared `tests` crate to ensure consistency and correctness.

    `src/lib.rs`

    ```rust
    pub fn do_stuff(num: i32) -> i32 {
        num + 1
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use tests::{common_input, assert_correct};

        #[test]
        fn test_do_stuff() {
            let input = common_input();
            // Example usage (adjust as needed)
            assert_eq!(do_stuff(input), 42);
        }
    }
    ```

    `src/main.rs`

    ```rust
    use your_crate_name::do_stuff;

    fn main() {
        let val = 41;
        println!("Result: {}", do_stuff(val));
    }
    ```

    Tests can be run with `cargo test` and `main.rs` can be run with `cargo run`

7. Open a Pull Request with the title: `Add solution for <MONTH-YEAR> by <USERNAME>`
8. (Optional) Include a short write-up in a `README.md` inside your folder

## About seq.rs

[**seq.rs**](https://dawnandrew100.github.io/seq.rs/) is a blog
exploring algorithms, tools, and experiments in
**bioinformatics using Rust**.

Each monthly challenge is inspired by topics I find interesting,
recent papers in the bioinformatics field, or community suggestions.
