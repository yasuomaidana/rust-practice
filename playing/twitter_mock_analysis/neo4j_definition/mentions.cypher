MATCH (user1:User {username: $username_1}), (user2:User {username: $username_2})
WHERE NOT EXISTS( (user1)-[:KNOWS]->(user2) ) // Avoid creating duplicate relationships
CREATE (user1)-[:MENTIONS]->(user2)
