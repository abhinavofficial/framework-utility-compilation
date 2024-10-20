# Modern System Architecture

System architecture is the study of early decision-making in complex systems. We will first learn how to capture experience and analysis about early system decisions. We will also learn you how to choose architectures that meet stakeholder needs, integrate easily, and evolve flexibly.

Most importantly, we’ll examine how to think, not what to think. The goal is to help develop a way of thinking about and implementing system architecture by placing it in the context of value and competitive advantage for the product and organization.

We’ll learn to manage the evolution of system complexity and account for future uncertainty so goals are met and functions are delivered. This will also ensure that the system remains fully comprehensible during its design, implementation, operation, and evolution.

The framework starts by defining the solution-neutral function based on the underlying need. Then comes the concept, which is generated based on the solution-neutral function. Finally, the architecture is created based on the concept.

This tool is called Design Structure Matrix (DSM). DSM helps you understand the interactions among entities, subsystems, and components. It also allows you to manage these interactions to align product development with the goals and needs of the system.

## Key Ideas

Function to architecture. Solution neutral functions. Concept.

The requirement should be provided in a solution neutral way, so it allows designers to bring in different ways to solve a problem.

Definitions:

* **Architecture** is a representation of entities organized in a way that supports reasoning about the entities and describes behaviors and relationships amongst the entities.
* A **concept** is a product or system vision, idea, notion, or mental image that maps function to form in brief. It is a simplification of the system architecture that allows for high-level reasoning.
* A **solution-neutral** function explains a function without specifying the solution that allows one to achieve that function. Solution-neutral is not an absolute property but rather a spectrum of more solution-neutral to less solution-neutral.

Examples of solution-neutral and solution-specific

|Solution-Neutral       |Solution-Specific              |
|-----------------------|-------------------------------|
|Controlling a vehicle  |Steering a vehicle             |
|Sorting an Array       |Sequentially exchanging entries|
|Identifying a package  |Lidar Scanning                 |
|Air pressure regulating|Actuating Blow Off value       |

Please note: Solution-neutral is relative, not absolute. Any requirement is more or less solution-neutral relative to another possible requirement. This spectrum is illustrated by the range of problem framings given in the video for a wine bottle opener.

> * Solution-neutral function is stated without reference to how the function is achieved.
> * Functional intent for a system should be stated as a solution-neutral function.
> * A solution-neutral statement doesn’t contain any reference to a specific product/process, so it doesn’t set the design parameters.

Less solution-neutral is focus and team's exploratory perspective is constrained. Useful is responding to a competition or challenge.

### Concept

You're already familiar with concepts such as the vision, or the idea, or the kernel at the heart of the architecture. Professor Cameron, from MIT xPro argues that concept requires both function (process) and form (operand) and defines it as the mapping of function to form.

If you list down the number of functions you need to perform, and all the forms that can do these functions individually, we can generate a lot of concepts using their combinations.

|x                         |Operand|Process |Instrument|
|--------------------------|-------|--------|----------|
|Operand/Process/Instrument|light  |emitting|diode     |
|Operand/Process           |lawn   |mow     |(er)      |
|Operand/Instrument        |light  |        |bulb      |
|Process/Instrument        |       |carrying|case      |

The concept name can lock us in some kind of solution.

Architect Steve Emerie rationalizes the structure of the architecture.

In summary, a concept:

* Is the transition point from the solution-neutral to the solution-specific.
* Must allow for value-related functions to be executed and enabled by form.
* Establishes the vocabulary for the solution and is the beginning of the development of architecture.
* Implicitly sets the design parameters of the system.
* Implicitly sets the level of technology

## System Architecture

The system concept is simple: It has a low amount of information and complexity. The architecture is quite complex — a lot of information involved.

Stepping Stones in Synthesis

1. Needs - It is vague. It is in the language in common people. It **answers why** are doing.
2. Functional goals - **Solution-neutral vocabulary**, which can be defined by someone in the system. It **answers what** is the system going to do.
3. Concept - **How** will the system perform the functional goals.
4. Architecture - It defines the concept in a **solution-specific vocabulary**.

Concept informs architecture. It not only conveys information about form but also contains information about function. Solution-neutral format by definition does not reference a specific solution. It scopes how broad a design exploration will be. Therefore, it is possible to use a solution-neutral statement that is too broad, which may encourage a team to embark on an exploration that takes too much design time.

See an example of system architecture solving a simple need to removing water from the basement.

![complexity_explained](images/relation_between_concept_architecture.png)

### Decisioning in System Architecture

We have no reason to expect the quality of intuition to improve with the importance of the problem. Perhaps the contrary: high-stake problems are likely to involve powerful emotions and strong impulses to action - Daniel Kahneman.

Engineers must make many design decisions. A subset of these design decisions are architectural decisions — arguably the most important decisions. They relate to

* Mapping function to form
* Determining the performance envelope
* Encoding the key trade-offs that become the eventual product.

Lets introduce two more concerns here which would influence decisions feeding into System Architecture - Sensitivity and Connectivity.

**Sensitivity** is a measure of the impact on metrics caused by a given decision.
**Connectivity** is the degree to which making a given decision influences other decisions. This shows how much rework would be required to make change the decision. A connected architectural decisions affect multiple subsystems or components within the system. In such cases, changing a connected architectural decision is more difficult as time progresses and things are added to the system.

Professor Cameron argues that with two critical criteria you can highlight the architectural decisions for the system. Those architectural decisions should be both highly sensitive as well as highly connected.

![Architectural Decisions](images/architectural_decisions.png)

Quadrant 1 is what we want to solve first - they would help fix my design. Quandrant 4 can be done at the last - these decisions can be done downstream.

## Design Structure Matrix

You can learn more about DSM at their [website](https://dsmweb.org/)

Design Structure Matrix is a method for mapping out or modeling networks of interactions in complex engineering systems. This is used to better manage the development of extensive development projects, which are naturally decomposed or split into different pieces.

- **Decomposition** is the breaking a larger entity into smaller entities.
- **Process** is a pattern of transformation applied to one or more objects. Generally, processes are creating, destroying, or changing something. They occur over time.
- **Design Structure Matrix (DSM)** is a simple complacent representation of a system or project in the form of a square matrix. It is the equivalent of an adjacency matrix in graph theory and is used in system engineering and project management to model the structure of complex systems or processes.
- **System architecture** is the embodiment of the concept that the allocation of physical information function to the elements of form and the definition of relationship among the elements and with the surrounding context.
- **Interface** is a point where two systems meet and interact. 
- **Process architecture** is the structural design of general process systems.

Product - Is decomposed into Sub-System and Components
Process - Is decomposed into Phases and Stages, finally into Work packages and Tasks. Planning-> Concept Development -> System-Level Design -> Design -> Build -> Test -> Production ramp-up
Organization -> Is decomposed into people and team to execute the process to create the system.

### Type of DSM

**Product/System DSM** is a square matrix which lists components on x and y axis, and denoting interfaces with x. You would note a pattern of how x are placed, and some related components may denote the sub-systems, and common or integrating components which run throughout the system.

**Process Architecture DSM** is an activity to activity square matrix.

**Organizational DSM**

### How does this work?
DSM is represented as a square N x N matrix, mapping the interactions among a set of N system elements. The system elements are often labeled in the rows to the left of the matrix and in the columns above the matrix. 

The off-diagonal elements are used to indicate relationships between the elements. A marking of the cell indicates a directed link between two elements. It can represent design relations or constraints between product components, communication between teams, information flow, or relations between activities. In one convention, reading across a row reveals the inputs that the element in that row receives from other elements. Scanning a column reveals the outputs that the element in that column provides to other elements. Alternatively, the rows and columns may be switched (without a change of meaning). Both conventions may be found in the literature and convey the same information. 

Here, you will begin by adopting the convention with outputs in rows and inputs in columns. For example, in the figure below, the eight system elements are labeled A–H across both rows and columns. Reading across Row B, you’ll see that B has inputs from D, F, and G represented by “X” marks in Row B. Reading down Column F, you’ll see that F has outputs going to B and D. Thus, the mark in off-diagonal cell (B, F) represents an interaction that is both an input and an output, depending on whether one takes the perspective of its provider (Column F) or receiver (Row B).

|x|A|B|C|D|E|F|G|H|
|-|-|-|-|-|-|-|-|-|
|A|A| | | |X| | | |
|B| |B| |X| |X|X| |
|C| | |C|X| | | |X|
|D|X|X| |D| | | | |
|E| |X| | |E| | | |
|F| | | | |X|F| | |
|G| | |X| | | |G| |
|H| | | | |X| | |H|

> A DSM can be sorted to identify potential groups that have more connections within the group and fewer connections outside the group.

> Processes are generally feed forward in nature (represented by marks below the diagonal). Feedback processes that return some input back to an upstream task (represented by marks above the diagonal) are rarer. Mainly validation and verification tasks have a feedback nature. Hence, Process DSMs have more marks below the diagonal than above the diagonal in the DSM square matrix.

> System architecture DSMs are mostly symmetric. We represent interfaces or interactions between the components, which are generally symmetric in nature. If one component has an interface with another, then vice versa is also true.

> Process architecture DSMs are mostly asymmetric. Processes are mostly feed forward in nature, i.e., one process's output is required as input for another process. Information, in this case, may not be passed in the opposite direction.

> A system architecture DSM is often called an N-squared matrix. This refers to the fact that the matrix is square — it has the same number of rows and columns. As shown in the video, it can be used to visualize relationships and with clustering algorithms to produce potential modularizations of the system.


### Advantages of Design Structure Matrix

Within the suites of potential system analysis tools, DSM offers some salient advantages:

- **Conciseness**: The structured N x N arrangement of elements and interactions provide a compact representation.
- **Visualization**: By looking at a DSM matrix alone, a designer can distinguish relationship patterns of particular interest. For example, a DSM may show regions of heavy interactions indicative of assigning particular components to a module.
- **Comprehension**: It’s easy to understand. Hierarchy and complexity become apparent in even a cursory review of the DSM.
- **Analysis and optimization potential**: The matrix-based nature of DSM allows application to a number of power analyses in graph theory, matrix mathematics, and specialized DSM analysis methods. These tools can be used to optimize the structure, modularity, and other important patterns and effects.

### Creating a System Architecture DSM

The steps covered can be generalized to provide a basic framework for building a product architecture DSM:
1. Decompose the system into elements.
2. Understand and document the interactions between the elements.
3. Layout the square DSM with components, labeling the rows and the columns.
4. Analyze potential groupings of the elements via clustering. The clustering should be done in such a way that it promotes interactions within the cluster and minimizes outside ones.

### Process Architecture DSM

Process Architecture represents the flow of information between tasks that makes up the development process. There are three kinds of relationship possible between two tasks

* Sequential: Task A provides information to task B, and hence must complete before B. Task B is dependent on A, and A and B are therefore, sequential.
* Parallel: If Task A and B are independent of each other, so both can be done at once in parallel.
* Coupled: Task A and B can be interdependent - this means, A needs something from B and B needs something from A. They must be done together and hence are coupled.

In Process Architecture DSM, the inputs are in the row and outputs are in the column. In the diagram below, Task D requires input from E, F, and H. Task B's output goes to C, E and G  

|x|A|B|C|D|E|F|G|H|
|-|-|-|-|-|-|-|-|-|
|A|A| | | |X| | | |
|B| |B| |X| |X|X| |
|C| |X|C|X| | | |X|
|D| | | |D|X|X| |X|
|E| |X| | |E| | | |
|F| | | | |X|F| | |
|G| |X|X| | | |G| |
|H| | | | |X| | |H|

Which task should go first? Well, it may the one with no input or minimum inputs. Once that task is complete, we can find the ones which needs the output of this with no other or minimum inputs. We can thereby sequence the entire process. Once you put the DSM in that sequence, we can move across to diagonal to see when the task can be done in sequence, parallel, or are coupled. So, process DSM can help identify -
1. Tasks that can be executed in parallel
2. The order in which tasks should be executed
3. Tasks that have to be performed together as a logical group because of their dependencies

Lets create the algorithm for the same.

#### DSM Sequencing model

1. Find any empty rows and move them to the front.
2. Find any empty columns and move them to the end.
3. Find any loop (if A depends of B and B depends on A, they are looped), collapse it, and schedule it according to step 1 and 2, if possible.
4. Repeat steps 1-3 until all the tasks and loops have been sequenced.

// Task - Create a python program to do so.

Process DSM allows us to answer the question - where are the iterations in an engineering process? There are planned and unplanned iterations. Planned iterations are the ones where we group task together to iterate to get the best outcome. Our job to make them happen faster. Unplanned iterations are hard to predict when it would take place. We can plan to remove or plan not to create the knock down impact.

> Any activity that lies above the diagonal in the process DSM is a feedback activity.

> Any feedback activity that is not part of a logical group leads to unplanned iteration.

> Any feedforward or feedback activities happening inside a logical group are part of planned iterations.

> Sometimes if a process contains all uncoupled tasks, then restructuring the DSM will not lead to other improvements. Rearranging the DSM may not always help. If a process contains only uncoupled tasks, then restructuring the DSM will not lead to other improvements. Note that the use of logical groups defines what is planned/unplanned. Therefore, what is outside the logical groups is unplanned. For iterative processes, in theory you could illustrate the whole DSM as a logical group, but in practice when performing tasks, in the second iteration there may be differences from the first iteration.

#### How to create a process Architecture DSM

1. Identify the process or of the sub-process to model - the inner working of the sub-systems
2. Identify the tasks and who is responsible for each one
3. List the outputs created by each task
4. Identify the inputs for each task
5. We can then mark X to match "input requirement for" (rows) and "output created by" (columns).
6. We can then mark 0 or something to mark special flows like unplanned iteration - they would be typical above the diagnal.
7. You can then sequence tasks differently, use different colors, markers, etc. to make sense for yourself. There is no right or wrong ones here.

## Decomposing a System

Architect decides on the decomposition of a system. It is a divide and conquer rule. Once they decide, many things emerge -
1. Interfaces
2. Approaches to testing
3. Ease of integration
4. Approaches to service

Two key decisions are to be made. The core decision is how many pieces do you want to decompose into, typically 7 +- 2. But the more subtle decision is to determine which **plane** you want to use - is it based on **form**, **function**, **suppliers**, **sequence of operations**, etc. We will get different decomposition based on what we select.

Irrespective of how we decompose, all of these need to derive from top-level system performance requirement.

To decompose, we can use **Two Down, One Up** principle that says the best information about how to cluster or group the system one level down from the reference is present in the structure and interactions two levels down. Therefore, you need to drill down to Level Two to better identify the Level One decomposition.

Level 0 -> Trial Level 1 decomposition -> Trial Level 2 decomposition -> Apply Clustering Algorithm as described in DSM process -> Final Level 1 decomposition

### Key take away

We want to modularize the system, keeping the following things mind -

1. Reduce and simplify the interfaces
2. Reduce the functional interactions and ensure emergence can take place
3. Modularize for easy implementation, integration, operation, and service
4. System changeability over lifetime
5. Modularize against legacy elements and suppliers

> Remember good modularization is the key to managing complexity in system.

We can have a great architecture if we can -
1. Decompose that modularize in the same way across all the planes
2. Create simple interfaces
3. Have teams aligned with the modules

## Change Propagation

Most products and systems emerge from predecessors and not through clean-sheet design. Understanding how and why changes propagate during engineering design is, therefore, critically important. Even new product development projects are iterative in nature with many build and test cycles that may result in frequent requirement updates. This can cause change propagation within the system. A change request can potentially propagate through a system to other components, leading to increases in project cost and lead time.

**Change propagation** is the process by which change to one part or element of an existing system configuration or design results in one or more additional changes to that same system.

Change propagation often exceeds two generations (child to child) but rarely exceed five generations (child to child to child to child to child). It follow Log-linear distribution - any change deeper than 5 generation are significantly small to be made.

Understanding change propagation impact is important to ensure that we are able to deliver the value (emergence) by the system with time and budget.

// To read - how to predict this efficiently.

It is important to track and document Engineering change history. We can them have an approach to engineering change analysis.

Three dimensions to capture in Engineering change history.

1. Time: Change activity over project timeline. If we draw a 2-D plot of change of necessary changes vs time, we may see three patterns. Most changes occur before the finalization of design and tapper down very quickly after the initial set of changes, these are probably good and are called Ripple. We may see another kind where changes occur before finalization of design, but the changes does not really tapper until the middle of the design and may have at the fag end as well. These are called Blossom and not really considered good. The worst kind is when the changes continue to happen even after the change and are called Avalanches.
2. Change Location: We would like to understand when and where these change are happens. It helps identifying the hotspots in the system.
3. Cost: Cost distributions. We want to look at which changes are causing additional cost or scope, changes which reduces cost/scope neutral, and changes with are cost/scope neutral against time. It is generally found that any change post design phase and when the build starts increase cost/scope. There is a cost estimate bias - we tend to underestimate the cost escalation due to change, and overestimate the cost saving due to change.

![Engineering Change Analysis](images/engineering_change_analysis.png)

We should also look into social network of people involved in a system, and the role some of integration/system engineers play during the change process.

## Value based decisioning

**Value** is the experienced net benefit at a cost, taking into account the importance and scarcity, that will result from the system. Different stakeholders will perceive value differently and it is likely to vary over time.

When making decisions, people often focus on a single criterion like price, but most decisions actually involve multiple, competing criteria. For instance, buying groceries involves both price and quality. In complex systems like designing a smartphone, multiple stakeholders have different needs. Users prioritize factors like battery life and camera quality, while manufacturers focus on production costs and timelines. It’s challenging to balance these diverse needs, so a structured process is needed to efficiently compare attributes, identify conflicts and synergies, and make better decisions.

### Traceability and Communication Support
One of the key benefits of engaging with stakeholders early and using value-focused thinking is that it increases the likelihood of success at decision time. Most systems engineering processes have key milestone reviews, the outcome of which are critical decisions that shape what a system can end up becoming. These include decisions such as concept selection, budget, and schedule allocation; requirements encoding; assigning specifications to a design; verification that a particular design meets requirements; and validation that a particular system meets the espoused needs.

Every organization may have a different set of processes and key milestones; however, usually, these are the critical times when preferences (i.e., the values) of critical stakeholders are made known. Sometimes key stakeholder needs are not even known until later in the lifecycle (i.e., new customers using a product after development). In order to reduce delays in the system development processes and increase the chances that the proposed system solutions meet or exceed expectations, you can use quantitative methods to generate compelling and aligned evidence that you are developing good solutions. For example, using quantitative value models as proxies for stakeholder needs ensures alignment of your development efforts in between those critical milestone reviews.


An **attribute** is a decision maker’s perceived metric of how well a perceived objective is met.

The Pareto frontier by definition contains alternatives that are the most efficient for a given cost. Therefore, if an alternative has a higher benefit than other alternatives at a given cost, then that alternative will now be a part of the Pareto frontier.

**Budget** and **desires constraints** represent limitations on what is acceptable for the project and system, and they can be used to eliminate alternatives from the decision analysis, since some alternatives will not satisfy them.

Dominated solutions are not the most optimal from the set of all of the available solutions, and therefore they don't maximize the value that could be derived from the system; hence, they could be eliminated. However, if you have any uncertainty about the value metrics, the performance, or cost of the alternatives, or wish to gain insights into the relationships within the tradespace, you should not eliminate dominated solutions, since they might become non-dominated at a later point in time (to be discussed in the coming weeks).



Multi-Attribute Tradespace exploration
* Structured framework
* Large number of alternatives
* Useful mechanism for stakeholder communication

Finding alternatives on the basis of experience, media, reports, etc.

Differentiating factors:

- Price
- Styling
- Reliability

Finding alternatives, and creating differentiation takes time and effort.

Value focused thinking
1. Starts with rationales
2. How do you create value?
3. What is the problem you are trying to solve?
4. Ideally these are solution neutral.

|Value-Focused|Alternative-Focused|
|-------------|-------------------|
|Focuses on value-creating rationale and tries to identify how to maximize the value derived from the system|Tries to identify the best possible solution/answers/designs/alternative for the problem from a set of available alternatives|
|Starts with the problem|Starts with a potential solution|
|Tends to be framed as solution-neutral|Tends to be framed as solution-specific|
|Tends to take longer|Tends to be quicker|
|We can recast problems into opportunities. It increases likelyhood for solution performance. It also helps in re-aligning scare resources.| It may start with small number of solutions - some of which may be sub-optimal. However, it reduces ambiguity. It may look like solving the problem, but may lead to scare resources and properly justification of why we opted for an alternative|

Value-focused thinking tries to formulate the problem in a more solution-neutral manner, which helps in generating new and innovative solutions that better solve the problem. For example, if your goal is affordable, on-demand transportation from point A to B, then value-focused thinking might ultimately result in the suggestion that you use Uber and Zipcar rather than selecting from a set of car purchase alternatives, since they help save the up-front cost of owning a vehicle. Value-focused thinking tries to provide better solutions for the problem, not necessarily more solutions.

Alternative-focused thinking identifies potential solutions for the problem up front and then compares alternatives to select the one that best solves the given problem. This ensures that it is easier to implement, since concrete solutions are provided up front, and this also helps reduce the ambiguity from the problem being solved. Alternative-focused thinking usually results in a small number of possibly sub-par solutions that are restricted to specific concepts or architectures. Value-focused thinking does not suffer from the same problem and might provide more solutions.

### Evidence Based Systems Engineering

Evidence-based SE is an extension of model-based SE that emphasizes not only using SysML or other system models as a basis of program decisions but also the use of other models to produce evidence that the system models describe a feasible system. Such evidence is generally desired, but it is often not produced because it is not identified as a project deliverable in a data item description (DID). Going forward with such unproven solutions frequently leads to large program overruns.

Based on experience in developing and using such a DID on a very large project, let’s summarize the content and form of such a DID and a rationale for its use. Its basic content is evidence that if a system were produced with the specified architecture, it would:

- Satisfy the specified operational concept and requirements;
- Be developable within the specified budget and schedule;
- Provide a superior return on investment over alternatives in terms of mission effectiveness; and
- Provide satisfactory outcomes for the system's success-critical stakeholders.

One key factor of the DID is that the content of the evidence should be risk-balanced between having too little evidence (often the case today) and having too much (analysis paralysis). Thus, it is not a one-size-fits-all DID, but it is one that has ways to be tailored to a project's most critical evidence needs.

Source - [DID, Conference of Systems Engineering Research](https://www.sciencedirect.com/science/article/pii/S1877050913000951?via%3Dihub)

### Multi-Attribute Tradespace Exploration with Concurrent Design as Value-Centric Framework

Abstract: The complexity inherent in space systems necessarily requires intense expenditures of resources both human and monetary. The high level of ambiguity present in the early design phases of these systems causes long, highly iterative, and costly design cycles, especially due to the need to create robust systems that are inaccessible after deployment. This thesis looks at incorporating decision theory methods into the early design processes to streamline communication of wants and needs among stakeholders and between levels of design. Communication channeled through formal utility interviews and analysis enables engineers to better understand the key drivers for the system and allows for a broad and more thorough exploration of the design tradespace. 

Multi-attribute tradespace exploration (MATE), an evolving process incorporating decision theory into model and simulation-based design, has been applied to several space system projects. The conclusions of these studies indicate that this process can improve the quality of communication to more quickly resolve project ambiguity and enable the engineer to discover better value designs for multiple stakeholders. Sets of design options, as opposed to point designs, in addition to the structure of the solution space, can be analyzed and communicated through the output of this process. 

MATE is also being integrated into a concurrent design environment to facilitate the transfer of knowledge of important drivers into higher fidelity design phases. Formal utility theory provides a mechanism to bridge the language barrier between experts of different backgrounds and differing needs (e.g., scientists, engineers, and managers). MATE with concurrent design (MATE-CON) couples decision makers more closely to the design and, most importantly, maintains their presence between formal reviews. The presence of a MATE-CON chair in the concurrent design environment represents a unique contribution of this process. In addition to the development of the process itself, this thesis uses design structure matrix (DSM) analysis to compare the structure of the MATE-CON process to that of the NASA systems engineering process and that of a US space organization to gain insights into their relative temporal performance. Through both qualitative and quantitative discussions, the MATE-CON process, which is derived from the fundamental concept of engineering, is shown to be a “better” method for delivering value to key decision makers. 

Source - Tradespace Exploration - Local copy.

Now lets move to overview of Value Models

### Overview of Design Value Loop

![Design_Value_Loop](images/Design_Value_Loop.png)

- **Design value loop perspective** is a problem-to-solution decision-making approach that uses three types of models in two categories — evaluative models (performance and cost) and value models — to encourage feedback and the explicit separation of objective (e.g., evaluative models) and subjective (e.g., value) considerations. The goal in this perspective is to find alternatives with a design space that best satisfies expectations on the value space. The basic design value loop includes a design space that contains all the proposed solutions that could address the problem. In order to evaluate the design space, let’s split the evaluation space further into three spaces: the resource space, the performance space, and the value space. The resource space defines what resources or costs are involved in solving the particular problem. These costs and resources are calculated using the cost model. Next is the performance space, which defines the performance criteria that the potential design solutions should meet in order to be considered. These performance criteria are calculated using certain performance models. The third and final one is the value space, which defines the set of value related attributes and criteria to evaluate the potential design space. These value criteria are over and above the resource and performance criteria mentioned earlier, and these attributes are identified using value models, which will be discussed in more detail in the next sections.
- **Design space** is the span of possible alternative solutions to the system design problem, from which one or more could be picked and are under the control of the designer, usually consisting of distinct concepts, architectures, and particular designs.
- **Cost models** evaluate potential design in terms of the resources (usually different types of cost, such as design, develop, manufacture, test, operate, and lifecycle) required for their realization.
- **Performance models** evaluate potential design in terms of capabilities or performance they provide to help address the underlying goals and objectives. These are usually related to the behavior of the design, such as top speed, range, efficiency, etc.
- **Value models** assign quantitative scores to potential design in terms of the perceived satisfaction, or benefit at cost, they generate while addressing the underlying goals and objectives. 

