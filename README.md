Lexical Expression Parser
=========================

```
✓ registers show
✓ registers flush
registers set A=0x01

memory add subsystem minifb         // add minfb extension
                     rom "program.bin" // add a ROM
✓ memory show #0x1234 0xff            // start, length
✓ memory load #0x1234 "program.bin"   // start, filename
memory flush

✓ run
✓ run until false
✓ run until #0x1234 = 0x12
✓ run #0x1234
✓ run #0x1234 until A >= 0x12

boot                // load CP with address at #0xFFFC
interrupt           // trigger an interrupt
reboot              // re-init memory & registers

state save "filename"     // save configuration, memory & registers
state load "filename"     // load configuration, memory & registers
```


expression:
-----------
```
Ok(
    [
        Pair {
            rule: boolean_expression,
            span: Span { str: "A > 0x12", start: 0, end: 8 },
            inner: [
                Pair {
                    rule: operation,
                    span: Span { str: "A > 0x12", start: 0, end: 8 },
                    inner: [
                        Pair {
                            rule: register8,
                            span: Span { str: "A", start: 0, end: 1 },
                            inner: []
                            },
                        Pair {
                            rule: operator,
                            span: Span { str: ">", start: 2, end: 3 },
                            inner: []
                        },
                        Pair {
                            rule: value8,
                            span: Span { str: "0x12", start: 4, end: 8 },
                            inner: []
                        }
                    ]
                }
            ]
        }
    ]
)
```

boolean:
--------
```
Ok(
    [
        Pair {
            rule: boolean_expression,
            span: Span {
                str: "true",
                start: 0,
                end: 4
            },
            inner: [
                Pair {
                    rule: boolean,
                    span: Span { str: "true", start: 0, end: 4 },
                    inner: []
                }
            ]
        }
    ]
)
```
