MATCH (u1:User), (u2:User)
MATCH (u1)-[m:MENTIONS]->(u2) DELETE(m);
MATCH (u:User) DELETE (u)