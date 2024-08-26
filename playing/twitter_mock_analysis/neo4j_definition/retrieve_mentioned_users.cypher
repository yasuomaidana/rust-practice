MATCH (u1:User {username:$username}), (u2:User)
MATCH (u1)-[m:MENTIONS]->(u2) RETURN u2