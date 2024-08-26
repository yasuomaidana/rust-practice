UNWIND $data AS row
WITH row[0] AS user, row[1] AS mention

MERGE (u:User {username: user})
MERGE (m:User {username: mention})
MERGE (u)-[:RETWEETED]->(m)