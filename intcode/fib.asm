loop:
    out val0
    add val0, val1, tmp
    add 0, val1, val0
    add 0, tmp, val1
    add 1, counter, counter
    equ counter, 25, tmp
    jz tmp, loop
    halt

val0:    dd 1
val1:    dd 1
tmp:     dd 0
counter: dd 0