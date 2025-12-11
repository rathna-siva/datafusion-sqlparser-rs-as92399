MATCH (a:bug {name:'Wasp'}), (b:bug {name:'Bee'})
CREATE (a)-[:EATS]->(b);
