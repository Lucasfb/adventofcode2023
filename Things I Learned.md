# Day 1
* Check for empty line at end of input file
* Map is useful for converting from one kind of iterator to another kind of iterator
* Parse() might need a type annotation
* Check how you are splitting an input string
* When matching strings, consider overlapping substrings
* The .matches() method can take functions and closures as a pattern, but only when the input is a character. It's not possible to use a function with a string input as a pattern
* That .matches() method can take an array of chars, but not an array of &str
* That .matches() method cannot take a pattern of multiple items using the | operator, even for chars. Inside the mathod, it's considered a bitwise operator always