# chai
### lisp, but no
example chai program:
```
fn fib ( (n int) ) int {
    def a int 0 ; def b int 1 ; def c int 0
    if [== n 0] { ret a }
    
    for {def i int 2} [<= i n] {inc i} {
        set c [+ a b]
        set a b
        set b c
    }

    ret b
}

foreach i [range 0 100] {
    println [fib i]
}
```
