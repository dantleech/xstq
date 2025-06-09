XSTQ
====

XPath Syntax Tree Query - query your source code with XPath expressions.

> **CAUTION**: Not to be used by anybody at this point. Pull requests not accepted.

How it Works
------------

XSTQ uses [TreeSitter](https://github.com/tree-sitter/tree-sitter) to parse
your source code and internally converts your source code to an XML document
that can be queried with XPath (supporting XPath 2.0 and 3.0 thanks to
[xee](https://github.com/Paligo/xee)).

Why not AST-Grep
----------------

[AST-Grep](https://ast-grep.github.io/) is a similar tool that allows you to
search your code base using the AST itself as a query language in addition to
being able to more complex rules.

XSTQ allows you to write more complex queries but also allows you to
**extract results**.

Why?
----

You can extract data from your source code, for example, list all methods that
contain calls to `dropDatabase`:

```
xstq eval "src/**/*.php" \
    './/scoped_call_expression[name[1]="Database"][name[2]="dropDatabase"]/ancestor::method_declaration/name/string()' 
```

If you are unsure what to query you can view the entire trees:

```
xstq eval "src/**/*.php" "."
```

Eval
----

Eval let's you execute an arbitrary XPath on a path or list of paths (globs
are also natively supported):

```
xstq eval src/Path/ToFile.php './/method_declaration/name/string()' 
```

Analyze
-------

Analyze allows you to specify a configuration file with rules in it:

```
[[rules]]
name = 'orm_find_one'
xpath = './/scoped_call_expression[./name[1]="Posts"][./name[2]="findOne"]/ancestor::method_declaration/name/string()'

[[rules]]
name = 'drop_database'
xpath = '//scoped_call_expression[name[1]="Database"][name[2]="dropDatabase"]/ancestor::method_declaration/name/string()'
```

You can then run an analysis and output in JSON format:

```
xstq --config rules.toml analyze "tests/**/*Test.php" --json > out.json
```

TODO
----

- [ ] Multi-threading
- [ ] Language selection (currently hardcoded to PHP)
- [ ] ...
