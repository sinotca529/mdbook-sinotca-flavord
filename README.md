# Description
This is a `mdbook` preprocessor for me.

There are many useful preprocessors but, install and build those tools will consume a lot of disc.
Also, apply N preprocessors may be a multi path process, which is not effective.

So, I implement preprocesses which I want into this one preprocessor. 

# Function
✔-mark indicates implemented function.

- [✔] Convert some characters for the usability of MathJax
    - [✔] convert `\\` surrounded by `$` or `$$` to `\\\\`
    - [✔] convert `_`  surrounded by `$` or `$$`to `\_`
- [　] Useful syntax for html's `details`

# Additional Markdown Syntax
## `details`
```
#!details
summary
^^^
contents
```

## 

# Usage
