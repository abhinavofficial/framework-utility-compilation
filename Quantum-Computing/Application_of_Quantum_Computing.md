# Quantum Simulation/Emulation

Quantum simulators investigate the properties of quantum mechanical systems to understand how they work and to predict their behavior. The simulations can be implemented using analog, digital, or hybrid analog-digital means. Specifically, digital simulations are implemented by applying single- and two-qubit gates in quantum physical systems as in the universal model of quantum computation.

```fix
Quantum computers can simulate a quantum system using both digital gates and analog evolution. They can also emulate a quantum system by tailoring qubits and their connectivity.
```

Quantum computers can be applied to many types of simulation problems. The challenge is that many of these problems require large numbers of well-behaved qubits, and such large-scale quantum computers are still likely a decade or more away. Before technology gets to the point where it can simulate systems entirely on quantum computers, there may be room for hybrid classical-quantum systems to hold a quantum advantage over purely classical algorithms. One example is called a _variational quantum eigensolver_ (VQE) .

A VQE  determines the lowest-energy state of a quantum system, a difficult task of importance to quantum chemistry. The protocol goes as follows:

- The quantum processor is set so that it simulates the dynamics of the quantum system of interest.
- The classical computer proposes a trial ground state that is then generated in the quantum processor using parameterized quantum gate operations.
- The quantum processor measures observables that are associated with the energy of that state and passes them back to the classical computer.
- The classical computer takes the measurement results, calculates the value for the energy, and uses it to generate parameters for the generation of a new ground trial state that should give a lower energy.
- This process repeats until the VQE cannot find a new trial state that gives a yet lower energy. This state is then likely the lowest-energy state of the system.

A key aspect of this process is that the classical computer is not merely being used to initialize a quantum computation; the classical and quantum computers are passing information back and forth throughout the simulation. Therefore, the quantum computer is acting as a coprocessor. It need not be coherent over the entire duration of the simulation but only over the period of time it takes to produce a single trial state and obtain the necessary observables to calculate its energy (via repeatedly producing the trial state and measuring its energy). This approach may allow useful computations to be done in the nearer term using less reliable qubits.

The VQE method has been shown to successfully determine the molecular spectra of a hydrogen molecule ( H2) . To simulate the hydrogen molecule composed of two electrons, a quantum processor with two superconducting qubits or two trapped ions is sufficient. It has also been used to simulate other small molecules, such as  LiH and  BeH2 . All these simulations to date are prototype problems that demonstrate the mechanics of the quantum simulation, but the problems themselves are easily solved on a classical computer, allowing them to be used as benchmarks for the quality of the quantum processor. As the number of qubits increases, VQE can be applied to larger, more challenging simulations.

The first proof of principle demonstrations of VQE on scalable quantum devices were realized in the following publications:

- [Scalable Quantum Simulation of Molecular Energies](https://journals.aps.org/prx/abstract/10.1103/PhysRevX.6.031007)
- [Hardware-Efficient Variational Quantum Eigensolver for Small Molecules and Quantum Magnets](https://arxiv.org/abs/1704.05018)
- [Quantum Chemistry Calculations on a Trapped-Ion Quantum Simulator](https://journals.aps.org/prx/abstract/10.1103/PhysRevX.8.031022)