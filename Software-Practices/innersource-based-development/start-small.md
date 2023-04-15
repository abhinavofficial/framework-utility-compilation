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
A publicly accessible Kanban board is a must. It can be on JIRA or GitLab. This the single version of truth for feature or bug list.

This board is not meant to collect ideas, raise questions, etc.

Idea collection should happen within a confluence site under the project. Questions regarding usage can be managed on communication platform as defined in Way of Working.

JIRAs are used to describe what should be fixed or changed, and high-level approaches, and pull requests describe how to implement that change in the project’s source code. For example, major design decisions are discussed in JIRA.

Define clearly what each of the fields in the issue means, and how they should be populated. This allows the project to define new JIRA types, affected version(s), priority, fix version(s), resolution, state, components, labels, language, etc. - they just need to define each of the attributes and its valid values.

Ultimately, define how you expect the JIRA to be managed. Please see below how Apache Spark does it for itself -

* Find the existing Spark JIRA that the change pertains to.
  * Do not create a new JIRA if creating a change to address an existing issue in JIRA; add to the existing discussion and work instead
  * Look for existing pull requests that are linked from the JIRA, to understand if someone is already working on the JIRA
* If the change is new, then it usually needs a new JIRA. However, trivial changes, where the "what" should change is virtually the same as the "how" it should change, do not require a JIRA. Example: Fix typos in Foo scaladoc
* If required, create a new JIRA:
  * Provide a descriptive Title. “Update web UI” or “Problem in scheduler” is not sufficient. “Kafka Streaming support fails to handle empty queue in YARN cluster mode” is good.
  * Write a detailed Description. For bug reports, this should ideally include a short reproduction of the problem. For new features, it may include a design document.
  * Set required fields:
    * Issue Type. Generally, Bug, Improvement and New Feature are the only types used in Spark. 
    * Priority. Set to Major or below; higher priorities are generally reserved for committers to set. The main exception is correctness or data-loss issues, which can be flagged as Blockers. JIRA tends to unfortunately conflate “size” and “importance” in its Priority field values. Their meaning is roughly:
      * Blocker: pointless to release without this change as the release would be unusable to a large minority of users. Correctness and data loss issues should be considered Blockers for their target versions.
      * Critical: a large minority of users are missing important functionality without this, and/or a workaround is difficult 
      * Major: a small minority of users are missing important functionality without this, and there is a workaround
      * Minor: a niche use case is missing some support, but it does not affect usage or is easily worked around
      * Trivial: a nice-to-have change but unlikely to be any problem in practice otherwise 
    * Component
    * Affects Version. For Bugs, assign at least one version that is known to exhibit the problem or need the change
    * Label. Not widely used, except for the following:
      * correctness: a correctness issue
      * data-loss: a data loss issue
      * release-notes: the change’s effects need mention in release notes. The JIRA or pull request should include detail suitable for inclusion in release notes – see “Docs Text” below. 
      * starter: small, simple change suitable for new contributors
    * Docs Text: For issues that require an entry in the release notes, this should contain the information that the release manager should include in Release Notes. This should include a short summary of what behavior is impacted, and detail on what behavior changed. It can be provisionally filled out when the JIRA is opened, but will likely need to be updated with final details when the issue is resolved.
  * Do not set the following fields:
    * Fix Version. This is assigned by committers only when resolved.
    * Target Version. This is assigned by committers to indicate a PR has been accepted for possible fix by the target version.
  * Do not include a patch file; pull requests are used to propose the actual change.
* If the change is a large change, consider inviting discussion on the issue at dev@spark.apache.org first before proceeding to implement the change.


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
CONTRIBUTING file should clear state the code management strategy. For example,

You can have two main branches - 
* master (which is always the latest code - nightly builds would be created on this branch), and
* individual "branch-major.minor". Patch release would always be merged in release branches.

and feature and bug branch. Under this developers can create feature/<JIRA_ID-Short_Desc> or bug/<JIRA_ID-Short_Desc>. 

Feature branch would be enhancements and hence merged via PR in main branch. If developer do not want to merge, they mark "[WIP]" to facilitate review.

Bug would be branched from affected version and same process to be followed. In case multiple versions (current - 2 would only be supported) are impacted by the same bug, multiple PRs to be created.

Merge strategy should be matured to avoid bad code being merged - this is a tricky portion. And therefore review process is important.

### Review process
* Other reviewers, including committers, may comment on the changes and suggest modifications. Changes can be added by simply pushing more commits to the same branch.
* Lively, polite, rapid technical debate is encouraged from everyone in the community. The outcome may be a rejection of the entire change.
* Keep in mind that changes to more critical parts of the project will be subjected to more review, and may require more testing and proof of its correctness than other changes. 
* Reviewers can indicate that a change looks suitable for merging with a comment such as: “I think this patch looks good”. The project can use the LGTM convention for indicating the strongest level of technical sign-off on a patch: simply comment with the word “LGTM”. It specifically means: “I’ve looked at this thoroughly and take as much ownership as if I wrote the patch myself”. If you comment LGTM you will be expected to help with bugs or follow-up issues on the patch. Consistent, judicious use of LGTMs is a great way to gain credibility as a reviewer with the broader community.
* Sometimes, other changes will be merged which conflict with your pull request’s changes. The PR can’t be merged until the conflict is resolved. This can be resolved by, for example, adding a remote to keep up with upstream changes by git remote add upstream <path to your git repo>, running git fetch upstream followed by git rebase upstream/master and resolving the conflicts by hand, then pushing the result to your branch.
* Try to be responsive to the discussion rather than let days pass between replies

### Closing the pull request / JIRA
* If a change is accepted, it will be merged and the pull request will automatically be closed, along with the associated JIRA if any
  * Note that in the rare case you are asked to open a pull request against a branch besides master, that you will actually have to close the pull request manually
  * The JIRA will be Assigned to the primary contributor to the change as a way of giving credit. If the JIRA isn’t closed and/or Assigned promptly, comment on the JIRA.
* If your pull request is ultimately rejected, please close it promptly
  * … because committers can’t close PRs directly
  * Pull requests will be automatically closed by an automated process at Apache after about a week if a committer has made a comment like “mind closing this PR?” This means that the committer is specifically requesting that it be closed.
* If a pull request has gotten little or no attention, consider improving the description or the change itself and ping likely reviewers again after a few days. Consider proposing a change that’s easier to include, like a smaller and/or less invasive change.
* If it has been reviewed but not taken up after weeks, after soliciting review from the most relevant reviewers, or, has met with neutral reactions, the outcome may be considered a “soft no”. It is helpful to withdraw and close the PR in this case.
* If a pull request is closed because it is deemed not the right approach to resolve a JIRA, then leave the JIRA open. However, if the review makes it clear that the issue identified in the JIRA is not going to be resolved by any pull request (not a problem, won’t fix) then also resolve the JIRA.

## Ensure Continuous Integration and Continuous Deployment
Even the basic InnerSource project requires a good CI-CD pipeline. 

## Define process for code changes
Community members can contribute in many ways, as explained in the note above. 

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