Boolean Expression Parser
=========================

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
                    rule: expression,
                    span: Span { str: "A > 0x12", start: 0, end: 8 },
                    inner: [
                        Pair {
                            rule: register8_location,
                            span: Span { str: "A", start: 0, end: 1 },
                            inner: []
                            },
                        Pair {
                            rule: operation,
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
