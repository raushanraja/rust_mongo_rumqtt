// Create a new database
db = db.getSiblingDB('relayday');

// Create a new user with readWrite role for 'mydatabase'
db.createUser({
    user: 'root',
    pwd: 'password',
    roles: [{ role: 'readWrite', db: 'relayday' }],
});
