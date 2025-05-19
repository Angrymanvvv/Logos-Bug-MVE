this is a short minimum viable example of a (potential) bug in the `Logos` crate. 

In it, the String "-1" gets matched wholly to the definition 
```
#[token("-")]
Sub,
```
, but only in presence of another specific definition: 

```
#[regex(r"([+-]?(([0-9]+[eE][+-]?[0-9]+)|([0-9]*\.[0-9]+[eE][+-]?[0-9]+|[0-9]*\.[0-9]+)))", |output| {
  output.slice().parse::<f64>().ok()
})]
FloatLiteral(f64),
```
.

The Output of the main() fn (in the Standard output) is >Ok(Sub): "-1"<, indicating that the 1 has been consumed in the matching of the Sub token. 

