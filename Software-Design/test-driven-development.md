# Test Driven Development

## Rules for Test Driven Development
* **Single assert Rule**: A test must be only one logical assert. Please do not single act rule. There should be one action. And there can many assert statements, but all to validate a single action, and hence one logical assert. **3A Rule**: Arrange, Act, and Assert.
* Avoid accidental complexity. Any system can have complexity, which can be necessary complexity. Any further 
* Wide scope must be avoided. Input should be very well-defined.
* We want test to complete in nano or microseconds.
* Colloquialisms: The test data should communicate the intent of test. Use the words which are have meaningful values.

To solve some of the issues - you can have a
* **Composed assertion**
  * Changes many asserts into one. Hides accidental complexity.
* **Composed test result**
  * A test result that merges many variables into one.
  * Human interpretation should be simple.
  * Changes many asserts into one.

## Stubbing vs Mocking, Testing behavior vs state 
Endo testing

Mock -> Spy -> Stub -> Dummy -> Test Doubles
Fake -> Testing

Spies: Stub that remembers facts about the method calls.
Mock: Spy that know how methods should be called.
