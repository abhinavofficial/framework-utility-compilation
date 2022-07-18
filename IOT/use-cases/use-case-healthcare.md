# IoT in Healthcare

We would work out few typical Use cases

## Monitor, Track and Assist the Elderly
In confined or move. The data would be reviewed or observed in a datacenter.

### Care for Elderly and people with serious morbidity
* Elderly with serious morbidity such as Disabilities and/or diseases. Aim is to prevent, know the pattern and respond in case of an event.
* Old age - Reduced reflexes and strength, Slow recovery
* Need support for activities, stationary monitoring

### Sense and Measure
* Sense the vitals - Heart Rate, Activity, ECG
* Wearables or external sensors - deduce type of activity
* Track the location - home, park, transport, etc.

### Monitor and Track
* Vitals - Abnormal heart rate & rhythm, heart function (ECG)
* Correlate - Activity to heart rate, etc.
* Track - Location to ensure assistance reach
* Alert - Medical professionals when required

## Hospital Bedside Monitor
Stationary and particular healthcare. Quickest possible response.

### Patient care
* Ailment and disease treatment at hospital
* Particular health condition - Example: Cardiac care
* Quick response on variations in condition

### Sense and Measure - Continuous
* Vitals - Blood Pressure, Heart Rate, ECG, SpO2, Breathing, Body temperature
* Continuous monitoring for normalcy
* Alert on abnormal conditions - Example: lower SpO2, breathing rate change. These may need to be configured

### Monitor and Track
* Vitals over a period of time
* Episodes and patterns - diagnosis. Patterns can also be used across industry wide collection of data.
* Alerts for specified parameters

## Fitness - Sense, Monitor and Enhance
Typically, wearable typically on wrist

### Fitness - regular and sports
* Device as a “coach” and for data collection - regular fitness or sports
* Vitals - 2 lead ECG, Heart rate, type of activity
* Wearable with instant action/feedback
* Activity-to-benefits and/or goals

### Sense and Measure - Continuous
* Vitals - Heart Rate (HR), ECG, Body temperature, Activity level
* Location, movement and activity (and maybe bring in weather information)
* Indicate operating limits (personalized baseline) and bands - HR in normal or burn zone

### Monitor and Track
* Vitals over a period of time - patterns and benefits
* Alerts for specified parameters - HR and correlate activity
* Enhance fitness - trends and aggregate visualizations

## How do these cases differ?

A lot of edge computing on bedside monitoring or elderly care for quicker response time and some data sent to cloud while in case of fitness, we could have some edge computing but largely data sent to cloud for analysis.

## Architectural Requirements and Type of Data

We would want to build any system with necessary robustness, reliability, fit-for-purpose data collection for further analysis. This is where E-2-E (from sensor to server) will come into play. These requirements would be both functional and non-functional requirement. Functional requirements would be driven off the use cases and sensors, which are smart enough are thereby selected to solve the problem.
Sensor gets the data at a particular frequency. We need to retain all the good values and prevent outliers to distort the analysis (data cleansing). Each of these sensors would then be connected to server/sensors over appropriate network topology. Data collection interval and resolution of data also need to be factored into design considerations.
We are also looking at certain edge computing either at the gateway or within a computing infrastructure. Appropriate level of data should then be sent to cloud infra for further analysis and computation.

> Do not forget sometimes data can be fed in from outside these sensors, for example, manual data loads of failed devices, diagnosis input from a specialist, etc. 

Goal solving the problem - remove pain point, enhance the system, realize use cases which would otherwise not be possible. This is what forms a RoI which may not be in monetary terms (healthcare may be way too important).

### Architectural Requirements for our use cases

#### Availability
* Continuous data push to edge or cloud
* Immediate local alerts from mobile or traditional edge
* Cloud scale for a large number of devices

#### Device Monitoring
* Ensure “on” state and correct device functioning/diagnostics
* Device hardware (battery, tampering) and software (updates, security) checks and alerts

#### Stakeholder Updates
* Push to family members and healthcare staff on multiple communication channels

#### Data correlation
* Correlation between health parameters and activity/location
* Summarizing the data for appropriate user
* Avoid false alarms
* Personalization and better usability

#### Data security
* Sensitive healthcare data - Protection at rest and in transmission
* Protection against tempering

#### Pattern Analytics
* Aggregate analytics for behavior modeling
* Long term diagnosis and suggestions

### Type of data in our use case
Heterogeneous data
* Heart Rate - Collected by Electrode or Optical sensor
* ECG - Collected by Electrical signal trace measurement sensor
* Blood Pressure - Collected by Sphygmomanometer, Oscillometric sensor
* Oxygen Saturation (SpO2) - Collected by Pulse oximeter based on light absorption
* Respiratory Rate & Volume - Collected by Piezoresistive & differential ﬂow sensors
* Body temperature - Collected by Skin-touch wearable sensors
* Activity - Collected by Movement and acceleration sensors
* Location - Collected by GPS monitoring in wearable or mobile