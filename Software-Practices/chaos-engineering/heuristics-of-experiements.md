# Running Chaos Experiments

* **Define steady state**. It is the normal behaviors of a system across time. We can use system or business metrics, with its ups and downs.
* **Form your hypothesis**. A hypothesis is a proposed explanation made based on limited resources as a starting point of our investigation. What if the primary database goes down, then we switch to the backup database. We use the scientific notation of If... then...
* **Plan and run your experiment**. Start with the smallest possible experiment, so we can contain our blast radius. Blast radius can be number of users affected, location of user effected, etc. Having a way of stopping the experiment is critical.
* **Measure and Learn**. The metrics that we had defined while defining steady states can now be used to validate your hypothesis - was the system robust or adjustable enough to handle the controlled failures?
* Next, **Scale up or abort and fix**. Each experiment may generate an action list or bug report or incident. Fix them now per your hypothesis.