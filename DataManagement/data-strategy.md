# Data Strategy

Source: [The Building Blocks of Data Strategy](https://www.linkedin.com/learning/data-strategy/the-building-blocks-of-data-strategy?autoSkip=true&autoplay=true&resume=false)

## Relationship between Data and Strategy

### Building blocks of data strategy

Even though data may seem undifferentiated in the sense that they are structured in the same way, that is not correct. There are different kind of data, and it is important to understand them to manage it effectively. Let's cover the six major types of data:

* **Master data**: It is the core data within enterprise that describes objects around which business is conducted. It is not transactional in nature, but it does describe transactions. According to DMBoK, most important types of master data are:
  * **Parties** - Individuals and organizations and
  * **Their roles**, such as customers, suppliers, employees, products, financial structures such as ledgers and cost centers, and locational concepts.
* **Unstructured data**: It is information that tends not to have a predefined structure or organization, and it tends not to have a predefined data model. It can be text heavier than other kinds of data. It can be found in emails, white papers, magazine articles, product specifications, marketing collateral, and through voice and video channels.
* **Transactional data**: It related to business events that have historical significance or are needed for analysis by other systems. It is often related to transactional systems such as sales, deliveries, invoices, claims, and the like. It typically describes an event. They have a time dimension, a numerical value, and refer to one or multiple objects.
* **Metadata**: It is the data that references other data. It may reside in a formal repository or various other forms such as XML documents, report definitions, column descriptions in the database, log files, connections, and other configuration files.
* **Hierarchical data**: It is often a set of data that are related to each other based on hierarchical relationships, sometimes considered a super master data management domain because it is critical to understand and sometimes discover the relationship between master data. It is where one datum is parent to another.
* **Reference data**: It is a specific type of master data that is used to categorize other data or used to relate data to information beyond the boundaries of the enterprise. It may include currencies, countries, timezones. They are often static or slowly changes over time.

### The six Vs of data

* **Volume**: It is the amount of data, which is likely to grow. Evaluating data volume relative to processing capacity is critical since that data is only useful if it can be effectively synthesized. It is also important to develop rules and algorithms that dictate how increased data volume will be managed.
* **Variety**: It refers to different types of data that flow through your company. There are structured data and there are unstructured data. It is important to understand, define and manage these varieties of data sources to categorize and use the insights from across those sources to drive your business forward.
* **Velocity**: It is the speed at which data is generated and collected. There is a need to process data quickly to take advantage of the context in which data is produced especially if it is related to trending situation which can provide the ability to capitalize on those insights and derive values from them and the ability to act quickly is key.
* **Value**: It is how to gauge whether you're managing your data effectively. This requires understanding which kinds of data potentially drive which kind of value. Questions like - which data will help identify cost savings or efficiencies in your operations, are you properly setting up the data streams that will allow you to capture that value, which types of data could generate insights that lead to revenue augmentation, which types of data would be able to identify new customer segments, etc. Use the volume of data to deliver new value.  
* **Veracity**: It will validate which data are and aren't reliable. It ultimately impacts your ability to make quality decisions. As a data leader, proving the veracity of data to colleagues and customers can also be a powerful way to build trust.
* **Variability**: It highlights how fast and broad your data structure changes over time. The key is to contextualize the data as you manage it to provide better structure to it. Remember that small deviations in data can impact the results of your analysis. When variability is high, it's necessary to continue to check the validity of conclusions you're drawing.

### Data Governance - the way to scale your data operation

A car's brakes allow you to go fast. Data governance sets standards and policies for the ability, usability, integrity, and security of data. When done well, data governance helps you grow fast, and do so with consistency. Without it, the data would not be in a form to be translated into information, knowledge, and ultimately wisdom, leading to action unless it is trusted. In fact, without proper data governance practices, it's not hyperbole to note that your data can actually be a liability.

Good governance practices should begin by aligning broader business, and data related activities. They should lead to greater standardization of data systems, policies, and procedures. It should have an added benefit of reducing operational friction.

Data governance should aid greater levels of transparency, monitoring, and tracking of data related activities. It should also increase the value for better decision-making along the way. It is also a way to de-risk the organization through simplifying all that must be managed, which should also have the added benefits of reduced costs. Finally, your company should drive increased data literacy through training and education.

There are several roles necessary to deliver data governance.

* The Data Governance Program Manager should define, and facilitate the development of the data governance operating model.
* The data steward is accountable for data and processes that ensure effective control and use of data assets. She/He should drive a more consistent set of policies and processes to manage data use, sharing, and collection, both internally, and externally.
* The data owner is responsible for ensuring that information is defined, governed, and used through systems and lines of business. The data owner should be clear point of contact who will own and manage data as strategic asset.
* The data architect is responsible for designing, and managing an organization's standard, and scalable approach for data management. This role provides a holistic view of how data is modeled, collected, processed, and distributed in a secured, secure, and scalable fashion across the company.
* The data user is a consumer of data for analytics, reporting, or regulatory needs. The role helps to ensure that the right data will be used by the right people to comply with regulations, and to perform in analytics.

## Who drives data strategy?

Chief Data Officer

### Role of Chief Data Officer

It is a role that is gaining steam in many companies. The tenure for such a role is 2 years and is such short partially due to good CDO's ability to jump from one company to the next, perhaps accruing more responsibilities and compensation in the process. However, the bigger reason is that the roles are often poorly defined, and the new chief data officer's ability to make change may be marred by a lack of clarity.

CIO also had short average tenures at the outset of the position due to lack of clarity regarding the role, as well as a lack of norms as to what good looked like for the role.

It's best if chief data officer have a mandate for change and accountability of data. This ensures that there is no misconception that the old way of doing things might still be possible. A lack of clarity on that part can cause politics and fear to get in the way of progress for the new leader. Optimally, they should be supported by CEO and perhaps even by the board to help inoculate the new executive from the political considerations that can impede progress.

For those hiring CDOs, it is critical that the new chief be empathetic, a great communicator and someone who is highly collaborative. This would help overcome the fear many will have that the new role represents a loss of some degree of autonomy over the data that their part of company gathers, which, truth be told is the reality of the situation.

At its best, the introduction of CDO is also an invitation for greater collaboration across the company, as the new leader should bring the leaders of the other functions together for better coordination of efforts across the company. Data strategy should not happen in silo - it should involve the traditional silos of the business more permeable so that collaborations can percolate across the enterprise.

### Enhancing Data Literacy across the enterprise

Data residing in each business not linked together. Build Data Lake - a repository of data stored in its raw format, ensuring the data is in a single place to be universally accessed.

* Train his team using analytics vendors.
* Create a machine learning training using a MOOC provider.
* Raise the data knowledge bar

## The puzzle pieces of Data Strategy

### Three interlocking components of data strategy

Data strategy is how structured data management enables the application of data to deliver business value.

Chris Davis, has written about this topic in his [column](https://lnkd.in/g5zcgS_7) and suggests that data strategy should have three interlocking components - business strategy, data application, and data management.

#### Business Strategy

Clearly defined business strategy should articulate the experience and outcomes that will be improved by the synthesis of data to develop information, knowledge, and wisdom. Use cases associated with business strategy create purpose, drive prioritization, and provide the rationale for investment into data capabilities. The rationale might come in the form of enhanced customer experience, development of new products and services, bolstered partner and go-to-market experiences, improve employee experiences, and the implementation of operational capabilities. This provides both an internal perspective relative to the company's operations and employees and the external perspective, relative to its customers and the products and services they buy.

#### Data application

Data application is the way an organization harnesses data to aid turning information into knowledge and ultimately into wisdom, as well as the means by which the company can drive action at scale. Data are most commonly applied through statistical models, visualizations of various kinds, and human or automated decision systems. A data strategy should define various formats, tools, and interfaces for how data can be used for reporting and descriptive analytics, as well as diagnostic, predictive, prescriptive, and cognitive analytics.

#### Data management

Data management fosters preparation of high-quality, scalable, secure, and trusted data to be used for general data application. Managing data requires defining your overall approach to data strategy, clearly specifying how your people, processes, technology, systems of measurement, and performance improvement will establish a sustainable operating model. The operating model will be required to define and manage data architecture and data platforms, data governance, data operations, data privacy, and data security. A critical discipline for complex organizations is to establish a master data management or MDM capability that enables trusted data to scale by integrating data governance into data operations.

#### The interlock

How can you evaluate the effectiveness of your company's data strategy? There will be specific metrics for many tactics you implement associated with the broader strategy, but at a high level, they should include better decisions, made faster than competitors based on the actions taken against the data strategy. Automation, enabling a product or service, a customer experience, or a business process to be delivered more rapidly with greater precision and accuracy at scale securely. If these first two items are in place, then the greater volumes of data that flow through the engines you develop should increase the value you derive. Therefore, the combination of speed, accuracy, and increased value, perhaps in the form of augmented revenue or decreased costs, should be the way to measure success.

### Data strategy's influence on five aspects of analysis

Since data is everywhere, creating a strategy to manage it can seem so difficult that one might simply choose to put the work off until an easier path materializes. To make the Herculean task of creating a data strategy easier, it's important to understand the types of analysis that the data will serve. These are

* **Descriptive reporting and analytics**: It refers to the method of using data to provide a retrospective view. This answers the question **what happened**? Examples of reporting that should be generated are transactional, financial, operational and historical reporting.
* **Diagnostic analytics**: It offers trends based on behaviors and correlations. It answers the question **why did this happen? And what can we learn from it?** Competitive analytics, performance evaluation, monitoring and alerts, self-service analytics and statistical analytics are each examples of diagnostic analytics.
* **Predictive analytics**: It comes into play when the amount of historical data enhances the ability to predict what might happen in the future. It answers the question **what's most likely to happen?** This should include a rules-based engine and ability to schedule outcomes, better forecasting the development of organizational scorecards and the development of dashboards. An example of predictive analytics can be found each time Netflix recommends a show to you based on what you've previously watched.
* **Prescriptive analytics**: It refers to analytics that provide suggestions about what actions to take based on previous analyses. This answers the question **what should we do?** Optimized outcomes, enhanced decision support, real-time analysis and machine learning are each examples of prescriptive analytics.
* **Cognitive analytics**. It approximates some facets of **human level intelligence when applied to common analytics tasks, bringing together analytics and practical decision-making**. Some combination of decision automation, natural language processing, neural networks, artificial intelligence, signal processing and robotics are examples of cognitive analytics.

Ultimately, this helps you get to know your users and customers better, leading to better customer and user experiences. It also leads to better decision-making where the evaluation of decisions proves to be correct. It also should drive cost-savings and other efficiencies. As more data is gathered and used, organizations can drive greater value by identifying issues and changing course as needed. This is what the author and New York University Stern School of Business Professor, Scott Galloway, refers to as the **Benjamin Button Effect**. Whereas most products are like humans, aging as time passes, the value of data increases with the passage of time so long as it's managed effectively.

We see this value creation regularly as consumers of companies, such as Amazon and Waze, which leverage data to help us find a new book to read, a show to watch or how best to avoid traffic. By focusing on the five aspects of analysis that are noted in this section, you can develop a better pathway to data and analytics success.

### From data warehouse to data lake to lake houses

Although the analysis of data goes back much further, in the 1980s, there was a hunger for better use of data and analytics. A field of business intelligence emerged, and with it, the concept of **data warehouses** that took external and operational data and combined that data from multiple sources into a single consistent data store loaded into data marts. The data could then be used for business intelligence and the generation of reports that once took hours of manual work. It was easy to locate, access and query data. As long as the data was well-organized and clean, the data preparation was straightforward. The downsides were that it was an expensive way to store and analyze unstructured data or streaming data, and it often required an overcommitment to a single vendor.

In the early part of the last decade, the concept of **data lakes** took shape. Both structured and unstructured data could be combined in a central storage repository, holding data from many sources in a raw granular format. This is the data lake. The data can either flow through to data preparation and validation, which will in turn, prepare the data for machine learning and data science activities, or it flows through to an extract transform and load, or ETL, which is the general procedure of copying data from one or more sources into a destination system, which represents the data differently from the source. The data can be translated to real-time databases or to data marts. From there, the data can be used for reporting and business intelligence analysis. The advantage of a data lake is that it works well for complex data processing, and it can handle structured and semi-structured data. It's also a cost-effective storage mechanism, no matter the data type. The data lake offers greater levels of flexibility.

Whereas data warehouses require predefined rules for storing data in order for it to be processed and queried, data lakes are more flexible, as they don't need to have predefined rules.

Ralph Loura, Chief Information Officer of Lumentum discovered that no matter the company or industry, there were remarkable inefficiencies in data analysis. He concluded that nearly 70% of activity related to data processing was simply moving data around, leaving only 30% of activity to actually adding value. One needs a way to store streaming data quickly and easily, and data warehouses tend not to be appropriate for that. Conversely, if you can't query, model and analyze that data while it's fresh and can provide business critical insights, you're not progressing at an appropriate level. This makes data lakes a less compelling choice. Ralph and Lumentum have implemented a hybrid approach between the data warehouse and data lake models, drawing upon the advantages of each, it's called a **data lake house**. As he noted to me, he believes this to be the option with the greatest flexibility. For instance, it allows one to connect many types of analytics engines. Data lake houses allow companies to move data between data lakes and purpose-built data stores more readily. They require a scalable data lakes be in place, that analytics services be purpose-built and that there be unified governance and unified data access. The use of data lake houses is an effective approach reflective of where data and analytics are headed, as opposed to where they've been. As this becomes more critical across the enterprise and as the rate of data growth continues to be exponential, implementing a data lake house is an important data strategy to consider.

### Data management across the organization

To bring data strategy to life requires data management. Data management is managing the vast amounts of data that you've collected to gain insights that will help you make better decisions. But there's more to it. 

The business strategy defines targeted outcomes. The data application defines how you'll apply data to address the outcomes. You might ask yourself what data do you need to determine your current revenue levels? How will you monitor its increase and define causality? Where did that increase come from? How will the data help you understand what's gone wrong, for example, and why was revenue growth not achieved in one of the tactical areas identified, among other questions you might ask.

Data management identifies how you store, prepare, maintain and secure data for the planned application. Let me define five areas that are critical to data management. They are

* **Data architecture and platforms**: Data architecture defines standards to collect, store, manipulate and disseminate data. Platforms provide an integrated set of technologies to meet your end-to-end data needs. The combination intends to deliver relevant data to team members who need it when they need it. It also ensures that when the team members interact with the data, the data has been organized to make higher-quality decisions based upon them.
* **Data governance**: This is the process of managing the availability, usability, integrity and security of the data in enterprise systems, using internal data standards and policies that also control data usage. When done effectively, data governance ensures consistency and trustworthiness of data. It's important to develop data governance awareness on the team, optimally even beyond those whose primary responsibilities are in the data realm.
* **Data operations**: It, sometimes referred to as data ops, defines the people, processes and products necessary to enable consistent and secure data in an automated fashion. It typically involves the combination and analysis of large databases, controlling the flow of data from its source, through to the point where it provides value.
* **Data privacy**: It relates to confidential data that must remain confidential, such as personal data, financial data, and data related to intellectual property. There are growing regulations that dictate data privacy at the state and federal level, protecting both the confidentiality, as well as the immutability of data. Regulations include the Payment Card Industry Data Security Standards or PCI DSS, often related to credit card payments and the Health Information Portability and Accountability Act or HIPAA related to health data.
* **Data security**: Data security is the field focused on protecting unauthorized access, theft or corruption of data. Though the topic of data would focus one's attention on security at the software and application level, it also must include physical security of storage devices and other hardware, as well as access controls. This should take into considerations threats from inside your company, as well as out, in addition to security issues that might result from human error.

Data management is a critical element to formulating a comprehensive data strategy and by focusing on each of these building blocks, you'll take what is a complex topic admittedly and build the right practices enabling better outcomes for the long term.

## The new Data Operating Model

### Data strategy's relationship with IT and digital strategy

Data strategy shouldn't be created in a vacuum. It's most effective when linked to business and IT strategy.

A business strategy should identify the why and the what. An IT strategy and data strategy translate the what into the how.

For those companies that attempt these strategies independently, they often have issues with adherence to data governance, regulatory and compliance standards, and have less ability to establish and maintain a single source of truth for enterprise data assets. These companies also may limit their ability to integrate real-time advanced analytics, automation, and machine learning into other core business applications. Finally, the disconnect may create gaps in the approaches that could become security issues. These can be serious impediments to long-term data success.

The IT strategy should articulate approaches to people, processes, and technology.

Data strategy should provide another level of specificity, especially if the data team reports to the IT chief. The data team may leverage the agile methods, the product orientation, the software development, and the cloud-based infrastructure that IT brings to life. The data team will also be responsible for unique and shared data management and data application capabilities to accomplish business strategies. The best data and analytics departments will be responsible both for the data architecture of its own data platform, visualization and analytical tools, and the data architecture that's a component of the company's overall enterprise architecture.

The business strategy should define experiences and outcomes that will be improved with data. They should include customer experience, employee experience, partner experience, operational capabilities, and product and service line innovation. Each of these should be defined through the strategy. And the way in which each might be enacted should be with better use of data. The data should help identify where the opportunities may lie in each of these domains. And they should also be used to gauge progress or lack thereof by providing metrics along the way. Therefore, data can help define the inputs as well as helping to define the efficacy of outcomes.

If there's a corporate objective to improve customer experience, for example, and there's a corresponding IT objective to improve digital customer experience, each layer can be brought to life through data strategy. The data strategy should ascertain current levels of customer and digital customer experience. As each goal is defined, the data should track progress toward these goals or lack thereof. The data strategy should also define how the data will be tracked using, for example, machine learning as a key component for data synthesis. You may feel as though you're swimming in data. By developing a well-defined data strategy linked to your company's corporate and IT strategies, you will drive better results using your data.

### Data strategy's relationship with business capabilities

Data strategy is the why of data. What are you solving for?

In the early stages of development of a data strategy, I recommend asking the following questions to help you identify specifics, since the initial use cases may be narrow in scope. The questions include: are consumer behaviors changing? Would it be helpful to leverage data, to track these changes, to garner insights on how to change them, or to influence that change? Is your company under cost pressure? Will collecting and analyzing data on various cost factors lead to better decisions on where to cut costs and where not to? Ultimately it's important to align strategies to business capabilities.

Business capabilities are an integrated set of processes, technologies, and deep expertise that are manifested as a functional capacity to capture or deliver value to the organization. They outline what the business should do to succeed, as opposed to how a business operates.

Lowe's chief information officer. Seemantini Godbole said, "When speaking with business partners and aligning IT strategy with the broader company strategy, it is important to speak the language of the business, and to be outcome-oriented." Business capabilities are a great way of accomplishing this, as they target the what without diving into the technical details, and connect strategy to near-term actions. When defined well, these business capabilities are a good place to turn to when developing a data strategy, as it helps identify topics of importance where better data can help advance the cause. Business capabilities are also useful in helping to determine the sequence and prioritization of different aspects of the data strategy.

One must continue to review data platforms and capabilities during this phase to highlight the flows of data. The data planning view establishes the current and future state of data, along with the migration path between them. This will clear the path to use the data effectively, leveraging reporting, business intelligence, advanced analytics, artificial intelligence, machine learning, and the like. Just like business technology in digital strategy, creating linkages to business capabilities are a great way to organize your efforts and build momentum for your data strategy.

### Developing a data-driven operating model

How do you focus the attention of your business on the right areas where data will have the greatest impact? A great place to start is to focus on customer outcomes. If you're able to improve customer experience, then you're on the surest pathway to grow your revenue and deepen your relationships with your customers.

Let me provide an example of Adobe to paint this picture more clearly. I like this example because it highlights how to organize a data-driven operating model at a significant scale in a dynamic company. It also highlights the need to focus efforts on some specific use cases that will be of high value. This provides the proving ground that will help you extend your data strategy more broadly across the enterprise. Starting in one area that is high value drives outcomes that matter while offering a learning environment to build on. Following this example will put your team on the path to successfully harness insights from your data.

#### Adobe's Data strategy

Adobe is a software as a service company that earns roughly $13 billion in annual revenue. The company's chief information officer is Cynthia Stoddard. She's been in her role for nearly six years, and has been a CIO multiple times, most recently at NetApp. Despite its scale, Adobe continues to grow at a fast pace, and early in her tenure, Cynthia realized that to support that growth, she needed to enable better decisions using data. As a result, she and her team helped define a data-driven operating model for Adobe. The area that Cynthia believed was most impactful was a data-driven framework to drive improvements in customer experiences. First, Cynthia aligned leaders of the various departments of the company meeting weekly to discuss performance based on a unifying plan that was developed to drive cross-functional collaboration. Once the framework was defined, IT introduced data management capabilities. The cross-functional leadership group agreed that the emphasis should be on information timeliness and accuracy. Cynthia identified that the key to delivering the data in the right way included consistency of measurement, having a well-defined governance process, and having a sound technology framework. Another ingredient for success has been to focus on well-defined customer-centric key performance indicators, which push the company to determine what data is focal. It requires assembling data from various sources that accurately represent the entire customer journey. An Adobe paper that covered this noted, "Being able to gather data, segment it, build audiences, and act on those segments is the backbone of personalization. The model helps us make sense of the data around us." The framework that was developed is called the Customer Improvement Framework. It involves five stages, discover, try, buy, use, and renew. Discover entails offering free signups for a product, for example. Trying it involves simply downloading and using it. The hope is to convert a meaningful percentage of those who try to buy. Ongoing use of the product is monitored and engagement scores are kept. Finally, client retention is evaluated through renewals. At each stage, data is collected and evaluated to determine the efficacy of the team's efforts, and then the team can course correct based on what the data tells the executive team.
