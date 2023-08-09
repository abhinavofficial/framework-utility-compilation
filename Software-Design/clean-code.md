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
* Leave the campground cleaner than you found it. Check the code in cleaner than you checked out. Make a code work, and then clean it, and then check in.

## Clean Code: Functions
* **Naming**: Functions do things and their name should have verb name.
* **Shifts between high level to low level code**: The code should NOT be jumping from high level to low level too quickly and too frequently. This does not allow reader to prepare the mental model of the program.
* Keep the function small. Functions should hardly ever be more than 20 lines long.
* Function should be doing ONE thing. One way to achieve this is - if we cannot further extract another function from the function in context "meaningfully", then your function must be doing one thing only. This also eliminates shifts between high level to low level code, since all your function codes should typically be at the same level, and just one level low from the function name. The name of the function can also act as a comment. You do not need to comment anymore.

## Clean Code: Comments
* Nothing can be quite so helpful as a good comment.
* Nothing can be quite so obscuring as a bad comment.
* Comments are not "pure good".

> The proper use of comments is to compensate for our failure to express ourselves in code. Every use of a comment represents a failure. So, don't comment first. Try everything else, then comment as a last resort.

The above is true for every programming language, like Java, Scala, Python, Go, C++, etc. We may need comment for C, Fortran, Assembly, etc.

Comment should be deleted else it starts to rot, since no one may be paying attention. It may confuse readers.

We should not delete a comment, if it is used for -
* License or legal purposes
* Information sharing. When comments acts as documentation. Example:
```java
import java.util.regex.Pattern;
class Test {
    // format matched hh:mm:ss EEE, MMM dd, yyyy
    String regex_string = "";
    Pattern timeMatcher = Pattern.compile(regex_string);
}
```
* Showing the intent of the coder
* Clarification of why something is written against general practice.
* Issuing warning of consequences.
* TODO list
* Amplification. For example
```java
class Test {
    String listItemContent = match.group(3).trim();
    // this trim is really important. It removes the starting spaces that could cause the item to be recognized as another list.
}
```

Javadocs should be used for PUBLIC APIs, but not for internal / private APIs.

A bad comment is -
* when 