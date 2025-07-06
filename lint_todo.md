# Lint Todo List

## Errors to Fix

### Source Code Errors
1. **src/common/pretty_print.rs:368** - dead code
   - [x] Remove unused function `write_flow_edge_with_nodes`

2. **src/common/validation.rs:304** - iterate on map's keys
   - [x] Change `for (node_id, _) in &diagram.nodes` to `for node_id in diagram.nodes.keys()`

3. **src/common/validation.rs:362** - iterate on map's values
   - [x] Change `for (_node_id, node) in &diagram.nodes` to `for node in diagram.nodes.values()`

4. **src/common/validation.rs:438** - parameter only used in recursion
   - [x] Remove `&self` parameter or make method static

5. **src/common/validation.rs:522** - parameter only used in recursion
   - [x] Remove `&self` parameter or make method static

6. **src/common/validation.rs:651** - parameter only used in recursion
   - [x] Remove `&self` parameter or make method static

7. **src/common/validation.rs:815** - iterate on map's keys
   - [x] Change `for (state_id, _) in &diagram.states` to `for state_id in diagram.states.keys()`

8. **src/common/validation.rs:828** - parameter only used in recursion
   - [x] Remove `&self` parameter or make method static

9. **src/error.rs:108** - write! with format string ending in newline
   - [x] Use `writeln!` instead of `write!` with \n

10. **src/parsers/flowchart.rs:305** - single match
    - [x] Change match to if let

11. **src/parsers/flowchart.rs:430** - length comparison to one
    - [x] Change `tokens.len() >= 1` to `!tokens.is_empty()`

12. **src/parsers/state.rs:233** - stripping prefix manually
    - [x] Use `strip_prefix` method instead

13. **src/parsers/sequence.rs:412** - stripping prefix manually
    - [x] Use `strip_prefix` method instead

14. **src/parsers/misc.rs:364** - length comparison to zero (in tests)
    - [x] Change `tokens.len() > 0` to `!tokens.is_empty()`

15. **src/parsers/class.rs:276** - useless use of vec! (in tests)
    - [x] Use array directly instead of vec!

### Test and Example Errors
16. **examples/error_handling.rs:139** - unused variable
    - [x] Prefix with underscore: `_parsed`

17. **examples/error_handling.rs:129** - useless use of vec!
    - [x] Use array instead of vec!

18. **tests/validation_test.rs:372** - length comparison to zero
    - [x] Change to `!errors.is_empty()`

19. **tests/validation_test.rs:380** - field assignment outside initializer
    - [x] Use struct initialization syntax

20. **tests/pretty_print_coverage_improvement_test.rs:195** - useless use of vec!
    - [x] Use array instead of vec!

21. **tests/sankey_test.rs:8** - function call inside expect
    - [x] Use `unwrap_or_else`

22. **tests/timeline_test.rs:8** - function call inside expect
    - [x] Use `unwrap_or_else`

23. **tests/packet_test.rs:8** - function call inside expect
    - [x] Use `unwrap_or_else`

24. **tests/requirement_test.rs:12** - function call inside expect
    - [x] Use `unwrap_or_else`

25. **tests/common/assertion_helpers.rs** - multiple unused functions
    - [x] Add `#[allow(dead_code)]` or remove unused functions

26. **tests/common/file_utils.rs** - multiple unused functions
    - [x] Add `#[allow(dead_code)]` or remove unused functions

27. **tests/git_test.rs:40** - length comparison to zero (multiple)
    - [x] Change to `!is_empty()`