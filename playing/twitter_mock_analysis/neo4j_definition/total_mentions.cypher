MATCH (u1:User), (u2:User)
MATCH (u1)-[m:MENTIONS]->(u2)
RETURN count(m) as count