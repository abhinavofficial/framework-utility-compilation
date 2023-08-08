# Clean Code

## What does it mean to be clean?
Implementation patterns by Kent Beck.

The productivity degrades with time. The change of scope is no longer proportional to time it takes to change it.

Once the code is dirty enough, even the smartest of coders run with Xeno's paradox. The new system never gets deployed. The company continues to support two development teams. Incremental redesigning may work.

> The only way to go fast is to go well.

### Salient Features
* **Elegant and efficient**: Clean code does one thing and does it well, as stated by Bjarne Stroustrup. One thing - ??
* **Simple, Direct, Prose**: Clean code is simple and direct. Clean code reads like well-written prose... as stated by Grady Booch.
* **Care**: Michael Feathers states clean code always like it was written by someone who cares about the reader.
* **What you expected**: Ward Cunningham, creator of wiki states that you know you are working on clean code when each routine you read turns out to be pretty much what you expected.

### The Boy Scout Rule
* It is not enough to write the code well.
* The code has to be kept clean.
* Leave the campground cleaner than you found it. Check the code in cleaner than you checked out.

## Clean Code: Functions
* **Naming**: Functions do things and their name should have verb name.
* **Shifts between high level to low level code**: The code should NOT be jumping from high level to low level too quickly and too frequently. This does not allow reader to prepare the mental model of the program.