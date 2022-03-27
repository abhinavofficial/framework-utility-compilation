# Mongodb

## About MongoDB and MongoDB-Compass

MongoDB is a document-oriented NoSQL database used to store large amounts of data. Instead of using tables with rows and columns present in the Relational Databases traditionally, MongoDB uses a collection of documents. Documents consist of key-value pairs, which are the basic units of data in MongoDB. The collection contains a set of documents and functions that correspond to the tables in a relational database.

MongoDB compass is nothing but a graphical user interface that can be connected to the MongoDB database and used to find, analyze, modify, visualize the data stored in the database without requiring any knowledge of queries. MongoDB compass acts as an alternative to Mongo Shell. Mongo Shell can also perform all the aforementioned tasks but requires a lot of technical expertise. MongoDB is GUI-based whereas Shell is more technicality based i.e. it uses specific commands and queries. MongoDB Compass is present in the Github repo since it is an open-source tool.

Benefits of MongoDB Compass:
* All the data stored in the database can be visualized and explored.
* Data stored in the database can be modified, created, updated, deleted.
* Shows the Server Statistics in Real-time. 
* The Visual representations clearly depict the performance issues and recommend plans. 
* All the indexes can be managed. 
* JSON schema validation rules can be sued to validate data. 
* A wide range of plugins is available.

MongoDB Compass is a very powerful tool and all of its features are free as the full version is open source and available on GitHub. It makes use of MongoDB easier to Use as well.

## Installing mongodb

* [Install Doc](https://docs.mongodb.com/manual/installation/)
* [mongodb on Ubuntu](https://www.mongodb.com/docs/manual/tutorial/install-mongodb-on-ubuntu/) It has steps of uninstalling as well.

After installation, when running mongo-shell, following warnings were received.

```shell
abhinavofficial@ubuntu:~$ mongosh
Current Mongosh Log ID:	623ff2ed26f72f53564999fe
Connecting to:		mongodb://127.0.0.1:27017/?directConnection=true&serverSelectionTimeoutMS=2000&appName=mongosh+1.3.1
Using MongoDB:		5.0.6
Using Mongosh:		1.3.1

For mongosh info see: https://docs.mongodb.com/mongodb-shell/


To help improve our products, anonymous usage data is collected and sent to MongoDB periodically (https://www.mongodb.com/legal/privacy-policy).
You can opt-out by running the disableTelemetry() command.

------
   The server generated these startup warnings when booting:
   2022-03-26T22:14:02.300-07:00: Using the XFS filesystem is strongly recommended with the WiredTiger storage engine. See http://dochub.mongodb.org/core/prodnotes-filesystem
   2022-03-26T22:14:03.156-07:00: Access control is not enabled for the database. Read and write access to data and configuration is unrestricted
------
```

## Installing mongodb Compass
MongoDB Compass is a GUI application to access the DB and collection and data stored inside it. Similar to MySql Workbench

Again this is an open-source software which you can get from its official website. 

* [Install Guide Official Page](https://docs.mongodb.com/compass/master/install)