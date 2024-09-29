
## Overview
This Rust program performs basic operations on two polynomials with integer coefficients, including:
- Polynomial addition
- Polynomial multiplication
- Degree calculation of the resulting polynomials
It calculates the sum, product, and degrees of the sum and product of any two polynomials.
## Features
- **Addition of polynomials:** Adds the corresponding terms of two polynomials.
- **Multiplication of polynomials:** Multiplies two polynomials by distributing terms and combining like powers.
- **Degree calculation:** Determines the degree (the highest power of $x$) of the sum and product of two polynomials.

## Requirements
- Rust installed on your machine. (If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it).
## Example Polynomials
The two polynomials are defined as follows:
\begin{align*}
P(x) &= 5x^2-4x + 2\\
Q(x) &= x^3-2x^2 + 5
\end{align*}
>```
>The Pedersen hash is: 8
## How It Works:
- **Modular Exponentiation:** The function `mod_exp` computes each generator raised to the power of the corresponding value modulo the group modulus.
- **Pedersen Hash Calculation:** The result of each exponentiation is multiplied together modulo `p`. This result is the Pedersen hash.
## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

## Usage
- To use this code, you can clone the repository.
- Compile the Rust code using cargo:
>```
>cargo build
>cargo run

## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
   git clone https://github.com/Polynomial_Addition_and_Multiplication_in_Rust.git
   cd Polynomial_Addition_and_Multiplication_in_Rust
