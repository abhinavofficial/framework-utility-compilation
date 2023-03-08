# Start Small

## Context
As stated earlier, in the early stages of a project, original developers experiment with new ideas and making decisions based on what they want. As the project increases in popularity, they’ll find themselves working with their users and contributors more.

Maintaining a project requires more than code. These tasks are often unexpected, but they’re just as important to a growing project. It is important therefore that we get some basic things right make life easier later, from documenting processes to leveraging the community.

Start with one project. However, recognize the set of challenges which were highlighted in the Challenges section. The idea is to implement all the best practices within one project, so it can be used as a model project.

## Define the vision
Start by writing down the goals of your project. Add them to your README, or create a separate file called VISION.

Having a clear, documented vision keeps you focused and helps you avoid “scope creep” from others’ contributions.

While vision can change over the course of time, it must be done unanimously by all the maintainers. In case of split, the project should be forked into another project.

## Define project roadmap
A publicly accessible project roadmap helps tremendously. This roadmap defines key features to be incorporated along with its tentative dates.

The roadmap should minimally project for three release cycles if project's release cycle is monthly. The roadmap is matured using ideas / enhancements proposed by maintainers, contributors and users using [MoSCoW](https://en.wikipedia.org/wiki/MoSCoW_method) or [RICE](https://productfolio.com/rice-scoring/) or any other of several good strategies as suited.

The roadmap is sealed using simple majority based consensus voting - maintainers can influence in case of split-head scenario. 

## Establish way of working
It would be extremely important to establish the rhythm for collaboration. The purpose is to
* Gain consensus on new idea. The documentation around the idea stating value, change required and NFRs should be present in the confluence earmarked for project.
* Discuss or Clarify for specific problem in the agenda. Participants are encouraged to get their specific problem enlisted on the meeting agenda.

It would be weekly at most.

A monthly knowledge sharing session is another important cadence amongst the participants and users. Each use case brings more value to the project.

A distribution list to include the community member for the project is a useful way to communicate. Microsoft team or any other such communication platform can also be used to communicate. Another alternative can be use of StackOverflow, where people can ask questions and get answers to.

## Create the Kanban board to implement the changes
A publicly accessible Kanban board is a must. It can be on JIRA or GitLab. This the single version of truth for feature or bug list. Define clearly what each of the fields in the issue means, and how they should be populated. This allows the project to define new JIRA types, affected version(s), priority, fix version(s), resolution, state, components, labels, language, etc. - they just need to define each of the attributes and its valid values.

This board is not meant to collect ideas, raise questions, etc.

Idea collection should happen within a confluence site under the project. Questions regarding usage can be managed on communication platform as defined in Way of Working.

## Define practices for versioning, release cycle and support model
It is important to define version - In general, 

For **framework, utility and tools** projects, we can follow major.minor.patch strategy

Patch is typically bug fixes and offers no change in feature. Users can confident upgrade without a need to full-fledged testing - a minor shakedown testing is recommended.

Minor release will include incremental features and upgrade to existing feature. This should have full backward compatibility with its two previous versions. User should be able to upgrade with a regression test and full functional test if they would like to use functionality.

Major release may include breaking changes. This typically will require a well planned user upgrade. A careful build-out following SOLID principle can reduce the effort significantly.

For **services** projects, the strategy can be based on microservices versioning - Example - https://www.deployhub.com/microservice-version-management/

## Establish the minimum toolset required to work on the project
It becomes extremely important to define what the minimal toolsets required are to successfully contribute to project.
> Please note contribution is not only contributing by code. It can be as simple as fixing a documentation, or testing a release candidate, or reporting a bug, or reviewing a code change, or as simple as helping others by responding to their questions.

So, toolset may include access to DL, Stackoverflow, confluence page, code repo, etc. One good strategy is to keep access open for Stackoverflow, Confluence page, code repo, etc. and a very simple process to add oneself to DL.

## Define how to run and build
You should have a CONTRIBUTING file (recommending a markdown file), which has a section on how to build and run the project, and how to run without building.

The project should have a build module, which helps install all requirements, like correct version of Java, Scala, Python, etc. The project may not additionally think about supporting the project on Linux, Windows or Mac systems - this is really nice to have. Since most members of the organization are Windows based - it should help users to get access to appropriate OS setup, say WSL(2) set up on Windows for Linux based applications (and call out limitation clearly).

This module should also a means to install executable (distributable binaries - executable jar, .exe), which people can directly set up, and test and report bugs.

CONTRIBUTING file should clearly link the build/run section to project build module.

## Define standards of code, test and documentation
CONTRIBUTING file should clearly define standards of code build, test and documenting the code. 

Ideally these should be integrated in project build, say via maven or sbt or any other suitable build tool. Each programming language has a style checks. For example, scala has scala-checkstyle, python has PEP8, etc. for code standards. Sonarqube also provides very useful code analysis tools.

Unit testing is very important and test coverage above 95% is a minimum desired state. It is worth noting that building out tests often takes more time and is more complex than building the code itself. Property based test can be used for frameworks for data driven applications - this ensures that the framework addresses all possible use cases it can be used for. This is one of the MOST CRITICAL aspects of any inner source projects - it would allow to upgrade and distribute next versions, and yet maintain backward compatibility or detect breaking changes for further communication. Each programming language provides unit test frameworks, ScalaTest for Scala, Junit for Java, PyTest for python, etc. 

A good project should also have vulnerability check within code build itself.

## Define strategy for error message and handling
CONTRIBUTING file should also clearly define error messaging and handling.

Remember that you are developing a framework, tool, utility or service which may be used very widely. In case of an issue, people would invariably reach out to you, derailing you from your work unless exception thrown in the project is associated with standardized and actionable error message.

Error messages should answer the following questions:
* **What** was the problem? 
* **Why** did the problem happen? 
* **How** can the problem be solved?

Apache Spark provides an excellent guide on error message and handling - https://spark.apache.org/error-message-guidelines.html

## Define Source Code Management strategy


## Ensure Continuous Integration and Continuous Deployment
CONTRIBUTING file should clearly link the build/run section to project build module.

## Define process for code changes
Define the process in CONTRIBUTING.md

## Define Code review process
CONTRIBUTING file should clearly link the build/run section to project build module.

## Building of appropriate mindset to welcome contribution
One of the key factors to achieve is depersonalization. This enhances collaboration and minimizes conflicts. 
Having everything written down, however, helps depersonalize situations when you do need to enforce your rules. Saying no applies to many situations you’ll come across as a maintainer: feature requests that don’t fit the scope, someone derailing a discussion, doing unnecessary work for others.

Invite for contribution to
* Help others
* Test releases
* Review changes
* Help with documentation
* Report bugs
* Maintain JIRA
* Code Changes

## Recommendation for implementation
I strongly recommend Apache Spark's methodology to invite contribution.

## Credit
This section of the document is inspired by
* https://spark.apache.org/contributing.html
* https://opensource.guide/best-practices/#what-does-it-mean-to-be-a-maintainer