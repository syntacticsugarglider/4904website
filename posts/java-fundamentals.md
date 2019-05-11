<!--
Java Fundamentals
2019-3-24 12:00
wiki,programming,java
Team 4904 uses Java with the programming library, WPILib, to program robots for FRC competitions. Checkout the guide for WPILib [here](WPILib-Basics). Java is an compiled, object oriented language distributed by Oracle. This guide was made to explain several commonly used features of Java for FRC programming on Team 4904. Feel free to scan through it to understand or review Java concepts.
-->

<a name="java-fundamentals-java-concepts-for-frc"></a>

# Java Fundamentals: Java Concepts for FRC
 
<a name="table-of-contents"></a>

## Table of Contents 
- [Java Fundamentals: Java Concepts for FRC](#java-fundamentals-java-concepts-for-frc)
  - [Table of Contents](#table-of-contents)
  - [Java Intro](#java-intro)
  - [Syntax](#syntax)
  - [Variables](#variables)
  - [Classes](#classes)
    - [Fields](#fields)
    - [Constructors and Instances](#constructors-and-instances)
    - [Methods](#methods)
    - [Inheritance](#inheritance)
    - [Interfaces](#interfaces)
    - [Modifiers](#modifiers)
      - [Access Modifiers](#access-modifiers)
        - [Public](#public)
        - [Private](#private)
        - [Protected](#protected)
      - [Non-access modifiers](#non-access-modifiers)
        - [Static](#static)
        - [Final](#final)
        - [Abstract](#abstract)
        - [Synchronized and Volatile](#synchronized-and-volatile)
  - [Challenges and Practice](#challenges-and-practice)

<a name="java-intro"></a>

## Java Intro

Team 4904 uses Java with the programming library, WPILib, to program robots for FRC competitions. Checkout the guide for WPILib [here](WPILib-Basics). Java is an compiled, object oriented language distributed by Oracle. This guide was made to explain several commonly used features of Java for FRC programming on Team 4904. Feel free to scan through it to understand or review Java concepts.

Install the Java 8 JDK (Java Developer Kit) by running:

    $ brew tap caskroom/versions
    $ brew cask install java8

You can now program in Java!

<a name="syntax"></a>

## Syntax

Java syntax is very similar that of other programming languages. Every line is ended with a semicolon, which lets the compiler know that a declaration is complete. To create [methods](#methods) and [classes](#classes), the [modifier](#modifiers), name, and inputs must be present, along with curly braces to signify the beginning of the command. More in depth syntax is mentioned in each section.

<a name="variables"></a>

## Variables

Variables store data. They can hold anything from numbers to strings (collection of characters, like words) to true or false. There are many different data types, but the most important ones for FRC, in no particular order, are:

- int
    - The integer type is used to hold integers. Integers are numbers that can be represented without the use of decimals.
- double
    - Doubles hold numbers that can't be represented by the integer type.
- string
    - As mentioned earlier, strings contain a sequence of characters. This can be a sequence of letters, numbers, and any other characters. They are enclosed by quotation marks as so "hi".
- boolean 
    - A boolean can either hold the value true or false.
- array
    - Can hold multiple values of any data types

If you want to learn about other data types in java, checkout out [this](https://docs.oracle.com/javase/tutorial/java/nutsandbolts/datatypes.html). All of these data types are called **primitive** data types. There are other more advanced data types which you can checkout [here](https://sites.google.com/site/javatutorialbynkraju/data-types-in-java). Some of them are covered in this wiki.

To declare a variable, you need to declare the type of the variable and a name for the variable.

    <type> variableName;

Often times, the value of a variable is assigned when it is declared as so:

    <type> variableName = <value>;

Example:

    string teamName = "Bot-Provoking";
    int teamNumber = 4904;

In most cases, you will be declaring variables in classes, which are coming up next.

<a name="classes"></a>

## Classes

Classes are one of the main concepts of java, since it is an object oriented language. 

<a name="fields"></a>

### Fields

Fields are variables in classes. To declare a field in a class, you not only need to declare the type and variable name, but also an [access modifier](#access-Modifiers). For example, let's create a class called `Point`, to represent a 2D point on the cartesian coordinate system. Just like before, we need to declare the class:

    public class Point {
        public double x = 0;
        public double y = 0;
    }

In this example, the class `Point` has two fields, `x`, and `y`, which represent the x and y coordinate of the point. The first word, `public`, is an [access modifier](#access-Modifiers), which allows the field to be accessed by any other class (more on this in the modifier section). The type, `double`, and variable names are declared the same way they are declared for normal variables. What if we wanted to create `Point` variables with different coordinates? 

<a name="constructors-and-instances"></a>

### Constructors and Instances

Now that we have the foundation of the stuff in a class, fields, we need to actually create an instance of a class. An instance is a variable of the type of the class it was made from. For example, if we wanted to create a `Point` instance, it would be a variable of the type `Point`, and would have the fields, `x` and `y`. To create an instance, we need the class to have a constructor. A constructor details what input is needed to create a such instance. Let's add a constructor to the `Point` class.

    public class Point {
        public double x;
        public double y;

        // constructor starts below
        public Point(double x, double y) {
            this.x = x;
            this.y = y;
        }
    }

This code snippet introduces a few new concepts. First, notice that the fields, `x` and `y`, are no longer set to a value in this example. Instead, they are set instead the constructor. The constructor, which needs to have the same name as the class, specifies what input is needed to create an instance of the class. In the case of the class `Point`, the constructor requires a double `x`, and a double `y`, to create a `Point` instance. Inside the constructor, the fields `x` and `y` are set to the inputs of the constructor. The keyword, `this`, points to the `Point` class itself. The first line in the constructor is setting the field `x` in the `Point` class, declared in line 1, to the input of the constructor.

Now that the Point class has a constructor, we can create `Point` instances as follows:

    Point pt = Point(1,2);
    Point pt2 = Point(0.5, 10);

As you can see, the declaration of a `Point` instance is very similar to declaration and assigning of a variable.

<a name="methods"></a>

### Methods

Methods are functions that act on fields in a class. Simply put, methods take in input, and modify fields in the class accordingly. Let's add some methods to the `Point` class:

    public class Point {
        public double x;
        public double y;

        // constructor starts below
        public Point(double x, double y) {
            this.x = x;
            this.y = y;
        }

        // distance method
        public double distance_squared(Point pt) {
            double xDist = (this.x - pt.x)**2;
            double yDist = (this.y - pt.y)**2;
            return xDist + yDist;
        }

        public double getX() {
            return this.x
        }
    }

Now the class `Point` has a method `distance_squared`. The method is declared [public](#public), so it can be accessed by any class, returns a double, and takes in a `Point` instance as its input. Every method requires an access modifier, like public, its return datatype, and the arguments it takes in as inputs.

General Form:

    <modifier> <returntype> <methodName>(args) {
        //logic
        return val;
    }

<a name="inheritance"></a>

### Inheritance

In Java, classes can be subclasses of other classes, called superclasses. Subclasses inheirit the fields, methods, and constructors of their superclass. This way, you don't need to rewrite code for two similar classes. In Java, every class can only have one superclass (try thinking about why this is). Using the Point example, let's say we want to create a class called `ThreeDPoint`, to represent 3D points. Since 3D points are just 2D points with another dimension, we can make `ThreeDPoint` a subclass of `Point`. We can also use a similar constructor to the one in `Point`.

    public class ThreeDPoint extends Point {
        public double z;
        public ThreeDPoint (double x, double y, double z){
            super(x, y);
            this.z = z;
        }
        @Override
        public double distance_squared(ThreeDPoint pt) {
            double xDist = (this.x - pt.x)**2;
            double yDist = (this.y - pt.y)**2;
            double zDist = (this.z - pt.z)**2;
            return xDist + yDist + zDist;
        } 

    }

In this example, we have a couple of new concepts. First, the `extends` keyword signifies that `Point` is the superclass of `ThreeDPoint`. Next, the constructor takes in three values, and uses the `super` keyword with the arguments. In this case, the `super` keyword is invoking the constructor of the superclass, `Point`, with the arguments, `x` and `y`. Finally, the `distance_squared` method is overriden with `@Override`. This overrides the superclass's method `distance_squared`, and replaces it with a custom version for `ThreeDPoint`. The class `ThreeDPoint` retains the method `getX` from before, and can still use it.

<a name="interfaces"></a>

### Interfaces

Java classes cannot have more than one superclass, but they can achieve a similar effect with Intefaces. Interfaces are essentially templates for a class. They specify fields and methods that a class implementing them must have, but do not implement the methods themselves. Any class that implements a interface must include all fields and methods that the interface specifies. For an example and more documentation on Interfaces, checkout [this](https://www.geeksforgeeks.org/interfaces-in-java/)

<a name="modifiers"></a>

### Modifiers

Modifiers are keywords (words that the compiler reserves for specific use) that, as the name suggests, modify the declaration of what they are modifying. Modifiers are mainly used when declaring classes, fields, methods, and constructors. There are two groups of modifiers: access modifiers and non-access modifiers.

<a name="access-modifiers"></a>

#### Access Modifiers

As the name suggests, access modifiers change the access permissions of the declaration they are modifying. There are three accesss modifiers:

<a name="public"></a>

##### Public
  
Public declarations are accessible by the entire code base. Any other parts of a program can access the thing declared. Consider the previously used class, Point.

Example Class:

    public class Point {
        public double x;
        public double y;
        public Point (double x, double y){
            this.x = x;
            this.y = y;
        }

    }

Because the `x` and `y` fields of a Point are `public`, they can be accessed by the rest of program.

Accessing X:

    Point pt = Point(1,2);
    x = pt.x;

The Point pt is an [instance](#Instances) of the Point class. 

In general java programming, most fields should be declared private, so they cannot be accessed by other classes (you don't want important data taken). However, for robotics, many fields are motors, solenoids, or commonly used constants, so they are often declared public.

<a name="private"></a>

##### Private
  
The `private` modifier is the opposite of the `public` modifier. Anything declared private can only be accessed by the class that contains it, and subclasses (a class in the class). Here is the same Point class from before, but with some fields declared private.

    public class Point {
        private double x;
        private double y;
        public Point (double x, double y){
            this.x = x;
            this.y = y;
        }
        public double getX(){
            return this.x;
        }
    }

In this case, the rest of the program would not be able to access the fields `x` and `y` of a Point Instance. The example access code from before:

    Point pt = Point(1,2);
    x = pt.x;

would not work. Instead, it is necessary to use a public method of the class, in this case the method `Point.getX()`. To get the value of `x` from a Point Instance, we need to call this public method as so:

    Point pt = Point (1,2);
    x = pt.getX();

<a name="protected"></a>

##### Protected

Protected is the middle ground between public and private declarations. Protected fields and methods can be accessed by the class and its subclasses, and any classes in the same packages (a special enclosing folder). The `protected` modifier isn't used too often in our FRC code, so I won't bore you with the details. If you want to dive deeper into the access modifiers, checkout [this](https://www.geeksforgeeks.org/access-modifiers-java/).

<a name="non-access-modifiers"></a>

#### Non-access modifiers

Non-access modifiers are modfiers that don't modify the visibility of a field. The ones that will be covered here are: [static](#static), [final](#final), [abstract](#abstract), synchronized, and [volatile](#synchronized-and-volatile). 

<a name="static"></a>

##### Static

Methods and Fields declared static are not linked to individual [instances](#Constructors-and-Instances), but rather to the class as a whole. This means that static methods and fields do not require an instance to be called. The static keyword is used for fields that are common to all instances of a class. For example, let's say we wanted to know how many points have been created in total. We could change the `Point` class as so:

      public class Point {
        public double x;
        public double y;
        public static int count = 0;

        // constructor starts below
        public Point(double x, double y) {
            this.x = x;
            this.y = y;
            count++;
        }

        // distance method
        public double distance_squared(Point pt) {
            double xDist = (this.x - pt.x)**2;
            double yDist = (this.y - pt.y)**2;
            return xDist + yDist;
        }
    }

When creating instances with this code, the static field `count` is augmented by 1 (`count++` is the same as `count = count + 1`). To get the count of points, we can access the count field from the class:

    Point pt = Point(1,2);
    Point pt2 = Point(0.5, 10);
    int count = Point.count;

Notice that the `count` field is being accessed directly from the `Point` class, and not from an instance.

<a name="final"></a>

##### Final

The final keyword does what it sounds like it does: it prevents the rest of the code from modifying the field declared final. It is used to declare constants.

<a name="abstract"></a>

##### Abstract

The abstract keyword applies differently to classes and methods.

Abstract classes cannot be instantiated. They can, however, be subclassed. They also may or may not have abstract methods. This means that they are usually used to create a framework for a commonly used class type. A good example is the `Command` class in WPILib. Abstract classes are very similar to [Interfaces](#interfaces), but unlike interfaces, can have non-static and non-final fields, and have public, private or protected methods. In interfaces, all fields are static and final, and all methods are public.

Abstract methods are methods that are declared, but not defined. They must be in an abstract class.

Example:

    public abstract class Robot {
        abstract void setSpeed(double speed);
    }

<a name="synchronized-and-volatile"></a>

##### Synchronized and Volatile

The synchronized and volatile keywords aren't used very often in FRC programming. If you want to learn more about them, checkout these resources. [Synchronized](https://www.geeksforgeeks.org/synchronized-in-java/). [Volatile](https://www.geeksforgeeks.org/volatile-keyword-in-java/)

<a name="challenges-and-practice"></a>

## Challenges and Practice

Check out [Coding Bat](https://codingbat.com/java) for cool puzzles in java. Check out the different problems and find a difficulty that's right for you (hopefully a little beyond what you're familiar with). Also this could be cool: [Top Coder Problems](https://community.topcoder.com/tc?module=ProblemArchive).

Try using an object oriented structure to represent any system of your choice! Maybe model traffic (encode roads, cars, drivers, etc.) or club schedules for different students and see if you can simulate them. Make sure to restrict information and methods to be visible only to classes that should be able to see them, and experiment with different inheritances and other features of Java. Do show us what you end up coding!
