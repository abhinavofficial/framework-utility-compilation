/** Created to handle learning pace, re-calibrate speed and then ensure that I am
able to catch up with random reads in a meaningful way
*/

/** How does this program run?
Program assumes that there is a README.md at the base project directory.
It has links to other markdown files, lets call it month-learning-file.
Month-learning-file contains learning topic in H2 header (##) followed by short description
of the topic. The description will have two mandatory lists (**Focus Before**) and (**Study Concluded**).
These two lists denotes when the learning is due by and if it is concluded. If the learning is
not concluded and due date is approaching,
- send email to apache.abhinav.official@outlook.com notifying of the fast approaching date.
- push the due date by 2 weeks automatically if learning is not concluded by the end date.

These actions should run when we execute cargo run for simple implementation.
*/

/**
Key learning to demonstrate
1. Validate before use
2. Modularize functionality
3. Generalize as far as possible. Example, allow the format as template
4. Build in event driven - action based pattern
5. Use a database (embedded for now) to capture each run
*/

fn main() {
    println!("Hello, world");
}