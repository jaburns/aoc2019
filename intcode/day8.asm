;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Day 8 - Part One
;; 

readLoop:
    ; Read a digit, and if we're at the end of the input break the read loop.
        in digit
        cmp digit, 99, compare
        jnz compare, done

    ; Increment the appropriate digit counter.
        cmp digit, 0, compare
        jz compare, notZero
        add 1, curZeroCount, curZeroCount
        jz 0, doneDigitCount
    notZero:
        cmp digit, 1, compare
        jz compare, notOne
        add 1, curOneCount, curOneCount
        jz 0, doneDigitCount
    notOne:
        add 1, curTwoCount, curTwoCount
    doneDigitCount:

    ; Increment the overall counter for the layer, and if we're not done with
    ; this layer then keep reading digits.
        add 1, curDigitCount, curDigitCount
        cmp curDigitCount, 150, compare
        jz compare, readLoop

    ; We're at the end of the current layer.
    ; If this layer's zero count is better than the best layer's, compute new product.
        less curZeroCount, bestZeroCount, compare
        jz compare, thisLayerNotBetter
        add 0, curLayer, bestLayer
        add 0, curZeroCount, bestZeroCount
        mul curOneCount, curTwoCount, bestProduct
    thisLayerNotBetter:

    ; Reset the counters, increment the layer index, and restart the digit counting loop.
        add 1, curLayer, curLayer
        mul 0, curDigitCount, curDigitCount
        mul 0, curZeroCount, curZeroCount
        mul 0, curOneCount, curOneCount
        mul 0, curTwoCount, curTwoCount
        jz 0, readLoop

done:
        out bestProduct
        halt

digit:   dd 0
compare: dd 0

curLayer:      dd 0
curDigitCount: dd 0
curZeroCount:  dd 0
curOneCount:   dd 0
curTwoCount:   dd 0

bestLayer:     dd 0
bestZeroCount: dd 15000
bestProduct:   dd 0