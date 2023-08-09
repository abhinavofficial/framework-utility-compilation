# Graph DB - Neo4J

## Graph Elements


Two elements that make up a graph are: Nodes (also known as vertices) and Relationships (also known as edges).

Nodes (or vertices) are the circles in a graph. Nodes commonly represent objects, entities, or merely things. In the picture below we can see 2 kinds of nodes - Person and Movie

Relationships (or edges) are used to describe how nodes are connected to each other. In the image below, ACTED_IN, REVIEWED, PRODUCED, WROTE and DIRECTED are all relationships connecting the corresponding types of nodes.

## Cypher Query Language
Cypher is a graph query language that is used to query the Neo4j Database. Cypher is like SQL - a declarative, textual query language.

In writing a cypher query, a node is enclosed between a parenthesis — like (p:Person) where p is a variable and Person is the type of node it is referring to.

Relationships are enclosed in square brackets - like [w:WORKS_FOR] where w is a variable and WORKS_FOR is the type of relationship it is referring to.

Putting it all together, the query below will return all people who have worked in Cloud Atlas movie. 

```
MATCH (p:Person)-[relatedTo]-(m:Movie {title: "Cloud Atlas"})
RETURN p, m, relatedTo
```

## SQL vs Cypher

Cypher is easy to learn and even more powerful and concise to write than SQL.

In graph, because we don't need join tables, Cypher expresses connections as graph patterns and is easy to read.

For example, if you want to find all actors that Tom Hanks has worked with, then below are the Cypher and SQL queries to fetch the data.

**Cypher**
```
MATCH (tom:Person {name: 'Tom Hanks'})-[a:ACTED_IN]->(m:Movie)<-[rel:ACTED_IN]-(p:Person)
return p, a, rel, m, tom
```

**SQL**
```
SELECT * FROM persons p JOIN roles r ON (p.id = r.person_id)
JOIN movies m ON (m.id = r.movie_id)
JOIN roles r2 ON (m.id = r2.movie_id)
JOIN persons p2 ON (p2.id = r2.person_id)
WHERE p2.name = "Tom Hanks"
```