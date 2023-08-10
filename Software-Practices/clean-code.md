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

A bad comment is when it is -
* **Mumbling**: When the author is commenting to convince himself, rather than the reader of the code. This is called .
* **Redundant**: It takes longer to read and understand the comment than it takes to read and understand the code. It is worse if it does not offer more insight to the reader than the code itself.
* **Misleading**: If the comment does not fully align to the code it is addressing, it may lead to confusion.
* **Journal**: We do not need to write down the change control document in the code file - source code management system tracks these changes.
* **Noise**: Valueless comment. It gets scary if you have copy-pasted and forgot to reflect what is there in the code.
* **Commented-out Code**: Generally we should not check in commented out code in the main branch, which other developers or reader are going to check out.
* **Non-local information**: If you must write a comment, make sure it describes code adjacent to the comment. If you mention something that is not around geo-graphically, it can get hard for readers to trust.
* **Too much information**

Sometimes, the mandated comment like every function must have a javadoc, or each function parameter needs to have comment, etc. Comments like these clutter the code and propagate lies, confusion, and disorganization.

> Use explanatory code, not comment.

## Clean code: Meaningful Names
The most meaningful work on this came from Tim Ottinger - link describing is  https://exelearning.org/wiki/OttingersNaming/

Since names are everywhere - files, directories, programs, classes, variables, arguments, etc. - we better do it well!

* Naming Rule:
  * The name of a variables should correspond to the size of its scope.
      * Short variable names for small scopes
      * Long variable names for larger scopes (example- global variable > instance variable > local variable > inline variable)
  * The name of a function should correspond inversely to the size of its scope.
      * Short function names for big scopes
      * Long function names for small private scopes. (public < instance method < private)
  * The name of the class follows the same rule as functions.
  * The name of properties follows the same rule as variables (even though they are really function)
* A good name reveals intent.
* The name should avoid disinformation.
  * False Clues
  * Disambiguate - similar looking variable names or symbols. Lower case of 1, l are similar looking as well.
* We should use meaningful name to distinguish for its uniqueness. 
  * Example, klass, aClass, theClass. 
  * Number series does not typically help - a1, a2 instead of source, destination.
  * Functions like getAccountAccount(), getActiveAccounts(), and getActiveAccountInfo() in the same context is confusing and not meaningful.
* We must avoid convenient misspellings. Avoid situations where code will break if a spelling error is fixed.
* Avoid noise or redundant noise.
* Keep the name pronounceable.
* Avoid encoding within name
  * Hungarian notation: Invented by Charles Simonyi. This was helpful in the 1980s, not anymore in the statically typed language. It may still be helpful in dynamically typed language like Python or Clojure.
  * Local and member prefixes.
  * The Irrepressible I (in Interface)
* We should avoid where people have to create mental map to understand the code. For example,
  * Something that forces translation: use price instead of amount, or use momentum instead of p.
* Class names should be nouns (and not plural!). Avoid words like Manager, Processor, Data, Info, etc.
* Method names should be verb or verb phrases. You can use accessors or mutators like get, set, is, etc. You can use javabean standard.
* In case of overloaded constructor, it is better to use factory method to use different constructors. You can consider enforcing them by making constructors private. For example, `Complex.ForRealNumber(23.4)` is better than `new Complex(23.4)`
* DO NOT BE CUTE.


## Further reading
Domain driven design by Eric Evans
The annotated Turing by Charles Petzold
