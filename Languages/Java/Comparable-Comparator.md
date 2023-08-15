# Comparable and Comparator in Java
Comparable and comparator are both interfaces in the Java programming language to determine the behaviour in which classes will be implemented. While Comparable is used on objects that are naturally ordered, the Comparator interface implements sorting by taking the attributes of an object into consideration. Further, Comparator sorting takes into account objects of two different classes and Comparable compares objects using the “this” reference. 

## When to use what?
**Comparator**: Use it if you want to sort the elements as per the default natural sorting order. The reason is if you choose the default natural sorting order, the object must be homogeneous as well as comparable. It is essential to note that an object is considered comparable if its class uses the Comparable interface. By default, all the String and Wrapper classes implement the Comparable interface. Hence, if you need to sort the String element, you need not implement a Comparable interface.

**Comparator**: The comparator interface helps to order the objects of the user-defined classes. The corresponding comparator object can compare two objects of the same class. For example, you have an Array/ArrayList of a class consisting of fields like name, address, roll no, DOB, etc. Now you want to sort the array depending on the name or Roll no. With the help of a comparator interface, you can order the objects present in a user-defined class.

If you have a **fixed requirement**, for example, the class objects would be sorted depending on a single field of the class, and also anticipate the same in the future, and this requirement will not change, then it is recommended to **use the comparable interface**.  If there is a **requirement for sorting class objects through multiple fields**, then it is recommended to **use multiple comparator class explicitly**.

A **comparator class provides flexibility**, which means you can create multiple comparator classes and also use them to override objects. On the other hand, a **comparable interface is fixed**. This is an important point to note when comparing comparator vs comparable in java.

## Difference Between Comparable and Comparator

### Method of Sorting
Comparable uses the compareTo() function for sorting. This is the only method present in the Comparable sorting interface. `compareTo(Object O)` takes as argument an object and compares it to another of the same type. If an object is a string, you can only compare it to another object of the same type. The same is the case with an int object and so on. compareTo() returns a negative, positive or zero integer if this object’s value is lesser than, greater than, or equal to that of the passed object respectively.

Comparator has two methods to sort elements in Comparator: `compare()` and `equals()`. `compare(Object O1, Object O2) `takes into account two arguments as input and provides the desired output and returns an integer to indicate how the first argument compares with the second. It returns a negative, positive or zero integer if object1's value is lesser than, greater than, or equal to that of the O2 object respectively. `equals(Object)` takes into account an object as input and compares it to the comparator. It returns True if the Object is equal to the Comparator. It also ensures that the order does not change.

> Note: Overriding equals( ) is useless, and the majority of simple comparators don’t use it.

### Package Used
Comparable exists in the java.lang package of Java, while Comparator exists in the java.util package of Java.

### Ordering and Class
* Comparable
  * It takes into account objects that follow natural ordering i.e. have a tendency to sort themselves. For example, alphabetical order or numerical order in case of names, price, salary, roll number, age, etc.
  * It is also important that both objects belong to the same class.
  * Comparable interface compares “this” reference with the object specified.
  * Sorting using Comparable affects the actual class.
* Comparator
  * This interface is used primarily to sort attributes of specified objects. They do not need to have a natural order; it can be customised.
  * For the Comparator interface, you have to override the method compare(obj).
  * The logic of sorting is required to be in separate classes to compare the attributes of the objects of the two classes in consideration. 
  * The actual class remains unaffected.

### Sequences and Collections
* Comparable
* It supports single sorting sequences only. This implies that you can only consider one element or attribute of a collection such as a roll number or age or rank.
* For sorting a collection of objects, arrays or list, you can use Collection. sort(List) or Arrays. sort(List). This will bring the objects in their natural order.
* Comparator 
* It supports multiple sorting sequences. This implies that you can consider multiple elements or attributes of a collection such as a roll number, age and rank.
* You can use a collection.sort(list, comparator) to sort a collection.
* Other points of difference between comparator and comparable:When it comes to comparator vs comparable, a fundamental differentiating feature is that when using comparable, you can only use one comparison. On the other hand, you can write more than one custom comparator according to your needs, for a specific type. In that case, all of the custom comparators will use unique interpretations of sorting. For example, in the comparative example, you can sort by only one attribute, but in the comparator, you can use different attributes.Another difference between comparator vs comparable is that the comparable interface can be used to offer a single way of sorting, but the Comparator interface is used to offer various ways of sorting.Class is required for implementation when using Comparable. On the other hand, when using the Comparator, you need not make any modifications in the class.Another difference between comparator and comparable is based on how the interface is implemented. A comparable interface is implemented by all the String class and wrapper classes. Moreover, custom objects use a comparable interface for sorting. On the other hand, the comparator interface is chiefly used to sort the custom objects. You can use it to compare the objects of different classes too. Furthermore, the comparator interface is useful for sorting objects in multiple fields, refelcting the key differences between comparable vs comparator in java.