# Quantum Annealing

Quantum Annealing is another type of quantum computer, one that addresses classical optimization problems by mapping them onto a set of interconnected qubits and then searching for a solution (or solutions) that minimize the total energy of the system.

This is an application-specific computer that is used solely to solve classical optimization problems. Quantum annealers do not use digital gates. Rather, an optimization problem is encoded directly into the qubits and their interactions (qubit interaction is referred to as coupling). Minimizing the total energy of the system is equivalent to optimizing and finding the answer to an encoded problem.

A quick explanation of some physics jargon: Every quantum system has some amount of energy that is determined by the state of each object (e.g., each qubit) in the system. The state of each individual object, in turn, determines the global state of the entire system, and each global state corresponds to the system having a particular total energy. The Hamiltonian is a function that matches each state to an energy value. So, for our purposes, the Hamiltonian is a function that tells us that our qubits are in the state $|\psi\rangle$ and that our system has energy $E\psi$. The state that gives the lowest energy for a system is called the ground state.

The idea behind quantum annealers is that if you have a quantum system governed by one Hamiltonian, it is possible to transfer the system to another Hamiltonian by changing the system's parameters.

Quantum annealing starts with a qubit state configuration for which the ground state is well-known. This is the starting configuration or starting Hamiltonian, say $H_s(s) = -\frac{1}{2}\Sigma_i\Delta(s)\sigma_i^x$. By gradually turning off the starting Hamiltonian while turning on the problem Hamiltonian, one can transition to the configuration that encodes the problem. If this transition is performed slowly enough and the system has no noise, then the computer will remain in its ground state throughout the evolution. Measuring the state of the computer in the final configuration yields the answer to the problem, $H_f(s) = \epsilon(s)(-\Sigma_ih_i\sigma_i^z + \Sigma_{i<j} J_{ij}\sigma_i^z\sigma_j^z)$. This is the description of an adiabatic quantum computer.

The challenge is that for problems of interesting size and complexity, it is almost certain that no matter how slowly the coupling parameters are changed, the annealer will leave its ground state. There are two primary reasons for this: first, the minimum gap between the ground-state energy and the nearby excited-state energy levels gets very small as the size of the problem increases; and second, there is always noise in the system due to nonzero temperature. Although operation in a cryogenic environment reduces noise, it does not eliminate it entirely. And, as the minimum gap gets smaller, eventually, even cryogenic temperatures begin to feel relatively warm.

Consequently, quantum annealers must find additional mechanisms, such as quantum tunneling or excess relaxation, to keep the computer in the ground state. However, too much relaxation will make the computer manifestly classical. As a result, it is presently unknown whether a quantum annealer can exhibit quantum enhancement for a general class of optimization problems, or whether it is simply another type of classical computer. Nevertheless, there is intense research into quantum annealers today because the application pull is so strong. Many real-world problems can be cast as optimization problems (e.g., financial portfolio optimization, product distribution, routing of autonomous vehicles), so there is a strong motivation to understand quantum annealers better.

```fix
Quantum annealing is a method for finding solutions to classical optimization problems.

The quantum annealer encodes the problem onto a set of qubits and biases those qubits into a starting state with a known ground state (that is not the solution to the problem).

The system is then slowly evolved toward a set of bias points that represent the problem being solved, where the ground state of the system is the answer to the problem.

However, a quantum annealer is likely to leave its ground state during this evolution.

The concept is that, despite a combination of quantum tunneling and relaxation processes,

the system should find its way back to the ground state (or a lower-energy state)

to find the encoded (approximate) solution to the optimization problem.
```

```fix
In Richard Feynman's own words, "I'm not happy with all the analyses that go with just the classical theory because nature isn't classical, dammit, and if you want to make a simulation of nature, you'd better make it quantum mechanical, and, by golly, it's a wonderful problem because it doesn't look so easy. Thank you." Here, you can read his original paper and watch a TEDxCaltech video about his life narrated by the physicist Leonard Susskind.
```

```fix
After the publication of Shor's algorithm and the increased focus on quantum error correction, the number of publications in quantum computing increased dramatically, as did the levels of interest and funding from national governments.
```

```fix
In the figure for this Bloch sphere question, the z-axis goes from the south to the north pole of the sphere. The x- and y-axes are in the transverse plane of the sphere that includes the equator. Here, y goes from left to right, and x is perpendicular to z and y. The axis labels are placed next to their corresponding arrows. The orange vector is the Bloch vector; it starts in the center of the sphere and ends at its surface. The Bloch vector representing state |0⟩ starts in the center of the sphere and ends at the north pole. The vector representing state |1⟩ starts in the center of the sphere and ends at the south pole. The polar angle is measured from the positive z-axis to the tip of the vector representing state |ψ⟩, meaning that it runs from zero to the maximum value of π. The azimuthal angle is measured from the positive x-axis to the projection of the Bloch vector representing state |ψ⟩ onto the transverse x−y plane, indicating that it takes values from zero to 2π.
```

```fix
A two-qubit system can have aspects of four classical states.

A π/2-pulse along the x-axis transforms a spin-up or a spin-down into an equal superposition of up and down.
```

```fix
In this type of gate, the operation applied to the target qubit is conditioned on the state of the control qubit. In the case of the CNOT gate, the target qubit is flipped (an X gate is applied) if the control qubit is in state |1>.
```

```fix
A universal set of quantum gates generally includes both single-qubit and two-qubit gates.

The input state of a quantum computer is not limited to a particular number of qubits.
```

```fix
Quantum annealing does not use digital gates but rather follows an annealing protocol that turns off the initial Hamiltonian while simultaneously turning on the final Hamiltonian that encodes the answer to a problem. It starts in a state with a well-known ground state, and it has not been proven that quantum annealers can exhibit quantum speed-up.
```