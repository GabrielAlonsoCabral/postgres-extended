require("dotenv").config()

const cpuCount = require(".")

cpuCount.show({
    user:process.env.POSTGRES_USER,
    port:process.env.POSTGRES_PORT,
    host:process.env.POSTGRES_HOST,
    password:process.env.POSTGRES_PASSWORD,
    database:process.env.POSTGRES_DB,
})
