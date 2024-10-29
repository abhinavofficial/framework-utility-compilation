Lot of boxes of tools, organization, data. This leads to waste, failures, and poor results. 

Bad Raw Data, Complex and Fragile Toolchains, too much to do, customer visible error.

DataOps is really about - Decrease production errors, increase development cycle time, Result: Less waste, 10x team productivity improvement, improved trust.

## Decrease Production Errors

There are many ways things may go wrong.

![Errors_in_Production](image/possibilities_of_errors.png)

Data Observability and Data Quality Tools are not to check for anomolies. There are five use cases of Data Observabiltity - end to end lifecycle of Data.

|x|Data Observability Use Case|When Does it Happen|Examples|
|Data Evoluation| New data set evaluation and cleansing| Before new Data sources are added to production| The step before 
2. Data Ingestion
3. Production
4. Development
5. Data Migration

*** Data Profile

Transcript

This editable transcript was computer generated and might contain errors.



Christopher Bergh: what are we doing here? our goal is and let I'm just gonna go through some slides here. I'll turn off my camera so you can see the big screen. and our goal here really is to talk about what it means to do data observability and to do data quality validation testing and we're doing it in the form of a certification and we're doing it kind of in two parts we're doing in four sessions and they're really sort of two ways to think about it one is that we're going to use our

Christopher Bergh: our open source tools as a way to teach the ideas and really this is an ideas based presentation. We're going to talk about different ways and different tools to accomplish the same goal, but that's the goal of sort of for you at the end to understand how to do data observability and how to validate data quality. And so we've broken it up into four sessions over every two weeks and just as a background where we're gonna record every session an organization share the slides and we're going to post them. And then in order to get the certification you actually have to answer 10 questions at the end every time so you're welcome to audit and not answer the questions, the certification is something that will mail to you at the end. So I don't know if you can use it for college credit but it's pretty in doubt.

Christopher Bergh: So we're going to go through four sessions and they're kind of broken out. What I'm gonna talk today is about the theme of each one of these at least our first three sessions. It's really kind of use cases for data quality testing and data observability. And then the last session is really thinking of it more holistically about roles and management about maturity model about benefits. So really the first three sessions are kind of use cases.

Christopher Bergh: so what I'm going to start off is sort of talk about today. I'm going to go through those use cases in general. So there's going to be about 20 minutes of me talking and then Eric's kind of gonna go into this first use case sort of data testing kind of pre-production phase it on as Eric says, so that's our topic today. And so the first section is kind of like, why bother right? Why do you want to care about this? And so

Christopher Bergh: we've been in business about ten years a profitable company even really been focused on I think one problem which is waste in data analytics teams. I think there's a lot of wasted time and energy and trust and how we do our work collectively as an data science teams. Mainly it comes in to project failures errors and deliverables lack of data trust and in general sort of unhappiness of data analytics teams and to all those things. I think there's kind of a core reason that we're going to talk about in these next few slides. The first is that they're just little boxes everywhere.

Christopher Bergh: You've got little boxes of tools acting upon data tools. You look at a data architecture diagram from from 20 years ago from today. There's just little boxes everywhere and then there's sort of Team organization boxes, right? We have bi teams. We have data science teams. There's Hobbit spoke their centralized they're meshed and then of course as everyone knows they're just lots of data happening everywhere. And so the result of this is that we have a lot of waste and failure and poor results. And the way we look at this is It's really comes from a bunch of different sources one is that we are getting sort of bad raw data or erroneous raw data that we have a lot of tools that we all of us as collectively have too much to do and that our customers are seeing The Strain we've got a custom visible errors or unhappy customers.

00:05:00

Christopher Bergh: and I think all of this is from a big theme of 10,000 foot them. It really comes from focusing on what I call Day One tasks are immediate tasks building with individual tools. It's about getting our job done and not to say that there's anything wrong with that when we're so stressed, but the theme here that we're going to talk about is really focusing on day two in day three tests. Once you've built something. how do you run it so that you don't have any problems? And then day three, how do you change it? So that you can respond to customer requests. So a lot of our Focus today is really on what you call day two and day three tasks.

Christopher Bergh: and for us we've written a couple books, we've been talking about this idea of applying agile and statistical process control and lean techniques to data analytics for a long time and they really come down to three points one is that if you can decrease your errors in production And increase the rate at which you can change production the amount of waste goes way down and your productivity goes way up because you're delivering things in smaller Cycles. Your customers are trusting errors and you're not having to rework things. And so where we're gonna start really is to focus on day two which is decreasing production errors, and that's kind of In general what we mean? And so as part of it just the last month. We released kind of two full complete open source data observability products, and we're going to use those as part of this.

Christopher Bergh: part of this certification

Christopher Bergh: And so sort of why really focus on day two. we build these systems, right? they have extraction and processors and data bases and dashboards and predictive models and in general, I know Eric knows this and because he's been doing it and I've been doing it what can go will go wrong. Right things will break you will get bad data or your data is perfect. Something will happen in the processing of the data. Something will happen. Once you integrate it with other data sources your dashboard or model may be wrong and all these cases where things can go wrong. And so I guess the biggest theme that we're talking about is don't live with this be able to find these problems before your customer sees them. You don't have to live with reacting fast and being passive. You can actually be proactive and build systems on top of this so that you find problems before your customers.

Christopher Bergh: And so enough you're a Lord of the Rings fan. but what we're going to talk here is like so you don't want to live with it. what does that mean? So I think with data observability and data quality One does not simply check anomalies. And so what we're really going to talk about are these five use cases?

Christopher Bergh: And so they have to do with kind of the life cycle of how you deal with data. And so there's another slide where I talk through this. So the first thing is you get some new data and maybe you have a whole data analytics system production, or maybe it's your first time building it but you get a file or you get some tables. What is in there? How do you understand it? Is it good enough to do analysis and we're gonna talk about that today. And then the second part is okay. I've taken that file. I've evaluated it. I put it into production now, I to I'm ingesting new changes to that data. And then how do I tell if it's right? How do I make sure that there's no problems in it, an example of that is maybe I'm loading. Data from my CRM system every 10 minutes. Maybe I'm doing some weekly data loads. How do you actually tell if that's still good stuff to do analytics on?

00:10:00

Christopher Bergh: And then second is like I'm producing reports or dashboards or a warehouse or some predictive models or feeding some other system. I've got a multi-hop data analytics production process. And so how do you run that production process like a good manufacturing line? how do you produce Toyota's and not AMC Pacers from the 70s?

Christopher Bergh: And then we're also going to talk about the development process. How do you actually do when you hire that smart 23 year old and that you want them to fix that one line of code change, how can you build a system around that 23 year old so if they make a small change that person can immediately see the impact of that change in a bigger system in development during deployment before their customer sees it, they may be adding some python or sequel or Gamel and then the last use case we're going to talk about is we go through very fashionable builds, from how we do things, five or seven years ago. Everyone was doing hadoopid data Lakes. Now everyone's doing sort of snow plank or lake houses and we're constantly replying. So how do you actually do data migration if you've got a system in production and new version of it, how do you actually do observability in those so we're gonna talk through those five cases get some new data.

Christopher Bergh: It's already in production or it's changing all the time. I've got it in production. I'm going to change my production and I'm gonna actually move my whole development stack to a new system. And so we're going to talk through those cases. and so what I'm going to do now in the next few slides is sort of talk through each one of these cases and I'm going to talk about this one today and there's some links to a blog post where we kind of go through these things at these slides. So the first cases I get a new data file are set of files and I'm gonna evaluate data. I want to find some hygiene issues and communicate that with your data providers. So I'm big fan of this tool called xcala Draw. So I've made some excaliburon diagrams. So let's say there's a functional view I get some files from an FTP. S3 bucket. I put it in a red shift, pick your favorite tools. Maybe this is an example.

Christopher Bergh: the data view it goes from an S3 file into some raw tables and a scheduler view this pretty much happens, manually, right because you're loading the files. So you get some things in your bucket you've got some tables and redshift and you don't know what the heck's going on. And so there's a bunch of questions right that you have about the data and I won't go through each one of these questions, but does this data even make sense? do I understand it? Is there something weird in it has my data providers sort of screwed me over and so I'm not gonna go through each of these questions and this is sort of a lot of what we're going to talk about today is how do you understand data? How do you make sure that it's right and even sort of analytics Worthy? and so that's the first case now the second K so what could happen with this you could have

Christopher Bergh: Badly quoted values you could have a leading spaces. You could have a zip code that doesn't have a zero on now you could have many types of null values. There's just lots of cases of things that could go wrong and how do you catch those and make sure that they don't happen and remember the number of a theme of this is if You save time later on. So if you find problems in data files before you're put in production, you're going to save a huge amount of work later think of it as sort of 5x 10x amount of work if you find things sooner and so that's find the problem sooner focus on the day two problem right away and you're gonna save yourself time and waste later on and don't rush these things evaluating data is and making sure that there you understand it and that you find any problems and perhaps pass or pushback on your data providers is a good first step.

00:15:00

Christopher Bergh: the second case I'm going to talk about is from the head here. So we're going to talk about some tools to do this our sort of data Ops test gen it's free. It's Apache 20, we're gonna to get certified. We're gonna ask you to download it and answer some exercises. It's not the only way you can do this. but we like it.

Christopher Bergh: So the second case is data ingestion. So imagine I've taken that file or set of files. I haven't in production and now I'm getting constant updates and maybe the updates happens once a week or once a month or once an hour. and so this is kind of going on all the time and maybe you've got maybe your data scientist looks at these raw files or raw tables, or maybe you've got an analytic team. You're an ingest team and you've got an analytic team looking at what could happen? So let's look at a functional view. So I'm loading data. Maybe I've got an airflow job that's running or a change data capture job or something similarly as Three Buck and redshift and it has similar sort of data flow. But, maybe airflows running every hour to pull data from some source.

Christopher Bergh: And so what could happen? I guess the theme Here is there something new that's strange. did I get too much data that I get the exact same data. I got last week, did the data I get did I get a new table did the schema change just a lot of things could happen when you're constantly loading data. And so you want again find that I was much better to find out problems early then catch them you're doing production on a deadline or worse yet. When you have an embarrassing error in front of your customers, and so what kind of Errors you can have? maybe the bucket more refreshed. Maybe you're normally getting five files but you've gotten for maybe somehow the table schema changed maybe something happened with a drift in the data or maybe your job just failed. There's lots of cases where things could go wrong when you're constantly updating data, and we're going to talk about how to do.

Christopher Bergh: That with Taft gen and observability how to set anomaly alerts and how to set up volume schema and sort of data drift anomaly detection test to be able to check that.

Christopher Bergh: Is a great way help you make sure that you're raw data is good before you do anything now. we all have production systems and some of those end up in production reports, but we have people who use this and in general it's sort of that multi-hop data analytics production process. And so what this looks like from an example functional View and this is just an example. So maybe you have an airflow. Maybe you have a DVD job that does elt. Maybe you've got a notebook and data bricks and maybe you've got a power bi dashboard all going it gets redshift or thank you pick your favorite tools here in their category. and if you watch it, you could in the data view, it's sort of follow the bouncing files. I get an FTP file I get something in S3. I get raw tables like a process tables. I've got a femoral tables that show up and data bricks and then I've got some extracts going in power bi so sort of follow the bouncing data and then you actually have one schedule or you may have multiple schedule.

Christopher Bergh: Flow may run and then data bricks has set to run after it and then the power bi extract is set to run and it's hard here. Because your customers They see Power bi so when problems happen you want to know where it is and here's a bunch of questions. I've got two slides and we're going to talk a lot about this in our next session, but in general if we look at it from a picture standpoint.

00:20:00

Christopher Bergh: Did the data load did one of my DBT test fails was I able to actually do the extract is the model have a prediction error. Is there some data that's missing or some column missing did the schedule never run is it actually late a whole bunch of things could happen during your production process? And again, these are huge time sucks. If you have these problems and finding the problem is an issue. And so again you want to be able to have another 23 year old be able to quickly diagnose where the problem is and be able to find it not have to do War rooms to get 50 people in a room and everyone's looking at log files for a day to find this problem. it's just unacceptable to your customers and they should be unacceptable to you as a team. So, how do you stop that? How do you observe a complicated system like this?

Christopher Bergh: And so we're going to show how to do this with something called the data journey and data observability. And then how you can actually generate the data quality validation tests and data Ops test gen and there's other ways to do this and we'll talk about those.

Christopher Bergh: And then the fourth use case here is development. And so this is the case where I'm changing something. I'm adding some sequel or changing some DBT model or changing some python code. And in this case, I've got a copy of mine production system, hopefully in development instead of using production data. Maybe I've got test data on it. But any good development system should be some close reflection of production. So it should be a lot like what you're running maybe you've got only one schedule or things are done, but it's…


Christopher Bergh: if you can get your development environment as close to your production environment,…


Christopher Bergh: that's another source of error in data analytics systems,…


Christopher Bergh: but you want to be able to do something called regression testing or…


Christopher Bergh: function testing if you change something

Christopher Bergh: How do you tell if it's right and how do you actually have that 23 year old find it? And so again, here's a categories of Errors. maybe I've added some new python code and…


Christopher Bergh: SQL code in the airflow and…


Christopher Bergh: DBT. Is the report empty did the sequel actually cause an error?

Christopher Bergh: Are there empty columns? Is there a low extract row count did something happen in my daily CI and…


Christopher Bergh: CD process being able to see the system and…


Christopher Bergh: make these changes and The impact is a huge. Enabler of you being able to make smaller changes and not have to spend months making work putting it into production only to find out your customers needs have changed and…


Christopher Bergh: so the ability to make changes quickly with low risk and…


Christopher Bergh: be able to judge the impact of their problems is a huge way to increase your own team's productivity.

Christopher Bergh: And so we're going to talk about CI and…

Christopher Bergh: CD testing and regression testing. And then lastly data migration and…

Christopher Bergh: this is where as I spoke of before you've got two systems you're moving from perhaps traditional Informatica SAS cognos teradata with staging files again,…


Christopher Bergh: follow the bouncing ball with the tools to a cloud architecture. Maybe us or Azure pick your tools. And then how do you balance between those two to make sure things are right.

Christopher Bergh: How do you make sure that the new system is the old system and…

Christopher Bergh: these are very hard projects.

Christopher Bergh: So there's a lot of wisdom that we've had having done these projects and…

Christopher Bergh: and to talk about how it happens.

Christopher Bergh: And so we're gonna talk about how we can do it with our tools and…

Christopher Bergh: other tools to make it work.

Christopher Bergh: And so lastly sort…

Christopher Bergh: why should you care?

Christopher Bergh: Why should you care about all these use cases?

Christopher Bergh: To…

Christopher Bergh: I think it comes down to the saving time and…

Christopher Bergh: lowering errors and improving the results so that your customers trust you and…

Christopher Bergh: that you're able to make changes quickly.

Christopher Bergh: So it comes down to the sort of values and data Ops that we talked about error rates cycle time leads to productivity and…

Christopher Bergh: increase lead to happy customers need to more people using your data analytics to get value and…

Christopher Bergh: change the business, which is actually the real goal. And so we're going to talk lastly about sort of what to Do I check my tools? Where do I check them? And so we're going to talk about that and since we've got one that does sort of data validation checking the other that's kind of data tool monitoring and brings a big picture together and we're going to talk about how to do that.


### Data Profiling

Some of the key things that you gather with data profiling and then what do you actually use it for and…

Eric “DK” Estabrooks: really a lot of this context is going to be thinking of profiling as a tool to figure out whether you're going to pass patch or pushback. what are the three p's past patch pushback? So the passing is when you've got data,…

Eric “DK” Estabrooks: it's good enough to use and…

Eric “DK” Estabrooks: you can just send that along to your developers and users.

Eric “DK” Estabrooks: you've probably The more experience you get the more time you get into this situation.


Eric “DK” Estabrooks: You find that there's problems with the data that you really can't send it along to your end users,…

Eric “DK” Estabrooks: but it's kind of obvious what the problem is. So with some consultation with your Downstream is probably Downstream customers as well as probably the Upstream data providers you figured out that you can actually patch the data to make it usable and…


Eric “DK” Estabrooks: we'll look at some examples of…


Eric “DK” Estabrooks: where that might be a good thing to do.

Eric “DK” Estabrooks: And then there's pushbacks.


Eric “DK” Estabrooks: Sometimes you get data that you can't fix it either because something's missing or…


Eric “DK” Estabrooks: it's just messed up enough that the effort to fix.


Eric “DK” Estabrooks: It is just going to introduce more problems.


Eric “DK” Estabrooks: Then you really want to have to deal with. so let me me turn into slideshow mode here.

Eric “DK” Estabrooks: Here we go. Pretty much better. so

Eric “DK” Estabrooks: One situation that people run into a lot is you get new data.


Eric “DK” Estabrooks: Right and…

Eric “DK” Estabrooks: customer drops it to you say hey here.


Eric “DK” Estabrooks: I got some new data and…


Eric “DK” Estabrooks: you're like,…


Eric “DK” Estabrooks: what is this? It's like I got it from somebody. They don't have a good answer. They don't have a clear lineage of…


Eric “DK” Estabrooks: where this is from so yes nicely for data dictionary again.


Eric “DK” Estabrooks: They're like, we're not really sure we just like you to figure out what's in it.


Eric “DK” Estabrooks: So you end up with this data that sometimes it's just a black box and your job is to figure out what's in there is a good enough to use and…


Eric “DK” Estabrooks: can you get this Downstream to your customers?


Eric “DK” Estabrooks: So I know the ideally, you…

Eric “DK” Estabrooks: the situation and most companies is it's not like that…

Eric “DK” Estabrooks: but it does happen.


Eric “DK” Estabrooks: So More often I would like to think about.


Eric “DK” Estabrooks: All what is data profiling so one way to think about it is it's the first step in a data quality journey.


Eric “DK” Estabrooks: And as we go through these different sessions, you'll see how the early work you do with profiling just sets you up for success as you start to move data into production and…

Eric “DK” Estabrooks: then actually run it in production.

Eric “DK” Estabrooks: And data profile is all about examining data that you have collecting characteristics the statistics on it.

Eric “DK” Estabrooks: And the goal for that is to understand its structure the content what's in there look for inconsistencies as well as anomalies. And we'll cover some of these very directly later on as we look into kind of doing some practical profiling. I find the profiling is a very valuable Baseline. Before I put data into production and what I can do with that Baseline is it helps me set expectations for the future. And what this data should look like and maybe how it's going to behave which is a good foundation for me to build tests in the future.

Eric “DK” Estabrooks: With profiling, I'm talking about that customer delivers you some data and you don't know what's in it just because you use it to get up and running with the data set. It's not something you should only do once if you can incorporate data profiling periodically into your kind of overall quality and production strategy. It's really gonna help you head off problems as they develop in the future. Maybe you've learned some more about the data that you didn't know and really kind of make what you're delivering just the highest quality.

Eric “DK” Estabrooks: I mentioned just understand what's there right A lot of it is how is this organized? Do I have columns with similar names and are there possible relationships between them, during the profiling you're going to be finding duplicates in some cases in consistency. And again, what you can decide is I'm gonna pass this data along you're gonna push it down to my customer or am I going to try to patch it before I use it and this is going to help you build up a trusted data source.

Eric “DK” Estabrooks: A lot of what you do in the profiling phase is going to help you with the next step which could start to be the data cleaning and prep. Ideally you get nice clean data profiling just helps you set that Baseline. There's no more work to do but a lot of times you're gonna find missing values might be some outliers to either need to be corrected or addressed or have some questions formatting and just inconsistencies with the content of the columns is always something that you want to discover early because once somebody is pivoting over and slash a versus not applicable Downstream and their bi tool. You don't want them to be doing it down there. You want to fix it up Upstream. profiling helps with data governance.

00:30:00

Christopher Bergh: yeah, and then my two cents on this is like,…

Eric “DK” Estabrooks: You may work in a larger organization that's been up and…

Christopher Bergh: I just find it incredibly embarrassing when customers find problems,…

Eric “DK” Estabrooks: running for a little while and have some standards where you need to comply and…

Christopher Bergh: and I've had them Misfortune of being yelled at or…

Eric “DK” Estabrooks: that could be anything from the way territory codes are managed.

Christopher Bergh: Shamed by a customers and saying what's wrong with you and so a lot of…

Eric “DK” Estabrooks: If you're working at a Pharma company or…

Christopher Bergh: why we started this company is sort of Shame avoidance.

Eric “DK” Estabrooks: just the way you handle nulls. It also can help set standards.

Christopher Bergh: And I think it is you can do something right?

Eric “DK” Estabrooks: So if you're profiling new sets of…

Christopher Bergh: It's sort of not acceptable to Poor data into your data analytics system have no idea…

Eric “DK” Estabrooks: you may not know what's in there. So you can kind of …

Christopher Bergh: what it is and…

Eric “DK” Estabrooks: as I mentioned rather the Baseline for the data that you can help that the standards for…

Christopher Bergh: then have no idea if it's going to produce the right results and then rely on your customers to tell you that's something wrong.

Eric “DK” Estabrooks: what comes next with that.

Christopher Bergh: I think we can all do better and…

Eric “DK” Estabrooks: And…

Christopher Bergh: we can do better by applying some of these observability and…

Eric “DK” Estabrooks: the number one thing. Almost all data quality work is really focused on obviously delivering high quality data your customers,…

Christopher Bergh: and data quality validation. It's actively and so that you…

Eric “DK” Estabrooks: but it's important you establish trust with people your Downstream users need to trust you.

Christopher Bergh: the big theme is You don't have to be passive. It's not that much work and partly one of the reasons why we've open source the software is to make sure there's even no cost and…

Eric “DK” Estabrooks: I usually coach my team to never promise that we're going to be perfect or…

Christopher Bergh: involved for you.

Eric “DK” Estabrooks: that we're going to catch everything but you want them to know that, the downstream users you want them to know that you have a process you care about the quality and that you're gonna keep an eye out. You've been address things when you find them and the ideal case before it reaches them, but if not, you have strong processes to come back and catch it and that's why data profiling up front helps set that stage and then regular data profiling helps you stay on top of issues before they emerge

Eric “DK” Estabrooks: 

Eric “DK” Estabrooks: Yeah, we want to. keep the lectures coming. Yeah, this is what everyone of the call if you're lucky Chris and I won't go too far on tangents and experience. But if you like that sort of thing, please do encourage us, so

Eric “DK” Estabrooks: Because we like to talk.

Eric “DK” Estabrooks: Okay data profile. I talked a lot about what it is. It's actually equally as important to recognize what data profiling is it's not data cleansing depending on the tool you're using for data cleansing or how your organized about who does what jobs? Yeah, and it may overlap but data profiling is not about fixing the issues or just here to identify the issues. you're not doing analytics right? You're gonna look at the data get some descriptive statistics the standard deviations the means the Max and all that, but that's not analytics you just Gathering characteristics that you're going to be able to use later. You're not validating the data against predefined rules.

Eric “DK” Estabrooks: This is where validated data and Screen rules that really gets into testing what you're looking for now are patterns and anomalies to help set the Baseline, maybe help Define what the rules are but you're not going to be about enforcing them at this as I mentioned early on it data profiling as I mentioned is part of a overall data quality journey to putting together robust production processes that deliver high quality data to your customers. It's not in itself a one one and done and it's not sufficient to ensure that quality data gets to your customers. It's just a piece of the puzzle. It's certainly not the whole thing.

Eric “DK” Estabrooks: And I've mentioned this a couple times and probably hit on it again a few times more. It's not a one-time activity. Again. It's really important when you first get a new set of data if it's the black box without any documentation or any Clarity on what's there? you definitely want to be profiling it even if you get a data dictionary get a lot of documentation on it. It's really incumbent upon you to profile the data to verify that what the provider is saying is actually in there. And when does this happen we do work with lots of different companies. But Pharma companies, especially on the commercial side, they buy all this syndicated data. So these are these curated data sets from some of the

00:35:00

Eric “DK” Estabrooks: biggest data aggregators in the world. They do really notch work. They put together just amazing amounts of Rich data to look at pharmaceutical sales in the US. They have great internal quality programs. However, they make mistakes And so anytime we start a new engagement. I insist were profile the data regardless of what the data providers telling us is in there, and I can't count. I can't.

Eric “DK” Estabrooks: On one hand. I don't know. It's happened more than once where early profiling with a new data set has uncovered a really serious issues in the data before it even got into production. So just don't trust the data from Upstream. Now if it's a great data the data providers really tough Notch, they have good controls, maybe don't put as much effort into it, but you got to do your own homework on this.

Eric “DK” Estabrooks: So when we're profiling data, there's two parts to it. The first is gathering. Characteristics about the data and then using those characteristics to look for anomalies. So focusing first on the characteristics, these are all the information you're going to gather about the structure of the data what's in there what the content is, the quality. Of it, doesn't have expected s the patterns follow some kind of logic to them. As well as just some Basics statistical measurements to it. And they always start simple and we'll look at some simple ones here, but they can get more complex and part of it is gathering statistics.

Eric “DK” Estabrooks: But then using them in an automated way when you can to look for anomalies and then sometimes just giving you basically a feel and insight into the data that you be normally wouldn't get just by reading a data dictionary or trying to walk through it in a results window or spreadsheet.

Eric “DK” Estabrooks: like I mentioned these low-level characteristics. you typically get a lot of them and they build this comprehensive picture for you. So there's a lot of different characteristics. You can Gathering them organizing them keeping track of them is a pretty big job in itself. If you're doing it manually and we're gonna look at what this looks like maybe from a sequel perspective.

Eric “DK” Estabrooks: And then what it looks like from a tooling perspective and we'll use SQL for the first part and then the data kitchen test gen tool for the second part. What I got here is kind of making it concrete about what are the types of characteristics. So there's 50 odd characteristics here. We're adding new ones all the time when we find something valuable or more often than not when something slips through if there's a problem that bound later that we're like, what if we just profiled This extra characteristic and used it feed into an anomaly detector. Then we probably would have caught that so that's how we grow it by learning, but this is where just

00:40:00

Eric “DK” Estabrooks: you know, how many embedded spaces are there leading spaces? Maybe the text string you got a text field where the value is set to effectively a blob 64k but the longest string is, 10 characters like, this is all good information together. It's gonna help your data Engineers as they start building up processes. It's gonna help you get a sense of the quality and it's just gonna help build up this overall picture for everybody with the data, you'll probably hear me talk a lot about things from a perspective of maybe a more technical data engineer who's slinging sequel or something but a lot of this stuff is valuable for data Stewarts data quality people who maybe are sitting up in a slightly higher level doing analytics not slinging heavy-duty sequel, but they just have the

Eric “DK” Estabrooks: feel for the data. They understand the domain they're working in and a lot of these characteristics help feed into giving them a tool they can use as well.

Eric “DK” Estabrooks: So I mentioned tools how am I practically going to do profiling?

Eric “DK” Estabrooks: I like to think about these kind of five major kind of tool groupings and then there's half a dozen or more different characteristics and they try to rate them and the best thing to do is to pick the tool that's compatible with your skill set and matches the data and the sources that you're gonna be using a lot of the discussion later. You'll notice we have a data base focus on things, but if you're just dealing with files, they can be reasonably loaded and managed in Excel Excel is a great way to do some light weight data profiling, and obviously Excel is not free, but it's rarely an office that doesn't have Excel licenses floating around they've got power query and some other profiling tools that are built in these days.

Eric “DK” Estabrooks: SQL I love SQL. It's ubiquitous. Even between the dialects between data bases the differences in the case of something like profile trivial the differences that are there it scales. as far as the amount of data it can handle just huge huge benefits there, python there's an enormous amount of

Eric “DK” Estabrooks: knowledge has gone into making robust python packages for doing things like data profiling again. You got to know profile. You got to know python got to be proficient, but it's still there and then you get into the application world. There's some tools that focus on things from a command line perspective and then others that have a UI that kind of make it easier for you. And again, the best thing to do is use a tool and then find a tool that just fits again with your skill set the data you're working with to help make things a little more concrete from a profiling perspective. We're going to look at two ways to do it today. The first is going to be SQL. And then a little bit later on we'll look at the test gen application itself.

Eric “DK” Estabrooks: Yep.

Eric “DK” Estabrooks: Yeah.

Eric “DK” Estabrooks: So yeah exactly Chris. Thank you. That's a very good point. The Tulia now is the best one so at least initially.

Eric “DK” Estabrooks: Okay with sequel so we talked about. I grew up that slide. I've got 50 different. data quality metrics there

00:45:00

Eric “DK” Estabrooks: Right one that I find valuable. That's almost like the first thing you should do. It's just how many rows are a moment table. It seems so silly. Sometimes it's like I feel silly even telling people about this but know how many rows are in your tables, Because this then becomes a baseline for how big a file should be you got to know whether it's like a full snapshot and incremental does it grow, you got to know the Dynamics of it, but how many rows are in my table is just so valuable and it's also on its own useful, but then it becomes important when you start mixing and matching it with other characteristics they be in a poll. So selects count star from my table. that there you've started data profiling.

Eric “DK” Estabrooks: a next favorite is distinct counts. It's always good to know in a column. Is this a very very sparse domain? There's a lot of nulls or is it a code where I have a super limited number of codes, right? Again on the SQL front select count distinct my product name. Boom got my second metric.

Eric “DK” Estabrooks: And now I can learn a lot by what that comes back. if it's an ID column, hopefully there's as many distinct as there are rows if it's a product code. Hopefully it's a much smaller domain than what's there and now I get to this point where I can now take record count and start mixing it so I can start to use distinct value account record count and now I've got another metric that I can use. It's something you start to track over time and it starts to build into your profile and picture. not to focus on text too much. I feel like if text too much love sometimes but just numbers right Max men median average just pulling all those statistics together when you've got a number column.

Eric “DK” Estabrooks: Just again gives you a sense of what the data looks like helps you look for outliers. And this is where you may see outliers on your own and then you need to understand what those outliers are and whether or not they're usable. That's where you might have to pull the domain people in or sometimes the numbers are just out of whack you can kind of tell

Eric “DK” Estabrooks: So what's the problem with SQL from doing a very comprehensive? kind of view of your data You got to write a lot of queries, I like writing queries. I love meetings where I can open up a query editor instead of making a debt but even I don't like writing too many queries to do my profiles, I've got a table with 50 columns, I got to write 50 queries just to see what the distinct feel count is. I've got two tables each one of those is gonna need another query the math on this gets a little crazy. I've got, relatively small data set right you tables you call them going together a lot of characteristics talking thousands of queries. I've got to look and manage and run and put that data somewhere. So

Eric “DK” Estabrooks: SQL fantastic tool for profiling it just you can't do the comprehensive queer profile that I think most data sets deserve and that I like to do. Even if I'm using another tool that does the comprehensive profiling I'm still going to load a table. Do a couple kind of quick and dirty ad hoc queries to get again. Those early ones distinct values row counts, but I probably just gonna use those as a bit of informative while my profile is running so

Eric “DK” Estabrooks: you don't want to write thousands of SQL queries. do you use a tool? we mentioned there's cool all over the place as far as data profiling tools themselves. X is a data profiling tool you're gonna find data profiling as a feature. In a lot of other tooling and that includes things like, you ETL tools so ssis has a filing node our bi I think as data profiling in it a little down the street down the workflow but still lineage data catalogs. A lot of them have data profile built into them. So again number one tool to use the one you have access to and then does it fit your use case?

00:50:00

Eric “DK” Estabrooks: Let's talk about Toyota profiling intestine itself. as Chris mentioned data

Eric “DK” Estabrooks: profiling data test gen itself. It's an open source project. So please, take a look at the repo start. We're looking for stars. I feel like I'm on YouTube looking for likes and subscribers. So do the homework at the end. As well as to set you up for the next session you're gonna need to download and get tested running anyways, so hopefully we'll see you there. We have issues open on GitHub. We have a slack channel, so don't hesitate to join both of those communities. If you see something that's broken, let us know you have trouble. Let us know if you have ideas about features or improvements. We want to hear those as well. The more people we get banging on this in feeding back to us what we can do the better off will be so.

Eric “DK” Estabrooks: So without further Ado, I'm going to jump into test gen itself. as you find you'll find out once you download a run test Jen it runs in a local container service the installs really simple. Straightforward it's a web-based interface. So we have customers that deploy this on a server and then multiple users are coming in and using it just for this demo. I'm gonna be using this and the kind of running at locally. So with the data profiling, can you just a second? I'm going to sync up my line over here

Eric “DK” Estabrooks: What do it justice? All right, I'm not going to go through the setup or the demo data. And this is one thing you'll find Once you do the install. There's a little come in the run to run the demo load the demo data and then run some profiling on it and it's just a great way to get a real understanding of the tool before you start putting it on your own jumping to the data profiling tab I can see that there I ran this yesterday. And I've got some profiling results. So let's take a look at what we've got. first thing I noticed I profile for tables and a total of 52 columns. So again doing the math on my prior slides that's a lot of SQL queries.

Eric “DK” Estabrooks: So with testing does give you schema table column and then it starts to tell you some of the stuff it discovered in here. So let's take a look at this column customer type. So what can I gain from this first screen? I can see the table. It's a VAR card 20. We've decided it's a functional data type is a code. And we'll get into that too much here. But if you look in the test gen documentation based on the data that we find we try to make assumptions about How are people probably using this column? We're not always right, but at least it gives you a good way to kind of group organize and think about things. So, let's see what we can learn or what we learned about this.

Eric “DK” Estabrooks: so again alphanumeric context 20 and then test gen looking at the data that I found data recommendation and say hey, what? Based on the data I found in there all as well as the column type that it is already, for car 20 looks like it's a good choice. This is a nice feature. A lot of times you' Sometimes you'll load data. You don't again, you don't know what's in it. A lot of times I'll just put everything into big text columns load everything up run the profiling and then use test gin to suggest to me what the schema might look like or as I'm going through the profile results kind of craft my schema for it.

Eric “DK” Estabrooks: I can see there's 502 values. So this means everything was populated. Which is now interesting, How is that interesting? This could become a test. Where is this table or this column in this table? Is it always populated? So now it's easy to say, what if I get a data set and I start seeing nulls. I think there could be a problem. I've also got distinct value count of four. So this is interesting 500 rows is only four different values over on the right. I can see here what they are. Since we know these are customers I'm assuming this is customer type. Again, it's sometimes a little dangerous reading too much into what people call their data and their columns, but this is probably like the sources where people are coming in.

00:55:00

Eric “DK” Estabrooks: And so you can see the rough distribution. Again. This is helping me build up a picture. Of what this data looks like and possibly setting expectations and baselines that I can use for tests in the future. So let's assume I get this file one day and there's no Amazon in it or it's a hundred percent Amazon. Is that a problem? You probably have to watch the data for a few cuts of the file. But that's probably a test. You would look at say hey, the rough distribution is this if I see somebody disappear or I see a jump to a hundred on somebody then there's probably a problem with the file. And anyone who's

Eric “DK” Estabrooks: Worked in an environment where you're handling data, which I assume you all are since you're on this call. You've seen that before right you have someone where they come and goes into salesforest and set the customer type to Amazon for everything by mistake, but if you have a test That you built off your data profiling, that's not a good situation.

Eric “DK” Estabrooks: We don't have any nulls No zero length counts. how do you handle nulls always becomes an important question in any data set, dummy values. This is where things like missing wn not applicable. Those are all dummy values, but sometimes you'll find that there's a mismatch of how people are applying those.

Eric “DK” Estabrooks: nothing we determine is a number nothing would determines a date count or a date. Right? So this data looks about what it is. this top patterns I find useful maybe not necessarily here, but this isn't telling me that there's a string of Three A's in there. This is telling me that the biggest pattern is a string with three alphanumeric characters or in this case just Alpha Characters probably and if I scroll up to here Yeah, that makes sense. I've got two club and Shop that are four in length. And that's the biggest category so then I'll kind of make sense.

Eric “DK” Estabrooks: Yeah, and a standard pattern matching there wasn't any pattern really that was picked up from here. So it didn't look like a date or a zip code or state or anything like that. So that's an example of profiling. where

Eric “DK” Estabrooks: there's no anomalies here. I've built up a little picture of what's in the data. I've said some baselines of how many value should be there. I've also really given myself a good foundation for thinking about pests. We'll talk about tests later. Right but everything for me ends up like am I going to write? What kind of tests are on my data? Chris mentioned you don't want your customers to find the problems. You want to find them. So depending on what your production cycle or system looks like, you may have tests we like to put tests in line in production such that nothing gets delivered unless the Fleet of tests are run and they pass I know one of my big questions is if something does slip through the customer did we have a test for it,

01:00:00

Eric “DK” Estabrooks: Know if this is a good or a bad attitude, but if a customer finds something I want to know whether or not we had a test if we had a test and someone ignored it. That's a process problem. We can fix that if we didn't have a test and it got through the customer. This is where the building of the trust comes because I'm going to say, I established with the customer very clearly. We're not perfect stuff's gonna slip through what we guarantee. Is something gets through. We're going to put a test for it after the fact so this gives us a leg up on adding tests. So, all right. So let's look at something that actually had a problem.

Eric “DK” Estabrooks: So what I'm doing is I'm sorting just the results By anomalies and I can see that there are some.

Eric “DK” Estabrooks: With anomalies. All give me a second here. We're going to jump back to the deck for A lot of my walking you through. what you call it, the column over there is in the slides as well, but you'll have access to this intestine itself. You'll be able to do it in there.

Eric “DK” Estabrooks: Okay, so I got a bunch of characteristics. I looked at some of my columns and I got a good picture of the data nothing seems off in the profiling. I have a good way to communicate with my Downstream customers, what they can expect from the data and I've given myself a baseline for adding some tests. However, that's in a perfect data set and the non-perfect data set. We look at those characteristics and determine that either some anomalies. So intestine, we're focusing on test gen now the way it thinks about anomalies, we have a few dozen different anomalies that we look for.

Eric “DK” Estabrooks: And that's all based on the characteristics. We're adding new ones all the time because the more we find early the better we are and so, what are some of the things we look for again. We look for discrepancies between the column type that you loaded it as well as what suggested there. Is the zip code funky or the columns without values there's tons of dead columns and data sets gets to the point of do we want to carry this around? we look for little things like standardizing values, do they look the same did I find an email in a text column where every is not a lot of emails in there? That's always a fun one where somebody is just data in the wrong column so

Eric “DK” Estabrooks: I'm gonna jump back into test Jen. And we're gonna go to the anomalies. So we're gonna focus on our I mentioned, data profiling it's a upfront activity, but also something you should do over time in this case. We just done it the once

Eric “DK” Estabrooks: all So what do we got for anomalies? What we do is we've looked at all those columns and we've given a bit of a ranking. Or maybe a priority order is the best way to think about it on How likely is this an actual data issue? And we do that because just seeing there's issues is fine. But a lot of times you need Focus you need priorities you need a task list and we found that this is a good way to do it. in the data intestine itself. we like to think of this as a task list because hey I should look through all these and determine whether these are real problems or not. Once you've actually looked at them done some judgment on it and address that one way or the other you either after the long.

01:05:00

Eric “DK” Estabrooks: Pushed back to the customer or you've patched it. We just give you a easy tool to just kind of like hey, what's the action on this? So this is where I can confirm it's relevant for this run, dismiss the issue as it's not relevant or what this is always showing up. I know it's a mess. It's an unlogical customer tell data sources provider tells me just ignore it. So give you a little way to keep track of things from this run. let's dig in to one of these here. So let's go to looking down a product type. What's the anomaly first of all similar values match when standardized? So this is likely a problem. And so if we look down here, what does that actually mean? you've probably all run into this. where

Eric “DK” Estabrooks: you've got a column where it's supposed to be a fixed domain of results. Sometimes there's two spaces where there should only be one. Sometimes people to limit things with a dash. Sometimes there's an extra period for some random reason or the casings weird, depending on what the usage of your data is that could be a real problem. You want to fix it Upstream as early as possible such that Downstream your data pipelines can start to work on just one expected value. So that's using this characteristics. This is one of the things we look for so this is great. We're explaining what it is. But what am I looking at? Right? let's look at the source data.

Eric “DK” Estabrooks: And then looking at the source data kind of becomes clear what the problem probably is. I can see all the way to the left. This is what's in the table and what the count is of it. So it seems like ebike doesn't have a lot of Upstream control. So whatever app they're using to gather this information doesn't have good controls over that value. So, what a test and suggest to test and suggesting. Hey, this is probably E Bike maybe you really like it differently, maybe you do want. Mixed case or whatever but really what this is is giving you visibility that you've got a product type column which probably should be a limited domain of values as well. As you know, what a possible standard value is.

Eric “DK” Estabrooks: for me I look at this clear case of a patch, right so the way, I would approach this is I use this information to recommend both to the downstream customers as well as the Downstream, I'd say hey we got this data. It's real messy product type. We think we should patch it which means we're gonna in our data pipeline We'll add a step that just takes product type column looks for all the permutations to eat bikes and sets them all to this standard. instead of just showing this to the customer. All right, our Downstream user. Yeah. I'm going to go to them with a plan. I'm gonna say look here's what the data looks like. I think we should just patch this and I can get this into production for you very quickly. I guess the real question for you is do you want

Eric “DK” Estabrooks: All caps ebike. What do you want e Dash bike right now come into the customer with a plan on how to patch this data takes a whole lot of work off of them typically in a situation like this. They're gonna say great. Let's get this into production quickly. The only thing that may change is how they want to present it, which standardized value they want anything you can do. To make it easy for the customer. To make a decision is great. And this is where you don't need to worry about or having an answer? That's perfect. You just need to get something in front of them. because they have to make a slake week for the value that's there instead of them having a hundred percent of the solution you're talking about, five percent just decide you're gonna use

Eric “DK” Estabrooks: this is a conversation probably that would go work well in conjunction with talking to the Upstream data provider. Hey, we're looking at your data. This is what we're seeing. We're recommending to our customer. We just patch it and we're gonna use this value as the standardized value Downstream. Again, you go to the cut you go to the data provider. They may be great. Thank you. And they're done right? They don They don't have to do any work. They also may say that's great. Thanks for pointing these errors out. We're gonna fix them because of our release cycle. It's going to be three weeks and we're gonna standardize on edash bike, right? So by coming into the customer with a patch

01:10:00

Eric “DK” Estabrooks: proposal getting agreement consensus from them and then going to your option data provider saying hey, here's the plan you've now taken the burden off of them. So instead of them rushing probably and possibly introducing more errors into their data process. You've given them, the solution and you're telling them just fix it when you get to it until then we're just gonna clean it up before it gets there. So now you made both sides of that successful and you ensure that this project, kind of help this project a little bit more making it Let's look at the product type a little more. So let's look at the profiling results for this too.

Eric “DK” Estabrooks: So if you remember we looked at the profiling results for the customer type now, we're looking at the product type. So alphanumeric it's recommending 50. We have 50, that's where

Eric “DK” Estabrooks: Pestgen is likely to say hey, if you decided on 50 it fits everything I see here. I'm not gonna paying by the column with anymore. So let's just leave it like it is because a lot of times leaving something as it is is the way to go this becomes important is now I've got and I don't think we do this feature yet. I know it's on the roadmap, but if I see product type is fit car 50 here and I find another table where product type is car 20 or 25. this is something that roadmap feature and testing will flag. It's saying hey This is what I'm seeing as possible column with you. this is an anomaly you should look into this.

Eric “DK” Estabrooks: If we this looks like a customer just sells three different major products bicycles these scooters and bikes. You can see this value chart and distribution will get cleaned up quite a bit.

Eric “DK” Estabrooks: I mentioned the patterns right again, we've identified bike is probably an error but this is where the string patterns come into play where we can see that hey, I've got two some stuff without dashes. Just all feeds into what we can see there.

Eric “DK” Estabrooks: let me see. How we doing on time. All let me so one of the jump back in here, with the anomalies we've got the column for you here, As you're working through then the anomalies themselves are typically summarized for you at the top of each column.

Eric “DK” Estabrooks: It doesn't link through yet. one's on the roadmap, too. I know someone else is working on that so

Eric “DK” Estabrooks: All So this was product type. All right, we talked about this, what's the, kind of coming back to the three p's at the beginning The data This is probably a patch depending on how much trust you've built with your customer what the relationship looks like, but the release Cycles are you don't want to unilaterally patch all the time. You probably want to go at it. Like I mentioned two set of proposals say hey, here's what we recommend we do and hey, here's the data we found sometimes you run into data issues where they're just isn't anything you can do. So, let's pop back in and take a look at last order. So we've got last order here. And what is this? This is on my bike suppliers table.

01:15:00

Eric “DK” Estabrooks: And we've identified it as a date column. Must mean that we did identify the data in there as dates itself. However, what am I looking at? it It's fully populated. Okay, that's good.

Eric “DK” Estabrooks: Those 28 records, you can vaguely derive from that you get 28 suppliers. Although we're not looking at the data here necessarily. We're just looking at the characteristics and anomalies. So that's where your SQL comes in handy to dig deeper. So what are we seeing here? We seeing that we've got, data back a couple years.

Eric “DK” Estabrooks: no table dates within six months. Most recent date 8 1923. That's what my maximum data is. This is probably a problem. Right? We've got a reasonable number of records for suppliers, but in the table itself, the maximum is not there. So let's actually take a look at that anomaly. So we're going to look at last order.

Eric “DK” Estabrooks: Pull this And so what is this test right among?

Eric “DK” Estabrooks: all the columns president table most recent date is Kind of Paul six months to a year back, right if you've got a date column and the rest of your data is fresh and alive. It's probably something we should look at. So let's take a look. Yeah, and it's just calling out that this is the last Max data. this is the characteristic and the information was gathered that the test was built on. So this is another one where mostly what do you do now? You tell your customer. Hey, the players tables messed up. We're gonna go back to the

Eric “DK” Estabrooks: data providers so you can get some clarity on it. I'm a big fan of getting data into people's hands as soon as possible. So where again this is an idealized demo data situation, but I've had tons of situations where we've had data, That we think there might be problems with and what I do always with the customer is unlike, we're in development. You got to start building dashboards. We're gonna give you this data.

Eric “DK” Estabrooks: Is a ton of known issues the quality is low. Don't go show this to anybody but I really want to get this out into your hands. And so just because you have a data issue and it's even a data issue that everybody agrees is a real problem. It probably pays to start moving forward with a whole bunch of your development and trying to get into people's hands, There's so many different ways. People work collaborative collaboratively on projects and companies big companies small companies, right? So some people this is a non-starter. It's like hey, we didn't pass profiling this data is old.

Eric “DK” Estabrooks: They're putting us on hold to the provider fixes it right. I usually try to push for agile environments where hey you got data which is always eating getting the data sometimes is hard right, but I've got the data now, I need to push it out and I want to get this to somebody so they can start developing against it. But will kind of qualify it with it's a problem in the future.

Eric “DK” Estabrooks: I wanted to look at another column and then I think that will kind of move on to the next phases and open it up for a little bit of questions if we can. so region code right pattern in consistency within the column. So I love this one. This is where if we find that there's a clear pattern to the data. Typically a length or some tons of limiters in there. What we try to do is call that out. So if I look at this here, I'm seeing that there's 27 that match a pattern of two Alpha Characters data underscore to Alpha underscore a number. However, I have one that looks like Let's take a look at the source data. And I can see here.

01:20:00

Eric “DK” Estabrooks: this looks funny. However, how does it look funny? Let's look at it in context. Kind of walking through my normal thing, 20 at rows 28 populated right now. I can see what is this is my region right They go North America out on the limb and I guess it means North America us two and then underscore right this. This definitely looks like something's fishy there. So again, I'd probably put this in the patch category. it's a pretty trivial patch from a pipeline data engineering perspective you go to your customer. You tell them look we found this anomaly.

Eric “DK” Estabrooks: In the data, we're gonna propose we put a patch in place such that we fix it. If we ever see this again, you go to your data providers and you tell them that you found and everyone's happy and everyone's successful.

Eric “DK” Estabrooks: People all right. yeah, so a lot of this has been me going through the tool looking at these things Discussing talking about answers, right and as I'm going through these I'm saying. Yeah, that's a problem. We're gonna talk to somebody this isn't a problem. We're going to talk to somebody.

Christopher Bergh: I do. Yeah, so we just share my video on share my screen share now

Christopher Bergh: 

Christopher Bergh: So yeah, the questions you can go through.

Eric “DK” Estabrooks: This isn't an issue this time or…

Christopher Bergh: So our next session is June 4th…

Eric “DK” Estabrooks: what just frame size is a mess.

Christopher Bergh: where we're going to talk about when you've got data that's constantly changing and…

Eric “DK” Estabrooks: We're just going to mute it, best case scenario you get data stewards domain experts in test gen and…

Christopher Bergh: being loaded. How do you find problems and anomalies and then the third is you've got a whole system of tools and…

Eric “DK” Estabrooks: they're actually able to look through these results probably,…

Christopher Bergh: data that's in production. How do you make sure that you don't have embarrassing errors so that I'm very excited about that one that there's a lot of good stuff to learn there.

Eric “DK” Estabrooks: alongside whoever's doing the analytics on it or the profiling analysis and let them kind of make some decisions more often than not though.

Eric “DK” Estabrooks: You're gonna have to send somebody a spreadsheet…

Christopher Bergh: …

Christopher Bergh: since we just released this last month,…

Eric “DK” Estabrooks: where we have a nice easy download for that.

Christopher Bergh: we only got 36 Stars. So help us out with some stars and…

Christopher Bergh: the repo, …

Eric “DK” Estabrooks: And it pops up. All the anomalies that you found and…

Christopher Bergh: if you would be so kind and…

Eric “DK” Estabrooks: what is it doing?

Christopher Bergh: just going through some details.

Eric “DK” Estabrooks: It's pulling in everything that we looked over there.

Christopher Bergh: I sent you an email on this, but just to repeat

Eric “DK” Estabrooks: It's non-standard values almost certainly an issue.

Christopher Bergh: First if you miss a session,…

Eric “DK” Estabrooks: We describe.

Christopher Bergh: don't worry, we'll email the slides in the video for you to review second is …

Eric “DK” Estabrooks: What we found kind of what does the anomaly mean? we're not a hundred percent on everything.

Christopher Bergh: we've got a free data Ops cookbook 250 Pages or…

Eric “DK” Estabrooks: But we try to explain our rationale for it some detail.

Christopher Bergh: the data Ops Manifesto.

Eric “DK” Estabrooks: And then…

Christopher Bergh: That's the idea. Eric's talked about…

Eric “DK” Estabrooks: what do we think suggested action is right.

Christopher Bergh: how to install our data observability software and…

Eric “DK” Estabrooks: So this one's considered cleansing the column replace the variance that we found with something else,…

Christopher Bergh: give the repo with and unfortunately, you've got to sign up for every session.

Eric “DK” Estabrooks: right and…

Christopher Bergh: I haven't found a way to do this…

Eric “DK” Estabrooks: sometimes or what review your Source data and…

Christopher Bergh: where you have to click on every one of those links and…

Eric “DK” Estabrooks: your ingestion process…

Christopher Bergh: and sign up and…

Eric “DK” Estabrooks: because we see that this is a problem and…

Christopher Bergh: join and if you …

Eric “DK” Estabrooks: it's probably not something that we can fix.

Christopher Bergh: if you don't just email…

Eric “DK” Estabrooks: We recommend you fixing again.

Christopher Bergh: if you can't do it just email me. I'll add you to the meeting…

Eric “DK” Estabrooks: These are our value judgments on things.

Christopher Bergh: but it's not too bad just to go and click your login information should be saved anyway,…

Eric “DK” Estabrooks: They're not a hundred percent so you gotta

Christopher Bergh: and then if you decide to have homework send it to Eric answer the questions,…

Eric “DK” Estabrooks: You got to use this as an advisor in a tool to build that big picture of things use it to help you focus in on areas of concern as well as with the data is…

Christopher Bergh: that's what you'll get them if the certificate and if you don't it's not a big deal if you're just sort of auditing in class or just want to listen and that's fine too, but we won't send you our Nifty PDF certification. So,…

Eric “DK” Estabrooks: but at the end of the day you as the owner of this step in the data quality of process use the data profiling owner.

Christopher Bergh: I

Christopher Bergh: You'll be crushed. But again appreciate you taking the time to do this. And we'll see you all again in two weeks.

Eric “DK” Estabrooks: I'm going to need to do the Judgment either on your own or…

Christopher Bergh: And…

Eric “DK” Estabrooks: in collaboration with your customers on…

Christopher Bergh: thank you very much for your time.

Eric “DK” Estabrooks: what to do with your anomalies if you find Thanks, everybody great day. all right. So that's the demo portion of it in the homework. You'll get to spend a lot more time with test gin and then as we go through different sessions test General come back for you. as we mentioned earlier on there's probably a data profiling tool in your stack already. You've ssis like, ETL tool there. I haven't used it in a long time. I know there was a data profiling note at one point. Alan used to do good on, data profiling data quality. They've moved away from their open source, So Your data governance tool your data catalogs. Everybody's throwing profiling in because it's part of a larger data quality activity.

Eric “DK” Estabrooks: 

01:25:00

Eric “DK” Estabrooks: From the open source standpoint obviously test gen is one. We recommend, there's also Great Expectations. why data profiling I think used to be called pandas profiling. Data cleaners got a Community Edition open refund. I think it was Google project patchy's got a contribution to this space again. These aren't necessarily data profiling only tools, but they are tools in the data quality space that includes some level of data profiling, depending on

Eric “DK” Estabrooks: what your technical environment looks like technical skill set, test gen serves a need. It's got a good UI even for SQL jockeys or even people like using the other tools. We've tried to make session be the tool that finds that balance of being sophisticated useful and fast and giving you a nice UI that you can use and then test gen itself isn't just profiling it includes testing and test Suites which then as Chris mentioned feeds into our data observability. So it's Part of more of a full solution for your testing needs and so the evolution path you get with test gen meets that and that's a good way to go. All right. Let's talk about homework. so