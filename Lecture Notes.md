# Zero Knowledge Proofs in Rust

## 1. Course Description
### 1. Course Description
* outline of chapters
    1. theoretical background
        * Chaum-Pedersen Protocol: interactive protocol
    2. practical implementation
        * Rust, crates, and tests
    3. develop a server/client
        * `gRPC` is a modern open source high performance Remote Procedure Call (RPC) framework
        * real-world application
    4. dockerization

### 2. Introduction to Modular Arithmetics
* the **modulo operation** returns the remainder or signed remainder of a division 
* such that the result of the division may be expressed in two parts: $c, r$ 
* the dividend can be reconstructed by $c \cdot \text{ divisor } + r$
* examples:
    * $ 4 ÷ 2 = 0, r=0$
    * $ 3 ÷ 2 = 1.5 \text{ or } c=1, r=1$
    * inversely, $c \cdot 2 + r = 3 $
* the remained is equal or greater than zero 
    * $ 1+0 \text{ mod } 2 = 1 $
    * $ 1+1 \text{ mod } 2 = 0 $
    * $ 1+2 \text{ mod } 2 = 1 $
    * $ 1+3 \text{ mod } 2 = 0 $
    * $ 1+4 \text{ mod } 2 = 1 $
    * $ 1+5 \text{ mod } 2 = 0 $
    * $ 1+(a) \text{ mod } 2 = \{0, 1\}$
    * modulo remained repeats cyclically
* modulo for negative number
    * $ -1 \text{ mod } 2 = 1 $
    * $ -2 \text{ mod } 13 = 11 $

### 3. Quiz Modular Arithmetics
* Q1: $34 \text{ mod } 11$
    * $r=1$ because $34=11 * 3 + 1$
* Q2: $3 \text{ mod } 4$
    * $r=3$ because $3=4 * 0 + 3$
* Q3: $4-7 \text{ mod } 2 $
    * $-3 \text{ mod } 2$
    * $c=-2, r=1$ because $-3=-2 * 2 + 1$
* Q4: $4-7 \text{ mod } 11 $
    * $-3 \text{ mod } 11$
    * $c=11,r=8$ because $-3=-1 * 11 + 8$

### 4. Groups
Introduction
* in mathematics, groups $G$ are a set of elements plus an operation "$\circ$"
* the operation associates an element of the set the other elements in the set
    * e.g. $\set{-1,0,1}$ and operator $+$
* any group must satisfy 4 properties

1. **Closure**
    * for any $a, b \in G$, then the operation of a by b must also belong to the group $a \circ b \in G$
2. **Associativity**
    * for any three elements $a, b, c \in G$, it must follow that $(a \circ b) \circ c = a \circ (b \circ c)$
3. **Identity**
    * there exists an element $i \in G$ such that for every $a \in G$, one has $a \circ i = a$ and $i \circ a = a$ 
    * this element is unique, aka as _neutral element_ or _identity element_
    * for natural numbers it's $0$
4. **Inverse**
    * for every $a \in G$, there exists an inverse element $a^{-1} \in G$  and $a^{-1} \circ a = i$
5. **Commutativity**
    * is not required, but groups that fulfill this are called _abelian group_
    * means that for every $a, b \in G$, the condition $a \circ b = b \circ a$

Example
* define a module group as set of natural numbers and an operation
    * $k=5$
    * set: $\mathbb{Z_5} = \set{0, 1, 2, 3, 4}$
    * operation: $a+b\text{ mod }5$
* test for all properties
* 1 closure:
    * $1 + 3 \text{ mod }5 = 4$
    * $4 \in \mathbb{Z_5}$
* 2 associativity:
    * $((a+b)\text{ mod }5 +c) \text{ mod }5 = a(+b+c)\text{ mod }5$
    * $a=2, b=3, c=4$
    * $((a+b)\text{ mod }5 +c) \text{ mod }5 = 4$
    * $a(+b+c)\text{ mod }5 = 4$
    * $4 = 4$ and $4 \in \mathbb{Z_5}$
* 3 identity is $0$
    * $i=0,a=2$
    * $2 + 0\text{ mod }5 = 2$
    * $0 + 2\text{ mod }5 = 2$
* 4 inverse
    * $0 + 0 \text{ mod }5 = 0$
    * $1 + 4 \text{ mod }5 = 0$
    * $2 + 3 \text{ mod }5 = 0$
    * $3 + 4 \text{ mod }5 = 0$
    * $4 + 1 \text{ mod }5 = 0$
* 5 commutativity
    * see 4

### 6. Generators
What are generators in Python?
* in Python, generators are functions that return an iterator that produces a sequence of values when iterated over
* the sequence of values does not yet exists in memory but each value is created once called from the iterator
* generators are useful when we want to operate on a large sequence of values or objects but we don't want to store them in memory all at once

Generator in Groups
* example modulo group 
    * $k=11$
    * set: $\mathbb{Z_{11}} = \set{1,2,...,10}$
    * operation $a \times b \text{ mod } 11$
    * the order of the set is the number of elements in the set $\text{ord}(\mathbb{Z_{11}})=10$ (can also be expressed as $|\mathbb{Z_{11}}|=10$)
* in python:
    ```python
    k = 11
    group = range(1, k)
    a = 2

    def operation(a, b, k):
        return a**b % k
        
    closure_a = [operation(a, b, k) for b in group]


    print(f"group = {set(group)}")
    print(f"closure_a = {set(closure_a)}")
    ```
    * result
    ```
    group = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
    closure_a = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
    ```
* $a$ is generator because it operating on it produced all elements in the group 

Elements that are **not** Generators
* let's do $c=3$ in the code above
    ```python
    c = 3
    closure_c = [operation(c, b, k) for b in group]
    print(f"closure_c = {set(closure_c)}")
    ```
    * result
    ```
    closure_c = {1, 3, 4, 5, 9}
    ```
* this returns an incomplete set of the group
    * $c=3$ is not a generator

Summary
> a generator is an element that if operated upon by other elements of the group can re-produce the entire group as result 

### 7. Discrete Logarithm Problem
* is at the foundation on many crypto-graphic concepts
* we'll discuss this with an informal approach
* let define our group
    * $G$ is a group with operator "$\circ$
    * $\alpha \in G$ is a generator
    * $d$ is an integer
    * $\beta \in G$ 
    * problem: how many times $d$ must $\alpha$ be operated on itself to produce $beta$ ?
    * $\beta = \alpha \circ ... \circ \alpha = \alpha^{d}$
* in integer arithmatic, this relatively easy and can be solved with a logarithm
    * $8 = 2^x$ -> solve for $x$
    * $\log_2(8)=x$
    * $x=3$
* in modular arithmatic, this is a difficult problem
    * finding $d$ from $\alpha, \beta$ cannot be solved in non-polynomial time
    * but verifying $\beta$ from $\alpha, d$ is very easy
* this is good for proof-based cryptography because prohibitively  expensive computing power must be spent to solve it

Public-Private Key Cryptography
* the public key and private key work together to ensure the security of the exchanged data
* message gets encrypted by a public key (available to everyone), but can only be decrypted with its unique private key (only available to its owner)
* let's assume that
    * $\beta$ is the public key
    * $d$ is the private key
* if we pass $\alpha, \beta$ as meta-data to a 2nd party
    * then if $d$ is known, the message  can be decrypted easy and fast
    * but if $d$ is unknown, it must be expensively computed from $\alpha, \beta$
* with very large numbers, like `256-bit` integers, it becomes impossible for a machine to solve this in a human lifetime

### 8. Chaum-Pedersen ZKP Protocol
Overview
* this is an important theoratical and practical foundation for ZKP
* many cryptographic number groups use  a set of prime numbers with the operation of multiplication and modulo
* for our implementation, we use
    * set: $G=\set{\mathbb{Z_{p'}}}$
    * operation: $*$, which means $\alpha^d \text{ mod } p'$
    * the order $q$ is also a prime (prime order)
    * with prime order, any number in the set is a generator
    * elements $\alpha, \beta \in G$ are generators

Diagram
<p align="left">
<img src="Chaum-Pedersen-ZKP-Protocol.png"  width=500/>
</p>     

Motivation behind Zero Knowledge Proof
* we have two actors
    1. Bob (prover; has secret, private key; wants to log-in)
    2. Alice (verifier; has server)
* traditionally, Alice and Bob would have shared the private key and both know it
    * through previous account setup and encrypted message
* but for Alice, centrally holding the private keys of all users carries increased risk
* with ZKP, Alice wants to know if Bob has the private key without knowing the secret herself
    * instead Alice will send a challenge/task to Bob that he needs to solve in order to proof he has the private key

Designing the Protocol
* $\alpha, \beta, q$ are public info, known between the two actors
* Bob has the secret key $x$
* Bob is going to compute two numbers $y_1, y_2$ using $\alpha^x, \beta^x$ and sends them
* Bob generates a random number $k$ and repeats the previous step
* he computes $r_1, r_2$ using $\alpha^k, \beta^k$ and sends them
* this is enough info for Alice to create a challenge
* Alice generates a random number $c$ and sends it back
* Bob has to compute the solution $s$ for the challenge with $s = k - c \cdot \text{ mod } q$
* Bob sends this solution back to Alice
* Alice can verify this solution with
    * $r_1 = \alpha^s \cdot y_1^c$
    * $r_2 = \beta^s \cdot y_1^c$
    * this verification is computationally easy
* with `256-bit` numbers from a secure group $G$ it highly probable that Bob knows the private key

Proof
* for $r_1$ we have $\\
    r_1 = \alpha^s \cdot y_1^c \\
    y_1 = \alpha^x  \\
    s = k - c \cdot x \text{ mod } q
    $
* so we can substitute $\\
    r_1 = \alpha^{k-c x} \cdot \alpha^{c x} \\
    r_1 = \alpha^k \cdot \alpha^{c x} \cdot \alpha^{-c x} \\
    r_1 = \alpha^k
    $ 
* proof that $r_1 = \alpha^k$ is satisfied

Exercise
* for $r_2$ we have $\\
    r_2 = \beta^s \cdot y_1^c \\
    y_2 = \beta^x
    s = k - c \cdot x \text{ mod } q
    $
* so we can substitute $\\
    r_2 = \beta^{k-c x} \cdot \beta^{c x} \\
    r_2 = \beta^k \cdot \beta^{c x} \cdot \beta^{-c x} \\
    r_2 = \beta^k
    $
* proof that $r_2 = \beta^k$ is satisfied 

Review
* $\alpha, \beta, p', q$ are shared between actors
* the user has a secret $x$
* the user sends 2 tuples of data to the server
* the server creates a challenge and sends it to the user
* the user solves the challenge which is easy, if he knows the secret but near-impossible if unknown
* the server can verify the solution easily

### 9. Quiz: Chaum-Pedersen ZKP Protocol
* Q1: What is known to both prover (client) and verifier (server) in the Chaum-Pedersen ZKP protocol?
    * the constants variables $g, h$ (group order)
* Q2: What is the secret $x$?
    * a large integer
* Q3: What are $k, c$?
    * random numbers generated by client & server, respectively
* Q4: Is it secure to re-use $k, c$ everytime?
    * no, it would make the protocol insecure

### 10. Toy Example
Setup
* let's build a toy example from the concepts of the last lecture
* group of $\mathbb{Z_{23}}$ and order $q=11$ (n of elements)
    * all elements are generators
    * there are theorems to determine sets with all elements as generator (not covered here)
* our generators are $\alpha=4$ and $\beta=9$

Bob
* secret is $x=6$
* computes $y_1, y_2$
    * $y_1 = \alpha^x = 4^6 \text{ mod } 23 = 2$
    * $y_2 = \beta^x = 9^6 \text{ mod } 23 = 3$
* the random is $k=7$
* computes $r_1, r_2$
    * $r_1 = \alpha^k = 4^7 \text{ mod } 23 = 8$
    * $r_2 = \beta^k = 9^7 \text{ mod } 23 = 4$
* send $y_1, y_2$ and $r_1, r_2$ to Alice

Alice
* the random is $c=4$

Bob
* computes the solution $s=(k-c \cdot x) \text{ mod } q$
    * $s = (7 - 4 * 6)\text{ mod } 11$
    * $s = 5$

Alice
* verifies the solution with the previous infos
* verify $r_1$
    * $r_1 = \alpha^s \cdot y_1^c$
    * $r_1 = 4^5 \cdot 2^4 \text{ mod } 23$
    * $r_1 = 8$ ☑
* verify $r_2$
    * $r_2 = \beta^s \cdot y_1^c$
    * $r_2 = 9^5 \cdot 3^4 \text{ mod } 23$
    * $r_2 = 4$ ☑
* Bob can be verified

### 11. Assignment: Importance of Good Random Number Generators