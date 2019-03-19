## Technical Debt
* type-checking: proc last statement must be return
* implement `-` operator
* CFG - truncate orphan nodes
* Interpreter
  * allocate globals
  * call a function without params
  * call a function with params (allocate locals on the stack)
    (allocate params as locals)
  * local 0 starts at offset 0 of the callee callstack-frame
    local 1 starts at offset 1 of the callee callstack-frame
    ...
  * open question: how to allocate locals which are created inside the procedure? (i.e: not params)
  * implement stackoverflow (for example: max call-stack frames = 10_000)
  * implement `STOP`
