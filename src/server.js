const express = require('express')
const path = require('path')

const publicPath = path.join(__dirname, '../public');

const app = express()
const port = 8888

app.use(express.static(publicPath))

app.get('/', (req, res) => {
    res.render('index')
})

app.listen(port, () => {
    console.log('Server up on port: ' + port)
})