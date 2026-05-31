# `qlang` - A DSL for evaluating simple quantum circuits.

`qlang` is an interpreted domain-specific language (DSL) for building and evaluating simple quantum circuits, implemented in Rust.

I started this project because I wanted to consolidate my understanding of the quantum circuit model and canonical quantum algorithms. Although practical quantum computing is still an emerging field, I find the underlying theory and computational model extremely interesting - I built `qlang` to better understand and appreciate this model.

The language currently supports:
- classical bitstring operations such as AND (`&`), OR (`|`), XOR (`^`) and dot-products modulo 2 (`*`).
- Bitstring assignment and classical function definitions.
- Unitary oracles determined by a classical function.
- Circuit construction (with up to 16 qubits), execution, and measurement.
- Simple circuit printing to stdout , using the `circuit.printCircuit();` routine.

Circuits are also capable of utilising the following gates:
- Unary Operators: Hadamard (`H`), the Pauli Operators (`X`, `Y`, and `Z`), and Phase operators (`S`, `T`).
- Binary Operators: Controlled Not (`CNOT`), `CZ`, and the `SWAP` operators.
- Ternary Operators: `CSWAP` and the Toffoli (`CCNOT`) operators.

Although the project is yet to be fully realised, the current implementation is capable of executing:
- [Deutsch's Algorithm](https://quantum.cloud.ibm.com/learning/en/courses/fundamentals-of-quantum-algorithms/quantum-query-algorithms/deutsch-algorithm).
- [The Deutsch-Jozsa Algorithm](https://quantum.cloud.ibm.com/learning/en/courses/fundamentals-of-quantum-algorithms/quantum-query-algorithms/deutsch-jozsa-algorithm).
- [The Bernstein-Vazirani problem](https://quantum.cloud.ibm.com/learning/en/courses/fundamentals-of-quantum-algorithms/quantum-query-algorithms/deutsch-jozsa-algorithm#the-bernstein-vazirani-problem).
- Bell State Preparation.


## Architecture
`qlang` is composed of:
- a lexer for tokenisation,
- a recursive-descent parser for AST construction,
- a static type-checker for ensuring programming correctness,
- an evaluator/runtime for semantic execution,
- and a quantum register engine for state evolution and measurement.


## Example: `qlang` executing the Bernstein-Vazirani algorithm
```txt
# This program executes the Bernstein-Vazirani  #
# algorithm. Given a function that encodes a    #
# secret bitstring `s`, this algorithm recovers #
# `s` in O(1) queries.                          #

const CSIZE = 4;
bits secret = 0b0110;

# dot-products modulo 2 #
function f(x: bits[CSIZE]) -> bits[1] {
    x * secret
}
oracle U_f(x: qubits[CSIZE], y: qubits[1]) loads f;

circuit bernstein_vazirani {
    register:
        qubits x = "0" * CSIZE;
        qubits y = "-";

    apply:
        H(x);
        U_f(x, y);
        H(x);
        measure(x);
}

bernstein_vazirani.printCircuit(spaces=1);
const SHOTS = 3;
bernstein_vazirani.measure(shots=SHOTS);
```

The above program will print to stdout:
```
Circuit 'bernstein_vazirani': 
|0> H-----------Bx-H-----------M--

|0> ---H--------Bx----H--------M--

|0> ------H-----Bx-------H-----M--

|0> ---------H--Bx----------H--M--

|-> ------------By----------------

Measuring circuit 'bernstein_vazirani' 3 time/s.
SHOT 1 OF 3: Got measurement [0, 1, 1, 0]
SHOT 2 OF 3: Got measurement [0, 1, 1, 0]
SHOT 3 OF 3: Got measurement [0, 1, 1, 0]
```

Where `Bx` and `By` denote the oracle operator `U_f`.

## Usage:
Building is as simple as executing:
```
cargo build
```

Running `qlang` can either be performed through `cargo`:
```
cargo run [file]
```

Or otherwise:
```
./path_to_binary/qlang.exe [file]
```


## Future Work:
The current release is functional, but still experimental, and several improvements and features have been planned before the language can be considered stable.

Architectural improvements include:
- Updating unitary oracles → oracles are currently constrained, in that they must be applied to an entire `QubitRegister`'s domain.
- Implementing an automated and extensible testing suite (that is compatible with `cargo test`).
- Improving error messaging & diagnosis.

## Notes:
This project was created as a learning exercise in programming language construction, and quantum computing. Although `qlang` is capable of correctly evaluating small circuits, It should not be considered as a substitute for more mature languages, libraries, or frameworks.

This work was heavily inspired by Robert Nystrom's [Crafting Interpreters](https://craftinginterpreters.com/), which was an excellent reference while I was building the language's lexer and parser. I would also like to credit Jack Hidary's [Quantum Computing: An Applied Approach](https://link.springer.com/book/10.1007/978-3-030-83274-2), for his accessible introduction to quantum computing.
