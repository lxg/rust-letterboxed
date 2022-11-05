# “Letter Boxed” Solver

This is a little Rust project that proposes solutions to the [NY Times’ “Letter Boxed” puzzle](https://www.nytimes.com/puzzles/letter-boxed).

## Background

Letter Boxed is a puzzle that presents twelve letters and requires the user to create a chain of words where every letter is used at least once. The letters are arranged around a square, three letters on each side. All words must begin with the last letter of the previous word (except for the first word, obviously). The solution words must not contain consecutive letters from one side. According to the description, the solution can contain up to four, five or six words (different each day), but there is always a solution with only two words. Such an *ideal* solution can be seen on the next day under “Yesterday’s Answers”. A *perfect* solution would be one where every letter is used exactly once; though this is not always the case even with the given solution.

## Internals

This program uses a word list with a bit more than 300,000 words from https://github.com/dwyl/english-words, as we don’t have as we don’t have access to that one that Letter Boxed itself uses. Not all of the available words are also in the Letter Boxed body, therefore the program will propose solutions that are not accepted by Letter Boxed.

The program in general is quite simple, implementing roughly the following algorithm:

- Receive today’s letters via command line argument, e.g. `abc/def/ghi/jkl`.
- Load the list of words from the dictionary.
- Create lists of valid and invalid letters as well as invalid sequences (i.e. combinations of letters from one side of the square).
- Remove all words with invalid letters or invalid sequences.
- For each word in the list, find all words that would be a valid continuation.
- For all word + continuations sets, create pairs where each of today’s letters is used at least once.
- Print a list of all found pairs, sorted from longest to shortest.

This way, the best solution, i.e. the one with the least redundancy, is printed as the last entry. Again, please note that not all pairs proposed by this program are seen as valid by Letter Boxed due to the different dictionaries.

