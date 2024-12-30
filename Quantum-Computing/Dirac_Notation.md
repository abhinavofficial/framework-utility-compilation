# Dirac's Notation

A qubit is a two-level quantum mechanical system. The particular physical system (Bloch's sphere) was the spin of an electron in an atom, where the two states were spin-up and spin-down. Although qubits are realized using physical systems, it is useful to think of them as mathematical objects because it is easier to work with them using mathematics, and the approach is technology-agnostic, which means that it is independent of any particular physical system. Let's learn the basic concepts of linear algebra necessary for understanding how quantum states and gates operate, using [Dirac notation](https://en.wikipedia.org/wiki/Bra%E2%80%93ket_notation). We aim to understand the concept of measurement at a high level.

## Communicating in Classical Computing

As an introductory example, consider the state-space representation of four light bulbs. In this classical system, each light bulb is a classical two-state system and can be either: off (state 0), or on (state 1). This means that your classical system can be in $2^4 = 16$ possible configurations. Suppose that you decide to communicate your ATM PIN (1248) to your neighbor using light bulbs. To do this, you would first write each digit of the PIN, using binary representation, and then turn the lights on and off according to the predefined state-space definition:

1 = 0001 → OFF, OFF, OFF, ON

2 = 0010 → OFF, OFF, ON, OFF

4 = 0100 → OFF, ON, OFF, OFF

8 = 1000 → ON, OFF, OFF, OFF

To send the decimal number 1, you will keep the first three bulbs off and the last one on. To send the number 2, you will keep the first two bulbs off, the third bulb on, and the fourth bulb off. Then, you will repeat the process with the remaining two numbers.

## Communicating in Quantum Computing

Qubits and classical bits both represent two-state systems, but qubits have unique quantum mechanical properties. Thus, to represent the state of a qubit, people use a standard notation called Dirac notation or bra-ket (read: bracket) notation. This representation uses vectors that can be manipulated using concepts from linear algebra, such as matrix multiplication.

### Key concepts of Bracket Notations

1. States 0 and 1 are represented as kets $|0⟩$ and $|1⟩$ (the ket in bra-ket) and correspond to column vectors. In particular, ket $|0⟩$ and ket $|1⟩$ are usually written as $|0⟩=(10), |1⟩=(01)$.

2. Bras (the bra in bra-ket) are the Hermitian conjugate of kets. Operationally, a Hermitian conjugate is found by transposing a vector (or matrix) and taking the complex conjugate of each element. Since the states $|0⟩$ and $|1⟩$ contain only real numbers, the Hermitian conjugate is equivalent to the transpose and results in the following row vectors: $⟨0| = (10), ⟨1| = (01)$. The use of the Hermitian conjugate may be clearer after the next point.

3. The inner product between two states, say $|ϕ⟩$ and $|ψ⟩$, is written as the bracket (as in bra-ket) $⟨ϕ|ψ⟩$, and in general results in a complex number. This will become evident in the following example. Consider the quantum state $|ψ⟩=α|0⟩+β|1⟩=α(10)+β(01)=(αβ)$.

4. Taking the inner product of two vectors shows that $⟨0|ψ⟩ = α$ and $⟨1|ψ⟩ = β$. In this example, the inner product of the general state $|ψ⟩$, with each of the basis states $|0⟩$ and $|1⟩$, returns a number that corresponds to the probability amplitude of $|ψ⟩$ in each of those states. Hence, the Hermitian conjugate is a mathematical tool that is used to calculate the projection of one state onto another. Finally, it should be noted that since the inner product is a complex number in general, it can be decomposed into a product of its modulus (its magnitude) — represented as $|⟨ψ|ϕ⟩|$ — and a phase factor, $eiθ$, where $θ$ is the angle between the vectors representing the states $|ψ⟩$ and $|ϕ⟩$.

5. The [norm](https://en.wikipedia.org/wiki/Norm_(mathematics)) or length of the vector representing a state $|ψ⟩$ is given by the square root of the inner product: $|⟨ψ|ψ⟩| = \sqrt{⟨ψ|ψ⟩}.\sqrt{⟨ψ|ψ⟩}$

6. Physical states represented in ket notation have a norm equal to one, that is #⟨ψ|ψ⟩ = 1$. Checking and ensuring that the norm of a state has unit value is a procedure called normalization. Since $|0⟩$ and $|1⟩$ are physical states with a unit norm, they must also satisfy the following condition (since $1_2=1$) $|⟨0|0⟩|=\sqrt{⟨0|0⟩},|⟨1|1⟩|= \sqrt{⟨1|1⟩}$

States |0⟩ and |1⟩ are orthogonal, i.e., ⟨0|1⟩ = ⟨1|0⟩ = 0. This means that there is no projection of state |0⟩ onto state |1⟩ and vice versa. They are independent vectors, so there is no way to write |0⟩ in terms of |1⟩ or vice versa. This is called linear independence.

When a quantum state is the sum of linearly independent states, such as |0⟩ and |1⟩, it is said to be in a superposition state. This is the case for the state |ψ⟩ = α|0⟩ + β|1⟩ defined above. The coefficients α and β are referred to as probability amplitudes, and they are, in general, complex numbers. The Hermitian conjugate of |ψ⟩ is ⟨ψ| = α∗⟨0|+β∗⟨1|, = (α∗β∗), where α∗ and β∗ are the complex conjugates of α and β, respectively.

To better understand what the probability amplitudes α and β of |ψ⟩ represent, let's think about the superposition concept. A light bulb is either on or off. When you look at it or measure it, you know exactly which state it had been in and continues to be in. On the contrary, while a quantum system can certainly be in the classical states |0⟩ or |1⟩, it can also be in a superposition state, a single state that carries aspects of both |0⟩ and |1⟩. Let’s see what this means.

Let's take a qubit prepared in the superposition state |ψ⟩ = α|0⟩ + β|1⟩. When this qubit is measured, quantum mechanics tells us that the qubit state will be projected onto the measurement basis. In the examples in Video 13.4, the presenter is measuring along the z-axis, that is the axis that represents states |0⟩ and |1⟩. The measurements must give a classical result, so any given measurement will result in one of the classical states: either state |0⟩ or state |1⟩. You never measure a superposition state directly.

However, if you prepare and measure the state |ψ⟩ identically many times, you will find that you will obtain state |0⟩ with probability |α|2 and state |1⟩ with probability |β|2. These coefficients are called α and β probability amplitudes, since their magnitudes squared will yield the probability that you measure their respective states with. As shown in the example in statement three above, these probability amplitudes can be found by projecting the vector representing state |ψ⟩ onto the vectors representing the states |0⟩ and |1⟩.

Dirac Notation 

This fact is represented by the figure above that shows the projection of the superposition state |ψ⟩ = α|0⟩ + β|1⟩ onto the measurement axis corresponding to states |0⟩ and |1⟩. Notice that the closer the state |ψ⟩ is to |0⟩, the larger the projection |α| and thus the probability |α|2 of measuring state |0⟩. In fact, when |ψ⟩ coincides with |0⟩, the value of |α| becomes equal to 1, and you will be able to measure state |0⟩ with certainty (probability |α|2 equals to 1).

To summarize, a superposition state |ψ⟩ = α|0⟩ + β|1⟩ satisfies the normalization condition |α|2 + |β|2 = 1 (this ensures that the probabilities of measuring all states add to unity), and the probability of measuring the states |0⟩ and |1⟩ are p(0) = |⟨0|ψ⟩|2 = |α|2 and p(1) = |⟨1|ψ⟩|2 = |β|2, respectively.

> An arbitrary single-qubit state, as shown on the Bloch sphere, can be written as cos(θ2)|0⟩ + eiϕsin(θ2)|1⟩. To find the solution, you first need to solve the following for the polar angle: cos(θ2) = sin(θ2) that is satisfied for θ = π/2. You then need to solve the following for the azimuthal angle: eiϕ = 1 that is satisfied for ϕ = 0 and eiϕ = -1 = eiπ that is satisfied for ϕ = π. The angles θ and ϕ are in units of radians. You must convert radians to degrees to answer the question. π radian = 180 degrees.
