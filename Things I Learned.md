# Day 1
* Check for empty line at end of input file
* Map is useful for converting from one kind of iterator to another kind of iterator
* Parse() might need a type annotation
* Check how you are splitting an input string
* When matching strings, consider overlapping substrings
* The .matches() method can take functions and closures as a pattern, but only when the input is a character. It's not possible to use a function with a string input as a pattern
* That .matches() method can take an array of chars, but not an array of &str
* That .matches() method cannot take a pattern of multiple items using the | operator, even for chars. Inside the mathod, it's considered a bitwise operator always
# Day 2
* How a nom parser works
* How to create a simple parser
* How to chain parsers
* A parser is not a "search"
* A parser goes from start of string to end, in order. You have to consider what is coming and how to separed it in parts
* If you can't make a big, chained parser, split into multiple statements and variables
* When importing a lot of stuff from one module, try to keep it organized
* Remember to #derive Clone