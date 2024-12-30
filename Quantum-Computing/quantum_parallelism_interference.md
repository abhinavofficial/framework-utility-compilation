# Quantum Parallelism and Interference

Let's introduce two quantum mechanical phenomena —  quantum parallelism  and  quantum interference  — that are fundamental to quantum information processing. We will see examples of how the quantum versions of parallelism and interference efficiently manipulate the weighting coefficients —  probability amplitudes  — within a quantum superposition state and, thereby, process quantum information.

## Quantum Parallelism

Let's revisit the concept of quantum parallelism introduced in Video 15.5. In the video, you looked at three qubits. Now, let's examine two.

Suppose you have two qubits realized by two separate electrons and their associated spins. Each electron spin can either be pointed up (the spin-up state $|↑⟩$ ) or down (the spin-down state $|↓⟩$ ). As qubits, they can also be in superposition states of $|↑⟩$ and $|↓⟩$.

A system of  $N = 2$ spins can be found in $2^N = 4$ possible spin configurations. An equal superposition of these configurations results in four complex probability amplitudes (weighting factors) $c_i$:

$|Ψ⟩=c_1|↓↓⟩ + c_2|↓↑⟩ + c_3|↑↓⟩ + c_4|↑↑⟩$

A  $π$-pulse applied to the first qubit (the leftmost spin in the bra-ket) will flip its spin. This rotation is implemented using an electromagnetic pulse with a precise amplitude and duration such that it rotates the spin by $180$ degrees.

$|Ψ⟩−→−−−−−−−−− π$-pulse on left

$spin|Ψ′⟩ = c3|↓↓⟩ + c4|↓↑⟩ +c1|↑↓⟩ + c2|↑↑⟩$

As you can see, a single $π$-pulse on a single qubit effectively shuffles the individual probability amplitudes among all the $2^N = 4$ spin configurations, making up a quantum superposition state. This is an example of quantum parallelism.

## Quantum Interference

Let's explore now what happens when a $π/2$-pulse is applied to the second qubit. There are two cases to consider:

If the second qubit is in the spin-up state, a  $π/2$-pulse applied along the y-axis will rotate the spin from the north pole down to the equator. This aligns the spin with the $+x$ direction, creating the equal superposition state  $\frac{(|↑⟩ + |↓⟩)}{\sqrt{2}}$  with a $+$ sign.
If the second qubit is instead in the spin-down state, a  $π /2$-pulse applied along the y-axis will rotate the spin counterclockwise, bringing it from the south pole up to the equator. This aligns the spin in the - x direction, creating the equal superposition state  $\frac{(|↑⟩ − |↓⟩)}{\sqrt{2}}$ , this time with a corresponding $-$ sign.
The resulting state is

|Ψ⟩−→−−−−π/2-pulse|Ψ"⟩=12–√(c2−c1)|↓↓⟩+12–√(c2+c1)|↓↑⟩+12–√(c4−c3)|↑↓⟩+12–√(c4+c3)|↑↑⟩ .

The probability amplitudes now add and subtract one another. Suppose there are two coefficients with equal values, for example,  c3 = c4 . In this case, there is a complete cancellation of the probability amplitude for  |↑↓⟩ . Such a reduction of the probability amplitude is called destructive quantum interference. On the contrary, there is a doubling of the probability amplitude in front of  |↑↑⟩ . Such an enhancement of the probability amplitude is called constructive quantum interference. And, since the constructive and destructive quantum interferences happen across the entire state space, this is also an example of quantum parallelism.