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

The roadmap is sealed using simple majority based consensus voting, with maintainers influence in case of split-head scenario. 

## Establish way of working
It would be extremely important to establish the rhythm for collaboration. The purpose is to
* Gain consensus on new idea. The documentation around the idea stating value, change required and NFRs should be present in the confluence earmarked for project.
* Discuss or Clarify for specific problem in the agenda. Participants are encouraged to get their specific problem enlisted on the meeting agenda.

It would be weekly at most.

A monthly knowledge sharing session is another important cadence amongst the participants and users. Each use case brings more value to the project. 

## Create the Kanban board to implement the changes
A publicly accessible Kanban board is a must. It can be on JIRA or GitLab. This the single version of truth for feature or bug list. Define clearly what each of the fields in the issue means, and how they should be populated. If

## Define practices for versioning, release cycle and support model
It is important to define version - In general, 

For **framework, utility and tools** projects, we can follow major.minor.patch strategy

Patch is typically bug fixes and offers no change in feature. Users can confident upgrade without a need to full-fledged testing - a minor shakedown testing is recommended.

Minor release will include incremental features and upgrade to existing feature. This should have full backward compatibility with its two previous versions. User should be able to upgrade with a regression test and full functional test if they would like to use functionality.

Major release may include breaking changes. This typically will require a well planned user upgrade. A careful build-out following SOLID principle can reduce the effort significantly.

For **services** projects, the strategy can be based on microservices versioning - Example - https://www.deployhub.com/microservice-version-management/

## Establish the minimum toolset required to work on the project
It becomes extremely important to define what the minimal toolsets required are to successfully

## Define standards of code build, test and documentation

## Define strategy for error message and handling

## Define Code review process

## Ensure Continuous Integration and Continuous Deployment

## Define process for code changes
Define the process in CONTRIBUTING.md

## 

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
I strongly recommend Apache Spark's methodology for contribution.

## Credit
This section of the document is inspired by
* https://spark.apache.org/contributing.html
* https://opensource.guide/best-practices/#what-does-it-mean-to-be-a-maintainer