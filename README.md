# SEP Protocol Cost Evaluator

An under 2 hours toy parser to evaluate the cost of the protocol
used in the project of the 
[Security Protocols](http://master.irisa.fr/courses/SEP.html) class of my M2.

## How to use
`git clone https:://github.com/redmie/`

`cargo run < your_protocol`

## Syntax

Everything before `:` is ignored
- Symetric encryption: `senc(m,k)`
- Pair: `<m1,m2>`
- Hash: `h(m)`

Example file:
```
A->B: senc(<<k, N_a>, h(N_b)>, K_AB) 
```

## License
This work is licensed under the MIT License (see LICENSE file)
