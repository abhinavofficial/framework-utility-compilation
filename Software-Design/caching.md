# Caching

## What are the top caching strategies?

Read data from the system:
🔹 **Cache aside**: Application reads from Cache. If cache miss happen, it reads from DB and updates the cache.
🔹 **Read through**: Application reads from Cache. If cache miss happen, the data is fetched from DB and updated in cache directly, and then serviced to the caller. 

Write data to the system:
🔹 **Write around**: data is written in DB only; I/O completion is confirmed when data is written in DB
🔹 **Write back**: data is written in cache first; When data is written in cache, I/O completion is confirmed; data is written to DB asynchronously (background job) and does not block the request from being processed
🔹 **Write through**: Data is written in cache & DB; I/O completion is confirmed only when data is written in both places

![caching-strategies](images/caching-strategies.png)
